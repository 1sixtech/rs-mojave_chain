use tracing_subscriber::{filter::Directive, EnvFilter, FmtSubscriber};

use crate::options::Opts;

pub fn init_logging(opts: &Opts) {
    let log_filter = EnvFilter::builder()
        .with_default_directive(Directive::from(opts.options.log_level))
        .from_env_lossy();
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(log_filter)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
