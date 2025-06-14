#[derive(Debug, thiserror::Error)]
pub enum RpcServerError {
    #[error("Failed to build RPC server: {0}")]
    Build(std::io::Error),
    #[error("Failed to register RPC method: {0}")]
    RegisterMethod(jsonrpsee::core::RegisterMethodError),
    #[error("RPC server stopped")]
    RpcServerStopped,
    #[error("Websocket server stopped")]
    WebsocketServerStopped,
}

impl From<jsonrpsee::core::RegisterMethodError> for RpcServerError {
    fn from(value: jsonrpsee::core::RegisterMethodError) -> Self {
        Self::RegisterMethod(value)
    }
}
