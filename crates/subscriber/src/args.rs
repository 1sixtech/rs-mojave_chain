use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "mojave-subscriber",
    version = "1.0",
    about = "Mojave Subscriber CLI"
)]
pub(crate) struct Args {
    #[arg(name = "websocket_url")]
    pub(crate) websocket_url: String,
}
