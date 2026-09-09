// src/cpu/r5000.rs
//! MIPS R5000 interpreter core.
//!
//! A single-issue, in-order interpreter for the MIPS IV ISA subset used by the
//! IP32 PROM. This is the reference implementation for milestone M2 (CPU +
//! memory execute the PROM). A JIT (Cranelift) is planned for Phase 3.
//!
//! The core is decoupled from the memory system via the [`MemoryAccess`]
//! trait, so it can be driven by the system bus or a test harness.

use super::{Cp0, CpuState};
use crate::cpu::cp0::{Cp0Reg, ExceptionCode};
use crate::log;

/// Memory access interface used by the CPU to fetch instructions and read/write
/// data. Implemented by the system bus.
pub trait MemoryAccess {
    /// Read a 32-bit word (big-endian).
    fn read32(&mut self, addr: u32) -> u32;
    /// Read a 16-bit halfword (big-endian).
    fn read16(&mut self, addr: u32) -> u16;
    /// Read a byte.
    fn read8(&mut self, addr: u32) -> u8;
    /// Read a 64-bit doubleword (big-endian).
    fn read64(&mut self, addr: u32) -> u64;
    /// Write a 32-bit word (big-endian).
    fn write32(&mut self, addr: u32, value: u32);
    /// Write a 16-bit halfword (big-endian).
    fn write16(&mut self, addr: u32, value: u16);
    /// Write a byte.
    fn write8(&mut self, addr: u32, value: u8);
    /// Write a 64-bit doubleword (big-endian).
    fn write64(&mut self, addr: u32, value: u64);
}

/// The MIPS R5000 CPU core.
pub struct R5000 {
    /// Architectural state (GPRs, FPRs, PC, HI/LO).
    pub state: CpuState,
    /// CP0 system control coprocessor.
    pub cp0: Cp0,
    /// Number of cycles executed.
    cycles: u64,
    /// Stop flag (set by `stop()`).
    stop_requested: bool,
}

impl Default for R5000 {
    fn default() -> Self {
        Self::new()
    }
}

impl R5000 {
    /// Create a new R5000 core in its reset state.
    pub fn new() -> Self {
        let mut state = CpuState::default();
        state.reset();
        Self {
            state,
            cp0: Cp0::new(),
            cycles: 0,
            stop_requested: false,
        }
    }

    /// Reset the CPU and set the program counter to `reset_vector`.
    pub fn reset(&mut self, reset_vector: u32) {
        self.state.reset();
        self.state.pc = reset_vector;
        self.state.next_pc = reset_vector.wrapping_add(4);
        self.cp0.reset();
        self.cycles = 0;
        self.stop_requested = false;
        log::info_msg(&format!("R5000 reset, PC = 0x{reset_vector:08x}"));
    }

    /// The number of cycles executed so far.
    pub fn cycles(&self) -> u64 {
        self.cycles
    }

    /// Request the CPU to stop executing.
    pub fn stop(&mut self) {
        self.stop_requested = true;
    }

    /// Execute a single instruction.
    pub fn step(&mut self, mem: &mut dyn MemoryAccess) {
        if self.stop_requested {
            return;
        }

        let pc = self.state.pc;
        let instr = mem.read32(pc);

        // Default: advance to the next instruction. Branch/jump handlers
        // override `next_pc`.
        self.state.next_pc = pc.wrapping_add(4);
        self.state.in_delay_slot = false;

        // Handle branch likely nullification: if the previous branch likely
        // was not taken, the delay slot instruction is nullified (treated as NOP).
        if self.state.nullify_delay_slot {
            self.state.nullify_delay_slot = false;
            // Skip execution - treat as NOP
        } else {
            self.execute(instr, mem);
        }

        // Commit the PC.
        self.state.pc = self.state.next_pc;

        // CP0 Count increments once per cycle.
        self.cp0.tick();

        // $zero is always zero.
        self.state.gpr[0] = 0;

        self.cycles += 1;
    }

    /// Run for `n` cycles (or until `stop()` is called).
    pub fn run(&mut self, mem: &mut dyn MemoryAccess, n: u64) {
        for _ in 0..n {
            if self.stop_requested {
                break;
            }
            self.step(mem);
        }
    }

    /// Run until the program counter reaches `target_pc`.
    pub fn run_until(&mut self, mem: &mut dyn MemoryAccess, target_pc: u32) {
        while self.state.pc != target_pc && !self.stop_requested {
            self.step(mem);
        }
    }

    /// Decode and execute a single instruction.
    fn execute(&mut self, instr: u32, mem: &mut dyn MemoryAccess) {
        let opcode = (instr >> 26) & 0x3f;
        match opcode {
            0x00 => self.execute_special(instr, mem),
            0x01 => self.execute_regimm(instr),
            0x02 => self.jump(instr, false),
            0x03 => self.jump(instr, true),
            0x04 => self.branch(instr, true, false),
            0x05 => self.branch(instr, false, false),
            0x06 => self.branch_zero(instr, true, false),
            0x07 => self.branch_zero(instr, false, false),
            0x14 => self.branch_zero(instr, true, true),   // BEQZL - Branch on Equal Zero Likely
            0x15 => self.branch_zero(instr, false, true),  // BNEZL - Branch on Not Equal Zero Likely
            0x08 => self.addi(instr, true),
            0x09 => self.addi(instr, false),
            0x0a => self.slti(instr, true),
            0x0b => self.slti(instr, false),
            0x0c => self.andi(instr),
            0x0d => self.ori(instr),
            0x0e => self.xori(instr),
            0x0f => self.lui(instr),
            0x10 => self.cop0(instr),
            0x11 => self.cop1(instr),
            0x18 => self.daddi(instr, true),   // DADDI - Doubleword Add Immediate
            0x19 => self.daddi(instr, false),  // DADDIU - Doubleword Add Immediate Unsigned
            0x1a => self.load_left(instr, mem, 8),  // LDL - Load Doubleword Left
            0x1b => self.load_right(instr, mem, 8), // LDR - Load Doubleword Right
            0x1e => self.load_quad(instr, mem),     // LQ - Load Quadword (MIPS IV)
            0x1f => self.store_quad(instr, mem),    // SQ - Store Quadword (MIPS IV)
            0x20 => self.load(instr, mem, 1, true),
            0x21 => self.load(instr, mem, 2, true),
            0x22 => self.load_left(instr, mem, 4),
            0x23 => self.load(instr, mem, 4, true),
            0x24 => self.load(instr, mem, 1, false),
            0x25 => self.load(instr, mem, 2, false),
            0x26 => self.load_right(instr, mem, 4),
            0x27 => self.load(instr, mem, 4, false), // LWU - Load Word Unsigned
            0x28 => self.store(instr, mem, 1),
            0x29 => self.store(instr, mem, 2),
            0x2a => self.store_left(instr, mem, 4),
            0x2b => self.store(instr, mem, 4),
            0x2c => self.store_left(instr, mem, 8),  // SDL - Store Doubleword Left
            0x2d => self.store_right(instr, mem, 8), // SDR - Store Doubleword Right
            0x2e => self.store_right(instr, mem, 4),
            0x2f => self.execute_cache(instr, mem),
            0x30 => self.load(instr, mem, 4, true), // LL (simplified)
            0x34 => self.load(instr, mem, 8, true), // LLD - Load Linked Doubleword
            0x35 => self.load_fp(instr, mem),       // LDC1 - Load Doubleword to Coprocessor 1
            0x37 => self.load(instr, mem, 8, true), // LD (Load Doubleword, 64-bit)
            0x38 => self.store(instr, mem, 4),      // SC (simplified)
            0x3c => self.store(instr, mem, 8),      // SCD - Store Conditional Doubleword
            0x3d => self.store_fp(instr, mem),      // SDC1 - Store Doubleword from Coprocessor 1
            0x3f => self.store(instr, mem, 8),      // SD (Store Doubleword, 64-bit)
            0x31 => self.load_fp(instr, mem),
            0x39 => self.store_fp(instr, mem),
            _ => {
                log::warn_msg(&format!(
                    "Unimplemented opcode 0x{opcode:02x} at PC 0x{:08x}",
                    self.state.pc
                ));
                self.exception(ExceptionCode::ReservedInstr);
            }
        }
    }

    /// Execute the CACHE instruction (opcode 0x2f).
    ///
    /// The CACHE instruction performs cache operations on the primary instruction
    /// cache, primary data cache, or secondary cache. The operation is encoded
    /// in bits 20:16 of the instruction:
    /// - bits 17:16 (op[1:0]): target cache (0=I-cache, 1=D-cache, 2=Secondary, 3=D-cache)
    /// - bits 20:18 (op[4:2]): operation
    ///   0 = Index_Invalidate / Index_Writeback_Invalidate / Flash
    ///   1 = Index_Load_Tag
    ///   2 = Index_Store_Tag
    ///   3 = Create_Dirty_Exclusive
    ///   4 = Hit_Invalidate
    ///   5 = Fill / Hit_Writeback_Invalidate / Page_Invalidate
    ///   6 = Hit_Writeback
    ///   7 = Hit_Writeback (alternate)
    ///
    /// The address is computed as: base_register + sign_extended_offset
    fn execute_cache(&mut self, instr: u32, _mem: &mut dyn MemoryAccess) {
        let rs = ((instr >> 21) & 0x1f) as usize;
        let offset = (instr & 0xffff) as i16 as i32 as u32;
        let base = self.state.gpr(rs) as u32;
        let addr = base.wrapping_add(offset);

        // Extract cache operation (bits 20:16)
        let op = (instr >> 16) & 0x1f;
        let target_cache = op & 0x3;        // bits 1:0
        let operation = (op >> 2) & 0x7;    // bits 4:2

        log::debug_msg(&format!(
            "CACHE: op=0x{:02x}, target_cache={}, operation={}, addr=0x{:08x}, base_reg=${}, offset=0x{:04x}",
            op, target_cache, operation, addr, rs, offset
        ));

        // For now, implement as no-op with logging. Full cache simulation
        // requires a cache model which is not yet implemented.
        // The PROM uses CACHE instructions for cache initialization and
        // management during boot.
        match (target_cache, operation) {
            // Index operations - use address to index into cache
            (0, 0) => { /* Index_Invalidate_I */ }
            (1, 0) => { /* Index_Writeback_Invalidate_D */ }
            (2, 0) => { /* Index_Writeback_Invalidate_S / Flash */ }
            (3, 0) => { /* Index_Writeback_Invalidate_D (alt) */ }

            // Index_Load_Tag - load cache tag into CP0 TagLo/TagHi
            (0, 1) => { /* Index_Load_Tag_I */ }
            (1, 1) => { /* Index_Load_Tag_D */ }
            (2, 1) => { /* Index_Load_Tag_S */ }
            (3, 1) => { /* Index_Load_Tag_D (alt) */ }

            // Index_Store_Tag - store CP0 TagLo/TagHi into cache tag
            (0, 2) => { /* Index_Store_Tag_I */ }
            (1, 2) => { /* Index_Store_Tag_D */ }
            (2, 2) => { /* Index_Store_Tag_S */ }
            (3, 2) => { /* Index_Store_Tag_D (alt) */ }

            // Create_Dirty_Exclusive
            (1, 3) => { /* Create_Dirty_Exclusive_D */ }
            (3, 3) => { /* Create_Dirty_Exclusive_D (alt) */ }

            // Hit operations - use address to check for cache hit
            (0, 4) => { /* Hit_Invalidate_I */ }
            (1, 4) => { /* Hit_Invalidate_D */ }
            (2, 4) => { /* Hit_Invalidate_S */ }
            (3, 4) => { /* Hit_Invalidate_D (alt) */ }

            (1, 5) => { /* Hit_Writeback_Invalidate_D / Fill */ }
            (2, 5) => { /* Hit_Writeback_Invalidate_S / Page_Invalidate */ }
            (3, 5) => { /* Hit_Writeback_Invalidate_D (alt) */ }

            (1, 6) => { /* Hit_Writeback_D */ }
            (2, 6) => { /* Hit_Writeback_S */ }
            (3, 6) => { /* Hit_Writeback_D (alt) */ }

            (1, 7) => { /* Hit_Writeback_D (alt) */ }
            (3, 7) => { /* Hit_Writeback_D (alt) */ }

            _ => {
                log::warn_msg(&format!(
                    "Unknown CACHE operation: target_cache={}, operation={} at PC 0x{:08x}",
                    target_cache, operation, self.state.pc
                ));
            }
        }
    }

    // === SPECIAL (opcode 0x00) ===
    fn execute_special(&mut self, instr: u32, _mem: &mut dyn MemoryAccess) {
        let funct = instr & 0x3f;
        let rs = ((instr >> 21) & 0x1f) as usize;
        let rt = ((instr >> 16) & 0x1f) as usize;
        let rd = ((instr >> 11) & 0x1f) as usize;
        let shamt = (instr >> 6) & 0x1f;

        match funct {
            0x00 => {
                // SLL
                let v = self.state.gpr(rt) << shamt;
                self.state.set_gpr(rd, v);
            }
            0x02 => {
                // SRL
                let v = self.state.gpr(rt) >> shamt;
                self.state.set_gpr(rd, v);
            }
            0x03 => {
                // SRA
                let v = ((self.state.gpr(rt) as i64) >> shamt) as u64;
                self.state.set_gpr(rd, v);
            }
            0x04 => {
                // SLLV
                let s = (self.state.gpr(rs) & 0x1f) as u32;
                let v = self.state.gpr(rt) << s;
                self.state.set_gpr(rd, v);
            }
            0x06 => {
                // SRLV
                let s = (self.state.gpr(rs) & 0x1f) as u32;
                let v = self.state.gpr(rt) >> s;
                self.state.set_gpr(rd, v);
            }
            0x07 => {
                // SRAV
                let s = (self.state.gpr(rs) & 0x1f) as u32;
                let v = ((self.state.gpr(rt) as i64) >> s) as u64;
                self.state.set_gpr(rd, v);
            }
            0x08 => {
                // JR
                self.state.next_pc = self.state.gpr(rs) as u32;
            }
            0x09 => {
                // JALR
                let target = self.state.gpr(rs) as u32;
                self.state.set_gpr(rd, self.state.pc.wrapping_add(8) as u64);
                self.state.next_pc = target;
            }
            0x0c => self.exception(ExceptionCode::Syscall),
            0x0d => self.exception(ExceptionCode::Breakpoint),
            0x0f => {} // SYNC — no-op
            0x10 => {
                // MFHI
                let v = self.state.hi;
                self.state.set_gpr(rd, v);
            }
            0x11 => {
                // MTHI
                self.state.hi = self.state.gpr(rs);
            }
            0x12 => {
                // MFLO
                let v = self.state.lo;
                self.state.set_gpr(rd, v);
            }
            0x13 => {
                // MTLO
                self.state.lo = self.state.gpr(rs);
            }
            0x18 => {
                // MULT
                let a = self.state.gpr(rs) as i32 as i64;
                let b = self.state.gpr(rt) as i32 as i64;
                let result = a.wrapping_mul(b) as u64;
                self.state.lo = (result as u32) as u64;
                self.state.hi = ((result >> 32) as u32) as u64;
            }
            0x19 => {
                // MULTU
                let a = self.state.gpr(rs) as u32 as u64;
                let b = self.state.gpr(rt) as u32 as u64;
                let result = a.wrapping_mul(b);
                self.state.lo = (result as u32) as u64;
                self.state.hi = ((result >> 32) as u32) as u64;
            }
            0x1a => {
                // DIV
                let a = self.state.gpr(rs) as i32;
                let b = self.state.gpr(rt) as i32;
                if b != 0 {
                    self.state.lo = (a.wrapping_div(b)) as u32 as u64;
                    self.state.hi = (a.wrapping_rem(b)) as u32 as u64;
                }
            }
            0x1b => {
                // DIVU
                let a = self.state.gpr(rs) as u32;
                let b = self.state.gpr(rt) as u32;
                if b != 0 {
                    self.state.lo = (a / b) as u64;
                    self.state.hi = (a % b) as u64;
                }
            }
            0x16 => {
                // DSRLV - Doubleword Shift Right Logical Variable (MIPS III)
                let s = (self.state.gpr(rs) & 0x3f) as u32;
                let v = self.state.gpr(rt) >> s;
                self.state.set_gpr(rd, v);
            }
            0x17 => {
                // DSRAV - Doubleword Shift Right Arithmetic Variable (MIPS III)
                let s = (self.state.gpr(rs) & 0x3f) as u32;
                let v = ((self.state.gpr(rt) as i64) >> s) as u64;
                self.state.set_gpr(rd, v);
            }
            0x1e => {
                // DDIV - Doubleword Divide (MIPS III)
                let a = self.state.gpr(rs) as i64;
                let b = self.state.gpr(rt) as i64;
                if b != 0 {
                    self.state.lo = (a.wrapping_div(b)) as u64;
                    self.state.hi = (a.wrapping_rem(b)) as u64;
                }
            }
            0x1f => {
                // DDIVU - Doubleword Divide Unsigned (MIPS III)
                let a = self.state.gpr(rs);
                let b = self.state.gpr(rt);
                if b != 0 {
                    self.state.lo = a / b;
                    self.state.hi = a % b;
                }
            }
            0x20 => {
                // ADD
                let v = self.state.gpr(rs).wrapping_add(self.state.gpr(rt));
                self.state.set_gpr(rd, v);
            }
            0x21 => {
                // ADDU
                let v = self.state.gpr(rs).wrapping_add(self.state.gpr(rt));
                self.state.set_gpr(rd, v);
            }
            0x22 => {
                // SUB
                let v = self.state.gpr(rs).wrapping_sub(self.state.gpr(rt));
                self.state.set_gpr(rd, v);
            }
            0x23 => {
                // SUBU
                let v = self.state.gpr(rs).wrapping_sub(self.state.gpr(rt));
                self.state.set_gpr(rd, v);
            }
            0x24 => {
                // AND
                let v = self.state.gpr(rs) & self.state.gpr(rt);
                self.state.set_gpr(rd, v);
            }
            0x25 => {
                // OR
                let v = self.state.gpr(rs) | self.state.gpr(rt);
                self.state.set_gpr(rd, v);
            }
            0x26 => {
                // XOR
                let v = self.state.gpr(rs) ^ self.state.gpr(rt);
                self.state.set_gpr(rd, v);
            }
            0x27 => {
                // NOR
                let v = !(self.state.gpr(rs) | self.state.gpr(rt));
                self.state.set_gpr(rd, v);
            }
            0x2a => {
                // SLT
                let v = ((self.state.gpr(rs) as i64) < (self.state.gpr(rt) as i64)) as u64;
                self.state.set_gpr(rd, v);
            }
            0x2b => {
                // SLTU
                let v = (self.state.gpr(rs) < self.state.gpr(rt)) as u64;
                self.state.set_gpr(rd, v);
            }
            0x0a => {
                // MOVZ (MIPS IV) - Move if Zero
                if self.state.gpr(rt) == 0 {
                    self.state.set_gpr(rd, self.state.gpr(rs));
                }
            }
            0x0b => {
                // MOVN (MIPS IV) - Move if Not Zero
                if self.state.gpr(rt) != 0 {
                    self.state.set_gpr(rd, self.state.gpr(rs));
                }
            }
            0x14 => {
                // DSLLV (MIPS III) - Doubleword Shift Left Logical Variable
                let s = (self.state.gpr(rs) & 0x3f) as u32;
                let v = self.state.gpr(rt) << s;
                self.state.set_gpr(rd, v);
            }
            0x1c => {
                // MADD (MIPS IV) - Multiply-Add
                let a = self.state.gpr(rs) as i64;
                let b = self.state.gpr(rt) as i64;
                let result = a.wrapping_mul(b) as i128;
                let (new_lo, new_hi) = self.madd_maddu(result, true);
                self.state.lo = new_lo;
                self.state.hi = new_hi;
            }
            0x1d => {
                // MADDU (MIPS IV) - Multiply-Add Unsigned
                let a = self.state.gpr(rs) as u64;
                let b = self.state.gpr(rt) as u64;
                let result = a.wrapping_mul(b) as i128;
                let (new_lo, new_hi) = self.madd_maddu(result, false);
                self.state.lo = new_lo;
                self.state.hi = new_hi;
            }
            0x28 => {
                // MFSA (MIPS IV) - Move from SA (Shift Amount register)
                // SA register is typically the shift amount from previous shift instruction
                // For now, we'll use a dedicated field or derive from context
                // In MIPS IV, SA is a special register; we'll store it in the CPU state
                self.state.set_gpr(rd, self.state.sa);
            }
            0x29 => {
                // MTSA (MIPS IV) - Move to SA (Shift Amount register)
                self.state.sa = self.state.gpr(rs) & 0x3f;
            }
            0x2c => {
                // DADD (MIPS III) - Doubleword Add (signed, traps on overflow)
                let a = self.state.gpr(rs) as i64;
                let b = self.state.gpr(rt) as i64;
                let (result, overflow) = a.overflowing_add(b);
                if overflow {
                    self.exception(ExceptionCode::Overflow);
                } else {
                    self.state.set_gpr(rd, result as u64);
                }
            }
            0x2d => {
                // DADDU (MIPS III) - Doubleword Add Unsigned
                let v = self.state.gpr(rs).wrapping_add(self.state.gpr(rt));
                self.state.set_gpr(rd, v);
            }
            0x2e => {
                // DSUB (MIPS III) - Doubleword Subtract (signed, traps on overflow)
                let a = self.state.gpr(rs) as i64;
                let b = self.state.gpr(rt) as i64;
                let (result, overflow) = a.overflowing_sub(b);
                if overflow {
                    self.exception(ExceptionCode::Overflow);
                } else {
                    self.state.set_gpr(rd, result as u64);
                }
            }
            0x2f => {
                // DSUBU (MIPS III) - Doubleword Subtract Unsigned
                let v = self.state.gpr(rs).wrapping_sub(self.state.gpr(rt));
                self.state.set_gpr(rd, v);
            }
            0x30 => {
                // TGE (MIPS II+) - Trap if Greater or Equal (signed)
                if (self.state.gpr(rs) as i64) >= (self.state.gpr(rt) as i64) {
                    self.exception(ExceptionCode::Trap);
                }
            }
            0x31 => {
                // TGEU (MIPS II+) - Trap if Greater or Equal (unsigned)
                if self.state.gpr(rs) >= self.state.gpr(rt) {
                    self.exception(ExceptionCode::Trap);
                }
            }
            0x32 => {
                // TLT (MIPS II+) - Trap if Less Than (signed)
                if (self.state.gpr(rs) as i64) < (self.state.gpr(rt) as i64) {
                    self.exception(ExceptionCode::Trap);
                }
            }
            0x33 => {
                // TLTU (MIPS II+) - Trap if Less Than (unsigned)
                if self.state.gpr(rs) < self.state.gpr(rt) {
                    self.exception(ExceptionCode::Trap);
                }
            }
            0x34 => {
                // TEQ (MIPS II+) - Trap if Equal
                if self.state.gpr(rs) == self.state.gpr(rt) {
                    self.exception(ExceptionCode::Trap);
                }
            }
            0x36 => {
                // TNE (MIPS II+) - Trap if Not Equal
                if self.state.gpr(rs) != self.state.gpr(rt) {
                    self.exception(ExceptionCode::Trap);
                }
            }
            0x38 => {
                // DSLL (MIPS III) - Doubleword Shift Left Logical
                let v = self.state.gpr(rt) << shamt;
                self.state.set_gpr(rd, v);
            }
            0x3a => {
                // DSRL (MIPS III) - Doubleword Shift Right Logical
                let v = self.state.gpr(rt) >> shamt;
                self.state.set_gpr(rd, v);
            }
            0x3b => {
                // DSRA (MIPS III) - Doubleword Shift Right Arithmetic
                let v = ((self.state.gpr(rt) as i64) >> shamt) as u64;
                self.state.set_gpr(rd, v);
            }
            0x3c => {
                // DSLL32 - Doubleword Shift Left Logical + 32 (MIPS IV)
                // Shift amount = shamt + 32 (range 32-63)
                let shift = (shamt + 32) as u32;
                let v = self.state.gpr(rt) << shift;
                self.state.set_gpr(rd, v);
            }
            0x3e => {
                // DSRL32 - Doubleword Shift Right Logical + 32 (MIPS IV)
                // Shift amount = shamt + 32 (range 32-63)
                let shift = (shamt + 32) as u32;
                let v = self.state.gpr(rt) >> shift;
                self.state.set_gpr(rd, v);
            }
            0x3f => {
                // DSRA32 - Doubleword Shift Right Arithmetic + 32 (MIPS IV)
                // Shift amount = shamt + 32 (range 32-63)
                let shift = (shamt + 32) as u32;
                let v = ((self.state.gpr(rt) as i64) >> shift) as u64;
                self.state.set_gpr(rd, v);
            }
            _ => {
                log::warn_msg(&format!(
                    "Unimplemented SPECIAL funct 0x{funct:02x} at PC 0x{:08x}",
                    self.state.pc
                ));
                self.exception(ExceptionCode::ReservedInstr);
            }
        }
    }

    // === REGIMM (opcode 0x01) ===
    fn execute_regimm(&mut self, instr: u32) {
        let rt = (instr >> 16) & 0x1f;
        let rs = ((instr >> 21) & 0x1f) as usize;
        let imm = (instr & 0xffff) as i16 as i32;

        match rt {
            0x00 => {
                // BLTZ - Branch on Less Than Zero
                if (self.state.gpr(rs) as i64) < 0 {
                    self.branch_offset(imm);
                }
            }
            0x01 => {
                // BGEZ - Branch on Greater Than or Equal Zero
                if (self.state.gpr(rs) as i64) >= 0 {
                    self.branch_offset(imm);
                }
            }
            0x02 => {
                // BLTZL - Branch on Less Than Zero Likely
                if (self.state.gpr(rs) as i64) < 0 {
                    self.branch_offset(imm);
                } else {
                    // Skip delay slot
                    self.state.pc = self.state.pc.wrapping_add(4);
                }
            }
            0x03 => {
                // BGEZL - Branch on Greater Than or Equal Zero Likely
                if (self.state.gpr(rs) as i64) >= 0 {
                    self.branch_offset(imm);
                } else {
                    // Skip delay slot
                    self.state.pc = self.state.pc.wrapping_add(4);
                }
            }
            0x10 => {
                // BLTZAL - Branch on Less Than Zero And Link
                self.state.set_gpr(31, self.state.pc.wrapping_add(8) as u64);
                if (self.state.gpr(rs) as i64) < 0 {
                    self.branch_offset(imm);
                }
            }
            0x11 => {
                // BGEZAL - Branch on Greater Than or Equal Zero And Link
                self.state.set_gpr(31, self.state.pc.wrapping_add(8) as u64);
                if (self.state.gpr(rs) as i64) >= 0 {
                    self.branch_offset(imm);
                }
            }
            0x12 => {
                // BLTZALL - Branch on Less Than Zero And Link Likely
                self.state.set_gpr(31, self.state.pc.wrapping_add(8) as u64);
                if (self.state.gpr(rs) as i64) < 0 {
                    self.branch_offset(imm);
                } else {
                    // Skip delay slot
                    self.state.pc = self.state.pc.wrapping_add(4);
                }
            }
            0x13 => {
                // BGEZALL - Branch on Greater Than or Equal Zero And Link Likely
                self.state.set_gpr(31, self.state.pc.wrapping_add(8) as u64);
                if (self.state.gpr(rs) as i64) >= 0 {
                    self.branch_offset(imm);
                } else {
                    // Skip delay slot
                    self.state.pc = self.state.pc.wrapping_add(4);
                }
            }
            0x08 => {
                // TGEI - Trap if Greater Than or Equal Immediate (MIPS III)
                if (self.state.gpr(rs) as i64) >= (imm as i64) {
                    self.exception(ExceptionCode::Trap);
                }
            }
            0x09 => {
                // TGEIU - Trap if Greater Than or Equal Immediate Unsigned (MIPS III)
                if self.state.gpr(rs) >= (imm as u32) as u64 {
                    self.exception(ExceptionCode::Trap);
                }
            }
            0x0a => {
                // TLTI - Trap if Less Than Immediate (MIPS III)
                if (self.state.gpr(rs) as i64) < (imm as i64) {
                    self.exception(ExceptionCode::Trap);
                }
            }
            0x0b => {
                // TLTIU - Trap if Less Than Immediate Unsigned (MIPS III)
                if self.state.gpr(rs) < (imm as u32) as u64 {
                    self.exception(ExceptionCode::Trap);
                }
            }
            0x0c => {
                // TEQI - Trap if Equal Immediate (MIPS III)
                if self.state.gpr(rs) == (imm as u32) as u64 {
                    self.exception(ExceptionCode::Trap);
                }
            }
            0x0e => {
                // TNEI - Trap if Not Equal Immediate (MIPS III)
                if self.state.gpr(rs) != (imm as u32) as u64 {
                    self.exception(ExceptionCode::Trap);
                }
            }
            _ => {
                log::warn_msg(&format!(
                    "Unimplemented REGIMM rt 0x{rt:02x} at PC 0x{:08x}",
                    self.state.pc
                ));
                self.exception(ExceptionCode::ReservedInstr);
            }
        }
    }

    // === Jumps and branches ===
    fn jump(&mut self, instr: u32, link: bool) {
        let target = instr & 0x03ff_ffff;
        if link {
            self.state.set_gpr(31, self.state.pc.wrapping_add(8) as u64);
        }
        self.state.next_pc = (self.state.pc & 0xf000_0000) | (target << 2);
    }

    fn branch(&mut self, instr: u32, eq: bool, _likely: bool) {
        let rs = ((instr >> 21) & 0x1f) as usize;
        let rt = ((instr >> 16) & 0x1f) as usize;
        let imm = (instr & 0xffff) as i16 as i32;
        let equal = self.state.gpr(rs) == self.state.gpr(rt);
        if equal == eq {
            self.branch_offset(imm);
        }
    }

    fn branch_zero(&mut self, instr: u32, lez: bool, likely: bool) {
        let rs = ((instr >> 21) & 0x1f) as usize;
        let imm = (instr & 0xffff) as i16 as i32;
        let val = self.state.gpr(rs) as i64;
        let take = if lez { val <= 0 } else { val > 0 };
        if take {
            self.branch_offset(imm);
        } else if likely {
            // Branch likely not taken: nullify the delay slot instruction
            self.state.nullify_delay_slot = true;
        }
    }

    fn branch_offset(&mut self, imm: i32) {
        self.state.next_pc = self.state.pc.wrapping_add(4).wrapping_add((imm << 2) as u32);
    }

    // === Immediate arithmetic ===
    fn addi(&mut self, instr: u32, _signed: bool) {
        let rs = ((instr >> 21) & 0x1f) as usize;
        let rt = ((instr >> 16) & 0x1f) as usize;
        let imm = (instr & 0xffff) as i16 as i32 as i64;
        let v = self.state.gpr(rs).wrapping_add(imm as u64);
        self.state.set_gpr(rt, v);
    }

    fn slti(&mut self, instr: u32, signed: bool) {
        let rs = ((instr >> 21) & 0x1f) as usize;
        let rt = ((instr >> 16) & 0x1f) as usize;
        let imm = (instr & 0xffff) as i16 as i32;
        let v = if signed {
            ((self.state.gpr(rs) as i64) < imm as i64) as u64
        } else {
            (self.state.gpr(rs) < (imm as u32 as u64)) as u64
        };
        self.state.set_gpr(rt, v);
    }

    fn andi(&mut self, instr: u32) {
        let rs = ((instr >> 21) & 0x1f) as usize;
        let rt = ((instr >> 16) & 0x1f) as usize;
        let imm = (instr & 0xffff) as u64;
        let v = self.state.gpr(rs) & imm;
        self.state.set_gpr(rt, v);
    }

    fn ori(&mut self, instr: u32) {
        let rs = ((instr >> 21) & 0x1f) as usize;
        let rt = ((instr >> 16) & 0x1f) as usize;
        let imm = (instr & 0xffff) as u64;
        let v = self.state.gpr(rs) | imm;
        self.state.set_gpr(rt, v);
    }

    fn xori(&mut self, instr: u32) {
        let rs = ((instr >> 21) & 0x1f) as usize;
        let rt = ((instr >> 16) & 0x1f) as usize;
        let imm = (instr & 0xffff) as u64;
        let v = self.state.gpr(rs) ^ imm;
        self.state.set_gpr(rt, v);
    }

    fn lui(&mut self, instr: u32) {
        let rt = ((instr >> 16) & 0x1f) as usize;
        let imm = (instr & 0xffff) as u64;
        self.state.set_gpr(rt, imm << 16);
    }

    // === Loads and stores ===
    fn load(&mut self, instr: u32, mem: &mut dyn MemoryAccess, size: u32, sign_extend: bool) {
        let rs = ((instr >> 21) & 0x1f) as usize;
        let rt = ((instr >> 16) & 0x1f) as usize;
        let imm = (instr & 0xffff) as i16 as i32;
        let addr = self.state.gpr(rs).wrapping_add(imm as u64) as u32;

        let value = match size {
            1 => mem.read8(addr) as u64,
            2 => mem.read16(addr) as u64,
            4 => mem.read32(addr) as u64,
            8 => mem.read64(addr),
            _ => mem.read32(addr) as u64,
        };

        let value = if sign_extend {
            match size {
                1 => (value as u8 as i8 as i64) as u64,
                2 => (value as u16 as i16 as i64) as u64,
                4 => (value as u32 as i32 as i64) as u64,
                8 => value, // 64-bit is already full width
                _ => value,
            }
        } else {
            value
        };

        self.state.set_gpr(rt, value);
    }

    fn load_left(&mut self, instr: u32, mem: &mut dyn MemoryAccess, size: u32) {
        let rs = ((instr >> 21) & 0x1f) as usize;
        let rt = ((instr >> 16) & 0x1f) as usize;
        let imm = (instr & 0xffff) as i16 as i32;
        let addr = self.state.gpr(rs).wrapping_add(imm as u64) as u32;
        
        match size {
            4 => {
                let aligned = addr & !3;
                let word = mem.read32(aligned) as u64;
                let shift = (addr & 3) * 8;
                let mask = 0xffff_ffff_ffff_ffffu64 << (32 - shift);
                let merged = (self.state.gpr(rt) & !mask) | (word & mask);
                self.state.set_gpr(rt, merged);
            }
            8 => {
                let aligned = addr & !7;
                let dword = mem.read64(aligned);
                let shift = (addr & 7) * 8;
                let mask = 0xffff_ffff_ffff_ffffu64 << (64 - shift);
                let merged = (self.state.gpr(rt) & !mask) | (dword & mask);
                self.state.set_gpr(rt, merged);
            }
            _ => {
                let aligned = addr & !3;
                let word = mem.read32(aligned) as u64;
                let shift = (addr & 3) * 8;
                let mask = 0xffff_ffff_ffff_ffffu64 << (32 - shift);
                let merged = (self.state.gpr(rt) & !mask) | (word & mask);
                self.state.set_gpr(rt, merged);
            }
        }
    }

    fn load_right(&mut self, instr: u32, mem: &mut dyn MemoryAccess, size: u32) {
        let rs = ((instr >> 21) & 0x1f) as usize;
        let rt = ((instr >> 16) & 0x1f) as usize;
        let imm = (instr & 0xffff) as i16 as i32;
        let addr = self.state.gpr(rs).wrapping_add(imm as u64) as u32;
        
        match size {
            4 => {
                let aligned = addr & !3;
                let word = mem.read32(aligned) as u64;
                let shift = (3 - (addr & 3)) * 8;
                let mask = 0xffff_ffff_ffff_ffffu64 >> (32 - shift);
                let merged = (self.state.gpr(rt) & !mask) | (word & mask);
                self.state.set_gpr(rt, merged);
            }
            8 => {
                let aligned = addr & !7;
                let dword = mem.read64(aligned);
                let shift = (7 - (addr & 7)) * 8;
                let mask = 0xffff_ffff_ffff_ffffu64 >> (64 - shift);
                let merged = (self.state.gpr(rt) & !mask) | (dword & mask);
                self.state.set_gpr(rt, merged);
            }
            _ => {
                let aligned = addr & !3;
                let word = mem.read32(aligned) as u64;
                let shift = (3 - (addr & 3)) * 8;
                let mask = 0xffff_ffff_ffff_ffffu64 >> (32 - shift);
                let merged = (self.state.gpr(rt) & !mask) | (word & mask);
                self.state.set_gpr(rt, merged);
            }
        }
    }

    fn store(&mut self, instr: u32, mem: &mut dyn MemoryAccess, size: u32) {
        let rs = ((instr >> 21) & 0x1f) as usize;
        let rt = ((instr >> 16) & 0x1f) as usize;
        let imm = (instr & 0xffff) as i16 as i32;
        let addr = self.state.gpr(rs).wrapping_add(imm as u64) as u32;
        let value = self.state.gpr(rt);

        match size {
            1 => mem.write8(addr, value as u8),
            2 => mem.write16(addr, value as u16),
            4 => mem.write32(addr, value as u32),
            8 => mem.write64(addr, value),
            _ => mem.write32(addr, value as u32),
        }
    }

    fn store_left(&mut self, instr: u32, mem: &mut dyn MemoryAccess, size: u32) {
        let rs = ((instr >> 21) & 0x1f) as usize;
        let rt = ((instr >> 16) & 0x1f) as usize;
        let imm = (instr & 0xffff) as i16 as i32;
        let addr = self.state.gpr(rs).wrapping_add(imm as u64) as u32;
        
        match size {
            4 => {
                let aligned = addr & !3;
                let word = mem.read32(aligned);
                let shift = (addr & 3) * 8;
                let mask = 0xffff_ffffu32 << (32 - shift);
                let merged = (word & !mask) | ((self.state.gpr(rt) as u32) & mask);
                mem.write32(aligned, merged);
            }
            8 => {
                let aligned = addr & !7;
                let dword = mem.read64(aligned);
                let shift = (addr & 7) * 8;
                let mask = 0xffff_ffff_ffff_ffffu64 << (64 - shift);
                let merged = (dword & !mask) | (self.state.gpr(rt) & mask);
                mem.write64(aligned, merged);
            }
            _ => {
                let aligned = addr & !3;
                let word = mem.read32(aligned);
                let shift = (addr & 3) * 8;
                let mask = 0xffff_ffffu32 << (32 - shift);
                let merged = (word & !mask) | ((self.state.gpr(rt) as u32) & mask);
                mem.write32(aligned, merged);
            }
        }
    }

    fn store_right(&mut self, instr: u32, mem: &mut dyn MemoryAccess, size: u32) {
        let rs = ((instr >> 21) & 0x1f) as usize;
        let rt = ((instr >> 16) & 0x1f) as usize;
        let imm = (instr & 0xffff) as i16 as i32;
        let addr = self.state.gpr(rs).wrapping_add(imm as u64) as u32;
        
        match size {
            4 => {
                let aligned = addr & !3;
                let word = mem.read32(aligned);
                let shift = (3 - (addr & 3)) * 8;
                let mask = 0xffff_ffffu32 >> (32 - shift);
                let merged = (word & !mask) | ((self.state.gpr(rt) as u32) & mask);
                mem.write32(aligned, merged);
            }
            8 => {
                let aligned = addr & !7;
                let dword = mem.read64(aligned);
                let shift = (7 - (addr & 7)) * 8;
                let mask = 0xffff_ffff_ffff_ffffu64 >> (64 - shift);
                let merged = (dword & !mask) | (self.state.gpr(rt) & mask);
                mem.write64(aligned, merged);
            }
            _ => {
                let aligned = addr & !3;
                let word = mem.read32(aligned);
                let shift = (3 - (addr & 3)) * 8;
                let mask = 0xffff_ffffu32 >> (32 - shift);
                let merged = (word & !mask) | ((self.state.gpr(rt) as u32) & mask);
                mem.write32(aligned, merged);
            }
        }
    }

    /// DADDI (0x18) / DADDIU (0x19) - 64-bit add immediate
    fn daddi(&mut self, instr: u32, is_signed: bool) {
        let rs = ((instr >> 21) & 0x1f) as usize;
        let rt = ((instr >> 16) & 0x1f) as usize;
        let imm = (instr & 0xffff) as i16 as i64;
        let rs_val = self.state.gpr(rs);
        
        if is_signed {
            // DADDI - signed add with overflow detection
            let result = rs_val.wrapping_add(imm as u64);
            // Check for overflow: (rs_val >= 0 && imm >= 0 && result < 0) || (rs_val < 0 && imm < 0 && result >= 0)
            let rs_sign = (rs_val >> 63) & 1;
            let imm_sign = ((imm as u64) >> 63) & 1;
            let result_sign = (result >> 63) & 1;
            
            if rs_sign == imm_sign && rs_sign != result_sign {
                // Overflow - trigger exception using Cp0 API
                let mut cause = self.cp0.cause() & !0x7c;
                cause |= (0x0c << 2); // Ovf exception code
                self.cp0.write(Cp0Reg::Cause, cause);
                self.cp0.set_epc(self.state.pc as u32);
                let mut status = self.cp0.status() & !0x3f;
                status |= 0x01; // Set EXL bit
                self.cp0.write(Cp0Reg::Status, status);
                self.state.pc = 0x80000080; // Exception vector
                return;
            }
            self.state.set_gpr(rt, result);
        } else {
            // DADDIU - unsigned add, no overflow
            self.state.set_gpr(rt, rs_val.wrapping_add(imm as u64));
        }
    }

    /// LQ (0x1e) - Load Quadword (128-bit load into register pair rt, rt+1)
    fn load_quad(&mut self, instr: u32, mem: &mut dyn MemoryAccess) {
        let rs = ((instr >> 21) & 0x1f) as usize;
        let rt = ((instr >> 16) & 0x1f) as usize;
        let imm = (instr & 0xffff) as i16 as i32;
        let addr = self.state.gpr(rs).wrapping_add(imm as u64) as u32;
        
        // LQ loads 128 bits (16 bytes) into rt and rt+1
        // Must be 16-byte aligned
        let aligned_addr = addr & !0xf;
        let low = mem.read64(aligned_addr);
        let high = mem.read64(aligned_addr + 8);
        
        self.state.set_gpr(rt, low);
        if rt < 31 {
            self.state.set_gpr(rt + 1, high);
        }
    }

    /// SQ (0x1f) - Store Quadword (128-bit store from register pair rt, rt+1)
    fn store_quad(&mut self, instr: u32, mem: &mut dyn MemoryAccess) {
        let rs = ((instr >> 21) & 0x1f) as usize;
        let rt = ((instr >> 16) & 0x1f) as usize;
        let imm = (instr & 0xffff) as i16 as i32;
        let addr = self.state.gpr(rs).wrapping_add(imm as u64) as u32;
        
        // SQ stores 128 bits (16 bytes) from rt and rt+1
        // Must be 16-byte aligned
        let aligned_addr = addr & !0xf;
        let low = self.state.gpr(rt);
        let high = if rt < 31 { self.state.gpr(rt + 1) } else { 0 };
        
        mem.write64(aligned_addr, low);
        mem.write64(aligned_addr + 8, high);
    }

    fn load_fp(&mut self, instr: u32, mem: &mut dyn MemoryAccess) {
        let rs = ((instr >> 21) & 0x1f) as usize;
        let ft = ((instr >> 16) & 0x1f) as usize;
        let imm = (instr & 0xffff) as i16 as i32;
        let addr = self.state.gpr(rs).wrapping_add(imm as u64) as u32;
        self.state.fpr[ft] = mem.read32(addr) as u64;
    }

    fn store_fp(&mut self, instr: u32, mem: &mut dyn MemoryAccess) {
        let rs = ((instr >> 21) & 0x1f) as usize;
        let ft = ((instr >> 16) & 0x1f) as usize;
        let imm = (instr & 0xffff) as i16 as i32;
        let addr = self.state.gpr(rs).wrapping_add(imm as u64) as u32;
        mem.write32(addr, self.state.fpr[ft] as u32);
    }

    // === Coprocessors ===
    fn cop0(&mut self, instr: u32) {
        let rs = (instr >> 21) & 0x1f;
        let rt = ((instr >> 16) & 0x1f) as usize;
        let rd = ((instr >> 11) & 0x1f) as usize;

        match rs {
            0x00 => {
                // MFC0 - Move From Coprocessor 0
                let v = self.cp0.read(reg_from_index(rd));
                self.state.set_gpr(rt, v as u64);
            }
            0x02 => {
                // CFC0 - Move Control From Coprocessor 0 (same as MFC0 for R5000)
                let v = self.cp0.read(reg_from_index(rd));
                self.state.set_gpr(rt, v as u64);
            }
            0x04 => {
                // MTC0 - Move To Coprocessor 0
                let v = self.state.gpr(rt) as u32;
                self.cp0.write(reg_from_index(rd), v);
            }
            0x06 => {
                // CTC0 - Move Control To Coprocessor 0 (same as MTC0 for R5000)
                let v = self.state.gpr(rt) as u32;
                self.cp0.write(reg_from_index(rd), v);
            }
            0x10 => {
                // COP0 function
                let funct = instr & 0x3f;
                match funct {
                    0x01 => {
                        // TLBR - TLB Read
                        self.cp0.tlb_read_indexed();
                    }
                    0x02 => {
                        // TLBWI - TLB Write Index
                        self.cp0.tlb_write_indexed();
                    }
                    0x06 => {
                        // TLBWR - TLB Write Random
                        self.cp0.tlb_write_random();
                    }
                    0x08 => {
                        // TLBP - TLB Probe
                        self.cp0.tlb_probe();
                    }
                    0x18 => {
                        // ERET - Exception Return
                        let target = self.cp0.eret();
                        self.state.next_pc = target;
                    }
                    0x20 => {
                        // WAIT - Wait for interrupt (R5000)
                        // In emulator, just continue
                    }
                    _ => {
                        log::warn_msg(&format!(
                            "Unimplemented COP0 funct 0x{funct:02x} at PC 0x{:08x}",
                            self.state.pc
                        ));
                        self.exception(ExceptionCode::ReservedInstr);
                    }
                }
            }
            _ => {
                log::warn_msg(&format!(
                    "Unimplemented COP0 rs 0x{rs:02x} at PC 0x{:08x}",
                    self.state.pc
                ));
                self.exception(ExceptionCode::ReservedInstr);
            }
        }
    }

    fn cop1(&mut self, instr: u32) {
        let rs = (instr >> 21) & 0x1f;
        let rt = ((instr >> 16) & 0x1f) as usize;
        let fs = ((instr >> 11) & 0x1f) as usize;
        let nd = (instr >> 16) & 0x1f; // For BC1: nd = condition bit

        match rs {
            0x00 => {
                // MFC1 - Move From Coprocessor 1 (32-bit)
                let v = self.state.fpr[fs] as u32;
                self.state.set_gpr(rt, v as u64);
            }
            0x01 => {
                // DMFC1 - Doubleword Move From Coprocessor 1 (64-bit, MIPS III)
                let v = self.state.fpr[fs];
                self.state.set_gpr(rt, v);
            }
            0x04 => {
                // MTC1 - Move To Coprocessor 1 (32-bit)
                self.state.fpr[fs] = self.state.gpr(rt) as u32 as u64;
            }
            0x05 => {
                // DMTC1 - Doubleword Move To Coprocessor 1 (64-bit, MIPS III)
                self.state.fpr[fs] = self.state.gpr(rt);
            }
            0x02 => {
                // CFC1 - Move Control From Coprocessor 1
                let v = if fs == 0 {
                    self.state.fcr0
                } else if fs == 31 {
                    self.state.fcr31
                } else {
                    0
                };
                self.state.set_gpr(rt, v as u64);
            }
            0x06 => {
                // CTC1 - Move Control To Coprocessor 1
                let v = self.state.gpr(rt) as u32;
                if fs == 0 {
                    self.state.fcr0 = v;
                } else if fs == 31 {
                    self.state.fcr31 = v;
                }
            }
            0x08 => {
                // BC1 - Branch on FPU Condition (MIPS IV)
                // nd[4:1] = condition code, nd[0] = tf (true/false)
                let cc = (nd >> 1) & 0x7; // condition code (0-7)
                let tf = nd & 1; // true/false
                let imm = (instr & 0xffff) as i16 as i32;
                
                // Get condition bit from FCR31
                let cond_bit = (self.state.fcr31 >> (23 + cc)) & 1;
                let take_branch = (cond_bit != 0) == (tf != 0);
                
                if take_branch {
                    self.branch_offset(imm);
                }
            }
            0x10..=0x1f => {
                // FPU arithmetic operations
                self.cop1_fpu(instr);
            }
            _ => {
                log::warn_msg(&format!(
                    "Unimplemented COP1 rs 0x{rs:02x} at PC 0x{:08x}",
                    self.state.pc
                ));
                self.exception(ExceptionCode::ReservedInstr);
            }
        }
    }

    /// Handle FPU arithmetic operations (COP1 with rs >= 0x10)
    fn cop1_fpu(&mut self, instr: u32) {
        let fmt = (instr >> 21) & 0x1f;
        let ft = ((instr >> 16) & 0x1f) as usize;
        let fs = ((instr >> 11) & 0x1f) as usize;
        let fd = ((instr >> 6) & 0x1f) as usize;
        let funct = instr & 0x3f;

        match fmt {
            0x10 => {
                // Single precision (S)
                self.fpu_s(instr, ft, fs, fd, funct);
            }
            0x11 => {
                // Double precision (D)
                self.fpu_d(instr, ft, fs, fd, funct);
            }
            0x14 => {
                // Word fixed point (W)
                self.fpu_w(instr, ft, fs, fd, funct);
            }
            0x15 => {
                // Long fixed point (L)
                self.fpu_l(instr, ft, fs, fd, funct);
            }
            0x18 => {
                // PS - Paired Single (MIPS IV extension, not fully implemented)
                // For now, treat as reserved instruction
                log::warn_msg(&format!(
                    "Paired Single FPU fmt not implemented at PC 0x{:08x}",
                    self.state.pc
                ));
                self.exception(ExceptionCode::ReservedInstr);
            }
            _ => {
                log::warn_msg(&format!(
                    "Unimplemented FPU fmt 0x{fmt:02x} at PC 0x{:08x}",
                    self.state.pc
                ));
                self.exception(ExceptionCode::ReservedInstr);
            }
        }
    }

    /// Single precision FPU operations
    fn fpu_s(&mut self, _instr: u32, ft: usize, fs: usize, fd: usize, funct: u32) {
        let fs_val = f32::from_bits(self.state.fpr[fs] as u32);
        let _ft_val = f32::from_bits(self.state.fpr[ft] as u32);
        let fd_val = f32::from_bits(self.state.fpr[fd] as u32);

        let result = match funct {
            0x00 => fd_val + fs_val,      // ADD.S
            0x01 => fd_val - fs_val,      // SUB.S
            0x02 => fd_val * fs_val,      // MUL.S
            0x03 => fd_val / fs_val,      // DIV.S
            0x04 => fd_val.sqrt(),        // SQRT.S
            0x05 => fd_val.abs(),         // ABS.S
            0x06 => fd_val,               // MOV.S
            0x07 => -fd_val,              // NEG.S
            0x08 => fd_val.round() as f32, // ROUND.L.S (round to long)
            0x09 => fd_val.trunc() as f32, // TRUNC.L.S (truncate to long)
            0x0a => fd_val.ceil() as f32,  // CEIL.L.S (ceiling to long)
            0x0b => fd_val.floor() as f32, // FLOOR.L.S (floor to long)
            0x0c => fd_val.round() as f32, // ROUND.W.S (round to word)
            0x0d => fd_val.trunc() as f32, // TRUNC.W.S (truncate to word)
            0x0e => fd_val.ceil() as f32,  // CEIL.W.S (ceiling to word)
            0x0f => fd_val.floor() as f32, // FLOOR.W.S (floor to word)
            0x10 => 1.0 / fd_val,         // RECIP.S (reciprocal approximation)
            0x11 => 1.0 / fd_val.sqrt(),  // RSQRT.S (reciprocal sqrt approximation)
            0x20 => {
                // CVT.S.D - convert double to single
                let d_val = f64::from_bits(self.state.fpr[fs] as u64);
                d_val as f32
            }
            0x21 => {
                // CVT.S.L - convert long to single
                let l_val = self.state.fpr[fs] as i64;
                l_val as f32
            }
            0x24 => {
                // CVT.S.W - convert word to single
                fs_val as i32 as f32
            }
            0x30..=0x3f => {
                // Comparison operations
                let cond = funct & 0x0f;
                let cmp_result = match cond {
                    0x00 => fd_val < fs_val,      // C.F.S
                    0x01 => fd_val == fs_val,     // C.UN.S
                    0x02 => fd_val <= fs_val,     // C.EQ.S
                    0x03 => fd_val < fs_val,      // C.UEQ.S
                    0x04 => fd_val <= fs_val,     // C.OLT.S
                    0x05 => fd_val < fs_val,      // C.ULT.S
                    0x06 => fd_val <= fs_val,     // C.OLE.S
                    0x07 => fd_val < fs_val,      // C.ULE.S
                    0x08 => fd_val > fs_val,      // C.SF.S
                    0x09 => fd_val != fs_val,     // C.NGLE.S
                    0x0a => fd_val >= fs_val,     // C.SEQ.S
                    0x0b => fd_val > fs_val,      // C.NGL.S
                    0x0c => fd_val >= fs_val,     // C.LT.S
                    0x0d => fd_val > fs_val,      // C.NGE.S
                    0x0e => fd_val >= fs_val,     // C.LE.S
                    0x0f => fd_val > fs_val,      // C.NGT.S
                    _ => false,
                };
                // Set condition bit in FCR31
                if cmp_result {
                    self.state.fcr31 |= 1 << 23;
                } else {
                    self.state.fcr31 &= !(1 << 23);
                }
                return; // Comparisons don't write to fd
            }
            _ => {
                log::warn_msg(&format!(
                    "Unimplemented FPU S funct 0x{funct:02x} at PC 0x{:08x}",
                    self.state.pc
                ));
                self.exception(ExceptionCode::ReservedInstr);
                return;
            }
        };

        self.state.fpr[fd] = result.to_bits() as u64;
    }

    /// Double precision FPU operations
    fn fpu_d(&mut self, _instr: u32, ft: usize, fs: usize, fd: usize, funct: u32) {
        let fs_val = f64::from_bits(self.state.fpr[fs] as u64);
        let _ft_val = f64::from_bits(self.state.fpr[ft] as u64);
        let fd_val = f64::from_bits(self.state.fpr[fd] as u64);

        let result = match funct {
            0x00 => fd_val + fs_val,      // ADD.D
            0x01 => fd_val - fs_val,      // SUB.D
            0x02 => fd_val * fs_val,      // MUL.D
            0x03 => fd_val / fs_val,      // DIV.D
            0x05 => fd_val.abs(),         // ABS.D
            0x06 => fd_val,               // MOV.D
            0x07 => -fd_val,              // NEG.D
            0x20 => {
                // CVT.D.S - convert single to double
                let s_val = f32::from_bits(self.state.fpr[fs] as u32);
                s_val as f64
            }
            0x25 => {
                // CVT.D.W - convert word to double
                fs_val as i32 as f64
            }
            0x30..=0x3f => {
                // Comparison operations
                let cond = funct & 0x0f;
                let cmp_result = match cond {
                    0x00 => fd_val < fs_val,      // C.F.D
                    0x01 => fd_val == fs_val,     // C.UN.D
                    0x02 => fd_val <= fs_val,     // C.EQ.D
                    0x03 => fd_val < fs_val,      // C.UEQ.D
                    0x04 => fd_val <= fs_val,     // C.OLT.D
                    0x05 => fd_val < fs_val,      // C.ULT.D
                    0x06 => fd_val <= fs_val,     // C.OLE.D
                    0x07 => fd_val < fs_val,      // C.ULE.D
                    0x08 => fd_val > fs_val,      // C.SF.D
                    0x09 => fd_val != fs_val,     // C.NGLE.D
                    0x0a => fd_val >= fs_val,     // C.SEQ.D
                    0x0b => fd_val > fs_val,      // C.NGL.D
                    0x0c => fd_val >= fs_val,     // C.LT.D
                    0x0d => fd_val > fs_val,      // C.NGE.D
                    0x0e => fd_val >= fs_val,     // C.LE.D
                    0x0f => fd_val > fs_val,      // C.NGT.D
                    _ => false,
                };
                // Set condition bit in FCR31
                if cmp_result {
                    self.state.fcr31 |= 1 << 23;
                } else {
                    self.state.fcr31 &= !(1 << 23);
                }
                return; // Comparisons don't write to fd
            }
            _ => {
                log::warn_msg(&format!(
                    "Unimplemented FPU D funct 0x{funct:02x} at PC 0x{:08x}",
                    self.state.pc
                ));
                self.exception(ExceptionCode::ReservedInstr);
                return;
            }
        };

        self.state.fpr[fd] = result.to_bits() as u64;
    }

    /// Word fixed point FPU operations
    fn fpu_w(&mut self, _instr: u32, _ft: usize, fs: usize, fd: usize, funct: u32) {
        let fs_val = f32::from_bits(self.state.fpr[fs] as u32);
        let _fd_val = f32::from_bits(self.state.fpr[fd] as u32);

        let result = match funct {
            0x20 => {
                // CVT.W.S - convert single to word
                fs_val as i32 as u32 as u64
            }
            0x21 => {
                // CVT.W.D - convert double to word
                let d_val = f64::from_bits(self.state.fpr[fs] as u64);
                d_val as i32 as u32 as u64
            }
            _ => {
                log::warn_msg(&format!(
                    "Unimplemented FPU W funct 0x{funct:02x} at PC 0x{:08x}",
                    self.state.pc
                ));
                self.exception(ExceptionCode::ReservedInstr);
                return;
            }
        };

        self.state.fpr[fd] = result;
    }

    /// Long fixed point FPU operations
    fn fpu_l(&mut self, _instr: u32, _ft: usize, fs: usize, fd: usize, funct: u32) {
        let fs_val = f32::from_bits(self.state.fpr[fs] as u32);
        let _fd_val = f32::from_bits(self.state.fpr[fd] as u32);

        let result = match funct {
            0x20 => {
                // CVT.L.S - convert single to long
                fs_val as i64 as u64
            }
            0x21 => {
                // CVT.L.D - convert double to long
                let d_val = f64::from_bits(self.state.fpr[fs] as u64);
                d_val as i64 as u64
            }
            _ => {
                log::warn_msg(&format!(
                    "Unimplemented FPU L funct 0x{funct:02x} at PC 0x{:08x}",
                    self.state.pc
                ));
                self.exception(ExceptionCode::ReservedInstr);
                return;
            }
        };

        self.state.fpr[fd] = result;
    }

    // === Exceptions ===
    fn exception(&mut self, code: ExceptionCode) {
        let vector = self.cp0.take_exception(&mut self.state, code);
        self.state.next_pc = vector;
    }

    /// MADD/MADDU helper (MIPS IV): multiply-add to HI/LO.
    /// Takes the product (already computed) and adds it to HI/LO.
    /// signed: true for MADD (signed), false for MADDU (unsigned).
    /// Returns (new_lo, new_hi).
    fn madd_maddu(&mut self, product: i128, signed: bool) -> (u64, u64) {
        let (hi, lo) = if signed {
            let sum = (self.state.hi as i128) << 64 | (self.state.lo as i128 & 0xFFFF_FFFF_FFFF_FFFF);
            let result = sum + product;
            ((result >> 64) as u64, (result & 0xFFFF_FFFF_FFFF_FFFF) as u64)
        } else {
            let sum = (self.state.hi as u128) << 64 | (self.state.lo as u128 & 0xFFFF_FFFF_FFFF_FFFF);
            let result = sum + product as u128;
            ((result >> 64) as u64, (result & 0xFFFF_FFFF_FFFF_FFFF) as u64)
        };

        self.state.hi = hi;
        self.state.lo = lo;
        (lo, hi)
    }

    /// Check for pending interrupts and take one if enabled.
    pub fn check_interrupts(&mut self) {
        if !self.cp0.interrupts_enabled() {
            return;
        }
        let pending = self.cp0.pending_interrupts();
        let mask = self.cp0.interrupt_mask();
        if pending & mask != 0 {
            self.exception(ExceptionCode::Int);
        }
    }

    /// Dump the register state to the log.
    pub fn dump_registers(&self) {
        log::info_msg(&format!("PC: 0x{:08x}", self.state.pc));
        for i in 0..32 {
            log::info_msg(&format!("${i:02}: 0x{:016x}", self.state.gpr[i]));
        }
        log::info_msg(&format!("HI: 0x{:016x}  LO: 0x{:016x}", self.state.hi, self.state.lo));
        log::info_msg(&format!(
            "Status: 0x{:08x}  Cause: 0x{:08x}  EPC: 0x{:08x}",
            self.cp0.status(),
            self.cp0.cause(),
            self.cp0.epc()
        ));
    }
}

/// Map a CP0 register index (0–31) to a [`Cp0Reg`], defaulting to a safe
/// register for out-of-range indices.
fn reg_from_index(index: usize) -> Cp0Reg {
    use Cp0Reg::*;
    const REGS: [Cp0Reg; 32] = [
        Index, Random, EntryLo0, EntryLo1, Context, PageMask, Wired, BadVAddr, Count, EntryHi,
        Compare, Status, Cause, Epc, PrId, Config, LlAddr, WatchLo, WatchHi, XContext, Ecc,
        CacheErr, TagLo, TagHi, ErrorEpc, ErrorEpc, ErrorEpc, ErrorEpc, ErrorEpc, ErrorEpc,
        ErrorEpc, ErrorEpc,
    ];
    REGS[index.min(31)]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A simple in-memory bus for testing.
    struct TestMem {
        data: Vec<u8>,
    }

    impl TestMem {
        fn new(size: usize) -> Self {
            Self { data: vec![0; size] }
        }
    }

    impl MemoryAccess for TestMem {
        fn read32(&mut self, addr: u32) -> u32 {
            let a = addr as usize;
            ((self.data[a] as u32) << 24)
                | ((self.data[a + 1] as u32) << 16)
                | ((self.data[a + 2] as u32) << 8)
                | (self.data[a + 3] as u32)
        }
        fn read16(&mut self, addr: u32) -> u16 {
            let a = addr as usize;
            ((self.data[a] as u16) << 8) | (self.data[a + 1] as u16)
        }
        fn read8(&mut self, addr: u32) -> u8 {
            self.data[addr as usize]
        }
        fn write32(&mut self, addr: u32, value: u32) {
            let a = addr as usize;
            self.data[a] = (value >> 24) as u8;
            self.data[a + 1] = (value >> 16) as u8;
            self.data[a + 2] = (value >> 8) as u8;
            self.data[a + 3] = value as u8;
        }
        fn write16(&mut self, addr: u32, value: u16) {
            let a = addr as usize;
            self.data[a] = (value >> 8) as u8;
            self.data[a + 1] = value as u8;
        }
        fn write8(&mut self, addr: u32, value: u8) {
            self.data[addr as usize] = value;
        }
    }

    #[test]
    fn executes_addiu_and_lui() {
        let mut cpu = R5000::new();
        let mut mem = TestMem::new(0x2000);
        cpu.state.pc = 0x1000;

        // ADDIU $t0, $zero, 0x1234  => 0x2408_1234
        mem.write32(0x1000, 0x2408_1234);
        // LUI $t1, 0x5678            => 0x3c09_5678
        mem.write32(0x1004, 0x3c09_5678);

        cpu.step(&mut mem);
        assert_eq!(cpu.state.gpr(8), 0x1234);
        assert_eq!(cpu.state.pc, 0x1004);

        cpu.step(&mut mem);
        assert_eq!(cpu.state.gpr(9), 0x5678_0000);
        assert_eq!(cpu.state.pc, 0x1008);
    }

    #[test]
    fn executes_jump_and_link() {
        let mut cpu = R5000::new();
        let mut mem = TestMem::new(0x2000);
        cpu.state.pc = 0x1000;

        // JAL 0x2000  => 0x0c00_0800
        mem.write32(0x1000, 0x0c00_0800);

        cpu.step(&mut mem);
        assert_eq!(cpu.state.gpr(31), 0x1008);
        assert_eq!(cpu.state.pc, 0x2000);
    }
}