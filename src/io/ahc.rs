//! Adaptec AIC-7880 SCSI controller (MACE PCI, device 1 = SCSI0, device 2 = SCSI1).
//!
//! Register offsets and bit definitions are sourced from the Linux driver
//! `samples/linux/drivers/scsi/aic7xxx/aic7xxx_reg.h_shipped` (which is
//! generated from `aic7xxx.reg`, in turn taken from the Adaptec/AMD 7880
//! register manual). The on-chip sequencer executes microcode that the host
//! downloads into SEQRAM; the sequencer interpreter itself is the major
//! remaining piece (see `ROADMAP.md`).
//!
//! This module implements the register block: attach/reset handshakes, the
//! SEQRAM download protocol, and the controller interrupt/status latches.
//! Each controller owns one [`ScsiBus`]: SCSI0 (ahc0) carries the internal
//! disk (target 1), SCSI1 (ahc1) the CD-ROM (target 6).

use crate::io::scsi::ScsiBus;

/// AIC-7880 register block size (BAR0 window).
pub const AIC7880_REG_WINDOW: u32 = 0x100;

// Register offsets (aic7xxx_reg.h_shipped).
pub const SCSIID: u32 = 0x05;
pub const SINDEX: u32 = 0x65;
pub const DINDEX: u32 = 0x66;
pub const ALLONES: u32 = 0x69;
pub const ALLZEROS: u32 = 0x6a;
pub const NONE: u32 = 0x6a;
pub const FLAGS: u32 = 0x6b;
pub const SELID: u32 = 0x19;
pub const SEECTL: u32 = 0x1e;
pub const SBLKCTL: u32 = 0x1f;
pub const SCSICONF: u32 = 0x5a;
pub const SEQCTL: u32 = 0x60;
pub const SEQRAM: u32 = 0x61;
pub const SEQADDR0: u32 = 0x62;
pub const SEQADDR1: u32 = 0x63;
pub const HCNTRL: u32 = 0x87;
pub const HADDR: u32 = 0x88;
pub const HCNT: u32 = 0x8c;
pub const SCBPTR: u32 = 0x90;
pub const INTSTAT: u32 = 0x91;
pub const CLRINT: u32 = 0x92;
pub const DFCNTRL: u32 = 0x93;
pub const DFSTATUS: u32 = 0x94;
pub const SCBCNT: u32 = 0x9a;
pub const QINFIFO: u32 = 0x9b;

// Field bits.
pub const SEECTL_SEERDY: u8 = 0x10;
pub const SEQCTL_PERRORDIS: u8 = 0x80;
pub const SEQCTL_PAUSEDIS: u8 = 0x40;
pub const SEQCTL_FAILDIS: u8 = 0x20;
pub const SEQCTL_FASTMODE: u8 = 0x10;
pub const SEQCTL_BRKADRINTEN: u8 = 0x08;
pub const SEQCTL_STEP: u8 = 0x04;
pub const SEQCTL_SEQRESET: u8 = 0x02;
pub const SEQCTL_LOADRAM: u8 = 0x01;
pub const HCNTRL_POWRDN: u8 = 0x40;
pub const HCNTRL_SWINT: u8 = 0x10;
pub const HCNTRL_IRQMS: u8 = 0x08;
pub const HCNTRL_PAUSE: u8 = 0x04;
pub const HCNTRL_INTEN: u8 = 0x02;
pub const HCNTRL_CHIPRST: u8 = 0x01;
pub const HCNTRL_CHIPRSTACK: u8 = 0x01;
pub const INTSTAT_BRKADRINT: u8 = 0x08;
pub const INTSTAT_SCSIINT: u8 = 0x04;
pub const INTSTAT_CMDCMPLT: u8 = 0x02;
pub const INTSTAT_SEQINT: u8 = 0x01;

/// Sequencer RAM capacity in bytes (the 7880 stores 2K 32-bit words).
pub const SEQRAM_BYTES: usize = 8192;

/// AIC-7880 register block and sequencer download state.
#[derive(Debug)]
pub struct Aic7880 {
    /// General register file (bytes, indexed by register offset).
    reg: [u8; AIC7880_REG_WINDOW as usize],
    /// Sequencer instruction/scratch RAM.
    seqram: [u8; SEQRAM_BYTES],
    /// Current sequencer address (word index, high 11 bits hold the value).
    seq_addr: u16,
    /// Next byte position within the current SEQRAM word (0..4). The
    /// address increments after the 4th (most significant) byte.
    seq_word_byte: u8,
    /// The SCSI bus this controller drives (its own channel).
    pub bus: ScsiBus,
    /// SCB RAM: 16 slots x 64 bytes, indexed through SCBPTR by the
    /// register-file window at 0xA0..0xDF (aic7xxx.reg, "SCB").
    pub scb: [u8; 1024],
    /// Sequencer scratch RAM (register file 0x20..0x5F).
    pub sram: [u8; 256],
    /// Sequencer program counter (word index into SEQRAM).
    pc: u16,
    /// Sequencer run state: halted until the host clears pause.
    seq_paused: bool,
    /// CALL return-address stack for the sequencer.
    call_stack: [u16; 8],
    call_depth: u8,
}

impl Default for Aic7880 {
    fn default() -> Self {
        Self::new()
    }
}

impl Aic7880 {
    pub fn new() -> Self {
        let mut c = Aic7880 {
            reg: [0; AIC7880_REG_WINDOW as usize],
            seqram: [0; SEQRAM_BYTES],
            seq_addr: 0,
            seq_word_byte: 0,
            bus: ScsiBus::new(),
            scb: [0; 1024],
            sram: [0; 256],
            pc: 0,
            seq_paused: true,
            call_stack: [0; 8],
            call_depth: 0,
        };
        c.internal_reset();
        c
    }

    /// Internal (chip) reset: registers and sequencer are cleared. The host
    /// drives this by writing HCNTRL with CHIPRST set.
    fn internal_reset(&mut self) {
        self.reg = [0; AIC7880_REG_WINDOW as usize];
        self.seq_addr = 0;
        self.seq_word_byte = 0;
        self.scb = [0; 1024];
        self.sram = [0; 256];
        self.pc = 0;
        self.call_stack = [0; 8];
        self.call_depth = 0;
        self.seq_paused = true;
        // The sequencer is halted after reset until the host runs it.
        self.reg[HCNTRL as usize] &= !HCNTRL_CHIPRST;
    }

    /// Read an 8-bit attach register.
    pub fn read8(&mut self, offset: u32) -> u8 {
        match offset {
            SEECTL => self.reg[SEECTL as usize] | SEECTL_SEERDY,
            SEQRAM => {
                // Return the current byte of the word at SEQADDR; the
                // address advances only on writes (aic7xxx.reg, p. 3-34).
                let idx = (self.seq_addr as usize * 4 + self.seq_word_byte as usize)
                    & (SEQRAM_BYTES - 1);
                self.seqram[idx]
            }
            _ => self.reg[(offset & 0xFF) as usize],
        }
    }

    /// Write an 8-bit attach register.
    pub fn write8(&mut self, offset: u32, value: u8) {
        match offset {
            HCNTRL => {
                if value & HCNTRL_CHIPRST != 0 {
                    // Preserve the reset-ack readback while asserting reset.
                    self.internal_reset();
                    self.reg[HCNTRL as usize] = value | HCNTRL_CHIPRSTACK;
                    self.seq_paused = true;
                } else {
                    self.reg[HCNTRL as usize] = value & !HCNTRL_CHIPRSTACK;
                    // Clearing the pause bit (while not in powerdown) lets
                    // the sequencer run.
                    if (value & HCNTRL_POWRDN) == 0 && (value & HCNTRL_PAUSE) == 0 {
                        self.seq_paused = false;
                    } else {
                        self.seq_paused = true;
                    }
                }
            }
            SEQADDR0 => {
                self.seq_addr = (self.seq_addr & 0xFF00) | value as u16;
                self.seq_word_byte = 0;
            }
            SEQADDR1 => {
                // Only SEQAADR1_MASK (bit 0) carries addressing info.
                self.seq_addr = (self.seq_addr & 0x00FF) | (((value as u16) & 0x01) << 8);
                self.seq_word_byte = 0;
            }
            SEQRAM => {
                let idx = self.seq_addr as usize * 4 + self.seq_word_byte as usize;
                self.seqram[idx & (SEQRAM_BYTES - 1)] = value;
                self.seq_word_byte += 1;
                if self.seq_word_byte == 4 {
                    self.seq_word_byte = 0;
                    self.seq_addr = self.seq_addr.wrapping_add(1);
                }
            }
            CLRINT => {
                // Write 1 to clear the matching INTSTAT bits.
                self.reg[INTSTAT as usize] &= !value;
                // CLRINT also de-asserts the interrupt output; nothing more
                // to track at this stage.
            }
            _ => {
                self.reg[(offset & 0xFF) as usize] = value;
            }
        }
    }
}

/// Host-memory access needed by the sequencer for DMA (SCB fetch/store and
/// data transfer) against main memory.
pub trait HostMemory {
    fn byte_read(&mut self, addr: u32) -> u8;
    fn byte_write(&mut self, addr: u32, value: u8);
}

// Sequencer opcodes (aicasm/aicasm_insformat.h).
const AIC_OP_OR: u8 = 0x0;
const AIC_OP_AND: u8 = 0x1;
const AIC_OP_XOR: u8 = 0x2;
const AIC_OP_ADD: u8 = 0x3;
const AIC_OP_ADC: u8 = 0x4;
const AIC_OP_ROL: u8 = 0x5;
const AIC_OP_BMOV: u8 = 0x6;
const AIC_OP_MVI16: u8 = 0x7;
const AIC_OP_JMP: u8 = 0x8;
const AIC_OP_JC: u8 = 0x9;
const AIC_OP_JNC: u8 = 0xa;
const AIC_OP_CALL: u8 = 0xb;
const AIC_OP_JNE: u8 = 0xc;
const AIC_OP_JNZ: u8 = 0xd;
const AIC_OP_JE: u8 = 0xe;
const AIC_OP_JZ: u8 = 0xf;

impl Aic7880 {
    /// Current SCB slot as selected through register SCBPTR.
    pub fn scbptr(&self) -> usize {
        (self.reg[SCBPTR as usize] & 0x0f) as usize
    }

    /// Sequencer register-file read. Special registers and indirect
    /// windows are handled before the plain register array:
    /// - ALLONES reads 0xff, ALLZEROS reads 0x00
    /// - the SCB window (0xA0..0xDF) routes through SCBPTR
    /// - 0x20..0x5F is scratch RAM (aliased with the register file)
    fn seq_reg_read(&self, addr: u16) -> u8 {
        let a = addr as u32;
        match a {
            ALLONES => 0xff,
            ALLZEROS => 0x00,
            0x20..=0x5f => self.reg[a as usize],
            0xa0..=0xdf => self.scb[self.scbptr() * 64 + (a as usize - 0xa0)],
            _ => self.reg[a as usize],
        }
    }

    /// Sequencer register-file write with the same windows as
    /// [`Aic7880::seq_reg_read`]. NONE writes are discarded.
    fn seq_reg_write(&mut self, addr: u16, value: u8) {
        let a = addr as u32;
        match a {
            NONE | ALLONES => {}
            0x20..=0x5f => self.reg[a as usize] = value,
            0xa0..=0xdf => {
                let idx = self.scbptr() * 64 + (a as usize - 0xa0);
                self.scb[idx] = value;
            }
            _ => self.write8(a, value),
        }
    }

    /// Fetch the 32-bit instruction word at PC (little-endian, as loaded
    /// through the SEQRAM/SEQADDR download protocol).
    fn seq_fetch(&self) -> u32 {
        let base = (self.pc as usize * 4) & (SEQRAM_BYTES - 1);
        u32::from_le_bytes([
            self.seqram[base],
            self.seqram[(base + 1) & (SEQRAM_BYTES - 1)],
            self.seqram[(base + 2) & (SEQRAM_BYTES - 1)],
            self.seqram[(base + 3) & (SEQRAM_BYTES - 1)],
        ])
    }

    fn seq_push_return(&mut self, addr: u16) {
        if self.call_depth < 8 {
            self.call_stack[self.call_depth as usize] = addr;
            self.call_depth += 1;
        }
    }

    fn seq_pop_return(&mut self) -> u16 {
        if self.call_depth > 0 {
            self.call_depth -= 1;
            self.call_stack[self.call_depth as usize]
        } else {
            0
        }
    }

    fn seq_set_flags(&mut self, zero: bool, carry: bool) {
        let mut f = self.reg[FLAGS as usize] & 0xfc;
        if zero {
            f |= 0x02;
        }
        if carry {
            f |= 0x01;
        }
        self.reg[FLAGS as usize] = f;
    }

    /// Execute a single sequencer instruction. Returns true if a sequencer
    /// exception (unimplemented/illegal instruction) halted the engine.
    pub fn seq_step(&mut self, mem: &mut dyn HostMemory) -> bool {
        if self.seq_paused {
            return false;
        }
        let _ = mem;
        let w = self.seq_fetch();
        let next_pc = (self.pc + 1) & 2047;
        let opcode = ((w >> 27) & 0xf) as u8;
        // Bits 25:17 (destination, 9 bits) and the 10-bit branch address
        // (formats 3/5/6) overlap; decode both views of the field.
        let destination = ((w >> 17) & 0x1ff) as u16;
        let address = ((w >> 17) & 0x3ff) as u16;
        let source = ((w >> 8) & 0x1ff) as u16;
        let immediate = (w & 0xff) as u8;
        let mut halted = false;
        let mut taken = false;
        let mut returned = false;

        match opcode {
            // Branch family (format 3).
            AIC_OP_JMP | AIC_OP_JC | AIC_OP_JNC | AIC_OP_CALL | AIC_OP_JNE
            | AIC_OP_JNZ | AIC_OP_JE | AIC_OP_JZ => {
                taken = self.seq_exec_branch(opcode, source, immediate, address);
                if taken {
                    self.pc = address;
                }
            }
            AIC_OP_MVI16 => {
                eprintln!("ahc: sequencer MVI16 unsupported at pc {}", self.pc);
                halted = true;
            }
            // 8-bit ALU operations (format 1) and shift/rotate (format 2);
            // the ret bit (RET pseudo-op) pops the return address into PC.
            _ => {
                let ret = (w >> 26) & 1 != 0;
                if opcode == AIC_OP_ROL {
                    // Plain shift/rotate, or (opcode_ext 0x80..0x85) an
                    // unused 16-bit ALU operation.
                    if (0x80..=0x85).contains(&immediate) {
                        eprintln!(
                            "ahc: sequencer 16-bit ALU op (0x{:02x}) unsupported at pc {}",
                            immediate, self.pc
                        );
                        halted = true;
                    } else {
                        self.seq_exec_shift(immediate, destination, source);
                    }
                } else if opcode <= AIC_OP_BMOV {
                    self.seq_exec_alu(opcode, destination, source, immediate);
                } else {
                    eprintln!(
                        "ahc: unknown sequencer opcode {:#x} at pc {}",
                        opcode, self.pc
                    );
                    halted = true;
                }
                if ret && !halted {
                    self.pc = self.seq_pop_return();
                    returned = true;
                }
            }
        }

        if halted {
            self.seq_paused = true;
            return true;
        }
        if !taken && !returned {
            self.pc = next_pc;
        }
        self.pc &= 2047;

        // Single-step mode stops the sequencer after one instruction.
        if self.reg[SEQCTL as usize] & SEQCTL_STEP != 0 {
            self.reg[SEQCTL as usize] &= !SEQCTL_STEP;
            self.seq_paused = true;
        }
        false
    }

    fn seq_exec_alu(&mut self, opcode: u8, destination: u16, source: u16, imm: u8) {
        let s = self.seq_reg_read(source);
        let none = destination as u32 == NONE;
        let prev_carry = self.reg[FLAGS as usize] & 0x01;
        let (result, zero, carry): (u8, bool, bool) = match opcode {
            AIC_OP_OR => (s | imm, (s | imm) == 0, false),
            AIC_OP_AND => (s & imm, (s & imm) == 0, false),
            AIC_OP_XOR => (s ^ imm, (s ^ imm) == 0, false),
            AIC_OP_ADD | AIC_OP_ADC => {
                // ADD/ADC compute source + immediate (+ carry). The
                // destination only selects the result register (INC would
                // otherwise double, aicasm emits src==dest).
                let c = if opcode == AIC_OP_ADC { prev_carry as u32 } else { 0 };
                let r = s as u32 + imm as u32 + c;
                ((r & 0xff) as u8, (r & 0xff) == 0, r > 0xff)
            }
            AIC_OP_BMOV => {
                // Block move: copy the source byte into the destination.
                // The immediate selects reset/FIFO side effects only.
                (s, s == 0, prev_carry != 0)
            }
            _ => (0, false, false),
        };
        self.seq_set_flags(zero, carry);
        if !none {
            self.seq_reg_write(destination, result);
        }
    }

    fn seq_exec_shift(&mut self, sc: u8, destination: u16, source: u16) {
        let s = self.seq_reg_read(source);
        let none = destination as u32 == NONE;
        let prev_carry = self.reg[FLAGS as usize] & 0x01;
        let (kind, n) = decode_shift(sc);
        let (result, carry): (u8, bool) = match kind {
            ShiftKind::Shl => {
                if n >= 8 {
                    (0, s & 0x01 != 0)
                } else {
                    (
                        s.wrapping_shl(n as u32) & 0xff,
                        s & (1u8 << (8 - n) as u32) != 0,
                    )
                }
            }
            ShiftKind::Shr => {
                if n >= 8 {
                    (0, s & 0x80 != 0)
                } else {
                    (s >> n as u32, s & (1u8 << (n - 1) as u32) != 0)
                }
            }
            ShiftKind::Rol => {
                if n == 0 {
                    (s, prev_carry != 0)
                } else {
                    // Rotate through 8 bits (widen to avoid debug overflow).
                    let r = ((s as u16) << n as u32) | ((s as u16) >> (8 - n) as u32);
                    (r as u8, s >> (8 - n) as u32 != 0)
                }
            }
            ShiftKind::Ror => {
                if n >= 8 {
                    (s, prev_carry != 0)
                } else {
                    let r = ((s as u16) >> n as u32) | ((s as u16) << (8 - n) as u32);
                    (r as u8, s & (1u8 << (n - 1) as u32) != 0)
                }
            }
        };
        self.seq_set_flags(result == 0, carry);
        if !none {
            self.seq_reg_write(destination, result);
        }
    }

    /// Format 3 branches. Returns true when the branch is taken. The mov
    /// forms (JMP/JC/JNC/CALL) load SINDEX with `source | immediate`; the
    /// test/cmp forms (JNZ/JZ/JNE/JE) combine `source` with `immediate`.
    fn seq_exec_branch(&mut self, opcode: u8, source: u16, imm: u8, _address: u16) -> bool {
        let s = self.seq_reg_read(source);
        let cf = self.reg[FLAGS as usize] & 0x01 != 0;
        match opcode {
            AIC_OP_JMP => {
                self.seq_reg_write(SINDEX as u16, s | imm);
                true
            }
            AIC_OP_JC => {
                self.seq_reg_write(SINDEX as u16, s | imm);
                cf
            }
            AIC_OP_JNC => {
                self.seq_reg_write(SINDEX as u16, s | imm);
                !cf
            }
            AIC_OP_CALL => {
                self.seq_reg_write(SINDEX as u16, s | imm);
                self.seq_push_return((self.pc + 1) & 2047);
                true
            }
            AIC_OP_JZ | AIC_OP_JE => {
                let f = if opcode == AIC_OP_JE { s ^ imm } else { s & imm };
                self.seq_set_flags(f == 0, cf);
                f == 0
            }
            AIC_OP_JNZ | AIC_OP_JNE => {
                let f = if opcode == AIC_OP_JNE { s ^ imm } else { s & imm };
                self.seq_set_flags(f == 0, cf);
                f != 0
            }
            _ => false,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum ShiftKind {
    Shl,
    Shr,
    Rol,
    Ror,
}

/// Decode a format 2 shift_control byte. Emitted values (aicasm_gram.y):
/// SHL n -> (n<<4)|n, n==8 -> 0xf0; SHR n -> (n<<4)|(8-n)|0x08, n==8 -> 0xf8;
/// ROL n -> n&7; ROR n -> (8-n)|0x08.
fn decode_shift(sc: u8) -> (ShiftKind, u8) {
    match sc {
        0xf0 => (ShiftKind::Shl, 8),
        0xf8 => (ShiftKind::Shr, 8),
        _ if sc & 0x08 == 0 => {
            let hi = sc >> 4;
            let lo = sc & 0x07;
            if hi != 0 && hi == lo {
                (ShiftKind::Shl, hi)
            } else if sc == 0 {
                (ShiftKind::Rol, 0)
            } else {
                (ShiftKind::Rol, lo)
            }
        }
        _ => {
            let hi = sc >> 4;
            if hi == 0 {
                // ROR n where sc == (8-n)|8.
                (ShiftKind::Ror, 8 - (sc & 0x07))
            } else {
                (ShiftKind::Shr, hi)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    include!("shipped_seqprog.rs");
    include!("seqprog_7880.rs");

    #[test]
    fn chip_reset_handshake() {
        let mut c = Aic7880::new();
        // Reset with pause asserted; the driver waits for CHIPRSTACK to set.
        c.write8(HCNTRL, HCNTRL_CHIPRST | HCNTRL_PAUSE);
        assert_ne!(c.read8(HCNTRL) & HCNTRL_CHIPRSTACK, 0);
        // Running the chip clears the reset ack.
        c.write8(HCNTRL, HCNTRL_PAUSE);
        assert_eq!(c.read8(HCNTRL) & HCNTRL_CHIPRSTACK, 0);
    }

    #[test]
    fn seqram_download_autoincrement() {
        let mut c = Aic7880::new();
        c.write8(SEQADDR0, 0);
        c.write8(SEQADDR1, 0);
        c.write8(SEQRAM, 0x12); // byte 0 of word 0
        c.write8(SEQRAM, 0x34);
        c.write8(SEQRAM, 0x56);
        c.write8(SEQRAM, 0x78); // MSB: address advances to word 1
        c.write8(SEQRAM, 0x9A); // byte 0 of word 1
        // Verify word 0 via the register window readback.
        c.write8(SEQADDR0, 0);
        c.write8(SEQADDR1, 0);
        assert_eq!(c.read8(SEQRAM), 0x12);
        c.write8(SEQADDR0, 1);
        c.write8(SEQADDR1, 0);
        assert_eq!(c.read8(SEQRAM), 0x9A);
    }

    #[test]
    fn seerdy_reports_ready() {
        let mut c = Aic7880::new();
        assert_ne!(c.read8(SEECTL) & SEECTL_SEERDY, 0);
    }

    struct NullMem;

    impl HostMemory for NullMem {
        fn byte_read(&mut self, _addr: u32) -> u8 {
            0
        }
        fn byte_write(&mut self, _addr: u32, _value: u8) {}
    }

    /// Place a 32-bit little-endian instruction word at a SEQRAM address
    /// through the register download protocol.
    fn put_seq(c: &mut Aic7880, addr: u16, w: u32) {
        c.write8(SEQADDR0, addr as u8);
        c.write8(SEQADDR1, ((addr >> 8) & 1) as u8);
        for b in w.to_le_bytes() {
            c.write8(SEQRAM, b);
        }
    }

    /// Un-pause the sequencer (equivalent to the host clearing HCNTRL PAUSE).
    fn boot(c: &mut Aic7880) {
        c.write8(HCNTRL, 0);
    }

    fn fmt1(opcode: u8, ret: bool, dest: u32, src: u32, imm: u8) -> u32 {
        ((opcode as u32 & 0xf) << 27)
            | ((ret as u32) << 26)
            | ((dest & 0x1ff) << 17)
            | ((src & 0x1ff) << 8)
            | (imm as u32)
    }

    fn fmt3(opcode: u8, src: u32, imm: u8, addr: u32) -> u32 {
        ((opcode as u32 & 0xf) << 27)
            | ((addr & 0x3ff) << 17)
            | ((src & 0x1ff) << 8)
            | (imm as u32)
    }

    #[test]
    fn seq_mvi_via_or_allzeros() {
        let mut c = Aic7880::new();
        // MVI DINDEX, 0x2a -> OR DINDEX, 0x2a, ALLZEROS
        put_seq(&mut c, 0, fmt1(AIC_OP_OR, false, DINDEX, ALLZEROS, 0x2a));
        boot(&mut c);
        c.seq_step(&mut NullMem);
        assert_eq!(c.read8(DINDEX), 0x2a);
    }

    #[test]
    fn seq_mvi_zero_via_bmov() {
        let mut c = Aic7880::new();
        c.write8(DINDEX, 0xff);
        // MVI DINDEX, 0 -> BMOV DINDEX, 1, ALLZEROS
        put_seq(&mut c, 0, fmt1(AIC_OP_BMOV, false, DINDEX, ALLZEROS, 1));
        boot(&mut c);
        c.seq_step(&mut NullMem);
        assert_eq!(c.read8(DINDEX), 0);
    }

    #[test]
    fn seq_mov_copies_byte() {
        let mut c = Aic7880::new();
        c.write8(SINDEX, 0xab);
        // MOV DINDEX, SINDEX -> BMOV
        put_seq(&mut c, 0, fmt1(AIC_OP_BMOV, false, DINDEX, SINDEX, 1));
        boot(&mut c);
        c.seq_step(&mut NullMem);
        assert_eq!(c.read8(DINDEX), 0xab);
    }

    #[test]
    fn seq_add_sets_carry_and_zero() {
        let mut c = Aic7880::new();
        c.write8(DINDEX, 0xff);
        // ADD DINDEX, 1, ALLONES -> 0xff + 1 = 0x00, carry set.
        put_seq(&mut c, 0, fmt1(AIC_OP_ADD, false, DINDEX, ALLONES, 1));
        boot(&mut c);
        c.seq_step(&mut NullMem);
        assert_eq!(c.read8(DINDEX), 0x00);
        assert_ne!(c.read8(FLAGS) & 0x01, 0);
        assert_ne!(c.read8(FLAGS) & 0x02, 0);
    }

    #[test]
    fn seq_stc_sets_carry() {
        let mut c = Aic7880::new();
        // STC -> ADD NONE, 1, ALLONES
        put_seq(&mut c, 0, fmt1(AIC_OP_ADD, false, NONE, ALLONES, 1));
        boot(&mut c);
        c.seq_step(&mut NullMem);
        assert_ne!(c.read8(FLAGS) & 0x01, 0);
    }

    #[test]
    fn seq_clc_clears_carry() {
        let mut c = Aic7880::new();
        c.write8(FLAGS, 0x01);
        // CLC -> ADD NONE, -1, ALLZEROS
        put_seq(&mut c, 0, fmt1(AIC_OP_ADD, false, NONE, ALLZEROS, 0xff));
        boot(&mut c);
        c.seq_step(&mut NullMem);
        assert_eq!(c.read8(FLAGS) & 0x01, 0);
    }

    #[test]
    fn seq_test_jnz_branches() {
        let mut c = Aic7880::new();
        c.write8(SINDEX, 0x10);
        // test SINDEX, 0x10 jnz 7 (taken: 0x10 & 0x10 != 0)
        put_seq(&mut c, 0, fmt3(AIC_OP_JNZ, SINDEX, 0x10, 7));
        // word 7: MVI DINDEX, 0x2d
        put_seq(&mut c, 7, fmt1(AIC_OP_OR, false, DINDEX, ALLZEROS, 0x2d));
        boot(&mut c);
        c.seq_step(&mut NullMem);
        c.seq_step(&mut NullMem);
        assert_eq!(c.read8(DINDEX), 0x2d);
    }

    #[test]
    fn seq_test_jz_not_taken() {
        let mut c = Aic7880::new();
        c.write8(SINDEX, 0x10);
        // test SINDEX, 0x02 jnz 7 (not taken: 0x10 & 2 == 0)
        put_seq(&mut c, 0, fmt3(AIC_OP_JNZ, SINDEX, 0x02, 7));
        put_seq(&mut c, 1, fmt1(AIC_OP_OR, false, DINDEX, ALLZEROS, 0x11));
        boot(&mut c);
        c.seq_step(&mut NullMem);
        c.seq_step(&mut NullMem);
        assert_eq!(c.read8(DINDEX), 0x11);
    }

    #[test]
    fn seq_cmp_jne_equality() {
        let mut c = Aic7880::new();
        c.write8(SINDEX, 0x10);
        // cmp SINDEX, 0x10 jne 7 -> ZERO set, not taken.
        put_seq(&mut c, 0, fmt3(AIC_OP_JNE, SINDEX, 0x10, 7));
        put_seq(&mut c, 1, fmt1(AIC_OP_OR, false, DINDEX, ALLZEROS, 0x33));
        boot(&mut c);
        c.seq_step(&mut NullMem);
        assert_ne!(c.read8(FLAGS) & 0x02, 0); // zero set by the compare
        c.seq_step(&mut NullMem);
        assert_eq!(c.read8(DINDEX), 0x33);
    }

    #[test]
    fn seq_call_and_ret() {
        let mut c = Aic7880::new();
        // word 0: call 3
        put_seq(&mut c, 0, fmt3(AIC_OP_CALL, SINDEX, 0, 3));
        // word 1: MVI DINDEX, 0xed (proves the call returned to pc 1)
        put_seq(&mut c, 1, fmt1(AIC_OP_OR, false, DINDEX, ALLZEROS, 0xed));
        // word 3: RET -> AND NONE, 0xff, ALLZEROS with ret bit
        put_seq(&mut c, 3, fmt1(AIC_OP_AND, true, NONE, ALLZEROS, 0xff));
        boot(&mut c);
        c.seq_step(&mut NullMem);
        assert_eq!(c.read8(SINDEX), 0); // mov form wrote allzeros
        c.seq_step(&mut NullMem);
        assert_eq!(c.read8(DINDEX), 0);
        c.seq_step(&mut NullMem);
        assert_eq!(c.read8(DINDEX), 0xed);
    }

    #[test]
    fn seq_jmp_plain() {
        let mut c = Aic7880::new();
        // jmp 5
        put_seq(&mut c, 0, fmt3(AIC_OP_JMP, SINDEX, 0, 5));
        put_seq(&mut c, 5, fmt1(AIC_OP_OR, false, DINDEX, ALLZEROS, 0x77));
        boot(&mut c);
        c.seq_step(&mut NullMem);
        c.seq_step(&mut NullMem);
        assert_eq!(c.read8(DINDEX), 0x77);
    }

    #[test]
    fn seq_shr_three() {
        let mut c = Aic7880::new();
        c.write8(DINDEX, 0x80);
        // shr A, 3 (shift_control 0x3d): 0x80 >> 3 = 0x10
        put_seq(&mut c, 0, fmt1(AIC_OP_ROL, false, DINDEX, DINDEX, 0x3d));
        boot(&mut c);
        c.seq_step(&mut NullMem);
        assert_eq!(c.read8(DINDEX), 0x10);
        assert_eq!(c.read8(FLAGS) & 0x01, 0);
    }

    #[test]
    fn seq_shl_eight_carries_bit0() {
        let mut c = Aic7880::new();
        c.write8(DINDEX, 0x01);
        // shl A, 8 (shift_control 0xf0): 0x01 << 8 = 0, carry = bit0
        put_seq(&mut c, 0, fmt1(AIC_OP_ROL, false, DINDEX, DINDEX, 0xf0));
        boot(&mut c);
        c.seq_step(&mut NullMem);
        assert_eq!(c.read8(DINDEX), 0x00);
        assert_ne!(c.read8(FLAGS) & 0x01, 0);
    }

    #[test]
    fn seq_rol_rotate() {
        let mut c = Aic7880::new();
        c.write8(DINDEX, 0x81);
        // rol A, 3 (shift_control 0x03): 0x81 -> 0x0c
        put_seq(&mut c, 0, fmt1(AIC_OP_ROL, false, DINDEX, DINDEX, 0x03));
        boot(&mut c);
        c.seq_step(&mut NullMem);
        assert_eq!(c.read8(DINDEX), 0x0c);
    }

    #[test]
    fn seq_scb_window_routes_through_scbptr() {
        let mut c = Aic7880::new();
        c.write8(SCBPTR, 1);
        // MVI SCB_CDB_PTR(0xa0), 0xc0
        put_seq(&mut c, 0, fmt1(AIC_OP_OR, false, 0xa0, ALLZEROS, 0xc0));
        boot(&mut c);
        c.seq_step(&mut NullMem);
        assert_eq!(c.scb[64], 0xc0);
        assert_eq!(c.scb[0], 0);
    }

    #[test]
    fn seq_single_step_pauses() {
        let mut c = Aic7880::new();
        put_seq(&mut c, 0, fmt1(AIC_OP_OR, false, DINDEX, ALLZEROS, 0x01));
        c.write8(SEQCTL, SEQCTL_STEP);
        boot(&mut c);
        c.seq_step(&mut NullMem);
        assert_eq!(c.read8(DINDEX), 0x01);
        assert_eq!(c.seq_step(&mut NullMem), false); // paused, no-op
    }

    #[test]
    fn seq_runs_shipped_7880_firmware() {
        let mut c = Aic7880::new();
        // Download the 880-word microcode exactly as aicasm lays it out.
        for (i, chunk) in SEQPROG_SHIPPED.chunks_exact(4).enumerate() {
            put_seq(&mut c, i as u16, u32::from_le_bytes(chunk.try_into().unwrap()));
        }
        boot(&mut c);
        // With nothing to select the firmware must busy-wait in its
        // poll_for_work / poll_for_selection loops. Hitting any instruction
        // the decoder does not support halts the engine (seq_step -> true).
        assert_eq!(SEQPROG_SHIPPED.len() % 4, 0);
        for i in 0..30_000 {
            let halt = c.seq_step(&mut NullMem);
            assert!(!halt, "sequencer halted at step {}", i);
        }
    }

    #[test]
    fn seq_runs_7880_downloaded_firmware() {
        // Download the program exactly as `ahc_loadseq` writes it after
        // patching for the AIC-7880 (396 words; see seqprog_7880.rs).
        assert_eq!(SEQPROG_7880.len(), 396);
        let mut c = Aic7880::new();
        for (i, w) in SEQPROG_7880.iter().enumerate() {
            put_seq(&mut c, i as u16, *w);
        }
        boot(&mut c);
        for i in 0..50_000 {
            let halt = c.seq_step(&mut NullMem);
            assert!(!halt, "sequencer halted at step {}", i);
        }
    }

    #[test]
    fn seq_downloaded_program_fits_instruction_ram() {
        // AIC-7880 instruction_ram_size is 512 words (aic7xxx_pci.c
        // aic_aic7880_setup); ahc_loadseq asserts the patched program
        // keeps the highest live PC within the RAM.
        assert!(SEQPROG_7880.len() <= 512);
    }

    #[test]
    fn scb_window_matches_hardware_scb_layout() {
        // The window at 0xa0..0xdf is the struct hardware_scb (64 bytes):
        // tag at offset 0x1b -> reg 0xbb, control 0xb8, sgptr 0xb4, etc.
        // (aic7xxx.h). add_scb_to_free_list mvi SCB_TAG, SCB_LIST_NULL
        // is word 392 of the 7880 program: `or+ SCB_TAG, NONE, 0xff`.
        assert_eq!(SEQPROG_7880[392], ((AIC_OP_OR as u32) << 27)
            | (1u32 << 26) | ((0xbb) << 17) | ((NONE as u32) << 8) | 0xff);
        let mut c = Aic7880::new();
        c.write8(SCBPTR, 3);
        c.seq_reg_write(0xbb, 0x7e);
        assert_eq!(c.scb[3 * 64 + 0x1b], 0x7e);
    }
}