use crate::rpc::{
    RpcHandler,
    full_node::{RpcApiContextFullNode, types::ordered_block::OrderedBlock},
    utils::RpcErr,
};

use ethrex_common::types::{Block, BlockBody, Transaction};
use ethrex_rpc::clients::eth::BlockByNumber;
use serde_json::Value;

pub struct BroadcastBlockRequest {
    block: Block,
}

impl RpcHandler<RpcApiContextFullNode> for BroadcastBlockRequest {
    fn parse(params: &Option<Vec<Value>>) -> Result<Self, RpcErr> {
        let block = get_block_data(params)?;
        Ok(Self { block })
    }

    async fn handle(&self, context: RpcApiContextFullNode) -> Result<Value, RpcErr> {
        let latest_block_number = context.l1_context.storage.get_latest_block_number().await? + 1;
        for block_number in latest_block_number..self.block.header.number {
            let block = context
                .eth_client
                .get_block_by_number(BlockByNumber::Number(block_number))
                .await?;

            match block.body {
                ethrex_rpc::types::block::BlockBodyWrapper::Full(full_block_body) => {
                    // transform RPCBlock to normal block
                    let transactions: Vec<Transaction> = full_block_body
                        .transactions
                        .iter()
                        .map(|b| b.tx.clone())
                        .collect();

                    context
                        .block_queue
                        .push(OrderedBlock(Block::new(
                            block.header,
                            BlockBody {
                                ommers: full_block_body.uncles,
                                transactions,
                                withdrawals: Some(full_block_body.withdrawals),
                            },
                        )))
                        .await;
                }
                ethrex_rpc::types::block::BlockBodyWrapper::OnlyHashes(..) => {
                    unreachable!()
                }
            }
        }

        context
            .block_queue
            .push(OrderedBlock(self.block.clone()))
            .await;
        Ok(Value::Null)
    }
}

fn get_block_data(req: &Option<Vec<Value>>) -> Result<Block, RpcErr> {
    let params = req
        .as_ref()
        .ok_or(RpcErr::EthrexRPC(ethrex_rpc::RpcErr::BadParams(
            "No params provided".to_owned(),
        )))?;
    if params.len() != 1 {
        return Err(RpcErr::EthrexRPC(ethrex_rpc::RpcErr::BadParams(format!(
            "Expected one param and {} were provided",
            params.len()
        ))));
    };

    let block = serde_json::from_value::<Block>(params[0].clone())?;
    Ok(block)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethrex_common::{
        Address, Bloom, Bytes, H256, U256,
        types::{Block, BlockBody, BlockHeader},
    };
    use serde_json::json;

    fn create_test_block() -> Block {
        Block {
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
        }
    }

    #[test]
    fn test_get_block_data_success() {
        let block = create_test_block();
        let block_json = serde_json::to_value(block.clone()).unwrap();
        let params = Some(vec![block_json]);

        let result = get_block_data(&params);
        assert!(result.is_ok());
        let parsed_block = result.unwrap();
        assert_eq!(parsed_block.header.number, block.header.number);
    }

    #[test]
    fn test_get_block_data_no_params() {
        let result = get_block_data(&None);
        assert!(result.is_err());
        if let Err(RpcErr::EthrexRPC(ethrex_rpc::RpcErr::BadParams(msg))) = result {
            assert_eq!(msg, "No params provided");
        } else {
            panic!("Expected BadParams error");
        }
    }

    #[test]
    fn test_get_block_data_empty_params() {
        let params = Some(vec![]);
        let result = get_block_data(&params);
        assert!(result.is_err());
        if let Err(RpcErr::EthrexRPC(ethrex_rpc::RpcErr::BadParams(msg))) = result {
            assert_eq!(msg, "Expected one param and 0 were provided");
        } else {
            panic!("Expected BadParams error");
        }
    }

    #[test]
    fn test_get_block_data_too_many_params() {
        let block = create_test_block();
        let block_json = serde_json::to_value(block).unwrap();
        let params = Some(vec![block_json.clone(), block_json]);

        let result = get_block_data(&params);
        assert!(result.is_err());
        if let Err(RpcErr::EthrexRPC(ethrex_rpc::RpcErr::BadParams(msg))) = result {
            assert_eq!(msg, "Expected one param and 2 were provided");
        } else {
            panic!("Expected BadParams error");
        }
    }

    #[test]
    fn test_get_block_data_invalid_block_format() {
        let invalid_block = json!({"invalid": "data"});
        let params = Some(vec![invalid_block]);

        let result = get_block_data(&params);
        assert!(result.is_err());
        // Should be a serde deserialization error converted to BadParams
        assert!(matches!(
            result,
            Err(RpcErr::EthrexRPC(ethrex_rpc::RpcErr::BadParams(_)))
        ));
    }

    #[test]
    fn test_broadcast_block_request_parse_success() {
        let block = create_test_block();
        let block_json = serde_json::to_value(block).unwrap();
        let params = Some(vec![block_json]);

        let result = BroadcastBlockRequest::parse(&params);
        assert!(result.is_ok());
    }

    #[test]
    fn test_broadcast_block_request_parse_no_params() {
        let result = BroadcastBlockRequest::parse(&None);
        assert!(result.is_err());
    }

    #[test]
    fn test_broadcast_block_request_parse_invalid_params() {
        let invalid_params = Some(vec![json!({"invalid": "block"})]);
        let result = BroadcastBlockRequest::parse(&invalid_params);
        assert!(result.is_err());
    }
}
