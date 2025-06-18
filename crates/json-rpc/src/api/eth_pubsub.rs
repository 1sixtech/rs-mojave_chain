use futures::stream::Stream;
use mojave_chain_types::{
    alloy::primitives::B256,
    network::AnyHeader,
    rpc::{Filter, Header, Log},
};

#[trait_variant::make(EthPubSubApi: Send)]
pub trait LocalEthPubSubApi: Clone + Send + Sync + 'static {
    async fn subscribe_new_heads(&self) -> impl Stream<Item = Header<AnyHeader>> + Send + Unpin;

    async fn subscribe_logs(
        &self,
        filter: Option<Box<Filter>>,
    ) -> impl Stream<Item = Log> + Send + Unpin;

    async fn subscribe_new_pending_transaction(&self) -> impl Stream<Item = B256> + Send + Unpin;
}
