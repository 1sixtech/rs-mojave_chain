use spawned_concurrency::{CallResponse, CastResponse, GenServer, GenServerInMsg};
use spawned_rt::mpsc::Sender;

use crate::sequencer::errors::L1CommitterError;

#[derive(Debug, Clone, Default)]
pub struct L1Committer;

#[derive(Debug, Clone, Default)]
pub struct L1CommitterState;

#[derive(Clone)]
pub enum InMessage {
    Commit,
}

#[derive(Clone, PartialEq)]
pub enum OutMessage {}

impl L1Committer {
    pub async fn spawn() -> Result<(), L1CommitterError> {
        let state = L1CommitterState;
        let mut block_producer = L1Committer::start(state);
        block_producer
            .cast(InMessage::Commit)
            .await
            .map_err(L1CommitterError::GenServerError)
    }
}

impl GenServer for L1Committer {
    type InMsg = InMessage;
    type OutMsg = OutMessage;
    type State = L1CommitterState;
    type Error = L1CommitterError;

    fn new() -> Self {
        L1Committer
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
