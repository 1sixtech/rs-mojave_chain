use crate::backend::{error::BackendError, Backend};
use mojave_chain_json_rpc::api::net::NetApi;
use mojave_chain_types::alloy::primitives::U64;

impl NetApi for Backend {
    type Error = BackendError;

    async fn version(&self) -> Result<String, Self::Error> {
        Ok(U64::from(self.evm_client().chain_id()).to_string())
    }

    async fn peer_count(&self) -> Result<U64, Self::Error> {
        Ok(U64::from(0))
    }

    async fn listening(&self) -> Result<bool, Self::Error> {
        self.evm_client()
            .net_listening()
            .map_err(BackendError::EthApi)
    }
}
