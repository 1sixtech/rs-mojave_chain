use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "drip-node", version = "1.0", about = "DRiP Node CLI")]
pub(crate) struct Args {
    #[arg(name = "home_directory")]
    pub(crate) home_directory: String,
}
