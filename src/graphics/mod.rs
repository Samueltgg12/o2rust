// src/graphics/mod.rs
//! Graphics subsystem — the CRM chipset.
//!
//! The O2 graphics pipeline consists of four ASICs:
//!
//! - **Microprocessor** (CRIME CPU Interface) — display list / vertex processing + MRE control.
//! - **ICE** (Imaging & Compression Engine) — pixel packaging/unpacking.
//! - **MRE** (Memory & Rendering Engine / Render Engine) — rasterization + texture mapping.
//! - **Display Engine** (GBE) — analog video generation.
//!
//! Under the UMA, the framebuffer and textures live in main memory (no
//! separate VRAM). The framebuffer is tile-based (GBE).
//!
//! Register maps sourced from:
//! - IRIX `stand/arcs/IP32prom/include/sys/crime.h`, `crimereg.h`, `crimedef.h`, `crimechip.h`
//! - IRIX `stand/arcs/IP32prom/include/sys/crime_gbe.h`, `gbedefs.h`
//! - Linux `drivers/video/fbdev/gbefb.c`, `include/video/gbe.h`
//! - NetBSD `sys/arch/sgimips/dev/crmfb.c`, `crmfbreg.h`

use crate::memory::AddressSpace;
use crate::ip32;

/// CRIME CPU Interface base address (Microprocessor).
pub const CRM_BASE: u32 = ip32::PHYS_BASE_CRIME; // 0x1400_0000

/// CRIME Render Engine base address (MRE).
pub const RENDER_BASE: u32 = ip32::PHYS_BASE_RENDER; // 0x1500_0000

/// GBE Display Engine base address.
pub const GBE_BASE: u32 = ip32::PHYS_BASE_GBE; // 0x1600_0000

// ============================================================================
// CRIME CPU Interface (Microprocessor) — base 0x1400_0000
// ============================================================================

/// CRIME CPU Interface register offsets (from IRIX `crime.h`).
mod crime_cpu {
    pub const CRM_ID: u32 = 0x00;
    pub const CRM_CONTROL: u32 = 0x08;
    pub const CRM_INTSTAT: u32 = 0x10;
    pub const CRM_INTMASK: u32 = 0x18;
    pub const CRM_SOFTINT: u32 = 0x20;
    pub const CRM_HARDINT: u32 = 0x28;
    pub const CRM_DOG: u32 = 0x30;
    pub const CRM_TIME: u32 = 0x38; // 64-bit at 0x38-0x3F
    pub const CRM_CPU_ERROR_ADDR: u32 = 0x40;
    pub const CRM_CPU_ERROR_STAT: u32 = 0x48;
    pub const CRM_CPU_ERROR_ENA: u32 = 0x50;
    pub const CRM_VICE_ERROR_ADDR: u32 = 0x58;
    pub const CRM_MEM_CONTROL: u32 = 0x200;
    pub const CRM_MEM_BANK_CTRL: u32 = 0x208; // 8 banks, 8 bytes each
    pub const CRM_MEM_REFRESH_CNTR: u32 = 0x248;
    pub const CRM_MEM_ERROR_STAT: u32 = 0x250;
    pub const CRM_MEM_ERROR_ADDR: u32 = 0x258;
    pub const CRM_MEM_ERROR_ECC_SYN: u32 = 0x260;
    pub const CRM_MEM_ERROR_ECC_CHK: u32 = 0x268;
    pub const CRM_MEM_ERROR_ECC_REPL: u32 = 0x270;
}

/// CRIME Control register bits (IRIX `crime.h`).
mod crime_control {
    pub const TRITON_SYSADC: u32 = 0x2000;
    pub const CRIME_SYSADC: u32 = 0x1000;
    pub const HARD_RESET: u32 = 0x0800;
    pub const SOFT_RESET: u32 = 0x0400;
    pub const DOG_ENA: u32 = 0x0200;
    pub const ENDIANESS: u32 = 0x0100;
    pub const ENDIAN_BIG: u32 = 0x0100;
    pub const ENDIAN_LITTLE: u32 = 0x0000;
    pub const CQUEUE_HWM_MASK: u32 = 0x000f;
    pub const CQUEUE_HWM_SHIFT: u32 = 0;
    pub const WBUF_HWM_MASK: u32 = 0x00f0;
    pub const WBUF_HWM_SHIFT: u32 = 8;
}

/// CRIME Interrupt bits (IRIX `crime.h`).
mod crime_int {
    pub const VICE: u32 = 0x8000_0000;
    pub const SOFT2: u32 = 0x4000_0000;
    pub const SOFT1: u32 = 0x2000_0000;
    pub const SOFT0: u32 = 0x1000_0000;
    pub const RE5: u32 = 0x0800_0000;
    pub const RE4: u32 = 0x0400_0000;
    pub const RE3: u32 = 0x0200_0000;
    pub const RE2: u32 = 0x0100_0000;
    pub const RE1: u32 = 0x0080_0000;
    pub const RE0: u32 = 0x0040_0000;
    pub const MEMERR: u32 = 0x0020_0000;
    pub const CRMERR: u32 = 0x0010_0000;
    pub const GBE3: u32 = 0x0008_0000;
    pub const GBE2: u32 = 0x0004_0000;
    pub const GBE1: u32 = 0x0002_0000;
    pub const GBE0: u32 = 0x0001_0000;
    pub const GBE_MASK: u32 = GBE0 | GBE1 | GBE2 | GBE3;
    pub const MACE_BASE: u32 = 0x0000_FFFF; // bits 0-15
}

/// CRIME CPU error status bits (IRIX `crime.h`).
mod crime_cpu_error {
    pub const CPU_ILL_ADDR: u32 = 0x4;
    pub const VICE_WRT_PRTY: u32 = 0x2;
    pub const CPU_WRT_PRTY: u32 = 0x1;
}

/// CRIME Memory error status bits (IRIX `crime.h`).
mod crime_mem_error {
    pub const MACE_ID_MASK: u32 = 0x0000_007f;
    pub const MACE_ACCESS: u32 = 0x0000_0080;
    pub const RE_ID_MASK: u32 = 0x0000_7f00;
    pub const RE_ACCESS: u32 = 0x0000_8000;
    pub const GBE_ACCESS: u32 = 0x0001_0000;
    pub const VICE_ACCESS: u32 = 0x0002_0000;
    pub const CPU_ACCESS: u32 = 0x0004_0000;
    pub const SOFT_ERR: u32 = 0x0010_0000;
    pub const HARD_ERR: u32 = 0x0020_0000;
    pub const MULTIPLE: u32 = 0x0040_0000;
    pub const MEM_ECC_RD: u32 = 0x0080_0000;
    pub const MEM_ECC_RMW: u32 = 0x0100_0000;
    pub const INV_MEM_ADDR_RD: u32 = 0x0200_0000;
    pub const INV_MEM_ADDR_WR: u32 = 0x0400_0000;
    pub const INV_MEM_ADDR_RMW: u32 = 0x0800_0000;
}

/// CRIME CPU Interface (Microprocessor) state.
#[derive(Debug)]
pub struct CrimeCpuInterface {
    // Core registers
    id: u32,
    control: u32,
    intstat: u32,
    intmask: u32,
    softint: u32,
    hardint: u32,
    dog: u32,
    time_lo: u32,
    time_hi: u32,
    cpu_error_addr: u32,
    cpu_error_stat: u32,
    cpu_error_ena: u32,
    vice_error_addr: u32,
    mem_control: u32,
    mem_bank_ctrl: [u32; 8],
    mem_refresh_cntr: u32,
    mem_error_stat: u32,
    mem_error_addr: u32,
    mem_error_ecc_syn: u32,
    mem_error_ecc_chk: u32,
    mem_error_ecc_repl: u32,
    /// Set when a hard/soft reset is written to the control register. The
    /// system loop checks this and resets the CPU to the reset vector.
    reset_requested: bool,
}

impl Default for CrimeCpuInterface {
    fn default() -> Self {
        Self {
            id: 0,
            control: 0,
            intstat: 0,
            intmask: 0,
            softint: 0,
            hardint: 0,
            dog: 0,
            time_lo: 0,
            time_hi: 0,
            cpu_error_addr: 0,
            cpu_error_stat: 0,
            cpu_error_ena: 0,
            vice_error_addr: 0,
            mem_control: 0,
            mem_bank_ctrl: [0; 8],
            mem_refresh_cntr: 0,
            mem_error_stat: 0,
            mem_error_addr: 0,
            mem_error_ecc_syn: 0,
            mem_error_ecc_chk: 0,
            mem_error_ecc_repl: 0,
            reset_requested: false,
        }
    }
}

impl CrimeCpuInterface {
    /// Create a new CRIME CPU Interface.
    pub fn new() -> Self {
        Self {
            id: 0x0000_0001, // CRIME revision 1
            control: crime_control::ENDIAN_BIG, // Big-endian default
            intmask: 0,
            mem_control: 0,
            mem_bank_ctrl: [0; 8],
            ..Default::default()
        }
    }

    /// Reset the CRIME CPU Interface.
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Whether a hard/soft reset has been requested via the control register.
    pub fn reset_requested(&self) -> bool {
        self.reset_requested
    }

    /// Clear the reset request flag (called after the system resets the CPU).
    pub fn clear_reset_request(&mut self) {
        self.reset_requested = false;
    }

    /// Get the interrupt status for the CPU.
    pub fn interrupt_status(&self) -> u32 {
        self.intstat & self.intmask
    }

    /// Assert an interrupt.
    pub fn assert_interrupt(&mut self, bit: u32) {
        self.intstat |= bit;
    }

    /// Clear an interrupt.
    pub fn clear_interrupt(&mut self, bit: u32) {
        self.intstat &= !bit;
    }
}

impl AddressSpace for CrimeCpuInterface {
    fn read8(&mut self, addr: u32) -> u8 {
        self.read32(addr) as u8
    }

    fn read16(&mut self, addr: u32) -> u16 {
        self.read32(addr) as u16
    }

    fn read32(&mut self, addr: u32) -> u32 {
        match addr {
            crime_cpu::CRM_ID => self.id,
            crime_cpu::CRM_CONTROL => self.control,
            crime_cpu::CRM_INTSTAT => self.intstat,
            crime_cpu::CRM_INTMASK => self.intmask,
            crime_cpu::CRM_SOFTINT => self.softint,
            crime_cpu::CRM_HARDINT => self.hardint,
            crime_cpu::CRM_DOG => self.dog,
            crime_cpu::CRM_TIME => self.time_lo,
            a if a == crime_cpu::CRM_TIME + 4 => self.time_hi,
            crime_cpu::CRM_CPU_ERROR_ADDR => self.cpu_error_addr,
            crime_cpu::CRM_CPU_ERROR_STAT => self.cpu_error_stat,
            crime_cpu::CRM_CPU_ERROR_ENA => self.cpu_error_ena,
            crime_cpu::CRM_VICE_ERROR_ADDR => self.vice_error_addr,
            crime_cpu::CRM_MEM_CONTROL => self.mem_control,
            a if (crime_cpu::CRM_MEM_BANK_CTRL..=crime_cpu::CRM_MEM_BANK_CTRL + 56).contains(&a) => {
                let idx = ((addr - crime_cpu::CRM_MEM_BANK_CTRL) / 8) as usize;
                if idx < 8 { self.mem_bank_ctrl[idx] } else { 0 }
            }
            crime_cpu::CRM_MEM_REFRESH_CNTR => self.mem_refresh_cntr,
            crime_cpu::CRM_MEM_ERROR_STAT => self.mem_error_stat,
            crime_cpu::CRM_MEM_ERROR_ADDR => self.mem_error_addr,
            crime_cpu::CRM_MEM_ERROR_ECC_SYN => self.mem_error_ecc_syn,
            crime_cpu::CRM_MEM_ERROR_ECC_CHK => self.mem_error_ecc_chk,
            crime_cpu::CRM_MEM_ERROR_ECC_REPL => self.mem_error_ecc_repl,
            _ => 0,
        }
    }

    fn read64(&mut self, addr: u32) -> u64 {
        if addr == crime_cpu::CRM_TIME {
            ((self.time_hi as u64) << 32) | (self.time_lo as u64)
        } else {
            ((self.read32(addr) as u64) << 32) | (self.read32(addr.wrapping_add(4)) as u64)
        }
    }

    fn write8(&mut self, addr: u32, value: u8) {
        self.write32(addr, value as u32);
    }

    fn write16(&mut self, addr: u32, value: u16) {
        self.write32(addr, value as u32);
    }

    fn write32(&mut self, addr: u32, value: u32) {
        match addr {
            crime_cpu::CRM_CONTROL => {
                self.control = value & 0x3fff; // Mask per spec
                // A hard or soft reset written to the control register resets
                // the entire system (including the CPU). Signal the system
                // loop to reset the CPU back to the reset vector.
                if value & (crime_control::HARD_RESET | crime_control::SOFT_RESET) != 0 {
                    self.reset_requested = true;
                }
            }
            crime_cpu::CRM_INTMASK => {
                self.intmask = value;
            }
            crime_cpu::CRM_SOFTINT => {
                self.softint = value;
                self.intstat = (self.intstat & !crime_int::SOFT0 & !crime_int::SOFT1 & !crime_int::SOFT2)
                    | (value & (crime_int::SOFT0 | crime_int::SOFT1 | crime_int::SOFT2));
            }
            crime_cpu::CRM_HARDINT => {
                self.hardint = value & 0xf0ff_ffff; // Mask per spec
            }
            crime_cpu::CRM_DOG => {
                self.dog = value & 0x1f_ffff; // Mask per spec
            }
            crime_cpu::CRM_TIME => {
                self.time_lo = value;
            }
            a if a == crime_cpu::CRM_TIME + 4 => {
                self.time_hi = value;
            }
            crime_cpu::CRM_CPU_ERROR_ENA => {
                self.cpu_error_ena = value & 0x7;
            }
            crime_cpu::CRM_MEM_CONTROL => {
                self.mem_control = value & 0x3;
            }
            a if (crime_cpu::CRM_MEM_BANK_CTRL..=crime_cpu::CRM_MEM_BANK_CTRL + 56).contains(&a) => {
                let idx = ((addr - crime_cpu::CRM_MEM_BANK_CTRL) / 8) as usize;
                if idx < 8 {
                    self.mem_bank_ctrl[idx] = value & 0x11f;
                }
            }
            crime_cpu::CRM_MEM_REFRESH_CNTR => {
                self.mem_refresh_cntr = value & 0x7ff;
            }
            crime_cpu::CRM_MEM_ERROR_STAT => {
                // Read-only mostly, but allow clearing some bits
                self.mem_error_stat &= !value;
            }
            _ => {
                // Ignore writes to read-only registers
            }
        }
    }

    fn write64(&mut self, addr: u32, value: u64) {
        if addr == crime_cpu::CRM_TIME {
            self.time_lo = value as u32;
            self.time_hi = (value >> 32) as u32;
        } else {
            self.write32(addr, (value >> 32) as u32);
            self.write32(addr.wrapping_add(4), value as u32);
        }
    }

    fn contains(&self, addr: u32) -> bool {
        // CRIME CPU Interface occupies 0x1000 bytes (4KB)
        addr < 0x1000
    }
}

// ============================================================================
// ICE (Imaging & Compression Engine) — part of CRIME, accessed via VICE
// ============================================================================

/// ICE (VICE) register offsets (from IRIX `crime.h` / `vice.h`).
mod ice {
    pub const VICE_BASE: u32 = 0x0000; // Relative to VICE region
    pub const VICE_ID: u32 = 0x00;
    pub const VICE_CONTROL: u32 = 0x08;
    pub const VICE_STATUS: u32 = 0x10;
    pub const VICE_INTSTAT: u32 = 0x18;
    pub const VICE_INTMASK: u32 = 0x20;
    pub const VICE_CMD_FIFO: u32 = 0x100; // Command FIFO
    pub const VICE_CMD_FIFO_END: u32 = 0x1FC; // VICE_CMD_FIFO + 252 (63 * 4)
    pub const VICE_DATA_FIFO: u32 = 0x200; // Data FIFO
    pub const VICE_DATA_FIFO_END: u32 = 0x2FC; // VICE_DATA_FIFO + 252 (63 * 4)
}

/// ICE state.
#[derive(Debug)]
pub struct Ice {
    id: u32,
    control: u32,
    status: u32,
    intstat: u32,
    intmask: u32,
    cmd_fifo: [u32; 64],
    cmd_fifo_head: usize,
    cmd_fifo_tail: usize,
    data_fifo: [u32; 64],
    data_fifo_head: usize,
    data_fifo_tail: usize,
}

impl Default for Ice {
    fn default() -> Self {
        Self {
            id: 0,
            control: 0,
            status: 0,
            intstat: 0,
            intmask: 0,
            cmd_fifo: [0; 64],
            cmd_fifo_head: 0,
            cmd_fifo_tail: 0,
            data_fifo: [0; 64],
            data_fifo_head: 0,
            data_fifo_tail: 0,
        }
    }
}

impl Ice {
    pub fn new() -> Self {
        Self {
            id: 0x0000_0002, // ICE revision
            ..Default::default()
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

impl AddressSpace for Ice {
    fn read8(&mut self, addr: u32) -> u8 {
        self.read32(addr) as u8
    }

    fn read16(&mut self, addr: u32) -> u16 {
        self.read32(addr) as u16
    }

    fn read32(&mut self, addr: u32) -> u32 {
        match addr {
            ice::VICE_ID => self.id,
            ice::VICE_CONTROL => self.control,
            ice::VICE_STATUS => self.status,
            ice::VICE_INTSTAT => self.intstat,
            ice::VICE_INTMASK => self.intmask,
            a if (ice::VICE_CMD_FIFO..=ice::VICE_CMD_FIFO_END).contains(&a) => {
                if self.cmd_fifo_head != self.cmd_fifo_tail {
                    let val = self.cmd_fifo[self.cmd_fifo_tail];
                    self.cmd_fifo_tail = (self.cmd_fifo_tail + 1) % 64;
                    val
                } else {
                    0
                }
            }
            a if (ice::VICE_DATA_FIFO..=ice::VICE_DATA_FIFO_END).contains(&a) => {
                if self.data_fifo_head != self.data_fifo_tail {
                    let val = self.data_fifo[self.data_fifo_tail];
                    self.data_fifo_tail = (self.data_fifo_tail + 1) % 64;
                    val
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    fn write8(&mut self, addr: u32, value: u8) {
        self.write32(addr, value as u32);
    }

    fn write16(&mut self, addr: u32, value: u16) {
        self.write32(addr, value as u32);
    }

    fn write32(&mut self, addr: u32, value: u32) {
        match addr {
            ice::VICE_CONTROL => {
                self.control = value;
            }
            ice::VICE_INTMASK => {
                self.intmask = value;
            }
            ice::VICE_CMD_FIFO..=ice::VICE_CMD_FIFO_END => {
                let next = (self.cmd_fifo_head + 1) % 64;
                if next != self.cmd_fifo_tail {
                    self.cmd_fifo[self.cmd_fifo_head] = value;
                    self.cmd_fifo_head = next;
                }
            }
            ice::VICE_DATA_FIFO..=ice::VICE_DATA_FIFO_END => {
                let next = (self.data_fifo_head + 1) % 64;
                if next != self.data_fifo_tail {
                    self.data_fifo[self.data_fifo_head] = value;
                    self.data_fifo_head = next;
                }
            }
            _ => {}
        }
    }

    fn contains(&self, addr: u32) -> bool {
        addr < 0x1000 // 4KB region
    }
}

// ============================================================================
// CRIME Render Engine (MRE) — base 0x1500_0000
// ============================================================================

/// Render Engine page size (4KB).
const RE_PAGE_SIZE: u32 = 0x1000;

/// Render Engine page base offsets (from IRIX `crimereg.h`).
mod re_page {
    pub const INTFBUF: u32 = 0x0000; // Page 0: Interface Buffer
    pub const TLB: u32 = 0x1000;     // Page 1: TLB
    pub const PIXPIPE: u32 = 0x2000; // Page 2: Pixel Pipe / Draw
    pub const MTE: u32 = 0x3000;     // Page 3: MTE
    pub const STATUS: u32 = 0x4000;  // Page 4: Status / SetStartPtr
}

/// Interface Buffer registers (page 0).
mod intfbuf {
    pub const DATA: u32 = 0x000; // data[64] - 256 bytes
    pub const DATA_END: u32 = 0x0FC; // DATA + 252 (63 * 4)
    pub const ADDR: u32 = 0x200; // addr[64] - 256 bytes
    pub const ADDR_END: u32 = 0x2FC; // ADDR + 252 (63 * 4)
    pub const CTL: u32 = 0x400;
    pub const RESET: u32 = 0x408;
}

/// TLB registers (page 1).
mod tlb {
    pub const FB_A: u32 = 0x000;    // 64 entries * 8 bytes = 512 bytes
    pub const FB_A_END: u32 = 0x1F8; // FB_A + 504 (63 * 8)
    pub const FB_B: u32 = 0x200;    // 64 entries
    pub const FB_B_END: u32 = 0x3F8; // FB_B + 504 (63 * 8)
    pub const FB_C: u32 = 0x400;    // 64 entries
    pub const FB_C_END: u32 = 0x5F8; // FB_C + 504 (63 * 8)
    pub const TEXTURE: u32 = 0x600; // 28 entries
    pub const TEXTURE_END: u32 = 0x6D8; // TEXTURE + 216 (27 * 8)
    pub const CID: u32 = 0x6e0;     // 4 entries
    pub const CID_END: u32 = 0x6F8; // CID + 24 (3 * 8)
    pub const LINEAR_A: u32 = 0x700; // 16 entries
    pub const LINEAR_A_END: u32 = 0x778; // LINEAR_A + 120 (15 * 8)
    pub const LINEAR_B: u32 = 0x780; // 16 entries
    pub const LINEAR_B_END: u32 = 0x7F8; // LINEAR_B + 120 (15 * 8)
}

/// Pixel Pipe / Draw registers (page 2).
mod pixpipe {
    pub const BUF_MODE_SRC: u32 = 0x000;
    pub const BUF_MODE_DST: u32 = 0x008;
    pub const CLIP_MODE: u32 = 0x010;
    pub const DRAW_MODE: u32 = 0x018;
    pub const SCISSOR: u32 = 0x048;
    pub const WIN_OFFSET_SRC: u32 = 0x050;
    pub const WIN_OFFSET_DST: u32 = 0x058;
    pub const PRIMITIVE: u32 = 0x060;
    pub const VERTEX_X: u32 = 0x070; // 3 vertices * 8 bytes
    pub const VERTEX_GL: u32 = 0x080; // 3 vertices * 8 bytes
    pub const START_SETUP: u32 = 0x098;
    pub const PIXEL_XFER_SRC: u32 = 0x0a0;
    pub const PIXEL_XFER_DST: u32 = 0x0b0;
    pub const STIPPLE: u32 = 0x0c0;
    pub const SHADE: u32 = 0x0d0; // 12 registers
    pub const TEXTURE: u32 = 0x110; // 23 registers
    pub const FOG: u32 = 0x170;
    pub const ANTIALIAS: u32 = 0x190;
    pub const ALPHA_TEST: u32 = 0x198;
    pub const BLEND: u32 = 0x1a0;
    pub const LOGIC_OP: u32 = 0x1b0;
    pub const COLOR_MASK: u32 = 0x1b8;
    pub const DEPTH: u32 = 0x1c0;
    pub const STENCIL: u32 = 0x1e0;
    pub const PIX_PIPE_NULL: u32 = 0x1f0;
    pub const PIX_PIPE_FLUSH: u32 = 0x1f8;
}

/// MTE registers (page 3).
mod mte {
    pub const MODE: u32 = 0x00;
    pub const BYTEMASK: u32 = 0x08;
    pub const STIPPLEMASK: u32 = 0x10;
    pub const FGVALUE: u32 = 0x18;
    pub const SRC0: u32 = 0x20;
    pub const SRC1: u32 = 0x28;
    pub const DST0: u32 = 0x30;
    pub const DST1: u32 = 0x38;
    pub const SRCYSTEP: u32 = 0x40;
    pub const DSTYSTEP: u32 = 0x48;
    pub const NULL: u32 = 0x70;
    pub const FLUSH: u32 = 0x78;
}

/// Status registers (page 4).
mod re_status {
    pub const STATUS: u32 = 0x00;
    pub const SET_START_PTR: u32 = 0x08;
}

/// Buffer mode bitfields (IRIX `crimedef.h`).
mod buf_mode {
    pub const DOUBLE_PIX_SEL: u32 = 1 << 0;
    pub const DOUBLE_PIX: u32 = 1 << 1;
    pub const PIX_DEPTH_SHIFT: u32 = 2;
    pub const PIX_DEPTH_MASK: u32 = 3 << 2;
    pub const PIX_TYPE_SHIFT: u32 = 4;
    pub const PIX_TYPE_MASK: u32 = 3 << 4;
    pub const BUF_DEPTH_SHIFT: u32 = 8;
    pub const BUF_DEPTH_MASK: u32 = 3 << 8;
    pub const BUF_TYPE_SHIFT: u32 = 10;
    pub const BUF_TYPE_MASK: u32 = 7 << 10;
}

/// Draw mode bitfields (IRIX `crimedef.h`).
mod draw_mode {
    pub const ENNOCONFLICT: u32 = 1 << 23;
    pub const ENGL: u32 = 1 << 22;
    pub const ENPIXXFER: u32 = 1 << 21;
    pub const ENSCISSORTEST: u32 = 1 << 20;
    pub const ENLINESTIPPLE: u32 = 1 << 19;
    pub const ENPOLYSTIPPLE: u32 = 1 << 18;
    pub const ENOPAQSTIPPLE: u32 = 1 << 17;
    pub const ENSMOOTHSHADE: u32 = 1 << 16;
    pub const ENTEXTURE: u32 = 1 << 15;
    pub const ENFOG: u32 = 1 << 14;
    pub const ENCOVERAGE: u32 = 1 << 13;
    pub const ENANTILINE: u32 = 1 << 12;
    pub const ENALPHATEST: u32 = 1 << 11;
    pub const ENBLEND: u32 = 1 << 10;
    pub const ENLOGICOP: u32 = 1 << 9;
    pub const ENDITHER: u32 = 1 << 8;
    pub const ENCOLORMASK: u32 = 1 << 7;
    pub const ENCOLORBYTEMASK_SHIFT: u32 = 3;
    pub const ENCOLORBYTEMASK_MASK: u32 = 0xf << 3;
    pub const ENDEPTHTEST: u32 = 1 << 2;
    pub const ENDEPTHMASK: u32 = 1 << 1;
    pub const ENSTENCILTEST: u32 = 1 << 0;
}

/// CRIME Render Engine (MRE) state.
#[derive(Debug)]
pub struct RenderEngine {
    // Interface Buffer (page 0)
    intfbuf_data: [u32; 64],
    intfbuf_addr: [u32; 64],
    intfbuf_ctl: u32,
    // TLB (page 1)
    tlb_fb_a: [u64; 64],
    tlb_fb_b: [u64; 64],
    tlb_fb_c: [u64; 64],
    tlb_texture: [u64; 28],
    tlb_cid: [u64; 4],
    tlb_linear_a: [u64; 16],
    tlb_linear_b: [u64; 16],
    // Pixel Pipe (page 2)
    buf_mode_src: u32,
    buf_mode_dst: u32,
    clip_mode: u32,
    draw_mode: u32,
    scissor: u32,
    win_offset_src: u32,
    win_offset_dst: u32,
    primitive: u32,
    vertex_x: [u32; 3],
    vertex_gl: [u64; 3],
    start_setup: u32,
    pixel_xfer_src: u32,
    pixel_xfer_dst: u32,
    stipple: u32,
    shade: [u32; 12],
    texture: [u32; 23],
    fog: u32,
    antialias: u32,
    alpha_test: u32,
    blend: u32,
    logic_op: u32,
    color_mask: u32,
    depth: u32,
    stencil: u32,
    // MTE (page 3)
    mte_mode: u32,
    mte_bytemask: u32,
    mte_stipplemask: u32,
    mte_fgvalue: u32,
    mte_src0: u32,
    mte_src1: u32,
    mte_dst0: u32,
    mte_dst1: u32,
    mte_srcystep: u32,
    mte_dstystep: u32,
    // Status (page 4)
    status: u32,
    set_start_ptr: u32,
}

impl Default for RenderEngine {
    fn default() -> Self {
        Self {
            intfbuf_data: [0; 64],
            intfbuf_addr: [0; 64],
            intfbuf_ctl: 0,
            tlb_fb_a: [0; 64],
            tlb_fb_b: [0; 64],
            tlb_fb_c: [0; 64],
            tlb_texture: [0; 28],
            tlb_cid: [0; 4],
            tlb_linear_a: [0; 16],
            tlb_linear_b: [0; 16],
            buf_mode_src: 0,
            buf_mode_dst: 0,
            clip_mode: 0,
            draw_mode: 0,
            scissor: 0,
            win_offset_src: 0,
            win_offset_dst: 0,
            primitive: 0,
            vertex_x: [0; 3],
            vertex_gl: [0; 3],
            start_setup: 0,
            pixel_xfer_src: 0,
            pixel_xfer_dst: 0,
            stipple: 0,
            shade: [0; 12],
            texture: [0; 23],
            fog: 0,
            antialias: 0,
            alpha_test: 0,
            blend: 0,
            logic_op: 0,
            color_mask: 0,
            depth: 0,
            stencil: 0,
            mte_mode: 0,
            mte_bytemask: 0,
            mte_stipplemask: 0,
            mte_fgvalue: 0,
            mte_src0: 0,
            mte_src1: 0,
            mte_dst0: 0,
            mte_dst1: 0,
            mte_srcystep: 0,
            mte_dstystep: 0,
            status: 0,
            set_start_ptr: 0,
        }
    }
}

impl RenderEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

impl AddressSpace for RenderEngine {
    fn read8(&mut self, addr: u32) -> u8 {
        self.read32(addr) as u8
    }

    fn read16(&mut self, addr: u32) -> u16 {
        self.read32(addr) as u16
    }

    fn read32(&mut self, addr: u32) -> u32 {
        let page = addr & 0xf000;
        let offset = addr & 0xfff;

        match page {
            re_page::INTFBUF => match offset {
                intfbuf::DATA..=intfbuf::DATA_END => {
                    let idx = ((offset - intfbuf::DATA) / 4) as usize;
                    if idx < 64 { self.intfbuf_data[idx] } else { 0 }
                }
                intfbuf::ADDR..=intfbuf::ADDR_END => {
                    let idx = ((offset - intfbuf::ADDR) / 4) as usize;
                    if idx < 64 { self.intfbuf_addr[idx] } else { 0 }
                }
                intfbuf::CTL => self.intfbuf_ctl,
                intfbuf::RESET => 0, // Write-only
                _ => 0,
            },
            re_page::TLB => {
                // TLB entries are 64-bit (2x32), return low 32 bits
                match offset {
                    tlb::FB_A..=tlb::FB_A_END => {
                        let idx = ((offset - tlb::FB_A) / 8) as usize;
                        if idx < 64 { self.tlb_fb_a[idx] as u32 } else { 0 }
                    }
                    tlb::FB_B..=tlb::FB_B_END => {
                        let idx = ((offset - tlb::FB_B) / 8) as usize;
                        if idx < 64 { self.tlb_fb_b[idx] as u32 } else { 0 }
                    }
                    tlb::FB_C..=tlb::FB_C_END => {
                        let idx = ((offset - tlb::FB_C) / 8) as usize;
                        if idx < 64 { self.tlb_fb_c[idx] as u32 } else { 0 }
                    }
                    tlb::TEXTURE..=tlb::TEXTURE_END => {
                        let idx = ((offset - tlb::TEXTURE) / 8) as usize;
                        if idx < 28 { self.tlb_texture[idx] as u32 } else { 0 }
                    }
                    tlb::CID..=tlb::CID_END => {
                        let idx = ((offset - tlb::CID) / 8) as usize;
                        if idx < 4 { self.tlb_cid[idx] as u32 } else { 0 }
                    }
                    tlb::LINEAR_A..=tlb::LINEAR_A_END => {
                        let idx = ((offset - tlb::LINEAR_A) / 8) as usize;
                        if idx < 16 { self.tlb_linear_a[idx] as u32 } else { 0 }
                    }
                    tlb::LINEAR_B..=tlb::LINEAR_B_END => {
                        let idx = ((offset - tlb::LINEAR_B) / 8) as usize;
                        if idx < 16 { self.tlb_linear_b[idx] as u32 } else { 0 }
                    }
                    _ => 0,
                }
            }
            re_page::PIXPIPE => {
                if offset == pixpipe::BUF_MODE_SRC { return self.buf_mode_src; }
                if offset == pixpipe::BUF_MODE_DST { return self.buf_mode_dst; }
                if offset == pixpipe::CLIP_MODE { return self.clip_mode; }
                if offset == pixpipe::DRAW_MODE { return self.draw_mode; }
                if offset == pixpipe::SCISSOR { return self.scissor; }
                if offset == pixpipe::WIN_OFFSET_SRC { return self.win_offset_src; }
                if offset == pixpipe::WIN_OFFSET_DST { return self.win_offset_dst; }
                if offset == pixpipe::PRIMITIVE { return self.primitive; }
                if offset >= pixpipe::VERTEX_X && offset <= pixpipe::VERTEX_X + 16 {
                    let idx = ((offset - pixpipe::VERTEX_X) / 8) as usize;
                    return if idx < 3 { self.vertex_x[idx] } else { 0 };
                }
                if offset >= pixpipe::VERTEX_GL && offset <= pixpipe::VERTEX_GL + 20 {
                    let idx = ((offset - pixpipe::VERTEX_GL) / 8) as usize;
                    return if idx < 3 { self.vertex_gl[idx] as u32 } else { 0 };
                }
                if offset == pixpipe::START_SETUP { return self.start_setup; }
                if offset == pixpipe::PIXEL_XFER_SRC { return self.pixel_xfer_src; }
                if offset == pixpipe::PIXEL_XFER_DST { return self.pixel_xfer_dst; }
                if offset == pixpipe::STIPPLE { return self.stipple; }
                if offset >= pixpipe::SHADE && offset <= pixpipe::SHADE + 44 {
                    let idx = ((offset - pixpipe::SHADE) / 4) as usize;
                    return if idx < 12 { self.shade[idx] } else { 0 };
                }
                if offset >= pixpipe::TEXTURE && offset <= pixpipe::TEXTURE + 88 {
                    let idx = ((offset - pixpipe::TEXTURE) / 4) as usize;
                    return if idx < 23 { self.texture[idx] } else { 0 };
                }
                if offset == pixpipe::FOG { return self.fog; }
                if offset == pixpipe::ANTIALIAS { return self.antialias; }
                if offset == pixpipe::ALPHA_TEST { return self.alpha_test; }
                if offset == pixpipe::BLEND { return self.blend; }
                if offset == pixpipe::LOGIC_OP { return self.logic_op; }
                if offset == pixpipe::COLOR_MASK { return self.color_mask; }
                if offset == pixpipe::DEPTH { return self.depth; }
                if offset == pixpipe::STENCIL { return self.stencil; }
                if offset == pixpipe::PIX_PIPE_NULL { return 0; }
                if offset == pixpipe::PIX_PIPE_FLUSH { return 0; }
                0
            }
            re_page::MTE => {
                if offset == mte::MODE { return self.mte_mode; }
                if offset == mte::BYTEMASK { return self.mte_bytemask; }
                if offset == mte::STIPPLEMASK { return self.mte_stipplemask; }
                if offset == mte::FGVALUE { return self.mte_fgvalue; }
                if offset == mte::SRC0 { return self.mte_src0; }
                if offset == mte::SRC1 { return self.mte_src1; }
                if offset == mte::DST0 { return self.mte_dst0; }
                if offset == mte::DST1 { return self.mte_dst1; }
                if offset == mte::SRCYSTEP { return self.mte_srcystep; }
                if offset == mte::DSTYSTEP { return self.mte_dstystep; }
                if offset == mte::NULL { return 0; }
                if offset == mte::FLUSH { return 0; }
                0
            }
            re_page::STATUS => {
                if offset == re_status::STATUS { return self.status; }
                if offset == re_status::SET_START_PTR { return self.set_start_ptr; }
                0
            }
            _ => 0,
        }
    }

    fn read64(&mut self, addr: u32) -> u64 {
        let page = addr & 0xf000;
        let offset = addr & 0xfff;

        // TLB entries are 64-bit
        if page == re_page::TLB {
            if offset >= tlb::FB_A && offset <= tlb::FB_A + 504 {
                let idx = ((offset - tlb::FB_A) / 8) as usize;
                if idx < 64 { return self.tlb_fb_a[idx]; }
            } else if offset >= tlb::FB_B && offset <= tlb::FB_B + 504 {
                let idx = ((offset - tlb::FB_B) / 8) as usize;
                if idx < 64 { return self.tlb_fb_b[idx]; }
            } else if offset >= tlb::FB_C && offset <= tlb::FB_C + 504 {
                let idx = ((offset - tlb::FB_C) / 8) as usize;
                if idx < 64 { return self.tlb_fb_c[idx]; }
            } else if offset >= tlb::TEXTURE && offset <= tlb::TEXTURE + 216 {
                let idx = ((offset - tlb::TEXTURE) / 8) as usize;
                if idx < 28 { return self.tlb_texture[idx]; }
            } else if offset >= tlb::CID && offset <= tlb::CID + 24 {
                let idx = ((offset - tlb::CID) / 8) as usize;
                if idx < 4 { return self.tlb_cid[idx]; }
            } else if offset >= tlb::LINEAR_A && offset <= tlb::LINEAR_A + 120 {
                let idx = ((offset - tlb::LINEAR_A) / 8) as usize;
                if idx < 16 { return self.tlb_linear_a[idx]; }
            } else if offset >= tlb::LINEAR_B && offset <= tlb::LINEAR_B + 120 {
                let idx = ((offset - tlb::LINEAR_B) / 8) as usize;
                if idx < 16 { return self.tlb_linear_b[idx]; }
            }
        }

        // VERTEX_GL are 64-bit
        if page == re_page::PIXPIPE && offset >= pixpipe::VERTEX_GL && offset <= pixpipe::VERTEX_GL + 20 {
            let idx = ((offset - pixpipe::VERTEX_GL) / 8) as usize;
            if idx < 3 { return self.vertex_gl[idx]; }
        }

        ((self.read32(addr) as u64) << 32) | (self.read32(addr.wrapping_add(4)) as u64)
    }

    fn write8(&mut self, addr: u32, value: u8) {
        self.write32(addr, value as u32);
    }

    fn write16(&mut self, addr: u32, value: u16) {
        self.write32(addr, value as u32);
    }

    fn write32(&mut self, addr: u32, value: u32) {
        let page = addr & 0xf000;
        let offset = addr & 0xfff;

        match page {
            re_page::INTFBUF => {
                if offset >= intfbuf::DATA && offset <= intfbuf::DATA_END {
                    let idx = ((offset - intfbuf::DATA) / 4) as usize;
                    if idx < 64 { self.intfbuf_data[idx] = value; }
                } else if offset >= intfbuf::ADDR && offset <= intfbuf::ADDR_END {
                    let idx = ((offset - intfbuf::ADDR) / 4) as usize;
                    if idx < 64 { self.intfbuf_addr[idx] = value; }
                } else if offset == intfbuf::CTL {
                    self.intfbuf_ctl = value;
                } else if offset == intfbuf::RESET {
                    // Reset interface buffer
                    self.intfbuf_ctl = 0;
                }
            }
            re_page::TLB => {
                // TLB entries are 64-bit, write low 32 bits
                if offset >= tlb::FB_A && offset <= tlb::FB_A + 504 {
                    let idx = ((offset - tlb::FB_A) / 8) as usize;
                    if idx < 64 { self.tlb_fb_a[idx] = (self.tlb_fb_a[idx] & 0xffff_ffff_0000_0000) | (value as u64); }
                } else if offset >= tlb::FB_B && offset <= tlb::FB_B + 504 {
                    let idx = ((offset - tlb::FB_B) / 8) as usize;
                    if idx < 64 { self.tlb_fb_b[idx] = (self.tlb_fb_b[idx] & 0xffff_ffff_0000_0000) | (value as u64); }
                } else if offset >= tlb::FB_C && offset <= tlb::FB_C + 504 {
                    let idx = ((offset - tlb::FB_C) / 8) as usize;
                    if idx < 64 { self.tlb_fb_c[idx] = (self.tlb_fb_c[idx] & 0xffff_ffff_0000_0000) | (value as u64); }
                } else if offset >= tlb::TEXTURE && offset <= tlb::TEXTURE + 216 {
                    let idx = ((offset - tlb::TEXTURE) / 8) as usize;
                    if idx < 28 { self.tlb_texture[idx] = (self.tlb_texture[idx] & 0xffff_ffff_0000_0000) | (value as u64); }
                } else if offset >= tlb::CID && offset <= tlb::CID + 24 {
                    let idx = ((offset - tlb::CID) / 8) as usize;
                    if idx < 4 { self.tlb_cid[idx] = (self.tlb_cid[idx] & 0xffff_ffff_0000_0000) | (value as u64); }
                } else if offset >= tlb::LINEAR_A && offset <= tlb::LINEAR_A + 120 {
                    let idx = ((offset - tlb::LINEAR_A) / 8) as usize;
                    if idx < 16 { self.tlb_linear_a[idx] = (self.tlb_linear_a[idx] & 0xffff_ffff_0000_0000) | (value as u64); }
                } else if offset >= tlb::LINEAR_B && offset <= tlb::LINEAR_B + 120 {
                    let idx = ((offset - tlb::LINEAR_B) / 8) as usize;
                    if idx < 16 { self.tlb_linear_b[idx] = (self.tlb_linear_b[idx] & 0xffff_ffff_0000_0000) | (value as u64); }
                }
            }
            re_page::PIXPIPE => {
                if offset == pixpipe::BUF_MODE_SRC { self.buf_mode_src = value; }
                else if offset == pixpipe::BUF_MODE_DST { self.buf_mode_dst = value; }
                else if offset == pixpipe::CLIP_MODE { self.clip_mode = value; }
                else if offset == pixpipe::DRAW_MODE { self.draw_mode = value; }
                else if offset == pixpipe::SCISSOR { self.scissor = value; }
                else if offset == pixpipe::WIN_OFFSET_SRC { self.win_offset_src = value; }
                else if offset == pixpipe::WIN_OFFSET_DST { self.win_offset_dst = value; }
                else if offset == pixpipe::PRIMITIVE { self.primitive = value; }
                else if offset >= pixpipe::VERTEX_X && offset <= pixpipe::VERTEX_X + 16 {
                    let idx = ((offset - pixpipe::VERTEX_X) / 8) as usize;
                    if idx < 3 { self.vertex_x[idx] = value; }
                }
                else if offset >= pixpipe::VERTEX_GL && offset <= pixpipe::VERTEX_GL + 20 {
                    let idx = ((offset - pixpipe::VERTEX_GL) / 8) as usize;
                    if idx < 3 { self.vertex_gl[idx] = (self.vertex_gl[idx] & 0xffff_ffff_0000_0000) | (value as u64); }
                }
                else if offset == pixpipe::START_SETUP { self.start_setup = value; }
                else if offset == pixpipe::PIXEL_XFER_SRC { self.pixel_xfer_src = value; }
                else if offset == pixpipe::PIXEL_XFER_DST { self.pixel_xfer_dst = value; }
                else if offset == pixpipe::STIPPLE { self.stipple = value; }
                else if offset >= pixpipe::SHADE && offset <= pixpipe::SHADE + 44 {
                    let idx = ((offset - pixpipe::SHADE) / 4) as usize;
                    if idx < 12 { self.shade[idx] = value; }
                }
                else if offset >= pixpipe::TEXTURE && offset <= pixpipe::TEXTURE + 88 {
                    let idx = ((offset - pixpipe::TEXTURE) / 4) as usize;
                    if idx < 23 { self.texture[idx] = value; }
                }
                else if offset == pixpipe::FOG { self.fog = value; }
                else if offset == pixpipe::ANTIALIAS { self.antialias = value; }
                else if offset == pixpipe::ALPHA_TEST { self.alpha_test = value; }
                else if offset == pixpipe::BLEND { self.blend = value; }
                else if offset == pixpipe::LOGIC_OP { self.logic_op = value; }
                else if offset == pixpipe::COLOR_MASK { self.color_mask = value; }
                else if offset == pixpipe::DEPTH { self.depth = value; }
                else if offset == pixpipe::STENCIL { self.stencil = value; }
                else if offset == pixpipe::PIX_PIPE_FLUSH {
                    // Flush pipeline - update status
                    self.status |= 0x1; // Pipeline busy cleared
                }
            }
            re_page::MTE => match offset {
                mte::MODE => self.mte_mode = value,
                mte::BYTEMASK => self.mte_bytemask = value,
                mte::STIPPLEMASK => self.mte_stipplemask = value,
                mte::FGVALUE => self.mte_fgvalue = value,
                mte::SRC0 => self.mte_src0 = value,
                mte::SRC1 => self.mte_src1 = value,
                mte::DST0 => self.mte_dst0 = value,
                mte::DST1 => self.mte_dst1 = value,
                mte::SRCYSTEP => self.mte_srcystep = value,
                mte::DSTYSTEP => self.mte_dstystep = value,
                mte::FLUSH => {
                    // MTE flush
                    self.status |= 0x2; // MTE done
                }
                _ => {}
            },
            re_page::STATUS => match offset {
                re_status::SET_START_PTR => self.set_start_ptr = value,
                _ => {}
            },
            _ => {}
        }
    }

    fn write64(&mut self, addr: u32, value: u64) {
        let page = addr & 0xf000;
        let offset = addr & 0xfff;

        // TLB entries are 64-bit
        if page == re_page::TLB {
            if offset >= tlb::FB_A && offset <= tlb::FB_A + 504 {
                let idx = ((offset - tlb::FB_A) / 8) as usize;
                if idx < 64 { self.tlb_fb_a[idx] = value; return; }
            } else if offset >= tlb::FB_B && offset <= tlb::FB_B + 504 {
                let idx = ((offset - tlb::FB_B) / 8) as usize;
                if idx < 64 { self.tlb_fb_b[idx] = value; return; }
            } else if offset >= tlb::FB_C && offset <= tlb::FB_C + 504 {
                let idx = ((offset - tlb::FB_C) / 8) as usize;
                if idx < 64 { self.tlb_fb_c[idx] = value; return; }
            } else if offset >= tlb::TEXTURE && offset <= tlb::TEXTURE + 216 {
                let idx = ((offset - tlb::TEXTURE) / 8) as usize;
                if idx < 28 { self.tlb_texture[idx] = value; return; }
            } else if offset >= tlb::CID && offset <= tlb::CID + 24 {
                let idx = ((offset - tlb::CID) / 8) as usize;
                if idx < 4 { self.tlb_cid[idx] = value; return; }
            } else if offset >= tlb::LINEAR_A && offset <= tlb::LINEAR_A + 120 {
                let idx = ((offset - tlb::LINEAR_A) / 8) as usize;
                if idx < 16 { self.tlb_linear_a[idx] = value; return; }
            } else if offset >= tlb::LINEAR_B && offset <= tlb::LINEAR_B + 120 {
                let idx = ((offset - tlb::LINEAR_B) / 8) as usize;
                if idx < 16 { self.tlb_linear_b[idx] = value; return; }
            }
        }

        // VERTEX_GL are 64-bit
        if page == re_page::PIXPIPE && offset >= pixpipe::VERTEX_GL && offset <= pixpipe::VERTEX_GL + 20 {
            let idx = ((offset - pixpipe::VERTEX_GL) / 8) as usize;
            if idx < 3 { self.vertex_gl[idx] = value; return; }
        }

        self.write32(addr, (value >> 32) as u32);
        self.write32(addr.wrapping_add(4), value as u32);
    }

    fn contains(&self, addr: u32) -> bool {
        // Render Engine occupies 5 pages * 4KB = 20KB
        addr < 0x5000
    }
}

// ============================================================================
// GBE Display Engine — base 0x1600_0000
// ============================================================================

/// GBE register block offsets (from IRIX `crime_gbe.h`, `gbedefs.h`).
mod gbe {
    // Control / clock / ID block (offset 0x00000)
    pub const CTRLSTAT: u32 = 0x00000;
    pub const DOTCLOCK: u32 = 0x00004;
    pub const I2C: u32 = 0x00008;
    pub const SYSCLK: u32 = 0x0000c;
    pub const I2CFP: u32 = 0x00010;
    pub const ID: u32 = 0x00014;

    // Video timing block (offset 0x10000)
    pub const VT_XY: u32 = 0x10000;
    pub const VT_XYMAX: u32 = 0x10004;
    pub const VT_VSYNC: u32 = 0x10008;
    pub const VT_HSYNC: u32 = 0x1000c;
    pub const VT_VBLANK: u32 = 0x10010;
    pub const VT_HBLANK: u32 = 0x10014;
    pub const VT_FLAGS: u32 = 0x10018;
    pub const VT_F2RF_LOCK: u32 = 0x1001c;
    pub const VT_INTR01: u32 = 0x10020;
    pub const VT_INTR23: u32 = 0x10024;
    pub const FP_HDRV: u32 = 0x10028;
    pub const FP_VDRV: u32 = 0x1002c;
    pub const FP_DE: u32 = 0x10030;
    pub const VT_HPIXEN: u32 = 0x10034;
    pub const VT_VPIXEN: u32 = 0x10038;
    pub const VT_HCMAP: u32 = 0x1003c;
    pub const VT_VCMAP: u32 = 0x10040;
    pub const DID_START_XY: u32 = 0x10044;
    pub const CRS_START_XY: u32 = 0x10048;
    pub const VC_START_XY: u32 = 0x1004c;

    // Overlay plane (offset 0x20000)
    pub const OVR_WIDTH_TILE: u32 = 0x20000;
    pub const OVR_CONTROL: u32 = 0x20004;

    // Framebuffer plane (offset 0x30000)
    pub const FRM_SIZE_TILE: u32 = 0x30000;
    pub const FRM_SIZE_PIXEL: u32 = 0x30004;
    pub const FRM_CONTROL: u32 = 0x30008;

    // DID control (offset 0x40000)
    pub const DID_CONTROL: u32 = 0x40000;

    // WID table (offset 0x48000)
    pub const WID_MODE: u32 = 0x48000; // 32 registers

    // Color / gamma map (offset 0x50000)
    pub const CMAP: u32 = 0x50000; // 4608 entries
    pub const CM_FIFO: u32 = 0x58000;
    pub const GMAP: u32 = 0x60000; // 256 entries

    // Cursor (offset 0x70000)
    pub const CRS_POS: u32 = 0x70000;
    pub const CRS_CTL: u32 = 0x70004;
    pub const CRS_CMAP: u32 = 0x70008; // 3 entries
    pub const CRS_GLYPH: u32 = 0x78000; // 64 entries

    // Video capture (offset 0x80000)
    pub const VC_LR: u32 = 0x80000;
    pub const VC_TB: u32 = 0x80004;
    pub const VC_FILTERS: u32 = 0x80008;
    pub const VC_CONTROL: u32 = 0x8000c;
}

/// GBE mode register bitfields (IRIX `gbedefs.h`).
mod gbe_mode {
    pub const WID_BUF_MASK: u32 = 0x3;
    pub const WID_TYPE_MASK: u32 = 0x1c;
    pub const WID_CM_MASK: u32 = 0x3e0;
    pub const WID_GM_MASK: u32 = 0x400;

    pub const CMODE_I8: u32 = 0;
    pub const CMODE_I12: u32 = 1;
    pub const CMODE_RG3B2: u32 = 2;
    pub const CMODE_RGB4: u32 = 3;
    pub const CMODE_RGB5: u32 = 4;
    pub const CMODE_RGB8: u32 = 5;
}

/// GBE Display Engine state.
#[derive(Debug)]
pub struct GbeDisplayEngine {
    // Control / clock / ID
    ctrlstat: u32,
    dotclock: u32,
    i2c: u32,
    sysclk: u32,
    i2cfp: u32,
    id: u32,

    // Video timing
    vt_xy: u32,
    vt_xymax: u32,
    vt_vsync: u32,
    vt_hsync: u32,
    vt_vblank: u32,
    vt_hblank: u32,
    vt_flags: u32,
    vt_f2rf_lock: u32,
    vt_intr01: u32,
    vt_intr23: u32,
    fp_hdrv: u32,
    fp_vdrv: u32,
    fp_de: u32,
    vt_hpixen: u32,
    vt_vpixen: u32,
    vt_hcmap: u32,
    vt_vcmap: u32,
    did_start_xy: u32,
    crs_start_xy: u32,
    vc_start_xy: u32,

    // Overlay
    ovr_width_tile: u32,
    ovr_control: u32,

    // Framebuffer
    frm_size_tile: u32,
    frm_size_pixel: u32,
    frm_control: u32,

    // DID
    did_control: u32,

    // WID table (32 entries)
    wid_mode: [u32; 32],

    // Color map (4608 entries)
    cmap: [u32; 4608],
    cm_fifo: u32,
    gmap: [u32; 256],

    // Cursor
    crs_pos: u32,
    crs_ctl: u32,
    crs_cmap: [u32; 3],
    crs_glyph: [u32; 64],

    // Video capture
    vc_lr: u32,
    vc_tb: u32,
    vc_filters: u32,
    vc_control: u32,

    // Framebuffer tile list (simplified - points to main memory)
    tile_list_ptr: u32,
}

impl Default for GbeDisplayEngine {
    fn default() -> Self {
        Self {
            ctrlstat: 0,
            dotclock: 0,
            i2c: 0,
            sysclk: 0,
            i2cfp: 0,
            id: 0,
            vt_xy: 0,
            vt_xymax: 0,
            vt_vsync: 0,
            vt_hsync: 0,
            vt_vblank: 0,
            vt_hblank: 0,
            vt_flags: 0,
            vt_f2rf_lock: 0,
            vt_intr01: 0,
            vt_intr23: 0,
            fp_hdrv: 0,
            fp_vdrv: 0,
            fp_de: 0,
            vt_hpixen: 0,
            vt_vpixen: 0,
            vt_hcmap: 0,
            vt_vcmap: 0,
            did_start_xy: 0,
            crs_start_xy: 0,
            vc_start_xy: 0,
            ovr_width_tile: 0,
            ovr_control: 0,
            frm_size_tile: 0,
            frm_size_pixel: 0,
            frm_control: 0,
            did_control: 0,
            wid_mode: [0; 32],
            cmap: [0; 4608],
            cm_fifo: 0,
            gmap: [0; 256],
            crs_pos: 0,
            crs_ctl: 0,
            crs_cmap: [0; 3],
            crs_glyph: [0; 64],
            vc_lr: 0,
            vc_tb: 0,
            vc_filters: 0,
            vc_control: 0,
            tile_list_ptr: 0,
        }
    }
}

impl GbeDisplayEngine {
    pub fn new() -> Self {
        Self {
            id: 0x0000_0003, // GBE revision
            ctrlstat: 0,
            vt_xymax: 0x03ff_03ff, // Default 1024x1024
            ..Default::default()
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Get the current framebuffer width in pixels.
    pub fn width(&self) -> u32 {
        (self.frm_size_tile & 0x1ff) * 8 // Tiles are 8 pixels wide
    }

    /// Get the current framebuffer height in pixels.
    pub fn height(&self) -> u32 {
        (self.frm_size_pixel >> 16) & 0xffff
    }

    /// Get the framebuffer depth (bits per pixel).
    pub fn depth(&self) -> u32 {
        (self.frm_size_tile >> 13) & 0x3
    }

    /// Get the framebuffer tile list pointer (physical address of the tile
    /// pointer list in main memory).
    pub fn tile_list_ptr(&self) -> u32 {
        self.tile_list_ptr
    }

    /// Get a color map entry as an (r, g, b) tuple.
    ///
    /// The GBE color map holds 4608 entries. Each entry is a 32-bit word
    /// encoding 8-bit RGB (the exact packing is `0x00RRGGBB` big-endian).
    pub fn cmap_entry(&self, index: usize) -> (u8, u8, u8) {
        if index >= self.cmap.len() {
            return (0, 0, 0);
        }
        let v = self.cmap[index];
        let r = ((v >> 16) & 0xff) as u8;
        let g = ((v >> 8) & 0xff) as u8;
        let b = (v & 0xff) as u8;
        (r, g, b)
    }
}

impl AddressSpace for GbeDisplayEngine {
    fn read8(&mut self, addr: u32) -> u8 {
        self.read32(addr) as u8
    }

    fn read16(&mut self, addr: u32) -> u16 {
        self.read32(addr) as u16
    }

    fn read32(&mut self, addr: u32) -> u32 {
        match addr {
            a if a == gbe::CTRLSTAT => self.ctrlstat,
            a if a == gbe::DOTCLOCK => self.dotclock,
            a if a == gbe::I2C => self.i2c,
            a if a == gbe::SYSCLK => self.sysclk,
            a if a == gbe::I2CFP => self.i2cfp,
            a if a == gbe::ID => self.id,
            a if a == gbe::VT_XY => self.vt_xy,
            a if a == gbe::VT_XYMAX => self.vt_xymax,
            a if a == gbe::VT_VSYNC => self.vt_vsync,
            a if a == gbe::VT_HSYNC => self.vt_hsync,
            a if a == gbe::VT_VBLANK => self.vt_vblank,
            a if a == gbe::VT_HBLANK => self.vt_hblank,
            a if a == gbe::VT_FLAGS => self.vt_flags,
            a if a == gbe::VT_F2RF_LOCK => self.vt_f2rf_lock,
            a if a == gbe::VT_INTR01 => self.vt_intr01,
            a if a == gbe::VT_INTR23 => self.vt_intr23,
            a if a == gbe::FP_HDRV => self.fp_hdrv,
            a if a == gbe::FP_VDRV => self.fp_vdrv,
            a if a == gbe::FP_DE => self.fp_de,
            a if a == gbe::VT_HPIXEN => self.vt_hpixen,
            a if a == gbe::VT_VPIXEN => self.vt_vpixen,
            a if a == gbe::VT_HCMAP => self.vt_hcmap,
            a if a == gbe::VT_VCMAP => self.vt_vcmap,
            a if a == gbe::DID_START_XY => self.did_start_xy,
            a if a == gbe::CRS_START_XY => self.crs_start_xy,
            a if a == gbe::VC_START_XY => self.vc_start_xy,
            a if a == gbe::OVR_WIDTH_TILE => self.ovr_width_tile,
            a if a == gbe::OVR_CONTROL => self.ovr_control,
            a if a == gbe::FRM_SIZE_TILE => self.frm_size_tile,
            a if a == gbe::FRM_SIZE_PIXEL => self.frm_size_pixel,
            a if a == gbe::FRM_CONTROL => self.frm_control,
            a if a == gbe::DID_CONTROL => self.did_control,
            a if (gbe::WID_MODE..=gbe::WID_MODE + 124).contains(&a) => {
                let idx = ((a - gbe::WID_MODE) / 4) as usize;
                if idx < 32 { self.wid_mode[idx] } else { 0 }
            }
            a if (gbe::CMAP..=gbe::CMAP + 18428).contains(&a) => {
                let idx = ((a - gbe::CMAP) / 4) as usize;
                if idx < 4608 { self.cmap[idx] } else { 0 }
            }
            a if a == gbe::CM_FIFO => self.cm_fifo,
            a if (gbe::GMAP..=gbe::GMAP + 1020).contains(&a) => {
                let idx = ((a - gbe::GMAP) / 4) as usize;
                if idx < 256 { self.gmap[idx] } else { 0 }
            }
            a if a == gbe::CRS_POS => self.crs_pos,
            a if a == gbe::CRS_CTL => self.crs_ctl,
            a if (gbe::CRS_CMAP..=gbe::CRS_CMAP + 8).contains(&a) => {
                let idx = ((a - gbe::CRS_CMAP) / 4) as usize;
                if idx < 3 { self.crs_cmap[idx] } else { 0 }
            }
            a if (gbe::CRS_GLYPH..=gbe::CRS_GLYPH + 252).contains(&a) => {
                let idx = ((a - gbe::CRS_GLYPH) / 4) as usize;
                if idx < 64 { self.crs_glyph[idx] } else { 0 }
            }
            a if a == gbe::VC_LR => self.vc_lr,
            a if a == gbe::VC_TB => self.vc_tb,
            a if a == gbe::VC_FILTERS => self.vc_filters,
            a if a == gbe::VC_CONTROL => self.vc_control,
            _ => 0,
        }
    }

    fn write8(&mut self, addr: u32, value: u8) {
        self.write32(addr, value as u32);
    }

    fn write16(&mut self, addr: u32, value: u16) {
        self.write32(addr, value as u32);
    }

    fn write32(&mut self, addr: u32, value: u32) {
        match addr {
            a if a == gbe::CTRLSTAT => self.ctrlstat = value,
            a if a == gbe::DOTCLOCK => self.dotclock = value,
            a if a == gbe::I2C => self.i2c = value,
            a if a == gbe::SYSCLK => self.sysclk = value,
            a if a == gbe::I2CFP => self.i2cfp = value,
            a if a == gbe::VT_XY => self.vt_xy = value,
            a if a == gbe::VT_XYMAX => self.vt_xymax = value,
            a if a == gbe::VT_VSYNC => self.vt_vsync = value,
            a if a == gbe::VT_HSYNC => self.vt_hsync = value,
            a if a == gbe::VT_VBLANK => self.vt_vblank = value,
            a if a == gbe::VT_HBLANK => self.vt_hblank = value,
            a if a == gbe::VT_FLAGS => self.vt_flags = value,
            a if a == gbe::VT_F2RF_LOCK => self.vt_f2rf_lock = value,
            a if a == gbe::VT_INTR01 => self.vt_intr01 = value,
            a if a == gbe::VT_INTR23 => self.vt_intr23 = value,
            a if a == gbe::FP_HDRV => self.fp_hdrv = value,
            a if a == gbe::FP_VDRV => self.fp_vdrv = value,
            a if a == gbe::FP_DE => self.fp_de = value,
            a if a == gbe::VT_HPIXEN => self.vt_hpixen = value,
            a if a == gbe::VT_VPIXEN => self.vt_vpixen = value,
            a if a == gbe::VT_HCMAP => self.vt_hcmap = value,
            a if a == gbe::VT_VCMAP => self.vt_vcmap = value,
            a if a == gbe::DID_START_XY => self.did_start_xy = value,
            a if a == gbe::CRS_START_XY => self.crs_start_xy = value,
            a if a == gbe::VC_START_XY => self.vc_start_xy = value,
            a if a == gbe::OVR_WIDTH_TILE => self.ovr_width_tile = value,
            a if a == gbe::OVR_CONTROL => self.ovr_control = value,
            a if a == gbe::FRM_SIZE_TILE => self.frm_size_tile = value,
            a if a == gbe::FRM_SIZE_PIXEL => self.frm_size_pixel = value,
            a if a == gbe::FRM_CONTROL => self.frm_control = value,
            a if a == gbe::DID_CONTROL => self.did_control = value,
            a if (gbe::WID_MODE..=gbe::WID_MODE + 124).contains(&a) => {
                let idx = ((a - gbe::WID_MODE) / 4) as usize;
                if idx < 32 { self.wid_mode[idx] = value; }
            }
            a if (gbe::CMAP..=gbe::CMAP + 18428).contains(&a) => {
                let idx = ((a - gbe::CMAP) / 4) as usize;
                if idx < 4608 { self.cmap[idx] = value; }
            }
            a if a == gbe::CM_FIFO => self.cm_fifo = value,
            a if (gbe::GMAP..=gbe::GMAP + 1020).contains(&a) => {
                let idx = ((a - gbe::GMAP) / 4) as usize;
                if idx < 256 { self.gmap[idx] = value; }
            }
            a if a == gbe::CRS_POS => self.crs_pos = value,
            a if a == gbe::CRS_CTL => self.crs_ctl = value,
            a if (gbe::CRS_CMAP..=gbe::CRS_CMAP + 8).contains(&a) => {
                let idx = ((a - gbe::CRS_CMAP) / 4) as usize;
                if idx < 3 { self.crs_cmap[idx] = value; }
            }
            a if (gbe::CRS_GLYPH..=gbe::CRS_GLYPH + 252).contains(&a) => {
                let idx = ((a - gbe::CRS_GLYPH) / 4) as usize;
                if idx < 64 { self.crs_glyph[idx] = value; }
            }
            a if a == gbe::VC_LR => self.vc_lr = value,
            a if a == gbe::VC_TB => self.vc_tb = value,
            a if a == gbe::VC_FILTERS => self.vc_filters = value,
            a if a == gbe::VC_CONTROL => self.vc_control = value,
            _ => {}
        }
    }

    fn contains(&self, addr: u32) -> bool {
        // GBE register block is ~512KB (0x80000)
        addr < 0x80000
    }
}

// ============================================================================
// Graphics subsystem — aggregates all four components
// ============================================================================

/// The complete graphics subsystem (CRM chipset).
#[derive(Debug)]
pub struct Graphics {
    /// CRIME CPU Interface (Microprocessor)
    pub crime_cpu: CrimeCpuInterface,
    /// ICE (Imaging & Compression Engine)
    pub ice: Ice,
    /// Render Engine (MRE)
    pub render_engine: RenderEngine,
    /// GBE Display Engine
    pub gbe: GbeDisplayEngine,
}

impl Default for Graphics {
    fn default() -> Self {
        Self::new()
    }
}

impl Graphics {
    /// Create a new graphics subsystem with all four CRM components.
    pub fn new() -> Self {
        Self {
            crime_cpu: CrimeCpuInterface::new(),
            ice: Ice::new(),
            render_engine: RenderEngine::new(),
            gbe: GbeDisplayEngine::new(),
        }
    }

    /// Reset all graphics components.
    pub fn reset(&mut self) {
        self.crime_cpu.reset();
        self.ice.reset();
        self.render_engine.reset();
        self.gbe.reset();
    }

    /// Get the framebuffer width in pixels.
    pub fn width(&self) -> u32 {
        self.gbe.width()
    }

    /// Get the framebuffer height in pixels.
    pub fn height(&self) -> u32 {
        self.gbe.height()
    }

    /// Get the framebuffer depth (bits per pixel).
    pub fn depth(&self) -> u32 {
        self.gbe.depth()
    }
}