use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(name = "full-node", about = "Run a full node")]
    FullNode,
    #[command(name = "sequencer", about = "Run a sequencer")]
    Sequencer,
}
