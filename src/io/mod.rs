// src/io/mod.rs
//! I/O subsystem — the MACE ASIC.
//!
//! MACE (the I/O Engine) provides:
//!
//! - 64-bit PCI bus (single expansion slot)
//! - ISA bus (used only for the Super I/O chip: serial/parallel)
//! - PS/2 keyboard + mouse
//! - 10/100 Ethernet (MAC110)
//! - Audio (AD1843 codec)
//! - SCSI (Adaptec AIC-7880)
//!
//! Register maps sourced from Linux `arch/mips/sgi-ip32/`, NetBSD `sys/arch/sgimips/`,
//! and leaked IRIX source `stand/arcs/`.

pub mod scsi;

pub use scsi::{SCSI_TARGET_CDROM, SCSI_TARGET_DISK, ScsiBus};

use std::collections::VecDeque;

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
    /// Ethernet (MAC110).
    pub const ENET: u32 = 0x280000;
    /// Peripheral (audio, ISA, keyboard/mouse, I2C, UST/MSC).
    pub const PERIF: u32 = 0x300000;
    /// ISA External (EPP, ECP, serial, RTC, game port).
    pub const ISA_EXT: u32 = 0x380000;
}

/// PCI Host Bridge registers (offset 0x080000 from MACE_BASE).
/// Sourced from Linux `arch/mips/include/asm/ip32/mace.h` and IRIX `mace.h`.
pub mod pci {
    pub const BASE: u32 = 0x080000;

    // PCI Configuration Space
    pub const CFG_ADDR: u32 = 0x0000;  // Configuration address
    pub const CFG_DATA: u32 = 0x0004;  // Configuration data

    // PCI Memory Space
    pub const MEM_BASE: u32 = 0x0010;  // Memory base address
    pub const MEM_LIMIT: u32 = 0x0014; // Memory limit address

    // PCI I/O Space
    pub const IO_BASE: u32 = 0x0018;   // I/O base address
    pub const IO_LIMIT: u32 = 0x001C;  // I/O limit address

    // PCI Control/Status
    pub const CTRL: u32 = 0x0020;      // PCI control register
    pub const STATUS: u32 = 0x0024;    // PCI status register

    // PCI Interrupt
    pub const INT_ACK: u32 = 0x0028;   // Interrupt acknowledge
    pub const INT_MASK: u32 = 0x002C;  // Interrupt mask

    // PCI Arbiter
    pub const ARB_CTRL: u32 = 0x0030;  // Arbiter control
    pub const ARB_PRI: u32 = 0x0034;   // Arbiter priority

    // PCI Error
    pub const ERR_ADDR: u32 = 0x0038;  // Error address
    pub const ERR_CMD: u32 = 0x003C;   // Error command
}

/// Ethernet MAC110 registers (offset 0x280000 from MACE_BASE).
/// Sourced from Linux `drivers/net/ethernet/sgi/mace.c` and IRIX `if_mace.c`.
pub mod enet {
    pub const BASE: u32 = 0x280000;

    // MAC110 Control/Status
    pub const CTRL: u32 = 0x0000;      // Control register
    pub const STATUS: u32 = 0x0004;    // Status register
    pub const INT_MASK: u32 = 0x0008;  // Interrupt mask
    pub const INT_STATUS: u32 = 0x000C; // Interrupt status

    // MAC Address
    pub const MAC_ADDR0: u32 = 0x0010; // MAC address bytes 0-3
    pub const MAC_ADDR1: u32 = 0x0014; // MAC address bytes 4-5

    // Transmit
    pub const TX_CTRL: u32 = 0x0020;   // Transmit control
    pub const TX_STATUS: u32 = 0x0024; // Transmit status
    pub const TX_DESC: u32 = 0x0028;   // Transmit descriptor
    pub const TX_BUF: u32 = 0x002C;    // Transmit buffer

    // Receive
    pub const RX_CTRL: u32 = 0x0030;   // Receive control
    pub const RX_STATUS: u32 = 0x0034; // Receive status
    pub const RX_DESC: u32 = 0x0038;   // Receive descriptor
    pub const RX_BUF: u32 = 0x003C;    // Receive buffer

    // MII Management
    pub const MII_CTRL: u32 = 0x0040;  // MII control
    pub const MII_DATA: u32 = 0x0044;  // MII data
    pub const MII_ADDR: u32 = 0x0048;  // MII address

    // Statistics
    pub const STATS_BASE: u32 = 0x0100; // Statistics counters base
}

/// Peripheral block registers (offset 0x300000 from MACE_BASE).
/// Contains: Audio (AD1843), ISA bridge (PC87312), Keyboard/Mouse (PS/2),
/// I2C, UST/MSC.
/// Sourced from Linux `arch/mips/sgi-ip32/mace.h` and IRIX `mace.h`.
pub mod perif {
    pub const BASE: u32 = 0x300000;

    // Audio (AD1843) - offset 0x00000
    pub mod audio {
        pub const BASE: u32 = 0x00000;
        pub const CTRL: u32 = 0x0000;   // Control
        pub const STATUS: u32 = 0x0004; // Status
        pub const DATA: u32 = 0x0008;   // Data port
        pub const INDIRECT: u32 = 0x000C; // Indirect register access

        /// Frames in the output ring between the emulated AD1843 and the
        /// front-end audio thread (~46 ms at 44.1 kHz). Power of two.
        pub const RING_CAPACITY: usize = 4096;
    }

    // ISA Bridge (PC87312) - offset 0x10000
    pub mod isa {
        pub const BASE: u32 = 0x10000;
        pub const CTRL: u32 = 0x0000;   // ISA control
        pub const STATUS: u32 = 0x0004; // ISA status
        pub const INT_CTRL: u32 = 0x0008; // Interrupt control
        pub const DMA_CTRL: u32 = 0x000C; // DMA control
    }

    // Keyboard/Mouse (PS/2) - offset 0x20000
    pub mod kbdms {
        pub const BASE: u32 = 0x20000;
        pub const KBD_DATA: u32 = 0x0000; // Keyboard data
        pub const KBD_CTRL: u32 = 0x0004; // Keyboard control
        pub const MS_DATA: u32 = 0x0008;  // Mouse data
        pub const MS_CTRL: u32 = 0x000C;  // Mouse control
        pub const STATUS: u32 = 0x0010;   // Combined status
    }

    // I2C - offset 0x30000
    pub mod i2c {
        pub const BASE: u32 = 0x30000;
        pub const CTRL: u32 = 0x0000;   // I2C control
        pub const STATUS: u32 = 0x0004; // I2C status
        pub const DATA: u32 = 0x0008;   // I2C data
        pub const ADDR: u32 = 0x000C;   // I2C address
    }

    // UST/MSC (Unadjusted System Time / Media Stream Counter) - offset 0x40000
    pub mod ustmsc {
        pub const BASE: u32 = 0x40000;
        pub const UST_LO: u32 = 0x0000; // UST low 32 bits
        pub const UST_HI: u32 = 0x0004; // UST high 32 bits
        pub const MSC_LO: u32 = 0x0008; // MSC low 32 bits
        pub const MSC_HI: u32 = 0x000C; // MSC high 32 bits
        pub const CTRL: u32 = 0x0010;   // Control
    }
}

/// ISA External block registers (offset 0x380000 from MACE_BASE).
/// Contains: EPP, ECP, Serial 1 (NS16550), Serial 2 (NS16550),
/// RTC (DS12887), Game port.
/// Sourced from Linux `arch/mips/sgi-ip32/mace.h` and IRIX `mace.h`.
pub mod isa_ext {
    pub const BASE: u32 = 0x380000;

    // EPP (Enhanced Parallel Port) - offset 0x00000
    pub mod epp {
        pub const BASE: u32 = 0x00000;
        pub const DATA: u32 = 0x0000;   // Data port
        pub const ADDR: u32 = 0x0004;   // Address port
        pub const CTRL: u32 = 0x0008;   // Control
        pub const STATUS: u32 = 0x000C; // Status
    }

    // ECP (Extended Capabilities Port) - offset 0x08000
    pub mod ecp {
        pub const BASE: u32 = 0x08000;
        pub const DATA: u32 = 0x0000;   // Data FIFO
        pub const ADDR: u32 = 0x0004;   // Address
        pub const CTRL: u32 = 0x0008;   // Control
        pub const STATUS: u32 = 0x000C; // Status
    }

    // Serial 1 (NS16550 UART) - offset 0x10000
    pub mod uart1 {
        pub const BASE: u32 = 0x10000;
        // NS16550 registers (8-bit, but accessed as 32-bit on O2)
        pub const RBR: u32 = 0x0000;    // Receive Buffer Register (read)
        pub const THR: u32 = 0x0000;    // Transmit Holding Register (write)
        pub const IER: u32 = 0x0004;    // Interrupt Enable Register
        pub const IIR: u32 = 0x0008;    // Interrupt Identification Register (read)
        pub const FCR: u32 = 0x0008;    // FIFO Control Register (write)
        pub const LCR: u32 = 0x000C;    // Line Control Register
        pub const MCR: u32 = 0x0010;    // Modem Control Register
        pub const LSR: u32 = 0x0014;    // Line Status Register
        pub const MSR: u32 = 0x0018;    // Modem Status Register
        pub const SCR: u32 = 0x001C;    // Scratch Register
        // Divisor latch (when DLAB=1 in LCR)
        pub const DLL: u32 = 0x0000;    // Divisor Latch Low
        pub const DLM: u32 = 0x0004;    // Divisor Latch High
    }

    // Serial 2 (NS16550 UART) - offset 0x18000
    pub mod uart2 {
        pub const BASE: u32 = 0x18000;
        pub const RBR: u32 = 0x0000;
        pub const THR: u32 = 0x0000;
        pub const IER: u32 = 0x0004;
        pub const IIR: u32 = 0x0008;
        pub const FCR: u32 = 0x0008;
        pub const LCR: u32 = 0x000C;
        pub const MCR: u32 = 0x0010;
        pub const LSR: u32 = 0x0014;
        pub const MSR: u32 = 0x0018;
        pub const SCR: u32 = 0x001C;
        pub const DLL: u32 = 0x0000;
        pub const DLM: u32 = 0x0004;
    }

    // RTC (DS12887) - offset 0x20000
    pub mod rtc {
        pub const BASE: u32 = 0x20000;
        pub const SECONDS: u32 = 0x0000;
        pub const SECONDS_ALARM: u32 = 0x0001;
        pub const MINUTES: u32 = 0x0002;
        pub const MINUTES_ALARM: u32 = 0x0003;
        pub const HOURS: u32 = 0x0004;
        pub const HOURS_ALARM: u32 = 0x0005;
        pub const DAY_OF_WEEK: u32 = 0x0006;
        pub const DAY_OF_MONTH: u32 = 0x0007;
        pub const MONTH: u32 = 0x0008;
        pub const YEAR: u32 = 0x0009;
        pub const REG_A: u32 = 0x000A;
        pub const REG_B: u32 = 0x000B;
        pub const REG_C: u32 = 0x000C;
        pub const REG_D: u32 = 0x000D;
        // 114 bytes of NVRAM at 0x000E-0x007F
    }

    // Game port - offset 0x30000
    pub mod game {
        pub const BASE: u32 = 0x30000;
        pub const DATA: u32 = 0x0000;   // Game port data
        pub const CTRL: u32 = 0x0004;   // Game port control
    }
}

/// Video In 1 registers (offset 0x100000 from MACE_BASE).
pub mod vin1 {
    pub const BASE: u32 = 0x100000;
    pub const CTRL: u32 = 0x0000;
    pub const STATUS: u32 = 0x0004;
    pub const BUF_ADDR: u32 = 0x0008;
    pub const BUF_SIZE: u32 = 0x000C;
}

/// Video In 2 registers (offset 0x180000 from MACE_BASE).
pub mod vin2 {
    pub const BASE: u32 = 0x180000;
    pub const CTRL: u32 = 0x0000;
    pub const STATUS: u32 = 0x0004;
    pub const BUF_ADDR: u32 = 0x0008;
    pub const BUF_SIZE: u32 = 0x000C;
}

/// Video Out registers (offset 0x200000 from MACE_BASE).
pub mod vout {
    pub const BASE: u32 = 0x200000;
    pub const CTRL: u32 = 0x0000;
    pub const STATUS: u32 = 0x0004;
    pub const BUF_ADDR: u32 = 0x0008;
    pub const BUF_SIZE: u32 = 0x000C;
}

/// MACE ASIC state.
#[derive(Debug)]
pub struct Mace {
    /// PCI Host Bridge state
    pub pci: PciState,
    /// Ethernet (MAC110) state
    pub enet: EnetState,
    /// Peripheral block state
    pub perif: PerifState,
    /// ISA External block state
    pub isa_ext: IsaExtState,
    /// Video In 1 state
    pub vin1: VinState,
    /// Video In 2 state
    pub vin2: VinState,
    /// Video Out state
    pub vout: VinState,
    /// Host side of the audio output ring (frames produced by the emulated
    /// codec). The CLI/GUI front-end pulls it via [`Mace::take_audio_consumer`].
    audio_out: Option<rtrb::Consumer<f32>>,
}

impl Mace {
    /// Create a new MACE ASIC with console I/O channels for UARTs.
    /// UART1 is used for console input/output (bidirectional).
    /// UART2 is used for console output only (tx only).
    pub fn with_console(
        uart1_tx: std::sync::mpsc::Sender<u8>,
        uart1_rx: std::sync::mpsc::Receiver<u8>,
        uart2_tx: std::sync::mpsc::Sender<u8>,
    ) -> Self {
        let mut perif = PerifState::default();
        let (audio_out, audio_from_guest) = rtrb::RingBuffer::new(perif::audio::RING_CAPACITY);
        perif.audio.out = Some(audio_out);
        Self {
            pci: PciState::default(),
            enet: EnetState::default(),
            perif,
            isa_ext: IsaExtState::with_console(uart1_tx, uart1_rx, uart2_tx),
            vin1: VinState::default(),
            vin2: VinState::default(),
            vout: VinState::default(),
            audio_out: Some(audio_from_guest),
        }
    }

    /// Create a new MACE ASIC (without console I/O).
    pub fn new() -> Self {
        let (uart1_tx, uart1_rx) = std::sync::mpsc::channel();
        let (uart2_tx, _uart2_rx) = std::sync::mpsc::channel();
        Self::with_console(uart1_tx, uart1_rx, uart2_tx)
    }

    /// Take the host side of the audio output ring.
    ///
    /// Exactly one consumer is produced; taking it moves ownership to the
    /// front-end (CLI/GUI). Returns `None` if already taken.
    pub fn take_audio_consumer(&mut self) -> Option<rtrb::Consumer<f32>> {
        self.audio_out.take()
    }

    /// Reset the MACE ASIC.
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Read a 32-bit register from MACE.
    pub fn read32(&mut self, offset: u32) -> u32 {
        match offset {
            // PCI
            offset if offset >= pci::BASE && offset <= pci::BASE + 0xFF => self.pci.read32(offset - pci::BASE),
            // Ethernet
            offset if offset >= enet::BASE && offset <= enet::BASE + 0x1FF => self.enet.read32(offset - enet::BASE),
            // Peripheral
            offset if offset >= perif::BASE && offset <= perif::BASE + 0x4FFFF => self.perif.read32(offset - perif::BASE),
            // ISA External
            offset if offset >= isa_ext::BASE && offset <= isa_ext::BASE + 0x3FFFF => self.isa_ext.read32(offset - isa_ext::BASE),
            // Video In 1
            offset if offset >= vin1::BASE && offset <= vin1::BASE + 0xFF => self.vin1.read32(offset - vin1::BASE),
            // Video In 2
            offset if offset >= vin2::BASE && offset <= vin2::BASE + 0xFF => self.vin2.read32(offset - vin2::BASE),
            // Video Out
            offset if offset >= vout::BASE && offset <= vout::BASE + 0xFF => self.vout.read32(offset - vout::BASE),
            _ => {
                log::warn!("MACE read32: unimplemented offset 0x{:08X}", offset);
                0
            }
        }
    }

    /// Read a 32-bit register from MACE (immutable version for read-only access).
    pub fn read32_immutable(&self, offset: u32) -> u32 {
        match offset {
            // PCI
            offset if offset >= pci::BASE && offset <= pci::BASE + 0xFF => self.pci.read32(offset - pci::BASE),
            // Ethernet
            offset if offset >= enet::BASE && offset <= enet::BASE + 0x1FF => self.enet.read32(offset - enet::BASE),
            // Peripheral
            offset if offset >= perif::BASE && offset <= perif::BASE + 0x4FFFF => self.perif.read32_immutable(offset - perif::BASE),
            // ISA External
            offset if offset >= isa_ext::BASE && offset <= isa_ext::BASE + 0x3FFFF => self.isa_ext.read32_immutable(offset - isa_ext::BASE),
            // Video In 1
            offset if offset >= vin1::BASE && offset <= vin1::BASE + 0xFF => self.vin1.read32(offset - vin1::BASE),
            // Video In 2
            offset if offset >= vin2::BASE && offset <= vin2::BASE + 0xFF => self.vin2.read32(offset - vin2::BASE),
            // Video Out
            offset if offset >= vout::BASE && offset <= vout::BASE + 0xFF => self.vout.read32(offset - vout::BASE),
            _ => {
                log::warn!("MACE read32_immutable: unimplemented offset 0x{:08X}", offset);
                0
            }
        }
    }

    /// Write a 32-bit register to MACE.
    pub fn write32(&mut self, offset: u32, value: u32) {
        match offset {
            // PCI
            offset if offset >= pci::BASE && offset <= pci::BASE + 0xFF => self.pci.write32(offset - pci::BASE, value),
            // Ethernet
            offset if offset >= enet::BASE && offset <= enet::BASE + 0x1FF => self.enet.write32(offset - enet::BASE, value),
            // Peripheral
            offset if offset >= perif::BASE && offset <= perif::BASE + 0x4FFFF => self.perif.write32(offset - perif::BASE, value),
            // ISA External
            offset if offset >= isa_ext::BASE && offset <= isa_ext::BASE + 0x3FFFF => self.isa_ext.write32(offset - isa_ext::BASE, value),
            // Video In 1
            offset if offset >= vin1::BASE && offset <= vin1::BASE + 0xFF => self.vin1.write32(offset - vin1::BASE, value),
            // Video In 2
            offset if offset >= vin2::BASE && offset <= vin2::BASE + 0xFF => self.vin2.write32(offset - vin2::BASE, value),
            // Video Out
            offset if offset >= vout::BASE && offset <= vout::BASE + 0xFF => self.vout.write32(offset - vout::BASE, value),
            _ => {
                log::warn!("MACE write32: unimplemented offset 0x{:08X} = 0x{:08X}", offset, value);
            }
        }
    }
}

impl crate::memory::AddressSpace for Mace {
    fn read8(&mut self, addr: u32) -> u8 {
        // MACE registers are 32-bit aligned; read the 32-bit word and extract the byte
        let word = self.read32(addr);
        ((word >> ((addr & 3) * 8)) & 0xFF) as u8
    }

    fn read16(&mut self, addr: u32) -> u16 {
        let word = self.read32(addr);
        ((word >> ((addr & 3) * 8)) & 0xFFFF) as u16
    }

    fn read32(&mut self, addr: u32) -> u32 {
        self.read32(addr)
    }

    fn read64(&mut self, addr: u32) -> u64 {
        // MACE doesn't have 64-bit registers; combine two 32-bit reads
        let low = self.read32(addr) as u64;
        let high = self.read32(addr.wrapping_add(4)) as u64;
        (high << 32) | low
    }

    fn write8(&mut self, addr: u32, value: u8) {
        // Read-modify-write for byte writes
        let word = self.read32(addr);
        let shift = (addr & 3) * 8;
        let new_word = (word & !(0xFF << shift)) | ((value as u32) << shift);
        self.write32(addr, new_word);
    }

    fn write16(&mut self, addr: u32, value: u16) {
        let word = self.read32(addr);
        let shift = (addr & 3) * 8;
        let new_word = (word & !(0xFFFF << shift)) | ((value as u32) << shift);
        self.write32(addr, new_word);
    }

    fn write32(&mut self, addr: u32, value: u32) {
        self.write32(addr, value);
    }

    fn write64(&mut self, addr: u32, value: u64) {
        self.write32(addr, value as u32);
        self.write32(addr.wrapping_add(4), (value >> 32) as u32);
    }

    fn contains(&self, addr: u32) -> bool {
        addr < 0x400000 // MACE address space is 4MB (0x1F000000 - 0x1F3FFFFF)
    }
}

impl Default for Mace {
    fn default() -> Self {
        Self::new()
    }
}

/// PCI Host Bridge state.
#[derive(Debug, Default)]
pub struct PciState {
    pub cfg_addr: u32,
    pub cfg_data: u32,
    pub mem_base: u32,
    pub mem_limit: u32,
    pub io_base: u32,
    pub io_limit: u32,
    pub ctrl: u32,
    pub status: u32,
    pub int_ack: u32,
    pub int_mask: u32,
    pub arb_ctrl: u32,
    pub arb_pri: u32,
    pub err_addr: u32,
    pub err_cmd: u32,
}

impl PciState {
    pub fn read32(&self, offset: u32) -> u32 {
        match offset {
            pci::CFG_ADDR => self.cfg_addr,
            pci::CFG_DATA => self.cfg_data,
            pci::MEM_BASE => self.mem_base,
            pci::MEM_LIMIT => self.mem_limit,
            pci::IO_BASE => self.io_base,
            pci::IO_LIMIT => self.io_limit,
            pci::CTRL => self.ctrl,
            pci::STATUS => self.status,
            pci::INT_ACK => self.int_ack,
            pci::INT_MASK => self.int_mask,
            pci::ARB_CTRL => self.arb_ctrl,
            pci::ARB_PRI => self.arb_pri,
            pci::ERR_ADDR => self.err_addr,
            pci::ERR_CMD => self.err_cmd,
            _ => {
                log::warn!("PCI read32: unimplemented offset 0x{:04X}", offset);
                0
            }
        }
    }

    pub fn write32(&mut self, offset: u32, value: u32) {
        match offset {
            pci::CFG_ADDR => self.cfg_addr = value,
            pci::CFG_DATA => self.cfg_data = value,
            pci::MEM_BASE => self.mem_base = value,
            pci::MEM_LIMIT => self.mem_limit = value,
            pci::IO_BASE => self.io_base = value,
            pci::IO_LIMIT => self.io_limit = value,
            pci::CTRL => self.ctrl = value,
            pci::STATUS => self.status = value,
            pci::INT_ACK => self.int_ack = value,
            pci::INT_MASK => self.int_mask = value,
            pci::ARB_CTRL => self.arb_ctrl = value,
            pci::ARB_PRI => self.arb_pri = value,
            pci::ERR_ADDR => self.err_addr = value,
            pci::ERR_CMD => self.err_cmd = value,
            _ => {
                log::warn!("PCI write32: unimplemented offset 0x{:04X} = 0x{:08X}", offset, value);
            }
        }
    }
}

/// Ethernet (MAC110) state.
#[derive(Debug, Default)]
pub struct EnetState {
    pub ctrl: u32,
    pub status: u32,
    pub int_mask: u32,
    pub int_status: u32,
    pub mac_addr0: u32,
    pub mac_addr1: u32,
    pub tx_ctrl: u32,
    pub tx_status: u32,
    pub tx_desc: u32,
    pub tx_buf: u32,
    pub rx_ctrl: u32,
    pub rx_status: u32,
    pub rx_desc: u32,
    pub rx_buf: u32,
    pub mii_ctrl: u32,
    pub mii_data: u32,
    pub mii_addr: u32,
    // Statistics counters (simplified)
    pub stats: [u32; 32],
}

impl EnetState {
    pub fn read32(&self, offset: u32) -> u32 {
        match offset {
            enet::CTRL => self.ctrl,
            enet::STATUS => self.status,
            enet::INT_MASK => self.int_mask,
            enet::INT_STATUS => self.int_status,
            enet::MAC_ADDR0 => self.mac_addr0,
            enet::MAC_ADDR1 => self.mac_addr1,
            enet::TX_CTRL => self.tx_ctrl,
            enet::TX_STATUS => self.tx_status,
            enet::TX_DESC => self.tx_desc,
            enet::TX_BUF => self.tx_buf,
            enet::RX_CTRL => self.rx_ctrl,
            enet::RX_STATUS => self.rx_status,
            enet::RX_DESC => self.rx_desc,
            enet::RX_BUF => self.rx_buf,
            enet::MII_CTRL => self.mii_ctrl,
            enet::MII_DATA => self.mii_data,
            enet::MII_ADDR => self.mii_addr,
            offset if offset >= enet::STATS_BASE && offset <= enet::STATS_BASE + 0x7C => {
                let idx = ((offset - enet::STATS_BASE) / 4) as usize;
                if idx < self.stats.len() { self.stats[idx] } else { 0 }
            }
            _ => {
                log::warn!("ENET read32: unimplemented offset 0x{:04X}", offset);
                0
            }
        }
    }

    pub fn write32(&mut self, offset: u32, value: u32) {
        match offset {
            enet::CTRL => self.ctrl = value,
            enet::STATUS => self.status = value,
            enet::INT_MASK => self.int_mask = value,
            enet::INT_STATUS => self.int_status = value,
            enet::MAC_ADDR0 => self.mac_addr0 = value,
            enet::MAC_ADDR1 => self.mac_addr1 = value,
            enet::TX_CTRL => self.tx_ctrl = value,
            enet::TX_STATUS => self.tx_status = value,
            enet::TX_DESC => self.tx_desc = value,
            enet::TX_BUF => self.tx_buf = value,
            enet::RX_CTRL => self.rx_ctrl = value,
            enet::RX_STATUS => self.rx_status = value,
            enet::RX_DESC => self.rx_desc = value,
            enet::RX_BUF => self.rx_buf = value,
            enet::MII_CTRL => self.mii_ctrl = value,
            enet::MII_DATA => self.mii_data = value,
            enet::MII_ADDR => self.mii_addr = value,
            offset if offset >= enet::STATS_BASE && offset <= enet::STATS_BASE + 0x7C => {
                let idx = ((offset - enet::STATS_BASE) / 4) as usize;
                if idx < self.stats.len() { self.stats[idx] = value; }
            }
            _ => {
                log::warn!("ENET write32: unimplemented offset 0x{:04X} = 0x{:08X}", offset, value);
            }
        }
    }
}

/// Peripheral block state.
#[derive(Debug, Default)]
pub struct PerifState {
    pub audio: AudioState,
    pub isa: IsaState,
    pub kbdms: KbdMsState,
    pub i2c: I2cState,
    pub ustmsc: UstMscState,
}

impl PerifState {
    pub fn read32(&mut self, offset: u32) -> u32 {
        match offset {
            offset if offset >= perif::audio::BASE && offset <= perif::audio::BASE + 0xFF => self.audio.read32(offset - perif::audio::BASE),
            offset if offset >= perif::isa::BASE && offset <= perif::isa::BASE + 0xFF => self.isa.read32(offset - perif::isa::BASE),
            offset if offset >= perif::kbdms::BASE && offset <= perif::kbdms::BASE + 0xFF => self.kbdms.read32(offset - perif::kbdms::BASE),
            offset if offset >= perif::i2c::BASE && offset <= perif::i2c::BASE + 0xFF => self.i2c.read32(offset - perif::i2c::BASE),
            offset if offset >= perif::ustmsc::BASE && offset <= perif::ustmsc::BASE + 0xFF => self.ustmsc.read32(offset - perif::ustmsc::BASE),
            _ => {
                log::warn!("PERIF read32: unimplemented offset 0x{:05X}", offset);
                0
            }
        }
    }

    /// Read without mutating state (used by the read-only address guard).
    pub fn read32_immutable(&self, offset: u32) -> u32 {
        match offset {
            offset if offset >= perif::audio::BASE && offset <= perif::audio::BASE + 0xFF => self.audio.read32(offset - perif::audio::BASE),
            offset if offset >= perif::isa::BASE && offset <= perif::isa::BASE + 0xFF => self.isa.read32(offset - perif::isa::BASE),
            offset if offset >= perif::kbdms::BASE && offset <= perif::kbdms::BASE + 0xFF => self.kbdms.read32_immutable(offset - perif::kbdms::BASE),
            offset if offset >= perif::i2c::BASE && offset <= perif::i2c::BASE + 0xFF => self.i2c.read32(offset - perif::i2c::BASE),
            offset if offset >= perif::ustmsc::BASE && offset <= perif::ustmsc::BASE + 0xFF => self.ustmsc.read32(offset - perif::ustmsc::BASE),
            _ => {
                log::warn!("PERIF read32_immutable: unimplemented offset 0x{:05X}", offset);
                0
            }
        }
    }

    pub fn write32(&mut self, offset: u32, value: u32) {
        match offset {
            offset if offset >= perif::audio::BASE && offset <= perif::audio::BASE + 0xFF => self.audio.write32(offset - perif::audio::BASE, value),
            offset if offset >= perif::isa::BASE && offset <= perif::isa::BASE + 0xFF => self.isa.write32(offset - perif::isa::BASE, value),
            offset if offset >= perif::kbdms::BASE && offset <= perif::kbdms::BASE + 0xFF => self.kbdms.write32(offset - perif::kbdms::BASE, value),
            offset if offset >= perif::i2c::BASE && offset <= perif::i2c::BASE + 0xFF => self.i2c.write32(offset - perif::i2c::BASE, value),
            offset if offset >= perif::ustmsc::BASE && offset <= perif::ustmsc::BASE + 0xFF => self.ustmsc.write32(offset - perif::ustmsc::BASE, value),
            _ => {
                log::warn!("PERIF write32: unimplemented offset 0x{:05X} = 0x{:08X}", offset, value);
            }
        }
    }
}

/// Audio (AD1843) state.
///
/// The emulated codec writes digitized output samples into `out` (an rtrb
/// ring); the front-end (CLI/GUI) consumes them through the corresponding
/// [`rtrb::Consumer`] obtained from [`Mace::take_audio_consumer`].
#[derive(Debug)]
pub struct AudioState {
    pub ctrl: u32,
    pub status: u32,
    pub data: u32,
    pub indirect: u32,
    /// Host-visible output sample ring (producer side fed by the codec).
    pub out: Option<rtrb::Producer<f32>>,
}

impl Default for AudioState {
    fn default() -> Self {
        Self {
            ctrl: 0,
            status: 0,
            data: 0,
            indirect: 0,
            out: None,
        }
    }
}

impl AudioState {
    pub fn read32(&self, offset: u32) -> u32 {
        match offset {
            perif::audio::CTRL => self.ctrl,
            perif::audio::STATUS => self.status,
            perif::audio::DATA => self.data,
            perif::audio::INDIRECT => self.indirect,
            _ => {
                log::warn!("AUDIO read32: unimplemented offset 0x{:04X}", offset);
                0
            }
        }
    }

    pub fn write32(&mut self, offset: u32, value: u32) {
        match offset {
            perif::audio::CTRL => self.ctrl = value,
            perif::audio::STATUS => self.status = value,
            perif::audio::DATA => self.data = value,
            perif::audio::INDIRECT => self.indirect = value,
            _ => {
                log::warn!("AUDIO write32: unimplemented offset 0x{:04X} = 0x{:08X}", offset, value);
            }
        }
    }

    /// Feed digitized output samples (mono `f32`) into the front-end ring.
    ///
    /// No-op when no front-end has taken the ring (headless runs).
    pub fn push_output_samples(&mut self, samples: &[f32]) {
        if let Some(producer) = self.out.as_mut() {
            let mut pushed = 0;
            for s in samples {
                if producer.push(*s).is_ok() {
                    pushed += 1;
                } else {
                    break; // ring is full; drop the rest
                }
            }
            if pushed < samples.len() {
                log::debug!(
                    "audio ring full, dropped {} of {} samples",
                    samples.len() - pushed,
                    samples.len()
                );
            }
        }
    }
}

/// ISA Bridge (PC87312) state.
#[derive(Debug, Default)]
pub struct IsaState {
    pub ctrl: u32,
    pub status: u32,
    pub int_ctrl: u32,
    pub dma_ctrl: u32,
}

impl IsaState {
    pub fn read32(&self, offset: u32) -> u32 {
        match offset {
            perif::isa::CTRL => self.ctrl,
            perif::isa::STATUS => self.status,
            perif::isa::INT_CTRL => self.int_ctrl,
            perif::isa::DMA_CTRL => self.dma_ctrl,
            _ => {
                log::warn!("ISA read32: unimplemented offset 0x{:04X}", offset);
                0
            }
        }
    }

    pub fn write32(&mut self, offset: u32, value: u32) {
        match offset {
            perif::isa::CTRL => self.ctrl = value,
            perif::isa::STATUS => self.status = value,
            perif::isa::INT_CTRL => self.int_ctrl = value,
            perif::isa::DMA_CTRL => self.dma_ctrl = value,
            _ => {
                log::warn!("ISA write32: unimplemented offset 0x{:04X} = 0x{:08X}", offset, value);
            }
        }
    }
}

/// Keyboard/Mouse (PS/2) state.
///
/// Host keyboard/mouse input arrives through [`KbdMsState::push_kbd_byte`] and
/// [`KbdMsState::push_ms_byte`] (PS/2 scan set 2 for the keyboard, standard
/// 3-byte relative packets for the mouse); the guest firmware / drivers read
/// them back out of [`PerifState::read32`] one byte at a time.
#[derive(Debug)]
pub struct KbdMsState {
    pub kbd_data: u32,
    pub kbd_ctrl: u32,
    pub ms_data: u32,
    pub ms_ctrl: u32,
    pub status: u32,
    /// PS/2 keyboard bytes queued from the host, LIFO-fed by the 8042.
    pub keyboard_input: VecDeque<u8>,
    /// PS/2 mouse bytes queued from the host.
    pub mouse_input: VecDeque<u8>,
}

/// Maximum number of queued input bytes per device before the host-side FIFO
/// starts dropping (guards the guest polling long stretches of dead code).
const INPUT_FIFO_CAPACITY: usize = 256;

impl Default for KbdMsState {
    fn default() -> Self {
        Self {
            kbd_data: 0,
            kbd_ctrl: 0,
            ms_data: 0,
            ms_ctrl: 0,
            status: 0,
            keyboard_input: VecDeque::new(),
            mouse_input: VecDeque::new(),
        }
    }
}

impl KbdMsState {
    /// Queue a host keyboard scan-code byte for the guest.
    pub fn push_kbd_byte(&mut self, byte: u8) {
        if self.keyboard_input.len() < INPUT_FIFO_CAPACITY {
            self.keyboard_input.push_back(byte);
            self.status |= 1; // 8042 OBF
        } else {
            log::debug!("keyboard input FIFO full, dropping byte 0x{byte:02X}");
        }
    }

    /// Queue a host mouse packet byte for the guest.
    pub fn push_ms_byte(&mut self, byte: u8) {
        if self.mouse_input.len() < INPUT_FIFO_CAPACITY {
            self.mouse_input.push_back(byte);
            self.status |= 4; // 8042 OBF (mouse)
        } else {
            log::debug!("mouse input FIFO full, dropping byte 0x{byte:02X}");
        }
    }

    /// Whether guest-pending keyboard data is available.
    pub fn has_kbd_data(&self) -> bool {
        !self.keyboard_input.is_empty()
    }

    /// Whether guest-pending mouse data is available.
    pub fn has_ms_data(&self) -> bool {
        !self.mouse_input.is_empty()
    }

    pub fn read32(&mut self, offset: u32) -> u32 {
        match offset {
            perif::kbdms::KBD_DATA => {
                if let Some(byte) = self.keyboard_input.pop_front() {
                    self.kbd_data = u32::from(byte);
                }
                // Output-buffer-full tracks whether more bytes remain.
                let obf_set = self.has_kbd_data();
                if obf_set {
                    self.status |= 1;
                } else {
                    self.status &= !1;
                }
                self.kbd_data
            }
            perif::kbdms::MS_DATA => {
                if let Some(byte) = self.mouse_input.pop_front() {
                    self.ms_data = u32::from(byte);
                }
                let obf_set = self.has_ms_data();
                if obf_set {
                    self.status |= 4;
                } else {
                    self.status &= !4;
                }
                self.ms_data
            }
            perif::kbdms::KBD_CTRL => self.kbd_ctrl,
            perif::kbdms::MS_CTRL => self.ms_ctrl,
            perif::kbdms::STATUS => self.status,
            _ => {
                log::warn!("KBD/MS read32: unimplemented offset 0x{:04X}", offset);
                0
            }
        }
    }

    /// Read without consuming queued input bytes (used by the address guard).
    pub fn read32_immutable(&self, offset: u32) -> u32 {
        match offset {
            perif::kbdms::KBD_DATA => self.kbd_data,
            perif::kbdms::KBD_CTRL => self.kbd_ctrl,
            perif::kbdms::MS_DATA => self.ms_data,
            perif::kbdms::MS_CTRL => self.ms_ctrl,
            perif::kbdms::STATUS => self.status,
            _ => {
                log::warn!("KBD/MS read32_immutable: unimplemented offset 0x{:04X}", offset);
                0
            }
        }
    }

    pub fn write32(&mut self, offset: u32, value: u32) {
        match offset {
            perif::kbdms::KBD_DATA => self.kbd_data = value,
            perif::kbdms::KBD_CTRL => self.kbd_ctrl = value,
            perif::kbdms::MS_DATA => self.ms_data = value,
            perif::kbdms::MS_CTRL => self.ms_ctrl = value,
            perif::kbdms::STATUS => self.status = value,
            _ => {
                log::warn!("KBD/MS write32: unimplemented offset 0x{:04X} = 0x{:08X}", offset, value);
            }
        }
    }
}

/// I2C state.
#[derive(Debug, Default)]
pub struct I2cState {
    pub ctrl: u32,
    pub status: u32,
    pub data: u32,
    pub addr: u32,
}

impl I2cState {
    pub fn read32(&self, offset: u32) -> u32 {
        match offset {
            perif::i2c::CTRL => self.ctrl,
            perif::i2c::STATUS => self.status,
            perif::i2c::DATA => self.data,
            perif::i2c::ADDR => self.addr,
            _ => {
                log::warn!("I2C read32: unimplemented offset 0x{:04X}", offset);
                0
            }
        }
    }

    pub fn write32(&mut self, offset: u32, value: u32) {
        match offset {
            perif::i2c::CTRL => self.ctrl = value,
            perif::i2c::STATUS => self.status = value,
            perif::i2c::DATA => self.data = value,
            perif::i2c::ADDR => self.addr = value,
            _ => {
                log::warn!("I2C write32: unimplemented offset 0x{:04X} = 0x{:08X}", offset, value);
            }
        }
    }
}

/// UST/MSC state.
#[derive(Debug, Default)]
pub struct UstMscState {
    pub ust_lo: u32,
    pub ust_hi: u32,
    pub msc_lo: u32,
    pub msc_hi: u32,
    pub ctrl: u32,
}

impl UstMscState {
    pub fn read32(&self, offset: u32) -> u32 {
        match offset {
            perif::ustmsc::UST_LO => self.ust_lo,
            perif::ustmsc::UST_HI => self.ust_hi,
            perif::ustmsc::MSC_LO => self.msc_lo,
            perif::ustmsc::MSC_HI => self.msc_hi,
            perif::ustmsc::CTRL => self.ctrl,
            _ => {
                log::warn!("UST/MSC read32: unimplemented offset 0x{:04X}", offset);
                0
            }
        }
    }

    pub fn write32(&mut self, offset: u32, value: u32) {
        match offset {
            perif::ustmsc::UST_LO => self.ust_lo = value,
            perif::ustmsc::UST_HI => self.ust_hi = value,
            perif::ustmsc::MSC_LO => self.msc_lo = value,
            perif::ustmsc::MSC_HI => self.msc_hi = value,
            perif::ustmsc::CTRL => self.ctrl = value,
            _ => {
                log::warn!("UST/MSC write32: unimplemented offset 0x{:04X} = 0x{:08X}", offset, value);
            }
        }
    }
}

/// ISA External block state.
#[derive(Debug)]
pub struct IsaExtState {
    pub epp: EppState,
    pub ecp: EcpState,
    pub uart1: UartState,
    pub uart2: UartState,
    pub rtc: RtcState,
    pub game: GameState,
}

impl IsaExtState {
    /// Create a new ISA External block state with console I/O channels for UARTs.
    /// UART1 is used for console input/output (bidirectional).
    /// UART2 is used for console output only (tx only).
    pub fn with_console(
        uart1_tx: std::sync::mpsc::Sender<u8>,
        uart1_rx: std::sync::mpsc::Receiver<u8>,
        uart2_tx: std::sync::mpsc::Sender<u8>,
    ) -> Self {
        let (_uart2_tx_dummy, uart2_rx_dummy) = std::sync::mpsc::channel();
        Self {
            epp: EppState::default(),
            ecp: EcpState::default(),
            uart1: UartState::with_console(uart1_tx, uart1_rx),
            uart2: UartState::with_console(uart2_tx, uart2_rx_dummy),
            rtc: RtcState::default(),
            game: GameState::default(),
        }
    }

    pub fn read32(&mut self, offset: u32) -> u32 {
        match offset {
            offset if offset >= isa_ext::epp::BASE && offset <= isa_ext::epp::BASE + 0xFF => self.epp.read32(offset - isa_ext::epp::BASE),
            offset if offset >= isa_ext::ecp::BASE && offset <= isa_ext::ecp::BASE + 0xFF => self.ecp.read32(offset - isa_ext::ecp::BASE),
            offset if offset >= isa_ext::uart1::BASE && offset <= isa_ext::uart1::BASE + 0xFF => self.uart1.read32(offset - isa_ext::uart1::BASE),
            offset if offset >= isa_ext::uart2::BASE && offset <= isa_ext::uart2::BASE + 0xFF => self.uart2.read32(offset - isa_ext::uart2::BASE),
            offset if offset >= isa_ext::rtc::BASE && offset <= isa_ext::rtc::BASE + 0xFF => self.rtc.read32(offset - isa_ext::rtc::BASE),
            offset if offset >= isa_ext::game::BASE && offset <= isa_ext::game::BASE + 0xFF => self.game.read32(offset - isa_ext::game::BASE),
            _ => {
                log::warn!("ISA_EXT read32: unimplemented offset 0x{:05X}", offset);
                0
            }
        }
    }

    /// Read a 32-bit register from ISA_EXT (immutable version for read-only access).
    pub fn read32_immutable(&self, offset: u32) -> u32 {
        match offset {
            offset if offset >= isa_ext::epp::BASE && offset <= isa_ext::epp::BASE + 0xFF => self.epp.read32(offset - isa_ext::epp::BASE),
            offset if offset >= isa_ext::ecp::BASE && offset <= isa_ext::ecp::BASE + 0xFF => self.ecp.read32(offset - isa_ext::ecp::BASE),
            offset if offset >= isa_ext::uart1::BASE && offset <= isa_ext::uart1::BASE + 0xFF => self.uart1.read32_immutable(offset - isa_ext::uart1::BASE),
            offset if offset >= isa_ext::uart2::BASE && offset <= isa_ext::uart2::BASE + 0xFF => self.uart2.read32_immutable(offset - isa_ext::uart2::BASE),
            offset if offset >= isa_ext::rtc::BASE && offset <= isa_ext::rtc::BASE + 0xFF => self.rtc.read32(offset - isa_ext::rtc::BASE),
            offset if offset >= isa_ext::game::BASE && offset <= isa_ext::game::BASE + 0xFF => self.game.read32(offset - isa_ext::game::BASE),
            _ => {
                log::warn!("ISA_EXT read32_immutable: unimplemented offset 0x{:05X}", offset);
                0
            }
        }
    }

    pub fn write32(&mut self, offset: u32, value: u32) {
        match offset {
            offset if offset >= isa_ext::epp::BASE && offset <= isa_ext::epp::BASE + 0xFF => self.epp.write32(offset - isa_ext::epp::BASE, value),
            offset if offset >= isa_ext::ecp::BASE && offset <= isa_ext::ecp::BASE + 0xFF => self.ecp.write32(offset - isa_ext::ecp::BASE, value),
            offset if offset >= isa_ext::uart1::BASE && offset <= isa_ext::uart1::BASE + 0xFF => self.uart1.write32(offset - isa_ext::uart1::BASE, value),
            offset if offset >= isa_ext::uart2::BASE && offset <= isa_ext::uart2::BASE + 0xFF => self.uart2.write32(offset - isa_ext::uart2::BASE, value),
            offset if offset >= isa_ext::rtc::BASE && offset <= isa_ext::rtc::BASE + 0xFF => self.rtc.write32(offset - isa_ext::rtc::BASE, value),
            offset if offset >= isa_ext::game::BASE && offset <= isa_ext::game::BASE + 0xFF => self.game.write32(offset - isa_ext::game::BASE, value),
            _ => {
                log::warn!("ISA_EXT write32: unimplemented offset 0x{:05X} = 0x{:08X}", offset, value);
            }
        }
    }
}

/// EPP (Enhanced Parallel Port) state.
#[derive(Debug, Default)]
pub struct EppState {
    pub data: u32,
    pub addr: u32,
    pub ctrl: u32,
    pub status: u32,
}

impl EppState {
    pub fn read32(&self, offset: u32) -> u32 {
        match offset {
            isa_ext::epp::DATA => self.data,
            isa_ext::epp::ADDR => self.addr,
            isa_ext::epp::CTRL => self.ctrl,
            isa_ext::epp::STATUS => self.status,
            _ => {
                log::warn!("EPP read32: unimplemented offset 0x{:04X}", offset);
                0
            }
        }
    }

    pub fn write32(&mut self, offset: u32, value: u32) {
        match offset {
            isa_ext::epp::DATA => self.data = value,
            isa_ext::epp::ADDR => self.addr = value,
            isa_ext::epp::CTRL => self.ctrl = value,
            isa_ext::epp::STATUS => self.status = value,
            _ => {
                log::warn!("EPP write32: unimplemented offset 0x{:04X} = 0x{:08X}", offset, value);
            }
        }
    }
}

/// ECP (Extended Capabilities Port) state.
#[derive(Debug, Default)]
pub struct EcpState {
    pub data: u32,
    pub addr: u32,
    pub ctrl: u32,
    pub status: u32,
}

impl EcpState {
    pub fn read32(&self, offset: u32) -> u32 {
        match offset {
            isa_ext::ecp::DATA => self.data,
            isa_ext::ecp::ADDR => self.addr,
            isa_ext::ecp::CTRL => self.ctrl,
            isa_ext::ecp::STATUS => self.status,
            _ => {
                log::warn!("ECP read32: unimplemented offset 0x{:04X}", offset);
                0
            }
        }
    }

    pub fn write32(&mut self, offset: u32, value: u32) {
        match offset {
            isa_ext::ecp::DATA => self.data = value,
            isa_ext::ecp::ADDR => self.addr = value,
            isa_ext::ecp::CTRL => self.ctrl = value,
            isa_ext::ecp::STATUS => self.status = value,
            _ => {
                log::warn!("ECP write32: unimplemented offset 0x{:04X} = 0x{:08X}", offset, value);
            }
        }
    }
}

/// NS16550 UART state.
#[derive(Debug, Default)]
pub struct UartState {
    // Registers (with DLAB=0)
    pub rbr: u8,   // Receive Buffer Register
    pub thr: u8,   // Transmit Holding Register
    pub ier: u8,   // Interrupt Enable Register
    pub iir: u8,   // Interrupt Identification Register
    pub fcr: u8,   // FIFO Control Register
    pub lcr: u8,   // Line Control Register
    pub mcr: u8,   // Modem Control Register
    pub lsr: u8,   // Line Status Register
    pub msr: u8,   // Modem Status Register
    pub scr: u8,   // Scratch Register
    // Divisor latch (with DLAB=1)
    pub dll: u8,   // Divisor Latch Low
    pub dlm: u8,   // Divisor Latch High
    // Internal state
    pub dlab: bool,
    // Console I/O support
    pub console_tx: Option<std::sync::mpsc::Sender<u8>>,
    pub console_rx: Option<std::sync::mpsc::Receiver<u8>>,
}

impl UartState {
    /// Create a new UART state with console I/O channels.
    pub fn with_console(tx: std::sync::mpsc::Sender<u8>, rx: std::sync::mpsc::Receiver<u8>) -> Self {
        Self {
            console_tx: Some(tx),
            console_rx: Some(rx),
            ..Default::default()
        }
    }

    /// Try to read a character from the console input (non-blocking).
    fn try_read_console(&mut self) -> Option<u8> {
        if let Some(rx) = &self.console_rx {
            rx.try_recv().ok()
        } else {
            None
        }
    }

    /// Try to write a character to the console output (non-blocking).
    fn try_write_console(&mut self, ch: u8) -> bool {
        if let Some(tx) = &self.console_tx {
            tx.send(ch).is_ok()
        } else {
            false
        }
    }

    pub fn read32(&mut self, offset: u32) -> u32 {
        // NS16550 registers are 8-bit but accessed at 32-bit aligned addresses on O2
        let reg_offset = offset & 0x1F; // Only lower 5 bits used
        match reg_offset {
            0x00 => { // RBR (read) / THR (write) / DLL (DLAB=1)
                if self.dlab {
                    self.dll as u32
                } else {
                    // Try to read from console if no data in RBR
                    if self.lsr & 0x01 == 0 { // DR (Data Ready) bit not set
                        if let Some(ch) = self.try_read_console() {
                            self.rbr = ch;
                            self.lsr |= 0x01; // Set DR bit
                        }
                    }
                    self.rbr as u32
                }
            }
            0x04 => { // IER / DLM (DLAB=1)
                if self.dlab { self.dlm as u32 } else { self.ier as u32 }
            }
            0x08 => self.iir as u32, // IIR (read) / FCR (write)
            0x0C => self.lcr as u32, // LCR
            0x10 => self.mcr as u32, // MCR
            0x14 => self.lsr as u32, // LSR
            0x18 => self.msr as u32, // MSR
            0x1C => self.scr as u32, // SCR
            _ => {
                log::warn!("UART read32: unimplemented offset 0x{:04X}", offset);
                0
            }
        }
    }

    pub fn write32(&mut self, offset: u32, value: u32) {
        let reg_offset = offset & 0x1F;
        let val = value as u8;
        match reg_offset {
            0x00 => { // THR (write) / DLL (DLAB=1)
                if self.dlab {
                    self.dll = val;
                } else {
                    self.thr = val;
                    // Write to console output
                    self.try_write_console(val);
                    // Set THRE (Transmitter Holding Register Empty) bit
                    self.lsr |= 0x20;
                }
            }
            0x04 => { // IER / DLM (DLAB=1)
                if self.dlab { self.dlm = val; } else { self.ier = val; }
            }
            0x08 => self.fcr = val, // FCR
            0x0C => { // LCR
                self.lcr = val;
                self.dlab = (val & 0x80) != 0; // DLAB is bit 7
            }
            0x10 => self.mcr = val, // MCR
            0x1C => self.scr = val, // SCR
            _ => {
                log::warn!("UART write32: unimplemented offset 0x{:04X} = 0x{:08X}", offset, value);
            }
        }
    }

    /// Read a 32-bit register from UART (immutable version for read-only access).
    /// This does not modify any state (no console reads, no LSR changes).
    pub fn read32_immutable(&self, offset: u32) -> u32 {
        let reg_offset = offset & 0x1F;
        match reg_offset {
            0x00 => { // RBR / DLL (DLAB=1)
                if self.dlab {
                    self.dll as u32
                } else {
                    self.rbr as u32
                }
            }
            0x04 => { // IER / DLM (DLAB=1)
                if self.dlab { self.dlm as u32 } else { self.ier as u32 }
            }
            0x08 => self.iir as u32, // IIR
            0x0C => self.lcr as u32, // LCR
            0x10 => self.mcr as u32, // MCR
            0x14 => self.lsr as u32, // LSR
            0x18 => self.msr as u32, // MSR
            0x1C => self.scr as u32, // SCR
            _ => {
                log::warn!("UART read32_immutable: unimplemented offset 0x{:04X}", offset);
                0
            }
        }
    }
}

/// RTC (DS12887) state.
#[derive(Debug)]
pub struct RtcState {
    pub regs: [u8; 128], // 0x00-0x7F
}

impl Default for RtcState {
    fn default() -> Self {
        Self { regs: [0; 128] }
    }
}

impl RtcState {
    pub fn read32(&self, offset: u32) -> u32 {
        if offset < 128 {
            self.regs[offset as usize] as u32
        } else {
            log::warn!("RTC read32: unimplemented offset 0x{:04X}", offset);
            0
        }
    }

    pub fn write32(&mut self, offset: u32, value: u32) {
        if offset < 128 {
            self.regs[offset as usize] = value as u8;
        } else {
            log::warn!("RTC write32: unimplemented offset 0x{:04X} = 0x{:08X}", offset, value);
        }
    }
}

/// Game port state.
#[derive(Debug, Default)]
pub struct GameState {
    pub data: u32,
    pub ctrl: u32,
}

impl GameState {
    pub fn read32(&self, offset: u32) -> u32 {
        match offset {
            isa_ext::game::DATA => self.data,
            isa_ext::game::CTRL => self.ctrl,
            _ => {
                log::warn!("GAME read32: unimplemented offset 0x{:04X}", offset);
                0
            }
        }
    }

    pub fn write32(&mut self, offset: u32, value: u32) {
        match offset {
            isa_ext::game::DATA => self.data = value,
            isa_ext::game::CTRL => self.ctrl = value,
            _ => {
                log::warn!("GAME write32: unimplemented offset 0x{:04X} = 0x{:08X}", offset, value);
            }
        }
    }
}

/// Video In state (VIN1/VIN2).
#[derive(Debug, Default)]
pub struct VinState {
    pub ctrl: u32,
    pub status: u32,
    pub buf_addr: u32,
    pub buf_size: u32,
}

impl VinState {
    pub fn read32(&self, offset: u32) -> u32 {
        match offset {
            vin1::CTRL => self.ctrl,
            vin1::STATUS => self.status,
            vin1::BUF_ADDR => self.buf_addr,
            vin1::BUF_SIZE => self.buf_size,
            _ => {
                log::warn!("VIN read32: unimplemented offset 0x{:04X}", offset);
                0
            }
        }
    }

    pub fn write32(&mut self, offset: u32, value: u32) {
        match offset {
            vin1::CTRL => self.ctrl = value,
            vin1::STATUS => self.status = value,
            vin1::BUF_ADDR => self.buf_addr = value,
            vin1::BUF_SIZE => self.buf_size = value,
            _ => {
                log::warn!("VIN write32: unimplemented offset 0x{:04X} = 0x{:08X}", offset, value);
            }
        }
    }
}

/// The I/O subsystem state.
///
/// Holds the MACE ASIC state (PCI, ISA, PS/2, Ethernet, audio, SCSI).
#[derive(Debug, Default)]
pub struct Io {
    /// The MACE ASIC.
    pub mace: Mace,
}

impl Io {
    /// Create a new I/O subsystem.
    pub fn new() -> Self {
        Self { mace: Mace::new() }
    }

    /// Reset the I/O subsystem.
    pub fn reset(&mut self) {
        self.mace.reset();
    }

    /// Read a 32-bit value from MACE address space.
    pub fn read32(&self, addr: u32) -> u32 {
        if addr >= MACE_BASE && addr < MACE_BASE + 0x400000 {
            self.mace.read32_immutable(addr - MACE_BASE)
        } else {
            log::warn!("Io read32: address 0x{:08X} outside MACE range", addr);
            0
        }
    }

    /// Write a 32-bit value to MACE address space.
    pub fn write32(&mut self, addr: u32, value: u32) {
        if addr >= MACE_BASE && addr < MACE_BASE + 0x400000 {
            self.mace.write32(addr - MACE_BASE, value);
        } else {
            log::warn!("Io write32: address 0x{:08X} outside MACE range", addr);
        }
    }
}