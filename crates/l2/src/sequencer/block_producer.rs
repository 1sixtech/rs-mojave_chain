use spawned_concurrency::{CallResponse, CastResponse, GenServer, GenServerInMsg};
use spawned_rt::mpsc::Sender;

use crate::sequencer::errors::BlockProducerError;

#[derive(Debug, Clone, Default)]
pub struct BlockProducer;

#[derive(Debug, Clone, Default)]
pub struct BlockProducerState;

#[derive(Debug, Clone)]
pub enum InMessage {
    Produce,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutMessage {}

impl BlockProducer {
    pub async fn spawn() -> Result<(), BlockProducerError> {
        let state = BlockProducerState;
        let mut block_producer = BlockProducer::start(state);
        block_producer
            .cast(InMessage::Produce)
            .await
            .map_err(BlockProducerError::GenServerError)
    }
}

impl GenServer for BlockProducer {
    type InMsg = InMessage;
    type OutMsg = OutMessage;
    type State = BlockProducerState;
    type Error = BlockProducerError;

    fn new() -> Self {
        BlockProducer
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
