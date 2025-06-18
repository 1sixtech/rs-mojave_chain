use anyhow::Result;
use clap::Subcommand;
use mojave_chain_json_rpc::{config::RpcConfig, server::RpcServer};

use crate::options::Options;

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(name = "full-node", about = "Run a full node")]
    FullNode {
        #[command(flatten)]
        opts: Options,
    },
    #[command(name = "sequencer", about = "Run a sequencer")]
    Sequencer,
}

impl Command {
    pub async fn run(self) -> Result<()> {
        match self {
            Command::FullNode { opts } => {
                let rpc_config: RpcConfig = opts.into();
                let backend = mojave_chain_full_node::dummy::DummyBackend::new();
                let rpc_handle = RpcServer::init(&rpc_config, backend, None).await?;

                tokio::spawn(rpc_handle);

                tokio::select! {
                    _ = tokio::signal::ctrl_c() => {
                        tracing::info!("Server shut down started...");
                        tracing::info!("Server shutting down!");
                    }
                }
            }
            Command::Sequencer => todo!(),
        }
        Ok(())
    }
}
