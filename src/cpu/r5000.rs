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
use crate::cpu::cp0::{cause, status, Cp0Reg, ExceptionCode};
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
    /// Write a 32-bit word (big-endian).
    fn write32(&mut self, addr: u32, value: u32);
    /// Write a 16-bit halfword (big-endian).
    fn write16(&mut self, addr: u32, value: u16);
    /// Write a byte.
    fn write8(&mut self, addr: u32, value: u8);
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

        self.execute(instr, mem);

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
            0x20 => self.load(instr, mem, 1, true),
            0x21 => self.load(instr, mem, 2, true),
            0x22 => self.load_left(instr, mem),
            0x23 => self.load(instr, mem, 4, true),
            0x24 => self.load(instr, mem, 1, false),
            0x25 => self.load(instr, mem, 2, false),
            0x26 => self.load_right(instr, mem),
            0x28 => self.store(instr, mem, 1),
            0x29 => self.store(instr, mem, 2),
            0x2a => self.store_left(instr, mem),
            0x2b => self.store(instr, mem, 4),
            0x2e => self.store_right(instr, mem),
            0x2f => {} // CACHE — no-op for now
            0x30 => self.load(instr, mem, 4, true), // LL (simplified)
            0x38 => self.store(instr, mem, 4),      // SC (simplified)
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

    // === SPECIAL (opcode 0x00) ===
    fn execute_special(&mut self, instr: u32, mem: &mut dyn MemoryAccess) {
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
            0x2c..=0x32 => {
                // TGE/TGEU/TLT/TLTU/TEQ/TNE — trap if condition holds
                let trap = match funct {
                    0x2c => self.state.gpr(rs) as i64 >= self.state.gpr(rt) as i64,
                    0x2d => self.state.gpr(rs) >= self.state.gpr(rt),
                    0x2e => (self.state.gpr(rs) as i64) < (self.state.gpr(rt) as i64),
                    0x2f => self.state.gpr(rs) < self.state.gpr(rt),
                    0x30 => self.state.gpr(rs) == self.state.gpr(rt),
                    0x32 => self.state.gpr(rs) != self.state.gpr(rt),
                    _ => false,
                };
                if trap {
                    self.exception(ExceptionCode::Trap);
                }
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
                // BLTZ
                if (self.state.gpr(rs) as i64) < 0 {
                    self.branch_offset(imm);
                }
            }
            0x01 => {
                // BGEZ
                if (self.state.gpr(rs) as i64) >= 0 {
                    self.branch_offset(imm);
                }
            }
            0x10 => {
                // BLTZAL
                self.state.set_gpr(31, self.state.pc.wrapping_add(8) as u64);
                if (self.state.gpr(rs) as i64) < 0 {
                    self.branch_offset(imm);
                }
            }
            0x11 => {
                // BGEZAL
                self.state.set_gpr(31, self.state.pc.wrapping_add(8) as u64);
                if (self.state.gpr(rs) as i64) >= 0 {
                    self.branch_offset(imm);
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

    fn branch_zero(&mut self, instr: u32, lez: bool, _likely: bool) {
        let rs = ((instr >> 21) & 0x1f) as usize;
        let imm = (instr & 0xffff) as i16 as i32;
        let val = self.state.gpr(rs) as i64;
        let take = if lez { val <= 0 } else { val > 0 };
        if take {
            self.branch_offset(imm);
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
            _ => mem.read32(addr) as u64,
        };

        let value = if sign_extend {
            match size {
                1 => (value as u8 as i8 as i64) as u64,
                2 => (value as u16 as i16 as i64) as u64,
                _ => value,
            }
        } else {
            value
        };

        self.state.set_gpr(rt, value);
    }

    fn load_left(&mut self, instr: u32, mem: &mut dyn MemoryAccess) {
        let rs = ((instr >> 21) & 0x1f) as usize;
        let rt = ((instr >> 16) & 0x1f) as usize;
        let imm = (instr & 0xffff) as i16 as i32;
        let addr = self.state.gpr(rs).wrapping_add(imm as u64) as u32;
        let aligned = addr & !3;
        let word = mem.read32(aligned) as u64;
        let shift = (addr & 3) * 8;
        let mask = 0xffff_ffff_ffff_ffffu64 << (32 - shift);
        let merged = (self.state.gpr(rt) & !mask) | (word & mask);
        self.state.set_gpr(rt, merged);
    }

    fn load_right(&mut self, instr: u32, mem: &mut dyn MemoryAccess) {
        let rs = ((instr >> 21) & 0x1f) as usize;
        let rt = ((instr >> 16) & 0x1f) as usize;
        let imm = (instr & 0xffff) as i16 as i32;
        let addr = self.state.gpr(rs).wrapping_add(imm as u64) as u32;
        let aligned = addr & !3;
        let word = mem.read32(aligned) as u64;
        let shift = (3 - (addr & 3)) * 8;
        let mask = 0xffff_ffff_ffff_ffffu64 >> (32 - shift);
        let merged = (self.state.gpr(rt) & !mask) | (word & mask);
        self.state.set_gpr(rt, merged);
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
            _ => mem.write32(addr, value as u32),
        }
    }

    fn store_left(&mut self, instr: u32, mem: &mut dyn MemoryAccess) {
        let rs = ((instr >> 21) & 0x1f) as usize;
        let rt = ((instr >> 16) & 0x1f) as usize;
        let imm = (instr & 0xffff) as i16 as i32;
        let addr = self.state.gpr(rs).wrapping_add(imm as u64) as u32;
        let aligned = addr & !3;
        let word = mem.read32(aligned);
        let shift = (addr & 3) * 8;
        let mask = 0xffff_ffffu32 << (32 - shift);
        let merged = (word & !mask) | ((self.state.gpr(rt) as u32) & mask);
        mem.write32(aligned, merged);
    }

    fn store_right(&mut self, instr: u32, mem: &mut dyn MemoryAccess) {
        let rs = ((instr >> 21) & 0x1f) as usize;
        let rt = ((instr >> 16) & 0x1f) as usize;
        let imm = (instr & 0xffff) as i16 as i32;
        let addr = self.state.gpr(rs).wrapping_add(imm as u64) as u32;
        let aligned = addr & !3;
        let word = mem.read32(aligned);
        let shift = (3 - (addr & 3)) * 8;
        let mask = 0xffff_ffffu32 >> (32 - shift);
        let merged = (word & !mask) | ((self.state.gpr(rt) as u32) & mask);
        mem.write32(aligned, merged);
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
                // MFC0
                let v = self.cp0.read(reg_from_index(rd));
                self.state.set_gpr(rt, v as u64);
            }
            0x04 => {
                // MTC0
                let v = self.state.gpr(rt) as u32;
                self.cp0.write(reg_from_index(rd), v);
            }
            0x10 => {
                // COP0 function
                let funct = instr & 0x3f;
                match funct {
                    0x18 => {
                        // ERET
                        let target = self.cp0.eret();
                        self.state.next_pc = target;
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

        match rs {
            0x00 => {
                // MFC1
                let v = self.state.fpr[fs] as u32;
                self.state.set_gpr(rt, v as u64);
            }
            0x04 => {
                // MTC1
                self.state.fpr[fs] = self.state.gpr(rt) as u32 as u64;
            }
            0x02 => {
                // CFC1
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
                // CTC1
                let v = self.state.gpr(rt) as u32;
                if fs == 0 {
                    self.state.fcr0 = v;
                } else if fs == 31 {
                    self.state.fcr31 = v;
                }
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

    // === Exceptions ===
    fn exception(&mut self, code: ExceptionCode) {
        let vector = self.cp0.take_exception(&mut self.state, code);
        self.state.next_pc = vector;
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

// Silence unused-import warnings for the status/cause modules (kept for
// documentation and future use).
#[allow(unused_imports)]
use {cause, status};

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
        let mut mem = TestMem::new(0x1000);
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
        let mut mem = TestMem::new(0x1000);
        cpu.state.pc = 0x1000;

        // JAL 0x2000  => 0x0c00_0800
        mem.write32(0x1000, 0x0c00_0800);

        cpu.step(&mut mem);
        assert_eq!(cpu.state.gpr(31), 0x1008);
        assert_eq!(cpu.state.pc, 0x2000);
    }
}