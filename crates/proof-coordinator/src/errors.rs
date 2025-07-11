use ethrex_l2_common::prover::BatchProof;
use mojave_prover::ProverData;
use tokio::sync::mpsc::error::SendError;

#[derive(Debug, thiserror::Error)]
pub enum ProofCoordinatorError {
    #[error("ProofCoordinator connection failed: {0}")]
    ConnectionError(#[from] std::io::Error),
    #[error("ProofCoordinator failed because of an EthClient error: {0}")]
    EthClientError(#[from] EthClientError),
    #[error("ProofCoordinator failed to send transaction: {0}")]
    FailedToVerifyProofOnChain(String),
    #[error("ProofCoordinator failed to access Store: {0}")]
    FailedAccessingStore(#[from] StoreError),
    #[error("ProverServer failed to access RollupStore: {0}")]
    FailedAccessingRollupStore(#[from] RollupStoreError),
    #[error("ProofCoordinator failed to retrieve block from storaga, data is None.")]
    StorageDataIsNone,
    #[error("ProofCoordinator failed to create ProverInputs: {0}")]
    FailedToCreateProverInputs(#[from] EvmError),
    #[error("ProofCoordinator failed to create ExecutionWitness: {0}")]
    FailedToCreateExecutionWitness(#[from] ChainError),
    #[error("ProofCoordinator JoinError: {0}")]
    JoinError(#[from] JoinError),
    #[error("ProofCoordinator failed: {0}")]
    Custom(String),
    #[error("ProofCoordinator failed to write to TcpStream: {0}")]
    WriteError(String),
    #[error("ProofCoordinator failed to get data from Store: {0}")]
    ItemNotFoundInStore(String),
    #[error("Failed to encode calldata: {0}")]
    CalldataEncodeError(#[from] CalldataEncodeError),
    #[error("Unexpected Error: {0}")]
    InternalError(String),
    #[error("ProofCoordinator failed when (de)serializing JSON: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("ProofCoordinator encountered a StateDiffError")]
    StateDiffError(#[from] StateDiffError),
    #[error("ProofCoordinator encountered a ExecutionCacheError")]
    ExecutionCacheError(#[from] ExecutionCacheError),
    #[error("ProofCoordinator encountered a BlobsBundleError: {0}")]
    BlobsBundleError(#[from] ethrex_common::types::BlobsBundleError),
    #[error("Failed to execute command: {0}")]
    ComandError(std::io::Error),
    #[error("ProofCoordinator failed failed because of a ProverDB error: {0}")]
    ProverDBError(#[from] ProverDBError),
    #[error("Missing blob for batch {0}")]
    MissingBlob(u64),
    #[error("ProofCoordinator prover data send error {0}")]
    ProverDataSendError(SendError<ProverData>),
    #[error("ProofCoordinator proof sender error {0}")]
    ProofSendError(SendError<(BatchProof, u64)>),
}
