use spawned_concurrency::{CallResponse, CastResponse, GenServer, GenServerInMsg};
use spawned_rt::mpsc::Sender;

use crate::sequencer::errors::L1ProofSenderError;

#[derive(Debug, Clone, Default)]
pub struct L1ProofSender;

#[derive(Debug, Clone, Default)]
pub struct L1ProofSenderState;

#[derive(Clone)]
pub enum InMessage {
    Send,
}

#[derive(Clone, PartialEq)]
pub enum OutMessage {}

impl L1ProofSender {
    pub async fn spawn() -> Result<(), L1ProofSenderError> {
        let state = L1ProofSenderState;
        let mut block_producer = L1ProofSender::start(state);
        block_producer
            .cast(InMessage::Send)
            .await
            .map_err(L1ProofSenderError::GenServerError)
    }
}

impl GenServer for L1ProofSender {
    type InMsg = InMessage;
    type OutMsg = OutMessage;
    type State = L1ProofSenderState;
    type Error = L1ProofSenderError;

    fn new() -> Self {
        L1ProofSender
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
