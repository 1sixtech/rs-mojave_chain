use crate::alloy::primitives::{Address, B256, Bytes, U64, U256};
/// Re-export RPC types
pub use alloy::rpc::types::*;
pub use anvil_core::eth::transaction::TypedReceipt;
pub use anvil_rpc::response::ResponseResult;
use serde::{Deserialize, Serialize};
use serde_helpers::WithOtherFields;

#[derive(Debug, Deserialize, Serialize)]
pub struct EthCall {
    pub request: WithOtherFields<TransactionRequest>,
    pub block_number: Option<BlockId>,
    // pub state_overrides: Option<StateOverride>,
    // pub block_overrides: Option<Box<BlockOverrides>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthCreateAccessList {
    pub request: WithOtherFields<TransactionRequest>,
    pub block_number: Option<BlockId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthEstimateGas {
    pub request: WithOtherFields<TransactionRequest>,
    // pub block_number: Option<BlockId>,
    // pub state_override: Option<StateOverride>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthFeeHistory {
    pub block_count: U64,
    pub newest_block: BlockNumberOrTag,
    pub reward_percentiles: Option<Vec<f64>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthGetBalance {
    pub address: Address,
    pub block_number: Option<BlockId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthGetBlockByHash {
    pub hash: B256,
    pub full: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthGetBlockByNumber {
    pub number: BlockNumberOrTag,
    pub full: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthBlockReceipts {
    pub block_id: BlockId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthGetBlockTransactionCountByHash {
    pub hash: B256,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthGetBlockTransactionCountByNumber {
    pub number: BlockNumberOrTag,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthGetCode {
    pub address: Address,
    pub block_number: Option<BlockId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthGetProof {
    pub address: Address,
    pub keys: Vec<B256>,
    pub block_number: Option<BlockId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthGetStorageAt {
    pub address: Address,
    pub index: U256,
    pub block_number: Option<BlockId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthGetTransactionByBlockHashAndIndex {
    pub hash: B256,
    pub index: Index,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthGetTransactionByBlockNumberAndIndex {
    pub number: BlockNumberOrTag,
    pub index: Index,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthgetTransactionByHash {
    pub hash: B256,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthGetTransactionCount {
    pub address: Address,
    pub block_number: Option<BlockId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthGetTransactionReceipt {
    pub hash: B256,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthGetUncleCountByBlockHash {
    pub hash: B256,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthGetUncleCountByBlockNumber {
    pub number: BlockNumberOrTag,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthSendRawTransaction {
    pub bytes: Bytes,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthSendTransaction {
    pub transaction: WithOtherFields<TransactionRequest>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthSign {
    pub address: Address,
    pub message: Bytes,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EthSignTransaction {
    pub transaction: WithOtherFields<TransactionRequest>,
}

pub type TransactionHash = B256;
