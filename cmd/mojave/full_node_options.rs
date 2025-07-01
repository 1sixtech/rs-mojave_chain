use std::fmt;

use clap::Parser;

#[derive(Parser)]
pub struct FullNodeOptions {
    #[arg(
        long = "sequencer.port",
        default_value = "1739",
        help = "Port for the sequencer",
        help_heading = "Full Node Options",
        required = true
    )]
    pub sequencer_port: u16,
    #[arg(
        long = "sequencer.host",
        default_value = "0.0.0.0",
        help = "Host for the sequencer",
        help_heading = "Full Node Options",
        required = true
    )]
    pub sequencer_host: String,
}

impl Default for FullNodeOptions {
    fn default() -> Self {
        Self {
            sequencer_port: 1739,
            sequencer_host: "0.0.0.0".to_string(),
        }
    }
}

impl fmt::Debug for FullNodeOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FullNodeOptions")
            .field("sequencer_port", &self.sequencer_port)
            .field("sequencer_host", &self.sequencer_host)
            .finish()
    }
}
