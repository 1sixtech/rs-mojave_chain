use std::time::Duration;

use anyhow::Result;
use clap::Subcommand;
use ethrex_vm::EvmEngine;
use mojave_chain_utils::resolve_datadir;

use crate::options::Opts;

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(name = "full-node", about = "Run a full node")]
    FullNode {
        #[command(flatten)]
        opts: Opts,
    },
    #[command(name = "sequencer", about = "Run a sequencer")]
    Sequencer {
        #[command(flatten)]
        opts: Opts,
    },
}

impl Command {
    pub async fn run(self) -> Result<()> {
        match self {
            Command::FullNode { opts } => {
                if opts.options.evm == EvmEngine::REVM {
                    panic!("Mojave doesn't support REVM, use LEVM instead.");
                }

                let data_dir = resolve_datadir(&opts.options.datadir);
                let rollup_store_dir = data_dir.clone() + "/rollup_store";

                // let network = get_network(&opts.options.node_opts);

                // let genesis = network.get_genesis()?;
                // let store = init_store(&data_dir, genesis).await;
                // let rollup_store = init_rollup_store(&rollup_store_dir).await;

                // let blockchain = init_blockchain(opts.node_opts.evm, store.clone());

                // let signer = get_signer(&data_dir);

                // let local_p2p_node = get_local_p2p_node(&opts.options.node_opts, &signer);

                // let local_node_record = Arc::new(Mutex::new(get_local_node_record(
                //     &data_dir,
                //     &local_p2p_node,
                //     &signer,
                // )));

                // let peer_table = peer_table(local_p2p_node.node_id());

                // // TODO: Check every module starts properly.
                // let tracker = TaskTracker::new();

                // let cancel_token = tokio_util::sync::CancellationToken::new();

                // init_rpc_api(
                //     &opts.options.node_opts,
                //     &opts,
                //     peer_table.clone(),
                //     local_p2p_node.clone(),
                //     local_node_record.lock().await.clone(),
                //     store.clone(),
                //     blockchain.clone(),
                //     cancel_token.clone(),
                //     tracker.clone(),
                //     rollup_store.clone(),
                // )
                // .await;

                // // Initialize metrics if enabled
                // if opts.node_opts.metrics_enabled {
                //     init_metrics(&opts.options.node_opts, tracker.clone());
                // }

                // if opts.node_opts.p2p_enabled {
                //     init_network(
                //         &opts.options.node_opts,
                //         &network,
                //         &data_dir,
                //         local_p2p_node,
                //         local_node_record.clone(),
                //         signer,
                //         peer_table.clone(),
                //         store.clone(),
                //         tracker.clone(),
                //         blockchain.clone(),
                //     )
                //     .await;
                // } else {
                //     info!("P2P is disabled");
                // }

                // let l2_sequencer_cfg = SequencerConfig::from(opts.sequencer_opts);

                // let l2_sequencer = ethrex_l2::start_l2(
                //     store,
                //     rollup_store,
                //     blockchain,
                //     l2_sequencer_cfg,
                //     format!(
                //         "http://{}:{}",
                //         opts.node_opts.http_addr, opts.node_opts.http_port
                //     ),
                // )
                // .into_future();

                // tracker.spawn(l2_sequencer);

                // tokio::select! {
                //     _ = tokio::signal::ctrl_c() => {
                //         tracing::info!("Server shut down started...");
                //         let node_config_path = PathBuf::from(data_dir + "/node_config.json");
                //         tracing::info!("Storing config at {:?}...", node_config_path);
                //         cancel_token.cancel();
                //         let node_config = NodeConfigFile::new(peer_table, local_node_record.lock().await.clone()).await;
                //         store_node_config_file(node_config, node_config_path).await;
                //         tokio::time::sleep(Duration::from_secs(1)).await;
                //         tracing::info!("Server shutting down!");
                //     }
                // }
            }
            Command::Sequencer { .. } => todo!(),
        }
        Ok(())
    }
}
