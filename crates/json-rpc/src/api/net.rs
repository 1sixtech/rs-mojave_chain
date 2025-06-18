use mojave_chain_types::alloy::primitives::U64;

#[trait_variant::make(NetApi: Send)]
pub trait LocalNetApi: Clone + Send + Sync + 'static {
    type Error: std::error::Error + Send + 'static;

    async fn version(&self) -> Result<String, Self::Error>;

    async fn peer_count(&self) -> Result<U64, Self::Error>;

    async fn listening(&self) -> Result<bool, Self::Error>;
}
