use spawned_concurrency::{CallResponse, CastResponse, GenServer, GenServerInMsg};
use spawned_rt::mpsc::Sender;

use crate::sequencer::errors::ProofCoordinatorError;

#[derive(Debug, Clone, Default)]
pub struct ProofCoordinator;

#[derive(Debug, Clone, Default)]
pub struct ProofCoordinatorState;

#[derive(Clone)]
pub enum InMessage {
    Listen,
}

#[derive(Clone, PartialEq)]
pub enum OutMessage {}

impl ProofCoordinator {
    pub async fn spawn() -> Result<(), ProofCoordinatorError> {
        let state = ProofCoordinatorState;
        let mut block_producer = ProofCoordinator::start(state);
        block_producer
            .cast(InMessage::Listen)
            .await
            .map_err(ProofCoordinatorError::GenServerError)
    }
}

impl GenServer for ProofCoordinator {
    type InMsg = InMessage;
    type OutMsg = OutMessage;
    type State = ProofCoordinatorState;
    type Error = ProofCoordinatorError;

    fn new() -> Self {
        ProofCoordinator
    }

    async fn handle_call(
        &mut self,
        _message: Self::InMsg,
        _tx: &Sender<GenServerInMsg<Self>>,
        _state: &mut Self::State,
    ) -> CallResponse<Self::OutMsg> {
        todo!()
    }

    async fn handle_cast(
        &mut self,
        _message: Self::InMsg,
        _tx: &Sender<GenServerInMsg<Self>>,
        _state: &mut Self::State,
    ) -> CastResponse {
        CastResponse::NoReply
    }
}
