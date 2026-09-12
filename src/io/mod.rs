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
    //
    // Register offsets and bit semantics per IRIX `sys/ad1843.h` and the IP32
    // PROM `definitions.h` (`MACE_AUDIO_*`); see also `docs/register-maps.md`.
    pub mod audio {
        pub const BASE: u32 = 0x00000;
        pub const CNTRL_STAT: u32 = 0x00;      // reset + CODEC_PRESENT + ring read aliases
        pub const CODEC_REG: u32 = 0x08;       // codec register address / data out
        pub const CODEC_INTR_MASK: u32 = 0x10; // codec read interrupt mask
        pub const CODEC_READ: u32 = 0x18;      // codec register data in
        /// Base of the three DMA channel register blocks (0x20 apart). Channel
        /// `n`'s registers live at `CH_BASE + n*0x20 + {0,8,0x10,0x18}` for
        /// control / read pointer / write pointer / depth.
        pub const CH_BASE: u32 = 0x20;
        pub const CH_STRIDE: u32 = 0x20;

        /// Frames in the output ring between the emulated AD1843 and the
        /// front-end audio thread (~46 ms at 44.1 kHz). Power of two.
        pub const RING_CAPACITY: usize = 4096;

        // cntrl_stat bits (from IRIX ad1843.h).
        pub const CODEC_RESET: u64 = 1 << 0;
        pub const CODEC_PRESENT: u64 = 1 << 1;
        pub const CH2_READ_ALIAS_SHIFT: u32 = 9;
        pub const CH2_READ_ALIAS_MASK: u64 = 0x7f00;

        // ch?_cntrl bits.
        pub const CHAN_DMA_ENABLE: u64 = 1 << 9;
        pub const CHAN_RESET: u64 = 1 << 10;

        // codec_reg command word bits.
        pub const CODEC_ADDR_SHIFT: u32 = 17;
        pub const CODEC_READ_BIT: u32 = 1 << 16;

        /// Byte range spanned by one DMA channel ring (one 4 KiB page).
        pub const RING_STRIDE: u32 = 0x1000;
    }

    // ISA Bridge (PC87312) - offset 0x10000
    //
    // `MACE_PERIPHERAL_ISA`; `ISA_RING_BASE_AND_RESET` (0x00) holds the
    // physical base of the audio DMA ring pages, `ISA_MISC_CONTROL` (0x08)
    // drives the front-panel LEDs.
    pub mod isa {
        pub const BASE: u32 = 0x10000;
        pub const RING_BASE: u32 = 0x00;     // audio DMA ring page base
        pub const MISC_CONTROL: u32 = 0x08;  // LED / flash write-enable
        pub const INT_STATUS: u32 = 0x10;    // interrupt status
        pub const INT_MASK: u32 = 0x18;      // interrupt mask
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
            offset if offset >= isa_ext::BASE && offset <= isa_ext::BASE + 0x3FFFF => {
                self.isa_ext.read32(offset - isa_ext::BASE)
            }
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
        // MACE registers are read as 32-bit words on the big-endian MIPS bus;
        // the addressed byte is the (3 - addr%4) lane of the word.
        let word = self.read32(addr);
        ((word >> ((3 - (addr & 3)) * 8)) & 0xFF) as u8
    }

    fn read16(&mut self, addr: u32) -> u16 {
        let word = self.read32(addr);
        ((word >> ((3 - (addr & 3)) * 8)) & 0xFFFF) as u16
    }

    fn read32(&mut self, addr: u32) -> u32 {
        self.read32(addr)
    }

    fn read64(&mut self, addr: u32) -> u64 {
        // MACE registers are 32-bit words packed into 64-bit big-endian slots:
        // the high half of the doubleword sits at `addr`, the low half at `addr + 4`.
        let hi = self.read32(addr) as u64;
        let lo = self.read32(addr.wrapping_add(4)) as u64;
        (hi << 32) | lo
    }

    fn write8(&mut self, addr: u32, value: u8) {
        // Read-modify-write for byte writes (big-endian lane ordering).
        let word = self.read32(addr);
        let shift = (3 - (addr & 3)) * 8;
        let new_word = (word & !(0xFF << shift)) | ((value as u32) << shift);
        self.write32(addr, new_word);
    }

    fn write16(&mut self, addr: u32, value: u16) {
        let word = self.read32(addr);
        let shift = (3 - (addr & 3)) * 8;
        let new_word = (word & !(0xFFFF << shift)) | ((value as u32) << shift);
        self.write32(addr, new_word);
    }

    fn write32(&mut self, addr: u32, value: u32) {
        self.write32(addr, value);
    }

    fn write64(&mut self, addr: u32, value: u64) {
        // Big-endian 64-bit slot: high half at `addr`, low half at `addr + 4`.
        self.write32(addr, (value >> 32) as u32);
        self.write32(addr.wrapping_add(4), value as u32);
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
            offset if offset >= perif::isa::BASE && offset <= perif::isa::BASE + 0xFF => {
                self.isa.write32(offset - perif::isa::BASE, value);
                // ISA_RING_BASE feeds the MACE audio DMA page rings.
                if (offset - perif::isa::BASE) == perif::isa::RING_BASE {
                    self.audio.set_ring_base(value);
                }
            }
            offset if offset >= perif::kbdms::BASE && offset <= perif::kbdms::BASE + 0xFF => self.kbdms.write32(offset - perif::kbdms::BASE, value),
            offset if offset >= perif::i2c::BASE && offset <= perif::i2c::BASE + 0xFF => self.i2c.write32(offset - perif::i2c::BASE, value),
            offset if offset >= perif::ustmsc::BASE && offset <= perif::ustmsc::BASE + 0xFF => self.ustmsc.write32(offset - perif::ustmsc::BASE, value),
            _ => {
                log::warn!("PERIF write32: unimplemented offset 0x{:05X} = 0x{:08X}", offset, value);
            }
        }
    }
}

/// One MACE audio DMA channel (ch1=ADC, ch2=DAC1, ch3=DAC2).
///
/// The guest drives a circular DMA ring in main memory (one 4 KiB page per
/// channel, physical = `ISA_RING_BASE` + channel*0x1000); each 64-bit ring
/// entry is one stereo frame as two `<left><right>` 32-bit samples, each
/// sample left-justified (`sample << 8`). The codec consumes the ring at the
/// configured sample rate; the emulator paces that consumption in emulated
/// time (CRIME ticks) so the PROM's `play_hello_tune` DMA loop makes progress.
#[derive(Debug, Clone, Copy, Default)]
pub struct AudioDmaChannel {
    pub control: u64,
    /// Bytes consumed by the codec from the ring (0..4095).
    pub read_ptr: u64,
    /// Bytes written by the guest into the ring (0..4095).
    pub write_ptr: u64,
    pub depth: u64,
}

/// Audio (AD1843 codec + MACE audio block) state.
///
/// The emulated codec writes digitized output samples into `out` (an rtrb
/// ring); the front-end (CLI/GUI) consumes them through the corresponding
/// [`rtrb::Consumer`] obtained from [`Mace::take_audio_consumer`].
///
/// Register layout and bit semantics follow IRIX `sys/ad1843.h` and the IP32
/// PROM `definitions.h` (`MACE_AUDIO_*`); see also `docs/register-maps.md`.
#[derive(Debug)]
pub struct AudioState {
    /// 64-bit `AUD_CNTRL_STAT`: bit0 CODEC_RESET, bit1 CODEC_PRESENT, plus
    /// the ch2 ring read-pointer alias in bits 15:9 (32-byte granularity).
    pub cntrl_stat: u64,
    /// 64-bit `AUD_CODEC_REG`: codec command (addr in bits 24:17, data/read).
    pub codec_reg: u64,
    /// 64-bit `AUD_CODEC_INTR_MASK`.
    pub codec_mask: u64,
    /// 64-bit `AUD_CODEC_READ`: latched 16-bit codec register value (lo word).
    pub codec_read: u64,
    /// The three DMA channels (ch1=ADC, ch2=DAC1, ch3=DAC2).
    pub channels: [AudioDmaChannel; 3],
    /// Physical base of the audio DMA page rings (via ISA_RING_BASE).
    pub ring_base: u32,
    /// AD1843 register file (indices per `AD1843_*` in `sys/ad1843.h`).
    pub codec: [u16; 32],
    /// Output sample rate in Hz (from AD1843_CLK_GEN1_RATE; boot tune = 34 kHz).
    pub sample_rate: f64,
    /// Fractional frames pending consumption between `advance_time` calls.
    pub pending_frames: f64,
    /// Frames the emulated codec has consumed from the DMA rings.
    pub total_frames: u64,
    /// Host-visible output sample ring (producer side fed by the codec).
    pub out: Option<rtrb::Producer<f32>>,
}

/// Emulated tick rate for MACE audio pacing: the CRIME time advances one tick
/// per CPU cycle, nominally one ~133 MHz VCLK tick per cycle. A stereo frame
/// is consumed once per `sample_rate` emulated seconds.
const AUDIO_TICK_HZ: f64 = 133.0e6;

/// Default DAC1 output rate for the boot tune when CLK_GEN1_RATE is 0.
const DEFAULT_SAMPLE_RATE: f64 = 34000.0;

impl Default for AudioState {
    fn default() -> Self {
        Self {
            cntrl_stat: 0,
            codec_reg: 0,
            codec_mask: 0,
            codec_read: 0,
            channels: [AudioDmaChannel::default(); 3],
            ring_base: 0,
            codec: [0; 32],
            sample_rate: DEFAULT_SAMPLE_RATE,
            pending_frames: 0.0,
            total_frames: 0,
            out: None,
        }
    }
}

impl AudioState {
    fn set_lane(reg: &mut u64, low_lane: bool, value: u32) {
        if low_lane {
            *reg = (*reg & !0xFFFF_FFFF) | (value as u64);
        } else {
            *reg = ((value as u64) << 32) | (*reg & 0xFFFF_FFFF);
        }
    }

    fn lane(reg: u64, low_lane: bool) -> u32 {
        if low_lane { reg as u32 } else { (reg >> 32) as u32 }
    }

    /// Reset the emulated codec (AUD_CNTRL_STAT CODEC_RESET write sequence).
    pub fn codec_hard_reset(&mut self) {
        self.codec = [0; 32];
        self.sample_rate = DEFAULT_SAMPLE_RATE;
        self.pending_frames = 0.0;
        self.codec_read = 0;
    }

    /// The physical base address of the audio DMA ring pages.
    pub fn set_ring_base(&mut self, base: u32) {
        self.ring_base = base & 0x1FFF_FFFF;
    }

    /// Apply a CODEC_REG command word (address in bits 24:17, READ flag at bit
    /// 16, 16-bit data in the low half) to the AD1843 register file.
    fn handle_codec_command(&mut self, value: u32) {
        let addr = ((value >> perif::audio::CODEC_ADDR_SHIFT) & 0xFF) as usize;
        if addr >= self.codec.len() {
            log::warn!("AUDIO codec command to out-of-range register {addr}");
            return;
        }
        let word = (value & 0xFFFF) as u16;
        if value & perif::audio::CODEC_READ_BIT != 0 {
            // Read: latch the register value for AUD_CODEC_READ to return.
            let mut v = self.codec[addr];
            // Status/revision register: INIT (bit 15) is clear once the
            // emulated codec is ready; PDNO (bit 14) mirrors the CONFIG
            // register's PDNI (power-down-initiate) bit.
            if addr == 0 {
                v = if self.codec[28] & 0x8000 != 0 { 0x4000 } else { 0 };
            }
            self.codec_read = v as u64;
        } else {
            self.write_codec_register(addr, word);
            self.codec_read = word as u64;
        }
    }

    /// Record a software write to an AD1843 register, tracking the sample rate
    /// from clock-generator-1's rate register.
    fn write_codec_register(&mut self, addr: usize, word: u16) {
        self.codec[addr] = word;
        // AD1843_CLK_GEN1_RATE (17): LSB == 1 Hz; DAC1 uses clock gen 1.
        if addr == 17 && word != 0 {
            self.sample_rate = f64::from(word);
            self.pending_frames = 0.0;
        }
    }

    fn channel_base(offset: u32) -> Option<(usize, u32)> {
        if offset < perif::audio::CH_BASE {
            return None;
        }
        let rel = offset - perif::audio::CH_BASE;
        if rel % 0x4 != 0 {
            return None;
        }
        let idx = (rel / perif::audio::CH_STRIDE) as usize;
        if idx >= 3 {
            return None;
        }
        let field = rel & 0x1C; // {0x00,0x08,0x10,0x18} (both 64-bit lanes)
        Some((idx, field))
    }

    /// The `AUD_CNTRL_STAT` value, with CODEC_PRESENT always set and the ch2
    /// read-pointer alias encoded per `GET_CH2_READ_ALIAS` (bits 15:9,
    /// 32-byte granularity).
    fn status_value(&self) -> u64 {
        let mut v = self.cntrl_stat;
        // Clear and rewrite the alias field; granularity 32 bytes matches the
        // firmware's `(cntrl >> 4) & 0xfe0` extraction.
        v &= !perif::audio::CH2_READ_ALIAS_MASK;
        let read = &self.channels[1];
        let alias = (read.read_ptr >> 5) & 0x7F;
        v |= (alias << perif::audio::CH2_READ_ALIAS_SHIFT) & perif::audio::CH2_READ_ALIAS_MASK;
        v |= perif::audio::CODEC_PRESENT;
        v
    }

    pub fn read32(&self, offset: u32) -> u32 {
        let low_lane = offset & 4 != 0;
        let base = offset & !7;
        let reg = match base {
            perif::audio::CNTRL_STAT => self.status_value(),
            perif::audio::CODEC_REG => self.codec_reg,
            perif::audio::CODEC_INTR_MASK => self.codec_mask,
            perif::audio::CODEC_READ => self.codec_read,
            b => match Self::channel_base(b) {
                Some((idx, 0x00)) => self.channels[idx].control,
                Some((idx, 0x08)) => self.channels[idx].read_ptr,
                Some((idx, 0x10)) => self.channels[idx].write_ptr,
                Some((idx, 0x18)) => self.channels[idx].depth,
                _ => {
                    log::warn!("AUDIO read32: unimplemented offset 0x{:04X}", offset);
                    return 0;
                }
            },
        };
        Self::lane(reg, low_lane)
    }

    pub fn write32(&mut self, offset: u32, value: u32) {
        let low_lane = offset & 4 != 0;
        let base = offset & !7;
        match base {
            perif::audio::CNTRL_STAT => {
                Self::set_lane(&mut self.cntrl_stat, low_lane, value);
                if low_lane && value & (perif::audio::CODEC_RESET as u32) != 0 {
                    self.codec_hard_reset();
                }
            }
            perif::audio::CODEC_REG => {
                Self::set_lane(&mut self.codec_reg, low_lane, value);
                if low_lane {
                    self.handle_codec_command(value);
                }
            }
            perif::audio::CODEC_INTR_MASK => {
                Self::set_lane(&mut self.codec_mask, low_lane, value);
            }
            perif::audio::CODEC_READ => {
                // Read-only; store for readback fidelity.
                Self::set_lane(&mut self.codec_read, low_lane, value);
            }
            b => match Self::channel_base(b) {
                Some((idx, 0x00)) => {
                    Self::set_lane(&mut self.channels[idx].control, low_lane, value);
                    if low_lane {
                        let value = value as u64;
                        if value & perif::audio::CHAN_RESET != 0 {
                            // Reset: pointers to 0 and DMA disabled.
                            self.channels[idx].read_ptr = 0;
                            self.channels[idx].write_ptr = 0;
                            self.channels[idx].control &= !perif::audio::CHAN_DMA_ENABLE;
                        }
                        if value & perif::audio::CHAN_DMA_ENABLE != 0 {
                            self.channels[idx].control |= perif::audio::CHAN_DMA_ENABLE;
                        }
                    }
                }
                Some((idx, 0x08)) => {
                    // Read pointer is hardware-controlled; store for readback.
                    Self::set_lane(&mut self.channels[idx].read_ptr, low_lane, value);
                }
                Some((idx, 0x10)) => {
                    Self::set_lane(&mut self.channels[idx].write_ptr, low_lane, value);
                }
                Some((idx, 0x18)) => {
                    Self::set_lane(&mut self.channels[idx].depth, low_lane, value);
                }
                _ => {
                    log::warn!("AUDIO write32: unimplemented offset 0x{:04X} = 0x{:08X}", offset, value);
                }
            },
        }
    }

    /// Whether DMA channel `idx` is started (CHAN_DMA_ENABLE written).
    pub fn channel_enabled(&self, idx: usize) -> bool {
        self.channels.get(idx).is_some_and(|ch| ch.control & perif::audio::CHAN_DMA_ENABLE != 0)
    }

    /// Physical address of the next unread ring entry for channel `idx` (one
    /// stereo frame, 8 bytes), or `None` when the ring is drained.
    pub fn next_frame_addr(&self, idx: usize) -> Option<u32> {
        let ch = self.channels.get(idx)?;
        if ch.read_ptr == ch.write_ptr {
            return None; // ring empty
        }
        let page = self.ring_base + (idx as u32) * perif::audio::RING_STRIDE;
        let off = ch.read_ptr & (perif::audio::RING_STRIDE - 1) as u64;
        Some(page + off as u32)
    }

    /// Consume one 64-bit ring entry (left-justified L/R 16.16 samples) from
    /// channel `idx`, decoding to mono `f32` and feeding the front-end ring.
    pub fn consume_frame(&mut self, idx: usize, entry: u64) {
        // Each 32-bit word is the 16-bit sample left-justified (`sample << 8`).
        let l = (entry >> 32) as u32 as i32;
        let sample = (l >> 8) as i16 as f32 / 32768.0;
        self.push_frame(idx, sample);
        // The codec advances the ring read pointer by one 8-byte frame.
        let ch = &mut self.channels[idx];
        ch.read_ptr = (ch.read_ptr + 8) & (perif::audio::RING_CAPACITY - 1) as u64;
    }

    /// Feed one decoded frame to the host ring (no-op headless).
    fn push_frame(&mut self, _idx: usize, sample: f32) {
        self.total_frames += 1;
        if let Some(producer) = self.out.as_mut() {
            if producer.push(sample).is_err() {
                log::debug!("audio ring full, dropped frame; total_frames={}", self.total_frames);
            }
        }
    }

    /// Advance the codec clock by `delta` CRIME ticks and return how many
    /// whole stereo frames elapsed (the DMA layers consume up to this many).
    pub fn advance_time(&mut self, delta: u64) -> u64 {
        if delta == 0 || self.sample_rate <= 0.0 {
            return 0;
        }
        self.pending_frames += delta as f64 * self.sample_rate / AUDIO_TICK_HZ;
        let whole = self.pending_frames.floor();
        self.pending_frames -= whole;
        whole as u64
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
///
/// Offset 0x00 (`ISA_RING_BASE_AND_RESET`) holds the physical base of the
/// MACE audio DMA ring pages; offset 0x08 (`ISA_MISC_CONTROL`) drives the
/// front-panel LEDs (`ISA_RED_LED`/`ISA_GREEN_LED`) and flash write-enable.
#[derive(Debug, Default)]
pub struct IsaState {
    pub ring_base: u32,
    pub misc_control: u32,
    pub int_status: u32,
    pub int_mask: u32,
}

impl IsaState {
    pub fn read32(&self, offset: u32) -> u32 {
        match offset {
            perif::isa::RING_BASE => self.ring_base,
            perif::isa::MISC_CONTROL => self.misc_control,
            perif::isa::INT_STATUS => self.int_status,
            perif::isa::INT_MASK => self.int_mask,
            _ => {
                log::warn!("ISA read32: unimplemented offset 0x{:04X}", offset);
                0
            }
        }
    }

    pub fn write32(&mut self, offset: u32, value: u32) {
        match offset {
            perif::isa::RING_BASE => self.ring_base = value & 0x1FFF_FFFF,
            perif::isa::MISC_CONTROL => self.misc_control = value,
            perif::isa::INT_STATUS => self.int_status = value,
            perif::isa::INT_MASK => self.int_mask = value,
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
///
/// The UST (Unadjusted System Time) is a free-running 64-bit counter that
/// increments once every 960 ns (`MACE_UST_PERIOD_NS`, i.e. ~127.68 CRIME
/// ticks at 133 MHz). The PROM and drivers spin on it as a delay source:
/// post1's wait routine reads `(ld, 0xbf340000)` until the counter passes a
/// deadline (`start + timeout`). The read-endianness matches the big-endian
/// 64-bit view a `ld` produces: the word at offset 0 carries the upper 32
/// bits, offset 4 the lower 32.
#[derive(Debug, Default)]
pub struct UstMscState {
    /// Microsecond timer count (unit = 960 ns).
    pub ust: u64,
    /// Media stream counter (not used by the boot path).
    pub msc: u64,
    pub ctrl: u32,
    /// Fractional UST advance pending, in 1/100 CRIME ticks.
    pending_frac: u64,
}

/// CRIME ticks per UST count: 133e6 ticks/s * 960e-9 s ≈ 127.68.
/// Represented as the integer ratio `UST_MULT/UST_DIV` (1 count per
/// 127.68 ticks) to stay in integer arithmetic.
const UST_MULT: u64 = 100;
const UST_DIV: u64 = 12768;

impl UstMscState {
    /// Advance the UST by `delta` CRIME ticks (133 MHz bus clock).
    pub fn advance(&mut self, delta: u64) {
        self.pending_frac = self.pending_frac.wrapping_add(delta.wrapping_mul(UST_MULT));
        let whole = self.pending_frac / UST_DIV;
        if whole != 0 {
            self.pending_frac %= UST_DIV;
            self.ust = self.ust.wrapping_add(whole);
        }
    }

    pub fn read32(&self, offset: u32) -> u32 {
        match offset {
            perif::ustmsc::UST_LO => (self.ust >> 32) as u32,
            perif::ustmsc::UST_HI => self.ust as u32,
            perif::ustmsc::MSC_LO => (self.msc >> 32) as u32,
            perif::ustmsc::MSC_HI => self.msc as u32,
            perif::ustmsc::CTRL => self.ctrl,
            _ => {
                log::warn!("UST/MSC read32: unimplemented offset 0x{:04X}", offset);
                0
            }
        }
    }

    pub fn write32(&mut self, offset: u32, value: u32) {
        match offset {
            perif::ustmsc::UST_LO => self.ust = (self.ust & 0xffff_ffff) | (u64::from(value) << 32),
            perif::ustmsc::UST_HI => self.ust = (self.ust & 0xffff_ffff_0000_0000) | u64::from(value),
            perif::ustmsc::MSC_LO => self.msc = (self.msc & 0xffff_ffff) | (u64::from(value) << 32),
            perif::ustmsc::MSC_HI => self.msc = (self.msc & 0xffff_ffff_0000_0000) | u64::from(value),
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
            // The O2 strides byte registers at (reg << 8), so each serial/RTC
            // window is 0x800 bytes (8 stride cells).
            offset if offset >= isa_ext::uart1::BASE && offset <= isa_ext::uart1::BASE + 0x7FF => {
                self.uart1.read32(offset - isa_ext::uart1::BASE)
            }
            offset if offset >= isa_ext::uart2::BASE && offset <= isa_ext::uart2::BASE + 0x7FF => self.uart2.read32(offset - isa_ext::uart2::BASE),
            offset if offset >= isa_ext::rtc::BASE && offset <= isa_ext::rtc::BASE + 0xF0F => self.rtc.read32(offset - isa_ext::rtc::BASE),
            offset if offset >= isa_ext::game::BASE && offset <= isa_ext::game::BASE + 0x7FF => self.game.read32(offset - isa_ext::game::BASE),
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
            offset if offset >= isa_ext::uart1::BASE && offset <= isa_ext::uart1::BASE + 0x7FF => self.uart1.read32_immutable(offset - isa_ext::uart1::BASE),
            offset if offset >= isa_ext::uart2::BASE && offset <= isa_ext::uart2::BASE + 0x7FF => self.uart2.read32_immutable(offset - isa_ext::uart2::BASE),
            offset if offset >= isa_ext::rtc::BASE && offset <= isa_ext::rtc::BASE + 0xF0F => self.rtc.read32(offset - isa_ext::rtc::BASE),
            offset if offset >= isa_ext::game::BASE && offset <= isa_ext::game::BASE + 0x7FF => self.game.read32(offset - isa_ext::game::BASE),
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
            offset if offset >= isa_ext::uart1::BASE && offset <= isa_ext::uart1::BASE + 0x7FF => self.uart1.write32(offset - isa_ext::uart1::BASE, value),
            offset if offset >= isa_ext::uart2::BASE && offset <= isa_ext::uart2::BASE + 0x7FF => self.uart2.write32(offset - isa_ext::uart2::BASE, value),
            offset if offset >= isa_ext::rtc::BASE && offset <= isa_ext::rtc::BASE + 0xF0F => self.rtc.write32(offset - isa_ext::rtc::BASE, value),
            offset if offset >= isa_ext::game::BASE && offset <= isa_ext::game::BASE + 0x7FF => self.game.write32(offset - isa_ext::game::BASE, value),
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
    /// Internal state
    pub dlab: bool,
    // Console I/O support
    // `console_tx` is the guest->host output channel (host never sees its
    // Sender); `console_out_rx` is the matching Receiver that
    // `drain_console_output` polls. `console_rx` is the host->guest input
    // channel fed by a front-end keyboard handler. Keeping output and input on
    // separate channels prevents the guest from reading back its own output.
    pub console_tx: Option<std::sync::mpsc::Sender<u8>>,
    pub console_out_rx: Option<std::sync::mpsc::Receiver<u8>>,
    pub console_rx: Option<std::sync::mpsc::Receiver<u8>>,
}

impl UartState {
    /// Create a new UART state with console I/O channels.
    /// `rx` is the host->guest input receiver; the host never sees the output
    /// Sender here since `console_out_rx` (returned by drainage) is kept inside.
    pub fn with_console(tx: std::sync::mpsc::Sender<u8>, rx: std::sync::mpsc::Receiver<u8>) -> Self {
        let _ = tx; // host->guest input sender; the guest reads it via `rx`
        let (out_tx, out_rx) = std::sync::mpsc::channel();
        Self {
            // A 16550 powers up with THRE (bit 5) and TEMT (bit 6) set; the
            // PROM polls LSR waiting for these before every putchar.
            lsr: 0x60,
            console_tx: Some(out_tx),
            console_out_rx: Some(out_rx),
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

    /// Stage an incoming console byte into RBR and set DR (Data Ready) if one
    /// is pending and DR is not already set. The PROM's scan-link monitor only
    /// ever *polls LSR for DR* (F_0xbfc01398) and reads RBR only once DR is
    /// seen, so input must be surfaced on LSR reads, not on RBR reads.
    fn poll_input(&mut self) {
        if self.lsr & 0x01 == 0 {
            if let Some(ch) = self.try_read_console() {
                self.rbr = ch;
                self.lsr |= 0x01;
            }
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
        // The O2 maps each NS16550 register at byte address (reg << 8) [+7 for
        // byte accesses]; the register *index* is bits [10:8], and the 8-bit
        // value sits in the least-significant byte.
        let reg = (offset >> 8) & 0xF;
        match reg {
            0 => { // RBR (read) / THR (write) / DLL (DLAB=1)
                if self.dlab {
                    self.dll as u32
                } else {
                    // Reading RBR clears DR; then pull any waiting host byte in.
                    let ch = self.rbr;
                    self.lsr &= !0x01;
                    if self.lsr & 0x01 == 0 { // DR cleared by the read above
                        if let Some(ch) = self.try_read_console() {
                            self.rbr = ch;
                            self.lsr |= 0x01; // Set DR bit
                        }
                    }
                    ch as u32
                }
            }
            1 => { // IER / DLM (DLAB=1)
                if self.dlab { self.dlm as u32 } else { self.ier as u32 }
            }
            2 => self.iir as u32, // IIR (read) / FCR (write)
            3 => self.lcr as u32, // LCR
            4 => self.mcr as u32, // MCR
            5 => { // LSR
                // Surface any pending host input on the DR poll; the firmware
                // spins on LSR rather than pulling RBR speculatively.
                self.poll_input();
                self.lsr as u32
            }
            6 => self.msr as u32, // MSR
            7 => self.scr as u32, // SCR
            _ => {
                log::warn!("UART read32: unimplemented offset 0x{:04X}", offset);
                0
            }
        }
    }

    pub fn write32(&mut self, offset: u32, value: u32) {
        let reg = (offset >> 8) & 0xF;
        let val = value as u8;
        match reg {
            0 => { // THR (write) / DLL (DLAB=1)
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
            1 => { // IER / DLM (DLAB=1)
                if self.dlab { self.dlm = val; } else { self.ier = val; }
            }
            2 => self.fcr = val, // FCR
            3 => { // LCR
                self.lcr = val;
                self.dlab = (val & 0x80) != 0; // DLAB is bit 7
            }
            4 => self.mcr = val, // MCR
            7 => self.scr = val, // SCR
            _ => {
                log::warn!("UART write32: unimplemented offset 0x{:04X} = 0x{:08X}", offset, value);
            }
        }
    }

    /// Read a 32-bit register from UART (immutable version for read-only access).
    /// This does not modify any state (no console reads, no LSR changes).
    pub fn read32_immutable(&self, offset: u32) -> u32 {
        let reg = (offset >> 8) & 0xF;
        match reg {
            0 => { // RBR / DLL (DLAB=1)
                if self.dlab {
                    self.dll as u32
                } else {
                    self.rbr as u32
                }
            }
            1 => { // IER / DLM (DLAB=1)
                if self.dlab { self.dlm as u32 } else { self.ier as u32 }
            }
            2 => self.iir as u32, // IIR
            3 => self.lcr as u32, // LCR
            4 => self.mcr as u32, // MCR
            5 => self.lsr as u32, // LSR
            6 => self.msr as u32, // MSR
            7 => self.scr as u32, // SCR
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
    // The O2 maps each DS12887 register at byte address (reg << 8) [+7 for
    // byte accesses on the ISA bus].
    pub fn read32(&self, offset: u32) -> u32 {
        let reg = (offset >> 8) as usize;
        if reg < self.regs.len() {
            self.regs[reg] as u32
        } else {
            log::warn!("RTC read32: unimplemented offset 0x{:04X}", offset);
            0
        }
    }

    pub fn write32(&mut self, offset: u32, value: u32) {
        let reg = (offset >> 8) as usize;
        if reg < self.regs.len() {
            self.regs[reg] = value as u8;
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

#[cfg(test)]
mod tests {
    use super::*;

    /// The O2 spaces 16550 byte registers at offset (reg << 8) within each
    /// UART window, so LSR (reg index 5) lives at uart1 + 0x500.
    const UART1_LSR: u32 = isa_ext::BASE + isa_ext::uart1::BASE + 5 * 0x100;
    const UART1_RBR: u32 = isa_ext::BASE + isa_ext::uart1::BASE + 0x000;

    /// Host->guest input must surface on LSR polls (the firmware's
    /// scan-link monitor spins reading LSR DR; it never reads RBR
    /// speculatively), and an RBR read must consume + clear DR so the byte
    /// is not echoed back to the guest twice.
    #[test]
    fn uart_stages_input_on_lsr_poll() {
        let (in_tx, in_rx) = std::sync::mpsc::channel();
        let (uart2_tx, _uart2_rx) = std::sync::mpsc::channel();
        let mut mace = Mace::with_console(in_tx.clone(), in_rx, uart2_tx);

        // Nothing queued: DR clear on LSR poll.
        assert_eq!(mace.read32(UART1_LSR) & 0x01, 0);

        // Queued byte is surfaced by the next LSR poll.
        in_tx.send(b'X').unwrap();
        assert_eq!(mace.read32(UART1_LSR) & 0x01, 0x01);

        // RBR read returns the byte and clears DR.
        assert_eq!(mace.read32(UART1_RBR), b'X' as u32);
        assert_eq!(mace.read32(UART1_LSR) & 0x01, 0);
    }

    /// cntrl_stat: CODEC_PRESENT + ch2 read-pointer alias on LSR-style polls.
    #[test]
    fn audio_cntrl_stat_reports_codec_present_and_read_alias() {
        let mut audio = AudioState::default();
        // CODEC_PRESENT (bit 1) set on every read of the lo lane.
        assert_eq!(audio.read32(perif::audio::CNTRL_STAT + 4) & 2, 2);
        // ch2 (DAC1) ring read alias advances with the codec's consumption.
        audio.channels[1].read_ptr = 512; // 64 stereo frames
        let lo = audio.read32(perif::audio::CNTRL_STAT + 4);
        let alias = (lo >> 4) & 0xfe0;
        assert_eq!(alias, 512, "GET_CH2_READ_ALIAS must return the byte offset");
    }

    /// AD1843 register file: CODEC_REG writes shadow regs, CODEC_READ returns
    /// them, and the sample rate tracks CLK_GEN1_RATE (register 17).
    #[test]
    fn audio_codec_reg_shadow_and_sample_rate() {
        let mut audio = AudioState::default();
        // Write CLK_GEN1_RATE (17) = 34000 via the codec command port.
        let cmd = (17 << perif::audio::CODEC_ADDR_SHIFT) | 0x84d0;
        audio.write32(perif::audio::CODEC_REG + 4, cmd);
        assert_eq!(audio.sample_rate, 34000.0);
        assert_eq!(audio.codec[17], 0x84d0);
        // Read it back through CODEC_READ.
        let read_cmd = (17 << perif::audio::CODEC_ADDR_SHIFT) | perif::audio::CODEC_READ_BIT;
        audio.write32(perif::audio::CODEC_REG + 4, read_cmd);
        assert_eq!(audio.read32(perif::audio::CODEC_READ + 4) & 0xffff, 0x84d0);
        // Status/revision register: STAT_REV INIT bit 15 clear, CONFIG (28)
        // PDNI (bit 15) mirrors into PDNO (bit 14).
        let read0 = (0 << perif::audio::CODEC_ADDR_SHIFT) | perif::audio::CODEC_READ_BIT;
        audio.write32(perif::audio::CODEC_REG + 4, read0);
        assert_eq!(audio.read32(perif::audio::CODEC_READ + 4) & 0x8000, 0); // INIT clear
        assert_eq!(audio.read32(perif::audio::CODEC_READ + 4) & 0x4000, 0); // PDNO clear
        audio.codec[28] = 0x8000; // CONFIG PDNI set -> PDNO set
        audio.write32(perif::audio::CODEC_REG + 4, read0);
        assert_eq!(audio.read32(perif::audio::CODEC_READ + 4) & 0x4000, 0x4000);
        // CODEC_RESET clears the register file back to default.
        audio.write32(perif::audio::CNTRL_STAT + 4, perif::audio::CODEC_RESET as u32);
        assert_eq!(audio.codec[17], 0);
    }

    /// Channel control: CHAN_RESET clears pointers + DMA, then
    /// CHAN_DMA_ENABLE starts DMA; write pointer registers are tracked.
    #[test]
    fn audio_channel_reset_and_enable() {
        let mut audio = AudioState::default();
        let ch2_ctrl = perif::audio::CH_BASE + perif::audio::CH_STRIDE + 0x00;
        assert!(!audio.channel_enabled(1));
        audio.write32(ch2_ctrl + 4, perif::audio::CHAN_RESET as u32);
        assert!(!audio.channel_enabled(1));
        audio.write32(ch2_ctrl + 4, perif::audio::CHAN_DMA_ENABLE as u32);
        assert!(audio.channel_enabled(1));
        // Track the guest's write pointer.
        let ch2_write = perif::audio::CH_BASE + perif::audio::CH_STRIDE + 0x10;
        audio.write32(ch2_write + 4, 0x100);
        assert_eq!(audio.channels[1].write_ptr, 0x100);
    }

    /// Pace: `advance_time` yields ~34 frames per emulated millisecond at the
    /// codec's 34 kHz default rate.
    #[test]
    fn audio_pacing_advances_by_sample_rate() {
        let mut audio = AudioState::default();
        assert_eq!(audio.sample_rate, 34000.0);
        // 1 ms of CRIME ticks at 133 MHz.
        let frames = audio.advance_time(133_000);
        assert!(frames >= 33 && frames <= 35, "got {frames} frames/ms");
        assert_eq!(audio.total_frames, 0);
    }

    /// The UST increments ~1.0417e6 counts/s (960 ns period). A 1 ms window
    /// at the 133 MHz CRIME clock yields ~1042 counts, and the 64-bit value
    /// splits across the offset-0 (upper) and offset-4 (lower) word lanes the
    /// way a MIPS `ld` presents it.
    #[test]
    fn ust_advances_with_crime_ticks() {
        let mut u = UstMscState::default();
        // 1 ms == 133_000 CRIME ticks.
        u.advance(133_000);
        assert!(u.ust >= 1000 && u.ust <= 1100, "got {} counts/ms", u.ust);
        // 1 s == 133_000_000 ticks == ~1.04e6 counts.
        u.advance(133_000_000 - 133_000);
        assert!(u.ust >= 1_040_000 && u.ust <= 1_045_000, "got {} counts/s", u.ust);
        // The 64-bit read at offset 0 = upper word, offset 4 = lower word.
        assert_eq!(u.read32(perif::ustmsc::UST_LO), (u.ust >> 32) as u32);
        assert_eq!(u.read32(perif::ustmsc::UST_HI), u.ust as u32);
        // Writes land in the matching half.
        u.write32(perif::ustmsc::UST_LO, 0xdead_beef);
        u.write32(perif::ustmsc::UST_HI, 0xcafe_1234);
        assert_eq!(u.ust, 0xdead_beef_cafe_1234);
    }
}