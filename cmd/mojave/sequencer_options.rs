use std::fmt;

use clap::Parser;

#[derive(Parser)]
pub struct SequencerOptions {
    #[arg(
        long = "full_node.addresses",
        help = "Allowed domain(s) and port(s) for the sequencer in the form 'domain:port', can be specified multiple times",
        help_heading = "Full Node Options",
        required = true
    )]
    pub full_node_addresses: Vec<String>,
}
impl Default for SequencerOptions {
    fn default() -> Self {
        Self {
            full_node_addresses: vec!["0.0.0.0:8545".to_string()],
        }
    }
}

impl fmt::Debug for SequencerOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SequencerOptions")
            .field("full_node_addresses", &self.full_node_addresses)
            .finish()
    }
}
