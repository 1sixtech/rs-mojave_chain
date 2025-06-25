use clap::Parser;
use mojave_chain_json_rpc::config::RpcConfig;
use std::fmt;

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
