use crate::rpc::{
    RpcHandler, full_node::types::transaction::SendRawTransactionRequest, utils::RpcErr,
};

use super::RpcApiContextFullNode;
use serde_json::Value;

impl RpcHandler<RpcApiContextFullNode> for SendRawTransactionRequest {
    fn parse(params: &Option<Vec<Value>>) -> Result<Self, RpcErr> {
        let data = get_transaction_data(params)?;

        Ok(SendRawTransactionRequest(data))
    }

    async fn handle(&self, context: RpcApiContextFullNode) -> Result<Value, RpcErr> {
        let tx_hash = context
            .mojave_client
            .send_forward_transaction(&self.0)
            .await
            .map_err(|e| RpcErr::EthrexRPC(ethrex_rpc::RpcErr::Internal(e.to_string())))?;

        serde_json::to_value(tx_hash)
            .map_err(|e| RpcErr::EthrexRPC(ethrex_rpc::RpcErr::Internal(e.to_string())))
    }
}

fn get_transaction_data(rpc_req_params: &Option<Vec<Value>>) -> Result<Vec<u8>, RpcErr> {
    let params =
        rpc_req_params
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

    let str_data = serde_json::from_value::<String>(params[0].clone())?;
    let str_data =
        str_data
            .strip_prefix("0x")
            .ok_or(RpcErr::EthrexRPC(ethrex_rpc::RpcErr::BadParams(
                "Params are note 0x prefixed".to_owned(),
            )))?;
    hex::decode(str_data)
        .map_err(|error| RpcErr::EthrexRPC(ethrex_rpc::RpcErr::BadParams(error.to_string())))
}
