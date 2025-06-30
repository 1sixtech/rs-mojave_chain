use spawned_concurrency::GenServerError;

#[derive(Debug, thiserror::Error)]
pub enum L1WatcherError {
    #[error("Failed to spawn L1 watcher")]
    SpawnFailed,
    #[error("Failed to cast message to L1 watcher")]
    GenServerError(GenServerError),
}

#[derive(Debug, thiserror::Error)]
pub enum BlockProducerError {
    #[error("Failed to cast message to BlockProducer")]
    GenServerError(GenServerError),
}

#[derive(Debug, thiserror::Error)]
pub enum L1ProofSenderError {
    #[error("Failed to cast message to L1ProofSender")]
    GenServerError(GenServerError),
}

#[derive(Debug, thiserror::Error)]
pub enum L1CommitterError {
    #[error("Failed to cast message to L1Commiter")]
    GenServerError(GenServerError),
}

#[derive(Debug, thiserror::Error)]
pub enum MetricsGathererError {
    #[error("Failed to cast message to MetricsGatherer")]
    GenServerError(GenServerError),
}

#[derive(Debug, thiserror::Error)]
pub enum ProofCoordinatorError {
    #[error("Failed to cast message to ProofCoordinator")]
    GenServerError(GenServerError),
}

// this is just a placeholder enum for now
#[derive(Debug, thiserror::Error)]
pub enum NeededProofError {
    #[error("Proof type not supported")]
    UnsupportedProofType,
}
