use std::{
    fs,
    future::IntoFuture,
    io,
    net::{Ipv4Addr, SocketAddr, ToSocketAddrs},
    path::PathBuf,
    sync::Arc,
    time::Duration,
};

use anyhow::Result;
use clap::Subcommand;
use ethrex::{
    initializers::{
        get_local_node_record, get_signer, init_blockchain, init_rollup_store, init_store,
    },
    utils::{
        get_client_version, read_jwtsecret_file, read_node_config_file, store_node_config_file,
        NodeConfigFile,
    },
};
use ethrex_blockchain::Blockchain;
use ethrex_common::Address;
use ethrex_l2::SequencerConfig;
use ethrex_p2p::{
    kademlia::KademliaTable,
    network::{peer_table, public_key_from_signing_key, P2PContext},
    peer_handler::PeerHandler,
    sync_manager::SyncManager,
    types::{Node, NodeRecord},
};
use ethrex_storage::Store;
use ethrex_storage_rollup::StoreRollup;
use ethrex_vm::EvmEngine;
use k256::ecdsa::SigningKey;
use local_ip_address::local_ip;
use mojave_chain_utils::resolve_datadir;
use tokio::sync::Mutex;
use tokio_util::{sync::CancellationToken, task::TaskTracker};

use crate::{
    networks::{self, Network},
    options::Options,
};

pub fn get_bootnodes(opts: &Options, network: &Network, data_dir: &str) -> Vec<Node> {
    let mut bootnodes: Vec<Node> = opts.bootnodes.clone();

    match network {
        Network::Mainnet => {
            tracing::info!("Adding mainnet preset bootnodes");
            bootnodes.extend(networks::MAINNET_BOOTNODES.clone());
        }
        Network::Testnet => {
            tracing::info!("Adding testnet preset bootnodes");
            bootnodes.extend(networks::TESTNET_BOOTNODES.clone());
        }
        _ => {}
    }

    if bootnodes.is_empty() {
        tracing::warn!(
            "No bootnodes specified. This node will not be able to connect to the network."
        );
    }

    let config_file = PathBuf::from(data_dir.to_owned() + "/node_config.json");

    tracing::info!("Reading known peers from config file {:?}", config_file);

    match read_node_config_file(config_file) {
        Ok(ref mut config) => bootnodes.append(&mut config.known_peers),
        Err(e) => tracing::error!("Could not read from peers file: {e}"),
    };

    bootnodes
}

#[allow(clippy::too_many_arguments)]
pub async fn init_network(
    opts: &Options,
    network: &Network,
    data_dir: &str,
    local_p2p_node: Node,
    local_node_record: Arc<Mutex<NodeRecord>>,
    signer: SigningKey,
    peer_table: Arc<Mutex<KademliaTable>>,
    store: Store,
    tracker: TaskTracker,
    blockchain: Arc<Blockchain>,
) {
    if opts.dev {
        tracing::error!("Binary wasn't built with The feature flag `dev` enabled.");
        panic!(
            "Build the binary with the `dev` feature in order to use the `--dev` cli's argument."
        );
    }

    let bootnodes = get_bootnodes(opts, network, data_dir);

    let context = P2PContext::new(
        local_p2p_node,
        local_node_record,
        tracker.clone(),
        signer,
        peer_table.clone(),
        store,
        blockchain,
        get_client_version(),
    );

    context.set_fork_id().await.expect("Set fork id");

    ethrex_p2p::start_network(context, bootnodes)
        .await
        .expect("Network starts");

    tracker.spawn(ethrex_p2p::periodically_show_peer_stats(peer_table.clone()));
}

pub fn init_metrics(opts: &Options, tracker: TaskTracker) {
    tracing::info!(
        "Starting metrics server on {}:{}",
        opts.metrics_addr,
        opts.metrics_port
    );
    let metrics_api = ethrex_metrics::api::start_prometheus_metrics_api(
        opts.metrics_addr.clone(),
        opts.metrics_port.clone(),
    );
    tracker.spawn(metrics_api);
}

pub fn parse_socket_addr(addr: &str, port: &str) -> io::Result<SocketAddr> {
    // NOTE: this blocks until hostname can be resolved
    format!("{addr}:{port}")
        .to_socket_addrs()?
        .next()
        .ok_or(io::Error::new(
            io::ErrorKind::NotFound,
            "Failed to parse socket address",
        ))
}

pub fn get_local_p2p_node(opts: &Options, signer: &SigningKey) -> Node {
    let udp_socket_addr = parse_socket_addr(&opts.discovery_addr, &opts.discovery_port)
        .expect("Failed to parse discovery address and port");
    let tcp_socket_addr =
        parse_socket_addr(&opts.p2p_addr, &opts.p2p_port).expect("Failed to parse addr and port");

    // TODO: If hhtp.addr is 0.0.0.0 we get the local ip as the one of the node, otherwise we use the provided one.
    // This is fine for now, but we might need to support more options in the future.
    let p2p_node_ip = if udp_socket_addr.ip() == Ipv4Addr::new(0, 0, 0, 0) {
        local_ip().expect("Failed to get local ip")
    } else {
        udp_socket_addr.ip()
    };

    let local_public_key = public_key_from_signing_key(signer);

    let node = Node::new(
        p2p_node_ip,
        udp_socket_addr.port(),
        tcp_socket_addr.port(),
        local_public_key,
    );

    // TODO Find a proper place to show node information
    // https://github.com/lambdaclass/ethrex/issues/836
    let enode = node.enode_url();
    tracing::info!("Node: {enode}");

    node
}

pub fn get_authrpc_socket_addr(opts: &Options) -> SocketAddr {
    parse_socket_addr(&opts.authrpc_addr, &opts.authrpc_port)
        .expect("Failed to parse authrpc address and port")
}

pub fn get_http_socket_addr(opts: &Options) -> SocketAddr {
    parse_socket_addr(&opts.http_addr, &opts.http_port)
        .expect("Failed to parse http address and port")
}

pub fn get_valid_delegation_addresses(opts: &Options) -> Vec<Address> {
    let Some(ref path) = opts.sponsorable_addresses_file_path else {
        tracing::warn!("No valid addresses provided, ethrex_SendTransaction will always fail");
        return Vec::new();
    };
    let addresses: Vec<Address> = fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Failed to load file {path}"))
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.to_string().parse::<Address>())
        .filter_map(Result::ok)
        .collect();
    if addresses.is_empty() {
        tracing::warn!("No valid addresses provided, ethrex_SendTransaction will always fail");
    }
    addresses
}

#[allow(clippy::too_many_arguments)]
pub async fn init_rpc_api(
    opts: &Options,
    peer_table: Arc<Mutex<KademliaTable>>,
    local_p2p_node: Node,
    local_node_record: NodeRecord,
    store: Store,
    blockchain: Arc<Blockchain>,
    cancel_token: CancellationToken,
    tracker: TaskTracker,
    rollup_store: StoreRollup,
) {
    let peer_handler = PeerHandler::new(peer_table);

    // Create SyncManager
    let syncer = SyncManager::new(
        peer_handler.clone(),
        opts.syncmode.clone(),
        cancel_token,
        blockchain.clone(),
        store.clone(),
    )
    .await;

    let rpc_api = ethrex_rpc::start_api(
        get_http_socket_addr(opts),
        get_authrpc_socket_addr(opts),
        store,
        blockchain,
        read_jwtsecret_file(&opts.authrpc_jwtsecret),
        local_p2p_node,
        local_node_record,
        syncer,
        peer_handler,
        get_client_version(),
        get_valid_delegation_addresses(opts),
        opts.sponsor_private_key,
        rollup_store,
    );

    tracker.spawn(rpc_api);
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(name = "full-node", about = "Run a full node")]
    FullNode {
        #[command(flatten)]
        opts: Options,
    },
    #[command(name = "sequencer", about = "Run a sequencer")]
    Sequencer {
        #[command(flatten)]
        opts: Options,
    },
}

impl Command {
    pub async fn run(self) -> Result<()> {
        match self {
            Command::FullNode { opts } => {
                if opts.evm == EvmEngine::REVM {
                    panic!("Mojave doesn't support REVM, use LEVM instead.");
                }

                let data_dir = resolve_datadir(&opts.datadir);
                let rollup_store_dir = data_dir.clone() + "/rollup_store";

                let genesis = opts.network.get_genesis()?;
                let store = init_store(&data_dir, genesis).await;
                let rollup_store = init_rollup_store(&rollup_store_dir).await;

                let blockchain = init_blockchain(opts.evm, store.clone());

                let signer = get_signer(&data_dir);

                let local_p2p_node = get_local_p2p_node(&opts, &signer);

                let local_node_record = Arc::new(Mutex::new(get_local_node_record(
                    &data_dir,
                    &local_p2p_node,
                    &signer,
                )));

                let peer_table = peer_table(local_p2p_node.node_id());

                // TODO: Check every module starts properly.
                let tracker = TaskTracker::new();

                let cancel_token = tokio_util::sync::CancellationToken::new();

                init_rpc_api(
                    &opts,
                    peer_table.clone(),
                    local_p2p_node.clone(),
                    local_node_record.lock().await.clone(),
                    store.clone(),
                    blockchain.clone(),
                    cancel_token.clone(),
                    tracker.clone(),
                    rollup_store.clone(),
                )
                .await;

                // Initialize metrics if enabled
                if opts.metrics_enabled {
                    init_metrics(&opts, tracker.clone());
                }

                if opts.p2p_enabled {
                    init_network(
                        &opts,
                        &opts.network,
                        &data_dir,
                        local_p2p_node,
                        local_node_record.clone(),
                        signer,
                        peer_table.clone(),
                        store.clone(),
                        tracker.clone(),
                        blockchain.clone(),
                    )
                    .await;
                } else {
                    tracing::info!("P2P is disabled");
                }

                let l2_sequencer_cfg = SequencerConfig::from(opts.sequencer_opts);

                let l2_sequencer = ethrex_l2::start_l2(
                    store,
                    rollup_store,
                    blockchain,
                    l2_sequencer_cfg,
                    #[cfg(feature = "metrics")]
                    format!("http://{}:{}", opts.http_addr, opts.http_port),
                )
                .into_future();

                tracker.spawn(l2_sequencer);

                tokio::select! {
                    _ = tokio::signal::ctrl_c() => {
                        tracing::info!("Server shut down started...");
                        let node_config_path = PathBuf::from(data_dir + "/node_config.json");
                        tracing::info!("Storing config at {:?}...", node_config_path);
                        cancel_token.cancel();
                        let node_config = NodeConfigFile::new(peer_table, local_node_record.lock().await.clone()).await;
                        store_node_config_file(node_config, node_config_path).await;
                        tokio::time::sleep(Duration::from_secs(1)).await;
                        tracing::info!("Server shutting down!");
                    }
                }
            }
            Command::Sequencer { .. } => todo!(),
        }
        Ok(())
    }
}
