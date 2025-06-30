use ethrex_l2::utils::prover::proving_systems::ProverType;

use crate::sequencer::errors::NeededProofError;

pub async fn get_needed_proof_types(dev_mode: bool) -> Result<Vec<ProverType>, NeededProofError> {
    let mut proof_types = vec![];
    if dev_mode {
        proof_types.push(ProverType::Exec);
    } else {
        return Err(NeededProofError::UnsupportedProofType);
    }

    Ok(proof_types)
}
