use std::sync::Arc;

pub mod error;
pub mod eth;
pub mod net;
pub mod web3;

#[derive(Clone)]
pub struct RPC {
    inner: Arc<RPCInner>,
}

struct RPCInner {
    evm_client: ethrex_rpc::clients::eth::EthClient,
}

impl RPC {
    pub fn new(evm_client: ethrex_rpc::clients::eth::EthClient) -> Self {
        Self {
            inner: Arc::new(RPCInner { evm_client }),
        }
    }

    pub fn evm_client(&self) -> &ethrex_rpc::clients::eth::EthClient {
        &self.inner.evm_client
    }
}
