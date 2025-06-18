#[derive(thiserror::Error, Debug)]
pub enum RpcServerError {
    #[error("Failed to build JSON-RPC server: `{0}`")]
    Build(std::io::Error),
    #[error("Failed to register JSON-RPC method: `{0}`")]
    RegisterMethod(jsonrpsee::core::RegisterMethodError),
    #[error("JSON-RPC server stopped")]
    RpcServerStopped,
    #[error("JSON-RPC websocket server stopped")]
    WebsocketServerStopped,
    #[error("JSON-RPC shut down signal received")]
    ShutdownSignalReceived,
}

impl From<jsonrpsee::core::RegisterMethodError> for RpcServerError {
    fn from(value: jsonrpsee::core::RegisterMethodError) -> Self {
        Self::RegisterMethod(value)
    }
}
