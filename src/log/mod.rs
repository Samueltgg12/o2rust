//! Logging and tracing infrastructure.
//!
//! The emulator uses the [`tracing`] crate for structured, level-filtered
//! diagnostics. The `cli` and `gui` front-ends install a subscriber; the core
//! library only emits events.

use tracing::{debug, error, info, trace, warn};

/// Initialize the default logging subscriber.
///
/// Reads the `RUST_LOG` environment variable for filtering (e.g.
/// `RUST_LOG=o2rust=debug`). Falls back to `info` level for the `o2rust`
/// crates when unset.
pub fn init() {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("o2rust=info,o2rust_cli=info,o2rust_gui=info"));

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_target(true))
        .init();
}

/// Log a debug-level message.
pub fn debug_msg(msg: &str) {
    debug!("{msg}");
}

/// Log an info-level message.
pub fn info_msg(msg: &str) {
    info!("{msg}");
}

/// Log a warning-level message.
pub fn warn_msg(msg: &str) {
    warn!("{msg}");
}

/// Log an error-level message.
pub fn error_msg(msg: &str) {
    error!("{msg}");
}

/// Log a trace-level message.
pub fn trace_msg(msg: &str) {
    trace!("{msg}");
}