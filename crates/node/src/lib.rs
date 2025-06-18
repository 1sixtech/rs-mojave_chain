mod args;
pub mod backend;
pub mod service;

use backend::{error::BackendError, Backend};
use clap::Parser;
use futures::FutureExt;
use mojave_chain_json_rpc::{
    config::RpcConfig,
    error::RpcServerError,
    server::{RpcServer, RpcServerHandle},
};
use mojave_chain_types::alloy::primitives::{utils::Unit, U256};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use crate::args::Args;

pub struct MojaveChainNode;

impl MojaveChainNode {
    pub async fn init() -> Result<MojaveChainNodeHandle, MojaveChainNodeError> {
        let _args = Args::parse();

        // Initialize anvil backend.
        let balance = Unit::ETHER.wei().saturating_mul(U256::from(10000u64));
        let node_config = anvil::NodeConfig::default().with_genesis_balance(balance);
        let (evm_client, evm_client_handle) = anvil::try_spawn(node_config)
            .await
            .map_err(|e| MojaveChainNodeError::Evm(e.to_string()))?;

        // Initialize the backend.
        let backend = Backend::init(evm_client);

        // Initialize RPC server.
        let rpc_config = RpcConfig::default();
        let rpc_server_handle = RpcServer::init(&rpc_config, backend, None).await?;

        let handle = MojaveChainNodeHandle {
            rpc_server: rpc_server_handle,
            evm_client_handle,
        };
        Ok(handle)
    }
}

pub struct MojaveChainNodeHandle {
    rpc_server: RpcServerHandle,
    #[allow(unused)]
    evm_client_handle: anvil::NodeHandle,
}

impl Future for MojaveChainNodeHandle {
    type Output = MojaveChainNodeError;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        if let Poll::Ready(error) = this.rpc_server.poll_unpin(cx) {
            return Poll::Ready(error.into());
        }

        Poll::Pending
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MojaveChainNodeError {
    #[error("RPC server error: {0}")]
    Rpc(#[from] RpcServerError),
    #[error("Backend error: {0}")]
    Backend(#[from] BackendError),
    #[error("EVM client error: {0}")]
    Evm(String),
}
