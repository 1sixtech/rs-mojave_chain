use mojave_chain_json_rpc::api::net::NetApi;
use mojave_chain_types::common::U64;

use crate::{error::RPCError, RPC};

impl NetApi for RPC {
    type Error = RPCError;

    async fn version(&self) -> Result<String, Self::Error> {
        Ok(self.evm_client().get_chain_id().await?.to_string())
    }

    async fn peer_count(&self) -> Result<U64, Self::Error> {
        todo!()
    }

    async fn listening(&self) -> Result<bool, Self::Error> {
        todo!()
    }
}
