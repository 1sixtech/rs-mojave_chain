use mojave_chain_json_rpc::api::{
    eth::EthApi, eth_pubsub::EthPubSubApi, net::NetApi, web3::Web3Api,
};

#[derive(Debug, thiserror::Error)]
pub enum DummyError {}

#[derive(Clone)]
pub struct DummyBackend;

impl DummyBackend {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for DummyBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl EthPubSubApi for DummyBackend {}

impl Web3Api for DummyBackend {
    type Error = DummyError;

    async fn client_version(&self) -> Result<String, Self::Error> {
        todo!()
    }
}

impl NetApi for DummyBackend {
    type Error = DummyError;

    async fn version(&self) -> Result<String, Self::Error> {
        todo!()
    }

    async fn listening(&self) -> Result<bool, Self::Error> {
        todo!()
    }
}

impl EthApi for DummyBackend {
    type Error = DummyError;

    async fn syncing(&self) -> Result<bool, Self::Error> {
        todo!()
    }

    async fn new_block_filter(&self) -> Result<String, Self::Error> {
        todo!()
    }

    async fn new_pending_transaction_filter(&self) -> Result<String, Self::Error> {
        todo!()
    }

    async fn uninstall_filter(&self, _: String) -> Result<bool, Self::Error> {
        todo!()
    }
}
