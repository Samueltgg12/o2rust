// src/cpu/cp0.rs
//! CP0 — the MIPS system control coprocessor.
//!
//! CP0 holds the processor status, cause, exception program counter, TLB, and
//! cache control registers. Register numbers and semantics follow the MIPS
//! R5000 (VR5000) architecture as documented in `docs/cpu-memory.md`.

use super::CpuState;

/// CP0 register numbers (select 0 unless noted).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Cp0Reg {
    Index = 0,
    Random = 1,
    EntryLo0 = 2,
    EntryLo1 = 3,
    Context = 4,
    PageMask = 5,
    Wired = 6,
    BadVAddr = 8,
    Count = 9,
    EntryHi = 10,
    Compare = 11,
    Status = 12,
    Cause = 13,
    Epc = 14,
    PrId = 15,
    Config = 16,
    LlAddr = 17,
    WatchLo = 18,
    WatchHi = 19,
    XContext = 20,
    Ecc = 26,
    CacheErr = 27,
    TagLo = 28,
    TagHi = 29,
    ErrorEpc = 30,
}

/// CP0 Status register bit fields.
pub mod status {
    /// Interrupt enable.
    pub const IE: u32 = 1 << 0;
    /// Exception level.
    pub const EXL: u32 = 1 << 1;
    /// Error level.
    pub const ERL: u32 = 1 << 2;
    /// Kernel/User mode (0 = kernel, 1 = user).
    pub const KSU: u32 = 0b11 << 3;
    /// User mode.
    pub const KSU_USER: u32 = 0b10 << 3;
    /// Supervisor mode.
    pub const KSU_SUPERVISOR: u32 = 0b01 << 3;
    /// Interrupt mask (8 bits).
    pub const IM: u32 = 0xff << 8;
    /// Boot exception vectors (1 = use `0xBFC0_0200`).
    pub const BEV: u32 = 1 << 22;
    /// TLB shutdown.
    pub const TS: u32 = 1 << 21;
    /// Soft reset.
    pub const SR: u32 = 1 << 20;
    /// Non-maskable interrupt.
    pub const NMI: u32 = 1 << 19;
    /// Coprocessor 1 (FPU) usable.
    pub const CU1: u32 = 1 << 29;
    /// Coprocessor 0 usable.
    pub const CU0: u32 = 1 << 28;
}

/// CP0 Cause register bit fields.
pub mod cause {
    /// Exception code (5 bits).
    pub const EXC_CODE: u32 = 0x1f << 2;
    /// Interrupt pending bits (8 bits).
    pub const IP: u32 = 0xff << 8;
    /// Branch delay slot indicator.
    pub const BD: u32 = 1 << 31;
}

/// MIPS exception codes (Cause.ExcCode).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ExceptionCode {
    Int = 0,
    Mod = 1,
    TlbLoad = 2,
    TlbStore = 3,
    AddrErrLoad = 4,
    AddrErrStore = 5,
    BusErrInstr = 6,
    BusErrData = 7,
    Syscall = 8,
    Breakpoint = 9,
    ReservedInstr = 10,
    CopUnusable = 11,
    Overflow = 12,
    Trap = 13,
    Fpe = 15,
}

/// A single TLB entry (48 entries on R5000).
///
/// Each TLB entry maps two pages (even/odd) and consists of:
/// - EntryHi: VPN2 (bits 31..13), ASID (bits 7..0)
/// - EntryLo0: PFN (bits 29..6), C (bits 5..3), D (bit 2), V (bit 1), G (bit 0) — even page
/// - EntryLo1: PFN (bits 29..6), C (bits 5..3), D (bit 2), V (bit 1), G (bit 0) — odd page
/// - PageMask: Mask (bits 28..13) — page size
#[derive(Debug, Clone, Copy, Default)]
pub struct TlbEntry {
    pub entry_hi: u32,
    pub entry_lo0: u32,
    pub entry_lo1: u32,
    pub page_mask: u32,
}

/// The CP0 coprocessor state.
#[derive(Debug, Clone)]
pub struct Cp0 {
    regs: [u32; 32],
    /// TLB entries (48 for R5000).
    tlb: [TlbEntry; 48],
    /// Index register (CP0 register 0) - index into TLB for TLBR/TLBWI.
    index: u32,
    /// Random register (CP0 register 1) - random index for TLBWR.
    random: u32,
    /// Wired register (CP0 register 6) - number of wired TLB entries.
    wired: u32,
}

impl Default for Cp0 {
    fn default() -> Self {
        Self::new()
    }
}

impl Cp0 {
    /// Create a new CP0 with power-on reset values.
    pub fn new() -> Self {
        let mut cp0 = Self {
            regs: [0; 32],
            tlb: [TlbEntry::default(); 48],
            index: 0,
            random: 31,
            wired: 0,
        };
        cp0.reset();
        cp0
    }

    /// Reset CP0 to power-on values.
    pub fn reset(&mut self) {
        self.regs = [0; 32];
        self.tlb = [TlbEntry::default(); 48];
        self.index = 0;
        self.random = 31;
        self.wired = 0;
        // BEV=1, TS=1, SR=0, NMI=0 — matches the O2Emu reference and the
        // MIPS reset state (boot exception vectors enabled).
        self.regs[Cp0Reg::Status as usize] = 0x0040_0004;
        // R5000 PRID: implementation 0x23 (R5000), revision 0.
        self.regs[Cp0Reg::PrId as usize] = 0x0000_2300;
        // Config register: K0=3 (cacheable), KU=0, K23=0
        self.regs[Cp0Reg::Config as usize] = 0x0006_E463;
    }

    /// Read a CP0 register.
    pub fn read(&self, reg: Cp0Reg) -> u32 {
        self.regs[reg as usize]
    }

    /// Write a CP0 register.
    pub fn write(&mut self, reg: Cp0Reg, value: u32) {
        self.regs[reg as usize] = value;
    }

    /// The Status register.
    pub fn status(&self) -> u32 {
        self.read(Cp0Reg::Status)
    }

    /// The Cause register.
    pub fn cause(&self) -> u32 {
        self.read(Cp0Reg::Cause)
    }

    /// The EPC (exception program counter) register.
    pub fn epc(&self) -> u32 {
        self.read(Cp0Reg::Epc)
    }

    /// Set the EPC register.
    pub fn set_epc(&mut self, value: u32) {
        self.write(Cp0Reg::Epc, value);
    }

    /// Whether interrupts are globally enabled (Status.IE set and not in an
    /// exception/error level).
    pub fn interrupts_enabled(&self) -> bool {
        let status = self.status();
        (status & status::IE) != 0 && (status & (status::EXL | status::ERL)) == 0
    }

    /// The interrupt mask (Status.IM).
    pub fn interrupt_mask(&self) -> u32 {
        (self.status() & status::IM) >> 8
    }

    /// The pending interrupt bits (Cause.IP).
    pub fn pending_interrupts(&self) -> u32 {
        (self.cause() & cause::IP) >> 8
    }

    /// Raise an interrupt line (0–7).
    pub fn raise_interrupt(&mut self, line: u8) {
        let cause = self.cause() | (1 << (8 + line));
        self.write(Cp0Reg::Cause, cause);
    }

    /// Clear an interrupt line (0–7).
    pub fn clear_interrupt(&mut self, line: u8) {
        let cause = self.cause() & !(1 << (8 + line));
        self.write(Cp0Reg::Cause, cause);
    }

    /// Take an exception: set Cause.ExcCode, save EPC, and enter the handler.
    ///
    /// Returns the exception vector address to jump to.
    pub fn take_exception(&mut self, state: &mut CpuState, code: ExceptionCode) -> u32 {
        let status = self.status();
        let bev = (status & status::BEV) != 0;

        // Save the current PC into EPC (adjusting for the delay slot).
        let epc = if state.in_delay_slot {
            state.pc.wrapping_sub(4)
        } else {
            state.pc
        };
        self.set_epc(epc);

        // Set Cause.ExcCode and Cause.BD.
        let mut cause = self.cause() & !cause::EXC_CODE;
        cause |= (code as u32) << 2;
        if state.in_delay_slot {
            cause |= cause::BD;
        }
        self.write(Cp0Reg::Cause, cause);

        // Enter exception level.
        self.write(Cp0Reg::Status, status | status::EXL);

        // Exception vector: BEV selects the boot vector.
        if bev {
            0xBFC0_0380 // Boot exception vector (general exception)
        } else {
            0x8000_0180 // Normal exception vector
        }
    }

    /// Return from exception (ERET): restore Status and jump to EPC.
    pub fn eret(&mut self) -> u32 {
        let status = self.status();
        // Clear EXL (or ERL if set).
        let new_status = if (status & status::ERL) != 0 {
            status & !status::ERL
        } else {
            status & !status::EXL
        };
        self.write(Cp0Reg::Status, new_status);
        self.epc()
    }

    /// Increment the Count register (called once per cycle).
    pub fn tick(&mut self) {
        self.regs[Cp0Reg::Count as usize] = self.regs[Cp0Reg::Count as usize].wrapping_add(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reset_values() {
        let cp0 = Cp0::new();
        assert_eq!(cp0.status(), 0x0040_0004);
        assert_eq!(cp0.read(Cp0Reg::PrId), 0x0000_2300);
        assert_eq!(cp0.read(Cp0Reg::Random), 31);
    }

    #[test]
    fn exception_sets_epc_and_exl() {
        let mut cp0 = Cp0::new();
        let mut state = CpuState::default();
        state.pc = 0x8000_1000;
        let vector = cp0.take_exception(&mut state, ExceptionCode::Syscall);
        assert_eq!(cp0.epc(), 0x8000_1000);
        assert_eq!(cp0.status() & status::EXL, status::EXL);
        // BEV is set at reset, so we use the boot vector.
        assert_eq!(vector, 0xBFC0_0380);
    }
}