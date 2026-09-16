// src/memory/mod.rs
//! Memory subsystem: address space abstraction and physical memory.
//!
//! The O2 uses a Unified Memory Architecture (UMA) — CPU, graphics, and I/O
//! all share main memory. This module provides:
//!
//! - [`PhysicalMemory`] — the main RAM backing store.
//! - [`AddressSpace`] — a trait for addressable regions (RAM, ROM, MMIO).
//! - [`MemoryMap`] — routes physical addresses to the right region.

pub mod cache;
pub mod physical;

pub use cache::DCache;

use crate::graphics::{CrimeCpuInterface, GbeDisplayEngine, Ice, RenderEngine};
use crate::io::ahc::HostMemory;
use crate::io::Mace;
use crate::ip32;

/// Host-memory bridge for the AIC-7880 DMA engine.
///
/// On the O2 (UMA) PCI masters address CPU physical memory directly, so the
/// sequencer's SCB / data DMA lands in [`PhysicalMemory`] at the bus
/// addresses the guest driver programs into HADDR. Out-of-range accesses read
/// zero / are dropped, mirroring `PhysicalMemory`'s own semantics.
pub struct AhcHostRam<'a>(pub &'a mut physical::PhysicalMemory);

impl HostMemory for AhcHostRam<'_> {
    // AIC-7880 DMA addresses carry the MACE view bits in the top nibble:
    // 0x4000_0000 = PCI_NATIVE_VIEW. Mask them off to reach physical RAM.
    fn byte_read(&mut self, addr: u32) -> u8 {
        let a = addr & 0x3FFF_FFFF;
        if (a as usize) < self.0.len() {
            self.0.read8(a)
        } else {
            0
        }
    }
    fn byte_write(&mut self, addr: u32, value: u8) {
        let a = addr & 0x3FFF_FFFF;
        if (a as usize) < self.0.len() {
            self.0.write8(a, value);
        }
    }
}

/// A region of the physical address space that can be read from and written
/// to. Devices (CRIME, MACE, GBE, PROM) implement this trait.
pub trait AddressSpace {
    /// Read a single byte at `addr`.
    fn read8(&mut self, addr: u32) -> u8;
    /// Read a 16-bit halfword at `addr` (big-endian).
    fn read16(&mut self, addr: u32) -> u16;
    /// Read a 32-bit word at `addr` (big-endian).
    fn read32(&mut self, addr: u32) -> u32;
    /// Read a 64-bit doubleword at `addr` (big-endian).
    fn read64(&mut self, addr: u32) -> u64 {
        ((self.read32(addr) as u64) << 32) | (self.read32(addr.wrapping_add(4)) as u64)
    }

    /// Write a single byte to `addr`.
    fn write8(&mut self, addr: u32, value: u8);
    /// Write a 16-bit halfword to `addr` (big-endian).
    fn write16(&mut self, addr: u32, value: u16);
    /// Write a 32-bit word to `addr` (big-endian).
    fn write32(&mut self, addr: u32, value: u32);
    /// Write a 64-bit doubleword to `addr` (big-endian).
    fn write64(&mut self, addr: u32, value: u64) {
        self.write32(addr, (value >> 32) as u32);
        self.write32(addr.wrapping_add(4), value as u32);
    }

    /// Whether this region contains `addr`.
    fn contains(&self, addr: u32) -> bool;
}

/// Routes physical addresses to the appropriate [`AddressSpace`] region.
///
/// This is the top-level memory map. It owns the RAM, PROM, and MMIO device
/// regions and dispatches reads/writes to them.
pub struct MemoryMap {
    /// Main memory (RAM).
    pub ram: physical::PhysicalMemory,
    /// PROM / system ROM.
    pub rom: physical::PhysicalMemory,
    /// CRIME CPU interface (Microprocessor) at 0x1400_0000.
    pub crime_cpu: CrimeCpuInterface,
    /// ICE (Imaging & Compression Engine / VICE) at 0x1700_0000.
    pub ice: Ice,
    /// Render Engine (MRE) at 0x1500_0000.
    pub render_engine: RenderEngine,
    /// GBE display engine at 0x1600_0000.
    pub gbe: GbeDisplayEngine,
    /// MACE I/O engine (MMIO).
    pub mace: Mace,
}

impl MemoryMap {
    /// Create a new memory map with `ram_mb` megabytes of RAM and console I/O channels for UARTs.
    /// UART1 is used for console input/output (bidirectional).
    /// UART2 is used for console output only (tx only).
    pub fn new(
        ram_mb: u32,
        uart1_tx: std::sync::mpsc::Sender<u8>,
        uart1_rx: std::sync::mpsc::Receiver<u8>,
        uart2_tx: std::sync::mpsc::Sender<u8>,
    ) -> Self {
        let ram_size = ram_mb.min(ip32::MAX_MEMORY / (1024 * 1024)) * 1024 * 1024;
        Self {
            ram: physical::PhysicalMemory::new(ram_size as usize),
            rom: physical::PhysicalMemory::new(ip32::SYSTEM_ROM_WINDOW_SIZE as usize),
            crime_cpu: CrimeCpuInterface::new(),
            ice: Ice::new(),
            render_engine: RenderEngine::new(),
            gbe: GbeDisplayEngine::new(),
            mace: Mace::with_console(uart1_tx, uart1_rx, uart2_tx),
        }
    }

    /// Create a new memory map with `ram_mb` megabytes of RAM (without console I/O).
    pub fn new_without_console(ram_mb: u32) -> Self {
        let (uart1_tx, uart1_rx) = std::sync::mpsc::channel();
        let (uart2_tx, _uart2_rx) = std::sync::mpsc::channel();
        Self::new(ram_mb, uart1_tx, uart1_rx, uart2_tx)
    }

    /// Render the GBE framebuffer into a linear RGBA8 buffer.
    ///
    /// The GBE framebuffer is **tile-based**, not linear. Tiles are 64 KiB
    /// (128 lines × 512 bytes), aligned on 64 KiB boundaries. The pixel width
    /// of a tile depends on the pixel depth:
    ///
    /// | depth | bytes/pixel | pixels per tile row |
    /// |-------|-------------|---------------------|
    /// | 8bpp  | 1           | 512                 |
    /// | 16bpp | 2           | 256                 |
    /// | 32bpp | 4           | 128                 |
    ///
    /// The `tile_list_ptr` register points to a list of 16-bit entries (the
    /// upper 16 bits of each tile's physical address), ordered top-to-bottom,
    /// left-to-right. Pixels are big-endian within the 256-bit memory width.
    ///
    /// `out` must be at least `width * height * 4` bytes. Returns the number
    /// of pixels written (width × height).
    pub fn render_framebuffer(&self, out: &mut [u8]) -> usize {
        let width = self.gbe.width() as usize;
        let height = self.gbe.height() as usize;
        let depth = self.gbe.depth();

        // Bytes per pixel and pixels per tile row from the depth code.
        let (bytes_per_pixel, pixels_per_tile_row): (usize, usize) = match depth {
            0 => (1, 512), // 8bpp
            1 => (2, 256), // 16bpp
            2 => (4, 128), // 32bpp
            _ => (4, 128), // unknown → treat as 32bpp
        };

        const TILE_LINES: usize = 128;
        const TILE_ROW_BYTES: usize = 512;

        let tiles_per_row = width.div_ceil(pixels_per_tile_row);
        let tile_list_ptr = self.gbe.tile_list_ptr() as usize;

        let ram = self.ram.as_slice();
        let required = width * height * 4;
        if out.len() < required {
            return 0;
        }

        for y in 0..height {
            // The RE addresses the framebuffer in GL convention (origin at the
            // bottom); the GBE scans out top-down, so tile rows are stored in
            // reverse order — flip vertically only.
            let ym = height - 1 - y;
            let tile_y = ym / TILE_LINES;
            let line_in_tile = ym % TILE_LINES;
            for x in 0..width {
                let tile_x = x / pixels_per_tile_row;
                let px_in_tile = x % pixels_per_tile_row;

                let tile_index = tile_y * tiles_per_row + tile_x;

                // Read the 16-bit tile pointer (upper 16 bits of the tile's
                // physical address) from the tile pointer list in main memory.
                let ptr_off = tile_list_ptr + tile_index * 2;
                let tile_ptr = if ptr_off + 2 <= ram.len() {
                    ((ram[ptr_off] as u32) << 8) | (ram[ptr_off + 1] as u32)
                } else {
                    0
                };

                // Physical address of the pixel within the tile.
                let tile_base = (tile_ptr << 16) as usize;
                let px_off = tile_base
                    + line_in_tile * TILE_ROW_BYTES
                    + px_in_tile * bytes_per_pixel;

                // Read the pixel (big-endian) and expand to RGBA8.
                let (r, g, b) = if px_off + bytes_per_pixel <= ram.len() {
                    match depth {
                        0 => {
                            // 8bpp: index into the GBE color map (cmap).
                            let idx = ram[px_off] as usize;
                            let entry = self.gbe.cmap_entry(idx);
                            (entry.0, entry.1, entry.2)
                        }
                        1 => {
                            // 16bpp: RGB565 (big-endian).
                            let v = ((ram[px_off] as u16) << 8) | (ram[px_off + 1] as u16);
                            let r = ((v >> 11) & 0x1f) as u8;
                            let g = ((v >> 5) & 0x3f) as u8;
                            let b = (v & 0x1f) as u8;
                            (
                                (r << 3) | (r >> 2),
                                (g << 2) | (g >> 4),
                                (b << 3) | (b >> 2),
                            )
                        }
                        _ => {
                            // 32bpp: ARGB8888 (big-endian).
                            let a = ram[px_off];
                            let r = ram[px_off + 1];
                            let g = ram[px_off + 2];
                            let b = ram[px_off + 3];
                            let _ = a;
                            (r, g, b)
                        }
                    }
                } else {
                    (0, 0, 0)
                };

                let out_off = (y * width + x) * 4;
                out[out_off] = r;
                out[out_off + 1] = g;
                out[out_off + 2] = b;
                out[out_off + 3] = 0xff;
            }
        }

        self.draw_cursor(out, width, height);

        width * height
    }

    /// Overlay the GBE hardware cursor when enabled (`crs_ctl & 1`, see
    /// crm_tp.c `CrmTpBlankscreen`). The glyph is a 32x32, 3-color cursor:
    /// 64 words × 16 packed 2bpp pixels, with `crs_glyph[2r]` and
    /// `crs_glyph[2r+1]` the left and right 16 pixels of glyph row `r`
    /// (GBE spec §2.10). `crm_cursor` (see crm_cursor.c) fills the top-left
    /// 16×16 quadrant with the arrow; the remaining words are zero.
    ///
    /// Pixels within a word are MSB-first: the bit pair `(2c, 2c+1)` counted
    /// from bit 30 down is pixel column `c` (confirmed against the PROM's
    /// `crm_cursor.c` bitmap, whose left-pointing arrow only decodes with this
    /// order; an LSB-first read renders the glyph mirrored), and value 1 =
    /// `crs_cmap[0]`, 2 = `crs_cmap[1]`, 3 = `crs_cmap[2]`. initCursor only
    /// programs cmap[0]=white and cmap[1]=red (crm_init.c), and the arrow
    /// uses exactly values 1 and 2 — a red body with a white outline.
    /// `crs_pos` is the glyph's upper-left
    /// corner in screen (scanline) coordinates; `out` is in RE order
    /// (row 0 = guest bottom scanline), so the overlay row must be
    /// `height-1-(cy+row)` to appear at the correct guest scanline.
    fn draw_cursor(&self, out: &mut [u8], width: usize, height: usize) {
        if self.gbe.crs_enabled() && width > 0 && height > 0 {
            let (cx, cy) = self.gbe.crs_position();
            let cm = self.gbe.crs_cmap();
            for (i, word) in self.gbe.crs_glyph().iter().enumerate() {
                let row = (i / 2) as i64;
                let x0 = ((i % 2) * 16) as i64;
                let y = height as i64 - 1 - (cy as i64 + row);
                if y < 0 || y >= height as i64 {
                    continue;
                }
                for col in 0..16i64 {
                    let x = cx as i64 + x0 + col;
                    if x < 0 || x >= width as i64 {
                        continue;
                    }
                    // MSB-first 2bpp: pixel col = bits (2c, 2c+1) from bit 30.
                    let bit = 30 - 2 * col as u32;
                    let p = (word >> bit) & 3;
                    if p == 0 {
                        continue;
                    }
                    let entry = cm[(p - 1) as usize];
                    let o = (y as usize * width + x as usize) * 4;
                    out[o] = (entry >> 24) as u8;
                    out[o + 1] = (entry >> 16) as u8;
                    out[o + 2] = (entry >> 8) as u8;
                    out[o + 3] = 0xff;
                }
            }
        }
    }

    /// Read a 32-bit word from the physical address space.
    pub fn read32(&mut self, addr: u32) -> u32 {
        match addr {
            a if a < ip32::PHYS_BASE_CRIME => self.ram.read32(a),
            a if a < ip32::PHYS_BASE_RENDER => self.crime_cpu.read32(a - ip32::PHYS_BASE_CRIME),
            a if a < ip32::PHYS_BASE_GBE => self.render_engine.read32(a - ip32::PHYS_BASE_RENDER),
            a if a < ip32::PHYS_BASE_ICE => self.gbe.read32(a - ip32::PHYS_BASE_GBE),
            a if a < ip32::PHYS_PCI_IO => self.ice.read32(a - ip32::PHYS_BASE_ICE),
            // PCI low I/O window (0x18000000).
            a if a < ip32::PHYS_PCI_IO + ip32::PHYS_SIZE_PCI_IO => self.mace.pci_window_read32(a),
            a if a < ip32::PHYS_PCI_MEM => {
                log::warn!("read32: unassigned physical hole at {:#010X}", a);
                0xFFFF_FFFF
            }
            // PCI low memory window (0x1A000000..0x1BFFFFFF).
            a if a < ip32::PHYS_PCI_MEM + ip32::PHYS_SIZE_PCI_MEM => {
                self.mace.pci_window_read32(a)
            }
            a if a < ip32::PHYS_BASE_MACE => {
                log::warn!("read32: unassigned physical hole at {:#010X}", a);
                0xFFFF_FFFF
            }
            a if a < ip32::PHYS_SYSTEM_ROM => self.mace.read32(a - ip32::PHYS_BASE_MACE),
            a => self.rom.read32(a - ip32::PHYS_SYSTEM_ROM),
        }
    }

    /// Write a 32-bit word to the physical address space.
    pub fn write32(&mut self, addr: u32, value: u32) {
        match addr {
            a if a < ip32::PHYS_BASE_CRIME => self.ram.write32(a, value),
            a if a < ip32::PHYS_BASE_RENDER => {
                self.crime_cpu.write32(a - ip32::PHYS_BASE_CRIME, value)
            }
            a if a < ip32::PHYS_BASE_GBE => {
                let off = a - ip32::PHYS_BASE_RENDER;
                self.render_engine.write32(off, value);
                // "Go" register write: execute the pending MRE primitive
                // into the framebuffer (see graphics::RenderEngine::rasterize).
                if off == crate::graphics::RenderEngine::GO_OFFSET {
                    self.render_engine.rasterize(&self.gbe, &mut self.ram);
                }
            }
            a if a < ip32::PHYS_BASE_ICE => {
                self.gbe.write32(a - ip32::PHYS_BASE_GBE, value)
            }
            a if a < ip32::PHYS_PCI_IO => {
                self.ice.write32(a - ip32::PHYS_BASE_ICE, value)
            }
            // PCI low I/O window (0x18000000).
            a if a < ip32::PHYS_PCI_IO + ip32::PHYS_SIZE_PCI_IO => {
                self.mace.pci_window_write32(a, value)
            }
            a if a < ip32::PHYS_PCI_MEM => {
                log::warn!("write32: unassigned physical hole at {:#010X}", a);
            }
            // PCI low memory window (0x1A000000..0x1BFFFFFF).
            a if a < ip32::PHYS_PCI_MEM + ip32::PHYS_SIZE_PCI_MEM => {
                self.mace.pci_window_write32(a, value)
            }
            a if a < ip32::PHYS_BASE_MACE => {
                log::warn!("write32: unassigned physical hole at {:#010X}", a);
            }
            a if a < ip32::PHYS_SYSTEM_ROM => {
                self.mace.write32(a - ip32::PHYS_BASE_MACE, value)
            }
            a => self.rom.write32(a - ip32::PHYS_SYSTEM_ROM, value),
        }
    }

    /// Read a 16-bit halfword from the physical address space.
    pub fn read16(&mut self, addr: u32) -> u16 {
        match addr {
            a if a < ip32::PHYS_BASE_CRIME => self.ram.read16(a),
            a if a < ip32::PHYS_BASE_RENDER => self.crime_cpu.read16(a - ip32::PHYS_BASE_CRIME),
            a if a < ip32::PHYS_BASE_GBE => self.render_engine.read16(a - ip32::PHYS_BASE_RENDER),
            a if a < ip32::PHYS_BASE_ICE => self.gbe.read16(a - ip32::PHYS_BASE_GBE),
            a if a < ip32::PHYS_PCI_IO => self.ice.read16(a - ip32::PHYS_BASE_ICE),
            a if a < ip32::PHYS_PCI_IO + ip32::PHYS_SIZE_PCI_IO => self.mace.pci_window_read16(a),
            a if a < ip32::PHYS_PCI_MEM => 0xFFFF,
            a if a < ip32::PHYS_PCI_MEM + ip32::PHYS_SIZE_PCI_MEM => {
                self.mace.pci_window_read16(a)
            }
            a if a < ip32::PHYS_BASE_MACE => 0xFFFF,
            a if a < ip32::PHYS_SYSTEM_ROM => self.mace.read16(a - ip32::PHYS_BASE_MACE),
            a => self.rom.read16(a - ip32::PHYS_SYSTEM_ROM),
        }
    }

    /// Write a 16-bit halfword to the physical address space.
    pub fn write16(&mut self, addr: u32, value: u16) {
        match addr {
            a if a < ip32::PHYS_BASE_CRIME => self.ram.write16(a, value),
            a if a < ip32::PHYS_BASE_RENDER => {
                self.crime_cpu.write16(a - ip32::PHYS_BASE_CRIME, value)
            }
            a if a < ip32::PHYS_BASE_GBE => {
                self.render_engine.write16(a - ip32::PHYS_BASE_RENDER, value)
            }
            a if a < ip32::PHYS_BASE_ICE => {
                self.gbe.write16(a - ip32::PHYS_BASE_GBE, value)
            }
            a if a < ip32::PHYS_PCI_IO => {
                self.ice.write16(a - ip32::PHYS_BASE_ICE, value)
            }
            a if a < ip32::PHYS_PCI_IO + ip32::PHYS_SIZE_PCI_IO => {
                self.mace.pci_window_write16(a, value)
            }
            a if a < ip32::PHYS_PCI_MEM => {}
            a if a < ip32::PHYS_PCI_MEM + ip32::PHYS_SIZE_PCI_MEM => {
                self.mace.pci_window_write16(a, value)
            }
            a if a < ip32::PHYS_BASE_MACE => {}
            a if a < ip32::PHYS_SYSTEM_ROM => {
                self.mace.write16(a - ip32::PHYS_BASE_MACE, value)
            }
            a => self.rom.write16(a - ip32::PHYS_SYSTEM_ROM, value),
        }
    }

    /// Read a single byte from the physical address space.
    pub fn read8(&mut self, addr: u32) -> u8 {
        match addr {
            a if a < ip32::PHYS_BASE_CRIME => self.ram.read8(a),
            a if a < ip32::PHYS_BASE_RENDER => self.crime_cpu.read8(a - ip32::PHYS_BASE_CRIME),
            a if a < ip32::PHYS_BASE_GBE => self.render_engine.read8(a - ip32::PHYS_BASE_RENDER),
            a if a < ip32::PHYS_BASE_ICE => self.gbe.read8(a - ip32::PHYS_BASE_GBE),
            a if a < ip32::PHYS_PCI_IO => self.ice.read8(a - ip32::PHYS_BASE_ICE),
            a if a < ip32::PHYS_PCI_IO + ip32::PHYS_SIZE_PCI_IO => self.mace.pci_window_read8(a),
            a if a < ip32::PHYS_PCI_MEM => 0xFF,
            a if a < ip32::PHYS_PCI_MEM + ip32::PHYS_SIZE_PCI_MEM => self.mace.pci_window_read8(a),
            a if a < ip32::PHYS_BASE_MACE => 0xFF,
            a if a < ip32::PHYS_SYSTEM_ROM => self.mace.read8(a - ip32::PHYS_BASE_MACE),
            a => self.rom.read8(a - ip32::PHYS_SYSTEM_ROM),
        }
    }

    /// Write a single byte to the physical address space.
    pub fn write8(&mut self, addr: u32, value: u8) {
        match addr {
            a if a < ip32::PHYS_BASE_CRIME => self.ram.write8(a, value),
            a if a < ip32::PHYS_BASE_RENDER => {
                self.crime_cpu.write8(a - ip32::PHYS_BASE_CRIME, value)
            }
            a if a < ip32::PHYS_BASE_GBE => {
                self.render_engine.write8(a - ip32::PHYS_BASE_RENDER, value)
            }
            a if a < ip32::PHYS_BASE_ICE => {
                self.gbe.write8(a - ip32::PHYS_BASE_GBE, value)
            }
            a if a < ip32::PHYS_PCI_IO => {
                self.ice.write8(a - ip32::PHYS_BASE_ICE, value)
            }
            a if a < ip32::PHYS_PCI_IO + ip32::PHYS_SIZE_PCI_IO => {
                self.mace.pci_window_write8(a, value)
            }
            a if a < ip32::PHYS_PCI_MEM => {}
            a if a < ip32::PHYS_PCI_MEM + ip32::PHYS_SIZE_PCI_MEM => {
                self.mace.pci_window_write8(a, value)
            }
            a if a < ip32::PHYS_BASE_MACE => {}
            a if a < ip32::PHYS_SYSTEM_ROM => {
                self.mace.write8(a - ip32::PHYS_BASE_MACE, value)
            }
            a => self.rom.write8(a - ip32::PHYS_SYSTEM_ROM, value),
        }
    }

    /// Read a 64-bit doubleword from the physical address space (big-endian).
    pub fn read64(&mut self, addr: u32) -> u64 {
        match addr {
            a if a < ip32::PHYS_BASE_CRIME => self.ram.read64(a),
            a if a < ip32::PHYS_BASE_RENDER => self.crime_cpu.read64(a - ip32::PHYS_BASE_CRIME),
            a if a < ip32::PHYS_BASE_GBE => self.render_engine.read64(a - ip32::PHYS_BASE_RENDER),
            a if a < ip32::PHYS_BASE_ICE => self.gbe.read64(a - ip32::PHYS_BASE_GBE),
            a if a < ip32::PHYS_PCI_IO => self.ice.read64(a - ip32::PHYS_BASE_ICE),
            a if a < ip32::PHYS_PCI_IO + ip32::PHYS_SIZE_PCI_IO => {
                ((self.mace.pci_window_read32(a) as u64) << 32)
                    | self.mace.pci_window_read32(a + 4) as u64
            }
            a if a < ip32::PHYS_PCI_MEM => 0xFFFF_FFFF_FFFF_FFFF,
            a if a < ip32::PHYS_PCI_MEM + ip32::PHYS_SIZE_PCI_MEM => {
                ((self.mace.pci_window_read32(a) as u64) << 32)
                    | self.mace.pci_window_read32(a + 4) as u64
            }
            a if a < ip32::PHYS_BASE_MACE => 0xFFFF_FFFF_FFFF_FFFF,
            a if a < ip32::PHYS_SYSTEM_ROM => self.mace.read64(a - ip32::PHYS_BASE_MACE),
            a => self.rom.read64(a - ip32::PHYS_SYSTEM_ROM),
        }
    }

    /// Write a 64-bit doubleword to the physical address space (big-endian).
    pub fn write64(&mut self, addr: u32, value: u64) {
        match addr {
            a if a < ip32::PHYS_BASE_CRIME => self.ram.write64(a, value),
            a if a < ip32::PHYS_BASE_RENDER => {
                self.crime_cpu.write64(a - ip32::PHYS_BASE_CRIME, value)
            }
            a if a < ip32::PHYS_BASE_GBE => {
                let off = a - ip32::PHYS_BASE_RENDER;
                self.render_engine.write64(off, value);
                if off == crate::graphics::RenderEngine::GO_OFFSET {
                    self.render_engine.rasterize(&self.gbe, &mut self.ram);
                }
            }
            a if a < ip32::PHYS_BASE_ICE => {
                self.gbe.write64(a - ip32::PHYS_BASE_GBE, value)
            }
            a if a < ip32::PHYS_PCI_IO => {
                self.ice.write64(a - ip32::PHYS_BASE_ICE, value)
            }
            a if a < ip32::PHYS_PCI_IO + ip32::PHYS_SIZE_PCI_IO => {
                self.mace.pci_window_write32(a, (value >> 32) as u32);
                self.mace.pci_window_write32(a + 4, value as u32);
            }
            a if a < ip32::PHYS_PCI_MEM => {}
            a if a < ip32::PHYS_PCI_MEM + ip32::PHYS_SIZE_PCI_MEM => {
                self.mace.pci_window_write32(a, (value >> 32) as u32);
                self.mace.pci_window_write32(a + 4, value as u32);
            }
            a if a < ip32::PHYS_BASE_MACE => {}
            a if a < ip32::PHYS_SYSTEM_ROM => {
                self.mace.write64(a - ip32::PHYS_BASE_MACE, value)
            }
            a => self.rom.write64(a - ip32::PHYS_SYSTEM_ROM, value),
        }
    }

    /// Advance the MACE UST (microsecond timer) by `delta` CRIME ticks.
    ///
    /// The PROM treats the UST as a free-running 960 ns counter and spins on
    /// it for timing (post1's wait loop reads `ld,0xbf340000` until the
    /// counter passes a deadline). See [`UstMscState::advance`].
    pub fn mace_advance_ust(&mut self, delta: u64) {
        self.mace.perif.ustmsc.advance(delta);
    }

    /// Refresh CRIME's interrupt status from the I/O side and expose the
    /// pending (unmasked) interrupt set.
    ///
    /// MACE PCI routes the two AIC-7880 INT# outputs to CRIME lines 8/9;
    /// this samples them so CRIME's `istat` always reflects the gadget
    /// states. Returns `istat & imask`, i.e. what CRIME would present to the
    /// CPU on IP0.
    pub fn sync_pci_interrupts(&mut self) -> u32 {
        self.mace.update_pci_interrupts(&mut self.crime_cpu);
        self.crime_cpu.interrupt_status()
    }

    /// Advance the MACE audio DMA by `delta` CRIME ticks (nominally one ~133
    /// MHz VCLK tick per CPU cycle).
    ///
    /// The codec consumes one stereo frame per `sample_rate` emulated seconds;
    /// this drains the guest DMA rings from main memory into the host output
    /// ring so the PROM's `play_hello_tune` loop observes space opening up and
    /// never deadlocks on a full ring.
    pub fn audio_tick(&mut self, delta: u64) {
        let mut frames = self.mace.perif.audio.advance_time(delta);
        if frames == 0 {
            return;
        }
        while frames > 0 {
            let mut consumed = false;
            for idx in 0..3 {
                if !self.mace.perif.audio.channel_enabled(idx) {
                    continue;
                }
                if let Some(addr) = self.mace.perif.audio.next_frame_addr(idx) {
                    let entry = self.read64(addr);
                    self.mace.perif.audio.consume_frame(idx, entry);
                    frames -= 1;
                    consumed = true;
                    break;
                }
            }
            if !consumed {
                // No channel has data; the remainder of the window is silence
                // (codec underrun at ring-empty).
                break;
            }
        }
    }
}