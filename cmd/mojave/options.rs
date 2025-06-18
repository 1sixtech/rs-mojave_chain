use clap::Parser;
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
}
