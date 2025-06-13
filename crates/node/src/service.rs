use futures::{Stream, StreamExt};
use mohave_chain_types::{
    network::AnyHeader,
    primitives::B256,
    rpc::{Filter, Header, Log},
};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;

pub const QUEUE_LIMIT: usize = 100;

pub struct PubSubService {
    new_heads: broadcast::Sender<Header<AnyHeader>>,
    logs: broadcast::Sender<Log>,
    pending_transaction: broadcast::Sender<B256>,
}

impl Default for PubSubService {
    fn default() -> Self {
        Self {
            new_heads: broadcast::channel(QUEUE_LIMIT).0,
            logs: broadcast::channel(QUEUE_LIMIT).0,
            pending_transaction: broadcast::channel(QUEUE_LIMIT).0,
        }
    }
}

impl PubSubService {
    pub fn subscribe_new_heads(&self) -> NewHeadsStream {
        self.new_heads.subscribe().into()
    }

    pub fn subscribe_logs(&self, _filter: Option<Box<Filter>>) -> LogsStream {
        self.logs.subscribe().into()
    }

    pub fn subscribe_new_pending_transaction(&self) -> PendingTransactionStream {
        self.pending_transaction.subscribe().into()
    }

    pub fn publish_new_head(&self, new_head: Header<AnyHeader>) {
        let _ = self.new_heads.send(new_head);
    }

    pub fn publish_pending_transaction(&self, transaction_hash: B256) {
        let _ = self.pending_transaction.send(transaction_hash);
    }
}

pub struct NewHeadsStream(BroadcastStream<Header<AnyHeader>>);

impl From<broadcast::Receiver<Header<AnyHeader>>> for NewHeadsStream {
    fn from(value: broadcast::Receiver<Header<AnyHeader>>) -> Self {
        Self(value.into())
    }
}

impl Stream for NewHeadsStream {
    type Item = Header<AnyHeader>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        match this.0.poll_next_unpin(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(value) => match value {
                Some(item) => match item {
                    Ok(item) => Poll::Ready(Some(item)),
                    Err(_error) => Poll::Ready(None),
                },
                None => Poll::Ready(None),
            },
        }
    }
}

pub struct LogsStream(BroadcastStream<Log>);

impl From<broadcast::Receiver<Log>> for LogsStream {
    fn from(value: broadcast::Receiver<Log>) -> Self {
        Self(value.into())
    }
}

impl Stream for LogsStream {
    type Item = Log;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        match this.0.poll_next_unpin(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(value) => match value {
                Some(item) => match item {
                    Ok(item) => Poll::Ready(Some(item)),
                    Err(_error) => Poll::Ready(None),
                },
                None => Poll::Ready(None),
            },
        }
    }
}

pub struct PendingTransactionStream(BroadcastStream<B256>);

impl From<broadcast::Receiver<B256>> for PendingTransactionStream {
    fn from(value: broadcast::Receiver<B256>) -> Self {
        Self(value.into())
    }
}

impl Stream for PendingTransactionStream {
    type Item = B256;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        match this.0.poll_next_unpin(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(value) => match value {
                Some(item) => match item {
                    Ok(item) => Poll::Ready(Some(item)),
                    Err(_error) => Poll::Ready(None),
                },
                None => Poll::Ready(None),
            },
        }
    }
}
