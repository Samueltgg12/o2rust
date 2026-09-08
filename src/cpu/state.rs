//! Shared CPU state: general-purpose registers, FPU registers, and the
//! program counter / HI-LO special registers.

/// The architectural state of a MIPS CPU.
///
/// This is kept separate from the execution engine so it can be inspected by
/// the debugger and serialized for save states.
#[derive(Debug, Clone)]
pub struct CpuState {
    /// General-purpose registers `$0`–`$31`. `$0` is always zero.
    pub gpr: [u64; 32],
    /// Floating-point registers (32 single-precision / 16 double-precision).
    pub fpr: [u64; 32],
    /// Program counter.
    pub pc: u32,
    /// Next program counter (used for branch delay slots).
    pub next_pc: u32,
    /// Multiply/divide high result.
    pub hi: u64,
    /// Multiply/divide low result.
    pub lo: u64,
    /// FPU control/status register 0 (implementation/revision).
    pub fcr0: u32,
    /// FPU control/status register 31 (rounding mode, exception flags).
    pub fcr31: u32,
    /// Load-linked reservation bit.
    pub llbit: bool,
    /// Load-linked reservation address.
    pub lladdr: u32,
    /// Whether the next instruction is a branch delay slot.
    pub in_delay_slot: bool,
}

impl Default for CpuState {
    fn default() -> Self {
        Self {
            gpr: [0; 32],
            fpr: [0; 32],
            pc: crate::ip32::PROM_RESET_VECTOR,
            next_pc: crate::ip32::PROM_RESET_VECTOR + 4,
            hi: 0,
            lo: 0,
            fcr0: 0,
            fcr31: 0,
            llbit: false,
            lladdr: 0,
            in_delay_slot: false,
        }
    }
}

impl CpuState {
    /// Reset the CPU state to its power-on values.
    pub fn reset(&mut self) {
        *self = Self::default();
        // The PROM sets up a stack in kseg0; pre-seed a sane default.
        self.gpr[29] = 0x8000_0000; // $sp
        self.gpr[28] = 0x8000_0000; // $gp
    }

    /// Read a GPR, forcing `$0` to always return zero.
    pub fn gpr(&self, index: usize) -> u64 {
        if index == 0 {
            0
        } else {
            self.gpr[index]
        }
    }

    /// Write a GPR, ignoring writes to `$0`.
    pub fn set_gpr(&mut self, index: usize, value: u64) {
        if index != 0 {
            self.gpr[index] = value;
        }
    }
}