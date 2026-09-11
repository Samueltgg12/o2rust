//! Command-line arguments for the O2Rust CLI.

use clap::Parser;

/// O2Rust — an accurate & fast SGI O2 (IP32) workstation emulator.
///
/// Boots the IP32 PROM in a windowed OpenGL framebuffer display with
/// keyboard/mouse input and audio output; pass `--headless` to fall back to
/// the console-only (UART) mode.
#[derive(Parser, Debug)]
#[command(name = "o2rust-cli", version, about, long_about)]
pub struct Args {
    /// Path to the IP32 PROM image (e.g. samples/ip32prom.rev4.18.bin).
    #[arg(index = 1, default_value = "samples/ip32prom.rev4.18.bin")]
    pub prom: String,

    /// Amount of RAM in megabytes.
    #[arg(short, long, default_value_t = 256)]
    pub ram_mb: u32,

    /// Run headlessly (console UART I/O) instead of opening a window.
    #[arg(long)]
    pub headless: bool,

    /// Number of instructions to execute in headless mode (0 = run forever).
    #[arg(short, long, default_value_t = 0)]
    pub steps: u64,

    /// Hard disk image (.raw/.img/.chd). Opens a file dialog when omitted.
    #[arg(short = 'd', long)]
    pub hard_disk: Option<String>,

    /// CD-ROM image (.iso/.img/.chd). Opens a file dialog when omitted.
    #[arg(short = 'c', long)]
    pub cdrom: Option<String>,

    /// Disable audio output (windowed mode).
    #[arg(long)]
    pub no_audio: bool,

    /// Log filter for the tracing subscriber (e.g. "o2rust=debug").
    #[arg(short = 'l', long, default_value = "o2rust=info,o2rust_cli=info")]
    pub log_filter: String,

    /// Shorthand for `-l o2rust=debug,o2rust_cli=debug`.
    #[arg(short, long)]
    pub verbose: bool,
}

impl Args {
    /// The effective log filter, honouring `--verbose`.
    pub fn effective_log_filter(&self) -> String {
        if self.verbose {
            "o2rust=debug,o2rust_cli=debug".to_string()
        } else {
            self.log_filter.clone()
        }
    }
}