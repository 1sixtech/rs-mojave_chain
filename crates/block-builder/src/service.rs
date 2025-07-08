use crate::{BlockBuilderContext, BlockBuilderError};
use ethrex_common::types::Block;
use tokio::sync::{
    mpsc::{self, error::TrySendError},
    oneshot,
};
use tokio_stream::{StreamExt, wrappers::ReceiverStream};
use tracing::error;

#[derive(Clone)]
pub struct BlockBuilder {
    sender: mpsc::Sender<Message>,
}

impl BlockBuilder {
    /// # Examples: Sequencer
    ///
    /// ```rust,ignore
    /// use std::time::Duration;
    /// use tokio::time::sleep;
    ///
    /// let block_builder = BlockBuilder::start(context, 100);
    /// let block_time_millis: u64 = 500;
    /// tokio::spawn(async move {
    ///     loop {
    ///         match block_builder.build_block().await {
    ///             Ok(block) => todo!("broadcast the block"),
    ///             Err(error) => {
    ///                 tracing::error!("Error {}", error);
    ///                 todo!("retry function");
    ///             }
    ///         }
    ///         sleep(Duration::from_millis(block_time)).await;
    ///     }
    /// });
    ///
    /// ```
    pub fn start(context: BlockBuilderContext, channel_capacity: usize) -> Self {
        let (sender, receiver) = mpsc::channel(channel_capacity);
        let mut receiver = ReceiverStream::new(receiver);

        tokio::spawn(async move {
            while let Some(message) = receiver.next().await {
                handle_message(&context, message).await;
            }

            error!("Block builder stopped because the sender dropped.");
        });
        Self { sender }
    }

    pub async fn build_block(&self) -> Result<Block, BlockBuilderError> {
        let (sender, receiver) = oneshot::channel();
        self.sender
            .try_send(Message::BuildBlock(sender))
            .map_err(|error| match error {
                TrySendError::Full(_) => BlockBuilderError::Full,
                TrySendError::Closed(_) => BlockBuilderError::Stopped,
            })?;
        receiver.await?
    }
}

async fn handle_message(context: &BlockBuilderContext, message: Message) {
    match message {
        Message::BuildBlock(sender) => {
            let _ = sender.send(context.build_block().await);
        }
    }
}

#[allow(clippy::large_enum_variant)]
enum Message {
    BuildBlock(oneshot::Sender<Result<Block, BlockBuilderError>>),
}
