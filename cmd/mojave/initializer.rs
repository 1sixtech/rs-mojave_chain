use std::{
    fs, io,
    net::{Ipv4Addr, SocketAddr, ToSocketAddrs},
    path::PathBuf,
    sync::Arc,
};

use anyhow::Result;
use ethrex::utils::{get_client_version, read_jwtsecret_file, read_node_config_file};
use ethrex_blockchain::Blockchain;
use ethrex_common::Address;
use ethrex_p2p::{
    kademlia::KademliaTable,
    network::{public_key_from_signing_key, P2PContext},
    peer_handler::PeerHandler,
    sync_manager::SyncManager,
    types::{Node, NodeRecord},
};
use ethrex_storage::Store;
use ethrex_storage_rollup::StoreRollup;
use k256::ecdsa::SigningKey;
use local_ip_address::local_ip;
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
