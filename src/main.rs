// src/main.rs
//! O2Rust — SGI O2 (IP32) emulator.
//!
//! This binary is a thin wrapper around the `o2rust` core library. The
//! primary front-ends are the `cli` and `gui` crates; this binary exists for
//! quick smoke-testing of the core library.

use o2rust::log;

fn main() {
    log::init();
    log::info_msg(&format!("O2Rust v{} — SGI O2 (IP32) emulator", o2rust::VERSION));
    log::info_msg("Core library loaded. Use the `cli` or `gui` crates to run the emulator.
    println!("Hello, world!");
}
