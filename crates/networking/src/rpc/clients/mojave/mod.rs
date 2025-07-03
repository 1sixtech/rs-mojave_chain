use ethrex_common::{H256, types::Block};
use reqwest::Url;
use serde::Deserialize;
use serde_json::json;

use crate::rpc::{
    clients::mojave::errors::MojaveClientError,
    utils::{RpcErrorResponse, RpcRequest, RpcRequestId, RpcSuccessResponse},
};

pub mod errors;

#[derive(Clone, Debug)]
pub struct Client {
    client: reqwest::Client,
    pub urls: Vec<Url>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum RpcResponse {
    Success(RpcSuccessResponse),
    Error(RpcErrorResponse),
}

impl Client {
    pub fn new(urls: Vec<&str>) -> Result<Self, MojaveClientError> {
        let urls = urls
            .iter()
            .map(|url| {
                Url::parse(url).map_err(|_| {
                    MojaveClientError::ParseUrlError("Failed to parse urls".to_string())
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            client: reqwest::Client::new(),
            urls,
        })
    }

    async fn send_request(&self, request: RpcRequest) -> Result<RpcResponse, MojaveClientError> {
        let mut response = Err(MojaveClientError::Custom(
            "All rpc calls failed".to_string(),
        ));

        for url in self.urls.iter() {
            let maybe_response = self.send_request_to_url(url, &request).await;
            if maybe_response.is_ok() {
                response = maybe_response;
            }
        }
        response
    }

    async fn send_request_to_url(
        &self,
        url: &Url,
        request: &RpcRequest,
    ) -> Result<RpcResponse, MojaveClientError> {
        self.client
            .post(url.as_str())
            .header("content-type", "application/json")
            .body(serde_json::ser::to_string(&request).map_err(|error| {
                MojaveClientError::FailedToSerializeRequestBody(format!("{error}: {request:?}"))
            })?)
            .send()
            .await?
            .json::<RpcResponse>()
            .await
            .map_err(MojaveClientError::from)
    }

    pub async fn send_forward_transaction(self, data: &[u8]) -> Result<H256, MojaveClientError> {
        let request = RpcRequest {
            id: RpcRequestId::Number(1),
            jsonrpc: "2.0".to_string(),
            method: "mojave_sendForwardTransaction".to_string(),
            params: Some(vec![json!("0x".to_string() + &hex::encode(data))]),
        };

        match self.send_request(request).await {
            Ok(RpcResponse::Success(_result)) => {
                todo!()
                // serde_json::from_value(result.result).map_err(MojaveClientError::from)
            }
            Ok(RpcResponse::Error(_error_response)) => {
                todo!()
            }
            Err(error) => Err(error),
        }
    }

    pub async fn send_broadcast_block(self, block: &Block) -> Result<(), MojaveClientError> {
        let request = RpcRequest {
            id: RpcRequestId::Number(1),
            jsonrpc: "2.0".to_string(),
            method: "mojave_sendBroadcastBlock".to_string(),
            params: Some(vec![json!(block)]),
        };

        match self.send_request(request).await {
            Ok(RpcResponse::Success(result)) => {
                serde_json::from_value(result.result).map_err(MojaveClientError::from)
            }
            Ok(RpcResponse::Error(error_response)) => {
                Err(MojaveClientError::RpcError(error_response.error.message))
            }
            Err(error) => Err(error),
        }
    }
}
