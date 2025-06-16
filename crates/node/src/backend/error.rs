#[derive(Debug, thiserror::Error)]
pub enum BackendError {
    #[error("Ethereum API error: {0}")]
    EthApi(anvil::eth::error::BlockchainError),
    #[error("Failed to decode ethFilter response: {0}")]
    EthFilter(String),
    #[error("Failed to decode ethFilter response")]
    EthFilterResponse,
    #[error("Undefined error")]
    Undefined,
    #[error("Unimplemented")]
    Unimplemented,
}
