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
            Ok(()) => AbciServerError::Server(None),
            Err(error) => AbciServerError::Server(Some(error)),
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

pub enum AbciServerError {
    Config(crate::config::CometBftConfigError),
    TimeoutCommitIsZero,
    BufferSize(std::num::TryFromIntError),
    Build(tendermint_abci::Error),
    Server(Option<tendermint_abci::Error>),
    CometBft(String),
    JoinServer,
    JoinCometBftNode(tokio::task::JoinError),
    MissingServerHandle,
}

impl std::fmt::Debug for AbciServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Config(error) => write!(f, "CometBFT config error: {error}"),
            Self::TimeoutCommitIsZero => write!(f, "Set timeout_commit to value other than zero"),
            Self::BufferSize(error) => write!(f, "CometBFT buffer size error: {error}"),
            Self::Build(error) => write!(f, "Failed to build ABCI server: {error}"),
            Self::Server(error) => match error {
                Some(error) => write!(f, "ABCI server stopped with an error: {error}"),
                None => write!(f, "ABCI server stopped"),
            },
            Self::CometBft(error) => write!(f, "CometBFT node stopped with an error: {error}"),
            Self::JoinServer => write!(f, "Failed to join ABCI server"),
            Self::JoinCometBftNode(error) => write!(f, "Failed to join CometBFT node: {error}"),
            Self::MissingServerHandle => write!(f, "ABCI server handle returned None"),
        }
    }
}

impl std::fmt::Display for AbciServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for AbciServerError {}
