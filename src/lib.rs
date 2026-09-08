// src/lib.rs
//! # O2Rust
//!
//! An accurate & fast emulator for the SGI O2 (IP32 / "Moosehead") workstation.
//!
//! This crate is the **core library** — it contains the CPU, memory, graphics,
//! I/O, and PROM firmware emulation. The `cli` and `gui` crates are thin
//! front-ends that drive this library.
//!
//! ## Phase 2 (Emulation) — current
//!
//! The emulator is built up in milestones:
//!
//! - **M2** — CPU + memory execute the PROM (reset vector `0xBFC00000`).
//! - **M3** — Graphics + I/O functional; an OS boots.
//! - **M4** — GUI complete; usable emulator.
//!
//! Every register, address, and behavior is backed by a source in `docs/`
//! (Linux/NetBSD driver, leaked IRIX source, or datasheet). No guessing.

pub mod cpu;
pub mod graphics;
pub mod io;
pub mod log;
pub mod memory;
pub mod prom;
pub mod system;

/// Common types and constants shared across the emulator.
pub mod types {
    /// Unsigned 8-bit value.
    pub type U8 = u8;
    /// Unsigned 16-bit value.
    pub type U16 = u16;
    /// Unsigned 32-bit value.
    pub type U32 = u32;
    /// Unsigned 64-bit value.
    pub type U64 = u64;
}

/// IP32 (O2) hardware constants, sourced from the decompiled PROM
/// `definitions.h` and the IRIX `crime.h`/`mace.h` headers.
pub mod ip32 {
    // === Memory segments (definitions.h) ===
    /// User segment (unmapped).
    pub const KUSEG: u32 = 0x0000_0000;
    /// Kernel segment 0 (cached, unmapped).
    pub const KSEG0: u32 = 0x8000_0000;
    /// Kernel segment 1 (uncached, unmapped).
    pub const KSEG1: u32 = 0xa000_0000;
    /// Kernel segment 2 (mapped).
    pub const KSEG2: u32 = 0xc000_0000;

    // === Physical device bases (IRIX crime.h / mace.h) ===
    /// CRIME CPU interface base.
    pub const PHYS_BASE_CRIME: u32 = 0x1400_0000;
    /// CRIME render engine base.
    pub const PHYS_BASE_RENDER: u32 = 0x1500_0000;
    /// GBE display engine base.
    pub const PHYS_BASE_GBE: u32 = 0x1600_0000;
    /// MACE (I/O engine) base.
    pub const PHYS_BASE_MACE: u32 = 0x1f00_0000;
    /// System ROM / PROM base (KSEG1 view).
    pub const PHYS_SYSTEM_ROM: u32 = 0x1fc0_0000;

    /// MIPS reset vector (uncached boot address).
    pub const PROM_RESET_VECTOR: u32 = 0xbfc0_0000;
    /// Firmware virtual memory address (where POST/sloader copies firmware).
    pub const PROM_VMA_BASE: u32 = 0x8100_0000;

    /// Size of the PROM window (512 KiB).
    pub const SYSTEM_ROM_WINDOW_SIZE: u32 = 0x0008_0000;

    /// Maximum main memory (1 GiB).
    pub const MAX_MEMORY: u32 = 0x4000_0000;
}

/// Re-export the version string.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");