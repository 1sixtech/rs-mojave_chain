use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use futures::StreamExt;
use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let arguments: Vec<String> = env::args().skip(1).collect();
    let websocket_url = arguments.first().ok_or("Provide the websocket URL")?;

    let connection_detail = WsConnect::new(websocket_url);
    let provider = ProviderBuilder::new().on_ws(connection_detail).await?;

    let mut block_stream = provider.clone().subscribe_blocks().await?.into_stream();
    let mut transaction_stream = provider
        .clone()
        .subscribe_pending_transactions()
        .await?
        .into_stream();

    let task_1 = tokio::spawn(async move {
        while let Some(block) = block_stream.next().await {
            println!("{block:#?}");
        }
    });

    let task_2 = tokio::spawn(async move {
        while let Some(transaction_hash) = transaction_stream.next().await {
            println!("{transaction_hash:?}");
        }
    });

    tokio::try_join!(task_1, task_2)?;

    Ok(())
}
