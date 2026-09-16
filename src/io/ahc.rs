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
//! disk (target 1) and CD-ROM (target 4); SCSI1 (ahc1) is the external bus.

use crate::io::scsi::ScsiBus;

include!("seqprog_7880.rs");

/// AIC-7880 register block size (BAR0 window).
pub const AIC7880_REG_WINDOW: u32 = 0x100;

/// Diagnostic toggle: set O2RUST_AHC=1 for a register/DMA trace.
pub fn ahc_verbose() -> bool {
    use std::sync::OnceLock;
    static V: OnceLock<bool> = OnceLock::new();
    *V.get_or_init(|| std::env::var_os("O2RUST_AHC").is_some())
}

macro_rules! ahc_trace {
    ($($arg:tt)*) => {
        if crate::io::ahc::ahc_verbose() {
            eprintln!($($arg)*);
        }
    };
}

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
/// CPU cycles per sequencer instruction: the engine clocks at the SCSI rate
/// (~10 MHz) under a ~133 MHz CPU clock.
pub const SEQ_CYCLES_PER_STEP: u64 = 13;
pub const QINFIFO: u32 = 0x9b;
pub const QINCNT: u32 = 0x9c;
pub const QOUTFIFO: u32 = 0x9d;
pub const QOUTCNT: u32 = 0x9e;

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

// OPTIMA hardware-assist dispatch (IP32 `adp78` driver).
/// MACE PCI_NATIVE_VIEW bit OR-ed into host pointers by the PROM driver.
const OPTIMA_PCI_VIEW: u32 = 0x4000_0000;
/// Selection timeout host-adapter status (him_scb.h).
const HOST_SEL_TO: u8 = 0x11;
/// "No status / no response" completion (him_scb.h), for malformed SCBs.
const HOST_NO_STATUS: u8 = 0x01;
/// Scratch offsets (host view, after MACE `off ^ 3` lane steering) where
/// `Ph_movPtrToScratch` positions the two real pointers (him_equ.h
/// SCRATCH_SCB_PTR_ARRAY=0x3e / SCRATCH_QOUT_PTR_ARRAY=0x55, LSB first).
const OPTIMA_SCRATCH_SCB_PTR: usize = 0x3d;
const OPTIMA_SCRATCH_QOUT_PTR: usize = 0x56;

// Sequencer-visible bus control registers (aic7xxx.reg).
pub const SCSISEQ: u32 = 0x00; // TEMODE|ENSELO|ENSELI|ENRSELI|ENAUTOATNO/I/P|SCSIRSTO
pub const SXFRCTL0: u32 = 0x01; // DFON|DFPEXP|FAST20|CLRSTCNT|SPIOEN|SCAMEN|CLRCHN
pub const SXFRCTL1: u32 = 0x02;
pub const SCSISIG: u32 = 0x03; // SCSISIGI (RO read) / SCSISIGO (WO write)
pub const SCSIRATE: u32 = 0x04;
pub const SCSIDATL: u32 = 0x06;
pub const SCSIDATH: u32 = 0x07;
pub const STCNT: u32 = 0x08; // 3 bytes 0x08..0x0a
pub const CLRSINT0: u32 = 0x0b; // == SSTAT0
pub const CLRSINT1: u32 = 0x0c; // == SSTAT1
pub const SSTAT0: u32 = 0x0b; // TARGET|SELDO|SELDI|SELINGO|SWRAP|SDONE|SPIORDY|DMADONE
pub const SSTAT1: u32 = 0x0c; // SELTO|ATNTARG|SCSIRSTI|PHASEMIS|BUSFREE|SCSIPERR|PHASECHG|REQINIT
pub const SIMODE0: u32 = 0x10;
pub const SIMODE1: u32 = 0x11; // ENSELTIMO|ENATNTARG|ENSCSIRST|ENPHASEMIS|ENBUSFREE|ENSCSIPERR|ENPHASECHG|ENREQINIT
pub const SCSIBUSL: u32 = 0x12;
pub const SCSIBUSH: u32 = 0x13;
pub const SINDIR: u32 = 0x6c; // read/write [SINDEX], increments SINDEX
pub const DINDIR: u32 = 0x6d; // read/write [DINDEX], increments DINDEX

pub const DFDAT: u32 = 0x99;
pub const DFWADDR: u32 = 0x95;

// SCSISIG I/O bits (aic7xxx.reg scsisigi).
pub const SCSISIG_CDI: u8 = 0x80;
pub const SCSISIG_IOI: u8 = 0x40;
pub const SCSISIG_MSGI: u8 = 0x20;
pub const SCSISIG_ATNI: u8 = 0x10;
pub const SCSISIG_SELI: u8 = 0x08;
pub const SCSISIG_BSYI: u8 = 0x04;
pub const SCSISIG_REQI: u8 = 0x02;
pub const SCSISIG_ACKI: u8 = 0x01;

// Bus-phase encodings (CDI|IOI|MSGI).
pub const PHASE_DATA_OUT: u8 = 0x00;
pub const PHASE_DATA_IN: u8 = SCSISIG_IOI; // 0x40
pub const PHASE_COMMAND: u8 = SCSISIG_CDI; // 0x80
pub const PHASE_MSG_OUT: u8 = SCSISIG_CDI | SCSISIG_MSGI; // 0xa0
pub const PHASE_STATUS: u8 = SCSISIG_CDI | SCSISIG_IOI; // 0xc0
pub const PHASE_MSG_IN: u8 = SCSISIG_CDI | SCSISIG_IOI | SCSISIG_MSGI; // 0xe0

// SCSISEQ bits.
pub const SCSISEQ_TEMODE: u8 = 0x80;
pub const SCSISEQ_ENSELO: u8 = 0x40;
pub const SCSISEQ_ENSELI: u8 = 0x20;
pub const SCSISEQ_ENRSELI: u8 = 0x10;
pub const SCSISEQ_ENAUTOATNP: u8 = 0x02;

// SXFRCTL0 bits.
pub const SXFR0_CLRSTCNT: u8 = 0x10;
pub const SXFR0_SPIOEN: u8 = 0x08;
pub const SXFR0_CLRCHN: u8 = 0x02;

// SSTAT0.
pub const SSTAT0_SELDO: u8 = 0x40;
pub const SSTAT0_SDONE: u8 = 0x04;
pub const SSTAT0_SPIORDY: u8 = 0x02;
pub const SSTAT0_DMADONE: u8 = 0x01;

// SSTAT1.
pub const SSTAT1_SELTO: u8 = 0x80;
pub const SSTAT1_SCSIRSTI: u8 = 0x20;
pub const SSTAT1_PHASEMIS: u8 = 0x10;
pub const SSTAT1_BUSFREE: u8 = 0x08;
pub const SSTAT1_REQINIT: u8 = 0x01;

// DFCNTRL.
pub const DFCNTRL_SCSIEN: u8 = 0x20;
pub const DFCNTRL_SDMAEN: u8 = 0x10;
pub const DFCNTRL_HDMAEN: u8 = 0x08;
pub const DFCNTRL_DIRECTION: u8 = 0x04;
pub const DFCNTRL_FIFOFLUSH: u8 = 0x02;
pub const DFCNTRL_FIFORESET: u8 = 0x01;

// DFSTATUS.
pub const DFSTATUS_FIFOQWDEMP: u8 = 0x20;
pub const DFSTATUS_MREQPEND: u8 = 0x10;
pub const DFSTATUS_HDONE: u8 = 0x08;
pub const DFSTATUS_DFTHRESH: u8 = 0x04;
pub const DFSTATUS_FIFOFULL: u8 = 0x02;
pub const DFSTATUS_FIFOEMP: u8 = 0x01;

/// AIC-7880 Data FIFO depth (bytes).
pub const FIFO_SIZE: usize = 32;

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
    /// Data FIFO (between SCSI bus and host DMA).
    pub fifo: std::collections::VecDeque<u8>,
    /// Hardware QINFIFO (host pushes SCB numbers, the sequencer pops them).
    /// 12 bytes deep per aic7xxx.reg; QINCNT readback reports occupancy.
    qinfifo: std::collections::VecDeque<u8>,
    /// Hardware QOUTFIFO (sequencer pushes completed SCB numbers, host
    /// pops them). 7 entries per OpenSCSI/dpti docs.
    qoutfifo: std::collections::VecDeque<u8>,
    /// Ticks the bus has been in selection-initiate (ENSELO on).
    sel_ticks: u32,
    /// Debug: number of steps for which to trace the sequencer.
    pub trace_ctr: u32,
    /// The last link phase our engine observed (for PHASCHG/PHASEMIS).
    last_link_phase: u8,
    /// Edge-latch for SSTAT1.REQINIT: armed when the target changes phase or
    /// a byte becomes available, de-armed when the sequencer consumes it.
    reqinit_armed: bool,
    /// Accumulated CPU cycles awaiting sequencer time. The sequencer runs at
    /// the SCSI clock (~10 MHz) while the CPU is ~133 MHz, so we downscale the
    /// cycle count and step the engine from `Emulator::tick_devices`.
    seq_acc: u64,
    /// Host physical address of the driver's `SCB_PTR_ARRAY` (as latched
    /// from `Ph_movPtrToScratch` in the PROM, incl. the PCI_NATIVE_VIEW
    /// bit). Zero until latched.
    scb_ptr_array: u32,
    /// Host physical address of the driver's `QOUT_PTR_ARRAY` (ditto).
    qout_ptr_array: u32,
    /// SCB numbers the host pushed via QINFIFO awaiting emulator dispatch.
    /// Used because the AHC sequencer fetch of the SCB cannot be decoded
    /// faithfully from the on-chip ROM microcode.
    scb_pending: std::collections::VecDeque<u8>,
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
            fifo: std::collections::VecDeque::new(),
            qinfifo: std::collections::VecDeque::new(),
            qoutfifo: std::collections::VecDeque::new(),
            sel_ticks: 0,
            trace_ctr: 0,
            last_link_phase: 0xff,
            reqinit_armed: false,
            seq_acc: 0,
            scb_ptr_array: 0,
            qout_ptr_array: 0,
            scb_pending: std::collections::VecDeque::new(),
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
        self.fifo = Default::default();
        self.qinfifo = Default::default();
        self.qoutfifo = Default::default();
        self.sel_ticks = 0;
        self.trace_ctr = 0;
        self.reqinit_armed = false;
        self.seq_acc = 0;
        self.scb_ptr_array = 0;
        self.qout_ptr_array = 0;
        self.scb_pending = Default::default();
        self.seq_paused = true;
        // The real AIC-7880 has on-chip ROM firmware that survives reset.
        // Reload it so the sequencer can execute immediately.
        self.load_rom();
        // The sequencer is halted after reset until the host runs it.
        self.reg[HCNTRL as usize] &= !HCNTRL_CHIPRST;
    }

    /// Load the on-chip ROM firmware into sequencer RAM.  The real AIC-7880
    /// has a built-in ROM that the sequencer executes from after reset; the
    /// IP32 PROM never downloads firmware — it relies on this ROM being
    /// present.
    fn load_rom(&mut self) {
        for (i, w) in SEQPROG_7880.iter().enumerate() {
            let base = i * 4;
            let bytes = w.to_le_bytes();
            self.seqram[base] = bytes[0];
            self.seqram[base + 1] = bytes[1];
            self.seqram[base + 2] = bytes[2];
            self.seqram[base + 3] = bytes[3];
        }
    }

    /// Read an 8-bit attach register.
    pub fn read8(&mut self, offset: u32) -> u8 {
        match offset {
            SEECTL => self.reg[SEECTL as usize] | SEECTL_SEERDY,
            //
            // Host/sequencer reads of the bus data latch => pop a byte
            // from the live link (for MSG_IN/STATUS byte exchanges).
            SCSIDATL | SCSIBUSL | SCSIDATH | SCSIBUSH => {
                // Pop a byte from the link in any in-direction phase.
                let v = self.pull_bus_byte();
                self.reg[offset as usize] = v;
                v
            }
            // SCSISIGI: live bus wire state driven by the link.
            SCSISIG => self.scsi_sig_in(),
            // DFDAT: pop a byte from the DMA/data FIFO.
            DFDAT => {
                let v = self.fifo.pop_front().unwrap_or(0);
                ahc_trace!("ahc: DFDAT pop => {:#04x} (fifo.len now {})", v, self.fifo.len());
                v
            }
            //
            // DMA status is derived from FIFO + HCNT/DMA state.
            DFSTATUS => self.dfstatus(),
            SEQRAM => {
                // Read the current byte of the word at SEQADDR; the byte
                // position (and word address) advance on reads exactly as on
                // writes (aic7xxx.reg, p. 3-34) so host read-back verification
                // streams work.
                let idx = (self.seq_addr as usize * 4 + self.seq_word_byte as usize)
                    & (SEQRAM_BYTES - 1);
                let v = self.seqram[idx];
                self.seq_word_byte += 1;
                if self.seq_word_byte == 4 {
                    self.seq_word_byte = 0;
                    self.seq_addr = self.seq_addr.wrapping_add(1);
                }
                v
            }
            SINDIR => {
                let v = self.seq_reg_kv_read(self.reg[SINDEX as usize] as u16);
                self.reg[SINDEX as usize] = self.reg[SINDEX as usize].wrapping_add(1);
                v
            }
            CLRSINT0 => self.reg[CLRSINT0 as usize],
            CLRSINT1 => self.reg[CLRSINT1 as usize],
            HCNTRL => self.reg[HCNTRL as usize],
            // QINCNT / QOUTCNT report hardware FIFO depths (aic7xxx.reg).
            QINCNT => self.qinfifo.len() as u8,
            QOUTCNT => self.qoutfifo.len() as u8,
            // Host pops an SCB number from the hardware QOUTFIFO.
            QOUTFIFO => self.qoutfifo.pop_front().unwrap_or(0xff),
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
            INTSTAT => {
                // Sequencer writes OR-set latches ("or INTSTAT, ZERO, X").
                self.reg[INTSTAT as usize] |= value;
            }
            CLRSINT0 => {
                // Writing a 1 clears the latch.
                self.reg[SSTAT0 as usize] &= !value;
            }
            CLRSINT1 => {
                self.reg[SSTAT1 as usize] &= !value;
                if value & SSTAT1_REQINIT != 0 {
                    self.reqinit_armed = false;
                }
            }
            DFCNTRL => {
                self.dfcntrl_write(value);
            }
            DFDAT => {
                self.fifo.push_back(value);
                if self.fifo.len() > FIFO_SIZE {
                    self.fifo.pop_front();
                }
            }
            SCSIDATL | SCSIDATH | SCSIBUSL | SCSIBUSH => {
                self.push_bus_byte(value);
                self.reg[offset as usize] = value;
            }
            SCSISIG => {} // SCSISIGO: bus phases are target-driven.
            SCSISEQ => {
                self.scsiseq_write(value);
            }
            SINDIR => {
                self.seq_reg_kv_write(self.reg[SINDEX as usize] as u16, value);
                self.reg[SINDEX as usize] = self.reg[SINDEX as usize].wrapping_add(1);
            }
            QINFIFO => {
                // Host pushes an SCB number onto the hardware QINFIFO
                // (12 deep; further writes drop, matching the truncated
                // QINCNT spill behavior of the real chip).
                if ahc_verbose() {
                    eprintln!("ahc: QINFIFO push scb={value} (depth now {})", self.qinfifo.len() + 1);
                    if self.trace_ctr == 0 {
                        self.trace_ctr = 4000; // trace next 4000 seq instr
                    }
                }
                if self.qinfifo.len() < 12 {
                    self.qinfifo.push_back(value);
                }
                // Also queue the SCB for the OPTIMA hardware-assist dispatch
                // (the ROM microcode's SCB fetch is not decodable).
                self.scb_pending.push_back(value);
            }
            DINDIR => {
                self.seq_reg_kv_write(self.reg[DINDEX as usize] as u16, value);
                self.reg[DINDEX as usize] = self.reg[DINDEX as usize].wrapping_add(1);
            }
            _ => {
                let o = (offset & 0xFF) as usize;
                self.reg[o] = value;
                if ahc_verbose() && o == 0x05 {
                    eprintln!("ahc: write SCSIID = {:#04x}", value);
                }
            }
        }
        // OPTIMA pointer latch: the IP32 PROM writes SCB_PTR_ARRAY and
        // QOUT_PTR_ARRAY base addresses into sequencer scratch via
        // `Ph_movPtrToScratch` (4 bytes each, one per write). MACE steers
        // byte lanes so the LE little-endian DWORDs land at scratch
        // offsets 0x3d..0x40 (SCB) and 0x56..0x59 (QOUT). Latch them once
        // they carry the PCI_NATIVE_VIEW (0x4000_0000) bit the PROM ORs in.
        let (scb_v, qout_v) = {
            let le = |b0: usize| {
                self.reg[b0] as u32
                    | (self.reg[b0 + 1] as u32) << 8
                    | (self.reg[b0 + 2] as u32) << 16
                    | (self.reg[b0 + 3] as u32) << 24
            };
            (
                le(OPTIMA_SCRATCH_SCB_PTR),
                le(OPTIMA_SCRATCH_QOUT_PTR),
            )
        };
        if scb_v & OPTIMA_PCI_VIEW != 0 {
            self.scb_ptr_array = scb_v;
        }
        if qout_v & OPTIMA_PCI_VIEW != 0 {
            self.qout_ptr_array = qout_v;
        }
    }

    /// Whether the chip's PCI interrupt output (INT#) is currently asserted.
    ///
    /// The 7880 drives INT# low whenever any INTSTAT bit is set and the
    /// host has enabled the interrupt output via HCNTRL.INTEN (the O2
    /// attaches in level-sensitive mode, IRQMS clear).  The host deasserts
    /// it by clearing INTSTAT with a CLRINT write; the sequencer reasserts
    /// it by OR-ing new bits into INTSTAT.
    pub fn int_out(&self) -> bool {
        self.reg[INTSTAT as usize] != 0 && (self.reg[HCNTRL as usize] & HCNTRL_INTEN) != 0
    }
}

/// Host-memory access needed by the sequencer for DMA (SCB fetch/store and
/// data transfer) against main memory.
pub trait HostMemory {
    fn byte_read(&mut self, addr: u32) -> u8;
    fn byte_write(&mut self, addr: u32, value: u8);
}

/// Read a little-endian 32-bit value from host memory. The host writes the
/// OPTIMA SCB array in the chip's little-endian format, so its DWORD fields
/// (SegPtr, CDBPtr, …) are stored LSB-first (`him_scb.h` seq_scb_array).
fn le32_read(mem: &mut dyn HostMemory, addr: u32) -> u32 {
    mem.byte_read(addr) as u32
        | (mem.byte_read(addr.wrapping_add(1)) as u32) << 8
        | (mem.byte_read(addr.wrapping_add(2)) as u32) << 16
        | (mem.byte_read(addr.wrapping_add(3)) as u32) << 24
}

/// Read a big-endian 32-bit value from host memory. The scatter/gather list
/// is never swizzled by the driver, so its DWORDs are stored as a native
/// big-endian MIPS program leaves them.
fn be32_read(mem: &mut dyn HostMemory, addr: u32) -> u32 {
    (mem.byte_read(addr) as u32) << 24
        | (mem.byte_read(addr.wrapping_add(1)) as u32) << 16
        | (mem.byte_read(addr.wrapping_add(2)) as u32) << 8
        | mem.byte_read(addr.wrapping_add(3)) as u32
}

/// Store an OPTIMA completion record into the host SCB: TargStat (byte 12),
/// HaStat (byte 31), residual count zeroed (ResCnt DWORD at 16 and the
/// remaining-length field at 28..30). `scb_addr == 0` writes nothing.
fn store_scb_status(mem: &mut dyn HostMemory, scb_addr: u32, targ: u8, ha: u8) {
    if scb_addr == 0 {
        return;
    }
    mem.byte_write(scb_addr + 12, targ);
    mem.byte_write(scb_addr + 16, 0);
    mem.byte_write(scb_addr + 17, 0);
    mem.byte_write(scb_addr + 18, 0);
    mem.byte_write(scb_addr + 19, 0);
    // DWORD7 layout (MIPS_BE): HaStat occupies the MSB -> memory byte 28;
    // Length fills bytes 29-31.
    mem.byte_write(scb_addr + 28, ha);
    mem.byte_write(scb_addr + 29, 0);
    mem.byte_write(scb_addr + 30, 0);
    mem.byte_write(scb_addr + 31, 0);
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
    /// windows are handled before the plain register array. This variant
    /// has side effects (DFDAT pops the data FIFO, SCSIDATL/SCSIBUSL pull
    /// bus bytes from the link).
    fn seq_reg_read(&mut self, addr: u16) -> u8 {
        let a = addr as u32;
        match a {
            DFDAT => {
                let v = self.fifo.pop_front().unwrap_or(0);
                ahc_trace!("ahc[seq]: DFDAT pop => {:#04x} (fifo.len now {})", v, self.fifo.len());
                v
            }
            SCSIDATL | SCSIBUSL | SCSIDATH | SCSIBUSH => self.pull_bus_byte(),
            SCSISIG => self.scsi_sig_in(),
            SINDIR => {
                let v = self.seq_reg_kv_read(self.reg[SINDEX as usize] as u16);
                self.reg[SINDEX as usize] = self.reg[SINDEX as usize].wrapping_add(1);
                v
            }
            DINDIR => {
                let v = self.seq_reg_kv_read(self.reg[DINDEX as usize] as u16);
                self.reg[DINDEX as usize] = self.reg[DINDEX as usize].wrapping_add(1);
                v
            }
            // Sequencer pops the next SCB number from the hardware QINFIFO.
            QINFIFO => {
                let v = self.qinfifo.pop_front().unwrap_or(0xff);
                if ahc_verbose() {
                    eprintln!("ahc: QINFIFO pop => scb={v} (depth now {})", self.qinfifo.len());
                }
                v
            }
            QINCNT => {
                let v = self.qinfifo.len() as u8;
                if ahc_verbose() {
                    eprintln!("ahc: seq read QINCNT = {v}");
                }
                v
            }
            _ => self.seq_reg_kv_read(addr),
        }
    }

    /// Raw register-file read (no side effects). Used by [`Self::read8`]
    /// and the sequencer: does not pop bus bytes.
    fn seq_reg_kv_read(&self, addr: u16) -> u8 {
        let a = addr as u32;
        match a {
            ALLONES => 0xff,
            ALLZEROS => 0x00,
            DFSTATUS => self.dfstatus(),
            0x20..=0x5f => self.reg[a as usize],
            0xa0..=0xdf => self.scb[self.scbptr() * 64 + (a as usize - 0xa0)],
            // 0x100..0x1ff is the 7880 sequencer's scratch-RAM bank (the
            // Lance-class microcode uses it for scratch/q-in cursors).
            0x100..=0x1ff => self.sram[(a & 0xff) as usize],
            // Out-of-window sequencer addresses (banked SRAM pages beyond
            // the host-visible scratch RAM) read as zero.
            _ => self
                .reg
                .get((a & 0xFF) as usize)
                .copied()
                .unwrap_or(0),
        }
    }

    /// Sequencer register-file write with the same windows as
    /// [`Aic7880::seq_reg_read`]. NONE writes are discarded.
    fn seq_reg_write(&mut self, addr: u16, value: u8) {
        self.seq_reg_kv_write(addr, value);
    }

    fn seq_reg_kv_write(&mut self, addr: u16, value: u8) {
        let a = addr as u32;
        match a {
            NONE | ALLONES => {}
            0x20..=0x5f => self.reg[a as usize] = value,
            QOUTFIFO => {
                // Sequencer reports a completed SCB: push to the host's
                // completion queue (7 deep on this generation).
                if self.qoutfifo.len() < 7 {
                    self.qoutfifo.push_back(value);
                }
            }
            0xa0..=0xdf => {
                let idx = self.scbptr() * 64 + (a as usize - 0xa0);
                self.scb[idx] = value;
            }
            0x100..=0x1ff => {
                self.sram[(a & 0xff) as usize] = value;
            }
            _ if a <= 0xff => self.write8(a, value),
            _ => {}
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

    /// Pop the return stack. `None` means the stack was empty: a taken
    /// `ret` then bounces to the idle-loop entry (word 0), which is how the
    /// OPTIMA poll (`add ..., QINCNT, -1 ret`) spins at the top of its loop
    /// until the host enqueues an SCB.
    fn seq_pop_return(&mut self) -> Option<u16> {
        if self.call_depth > 0 {
            self.call_depth -= 1;
            Some(self.call_stack[self.call_depth as usize])
        } else {
            None
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

    // ===================================================================
    // SCSI/DMA byte engine
    // ===================================================================

    /// Current SCB window base in `self.scb`.
    #[inline]
    fn scb_off(&self) -> usize {
        self.scbptr() * 64
    }

    /// 3-byte counter helpers. STCNT (0x08..0x0a) counts SCSI bus bytes;
    /// HCNT (0x8c..0x8e) counts host bytes moved by the DMA engine.
    fn stcnt(&self) -> u32 {
        self.reg[0x08] as u32 | (self.reg[0x09] as u32) << 8 | (self.reg[0x0a] as u32) << 16
    }
    fn set_stcnt(&mut self, v: u32) {
        self.reg[0x08] = v as u8;
        self.reg[0x09] = (v >> 8) as u8;
        self.reg[0x0a] = (v >> 16) as u8;
    }
    fn hcnt(&self) -> u32 {
        self.reg[0x8c] as u32 | (self.reg[0x8d] as u32) << 8 | (self.reg[0x8e] as u32) << 16
    }
    fn set_hcnt(&mut self, v: u32) {
        self.reg[0x8c] = v as u8;
        self.reg[0x8d] = (v >> 8) as u8;
        self.reg[0x8e] = (v >> 16) as u8;
    }
    fn haddr(&self) -> u32 {
        self.reg[0x88] as u32
            | (self.reg[0x89] as u32) << 8
            | (self.reg[0x8a] as u32) << 16
            | (self.reg[0x8b] as u32) << 24
    }
    fn set_haddr(&mut self, a: u32) {
        self.reg[0x88] = a as u8;
        self.reg[0x89] = (a >> 8) as u8;
        self.reg[0x8a] = (a >> 16) as u8;
        self.reg[0x8b] = (a >> 24) as u8;
    }

    /// Whether the initiator-side link is up (a target is selected).
    fn link_active(&self) -> bool {
        self.bus.link.target.is_some()
    }

    /// DFSTATUS as computed from FIFO and DMA engine state.
    fn dfstatus(&self) -> u8 {
        let mut s = 0u8;
        if self.fifo.is_empty() {
            s |= DFSTATUS_FIFOEMP;
        }
        if self.fifo.len() < 4 {
            s |= DFSTATUS_FIFOQWDEMP;
        }
        if self.fifo.len() >= FIFO_SIZE {
            s |= DFSTATUS_FIFOFULL;
        }
        // DFTHRESH: hardware asserts when the FIFO has >= 4 bytes of data
        // (for into-SCSI) or <= 4 free (for out).
        let dfcntrl = self.reg[DFCNTRL as usize];
        if dfcntrl & DFCNTRL_DIRECTION != 0 {
            if self.fifo.len() >= self.fifo.len().min(4) {
                s |= DFSTATUS_DFTHRESH;
            }
        } else if self.fifo.len() >= 4 {
            s |= DFSTATUS_DFTHRESH;
        }
        if self.reg[DFSTATUS as usize] & DFSTATUS_HDONE != 0 {
            s |= DFSTATUS_HDONE;
        }
        if self.reg[DFSTATUS as usize] & DFSTATUS_MREQPEND != 0 {
            s |= DFSTATUS_MREQPEND;
        }
        s
    }

    /// Live SCSI bus signal byte as seen at SCSISIGI.
    fn scsi_sig_in(&self) -> u8 {
        let mut v = 0u8;
        if self.link_active() {
            // The target drives C/D, I/O, MSG.
            let phase = self.bus.link_phase();
            v |= phase & (SCSISIG_CDI | SCSISIG_IOI | SCSISIG_MSGI);
            // REQ rises when the target has a byte waiting or a
            // phase-transition post.
            if self.bus.link.req || self.bus.link_has_in_byte() || self.bus.link_wants_out_byte()
            {
                v |= SCSISIG_REQI;
            }
            v |= SCSISIG_BSYI;
        }
        v
    }

    /// Write a byte into the SCSI data FIFO (cmd/data out stages) or the
    /// active target (message out). The chip writes via SCSIDATL only in
    /// MSG_OUT/status phases and via the FIFO for command/data phases (the
    /// DMA engine drains FIFO -> bus under SDMAEN/SCSIEN).
    fn push_bus_byte(&mut self, v: u8) {
        if !self.link_active() {
            return;
        }
        match self.bus.link_phase() & (SCSISIG_CDI | SCSISIG_IOI | SCSISIG_MSGI) {
            PHASE_MSG_OUT => {
                self.bus.link_out_byte(v);
                // Byte consumed by target; SEQ waits via phase_lock.
            }
            _ => {}
        }
    }

    /// Read a byte from the SCSI bus into the chip (PIO read paths:
    /// SCSIDATL/SCSIBUSL). Only valid in IN-direction phases.
    fn pull_bus_byte(&mut self) -> u8 {
        if !self.link_active() {
            return 0xff;
        }
        match self.bus.link_phase() & (SCSISIG_CDI | SCSISIG_IOI | SCSISIG_MSGI) {
            PHASE_MSG_IN => self.bus.link_next_msg_in().unwrap_or(0),
            PHASE_STATUS => {
                // The link's `in_byte` returns the status byte, advances to
                // MSG_IN and queues COMMAND_COMPLETE; the next MSG_IN read
                // then pops it and releases the bus.
                self.bus.link_in_byte()
            }
            _ => 0xff,
        }
    }

    /// SCSISEQ write: selection trigger. With ENSELO set the chip drives
    /// the target selection; we complete it a few ticks later.
    fn scsiseq_write(&mut self, value: u8) {
        let old = self.reg[SCSISEQ as usize];
        self.reg[SCSISEQ as usize] = value;
        if value & SCSISEQ_ENSELO != 0 && old & SCSISEQ_ENSELO == 0 {
            self.sel_ticks = 0;
        }
    }

    /// DFCNTRL write hook (FIFO reset/flush are one-shot actions).
    fn dfcntrl_write(&mut self, value: u8) {
        if value & DFCNTRL_FIFORESET != 0 {
            // FIFORESET resets the FIFO pointers only; HADDR/HCNT are
            // retained (the microcode relies on this: post_byte_setup sets
            // HCNT then raises FIFORESET, dma_scb likewise).
            self.fifo.clear();
            self.reg[DFSTATUS as usize] &= !DFSTATUS_HDONE;
        }
        if value & DFCNTRL_FIFOFLUSH != 0 {
            // FIFOFLUSH commits the current FIFO to the host side (d2h) or
            // finishes an in-flight drain (h2d). For our instant-DMA model
            // this is a soft latch: no state to flush beyond what the engine
            // already transferred.
            self.reg[DFSTATUS as usize] &= !DFSTATUS_HDONE;
        }
        let old = self.reg[DFCNTRL as usize];
        self.reg[DFCNTRL as usize] = value & !(DFCNTRL_FIFORESET | DFCNTRL_FIFOFLUSH);
        // Real chip clears the completion latches (SDONE/DMADONE/HDONE) on the
        // rising edge of the corresponding enable bit - i.e. the write that
        // ARMS a new transfer - not on every write that happens to carry the
        // bit.  The sequencer relies on this: it frequently does
        // `or DFCNTRL, FIFOFLUSH` with SCSIEN already set (p_command_xfer)
        // and tests SSTAT0.SDONE right after (aic7xxx.seq:1415-1445).  With
        // edge detection a fresh dma_loop still sees a clean slate while the
        // in-phase SDONE/DMADONE survives until the next real tranfer start.
        if value & DFCNTRL_HDMAEN != 0 && old & DFCNTRL_HDMAEN == 0 {
            self.reg[SSTAT0 as usize] &= !SSTAT0_DMADONE;
            self.reg[DFSTATUS as usize] &= !DFSTATUS_HDONE;
        }
        if value & DFCNTRL_SCSIEN != 0 && old & DFCNTRL_SCSIEN == 0 {
            self.reg[SSTAT0 as usize] &= !SSTAT0_SDONE;
        }
    }

    /// One engine step (called once per sequencer instruction plus once per
    /// MACE tick): move host DMA bytes, SCSI bytes, and latches.
    pub fn dma_scsi_step(&mut self, mem: &mut dyn HostMemory) {
        self.hardware_assist_scb(mem);
        self.selection_update();
        self.host_dma(mem);
        self.scsi_byte_move();
        self.update_latching();
    }

    /// OPTIMA mode hardware-assisted SCB dispatch.
    ///
    /// The IP32 PROM (`adp78.c`/`himdopt.c`) enqueues SCSI commands by
    /// writing the SCB number to QINFIFO after the microcode's SCB search
    /// drives selection — but the on-chip ROM microcode of our model cannot
    /// be decoded faithfully, so we complete the SCB directly: fetch the
    /// 32-byte OPTIMA SCB (little-endian, `him_scb.h`) the driver placed in
    /// host RAM, run the CDB against the attached device, DMA the data to
    /// the scatter/gather list, write the result into the SCB's
    /// TargStat/HaStat bytes, post the SCB number to `QOUT_PTR_ARRAY`, and
    /// raise CMDCMPLT.  This satisfies `adp_poll_status`/`PH_IntHandler`
    /// which expect the QOUT entry and CMDCMPLT once the transfer is done.
    fn hardware_assist_scb(&mut self, mem: &mut dyn HostMemory) {
        if self.scb_ptr_array == 0 {
            return;
        }
        let Some(scb_num) = self.scb_pending.pop_front() else {
            return;
        };
        // The poll loop pauses the sequencer; a second enque is not pushed
        // until the previous one is drained. Guard depth regardless.
        if self.scb_pending.len() > 0 {
            self.scb_pending.clear();
        }

        let complete = |self_: &mut Self, mem: &mut dyn HostMemory, scb_addr: u32, targ: u8, ha: u8, scb_num: u8| {
            // Write the SCSI status into the host SCB (byte 12 = TargStat,
            // byte 31 = HaStat; clear residual/remaining-length fields).
            store_scb_status(mem, scb_addr, targ, ha);
            self_.post_optima_qout(mem, scb_num);
            self_.reg[INTSTAT as usize] |= INTSTAT_CMDCMPLT;
            ahc_trace!(
                "ahc: hw-assist scb={} done targ={:#04x} hastat={:#04x}",
                scb_num,
                targ,
                ha
            );
        };

        let base = self.scb_ptr_array & !OPTIMA_PCI_VIEW;
        let entry = if scb_num < 32 {
            be32_read(mem, base + (scb_num as u32) * 4)
        } else {
            0xffff_ffff
        };
        // A freed/reused slot (0xffffffff) or NULL_SCB (0xff) being pushed
        // would hang the poll; surface an error completion instead.
        let scb_addr = entry & !OPTIMA_PCI_VIEW;
        if entry == 0xffff_ffff || entry == 0x0000_00ff || scb_addr == 0 {
            ahc_trace!("ahc: hw-assist scb={} bad SCB_PTR_ARRAY entry {:#010x}", scb_num, entry);
            complete(self, mem, 0, 0, HOST_NO_STATUS, scb_num);
            return;
        }

        // MIPS_BE SCB layout (him_scb.h): the bitfield is reversed ("this
        // word field was switched") so SegCnt lands at byte0, CDBLen at
        // byte1, flags at byte2, and Tarlun at byte3; pointer DWORDs stay
        // native big-endian.
        let segcnt = mem.byte_read(scb_addr) as usize;
        let cdblen = (mem.byte_read(scb_addr + 1) as usize).max(6);
        let tarlun = mem.byte_read(scb_addr + 3);
        let target = tarlun >> 4;
        let segptr = be32_read(mem, scb_addr + 4) & !OPTIMA_PCI_VIEW;
        let cdbptr = be32_read(mem, scb_addr + 8) & !OPTIMA_PCI_VIEW;

        let mut cdb = [0u8; 16];
        let n = cdblen.min(16);
        for i in 0..n {
            cdb[i] = mem.byte_read(cdbptr + i as u32);
        }

        // Scatter/gather list: 8-byte elements {data_ptr, data_len} (BE
        // DWORDs; the driver "never swizzles" them). Len's top bit set =
        // SG_LAST_ELEMENT. No-data commands carry sg[0] = {0, LAST}.
        let nsg = segcnt.min(64);
        let mut sgs: Vec<(u32, u32)> = Vec::new();
        let mut total: usize = 0;
        if segptr != 0 {
            for i in 0..nsg {
                let p = segptr + (i as u32) * 8;
                let a = be32_read(mem, p) & !OPTIMA_PCI_VIEW;
                let l = be32_read(mem, p + 4) & 0x7fff_ffff;
                if l == 0 {
                    continue;
                }
                if a == 0 && total == 0 {
                    continue; // empty "segment" for no-data commands
                }
                sgs.push((a, l));
                total += l as usize;
                if total > (1 << 20) {
                    break;
                }
            }
        }

        if !self.bus.is_attached(target) {
            ahc_trace!("ahc: hw-assist scb={} select target {} SELTO", scb_num, target);
            complete(self, mem, scb_addr, 0, HOST_SEL_TO, scb_num);
            return;
        }

        // Pre-fill the data buffer from host memory: read commands overwrite
        // it, write commands consume it (sector-aligned payload).
        let mut data = vec![0u8; total];
        {
            let mut off = 0usize;
            for &(a, l) in &sgs {
                let n = l as usize;
                for k in 0..n {
                    data[off + k] = mem.byte_read(a + k as u32);
                }
                off += n;
            }
        }

        let exec = self.bus.exec_cdb(target, &cdb, &mut data);
        use crate::io::scsi::ScsiPhase;
        let mut status = exec.status;
        match exec.phase {
            ScsiPhase::DataIn { len } => {
                let mut off = 0usize;
                let mut left = len.min(data.len());
                let mut it = sgs.iter();
                while left > 0 {
                    let Some(&(a, l)) = it.next() else { break };
                    let n = (l as usize).min(left);
                    for k in 0..n {
                        mem.byte_write(a + k as u32, data[off + k]);
                    }
                    off += n;
                    left -= n;
                }
                // Residual: the chip reports untransferred bytes in Length and
                // ResCnt; zero is fine for the PROM's fixed-size reads.
            }
            ScsiPhase::NoData | ScsiPhase::DataOut { .. } => {
                if status != crate::io::scsi::STATUS_GOOD {
                    // keep status; driver routes CHECK CONDITION via TargStat
                } else {
                    status = crate::io::scsi::STATUS_GOOD;
                }
            }
        }

        ahc_trace!(
            "ahc: hw-assist scb={} target={} cdb={:02x} status={:#04x} sg={} bytes={}/{total}",
            scb_num,
            target,
            cdb[0],
            status,
            sgs.len(),
            match exec.phase { ScsiPhase::DataIn { len } => len, _ => 0 }
        );
        complete(self, mem, scb_addr, status, 0, scb_num);
    }

    /// Post an OPTIMA completion: write the SCB number into the host
    /// `QOUT_PTR_ARRAY` (entry 4 matches the driver's `qout_index` start and
    /// is found by `PH_IntHandler`'s stride-1 scan).
    fn post_optima_qout(&mut self, mem: &mut dyn HostMemory, scb_num: u8) {
        let base = self.qout_ptr_array & !OPTIMA_PCI_VIEW;
        if base == 0 {
            return;
        }
        mem.byte_write(base + 4, scb_num);
    }

    fn selection_update(&mut self) {
        let scsiseq = self.reg[SCSISEQ as usize];
        if scsiseq & SCSISEQ_ENSELO != 0 && !self.link_active() {
            self.sel_ticks += 1;
            // Completes quickly when a target answers; after ~64 ticks with
            // no response, raise SELTO so the sequencer's interrupt path
            // sees the selection timeout (real HW uses SIMODE1 timer).
            if self.sel_ticks >= 2 {
                let scsiid = self.reg[SCSIID as usize];
                let target = scsiid >> 4;
                if self.bus.link_device_present(target % 16) {
                    self.bus.link_select(target % 16);
                    self.reg[SSTAT0 as usize] |= SSTAT0_SELDO;
                    self.reg[SSTAT1 as usize] &= !SSTAT1_SELTO;
                    self.reg[SSTAT1 as usize] |= SSTAT1_REQINIT;
                    self.reqinit_armed = true;
                    ahc_trace!("ahc: select target {} ok", target);
                } else {
                    // No unit answered: after ~64 ticks surface SSTAT1.SELTO.
                    // The sequencer then abandons the attempt (real chip) —
                    // release ENSELO so a misdecoded selector cannot spin the
                    // engine forever on an empty bus.
                    if self.sel_ticks >= 64 {
                        if self.reg[SSTAT1 as usize] & SSTAT1_SELTO == 0 {
                            if ahc_verbose() {
                                eprintln!("ahc: select target {} TIMEOUT (SELTO)", target);
                            }
                            self.reg[SSTAT1 as usize] |= SSTAT1_SELTO;
                        }
                        self.reg[SCSISEQ as usize] &= !SCSISEQ_ENSELO;
                        self.sel_ticks = 0;
                    } else if self.sel_ticks <= 3 {
                        ahc_trace!("ahc: select target {} (no device yet, tick {})", target, self.sel_ticks);
                    }
                }
            }
        } else {
            self.sel_ticks = 0;
        }
    }

    /// Host <-> FIFO DMA moves.
    fn host_dma(&mut self, mem: &mut dyn HostMemory) {
        let dfcntrl = self.reg[DFCNTRL as usize];
        if dfcntrl & DFCNTRL_HDMAEN == 0 {
            return;
        }
        let mut moved = false;
        if ahc_verbose() {
            ahc_trace!(
                "ahc: host_dma dir={} fifo.len={} haddr={:#08x} hcnt={}",
                dfcntrl & DFCNTRL_DIRECTION != 0,
                self.fifo.len(),
                self.haddr(),
                self.hcnt()
            );
        }
        // Move up to FAIRNESS bytes per call to keep simulation latency
        // bounded.
        for _ in 0..8 {
            if self.hcnt() == 0 {
                break;
            }
            if dfcntrl & DFCNTRL_DIRECTION != 0 {
                // DIRECTION set = data moves FROM host INTO the card
                // (aic7xxx.reg DFCNTRL: "1 = host read = from host").
                if self.fifo.len() < FIFO_SIZE {
                    let b = mem.byte_read(self.haddr());
                    self.fifo.push_back(b);
                    self.set_haddr(self.haddr().wrapping_add(1));
                    self.set_hcnt(self.hcnt() - 1);
                    moved = true;
                } else {
                    break;
                }
            } else {
                // DIRECTION clear = data moves from the card/FIFO TO host.
                if let Some(b) = self.fifo.pop_front() {
                    mem.byte_write(self.haddr(), b);
                    self.set_haddr(self.haddr().wrapping_add(1));
                    self.set_hcnt(self.hcnt() - 1);
                    moved = true;
                } else {
                    break;
                }
            }
        }
        if self.hcnt() == 0 && (moved || true) {
            self.reg[DFSTATUS as usize] |= DFSTATUS_HDONE;
            self.reg[SSTAT0 as usize] |= SSTAT0_DMADONE;
        }
        let _ = moved;
    }

    /// SCSI side byte movement: bytes between the FIFO and the bus.
    fn scsi_byte_move(&mut self) {
        let dfcntrl = self.reg[DFCNTRL as usize];
        if dfcntrl & DFCNTRL_SCSIEN == 0 {
            return;
        }
        // Only one byte per tick is moved.
        if self.stcnt() == 0 {
            return;
        }
        match self.bus.link_phase() & (SCSISIG_CDI | SCSISIG_IOI | SCSISIG_MSGI) {
            PHASE_COMMAND | PHASE_DATA_OUT => {
                // Bus <- FIFO (out).
                if let Some(b) = self.fifo.pop_front() {
                    self.bus.link_out_byte(b);
                    let s = self.stcnt().saturating_sub(1);
                    self.set_stcnt(s);
                    if s == 0 {
                        self.reg[SSTAT0 as usize] |= SSTAT0_SDONE;
                        self.reg[SSTAT1 as usize] |= SSTAT1_PHASEMIS;
                    }
                }
            }
            PHASE_DATA_IN => {
                // Bus -> FIFO (in).
                if self.fifo.len() < FIFO_SIZE {
                    if let Some(b) = self.bus.link_next_data() {
                        self.fifo.push_back(b);
                        let s = self.stcnt().saturating_sub(1);
                        self.set_stcnt(s);
                        if s == 0 {
                            self.reg[SSTAT0 as usize] |= SSTAT0_SDONE;
                            self.reg[SSTAT1 as usize] |= SSTAT1_PHASEMIS;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    /// Update SSTAT* flags based on engine state; generate PHASEMIS.
    fn update_latching(&mut self) {
        // REQINIT: a target-driven latch that rises on a phase change or
        // byte-ready/want event and must be cleared by CLRSINT1 (without the
        // chip re-arming it until the next event).
        if self.link_active() && self.reqinit_armed {
            self.reg[SSTAT1 as usize] |= SSTAT1_REQINIT;
        }
        // Detect a phase change from the target side: latch PHASECHG and,
        // if a data phase just ended with the chip mid-DMA, also PHASEMIS.
        let phase = self.bus.link.phase;
        if self.link_active() && self.last_link_phase != phase {
            let was_data = matches!(
                self.last_link_phase & 0xe0,
                PHASE_DATA_IN | PHASE_DATA_OUT | PHASE_COMMAND | PHASE_MSG_OUT | PHASE_MSG_IN
            );
            self.reg[SSTAT1 as usize] |= 0x02; // PHASECHG
            self.reqinit_armed = true;
            // The hardware asserts PHASEMIS when a DMA-phase ends.
            if was_data && phase & 0xe0 == 0xc0 /* status-side */ {
                self.reg[SSTAT1 as usize] |= SSTAT1_PHASEMIS;
                self.reg[SSTAT0 as usize] |= SSTAT0_DMADONE;
            }
            self.last_link_phase = phase;
        }
        // If the link has gone bus-free (command finished), latch BUSFREE.
        if self.bus.link.released || self.bus.link.free {
            if self.link_target_on() {
                self.reg[SSTAT1 as usize] |= SSTAT1_BUSFREE;
                self.reqinit_armed = false;
                self.reg[SSTAT1 as usize] &= !SSTAT1_REQINIT;
                self.bus.link.free = false;
                self.bus.link.released = false;
                self.last_link_phase = 0xff;
                self.reg[SCSISIG as usize] = 0; // no bus signals when free
            }
        }
    }

    fn link_target_on(&self) -> bool {
        self.bus.link.target.is_some() || self.bus.link.released
    }

    /// Disassemble one sequencer word (for the `_verbose` trace hook).
    fn seq_disassemble(&self, w: u32) -> String {
        const OP: [&str; 16] = [
            "or", "and", "xor", "add", "adc", "rol", "bmov", "mvi16", "jmp", "jc", "jnc",
            "call", "jne", "jnz", "je", "jz",
        ];
        let op = ((w >> 27) & 0xf) as usize;
        let ret = (w >> 26) & 1 != 0;
        let dst = (w >> 17) & 0x1ff;
        let src = (w >> 8) & 0x1ff;
        let imm = w & 0xff;
        if op >= 8 {
            format!(
                "{}{} src={:03x} imm={:02x} ->{}",
                OP[op],
                if ret { "+" } else { "" },
                src,
                imm,
                (w >> 17) & 0x3ff
            )
        } else {
            format!(
                "{}{} dst={:03x} src={:03x} imm={:02x}",
                OP[op],
                if ret { "+" } else { "" },
                dst,
                src,
                imm
            )
        }
    }

    /// Execute a single sequencer instruction. Returns true if a sequencer
    /// exception (unimplemented/illegal instruction) halted the engine.
    pub fn seq_step(&mut self, mem: &mut dyn HostMemory) -> bool {
        if self.seq_paused {
            return false;
        }
        // Advance the SCSI link + DMA engine alongside the sequencer clock.
        self.dma_scsi_step(mem);
        let w = self.seq_fetch();
        if ahc_verbose() && self.trace_ctr > 0 {
            self.trace_ctr -= 1;
            let op = (w >> 27) & 0xf;
            ahc_trace!(
                "pc={:04x} w={:08x} op={:x} ASM={} scsiseq={:02x} scsiid={:02x} intstat={:02x} sstat0={:02x} sstat1={:02x} lastphase={:02x} dfcntrl={:02x} dfstatus={:02x} fifo={} stcnt={} hcnt={}",
                self.pc,
                w,
                op,
                self.seq_disassemble(w),
                self.reg[SCSISEQ as usize],
                self.reg[SCSIID as usize],
                self.reg[INTSTAT as usize],
                self.reg[SSTAT0 as usize],
                self.reg[SSTAT1 as usize],
                self.reg[0x3f],
                self.reg[DFCNTRL as usize],
                self.reg[DFSTATUS as usize],
                self.fifo.len(),
                self.stcnt(),
                self.hcnt()
            );
        }
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
                    // The `ret` pseudo-op is a conditional return: taken
                    // when the ALU result of this instruction is non-zero
                    // (Z clear). The OPTIMA idle poll is `add SCRATCH0,
                    // QINCNT, -1 ret` -> QINCNT==0 yields 0xff (NZ) so it
                    // bounces back to the top of the poll loop, while
                    // QINCNT>=1 yields 0 (Z) so it falls through to pop
                    // QINFIFO. An empty return stack falls back to the
                    // idle entry (word 0).
                    if self.reg[FLAGS as usize] & 0x02 == 0 {
                        self.pc = self.seq_pop_return().unwrap_or(0);
                        returned = true;
                    }
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

    /// Advance the sequencer across `delta` CPU cycles.
    ///
    /// The 7880's sequencer clocks at the SCSI frequency (~10 MHz) against a
    /// ~133 MHz CPU, so one instruction is executed per [`SEQ_CYCLES_PER_STEP`]
    /// elapsed cycles. The host-side DMA (SCB fetch, data transfer) lands in
    /// guest RAM through `mem`. Called from the device tick so a guest driver
    /// making real SCSI progress gets it during `Emulator::run`.
    pub fn advance_cycles(&mut self, delta: u64, mem: &mut dyn HostMemory) {
        if self.seq_paused || delta == 0 {
            return;
        }
        self.seq_acc = self.seq_acc.saturating_add(delta);
        let mut steps = 0u64;
        while !self.seq_paused && self.seq_acc >= SEQ_CYCLES_PER_STEP && steps < delta {
            let halted = self.seq_step(mem);
            self.seq_acc -= SEQ_CYCLES_PER_STEP;
            steps += 1;
            if halted {
                break;
            }
        }
        // Drop any backlog accumulated while paused (chip can't have been
        // executing); keeps latemost timebases from bursting.
        if self.seq_acc > (1 << 20) {
            self.seq_acc = 0;
        }
    }

    fn seq_exec_alu(&mut self, opcode: u8, destination: u16, source: u16, imm: u8) {
        let s = self.seq_reg_read(source);
        let none = destination as u32 == NONE;
        let prev_carry = self.reg[FLAGS as usize] & 0x01;
        // aicasm's immediate_or_a rejects a literal 0 in favor of `A`
        // (ACCUM). The hardware therefore decodes imm==0 as "use ACCUM".
        let imm = if imm == 0 { self.reg[0x64] } else { imm };
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
        // aicasm rejects a literal 0 immediate here (use `A`); an imm==0
        // encoding means "compare against ACCUM". See fmt3 immediate_or_a.
        let test_imm = if imm == 0 { self.reg[0x64] } else { imm };
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
                let f = if opcode == AIC_OP_JE { s ^ test_imm } else { s & test_imm };
                self.seq_set_flags(f == 0, cf);
                f == 0
            }
            AIC_OP_JNZ | AIC_OP_JNE => {
                let f = if opcode == AIC_OP_JNE { s ^ test_imm } else { s & test_imm };
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

    // ================================================================
    // End-to-end: drive a single SCB (TEST_UNIT_READY -> INQUIRY) through
    // the downloaded 7880 microcode and the emulated bus target.
    // ================================================================

    /// Ram-backed guest memory for the sequencer DMA engine.
    struct VecMem {
        ram: Vec<u8>,
    }
    impl HostMemory for VecMem {
        fn byte_read(&mut self, addr: u32) -> u8 {
            *self.ram.get(addr as usize).unwrap_or(&0)
        }
        fn byte_write(&mut self, addr: u32, value: u8) {
            if (addr as usize) < self.ram.len() {
                self.ram[addr as usize] = value;
            }
        }
    }

    const HOST_SCB_BASE: u32 = 0x2_0000; // HSCB array base in guest RAM
    const SHARED_DATA: u32 = 0x3_0000; // queue-in/out host ring (qinfo)
    const DATA_BUF: u32 = 0x4_0000; // data-in landing pad

    fn le32(v: u32) -> [u8; 4] {
        v.to_le_bytes()
    }

    fn write_scb(mem: &mut VecMem, tag: u8, cdb: &[u8], dataptr: u32, datacnt: u32) {
        let base = (HOST_SCB_BASE + 64 * tag as u32) as usize;
        for b in 0..64 {
            mem.ram[base + b] = 0;
        }
        // SCB bytes (sequencer window view, struct hardware_scb):
        for (i, b) in cdb.iter().enumerate() {
            mem.ram[base + i] = *b;
        }
        // Scsi status byte is not pre-written: shared_data is a union, so
        // host[0x08] holds cdb[8] (for cdbs > 8 bytes) until the sequencer
        // latches the real status there during the STATUS phase.
        mem.ram[base + 0x0c..(base + 0x10)].copy_from_slice(&le32(dataptr)); // SCB_DATAPTR
        if datacnt > 0 {
            // Single-SG transfer where the one segment is also the last. The
            // driver marks it with AHC_DMA_LAST_SEG in the high byte of the
            // segment length and sets sgptr = sg_list_phys | SG_FULL_RESID
            // (aic7xxx_osm.c:1545-1548): SG_LIST_NULL stays CLEAR so the data
            // phase isn't treated as an overrun.
            mem.ram[base + 0x10..(base + 0x14)]
                .copy_from_slice(&le32(datacnt | 0x8000_0000)); // SCB_DATACNT (SG_LAST_SEG)
            mem.ram[base + 0x14..(base + 0x18)]
                .copy_from_slice(&le32(0x02)); // SCB_SGPTR = SG_FULL_RESID (nothing more after)
        } else {
            // No data phase: sgptr = SG_LIST_NULL (aic7xxx_osm.c:1550).
            mem.ram[base + 0x10..(base + 0x14)].copy_from_slice(&le32(0)); // SCB_DATACNT = 0
            mem.ram[base + 0x14..(base + 0x18)].copy_from_slice(&le32(0x01)); // SCB_SGPTR = SG_LIST_NULL
        }
        mem.ram[base + 0x18] = 0x00; // SCB_CONTROL = 0 (no tag, no disc)
        mem.ram[base + 0x19] = 0x10; // SCB_SCSIID = target 1 in TID (high)
        mem.ram[base + 0x1a] = 0x00; // SCB_LUN = 0
        mem.ram[base + 0x1b] = tag; // SCB_TAG
        mem.ram[base + 0x1c] = cdb.len() as u8; // SCB_CDB_LEN
        mem.ram[base + 0x1d] = 0x00; // SCB_SCSIRATE (async for now)
        mem.ram[base + 0x1e] = 0x00; // SCB_SCSIOFFSET
        mem.ram[base + 0x1f] = 0xff; // SCB_NEXT = SCB_LIST_NULL
    }

    #[test]
    fn e2e_test_unit_ready() {
        let mut c = Aic7880::new();
        // Attach a disk at SCSI target 1.
        use crate::storage::tests::MemDisk;
        use std::sync::{Arc, Mutex};
        let dev = MemDisk {
            name: "t.img".into(),
            data: Arc::new(Mutex::new(vec![0x77u8; 512 * 32])),
            sector_size: 512,
        };
        c.bus.mount(1, Box::new(dev)).unwrap();

        // Download firmware.
        for (i, w) in SEQPROG_7880.iter().enumerate() {
            put_seq(&mut c, i as u16, *w);
        }
        // Match ahc_chip_init: while paused, prime the scratch registers the
        // driver sets before it runs the sequencer (aic7xxx_core.c:5140+).
        c.write8(SCSICONF, 0x00); // default config
        c.write8(0x3a, 0x08); // MSG_OUT = MSG_NOOP
        c.write8(0x3c, 0x00); // SEQ_FLAGS = 0
        c.write8(0x3f, 0x00); // LASTPHASE = 0
        c.write8(0x40, 0xff); // WAITING_SCBH = SCB_LIST_NULL
        c.write8(0x4c, 0x00); // KERNEL_QINPOS
        c.write8(0x4d, 0x00); // QINPOS
        c.write8(0x4e, 0x00); // QOUTPOS
        c.write8(0x51, 0x00); // ARG_1
        c.write8(0x52, 0x00); // ARG_2
        c.write8(0x54, 0x5a); // SCSISEQ_TEMPLATE = ENSELO|ENAUTOATNO|ENRSELI|ENAUTOATNP
        c.write8(0x39, 0xff); // NEXT_QUEUED_SCB = no pending
        boot(&mut c);

        // Program memory map registers:
        let mut mem = VecMem { ram: vec![0u8; 0x10_0000] };
        // shared_data ring (qinfifo/qoutfifo) zeroed
        c.write8(0x44, (HOST_SCB_BASE & 0xff) as u8);
        c.write8(0x45, ((HOST_SCB_BASE >> 8) & 0xff) as u8);
        c.write8(0x46, ((HOST_SCB_BASE >> 16) & 0xff) as u8);
        c.write8(0x47, ((HOST_SCB_BASE >> 24) & 0xff) as u8);
        c.write8(0x48, (SHARED_DATA & 0xff) as u8);
        c.write8(0x49, ((SHARED_DATA >> 8) & 0xff) as u8);
        c.write8(0x4a, ((SHARED_DATA >> 16) & 0xff) as u8);
        c.write8(0x4b, ((SHARED_DATA >> 24) & 0xff) as u8);

        /// One submission = bump kernel queue pointer so the seq picks SCB up.
        fn submit(c: &mut Aic7880, _mem: &mut VecMem, tag: u8) {
            c.write8(0x39, tag); // NEXT_QUEUED_SCB = tag
            let q = c.read8(0x4c);
            c.write8(0x4c, q.wrapping_add(1)); // KERNEL_QINPOS++
        }

        /// Run the sequencer until CMDCMPLT is latched and return QOUTPOS.
        fn run_to_completion(c: &mut Aic7880, mem: &mut VecMem, what: &str) -> u8 {
            // Acknowledge any stale CTRL interrupt state before polling.
            c.write8(CLRINT, INTSTAT_CMDCMPLT | INTSTAT_SCSIINT | INTSTAT_SEQINT);
            for _ in 0..2_000_000 {
                assert!(!c.seq_step(mem), "sequencer halted: {}", what);
                if c.read8(INTSTAT) & INTSTAT_CMDCMPLT != 0 {
                    return c.read8(0x4e); // QOUTPOS
                }
            }
            panic!("timeout waiting for {}", what);
        }

        // ---- TEST_UNIT_READY (tag 1): first probe hits UNIT ATTENTION ----
        c.trace_ctr = 3000;
        write_scb(&mut mem, 1, &[0x00, 0, 0, 0, 0, 0], DATA_BUF, 0);
        submit(&mut c, &mut mem, 1);
        run_to_completion(&mut c, &mut mem, "tur-1");
        c.trace_ctr = 0;
        eprintln!(
            "after tur-1: qin={} kern={} nxt={} wait={} seldo_x={} sstat1={:02x} sstat0={:02x} NEXTQ=..",
            c.read8(0x4d), c.read8(0x4c), c.read8(0x39), c.read8(0x40),
            c.read8(SCSIID), c.read8(SSTAT1), c.read8(SSTAT0)
        );

        // ---- TEST_UNIT_READY (tag 2): now that UA was consumed, GOOD ----
        c.trace_ctr = 4000;
        write_scb(&mut mem, 2, &[0x00, 0, 0, 0, 0, 0], DATA_BUF, 0);
        submit(&mut c, &mut mem, 2);
        run_to_completion(&mut c, &mut mem, "tur-2");
        c.trace_ctr = 0;

        // ---- INQUIRY (tag 3): 36 bytes data-in ----
        let cdb_inq: [u8; 6] = [0x12, 0, 0, 0, 36, 0];
        write_scb(&mut mem, 3, &cdb_inq, DATA_BUF, 36);
        c.trace_ctr = 4000;
        submit(&mut c, &mut mem, 3);
        run_to_completion(&mut c, &mut mem, "inquiry");
        c.trace_ctr = 0;

        let inq = &mem.ram[DATA_BUF as usize..DATA_BUF as usize + 36];
        eprintln!("INQ buf: {:02x?}", &inq[..36]);
        assert_eq!(inq[0], 0x00, "inq byte0");
        assert_eq!(&inq[8..16], b"SGI     ");
        assert_eq!(&inq[16..24], b"O2RUST-D");

        // ---- READ10 (tag 4) of LBA 0..2 (the test disk is 0x77-filled) ----
        let cdb_rd: [u8; 10] = [0x28, 0, 0, 0, 0, 0, 0, 0, 2, 0];
        write_scb(&mut mem, 4, &cdb_rd, DATA_BUF, 1024);
        c.trace_ctr = 4000;
        submit(&mut c, &mut mem, 4);
        run_to_completion(&mut c, &mut mem, "read10");
        c.trace_ctr = 0;
        assert!(mem.ram[DATA_BUF as usize..DATA_BUF as usize + 1024]
            .iter().all(|&b| b == 0x77), "READ10 payload");

        eprintln!("e2e_test_unit_ready: all stages OK");
    }

    /// End-to-end integration test: exercise a SCSI command through the full
    /// Emulator stack — AIC registers via PCI-window, firmware downloaded into
    /// SEQRAM, SCB written into guest RAM, sequencer clocked by
    /// `Emulator::run` → `tick_devices` → `advance_cycles`, and finally the
    /// CMDCMPLT interrupt chain: AHC → MACE → CRIME → CPU IP0.
    #[test]
    fn e2e_emulator_scu_inquiry_through_memory_map() {
        use crate::memory::AddressSpace as _;
        use crate::storage::tests::MemDisk;
        use crate::system::Emulator;
        use std::sync::{Arc, Mutex};

        let mut emu = Emulator::with_ram_no_console(16);

        // Attach a hard disk at SCSI0 target 1.
        let dev = MemDisk {
            name: "t.img".into(),
            data: Arc::new(Mutex::new(vec![0x77u8; 512 * 32])),
            sector_size: 512,
        };
        emu.mount_hard_disk(Box::new(dev)).unwrap();

        // ---- program firmware + scratch registers via the AIC registers ----
        let ahc = &mut emu.memory.mace.ahc0;
        for (i, w) in SEQPROG_7880.iter().enumerate() {
            put_seq(ahc, i as u16, *w);
        }
        ahc.write8(SCSICONF, 0x00);
        ahc.write8(0x3a, 0x08); // MSG_OUT = MSG_NOOP
        ahc.write8(0x3c, 0x00); // SEQ_FLAGS
        ahc.write8(0x3f, 0x00); // LASTPHASE
        ahc.write8(0x40, 0xff); // WAITING_SCBH = SCB_LIST_NULL
        ahc.write8(0x4c, 0x00); // KERNEL_QINPOS
        ahc.write8(0x4d, 0x00); // QINPOS
        ahc.write8(0x4e, 0x00); // QOUTPOS
        ahc.write8(0x51, 0x00); // ARG_1
        ahc.write8(0x52, 0x00); // ARG_2
        ahc.write8(0x54, 0x5a); // SCSISEQ_TEMPLATE
        ahc.write8(0x39, 0xff); // NEXT_QUEUED_SCB = NULL

        // Memory-map registers (HNSCB / HSHARED bases).
        ahc.write8(0x44, (HOST_SCB_BASE & 0xff) as u8);
        ahc.write8(0x45, ((HOST_SCB_BASE >> 8) & 0xff) as u8);
        ahc.write8(0x46, ((HOST_SCB_BASE >> 16) & 0xff) as u8);
        ahc.write8(0x47, ((HOST_SCB_BASE >> 24) & 0xff) as u8);
        ahc.write8(0x48, (SHARED_DATA & 0xff) as u8);
        ahc.write8(0x49, ((SHARED_DATA >> 8) & 0xff) as u8);
        ahc.write8(0x4a, ((SHARED_DATA >> 16) & 0xff) as u8);
        ahc.write8(0x4b, ((SHARED_DATA >> 24) & 0xff) as u8);

        // Un-pause the sequencer.
        boot(ahc);

        // ---- write the SCB (INQUIRY, tag 3) into guest RAM ----
        let ram = emu.memory.ram.as_mut_slice();
        let base = (HOST_SCB_BASE + 64 * 3) as usize;
        for b in base..base + 64 {
            ram[b] = 0;
        }
        let cdb: [u8; 6] = [0x12, 0, 0, 0, 36, 0];
        for (i, c) in cdb.iter().enumerate() {
            ram[base + i] = *c;
        }
        // SCB fields (mirrors the write_scb in e2e_test_unit_ready).
        ram[base + 0x0c] = (DATA_BUF >> 0) as u8;
        ram[base + 0x0d] = (DATA_BUF >> 8) as u8;
        ram[base + 0x0e] = (DATA_BUF >> 16) as u8;
        ram[base + 0x0f] = (DATA_BUF >> 24) as u8;
        // DATACNT (SG) with LAST flag.
        let dcnt: u32 = 36 | 0x8000_0000;
        ram[base + 0x10] = (dcnt >> 0) as u8;
        ram[base + 0x11] = (dcnt >> 8) as u8;
        ram[base + 0x12] = (dcnt >> 16) as u8;
        ram[base + 0x13] = (dcnt >> 24) as u8;
        // SGPTR = SG_FULL_RESID (0x02).
        ram[base + 0x14] = 0x00;
        ram[base + 0x15] = 0x00;
        ram[base + 0x16] = 0x00;
        ram[base + 0x17] = 0x02;
        // CONTROL=0, SCSIID=target1, TAG=3, CDB_LEN=6.
        ram[base + 0x19] = 0x10; // target 1 in TID field
        ram[base + 0x1b] = 3; // tag
        ram[base + 0x1c] = cdb.len() as u8;

        // ---- enable interrupts (AHC INTEN + CRIME mask) ----
        ahc.write8(CLRINT, 0xff);
        ahc.write8(HCNTRL, HCNTRL_INTEN);
        emu.memory.crime_cpu.write32(crate::graphics::crime_cpu::CRM_INTMASK, 1 << crate::io::pci::IRQ_SCSI0);

        // ---- submit & run ----
        {
            let ahc = &mut emu.memory.mace.ahc0;
            ahc.write8(0x39, 3); // NEXT_QUEUED_SCB = tag 3
            let q = ahc.read8(0x4c);
            ahc.write8(0x4c, q.wrapping_add(1)); // KERNEL_QINPOS++
        }

        let mut completed = false;
        for _ in 0..120 {
            emu.run(8_000_000);
            let ahc = &mut emu.memory.mace.ahc0;
            if ahc.read8(INTSTAT) & INTSTAT_CMDCMPLT != 0 {
                completed = true;
                break;
            }
        }
        if !completed {
            let ahc = &mut emu.memory.mace.ahc0;
            eprintln!(
                "dbg: INTSTAT={:02x} SSTAT0={:02x} SSTAT1={:02x} SCSISIG={:02x} DFSTATUS={:02x} SCSISEQ={:02x} lastphase={:02x} fifo={}",
                ahc.read8(INTSTAT),
                ahc.read8(SSTAT0),
                ahc.read8(SSTAT1),
                ahc.read8(SCSISIG),
                ahc.read8(DFSTATUS),
                ahc.read8(SCSISEQ),
                ahc.read8(0x3f),
                emu.memory.mace.ahc0.fifo.len(),
            );
        }
        assert!(completed, "CMDCMPLT must assert within budget");
        assert_ne!(
            emu.cpu.cp0.pending_interrupts() & 0x01,
            0,
            "CPU IP0 must be asserted with CRM SCSI0 mask enabled"
        );

        // Verify the INQUIRY response in guest RAM at DATA_BUF.
        let ram = emu.memory.ram.as_slice();
        let inq = &ram[DATA_BUF as usize..DATA_BUF as usize + 36];
        assert_eq!(inq[0], 0x00, "peripheral qualifier + device type");
        assert_eq!(&inq[8..16], b"SGI     ", "vendor");
        assert_eq!(&inq[16..24], b"O2RUST-D", "product");
    }
}
