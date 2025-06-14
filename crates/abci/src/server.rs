use super::api::AbciApi;
use crate::config::CometBftConfig;
use futures::FutureExt;
use std::{
    future::Future,
    marker::PhantomData,
    path::PathBuf,
    pin::Pin,
    process::Command as BlockingCommand,
    task::{Context, Poll},
    thread::{self, JoinHandle as ThreadJoinHandle},
};
use tendermint_abci::ServerBuilder;
use tendermint_config::net::Address;
use tokio::{process::Command, task::JoinHandle};

pub struct AbciServer<T>
where
    T: AbciApi,
{
    backend: PhantomData<T>,
}

impl<T> AbciServer<T>
where
    T: AbciApi,
{
    #[allow(clippy::result_large_err)]
    pub fn init_config(home_directory: impl AsRef<str>) -> Result<CometBftConfig, AbciServerError> {
        let mut cometbft_node = BlockingCommand::new("cometbft");
        cometbft_node.args(["init", "--home", home_directory.as_ref()]);
        cometbft_node
            .output()
            .map_err(|_| AbciServerError::CometBft("CometBFT not installed".to_owned()))?;

        let config_path = PathBuf::from(home_directory.as_ref())
            .join("config")
            .join("config.toml");
        let config = CometBftConfig::from_file(config_path).map_err(AbciServerError::Config)?;
        if config.consensus.timeout_commit.as_secs() == 0 {
            return Err(AbciServerError::TimeoutCommitIsZero);
        }
        Ok(config)
    }

    fn start_cometbft_node(
        home_directory: impl AsRef<str>,
        proxy_app_address: impl AsRef<str>,
    ) -> JoinHandle<AbciServerError> {
        let mut cometbft_node = Command::new("cometbft");
        cometbft_node.args([
            "start",
            "--home",
            home_directory.as_ref(),
            "--proxy_app",
            proxy_app_address.as_ref(),
        ]);

        tokio::spawn(async move {
            match cometbft_node.kill_on_drop(true).spawn() {
                Ok(mut handle) => match handle.wait().await {
                    Ok(status) => AbciServerError::CometBft(status.to_string()),
                    Err(error) => AbciServerError::CometBft(error.to_string()),
                },
                Err(error) => AbciServerError::CometBft(error.to_string()),
            }
        })
    }

    #[allow(clippy::result_large_err)]
    pub fn init(
        home_directory: impl AsRef<str>,
        config: CometBftConfig,
        backend: T,
    ) -> Result<AbciServerHandle, AbciServerError> {
        let max_buffer_size: usize = config
            .mempool
            .max_tx_bytes
            .try_into()
            .map_err(AbciServerError::BufferSize)?;
        let address = match config.proxy_app {
            Address::Tcp {
                peer_id: _,
                host,
                port,
            } => format!("{host}:{port}"),
            Address::Unix { path: _ } => {
                return Err(AbciServerError::CometBft(
                    "Unexpected address type".to_owned(),
                ));
            }
        };

        let server = ServerBuilder::new(max_buffer_size)
            .bind(&address, backend)
            .map_err(AbciServerError::Build)?;
        let server_handle = thread::spawn(move || match server.listen() {
            Ok(()) => AbciServerError::ServerStopped,
            Err(error) => AbciServerError::ServerError(error),
        });

        let cometbft_node_handle = Self::start_cometbft_node(home_directory, address);

        let abci_server_handle = AbciServerHandle {
            server: Some(server_handle),
            cometbft_node: cometbft_node_handle,
        };
        Ok(abci_server_handle)
    }
}

pub struct AbciServerHandle {
    server: Option<ThreadJoinHandle<AbciServerError>>,
    cometbft_node: JoinHandle<AbciServerError>,
}

impl Future for AbciServerHandle {
    type Output = AbciServerError;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        if let Some(server_handle) = this.server.take() {
            match server_handle.is_finished() {
                false => {}
                true => match server_handle.join() {
                    Ok(value) => return Poll::Ready(value),
                    Err(_error) => return Poll::Ready(AbciServerError::JoinServer),
                },
            }

            this.server.replace(server_handle);
        } else {
            return Poll::Ready(AbciServerError::MissingServerHandle);
        }

        match this.cometbft_node.poll_unpin(cx) {
            Poll::Pending => {}
            Poll::Ready(value) => match value {
                Ok(value) => return Poll::Ready(value),
                Err(error) => return Poll::Ready(AbciServerError::JoinCometBftNode(error)),
            },
        }

        Poll::Pending
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AbciServerError {
    #[error("CometBFT config error: {0}")]
    Config(crate::config::CometBftConfigError),
    #[error("Set timeout_commit to value other than zero")]
    TimeoutCommitIsZero,
    #[error("CometBFT buffer size error: {0}")]
    BufferSize(std::num::TryFromIntError),
    #[error("Failed to build ABCI server: {0}")]
    Build(tendermint_abci::Error),
    #[error("ABCI server stopped with an error: {0}")]
    ServerError(tendermint_abci::Error),
    #[error("ABCI server stopped unexpectedly")]
    ServerStopped,
    #[error("CometBFT node stopped with an error: {0}")]
    CometBft(String),
    #[error("Failed to join ABCI server")]
    JoinServer,
    #[error("Failed to join CometBFT node: {0}")]
    JoinCometBftNode(tokio::task::JoinError),
    #[error("ABCI server handle returned None")]
    MissingServerHandle,
}
