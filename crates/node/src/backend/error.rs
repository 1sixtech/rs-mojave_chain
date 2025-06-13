pub enum BackendError {
    EthApi(anvil::eth::error::BlockchainError),
    EthFilter(String),
    EthFilterResponse,
    Undefined,
    Unimplemented,
}

impl std::fmt::Debug for BackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EthApi(error) => write!(f, "{error}"),
            Self::EthFilter(error) => write!(f, "{error}"),
            Self::EthFilterResponse => write!(f, "Failed to decode ethFilter response"),
            Self::Broadcast(error) => write!(f, "Failed to broadcast the transaction: {error}"),
            Self::CheckTx(error) => write!(f, "Failed to check the transaction: {error}"),
            Self::Undefined => write!(f, "Undefined error"),
            Self::Unimplemented => write!(f, "Unimplemented"),
        }
    }
}

impl std::fmt::Display for BackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for BackendError {}
