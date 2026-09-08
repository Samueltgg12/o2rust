// src/io/mod.rs
//! I/O subsystem — the MACE ASIC.
//!
//! MACE (the I/O Engine) provides:
//!
//! - 64-bit PCI bus (single expansion slot)
//! - ISA bus (used only for the Super I/O chip: serial/parallel)
//! - PS/2 keyboard + mouse
//! - 10/100 Ethernet
//! - Audio (AD1843 codec)
//! - SCSI (Adaptec AIC-7880)
//!
//! **Status:** milestone M3 (graphics + I/O functional). This module is a
//! placeholder; the full MACE emulation is implemented in later milestones.

/// MACE base address (IRIX `mace.h`).
pub const MACE_BASE: u32 = 0x1f00_0000;

/// MACE sub-offsets (relative to `MACE_BASE`).
pub mod offsets {
    /// PCI bridge.
    pub const PCI: u32 = 0x080000;
    /// Video In 1.
    pub const VIN1: u32 = 0x100000;
    /// Video In 2.
    pub const VIN2: u32 = 0x180000;
    /// Video Out.
    pub const VOUT: u32 = 0x200000;
    /// Ethernet.
    pub const ENET: u32 = 0x280000;
    /// Peripheral (audio, ISA, keyboard/mouse, I2C, UST/MSC).
    pub const PERIF: u32 = 0x300000;
    /// ISA External (EPP, ECP, serial, RTC, game port).
    pub const ISA_EXT: u32 = 0x380000;
}

/// The I/O subsystem state.
///
/// Placeholder — will hold the MACE ASIC state (PCI, ISA, PS/2, Ethernet,
/// audio, SCSI) once milestone M3 begins.
#[derive(Debug, Default)]
pub struct Io {
    /// Whether the MACE ASIC is present.
    pub present: bool,
}

impl Io {
    /// Create a new I/O subsystem.
    pub fn new() -> Self {
        Self { present: true }
    }

    /// Reset the I/O subsystem.
    pub fn reset(&mut self) {
        *self = Self::default();
        self.present = true;
    }
}