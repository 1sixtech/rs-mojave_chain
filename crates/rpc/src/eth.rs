use ethrex_rpc::clients::eth::BlockByNumber;
use mojave_chain_json_rpc::api::eth::EthApi;
use mojave_chain_types::{
    //network::{AnyRpcBlock, AnyRpcTransaction},
    common::{types::GenericTransaction, Address, Bytes, H256, U256},
    primitives::B256,
    rpc::*,
};

use crate::{error::RPCError, RPC};

impl EthApi for RPC {
    type Error = RPCError;

    async fn accounts(&self) -> Result<Vec<Address>, Self::Error> {
        todo!()
    }

    async fn blob_base_fee(&self) -> Result<U256, Self::Error> {
        todo!()
    }

    async fn block_number(&self) -> Result<U256, Self::Error> {
        self.evm_client()
            .get_block_number()
            .await
            .map_err(RPCError::from)
    }

    async fn call(&self, _parameter: EthCall) -> Result<Bytes, Self::Error> {
        todo!()
    }

    async fn chain_id(&self) -> Result<Option<U256>, Self::Error> {
        let chain_id = self
            .evm_client()
            .get_chain_id()
            .await
            .map_err(RPCError::from)?;
        Ok(Some(chain_id))
    }

    async fn coinbase(&self) -> Result<Address, Self::Error> {
        todo!()
    }

    //async fn create_access_list(
    //    &self,
    //    _parameter: EthCreateAccessList,
    //) -> Result<AccessListResult, Self::Error> {
    //    todo!()
    //}

    async fn estimate_gas(&self, parameter: EthEstimateGas) -> Result<U256, Self::Error> {
        let gas = self
            .evm_client()
            .estimate_gas(parameter.request.into())
            .await
            .map_err(RPCError::from)?;
        Ok(U256::from(gas))
    }

    //async fn fee_history(&self, _parameter: EthFeeHistory) -> Result<FeeHistory, Self::Error> {
    //    todo!()
    //}

    async fn gas_price(&self) -> Result<U256, Self::Error> {
        todo!()
    }

    async fn get_balance(&self, parameter: EthGetBalance) -> Result<U256, Self::Error> {
        todo!()
    }

    //fn block_id_to_block_numer(&self, block: BlockId) -> Result<BlockByNumber, Self::Error> {
    //    todo!()
    //}

    //async fn get_block_by_hash(
    //    &self,
    //    _parameter: EthGetBlockByHash,
    //) -> Result<Option<AnyRpcBlock>, Self::Error> {
    //    todo!()
    //}

    //async fn get_block_by_number(
    //    &self,
    //    _parameter: EthGetBlockByNumber,
    //) -> Result<Option<AnyRpcBlock>, Self::Error> {
    //    todo!()
    //}

    //async fn get_block_receipts(&self, _parameter: EthBlockReceipts) -> Result<Option<Vec<TransactionReceipt<TypedReceipt<Receipt<Log>>>>>, Self::Error> {
    //    todo!()
    //}

    async fn get_block_transaction_count_by_hash(
        &self,
        _parameter: EthGetBlockTransactionCountByHash,
    ) -> Result<Option<U256>, Self::Error> {
        todo!()
    }

    async fn get_block_transaction_count_by_number(
        &self,
        _parameter: EthGetBlockTransactionCountByNumber,
    ) -> Result<Option<U256>, Self::Error> {
        todo!()
    }

    async fn get_code(&self, _parameter: EthGetCode) -> Result<Bytes, Self::Error> {
        todo!()
    }

    //async fn get_proof(
    //    &self,
    //    _parameter: EthGetProof,
    //) -> Result<EIP1186AccountProofResponse, Self::Error> {
    //    todo!()
    //}

    async fn get_storage_at(&self, _parameter: EthGetStorageAt) -> Result<B256, Self::Error> {
        todo!()
    }

    //async fn get_transaction_by_block_hash_and_index(
    //    &self,
    //    _parameter: EthGetTransactionByBlockHashAndIndex,
    //) -> Result<Option<AnyRpcTransaction>, Self::Error> {
    //    todo!()
    //}

    //async fn get_transaction_by_block_number_and_index(
    //    &self,
    //    _parameter: EthGetTransactionByBlockNumberAndIndex,
    //) -> Result<Option<AnyRpcTransaction>, Self::Error> {
    //    todo!()
    //}

    //async fn get_transaction_by_hash(
    //    &self,
    //    _parameter: EthgetTransactionByHash,
    //) -> Result<Option<AnyRpcTransaction>, Self::Error> {
    //    todo!()
    //}

    async fn get_transaction_count(
        &self,
        _parameter: EthGetTransactionCount,
    ) -> Result<U256, Self::Error> {
        todo!()
    }

    //async fn get_transaction_receipt(
    //    &self,
    //    _parameter: EthGetTransactionReceipt,
    //) -> Result<Option<TransactionReceipt<TypedReceipt<Receipt<Log>>>>, Self::Error> {
    //    todo!()
    //}

    async fn get_uncle_count_by_block_hash(
        &self,
        _parameter: EthGetUncleCountByBlockHash,
    ) -> Result<U256, Self::Error> {
        todo!()
    }

    async fn get_uncle_count_by_block_number(
        &self,
        _parameter: EthGetUncleCountByBlockNumber,
    ) -> Result<U256, Self::Error> {
        todo!()
    }

    async fn max_priority_fee_per_gas(&self) -> Result<U256, Self::Error> {
        let gas = self
            .evm_client()
            .get_max_priority_fee()
            .await
            .map_err(RPCError::from)?;
        Ok(U256::from(gas))
    }

    async fn send_raw_transaction(
        &self,
        _parameter: EthSendRawTransaction,
    ) -> Result<H256, Self::Error> {
        self.evm_client()
            .send_raw_transaction(_parameter.bytes.as_ref())
            .await
            .map_err(RPCError::from)
    }

    async fn send_transaction(&self, _parameter: EthSendTransaction) -> Result<B256, Self::Error> {
        todo!()
    }

    async fn sign(&self, _parameter: EthSign) -> Result<String, Self::Error> {
        todo!()
    }

    async fn sign_transaction(
        &self,
        _parameter: EthSignTransaction,
    ) -> Result<String, Self::Error> {
        todo!()
    }

    async fn syncing(&self) -> Result<bool, Self::Error> {
        todo!()
    }

    //async fn get_filter_changes(&self, _id: String) -> Result<FilterChanges, Self::Error> {
    //    todo!()
    //}

    //async fn get_filter_logs(&self, _id: String) -> Result<Vec<Log>, Self::Error> {
    //    todo!()
    //}

    //async fn get_logs(&self, _filter: Filter) -> Result<Vec<Log>, Self::Error> {
    //    todo!()
    //}

    async fn new_block_filter(&self) -> Result<String, Self::Error> {
        todo!()
    }

    //async fn new_filter(&self, _filter: Filter) -> Result<String, Self::Error> {
    //    todo!()
    //}

    async fn new_pending_transaction_filter(&self) -> Result<String, Self::Error> {
        todo!()
    }

    async fn uninstall_filter(&self, _id: String) -> Result<bool, Self::Error> {
        todo!()
    }
}
