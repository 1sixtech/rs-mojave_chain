use tracing_subscriber::{filter::Directive, EnvFilter, FmtSubscriber};

use crate::options::Options;

pub fn init_logging(opts: &Options) {
    let log_filter = EnvFilter::builder()
        .with_default_directive(Directive::from(opts.log_level))
        .from_env_lossy();
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(log_filter)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
