#[derive(Debug, thiserror::Error)]
pub enum MojaveClientError {
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Failed to serialize request body: {0}")]
    FailedToSerializeRequestBody(String),
    #[error("Error: {0}")]
    Custom(String),
    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("RPCError: {0}")]
    RpcError(String),
    #[error("Parse Url Error. {0}")]
    ParseUrlError(String),
}
