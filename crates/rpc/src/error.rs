pub enum RpcServerError {
    Build(std::io::Error),
    RegisterMethod(jsonrpsee::core::RegisterMethodError),
    RpcServerStopped,
    WebsocketServerStopped,
}

impl From<jsonrpsee::core::RegisterMethodError> for RpcServerError {
    fn from(value: jsonrpsee::core::RegisterMethodError) -> Self {
        Self::RegisterMethod(value)
    }
}

impl std::fmt::Debug for RpcServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Build(error) => write!(f, "Failed to build RPC server: {error}"),
            Self::RegisterMethod(error) => write!(f, "Failed to register RPC method: {error}"),
            Self::RpcServerStopped => write!(f, "RPC server stopped"),
            Self::WebsocketServerStopped => write!(f, "Websocket server stopped"),
        }
    }
}

impl std::fmt::Display for RpcServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for RpcServerError {}
