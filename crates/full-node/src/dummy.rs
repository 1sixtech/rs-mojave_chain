use futures::stream::{empty, Stream};
use mojave_chain_json_rpc::api::{
    eth::EthApi, eth_pubsub::EthPubSubApi, net::NetApi, web3::Web3Api,
};
use mojave_chain_types::{
    network::{AnyHeader, AnyRpcBlock, AnyRpcTransaction},
    primitives::{Address, Bytes, B256, U256, U64},
    rpc::*,
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

impl EthPubSubApi for DummyBackend {
    async fn subscribe_new_heads(&self) -> impl Stream<Item = Header<AnyHeader>> + Send + Unpin {
        empty()
    }

    async fn subscribe_logs(
        &self,
        _: Option<Box<Filter>>,
    ) -> impl Stream<Item = Log> + Send + Unpin {
        empty()
    }

    async fn subscribe_new_pending_transaction(&self) -> impl Stream<Item = B256> + Send + Unpin {
        empty()
    }
}

impl Web3Api for DummyBackend {
    type Error = DummyError;

    async fn client_version(&self) -> Result<String, Self::Error> {
        todo!()
    }

    async fn sha3(&self, _: Bytes) -> Result<String, Self::Error> {
        todo!()
    }
}

impl NetApi for DummyBackend {
    type Error = DummyError;

    async fn version(&self) -> Result<String, Self::Error> {
        todo!()
    }

    async fn peer_count(&self) -> Result<U64, Self::Error> {
        todo!()
    }

    async fn listening(&self) -> Result<bool, Self::Error> {
        todo!()
    }
}

impl EthApi for DummyBackend {
    type Error = DummyError;

    async fn accounts(&self) -> Result<Vec<Address>, Self::Error> {
        todo!()
    }

    async fn blob_base_fee(&self) -> Result<U256, Self::Error> {
        todo!()
    }

    async fn block_number(&self) -> Result<U256, Self::Error> {
        todo!()
    }

    async fn call(&self, _: EthCall) -> Result<Bytes, Self::Error> {
        todo!()
    }

    async fn chain_id(&self) -> Result<Option<U64>, Self::Error> {
        todo!()
    }

    async fn coinbase(&self) -> Result<Address, Self::Error> {
        todo!()
    }

    async fn create_access_list(
        &self,
        _: EthCreateAccessList,
    ) -> Result<AccessListResult, Self::Error> {
        todo!()
    }

    async fn estimate_gas(&self, _: EthEstimateGas) -> Result<U256, Self::Error> {
        todo!()
    }

    async fn fee_history(&self, _: EthFeeHistory) -> Result<FeeHistory, Self::Error> {
        todo!()
    }

    async fn gas_price(&self) -> Result<U256, Self::Error> {
        todo!()
    }

    async fn get_balance(&self, _: EthGetBalance) -> Result<U256, Self::Error> {
        todo!()
    }

    async fn get_block_by_hash(
        &self,
        _: EthGetBlockByHash,
    ) -> Result<Option<AnyRpcBlock>, Self::Error> {
        todo!()
    }

    async fn get_block_by_number(
        &self,
        _: EthGetBlockByNumber,
    ) -> Result<Option<AnyRpcBlock>, Self::Error> {
        todo!()
    }

    async fn get_block_receipts(
        &self,
        _: EthBlockReceipts,
    ) -> Result<Option<Vec<TransactionReceipt<TypedReceipt<Receipt<Log>>>>>, Self::Error> {
        todo!()
    }

    async fn get_block_transaction_count_by_hash(
        &self,
        _: EthGetBlockTransactionCountByHash,
    ) -> Result<Option<U256>, Self::Error> {
        todo!()
    }

    async fn get_block_transaction_count_by_number(
        &self,
        _: EthGetBlockTransactionCountByNumber,
    ) -> Result<Option<U256>, Self::Error> {
        todo!()
    }

    async fn get_code(&self, _: EthGetCode) -> Result<Bytes, Self::Error> {
        todo!()
    }

    async fn get_proof(&self, _: EthGetProof) -> Result<EIP1186AccountProofResponse, Self::Error> {
        todo!()
    }

    async fn get_storage_at(&self, _: EthGetStorageAt) -> Result<B256, Self::Error> {
        todo!()
    }

    async fn get_transaction_by_block_hash_and_index(
        &self,
        _: EthGetTransactionByBlockHashAndIndex,
    ) -> Result<Option<AnyRpcTransaction>, Self::Error> {
        todo!()
    }

    async fn get_transaction_by_block_number_and_index(
        &self,
        _: EthGetTransactionByBlockNumberAndIndex,
    ) -> Result<Option<AnyRpcTransaction>, Self::Error> {
        todo!()
    }

    async fn get_transaction_by_hash(
        &self,
        _: EthgetTransactionByHash,
    ) -> Result<Option<AnyRpcTransaction>, Self::Error> {
        todo!()
    }

    async fn get_transaction_count(&self, _: EthGetTransactionCount) -> Result<U256, Self::Error> {
        todo!()
    }

    async fn get_transaction_receipt(
        &self,
        _: EthGetTransactionReceipt,
    ) -> Result<Option<TransactionReceipt<TypedReceipt<Receipt<Log>>>>, Self::Error> {
        todo!()
    }

    async fn get_uncle_count_by_block_hash(
        &self,
        _: EthGetUncleCountByBlockHash,
    ) -> Result<U256, Self::Error> {
        todo!()
    }

    async fn get_uncle_count_by_block_number(
        &self,
        _: EthGetUncleCountByBlockNumber,
    ) -> Result<U256, Self::Error> {
        todo!()
    }

    async fn max_priority_fee_per_gas(&self) -> Result<U256, Self::Error> {
        todo!()
    }

    async fn send_raw_transaction(&self, _: EthSendRawTransaction) -> Result<B256, Self::Error> {
        todo!()
    }

    async fn send_transaction(&self, _: EthSendTransaction) -> Result<B256, Self::Error> {
        todo!()
    }

    async fn sign(&self, _: EthSign) -> Result<String, Self::Error> {
        todo!()
    }

    async fn sign_transaction(&self, _: EthSignTransaction) -> Result<String, Self::Error> {
        todo!()
    }

    async fn syncing(&self) -> Result<bool, Self::Error> {
        todo!()
    }

    async fn get_filter_changes(&self, _: String) -> Result<FilterChanges, Self::Error> {
        todo!()
    }

    async fn get_filter_logs(&self, _: String) -> Result<Vec<Log>, Self::Error> {
        todo!()
    }

    async fn get_logs(&self, _: Filter) -> Result<Vec<Log>, Self::Error> {
        todo!()
    }

    async fn new_block_filter(&self) -> Result<String, Self::Error> {
        todo!()
    }

    async fn new_filter(&self, _: Filter) -> Result<String, Self::Error> {
        todo!()
    }

    async fn new_pending_transaction_filter(&self) -> Result<String, Self::Error> {
        todo!()
    }

    async fn uninstall_filter(&self, _: String) -> Result<bool, Self::Error> {
        todo!()
    }
}
