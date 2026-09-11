//! Startup ASCII-art banner.

/// The O2 logo, printed on startup (uses `o2`'s own ASCII art).
pub const ASCII_ART: &str = include_str!("../../src/ascii-art.txt");

/// Print the banner to stdout.
pub fn print_banner() {
    println!("{ASCII_ART}");
    println!();
    println!(
        "  O2Rust v{} — an accurate & fast SGI O2 (IP32) workstation emulator",
        o2rust::VERSION
    );
    println!(
        "  MIPS R5000 CPU, CRIME/GBE graphics, MACE I/O, $1GiB UMA. Phase 2 (core emulation)."
    );
    println!();
}