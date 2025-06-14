#[derive(Debug, thiserror::Error)]
pub enum BackendError {
    #[error("Ethereum API error: {0}")]
    EthApi(anvil::eth::error::BlockchainError),
    #[error("Failed to decode ethFilter response: {0}")]
    EthFilter(String),
    // CheckTx error.
    #[error("Failed to broadcast the transaction: {0}")]
    Broadcast(drip_chain_abci::client::AbciClientError),
    #[error("Failed to check the transaction: {0}")]
    CheckTx(String),
    #[error("Undefined error")]
    Undefined,
    #[error("Unimplemented")]
    Unimplemented,
    #[error("Failed to mine a block {0}")]
    BlockchainError(#[from] anvil::eth::error::BlockchainError),
    #[error("Failed to mine a block got none")]
    BlockNone,
}
