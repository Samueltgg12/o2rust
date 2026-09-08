//! O2Rust command-line interface.
//!
//! Boots the IP32 PROM and runs the emulator headlessly (no GUI). Useful for
//! testing the core library and for milestone M2 (CPU + memory execute the
//! PROM).

use anyhow::Result;
use clap::Parser;
use o2rust::system::Emulator;

/// O2Rust — an accurate & fast SGI O2 (IP32) emulator.
#[derive(Parser, Debug)]
#[command(name = "o2rust-cli", version, about)]
struct Args {
    /// Path to the IP32 PROM image (e.g. samples/ip32prom.rev4.18.bin).
    #[arg(short, long, default_value = "samples/ip32prom.rev4.18.bin")]
    prom: String,

    /// Amount of RAM in megabytes.
    #[arg(short, long, default_value_t = 256)]
    ram_mb: u32,

    /// Number of instructions to execute (0 = run until stopped).
    #[arg(short, long, default_value_t = 0)]
    steps: u64,

    /// Enable verbose (debug) logging.
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.verbose {
        unsafe {
            std::env::set_var("RUST_LOG", "o2rust=debug,o2rust_cli=debug");
        }
    }
    o2rust::log::init();

    o2rust::log::info_msg(&format!("O2Rust v{} — CLI", o2rust::VERSION));

    let mut emulator = Emulator::with_ram(args.ram_mb);
    emulator.load_prom(&args.prom)?;

    o2rust::log::info_msg(&format!(
        "Booting PROM at reset vector 0x{:08x}",
        emulator.pc()
    ));

    if args.steps > 0 {
        emulator.run(args.steps);
        o2rust::log::info_msg(&format!(
            "Executed {} instructions, PC = 0x{:08x}",
            args.steps,
            emulator.pc()
        ));
    } else {
        o2rust::log::info_msg("Running until stopped (Ctrl+C to quit)...");
        // Run in a loop until interrupted.
        loop {
            emulator.run(1_000_000);
        }
    }

    Ok(())
}