use clap::Parser;

use crate::{command::Command, options::Opts, version::get_version};

#[allow(clippy::upper_case_acronyms)]
#[derive(Parser)]
#[command(name = "mojave", author = "1six Technologies", version=get_version(), about = "Mojave is a blockchain node implementation for the Mojave network")]
pub struct CLI {
    #[command(flatten)]
    pub opts: Opts,
    #[command(subcommand)]
    pub command: Command,
}
