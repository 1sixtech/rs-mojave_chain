use anyhow::Result;
use clap::Parser;
use mojave::{cli::CLI, logging::init_logging};

#[tokio::main]
async fn main() -> Result<()> {
    let CLI { log_level, command } = CLI::parse();

    init_logging(log_level);

    tracing::debug!( command = ?command, "Starting Mojave node");

    command.run().await?;

    Ok(())
}
