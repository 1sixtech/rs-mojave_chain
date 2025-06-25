use anyhow::anyhow;
use clap::{ArgAction, Parser};
use ethrex_vm::EvmEngine;
use mojave_chain_json_rpc::config::RpcConfig;
use std::fmt;
use tracing::Level;

use crate::{network::Network, DEFAULT_DATADIR};

pub fn parse_evm_level(s: &str) -> anyhow::Result<EvmEngine> {
    EvmEngine::try_from(s.to_owned()).map_err(|e| anyhow!(e))
}

#[derive(Parser)]
pub struct Opts {
    #[arg(
        long = "ws.port",
        default_value_t = 8546,
        value_name = "WS_PORT",
        help = "The port to listen for WebSocket requests.",
        help_heading = "Node options"
    )]
    pub ws_port: u16,
    #[arg(
        long = "ws.host",
        default_value = "0.0.0.0",
        value_name = "WS_HOST",
        help = "The host to listen for WebSocket requests.",
        help_heading = "Node options"
    )]
    pub ws_host: String,
    #[arg(
        long = "network",
        default_value_t = Network::default(),
        value_name = "GENESIS_FILE_PATH",
        help = "Receives a `Genesis` struct in json format. This is the only argument which is required. You can look at some example genesis files at `test_data/genesis*`.",
        long_help = "Alternatively, the name of a known network can be provided instead to use its preset genesis file and include its preset bootnodes. The networks currently supported include holesky, sepolia, hoodi and mainnet.",
        help_heading = "Node options",
        env = "ETHREX_NETWORK",
        value_parser = clap::value_parser!(Network),
    )]
    pub network: Network,
    // #[arg(long = "bootnodes", value_parser = clap::value_parser!(Node), value_name = "BOOTNODE_LIST", value_delimiter = ',', num_args = 1.., help = "Comma separated enode URLs for P2P discovery bootstrap.", help_heading = "P2P options")]
    // pub bootnodes: Vec<Node>,
    #[arg(
        long = "datadir",
        value_name = "DATABASE_DIRECTORY",
        help = "If the datadir is the word `memory`, ethrex will use the InMemory Engine",
        default_value = DEFAULT_DATADIR,
        help = "Receives the name of the directory where the Database is located.",
        long_help = "If the datadir is the word `memory`, ethrex will use the `InMemory Engine`.",
        help_heading = "Node options",
        env = "ETHREX_DATADIR"
    )]
    pub datadir: String,
    #[arg(
        long = "force",
        help = "Force remove the database",
        long_help = "Delete the database without confirmation.",
        action = clap::ArgAction::SetTrue,
        help_heading = "Node options"
    )]
    pub force: bool,
    #[arg(
        long = "metrics.addr",
        value_name = "ADDRESS",
        default_value = "0.0.0.0",
        help_heading = "Node options"
    )]
    pub metrics_addr: String,
    #[arg(
        long = "metrics.port",
        value_name = "PROMETHEUS_METRICS_PORT",
        default_value = "9090", // Default Prometheus port (https://prometheus.io/docs/tutorials/getting_started/#show-me-how-it-is-done).
        help_heading = "Node options",
        env = "ETHREX_METRICS_PORT"
    )]
    pub metrics_port: String,
    #[arg(
        long = "metrics",
        action = ArgAction::SetTrue,
        help = "Enable metrics collection and exposition",
        help_heading = "Node options"
    )]
    pub metrics_enabled: bool,
    #[arg(
        long = "dev",
        action = ArgAction::SetTrue,
        help = "Used to create blocks without requiring a Consensus Client",
        long_help = "If set it will be considered as `true`. The Binary has to be built with the `dev` feature enabled.",
        help_heading = "Node options"
    )]
    pub dev: bool,
    #[arg(
        long = "evm",
        default_value_t = EvmEngine::default(),
        value_name = "EVM_BACKEND",
        help = "Has to be `levm` or `revm`",
        value_parser = parse_evm_level,
        help_heading = "Node options",
        env = "ETHREX_EVM")]
    pub evm: EvmEngine,
    #[arg(
        long = "log.level",
        default_value_t = Level::INFO,
        value_name = "LOG_LEVEL",
        help = "The verbosity level used for logs.",
        long_help = "Possible values: info, debug, trace, warn, error",
        help_heading = "Node options")]
    pub log_level: Level,
    #[arg(
        long = "http.addr",
        default_value = "localhost",
        value_name = "ADDRESS",
        help = "Listening address for the http rpc server.",
        help_heading = "RPC options",
        env = "ETHREX_HTTP_ADDR"
    )]
    pub http_addr: String,
    #[arg(
        long = "http.port",
        default_value = "8545",
        value_name = "PORT",
        help = "Listening port for the http rpc server.",
        help_heading = "RPC options",
        env = "ETHREX_HTTP_PORT"
    )]
    pub http_port: String,
    #[arg(
        long = "authrpc.addr",
        default_value = "localhost",
        value_name = "ADDRESS",
        help = "Listening address for the authenticated rpc server.",
        help_heading = "RPC options"
    )]
    pub authrpc_addr: String,
    #[arg(
        long = "authrpc.port",
        default_value = "8551",
        value_name = "PORT",
        help = "Listening port for the authenticated rpc server.",
        help_heading = "RPC options"
    )]
    pub authrpc_port: String,
    #[arg(
        long = "authrpc.jwtsecret",
        default_value = "jwt.hex",
        value_name = "JWTSECRET_PATH",
        help = "Receives the jwt secret used for authenticated rpc requests.",
        help_heading = "RPC options"
    )]
    pub authrpc_jwtsecret: String,
    #[arg(long = "p2p.enabled", default_value =  "true" , value_name = "P2P_ENABLED", action = ArgAction::SetTrue, help_heading = "P2P options")]
    pub p2p_enabled: bool,
    #[arg(
        long = "p2p.addr",
        default_value = "0.0.0.0",
        value_name = "ADDRESS",
        help_heading = "P2P options"
    )]
    pub p2p_addr: String,
    #[arg(
        long = "p2p.port",
        default_value = "30303",
        value_name = "PORT",
        help_heading = "P2P options"
    )]
    pub p2p_port: String,
    #[arg(
        long = "discovery.addr",
        default_value = "0.0.0.0",
        value_name = "ADDRESS",
        help = "UDP address for P2P discovery.",
        help_heading = "P2P options"
    )]
    pub discovery_addr: String,
    #[arg(
        long = "discovery.port",
        default_value = "30303",
        value_name = "PORT",
        help = "UDP port for P2P discovery.",
        help_heading = "P2P options"
    )]
    pub discovery_port: String,
    #[command(flatten)]
    pub options: ethrex::cli::Options,
}

impl Default for Opts {
    fn default() -> Self {
        Self {
            ws_port: 8546,
            ws_host: "0.0.0.0".to_string(),
            options: ethrex::cli::Options::default(),
        }
    }
}

impl From<Opts> for RpcConfig {
    fn from(options: Opts) -> Self {
        Self {
            rpc_address: format!(
                "{}:{}",
                options.options.http_addr, options.options.http_port
            ),
            websocket_address: format!("{}:{}", options.ws_host, options.ws_port),
        }
    }
}

impl fmt::Debug for Opts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Options")
            .field("network", &self.options.network)
            .field("bootnodes", &self.options.bootnodes)
            .field("datadir", &self.options.datadir)
            .field("force", &self.options.force)
            .field("syncmode", &self.options.syncmode)
            .field("metrics_addr", &self.options.metrics_addr)
            .field("metrics_port", &self.options.metrics_port)
            .field("metrics_enabled", &self.options.metrics_enabled)
            .field("dev", &self.options.dev)
            .field("evm", &self.options.evm)
            .field("log_level", &self.options.log_level)
            .field("http_addr", &self.options.http_addr)
            .field("http_port", &self.options.http_port)
            .field("websocket_host", &self.ws_host)
            .field("websocket_port", &self.ws_port)
            .field("authrpc_addr", &self.options.authrpc_addr)
            .field("authrpc_port", &self.options.authrpc_port)
            .field("authrpc_jwtsecret", &self.options.authrpc_jwtsecret)
            .field("p2p_enabled", &self.options.p2p_enabled)
            .field("p2p_addr", &self.options.p2p_addr)
            .field("p2p_port", &self.options.p2p_port)
            .field("discovery_addr", &self.options.discovery_addr)
            .field("discovery_port", &self.options.discovery_port)
            .finish()
    }
}
