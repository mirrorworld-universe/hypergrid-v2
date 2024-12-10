use anyhow::Result;
use tracing::*;
use tracing_subscriber::{
    layer::{Layer, SubscriberExt},
    util::SubscriberInitExt,
};

pub use tracing;
pub use tracing_subscriber;

/// Initialize the Traces.
///
/// Initializes the logger with a specific
/// verbosity
///
/// * `verbosity` - Verbosity level [options: 0,1,2,3,4]
pub fn initialize_logger(verbosity: u8) -> Result<()> {
    match verbosity {
        0 => std::env::set_var("RUST_LOG", "info"),
        1 => std::env::set_var("RUST_LOG", "debug"),
        2..=4 => std::env::set_var("RUST_LOG", "trace"),
        _ => std::env::set_var("RUST_LOG", "info"),
    };

    // Basic filter.
    let filter = tracing_subscriber::EnvFilter::from_default_env();

    // Initialize tracing.
    let _ = tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::Layer::default()
                .with_target(verbosity > 2)
                .with_filter(filter),
        )
        .try_init()?;

    Ok(())
}
