//! MIPS CPU emulation.
//!
//! Phase 2 milestone M2 targets the **MIPS R5000** core (integer, FPU,
//! MMU/TLB, caches) sufficient to execute the IP32 PROM. R10000/R12000
//! support is a later milestone.
//!
//! The CPU is split into:
//!
//! - [`cp0`] — the CP0 system control coprocessor (status, cause, EPC, TLB).
//! - [`r5000`] — the R5000 interpreter core.
//! - [`state`] — shared CPU state (GPRs, FPRs, PC, HI/LO).

pub mod cp0;
pub mod r5000;
pub mod state;

pub use cp0::Cp0;
pub use r5000::R5000;
pub use state::CpuState;

/// CPU model variants supported by the emulator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuModel {
    /// MIPS R5000 (default for the O2).
    R5000,
    /// MIPS R10000.
    R10000,
    /// MIPS R12000.
    R12000,
}

impl CpuModel {
    /// The human-readable name of this CPU model.
    pub fn name(self) -> &'static str {
        match self {
            CpuModel::R5000 => "MIPS R5000",
            CpuModel::R10000 => "MIPS R10000",
            CpuModel::R12000 => "MIPS R12000",
        }
    }
}