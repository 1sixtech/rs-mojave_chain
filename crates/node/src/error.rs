use drip_chain_abci::{client::AbciClientError, server::AbciServerError};
use drip_chain_rpc::error::RpcServerError;

use crate::backend::error::BackendError;

#[derive(Debug, thiserror::Error)]
pub enum DRiPNodeError {
    #[error("ABCI server error: {0}")]
    AbciServer(#[from] AbciServerError),
    #[error("ABCI client error: {0}")]
    AbciClient(#[from] AbciClientError),
    #[error("RPC server error: {0}")]
    Rpc(#[from] RpcServerError),
    #[error("Backend error: {0}")]
    Backend(#[from] BackendError),
    #[error("Home directory CLI argument missing")]
    MissingHomeDirectory,
    #[error("Invalid RPC address returned from CometBFT config: {0}")]
    InvalidRpcAddress(String),
    #[error("EVM client error: {0}")]
    Evm(String),
}
