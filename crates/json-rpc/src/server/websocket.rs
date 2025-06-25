use crate::{api::eth_pubsub::EthPubSubApi, config::RpcConfig, error::RpcServerError};
use jsonrpsee::{
    RpcModule,
    server::{Server, ServerHandle},
};
use std::marker::PhantomData;

pub struct WebsocketServer<T: EthPubSubApi> {
    _backend: PhantomData<T>,
}

impl<T: EthPubSubApi> WebsocketServer<T> {
    pub async fn init(config: &RpcConfig, backend: T) -> Result<ServerHandle, RpcServerError> {
        let rpc_module = RpcModule::new(backend);
        // rpc_module.register_subscription(
        //     "eth_subscribe",
        //     "eth_subscription",
        //     "eth_unsubscribe",
        //     Self::subscribe,
        // )?;

        let server = Server::builder()
            .build(&config.websocket_address)
            .await
            .map_err(RpcServerError::Build)?;

        Ok(server.start(rpc_module))
    }

    // /// Handler for [EthPubSubApi::subscribe_new_heads]
    // async fn new_heads(pending: PendingSubscriptionSink, backend: Arc<T>) -> SubscriptionResult {
    //     let sink = pending.accept().await?;
    //     tokio::spawn(async move {
    //         let mut stream = backend.subscribe_new_heads().await;
    //         while let Some(header) = stream.next().await {
    //             match SubscriptionMessage::from_json(&header) {
    //                 Ok(message) => {
    //                     if sink.send(message).await.is_err() {
    //                         break;
    //                     }
    //                 }
    //                 Err(e) => {
    //                     tracing::error!(error = ?e, "Failed to deserialize header");
    //                     break;
    //                 }
    //             }
    //         }

    //         sink.closed().await;
    //     });
    //     Ok(())
    // }

    // /// Handler for [EthPubSubApi::subscribe_new_pending_transaction]
    // async fn new_pending_transactions(
    //     pending: PendingSubscriptionSink,
    //     backend: Arc<T>,
    // ) -> SubscriptionResult {
    //     let sink = pending.accept().await?;
    //     tokio::spawn(async move {
    //         let mut stream = backend.subscribe_new_pending_transaction().await;
    //         while let Some(new_pending_transaction) = stream.next().await {
    //             match SubscriptionMessage::from_json(&new_pending_transaction) {
    //                 Ok(message) => {
    //                     if sink.send(message).await.is_err() {
    //                         break;
    //                     }
    //                 }
    //                 Err(e) => {
    //                     tracing::error!(error = ?e, "Failed to deserialize new pending transaction");
    //                     break;
    //                 }
    //             }
    //         }

    //         sink.closed().await;
    //     });
    //     Ok(())
    // }
}

#[derive(Debug, thiserror::Error)]
pub enum SubscriptionError {
    #[error("Unsupported subscription kind")]
    Unsupported,
    #[error("Invalid parameter")]
    InvalidParameter,
}
