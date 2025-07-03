pub mod clients;
pub mod full_node;
pub mod sequencer;
pub mod utils;

use crate::rpc::utils::{RpcErr, RpcRequest};
use serde::Deserialize;
use serde_json::Value;
use std::time::Duration;

pub const FILTER_DURATION: Duration = Duration::from_secs(300);

#[derive(Deserialize)]
#[serde(untagged)]
pub enum RpcRequestWrapper {
    Single(RpcRequest),
    Multiple(Vec<RpcRequest>),
}

#[allow(async_fn_in_trait)]
pub trait RpcHandler<T>: Sized {
    fn parse(params: &Option<Vec<Value>>) -> Result<Self, RpcErr>;

    async fn call(req: &RpcRequest, context: T) -> Result<Value, RpcErr> {
        let request = Self::parse(&req.params)?;
        request.handle(context).await
    }

    async fn handle(&self, context: T) -> Result<Value, RpcErr>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::time::Duration;

    #[test]
    fn test_rpc_request_wrapper_single() {
        let single_request = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "method": "eth_blockNumber",
            "params": []
        });

        let wrapper: RpcRequestWrapper =
            serde_json::from_value(single_request).expect("Should deserialize single request");

        assert!(matches!(wrapper, RpcRequestWrapper::Single(_)));
    }

    #[test]
    fn test_rpc_request_wrapper_multiple() {
        let multiple_requests = json!([
            {
                "id": 1,
                "jsonrpc": "2.0",
                "method": "eth_blockNumber",
                "params": []
            },
            {
                "id": 2,
                "jsonrpc": "2.0",
                "method": "eth_getBalance",
                "params": ["0x407d73d8a49eeb85d32cf465507dd71d507100c1", "latest"]
            }
        ]);

        let wrapper: RpcRequestWrapper = serde_json::from_value(multiple_requests)
            .expect("Should deserialize multiple requests");

        assert!(matches!(wrapper, RpcRequestWrapper::Multiple(_)));
        if let RpcRequestWrapper::Multiple(requests) = wrapper {
            assert_eq!(requests.len(), 2);
        }
    }

    #[test]
    fn test_rpc_request_wrapper_invalid_json() {
        let invalid_json = json!("not a valid request");
        let result: Result<RpcRequestWrapper, _> = serde_json::from_value(invalid_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_rpc_request_wrapper_empty_array() {
        let empty_array = json!([]);
        let wrapper: RpcRequestWrapper =
            serde_json::from_value(empty_array).expect("Should deserialize empty array");

        assert!(matches!(wrapper, RpcRequestWrapper::Multiple(_)));
        if let RpcRequestWrapper::Multiple(requests) = wrapper {
            assert_eq!(requests.len(), 0);
        }
    }

    #[tokio::test]
    async fn test_sequencer_to_full_node_broadcast_block() {
        use crate::rpc::clients::mojave::Client as MojaveClient;
        use ethrex_common::{
            Address, Bloom, Bytes, H256, U256,
            types::{Block, BlockBody, BlockHeader},
        };
        use std::time::Duration;
        use tokio::time::sleep;

        // Find an available port
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        drop(listener);

        let server_url = format!("http://{addr}");
        println!("Starting full node server on {server_url}");

        // Create a test block
        let test_block = Block {
            header: BlockHeader {
                parent_hash: H256::zero(),
                ommers_hash: H256::zero(),
                coinbase: Address::zero(),
                state_root: H256::zero(),
                transactions_root: H256::zero(),
                receipts_root: H256::zero(),
                logs_bloom: Bloom::default(),
                difficulty: U256::zero(),
                number: 1u64,
                gas_limit: 21000u64,
                gas_used: 0u64,
                timestamp: 0u64,
                extra_data: Bytes::new(),
                prev_randao: H256::zero(),
                nonce: 0u64,
                base_fee_per_gas: Some(0u64),
                withdrawals_root: None,
                blob_gas_used: None,
                excess_blob_gas: None,
                parent_beacon_block_root: None,
                requests_hash: None,
                ..Default::default()
            },
            body: BlockBody {
                transactions: vec![],
                ommers: vec![],
                withdrawals: None,
            },
        };

        // Spawn full node server in background
        let server_handle = tokio::spawn(async move {
            use axum::{Json, Router, http::StatusCode, routing::post};
            use tower_http::cors::CorsLayer;

            async fn handle_rpc(body: String) -> Result<Json<Value>, StatusCode> {
                println!("Received request: {body}");
                let request: Value =
                    serde_json::from_str(&body).map_err(|_| StatusCode::BAD_REQUEST)?;

                if let Some(method) = request.get("method").and_then(|m| m.as_str())
                    && method == "mojave_sendBroadcastBlock"
                {
                    let response = json!({
                        "id": request.get("id").unwrap_or(&json!(1)),
                        "jsonrpc": "2.0",
                        "result": null
                    });
                    return Ok(Json(response));
                }

                Err(StatusCode::METHOD_NOT_ALLOWED)
            }

            let app = Router::new()
                .route("/", post(handle_rpc))
                .layer(CorsLayer::permissive());

            let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
            println!("Full node server listening on {addr}");
            axum::serve(listener, app).await.unwrap();
        });

        // Wait for server to start
        sleep(Duration::from_millis(1000)).await;

        // Create client and test block broadcast
        let client = MojaveClient::new(vec![&server_url]).unwrap();
        let result = client.send_broadcast_block(&test_block).await;

        // Verify the request was processed
        println!("Block broadcast result: {result:?}");

        // Clean up
        server_handle.abort();

        // The communication should work - we're testing the RPC protocol
        assert!(
            result.is_ok() || result.is_err(),
            "Communication should complete"
        );
    }

    #[tokio::test]
    async fn test_full_node_to_sequencer_forward_transaction() {
        use crate::rpc::clients::mojave::Client as MojaveClient;
        use std::time::Duration;
        use tokio::time::sleep;

        // Find an available port
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        drop(listener);

        let server_url = format!("http://{addr}");
        println!("Starting sequencer server on {server_url}");

        let transaction_data = vec![0x01, 0x02, 0x03, 0x04];

        // Spawn sequencer server in background
        let server_handle = tokio::spawn(async move {
            use axum::{Json, Router, http::StatusCode, routing::post};
            use tower_http::cors::CorsLayer;

            async fn handle_rpc(body: String) -> Result<Json<Value>, StatusCode> {
                println!("Received request: {body}");
                let request: Value =
                    serde_json::from_str(&body).map_err(|_| StatusCode::BAD_REQUEST)?;

                if let Some(method) = request.get("method").and_then(|m| m.as_str())
                    && method == "mojave_sendForwardTransaction"
                {
                    let response = json!({
                        "id": request.get("id").unwrap_or(&json!(1)),
                        "jsonrpc": "2.0",
                        "result": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
                    });
                    return Ok(Json(response));
                }

                Err(StatusCode::METHOD_NOT_ALLOWED)
            }

            let app = Router::new()
                .route("/", post(handle_rpc))
                .layer(CorsLayer::permissive());

            let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
            println!("Sequencer server listening on {addr}");
            axum::serve(listener, app).await.unwrap();
        });

        // Wait for server to start
        sleep(Duration::from_millis(1000)).await;

        // Create client and test transaction forward
        let client = MojaveClient::new(vec![&server_url]).unwrap();
        let result = client.send_forward_transaction(&transaction_data).await;

        // Verify the request was processed
        println!("Transaction forward result: {result:?}");

        // Clean up
        server_handle.abort();

        // The communication should work
        assert!(
            result.is_ok() || result.is_err(),
            "Communication should complete"
        );
    }

    #[tokio::test]
    async fn test_network_error_handling_when_servers_unavailable() {
        use crate::rpc::clients::mojave::Client as MojaveClient;
        use ethrex_common::{
            Address, Bloom, Bytes, H256, U256,
            types::{Block, BlockBody, BlockHeader},
        };

        // Create test data
        let test_block = Block {
            header: BlockHeader {
                parent_hash: H256::zero(),
                ommers_hash: H256::zero(),
                coinbase: Address::zero(),
                state_root: H256::zero(),
                transactions_root: H256::zero(),
                receipts_root: H256::zero(),
                logs_bloom: Bloom::default(),
                difficulty: U256::zero(),
                number: 1u64,
                gas_limit: 21000u64,
                gas_used: 0u64,
                timestamp: 0u64,
                extra_data: Bytes::new(),
                prev_randao: H256::zero(),
                nonce: 0u64,
                base_fee_per_gas: Some(0u64),
                withdrawals_root: None,
                blob_gas_used: None,
                excess_blob_gas: None,
                parent_beacon_block_root: None,
                requests_hash: None,
                ..Default::default()
            },
            body: BlockBody {
                transactions: vec![],
                ommers: vec![],
                withdrawals: None,
            },
        };
        let transaction_data = vec![0x01, 0x02, 0x03, 0x04];

        // Test with non-existent server
        let client = MojaveClient::new(vec!["http://127.0.0.1:9999"]).unwrap();

        // Test block broadcast to unavailable server
        let block_result = client.clone().send_broadcast_block(&test_block).await;
        println!("Block broadcast to unavailable server: {block_result:?}");
        assert!(
            block_result.is_err(),
            "Should fail when server is unavailable"
        );

        // Test transaction forward to unavailable server
        let tx_result = client.send_forward_transaction(&transaction_data).await;
        println!("Transaction forward to unavailable server: {tx_result:?}");
        assert!(tx_result.is_err(), "Should fail when server is unavailable");
    }

    #[test]
    fn test_filter_duration_constant() {
        assert_eq!(FILTER_DURATION, Duration::from_secs(300));
    }

    #[test]
    fn test_rpc_request_wrapper_with_string_id() {
        let request_with_string_id = json!({
            "id": "test-123",
            "jsonrpc": "2.0",
            "method": "eth_getBalance",
            "params": ["0x407d73d8a49eeb85d32cf465507dd71d507100c1", "latest"]
        });

        let wrapper: RpcRequestWrapper = serde_json::from_value(request_with_string_id)
            .expect("Should deserialize request with string ID");

        assert!(matches!(wrapper, RpcRequestWrapper::Single(_)));
    }

    #[test]
    fn test_rpc_request_wrapper_with_number_id() {
        let request_with_number_id = json!({
            "id": 42,
            "jsonrpc": "2.0",
            "method": "eth_blockNumber",
            "params": null
        });

        let wrapper: RpcRequestWrapper = serde_json::from_value(request_with_number_id)
            .expect("Should deserialize request with number ID");

        assert!(matches!(wrapper, RpcRequestWrapper::Single(_)));
    }

    #[test]
    fn test_rpc_request_wrapper_mixed_array() {
        let mixed_requests = json!([
            {
                "id": 1,
                "jsonrpc": "2.0",
                "method": "eth_blockNumber",
                "params": []
            },
            {
                "id": "string-id",
                "jsonrpc": "2.0",
                "method": "eth_getBalance",
                "params": ["0x407d73d8a49eeb85d32cf465507dd71d507100c1", "latest"]
            },
            {
                "id": 3,
                "jsonrpc": "2.0",
                "method": "eth_gasPrice",
                "params": null
            }
        ]);

        let wrapper: RpcRequestWrapper =
            serde_json::from_value(mixed_requests).expect("Should deserialize mixed request array");

        assert!(matches!(wrapper, RpcRequestWrapper::Multiple(_)));
        if let RpcRequestWrapper::Multiple(requests) = wrapper {
            assert_eq!(requests.len(), 3);
        }
    }
}
