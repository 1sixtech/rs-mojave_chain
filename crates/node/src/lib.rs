pub mod backend;
pub mod service;

use backend::Backend;
use clap::Parser;
use drip_chain_abci::{
    client::AbciClient,
    server::{AbciServer, AbciServerHandle},
};
use drip_chain_rpc::{
    config::RpcConfig,
    server::{RpcServer, RpcServerHandle},
};
use drip_chain_types::primitives::{utils::Unit, U256};
use futures::FutureExt;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

mod args;
mod error;

use crate::error::DRiPNodeError;

pub struct DRiPNode;

impl DRiPNode {
    pub async fn init() -> Result<DRiPNodeHandle, DRiPNodeError> {
        let args = args::Args::parse();
        let home_directory = args.home_directory;

        // Initialize anvil backend.
        let balance = Unit::ETHER.wei().saturating_mul(U256::from(10000u64));
        let node_config = anvil::NodeConfig::default().with_genesis_balance(balance);
        let (evm_client, evm_client_handle) = anvil::try_spawn(node_config)
            .await
            .map_err(|e| DRiPNodeError::Evm(e.to_string()))?;

        // Initialize ABCI configuration and client.
        let abci_config = AbciServer::<Backend>::init_config(&home_directory)?;
        let abci_rpc_address = {
            let rpc_address = abci_config.rpc.laddr.to_string();
            let address = rpc_address
                .split_once("://")
                .map(|(_, addr)| addr)
                .ok_or_else(|| DRiPNodeError::InvalidRpcAddress(rpc_address.clone()))?;
            format!("http://{address}")
        };
        let abci_client = AbciClient::new(abci_rpc_address)?;

        // Initialize the backend.
        let backend = Backend::init(evm_client, abci_client);

        // Initialize ABCI server.
        let abci_server_handle = AbciServer::init(&home_directory, abci_config, backend.clone())?;

        // Initialize RPC server.
        let rpc_config = RpcConfig::default();
        let rpc_server_handle = RpcServer::init(&rpc_config, backend).await?;

        let handle = DRiPNodeHandle {
            abci_server: abci_server_handle,
            rpc_server: rpc_server_handle,
            evm_client_handle,
        };
        Ok(handle)
    }
}

pub struct DRiPNodeHandle {
    abci_server: AbciServerHandle,
    rpc_server: RpcServerHandle,
    #[allow(unused)]
    evm_client_handle: anvil::NodeHandle,
}

impl Future for DRiPNodeHandle {
    type Output = DRiPNodeError;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        if let Poll::Ready(error) = this.abci_server.poll_unpin(cx) {
            return Poll::Ready(error.into());
        }

        if let Poll::Ready(error) = this.rpc_server.poll_unpin(cx) {
            return Poll::Ready(error.into());
        }

        Poll::Pending
    }
}
