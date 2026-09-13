//! Startup ASCII-art banner.

/// The O2 logo, printed on startup (uses `o2`'s own ASCII art).
pub const ASCII_ART: &str = include_str!("../../src/ascii-art.txt");

/// Print the banner to stderr (stdout is the emulated machine's serial
/// console in headless mode and must carry only guest output).
pub fn print_banner() {
    eprintln!("{ASCII_ART}");
    eprintln!();
    eprintln!(
        "  O2Rust v{} — an accurate & fast SGI O2 (IP32) workstation emulator",
        o2rust::VERSION
    );
    eprintln!(
        "  MIPS R5000 CPU, CRIME/GBE graphics, MACE I/O, $1GiB UMA. Phase 2 (core emulation)."
    );
    eprintln!();
}