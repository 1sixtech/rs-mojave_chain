use ethrex_blockchain::error::{ChainError, MempoolError};
use ethrex_rpc::RpcErrorMetadata;
use ethrex_storage::error::StoreError;
use ethrex_vm::EvmError;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, thiserror::Error)]
pub enum RpcErr {
    #[error(transparent)]
    EthrexRPC(ethrex_rpc::RpcErr),
    #[error("Custom error: {0}")]
    CustomError(String),
    #[error("Blockchain error: {0}")]
    BlockchainError(#[from] ChainError),
}

impl From<RpcErr> for RpcErrorMetadata {
    fn from(value: RpcErr) -> Self {
        match value {
            RpcErr::EthrexRPC(err) => err.into(),
            RpcErr::CustomError(err) => RpcErrorMetadata {
                code: -38000,
                data: None,
                message: err,
            },
            RpcErr::BlockchainError(err) => RpcErrorMetadata {
                code: -38001,
                data: None,
                message: err.to_string(),
            },
        }
    }
}

impl From<serde_json::Error> for RpcErr {
    fn from(error: serde_json::Error) -> Self {
        Self::EthrexRPC(ethrex_rpc::RpcErr::BadParams(error.to_string()))
    }
}

// TODO: Actually return different errors for each case
// here we are returning a BadParams error
impl From<MempoolError> for RpcErr {
    fn from(err: MempoolError) -> Self {
        match err {
            MempoolError::StoreError(err) => {
                Self::EthrexRPC(ethrex_rpc::RpcErr::Internal(err.to_string()))
            }
            other_err => Self::EthrexRPC(ethrex_rpc::RpcErr::BadParams(other_err.to_string())),
        }
    }
}

impl From<secp256k1::Error> for RpcErr {
    fn from(err: secp256k1::Error) -> Self {
        Self::EthrexRPC(ethrex_rpc::RpcErr::Internal(format!(
            "Cryptography error: {err}"
        )))
    }
}

pub enum RpcNamespace {
    Engine,
    Eth,
    Admin,
    Debug,
    Web3,
    Net,
    Mempool,
    Mojave,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum RpcRequestId {
    Number(u64),
    String(String),
}

impl From<RpcRequestId> for ethrex_rpc::utils::RpcRequestId {
    fn from(id: RpcRequestId) -> Self {
        match id {
            RpcRequestId::Number(num) => ethrex_rpc::utils::RpcRequestId::Number(num),
            RpcRequestId::String(str) => ethrex_rpc::utils::RpcRequestId::String(str),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcRequest {
    pub id: RpcRequestId,
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<Vec<Value>>,
}

impl RpcRequest {
    pub fn namespace(&self) -> Result<RpcNamespace, RpcErr> {
        let mut parts = self.method.split('_');
        let Some(namespace) = parts.next() else {
            return Err(RpcErr::EthrexRPC(ethrex_rpc::RpcErr::MethodNotFound(
                self.method.clone(),
            )));
        };
        resolve_namespace(namespace, self.method.clone())
    }
}

impl From<RpcRequest> for ethrex_rpc::utils::RpcRequest {
    fn from(req: RpcRequest) -> Self {
        ethrex_rpc::utils::RpcRequest {
            id: req.id.into(),
            jsonrpc: req.jsonrpc,
            method: req.method,
            params: req.params,
        }
    }
}

impl From<&RpcRequest> for ethrex_rpc::utils::RpcRequest {
    fn from(req: &RpcRequest) -> Self {
        ethrex_rpc::utils::RpcRequest {
            id: req.id.clone().into(),
            jsonrpc: req.jsonrpc.clone(),
            method: req.method.clone(),
            params: req.params.clone(),
        }
    }
}

pub fn resolve_namespace(maybe_namespace: &str, method: String) -> Result<RpcNamespace, RpcErr> {
    match maybe_namespace {
        "engine" => Ok(RpcNamespace::Engine),
        "eth" => Ok(RpcNamespace::Eth),
        "mojave" => Ok(RpcNamespace::Mojave),
        "admin" => Ok(RpcNamespace::Admin),
        "debug" => Ok(RpcNamespace::Debug),
        "web3" => Ok(RpcNamespace::Web3),
        "net" => Ok(RpcNamespace::Net),
        // TODO: The namespace is set to match geth's namespace for compatibility, consider changing it in the future
        "txpool" => Ok(RpcNamespace::Mempool),
        _ => Err(RpcErr::EthrexRPC(ethrex_rpc::RpcErr::MethodNotFound(
            method,
        ))),
    }
}

impl Default for RpcRequest {
    fn default() -> Self {
        RpcRequest {
            id: RpcRequestId::Number(1),
            jsonrpc: "2.0".to_string(),
            method: "".to_string(),
            params: None,
        }
    }
}

pub fn rpc_response<E>(id: RpcRequestId, res: Result<Value, E>) -> Result<Value, RpcErr>
where
    E: Into<RpcErrorMetadata>,
{
    Ok(match res {
        Ok(result) => serde_json::to_value(RpcSuccessResponse {
            id,
            jsonrpc: "2.0".to_string(),
            result,
        }),
        Err(error) => serde_json::to_value(RpcErrorResponse {
            id,
            jsonrpc: "2.0".to_string(),
            error: error.into(),
        }),
    }?)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcSuccessResponse {
    pub id: RpcRequestId,
    pub jsonrpc: String,
    pub result: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcErrorResponse {
    pub id: RpcRequestId,
    pub jsonrpc: String,
    pub error: RpcErrorMetadata,
}

/// Failure to read from DB will always constitute an internal error
impl From<StoreError> for RpcErr {
    fn from(value: StoreError) -> Self {
        RpcErr::EthrexRPC(ethrex_rpc::RpcErr::Internal(value.to_string()))
    }
}

impl From<EvmError> for RpcErr {
    fn from(value: EvmError) -> Self {
        RpcErr::EthrexRPC(ethrex_rpc::RpcErr::Vm(value.to_string()))
    }
}

pub fn parse_json_hex(hex: &serde_json::Value) -> Result<u64, String> {
    if let Value::String(maybe_hex) = hex {
        let trimmed = maybe_hex.trim_start_matches("0x");
        let maybe_parsed = u64::from_str_radix(trimmed, 16);
        maybe_parsed.map_err(|_| format!("Could not parse given hex {maybe_hex}"))
    } else {
        Err(format!("Could not parse given hex {hex}"))
    }
}
