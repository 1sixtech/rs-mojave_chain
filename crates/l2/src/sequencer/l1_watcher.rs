use spawned_concurrency::{CallResponse, CastResponse, GenServer, GenServerInMsg};
use spawned_rt::mpsc::Sender;

use crate::sequencer::errors::L1WatcherError;

pub struct L1Watcher;

impl L1Watcher {
    pub fn new() -> Self {
        L1Watcher {}
    }
}

impl Default for L1Watcher {
    fn default() -> Self {
        L1Watcher::new()
    }
}

#[derive(Debug, Clone)]
pub enum InMessage {
    Watch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutMessage {
    Done,
}

#[derive(Debug, Clone, Default)]
pub struct L1WatcherState;

impl L1Watcher {
    pub async fn spawn() -> Result<(), L1WatcherError> {
        let state = L1WatcherState;
        let mut l1_watcher = L1Watcher::start(state);

        l1_watcher
            .cast(InMessage::Watch)
            .await
            .map_err(L1WatcherError::GenServerError)
    }
}

impl GenServer for L1Watcher {
    type InMsg = InMessage;
    type OutMsg = OutMessage;
    type State = L1WatcherState;
    type Error = L1WatcherError;

    fn new() -> Self {
        L1Watcher
    }

    async fn handle_call(
        &mut self,
        _message: Self::InMsg,
        _tx: &Sender<GenServerInMsg<Self>>,
        _state: &mut Self::State,
    ) -> CallResponse<Self::OutMsg> {
        CallResponse::Reply(OutMessage::Done)
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
