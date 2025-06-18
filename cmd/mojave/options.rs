use clap::Parser;
use mojave_chain_json_rpc::config::RpcConfig;
use tracing::Level;

#[derive(Parser, Debug)]
pub struct Options {
    #[arg(
        long = "log.level",
        default_value_t = Level::INFO,
        value_name = "LOG_LEVEL",
        help = "The verbosity level used for logs.",
        long_help = "Possible values: info, debug, trace, warn, error",
        help_heading = "Node options")]
    pub log_level: Level,
    #[arg(
        long = "rpc.port",
        default_value_t = 8545,
        value_name = "RPC_PORT",
        help = "The port to listen for RPC requests.",
        help_heading = "Node options"
    )]
    pub rpc_port: u16,
    #[arg(
        long = "rpc.host",
        default_value = "0.0.0.0",
        value_name = "RPC_HOST",
        help = "The host to listen for RPC requests.",
        help_heading = "Node options"
    )]
    pub rpc_host: String,
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
}

impl Default for Options {
    fn default() -> Self {
        Self {
            log_level: Level::INFO,
            rpc_port: 8545,
            rpc_host: "0.0.0.0".to_owned(),
            ws_port: 8546,
            ws_host: "0.0.0.0".to_owned(),
        }
    }
}

impl From<Options> for RpcConfig {
    fn from(options: Options) -> Self {
        Self {
            rpc_address: format!("{}:{}", options.rpc_host, options.rpc_port),
            websocket_address: format!("{}:{}", options.ws_host, options.ws_port),
        }
    }
}
