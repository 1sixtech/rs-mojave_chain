use crate::rpc::{
    FILTER_DURATION, RpcRequestWrapper,
    clients::mojave::Client as MojaveClient,
    utils::{RpcErr, RpcNamespace, RpcRequest, RpcRequestId, rpc_response},
};
use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use ethrex_blockchain::Blockchain;
use ethrex_common::Bytes;
use ethrex_p2p::{
    peer_handler::PeerHandler,
    sync_manager::SyncManager,
    types::{Node, NodeRecord},
};
use ethrex_rpc::{
    GasTipEstimator, NodeData, RpcApiContext as L1Context,
    types::transaction::SendRawTransactionRequest,
};
use ethrex_storage::Store;
use ethrex_storage_rollup::StoreRollup;
use serde_json::Value;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::{net::TcpListener, sync::Mutex as TokioMutex};
use tower_http::cors::CorsLayer;
use tracing::info;

mod transaction;

#[derive(Clone, Debug)]
pub struct RpcApiContextSequencer {
    pub l1_context: L1Context,
    pub rollup_store: StoreRollup,
    pub mojave_client: MojaveClient,
}

#[allow(async_fn_in_trait)]
pub trait RpcHandler: Sized {
    fn parse(params: &Option<Vec<Value>>) -> Result<Self, RpcErr>;

    async fn call(req: &RpcRequest, context: RpcApiContextSequencer) -> Result<Value, RpcErr> {
        let request = Self::parse(&req.params)?;
        request.handle(context).await
    }

    async fn handle(&self, context: RpcApiContextSequencer) -> Result<Value, RpcErr>;
}

#[expect(clippy::too_many_arguments)]
pub async fn start_api_sequencer(
    http_addr: SocketAddr,
    authrpc_addr: SocketAddr,
    storage: Store,
    blockchain: Arc<Blockchain>,
    jwt_secret: Bytes,
    local_p2p_node: Node,
    local_node_record: NodeRecord,
    syncer: SyncManager,
    peer_handler: PeerHandler,
    client_version: String,
    rollup_store: StoreRollup,
    mojave_client: MojaveClient,
) -> Result<(), RpcErr> {
    let active_filters = Arc::new(Mutex::new(HashMap::new()));
    let context = RpcApiContextSequencer {
        l1_context: L1Context {
            storage,
            blockchain,
            active_filters: active_filters.clone(),
            syncer: Arc::new(syncer),
            peer_handler,
            node_data: NodeData {
                jwt_secret,
                local_p2p_node,
                local_node_record,
                client_version,
            },
            gas_tip_estimator: Arc::new(TokioMutex::new(GasTipEstimator::new())),
        },
        rollup_store,
        mojave_client,
    };

    // Periodically clean up the active filters for the filters endpoints.
    tokio::task::spawn(async move {
        let mut interval = tokio::time::interval(FILTER_DURATION);
        let filters = active_filters.clone();
        loop {
            interval.tick().await;
            tracing::info!("Running filter clean task");
            ethrex_rpc::clean_outdated_filters(filters.clone(), FILTER_DURATION);
            tracing::info!("Filter clean task complete");
        }
    });

    // All request headers allowed.
    // All methods allowed.
    // All origins allowed.
    // All headers exposed.
    let cors = CorsLayer::permissive();

    let http_router = Router::new()
        .route("/", post(handle_http_request))
        .layer(cors)
        .with_state(context.clone());
    let http_listener = TcpListener::bind(http_addr)
        .await
        .map_err(|error| RpcErr::EthrexRPC(ethrex_rpc::RpcErr::Internal(error.to_string())))?;
    let http_server = axum::serve(http_listener, http_router)
        .with_graceful_shutdown(ethrex_rpc::shutdown_signal())
        .into_future();
    info!("Starting HTTP server at {http_addr}");

    info!("Not starting Auth-RPC server. The address passed as argument is {authrpc_addr}");

    let _ =
        tokio::try_join!(http_server).inspect_err(|e| info!("Error shutting down servers: {e:?}"));

    Ok(())
}

async fn handle_http_request(
    State(service_context): State<RpcApiContextSequencer>,
    body: String,
) -> Result<Json<Value>, StatusCode> {
    let res = match serde_json::from_str::<RpcRequestWrapper>(&body) {
        Ok(RpcRequestWrapper::Single(request)) => {
            let res = map_http_requests(&request, service_context).await;
            rpc_response(request.id, res).map_err(|_| StatusCode::BAD_REQUEST)?
        }
        Ok(RpcRequestWrapper::Multiple(requests)) => {
            let mut responses = Vec::new();
            for req in requests {
                let res = map_http_requests(&req, service_context.clone()).await;
                responses.push(rpc_response(req.id, res).map_err(|_| StatusCode::BAD_REQUEST)?);
            }
            serde_json::to_value(responses).map_err(|_| StatusCode::BAD_REQUEST)?
        }
        Err(_) => rpc_response(
            RpcRequestId::String("".to_string()),
            Err(ethrex_rpc::RpcErr::BadParams(
                "Invalid request body".to_string(),
            )),
        )
        .map_err(|_| StatusCode::BAD_REQUEST)?,
    };
    Ok(Json(res))
}

/// TODO: Export map_ns_requests in different branch.
async fn map_http_requests(
    req: &RpcRequest,
    context: RpcApiContextSequencer,
) -> Result<Value, RpcErr> {
    match req.namespace() {
        Ok(RpcNamespace::Eth) => map_eth_requests(req, context).await,
        Ok(RpcNamespace::Mojave) => map_mojave_requests(req, context).await,
        Ok(_other_namespaces) => Err(RpcErr::EthrexRPC(ethrex_rpc::RpcErr::Internal(
            "Unsuppored namespace".to_owned(),
        ))),
        // Ok(RpcNamespace::Admin) => map_admin_requests(req, context.l1_context),
        // Ok(RpcNamespace::Debug) => map_debug_requests(req, context.l1_context).await,
        // Ok(RpcNamespace::Web3) => map_web3_requests(req, context.l1_context),
        // Ok(RpcNamespace::Net) => map_net_requests(req, context.l1_context),
        // Ok(RpcNamespace::Mempool) => map_mempool_requests(req, context.l1_context).await,
        // Ok(RpcNamespace::Engine) => Err(RpcErr::Internal(
        //     "Engine namespace not allowed in map_http_requests".to_owned(),
        // )),
        Err(rpc_err) => Err(rpc_err),
    }
}

pub async fn map_mojave_requests(
    req: &RpcRequest,
    context: RpcApiContextSequencer,
) -> Result<Value, RpcErr> {
    match req.method.as_str() {
        "mojave_broadcastBlock" => unimplemented!(),
        "mojave_forwardTransaction" => unimplemented!(),
        _others => ethrex_rpc::map_eth_requests(&req.into(), context.l1_context)
            .await
            .map_err(RpcErr::EthrexRPC),
    }
}

pub async fn map_eth_requests(
    req: &RpcRequest,
    context: RpcApiContextSequencer,
) -> Result<Value, RpcErr> {
    match req.method.as_str() {
        "eth_sendRawTransaction" => SendRawTransactionRequest::call(req, context).await,
        _others => ethrex_rpc::map_eth_requests(&req.into(), context.l1_context)
            .await
            .map_err(RpcErr::EthrexRPC),
    }
}
