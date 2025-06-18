use crate::backend::{error::BackendError, Backend};
use mojave_chain_json_rpc::api::eth::EthApi;
use mojave_chain_types::{
    network::{AnyRpcBlock, AnyRpcTransaction},
    primitives::{Address, Bytes, B256, U256, U64},
    rpc::*,
};

impl EthApi for Backend {
    type Error = BackendError;

    /// Returns a list of addresses owned by client.
    async fn accounts(&self) -> Result<Vec<Address>, Self::Error> {
        self.evm_client().accounts().map_err(BackendError::EthApi)
    }

    /// Introduced in EIP-4844, returns the current blob base fee in wei.
    async fn blob_base_fee(&self) -> Result<U256, Self::Error> {
        self.evm_client()
            .blob_base_fee()
            .map_err(BackendError::EthApi)
    }

    /// Returns the number of most recent block.
    async fn block_number(&self) -> Result<U256, Self::Error> {
        self.evm_client()
            .block_number()
            .map_err(BackendError::EthApi)
    }

    /// Executes a new message call immediately without creating a transaction on the block chain.
    async fn call(&self, parameter: EthCall) -> Result<Bytes, Self::Error> {
        self.evm_client()
            .call(parameter.request, parameter.block_number, None)
            .await
            .map_err(BackendError::EthApi)
    }

    /// Returns the chain ID of the current network.
    async fn chain_id(&self) -> Result<Option<U64>, Self::Error> {
        Ok(Some(U64::from(self.evm_client().chain_id())))
    }

    /// Returns the client coinbase address.
    async fn coinbase(&self) -> Result<Address, Self::Error> {
        Err(BackendError::Unimplemented)
    }

    /// Generates an access list for a transaction.
    ///
    /// This method creates an [EIP2930](https://eips.ethereum.org/EIPS/eip-2930) type accessList based on a given Transaction.
    ///
    /// An access list contains all storage slots and addresses touched by the transaction, except
    /// for the sender account and the chain's precompiles.
    ///
    /// It returns list of addresses and storage keys used by the transaction, plus the gas
    /// consumed when the access list is added. That is, it gives you the list of addresses and
    /// storage keys that will be used by that transaction, plus the gas consumed if the access
    /// list is included. Like eth_estimateGas, this is an estimation; the list could change
    /// when the transaction is actually mined. Adding an accessList to your transaction does
    /// not necessary result in lower gas usage compared to a transaction without an access
    /// list.
    async fn create_access_list(
        &self,
        parameter: EthCreateAccessList,
    ) -> Result<AccessListResult, Self::Error> {
        self.evm_client()
            .create_access_list(parameter.request, parameter.block_number)
            .await
            .map_err(BackendError::EthApi)
    }

    /// Generates and returns an estimate of how much gas is necessary to allow the transaction to
    /// complete.
    async fn estimate_gas(&self, parameter: EthEstimateGas) -> Result<U256, Self::Error> {
        self.evm_client()
            .estimate_gas(parameter.request, None, None)
            .await
            .map_err(BackendError::EthApi)
    }

    /// Returns the Transaction fee history
    ///
    /// Introduced in EIP-1559 for getting information on the appropriate priority fee to use.
    ///
    /// Returns transaction base fee per gas and effective priority fee per gas for the
    /// requested/supported block range. The returned Fee history for the returned block range
    /// can be a subsection of the requested range if not all blocks are available.
    async fn fee_history(&self, parameter: EthFeeHistory) -> Result<FeeHistory, Self::Error> {
        self.evm_client()
            .fee_history(
                U256::from(parameter.block_count),
                parameter.newest_block,
                parameter.reward_percentiles.unwrap_or_default(),
            )
            .await
            .map_err(BackendError::EthApi)
    }

    /// Returns the current price per gas in wei.
    async fn gas_price(&self) -> Result<U256, Self::Error> {
        let gas_price = self.evm_client().gas_price();
        Ok(U256::from(gas_price))
    }

    /// Returns the balance of the account of given address.
    async fn get_balance(&self, parameter: EthGetBalance) -> Result<U256, Self::Error> {
        self.evm_client()
            .balance(parameter.address, parameter.block_number)
            .await
            .map_err(BackendError::EthApi)
    }

    /// Returns information about a block by hash.
    async fn get_block_by_hash(
        &self,
        parameter: EthGetBlockByHash,
    ) -> Result<Option<AnyRpcBlock>, Self::Error> {
        self.evm_client()
            .block_by_hash(parameter.hash)
            .await
            .map_err(BackendError::EthApi)
    }

    /// Returns information about a block by number.
    async fn get_block_by_number(
        &self,
        parameter: EthGetBlockByNumber,
    ) -> Result<Option<AnyRpcBlock>, Self::Error> {
        self.evm_client()
            .block_by_number(parameter.number)
            .await
            .map_err(BackendError::EthApi)
    }

    /// Returns all transaction receipts for a given block.
    async fn get_block_receipts(
        &self,
        parameter: EthBlockReceipts,
    ) -> Result<Option<Vec<TransactionReceipt<TypedReceipt<Receipt<Log>>>>>, Self::Error> {
        self.evm_client()
            .block_receipts(parameter.block_id)
            .await
            .map_err(BackendError::EthApi)
    }

    /// Returns the number of transactions in a block from a block matching the given block hash.
    async fn get_block_transaction_count_by_hash(
        &self,
        parameter: EthGetBlockTransactionCountByHash,
    ) -> Result<Option<U256>, Self::Error> {
        self.evm_client()
            .block_transaction_count_by_hash(parameter.hash)
            .await
            .map_err(BackendError::EthApi)
    }

    /// Returns the number of transactions in a block matching the given block number.
    async fn get_block_transaction_count_by_number(
        &self,
        parameter: EthGetBlockTransactionCountByNumber,
    ) -> Result<Option<U256>, Self::Error> {
        self.evm_client()
            .block_transaction_count_by_number(parameter.number)
            .await
            .map_err(BackendError::EthApi)
    }

    /// Returns code at a given address at given block number.
    async fn get_code(&self, parameter: EthGetCode) -> Result<Bytes, Self::Error> {
        self.evm_client()
            .get_code(parameter.address, parameter.block_number)
            .await
            .map_err(BackendError::EthApi)
    }

    /// Returns the account and storage values of the specified account including the Merkle-proof.
    /// This call can be used to verify that the data you are pulling from is not tampered with.
    async fn get_proof(
        &self,
        parameter: EthGetProof,
    ) -> Result<EIP1186AccountProofResponse, Self::Error> {
        self.evm_client()
            .get_proof(parameter.address, parameter.keys, parameter.block_number)
            .await
            .map_err(BackendError::EthApi)
    }

    /// Returns the value from a storage position at a given address
    async fn get_storage_at(&self, parameter: EthGetStorageAt) -> Result<B256, Self::Error> {
        self.evm_client()
            .storage_at(parameter.address, parameter.index, parameter.block_number)
            .await
            .map_err(BackendError::EthApi)
    }

    /// Returns information about a transaction by block hash and transaction index position.
    async fn get_transaction_by_block_hash_and_index(
        &self,
        parameter: EthGetTransactionByBlockHashAndIndex,
    ) -> Result<Option<AnyRpcTransaction>, Self::Error> {
        self.evm_client()
            .transaction_by_block_hash_and_index(parameter.hash, parameter.index)
            .await
            .map_err(BackendError::EthApi)
    }

    /// Returns information about a transaction by block number and transaction index position.
    async fn get_transaction_by_block_number_and_index(
        &self,
        parameter: EthGetTransactionByBlockNumberAndIndex,
    ) -> Result<Option<AnyRpcTransaction>, Self::Error> {
        self.evm_client()
            .transaction_by_block_number_and_index(parameter.number, parameter.index)
            .await
            .map_err(BackendError::EthApi)
    }

    /// Returns the information about a transaction requested by transaction hash.
    async fn get_transaction_by_hash(
        &self,
        parameter: EthgetTransactionByHash,
    ) -> Result<Option<AnyRpcTransaction>, Self::Error> {
        self.evm_client()
            .transaction_by_hash(parameter.hash)
            .await
            .map_err(BackendError::EthApi)
    }

    /// Returns the number of transactions sent from an address at given block number.
    async fn get_transaction_count(
        &self,
        parameter: EthGetTransactionCount,
    ) -> Result<U256, Self::Error> {
        self.evm_client()
            .transaction_count(parameter.address, parameter.block_number)
            .await
            .map_err(BackendError::EthApi)
    }

    /// Returns the receipt of a transaction by transaction hash.
    async fn get_transaction_receipt(
        &self,
        parameter: EthGetTransactionReceipt,
    ) -> Result<Option<TransactionReceipt<TypedReceipt<Receipt<Log>>>>, Self::Error> {
        self.evm_client()
            .transaction_receipt(parameter.hash)
            .await
            .map_err(BackendError::EthApi)
    }

    /// Returns the number of uncles in a block from a block matching the given block hash.
    async fn get_uncle_count_by_block_hash(
        &self,
        parameter: EthGetUncleCountByBlockHash,
    ) -> Result<U256, Self::Error> {
        self.evm_client()
            .block_uncles_count_by_hash(parameter.hash)
            .await
            .map_err(BackendError::EthApi)
    }

    /// Returns the number of uncles in a block with given block number.
    async fn get_uncle_count_by_block_number(
        &self,
        parameter: EthGetUncleCountByBlockNumber,
    ) -> Result<U256, Self::Error> {
        self.evm_client()
            .block_uncles_count_by_number(parameter.number)
            .await
            .map_err(BackendError::EthApi)
    }

    /// Introduced in EIP-1559, returns suggestion for the priority for dynamic fee transactions.
    async fn max_priority_fee_per_gas(&self) -> Result<U256, Self::Error> {
        self.evm_client()
            .max_priority_fee_per_gas()
            .map_err(BackendError::EthApi)
    }

    /// Sends signed transaction, returning its hash.
    async fn send_raw_transaction(
        &self,
        parameter: EthSendRawTransaction,
    ) -> Result<B256, Self::Error> {
        // // Broacast the transaction.
        // let response = self
        //     .abci_client()
        //     .broadcast_transaction(parameter.bytes.to_vec())
        //     .await
        //     .map_err(BackendError::Broadcast)?;
        // match response.code {
        //     Code::Ok => {
        //         self.pubsub_service()
        //             .publish_pending_transaction(transaction_hash);
        //         Ok(transaction_hash)
        //     }
        //     Code::Err(_) => Err(BackendError::CheckTx(response.log)),
        // }
        let transaction_hash = self
            .evm_client()
            .send_raw_transaction(parameter.bytes.clone())
            .await
            .map_err(BackendError::EthApi)?;

        self.pubsub_service()
            .publish_pending_transaction(transaction_hash);

        Ok(transaction_hash)
    }

    /// Signs transaction with a matching signer, if any and submits the transaction to the pool.
    /// Returns the hash of the signed transaction.
    async fn send_transaction(&self, parameter: EthSendTransaction) -> Result<B256, Self::Error> {
        // // Broacast the transaction.
        // let response = self
        //     .abci_client()
        //     .broadcast_transaction(serde_json::to_vec(&parameter.transaction).unwrap())
        //     .await
        //     .map_err(BackendError::Broadcast)?;
        // match response.code {
        //     Code::Ok => {
        //         self.pubsub_service()
        //             .publish_pending_transaction(transaction_hash);
        //         Ok(transaction_hash)
        //     }
        //     Code::Err(_) => Err(BackendError::CheckTx(response.log)),
        // }
        let transaction_hash = self
            .evm_client()
            .send_transaction(parameter.transaction.clone())
            .await
            .map_err(BackendError::EthApi)?;

        self.pubsub_service()
            .publish_pending_transaction(transaction_hash);

        Ok(transaction_hash)
    }

    /// Returns an Ethereum specific signature with: sign(keccak256("\x19Ethereum Signed Message:\n"
    /// + len(message) + message))).
    async fn sign(&self, parameter: EthSign) -> Result<String, Self::Error> {
        self.evm_client()
            .sign(parameter.address, parameter.message)
            .await
            .map_err(BackendError::EthApi)
    }

    /// Signs a transaction that can be submitted to the network at a later time using with
    /// `sendRawTransaction.`
    async fn sign_transaction(&self, parameter: EthSignTransaction) -> Result<String, Self::Error> {
        self.evm_client()
            .sign_transaction(parameter.transaction)
            .await
            .map_err(BackendError::EthApi)
    }

    /// Returns an object with data about the sync status or false.
    async fn syncing(&self) -> Result<bool, Self::Error> {
        self.evm_client().syncing().map_err(BackendError::EthApi)
    }

    /// Returns all filter changes since last poll.
    async fn get_filter_changes(&self, id: String) -> Result<FilterChanges, Self::Error> {
        match self.evm_client().get_filter_changes(&id).await {
            ResponseResult::Success(value) => {
                let response =
                    serde_json::from_value(value).map_err(|_| BackendError::EthFilterResponse)?;
                Ok(response)
            }
            ResponseResult::Error(error) => Err(BackendError::EthFilter(error.message.to_string())),
        }
    }

    /// Returns all logs matching given filter (in a range 'from' - 'to').
    async fn get_filter_logs(&self, id: String) -> Result<Vec<Log>, Self::Error> {
        self.evm_client()
            .get_filter_logs(&id)
            .await
            .map_err(BackendError::EthApi)
    }

    /// Returns logs matching given filter object.
    async fn get_logs(&self, filter: Filter) -> Result<Vec<Log>, Self::Error> {
        self.evm_client()
            .logs(filter)
            .await
            .map_err(BackendError::EthApi)
    }

    /// Creates a new block filter and returns its id.
    async fn new_block_filter(&self) -> Result<String, Self::Error> {
        self.evm_client()
            .new_block_filter()
            .await
            .map_err(BackendError::EthApi)
    }

    /// Creates a new filter and returns its id.
    async fn new_filter(&self, filter: Filter) -> Result<String, Self::Error> {
        self.evm_client()
            .new_filter(filter)
            .await
            .map_err(BackendError::EthApi)
    }

    /// Creates a pending transaction filter and returns its id.
    async fn new_pending_transaction_filter(&self) -> Result<String, Self::Error> {
        self.evm_client()
            .new_pending_transaction_filter()
            .await
            .map_err(BackendError::EthApi)
    }

    /// Uninstalls the filter.
    async fn uninstall_filter(&self, id: String) -> Result<bool, Self::Error> {
        self.evm_client()
            .uninstall_filter(&id)
            .await
            .map_err(BackendError::EthApi)
    }
}
