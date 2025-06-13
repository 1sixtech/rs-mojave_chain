use std::str::FromStr;
use tendermint_rpc::{
    HttpClientUrl,
    client::{Client, CompatMode, HttpClient},
    endpoint::broadcast::tx_sync::Response,
};

pub struct AbciClient {
    client: HttpClient,
}

impl AbciClient {
    pub fn new(cometbft_rpc_url: impl AsRef<str>) -> Result<Self, AbciClientError> {
        let rpc_url = HttpClientUrl::from_str(cometbft_rpc_url.as_ref())?;

        let client = HttpClient::builder(rpc_url)
            .compat_mode(CompatMode::V0_38)
            .build()?;

        Ok(Self { client })
    }

    pub async fn broadcast_transaction(
        &self,
        transaction: Vec<u8>,
    ) -> Result<Response, AbciClientError> {
        self.client
            .broadcast_tx_sync(transaction)
            .await
            .map_err(|error| error.into())
    }
}

pub struct AbciClientError(tendermint_rpc::Error);

impl std::fmt::Debug for AbciClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for AbciClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for AbciClientError {}

impl From<tendermint_rpc::Error> for AbciClientError {
    fn from(value: tendermint_rpc::Error) -> Self {
        Self(value)
    }
}
