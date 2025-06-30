use spawned_concurrency::{CallResponse, CastResponse, GenServer, GenServerInMsg};
use spawned_rt::mpsc::Sender;

use crate::sequencer::errors::MetricsGathererError;

#[derive(Debug, Clone, Default)]
pub struct MetricsGatherer;

#[derive(Debug, Clone, Default)]
pub struct MetricsGathererState;

#[derive(Clone)]
pub enum InMessage {
    Gather,
}

#[derive(Clone, PartialEq)]
pub enum OutMessage {}

impl MetricsGatherer {
    pub async fn spawn() -> Result<(), MetricsGathererError> {
        let state = MetricsGathererState;
        let mut block_producer = MetricsGatherer::start(state);
        block_producer
            .cast(InMessage::Gather)
            .await
            .map_err(MetricsGathererError::GenServerError)
    }
}

impl GenServer for MetricsGatherer {
    type InMsg = InMessage;
    type OutMsg = OutMessage;
    type State = MetricsGathererState;
    type Error = MetricsGathererError;

    fn new() -> Self {
        MetricsGatherer
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
