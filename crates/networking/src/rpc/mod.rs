mod block;
pub mod clients;
pub mod full_node;
pub mod sequencer;
pub mod utils;

use crate::{
    rpc::utils::RpcRequest,
    // sync::SyncClient,
};
use serde::Deserialize;
use std::time::Duration;

pub const FILTER_DURATION: Duration = Duration::from_secs(300);

#[derive(Deserialize)]
#[serde(untagged)]
pub enum RpcRequestWrapper {
    Single(RpcRequest),
    Multiple(Vec<RpcRequest>),
}
