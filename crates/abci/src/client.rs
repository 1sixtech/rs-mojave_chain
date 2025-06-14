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

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct AbciClientError(#[from] tendermint_rpc::Error);
