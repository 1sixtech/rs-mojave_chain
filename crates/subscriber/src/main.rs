mod args;

use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use clap::Parser;
use futures::StreamExt;
use std::error::Error;

use crate::args::Args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    let connection_detail = WsConnect::new(args.websocket_url);
    let provider = ProviderBuilder::new().on_ws(connection_detail).await?;

    let mut block_stream = provider.clone().subscribe_blocks().await?.into_stream();
    let mut transaction_stream = provider
        .clone()
        .subscribe_pending_transactions()
        .await?
        .into_stream();

    let task_1 = tokio::spawn(async move {
        while let Some(block) = block_stream.next().await {
            tracing::info!(block = ?block, "new block");
        }
    });

    let task_2 = tokio::spawn(async move {
        while let Some(transaction_hash) = transaction_stream.next().await {
            tracing::info!(transaction_hash = ?transaction_hash);
        }
    });

    tokio::try_join!(task_1, task_2)?;

    Ok(())
}
