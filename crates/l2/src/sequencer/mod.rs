use std::sync::Arc;

use ethrex_blockchain::Blockchain;
use ethrex_l2::{
    SequencerConfig,
    based::sequencer_state::{SequencerState, SequencerStatus},
    utils::prover::proving_systems::ProverType,
};
use ethrex_storage::Store;
use ethrex_storage_rollup::StoreRollup;

#[cfg(feature = "metrics")]
use crate::sequencer::metrics_gatherer::MetricsGatherer;
use crate::sequencer::{
    block_producer::BlockProducer, l1_committer::L1Committer, l1_proof_sender::L1ProofSender,
    l1_watcher::L1Watcher, proof_coordinator::ProofCoordinator, utils::get_needed_proof_types,
};

pub mod block_producer;
pub mod l1_committer;
pub mod l1_proof_sender;
mod l1_watcher;
#[cfg(feature = "metrics")]
pub mod metrics_gatherer;
pub mod proof_coordinator;

pub mod errors;
pub mod utils;

pub async fn start_l2(
    _store: Store,
    _rollup_store: StoreRollup,
    _blockchain: Arc<Blockchain>,
    // TODO: probably need to be replaced with our own config
    cfg: SequencerConfig,
    #[cfg(feature = "metrics")] _l2_url: String,
) {
    let initial_status = if cfg.based.based {
        SequencerStatus::default()
    } else {
        SequencerStatus::Sequencing
    };

    tracing::info!("Starting Sequencer in {initial_status} mode");

    let _shared_state = SequencerState::from(initial_status);

    let Ok(needed_proof_types) = get_needed_proof_types(cfg.proof_coordinator.dev_mode)
        .await
        .inspect_err(|e| tracing::error!("Error starting the Proposer: {e}"))
    else {
        return;
    };

    if needed_proof_types.contains(&ProverType::Aligned) && !cfg.aligned.aligned_mode {
        tracing::error!(
            "Aligned mode is required. Please set the `--aligned` flag or use the `ALIGNED_MODE` environment variable to true."
        );
        return;
    }

    let _ = L1Watcher::spawn().await.inspect_err(|err| {
        tracing::error!("Error starting Watcher: {err}");
    });

    let _ = L1Committer::spawn().await.inspect_err(|err| {
        tracing::error!("Error starting Committer: {err}");
    });

    let _ = ProofCoordinator::spawn().await.inspect_err(|err| {
        tracing::error!("Error starting Proof Coordinator: {err}");
    });

    let _ = L1ProofSender::spawn().await.inspect_err(|err| {
        tracing::error!("Error starting L1 Proof Sender: {err}");
    });
    let _ = BlockProducer::spawn().await.inspect_err(|err| {
        tracing::error!("Error starting Block Producer: {err}");
    });

    #[cfg(feature = "metrics")]
    let _ = MetricsGatherer::spawn().await.inspect_err(|err| {
        tracing::error!("Error starting Block Producer: {err}");
    });
}
