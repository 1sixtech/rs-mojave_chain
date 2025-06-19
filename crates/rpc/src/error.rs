#[derive(Debug, thiserror::Error)]
pub enum RPCError {
    #[error("Eth client error: {0}")]
    EthClientError(#[from] ethrex_rpc::clients::eth::errors::EthClientError),
}
