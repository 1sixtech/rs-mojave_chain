use anyhow::Result;
use clap::Parser;
use mojave::{cli::CLI, logging::init_logging};

#[tokio::main]
async fn main() -> Result<()> {
    let CLI { opts, command } = CLI::parse();

    init_logging(&opts);

    tracing::info!(opts = ?opts, command = ?command, "Starting Mojave node");

    command.run().await?;

    Ok(())
}
