use mojave_chain_json_rpc::api::web3::Web3Api;
use mojave_chain_types::{
    common::Bytes,
    primitives::{hex, utilities::keccak256},
};

use crate::{error::RPCError, RPC};

/// The client version: `mojave/v{major}.{minor}.{patch}`
pub const CLIENT_VERSION: &str = concat!("mojave/v", env!("CARGO_PKG_VERSION"));

impl Web3Api for RPC {
    type Error = RPCError;

    async fn client_version(&self) -> Result<String, Self::Error> {
        Ok(CLIENT_VERSION.to_string())
    }

    async fn sha3(&self, bytes: Bytes) -> Result<String, Self::Error> {
        let hash = keccak256(bytes.as_ref());
        Ok(hex::encode_prefixed(&hash[..]))
    }
}
