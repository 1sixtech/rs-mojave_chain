use crate::{
    api::{eth::EthApi, net::NetApi, web3::Web3Api},
    config::RpcConfig,
    error::RpcServerError,
};
use hyper::Method;
use jsonrpsee::{
    Extensions, RpcModule,
    core::RpcResult,
    server::{Server, ServerHandle},
    types::{ErrorCode, ErrorObjectOwned, Params},
};
use mojave_chain_types::{
    alloy::primitives::{Address, B256, Bytes, U64, U256},
    network::{AnyRpcBlock, AnyRpcTransaction},
    rpc::*,
};
use std::{marker::PhantomData, sync::Arc};
use tower_http::cors::{Any, CorsLayer};

pub struct HttpServer<T>
where
    T: Web3Api + NetApi + EthApi,
{
    _backend: PhantomData<T>,
}

impl<T> HttpServer<T>
where
    T: Web3Api + NetApi + EthApi,
{
    pub async fn init(config: &RpcConfig, backend: T) -> Result<ServerHandle, RpcServerError> {
        let mut rpc_module = RpcModule::new(backend);
        Self::register_eth_api(&mut rpc_module)?;
        Self::register_net_api(&mut rpc_module)?;
        Self::register_web3_api(&mut rpc_module)?;

        let cors = CorsLayer::new()
            .allow_methods([Method::POST])
            .allow_origin(Any)
            .allow_headers([hyper::header::CONTENT_TYPE]);
        let cors_middleware = tower::ServiceBuilder::new().layer(cors);

        let server = Server::builder()
            .set_http_middleware(cors_middleware)
            .build(&config.rpc_address)
            .await
            .map_err(RpcServerError::Build)?;

        Ok(server.start(rpc_module))
    }
}

/// Web3Api implementations
impl<T> HttpServer<T>
where
    T: Web3Api + NetApi + EthApi,
{
    fn register_web3_api(rpc_module: &mut RpcModule<T>) -> Result<(), RpcServerError> {
        rpc_module.register_async_method("web3_clientVersion", Self::client_version)?;
        rpc_module.register_async_method("web3_sha3", Self::sha3)?;
        Ok(())
    }

    async fn client_version(
        _parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<String> {
        backend.client_version().await.into_rpc_result()
    }

    async fn sha3(
        parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<String> {
        let parameter = parameter.parse::<Bytes>()?;
        backend.sha3(parameter).await.into_rpc_result()
    }
}

/// NetApi implementations
impl<T> HttpServer<T>
where
    T: Web3Api + NetApi + EthApi,
{
    fn register_net_api(rpc_module: &mut RpcModule<T>) -> Result<(), RpcServerError> {
        rpc_module.register_async_method("net_version", Self::version)?;
        rpc_module.register_async_method("net_peerCount", Self::peer_count)?;
        rpc_module.register_async_method("net_listening", Self::listening)?;
        Ok(())
    }

    async fn version(
        _parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<String> {
        backend.version().await.into_rpc_result()
    }

    async fn peer_count(
        _parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<U64> {
        backend.peer_count().await.into_rpc_result()
    }

    async fn listening(
        _parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<bool> {
        backend.listening().await.into_rpc_result()
    }
}

/// EthApi implementations
impl<T> HttpServer<T>
where
    T: Web3Api + NetApi + EthApi,
{
    fn register_eth_api(rpc_module: &mut RpcModule<T>) -> Result<(), RpcServerError> {
        rpc_module.register_async_method("eth_accounts", Self::accounts)?;
        rpc_module.register_async_method("eth_blobBaseFee", Self::blob_base_fee)?;
        rpc_module.register_async_method("eth_blockNumber", Self::block_number)?;
        rpc_module.register_async_method("eth_call", Self::call)?;
        rpc_module.register_async_method("eth_chainId", Self::chain_id)?;
        rpc_module.register_async_method("eth_coinbase", Self::coinbase)?;
        rpc_module.register_async_method("eth_createAccessList", Self::create_access_list)?;
        rpc_module.register_async_method("eth_estimateGas", Self::estimate_gas)?;
        rpc_module.register_async_method("eth_feeHistory", Self::fee_history)?;
        rpc_module.register_async_method("eth_gasPrice", Self::gas_price)?;
        rpc_module.register_async_method("eth_getBalance", Self::get_balance)?;
        rpc_module.register_async_method("eth_getBlockByHash", Self::get_block_by_hash)?;
        rpc_module.register_async_method("eth_getBlockByNumber", Self::get_block_by_number)?;
        rpc_module.register_async_method("eth_getBlockReceipts", Self::get_block_receipts)?;
        rpc_module.register_async_method(
            "eth_getBlockTransactionCountByHash",
            Self::get_block_transaction_count_by_hash,
        )?;
        rpc_module.register_async_method(
            "eth_getBlockTransactionCountByNumber",
            Self::get_block_transaction_count_by_number,
        )?;
        rpc_module.register_async_method("eth_getCode", Self::get_code)?;
        rpc_module.register_async_method("eth_getProof", Self::get_proof)?;
        rpc_module.register_async_method("eth_getStorageAt", Self::get_storage_at)?;
        rpc_module.register_async_method(
            "eth_getTransactionByBlockHashAndIndex",
            Self::get_transaction_by_block_hash_and_index,
        )?;
        rpc_module.register_async_method(
            "eth_getTransactionByBlockNumberAndIndex",
            Self::get_transaction_by_block_number_and_index,
        )?;
        rpc_module
            .register_async_method("eth_getTransactionByHash", Self::get_transaction_by_hash)?;
        rpc_module.register_async_method("eth_getTransactionCount", Self::get_transaction_count)?;
        rpc_module
            .register_async_method("eth_getTransactionReceipt", Self::get_transaction_receipt)?;
        rpc_module.register_async_method(
            "eth_getUncleCountByBlockHash",
            Self::get_uncle_count_by_block_hash,
        )?;
        rpc_module.register_async_method(
            "eth_getUncleCountByBlockNumber",
            Self::get_uncle_count_by_block_number,
        )?;
        rpc_module
            .register_async_method("eth_maxPriorityFeePerGas", Self::max_priority_fee_per_gas)?;
        rpc_module.register_async_method("eth_sendRawTransaction", Self::send_raw_transaction)?;
        rpc_module.register_async_method("eth_sendTransaction", Self::send_transaction)?;
        rpc_module.register_async_method("eth_sign", Self::sign)?;
        rpc_module.register_async_method("eth_signTransaction", Self::sign_transaction)?;
        rpc_module.register_async_method("eth_syncing", Self::syncing)?;
        rpc_module.register_async_method("eth_getFilterChanges", Self::get_filter_changes)?;
        rpc_module.register_async_method("eth_getFilterLogs", Self::get_filter_logs)?;
        rpc_module.register_async_method("eth_getLogs", Self::get_logs)?;
        rpc_module.register_async_method("eth_newBlockFilter", Self::new_block_filter)?;
        rpc_module.register_async_method("eth_newFilter", Self::new_filter)?;
        rpc_module.register_async_method(
            "eth_newPendingTransactionFilter",
            Self::new_pending_transaction_filter,
        )?;
        rpc_module.register_async_method("eth_uninstallFilter", Self::uninstall_filter)?;
        Ok(())
    }

    /// Handler for [EthApi::accounts]
    async fn accounts(
        _parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<Vec<Address>> {
        backend.accounts().await.into_rpc_result()
    }

    /// Handler for [EthApi::blob_base_fee]
    async fn blob_base_fee(
        _parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<U256> {
        backend.blob_base_fee().await.into_rpc_result()
    }

    /// Handler for [EthApi::block_number]
    async fn block_number(
        _parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<U256> {
        backend.block_number().await.into_rpc_result()
    }

    /// Handler for [EthApi::call]
    async fn call(
        parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<Bytes> {
        let parameter = parameter.parse::<EthCall>()?;
        backend.call(parameter).await.into_rpc_result()
    }

    /// Handler for [EthApi::chain_id]
    async fn chain_id(
        _parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<Option<U64>> {
        backend.chain_id().await.into_rpc_result()
    }

    /// Handler for [EthApi::coinbase]
    async fn coinbase(
        _parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<Address> {
        backend.coinbase().await.into_rpc_result()
    }

    /// Handler for [EthApi::create_access_list]
    async fn create_access_list(
        parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<AccessListResult> {
        let parameter = parameter.parse::<EthCreateAccessList>()?;
        backend
            .create_access_list(parameter)
            .await
            .into_rpc_result()
    }

    /// Handler for [EthApi::estimate_gas]
    async fn estimate_gas(
        parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<U256> {
        let parameter = parameter.parse::<EthEstimateGas>()?;
        backend.estimate_gas(parameter).await.into_rpc_result()
    }

    /// Handler for [EthApi::fee_history]
    async fn fee_history(
        parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<FeeHistory> {
        let parameter = parameter.parse::<EthFeeHistory>()?;
        backend.fee_history(parameter).await.into_rpc_result()
    }

    /// Handler for [EthApi::gas_price]
    async fn gas_price(
        _parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<U256> {
        backend.gas_price().await.into_rpc_result()
    }

    /// Handler for [EthApi::get_balance]
    async fn get_balance(
        parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<U256> {
        let parameter = parameter.parse::<EthGetBalance>()?;
        backend.get_balance(parameter).await.into_rpc_result()
    }

    /// Handler for [EthApi::get_block_by_hash]
    async fn get_block_by_hash(
        parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<Option<AnyRpcBlock>> {
        let parameter = parameter.parse()?;
        backend.get_block_by_hash(parameter).await.into_rpc_result()
    }

    /// Handler for [EthApi::get_block_by_number]
    async fn get_block_by_number(
        parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<Option<AnyRpcBlock>> {
        let parameter = parameter.parse()?;
        backend
            .get_block_by_number(parameter)
            .await
            .into_rpc_result()
    }

    /// Handler for [EthApi::get_block_receipts]
    async fn get_block_receipts(
        parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<Option<Vec<TransactionReceipt<TypedReceipt<Receipt<Log>>>>>> {
        let parameter = parameter.parse::<EthBlockReceipts>()?;
        backend
            .get_block_receipts(parameter)
            .await
            .into_rpc_result()
    }

    /// Handler for [EthApi::get_block_transaction_count_by_hash]
    async fn get_block_transaction_count_by_hash(
        parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<Option<U256>> {
        let parameter = parameter.parse::<EthGetBlockTransactionCountByHash>()?;
        backend
            .get_block_transaction_count_by_hash(parameter)
            .await
            .into_rpc_result()
    }

    /// Handler for [EthApi::get_block_transaction_count_by_number]
    async fn get_block_transaction_count_by_number(
        parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<Option<U256>> {
        let parameter = parameter.parse::<EthGetBlockTransactionCountByNumber>()?;
        backend
            .get_block_transaction_count_by_number(parameter)
            .await
            .into_rpc_result()
    }

    /// Handler for [EthApi::get_code]
    async fn get_code(
        parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<Bytes> {
        let parameter = parameter.parse::<EthGetCode>()?;
        backend.get_code(parameter).await.into_rpc_result()
    }

    /// Handler for [EthApi::get_proof]
    async fn get_proof(
        parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<EIP1186AccountProofResponse> {
        let parameter = parameter.parse::<EthGetProof>()?;
        backend.get_proof(parameter).await.into_rpc_result()
    }

    /// Handler for [EthApi::get_storage_at]
    async fn get_storage_at(
        parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<B256> {
        let parameter = parameter.parse::<EthGetStorageAt>()?;
        backend.get_storage_at(parameter).await.into_rpc_result()
    }

    /// Handler for [EthApi::get_transaction_by_block_hash_and_index]
    async fn get_transaction_by_block_hash_and_index(
        parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<Option<AnyRpcTransaction>> {
        let parameter = parameter.parse::<EthGetTransactionByBlockHashAndIndex>()?;
        backend
            .get_transaction_by_block_hash_and_index(parameter)
            .await
            .into_rpc_result()
    }

    /// Handler for [EthApi::get_transaction_by_block_number_and_index]
    async fn get_transaction_by_block_number_and_index(
        parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<Option<AnyRpcTransaction>> {
        let parameter = parameter.parse::<EthGetTransactionByBlockNumberAndIndex>()?;
        backend
            .get_transaction_by_block_number_and_index(parameter)
            .await
            .into_rpc_result()
    }

    /// Handler for [EthApi::get_transaction_by_hash]
    async fn get_transaction_by_hash(
        parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<Option<AnyRpcTransaction>> {
        let parameter = parameter.parse::<EthgetTransactionByHash>()?;
        backend
            .get_transaction_by_hash(parameter)
            .await
            .into_rpc_result()
    }

    /// Handler for [EthApi::get_transaction_count]
    async fn get_transaction_count(
        parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<U256> {
        let parameter = parameter.parse::<EthGetTransactionCount>()?;
        backend
            .get_transaction_count(parameter)
            .await
            .into_rpc_result()
    }

    /// Handler for [EthApi::get_transaction_receipt]
    async fn get_transaction_receipt(
        parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<Option<TransactionReceipt<TypedReceipt<Receipt<Log>>>>> {
        let parameter = parameter.parse::<EthGetTransactionReceipt>()?;
        backend
            .get_transaction_receipt(parameter)
            .await
            .into_rpc_result()
    }

    /// Handler for [EthApi::get_uncle_count_by_block_hash]
    async fn get_uncle_count_by_block_hash(
        parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<U256> {
        let parameter = parameter.parse::<EthGetUncleCountByBlockHash>()?;
        backend
            .get_uncle_count_by_block_hash(parameter)
            .await
            .into_rpc_result()
    }

    /// Handler for [EthApi::get_uncle_count_by_block_number]
    async fn get_uncle_count_by_block_number(
        parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<U256> {
        let parameter = parameter.parse::<EthGetUncleCountByBlockNumber>()?;
        backend
            .get_uncle_count_by_block_number(parameter)
            .await
            .into_rpc_result()
    }

    /// Handler for [EthApi::max_priority_fee_per_gas]
    async fn max_priority_fee_per_gas(
        _parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<U256> {
        backend.max_priority_fee_per_gas().await.into_rpc_result()
    }

    /// Handler for [EthApi::send_raw_transaction]
    async fn send_raw_transaction(
        parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<B256> {
        let parameter = parameter.parse::<EthSendRawTransaction>()?;
        backend
            .send_raw_transaction(parameter)
            .await
            .into_rpc_result()
    }

    /// Handler for [EthApi::send_transaction]
    async fn send_transaction(
        parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<B256> {
        let parameter = parameter.parse::<EthSendTransaction>()?;
        backend.send_transaction(parameter).await.into_rpc_result()
    }

    /// Handler for [EthApi::sign]
    async fn sign(
        parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<String> {
        let parameter = parameter.parse::<EthSign>()?;
        backend.sign(parameter).await.into_rpc_result()
    }

    /// Handler for [EthApi::sign_transaction]
    async fn sign_transaction(
        parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<String> {
        let parameter = parameter.parse::<EthSignTransaction>()?;
        backend.sign_transaction(parameter).await.into_rpc_result()
    }

    /// Handler for [EthApi::syncing]
    async fn syncing(
        _parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<bool> {
        backend.syncing().await.into_rpc_result()
    }

    async fn get_filter_changes(
        parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<FilterChanges> {
        let parameter = parameter.parse::<String>()?;
        backend
            .get_filter_changes(parameter)
            .await
            .into_rpc_result()
    }

    async fn get_filter_logs(
        parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<Vec<Log>> {
        let parameter = parameter.parse::<String>()?;
        backend.get_filter_logs(parameter).await.into_rpc_result()
    }

    async fn get_logs(
        parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<Vec<Log>> {
        let parameter = parameter.parse::<Filter>()?;
        backend.get_logs(parameter).await.into_rpc_result()
    }

    async fn new_block_filter(
        _parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<String> {
        backend.new_block_filter().await.into_rpc_result()
    }

    async fn new_filter(
        parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<String> {
        let parameter = parameter.parse::<Filter>()?;
        backend.new_filter(parameter).await.into_rpc_result()
    }

    async fn new_pending_transaction_filter(
        _parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<String> {
        backend
            .new_pending_transaction_filter()
            .await
            .into_rpc_result()
    }

    async fn uninstall_filter(
        parameter: Params<'static>,
        backend: Arc<T>,
        _extension: Extensions,
    ) -> RpcResult<bool> {
        let parameter = parameter.parse::<String>()?;
        backend.uninstall_filter(parameter).await.into_rpc_result()
    }
}

/// Helper trait to convert [std::result::Result<T, E>] to [RpcResult<T>].
pub(crate) trait IntoRpcResult<T> {
    fn into_rpc_result(self) -> RpcResult<T>;
}

impl<T, E> IntoRpcResult<T> for Result<T, E>
where
    E: std::error::Error + Send + 'static,
{
    fn into_rpc_result(self) -> RpcResult<T> {
        match self {
            Ok(value) => Ok(value),
            Err(error) => Err(ErrorObjectOwned::owned::<String>(
                ErrorCode::InternalError.code(),
                ErrorCode::InternalError.message(),
                Some(error.to_string()),
            )),
        }
    }
}
