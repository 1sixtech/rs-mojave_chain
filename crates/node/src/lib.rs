pub mod backend;
pub mod service;

use backend::{error::BackendError, Backend};
use futures::FutureExt;
use mohave_chain_json_rpc::{
    config::RpcConfig,
    error::RpcServerError,
    server::{RpcServer, RpcServerHandle},
};
use mohave_chain_types::primitives::{utils::Unit, U256};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

pub struct MohaveChainNode;

impl MohaveChainNode {
    pub async fn init() -> Result<MohaveChainNodeHandle, MohaveChainNodeError> {
        // TODO: replace it with clap parser for advance CLI.
        let arguments: Vec<String> = env::args().skip(1).collect();
        let home_directory = arguments.first().expect("Provide the home directory");

        // Initialize anvil backend.
        let balance = Unit::ETHER.wei().saturating_mul(U256::from(10000u64));
        let node_config = anvil::NodeConfig::default().with_genesis_balance(balance);
        let (evm_client, evm_client_handle) = anvil::try_spawn(node_config)
            .await
            .map_err(|e| DRiPNodeError::Evm(e.to_string()))?;

        // Initialize the backend.
        let backend = Backend::init(evm_client);

        // Initialize RPC server.
        let rpc_config = RpcConfig::default();
        let rpc_server_handle = RpcServer::init(&rpc_config, backend).await?;

        let handle = MohaveChainNodeHandle {
            rpc_server: rpc_server_handle,
            evm_client_handle,
        };
        Ok(handle)
    }
}

pub struct MohaveChainNodeHandle {
    rpc_server: RpcServerHandle,
    #[allow(unused)]
    evm_client_handle: anvil::NodeHandle,
}

impl Future for MohaveChainNodeHandle {
    type Output = MohaveChainNodeError;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        if let Poll::Ready(error) = this.rpc_server.poll_unpin(cx) {
            return Poll::Ready(error.into());
        }

        Poll::Pending
    }
}

#[derive(Debug)]
pub enum MohaveChainNodeError {
    Rpc(RpcServerError),
    Backend(BackendError),
    MissingHomeDirectory,
    InvalidRpcAddress(String),
    Evm(String),
}

impl std::fmt::Display for MohaveChainNodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AbciServer(value) => write!(f, "ABCI server error: {value}"),
            Self::AbciClient(value) => write!(f, "ABCI client error: {value}"),
            Self::Rpc(value) => write!(f, "RPC server error: {value}"),
            Self::Backend(value) => write!(f, "Backend error: {value}"),
            Self::MissingHomeDirectory => write!(f, "Home directory CLI argument missing"),
            Self::InvalidRpcAddress(value) => {
                write!(
                    f,
                    "Invalid RPC address returned from CometBFT config: {value}"
                )
            }
            Self::Evm(value) => write!(f, "EVM client error: {value}"),
        }
    }
}

impl std::error::Error for MohaveChainNodeError {}

impl From<RpcServerError> for MohaveChainNodeError {
    fn from(value: RpcServerError) -> Self {
        Self::Rpc(value)
    }
}

impl From<BackendError> for MohaveChainNodeError {
    fn from(value: BackendError) -> Self {
        Self::Backend(value)
    }
}
