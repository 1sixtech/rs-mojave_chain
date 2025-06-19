use mojave_chain_types::common::Bytes;

#[trait_variant::make(Web3Api: Send)]
pub trait LocalWeb3Api: Clone + Send + Sync + 'static {
    type Error: std::error::Error + Send + 'static;

    async fn client_version(&self) -> Result<String, Self::Error>;

    async fn sha3(&self, bytes: Bytes) -> Result<String, Self::Error>;
}
