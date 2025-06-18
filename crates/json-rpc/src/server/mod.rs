pub mod http;
pub mod websocket;

use crate::{
    api::{eth::EthApi, eth_pubsub::EthPubSubApi, net::NetApi, web3::Web3Api},
    config::RpcConfig,
    error::RpcServerError,
};
use http::HttpServer;
use jsonrpsee::server::ServerHandle;
use std::{
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};
use websocket::WebsocketServer;

pub struct RpcServer<T>
where
    T: Web3Api + NetApi + EthApi + EthPubSubApi,
{
    http_context: PhantomData<T>,
}

impl<T> RpcServer<T>
where
    T: Web3Api + NetApi + EthApi + EthPubSubApi,
{
    pub async fn init(
        config: &RpcConfig,
        backend: T,
        shutdown_signal: Option<Pin<Box<dyn Future<Output = ()> + Send>>>,
    ) -> Result<RpcServerHandle, RpcServerError> {
        let http_server_handle = HttpServer::init(config, backend.clone()).await?;
        let websocket_server_handle = WebsocketServer::init(config, backend).await?;
        let shutdown_signal = shutdown_signal.unwrap_or_else(|| {
            Box::pin(async {
                tokio::signal::ctrl_c()
                    .await
                    .expect("failed to add Ctrl-C signal");
            })
        });
        Ok(RpcServerHandle {
            http_server_handle,
            websocket_server_handle,
            shutdown_signal,
        })
    }
}

pub struct RpcServerHandle {
    http_server_handle: ServerHandle,
    websocket_server_handle: ServerHandle,
    shutdown_signal: Pin<Box<dyn Future<Output = ()> + Send>>,
}

impl Future for RpcServerHandle {
    type Output = RpcServerError;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        if this.http_server_handle.is_stopped() {
            tracing::info!("RPC server: HTTP server stopped");
            let _ = this.websocket_server_handle.stop();
            tracing::info!("RPC server: HTTP server stopped");
            return Poll::Ready(RpcServerError::RpcServerStopped);
        }

        if this.websocket_server_handle.is_stopped() {
            tracing::info!("RPC server: Websocket server stopped");
            let _ = this.http_server_handle.stop();
            tracing::info!("RPC server: HTTP server stopped");
            return Poll::Ready(RpcServerError::WebsocketServerStopped);
        }

        if this.shutdown_signal.as_mut().poll(cx).is_ready() {
            tracing::info!("RPC server shutting down...");
            let _ = this.http_server_handle.stop();
            let _ = this.websocket_server_handle.stop();
            tracing::info!("RPC server shut down!");
            return Poll::Ready(RpcServerError::ShutdownSignalReceived);
        }

        Poll::Pending
    }
}
