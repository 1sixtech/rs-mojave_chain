use clap::Parser;
use tracing::Level;

use crate::{command::Command, version::get_version};

#[allow(clippy::upper_case_acronyms)]
#[derive(Parser)]
#[command(name = "mojave", author = "1six Technologies", version=get_version(), about = "Mojave is a blockchain node implementation for the Mojave network")]
pub struct CLI {
    #[arg(
      long = "log.level",
      default_value_t = Level::INFO,
      value_name = "LOG_LEVEL",
      help = "The verbosity level used for logs.",
      long_help = "Possible values: info, debug, trace, warn, error",
      help_heading = "Node options")]
    pub log_level: Level,
    #[command(subcommand)]
    pub command: Command,
}
