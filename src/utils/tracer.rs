//! This module initializes the tracing subscriber for logging events.
//! It sets up the subscriber to filter logs based on the `RUST_LOG` environment

use tracing_subscriber::EnvFilter;


/// Initialize the tracing subscriber with a filter.
/// This function sets up the tracing subscriber to log events based on the
/// 
/// environment variable `RUST_LOG`. If the variable is not set, it defaults to
/// 
pub fn init_tracing() {
    let filter: EnvFilter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt().with_env_filter(filter).init();
}
