// src/memory/mod.rs
//! Memory subsystem: address space abstraction and physical memory.
//!
//! The O2 uses a Unified Memory Architecture (UMA) — CPU, graphics, and I/O
//! all share main memory. This module provides:
//!
//! - [`PhysicalMemory`] — the main RAM backing store.
//! - [`AddressSpace`] — a trait for addressable regions (RAM, ROM, MMIO).
//! - [`MemoryMap`] — routes physical addresses to the right region.

pub mod physical;

use crate::io::Mace;
use crate::ip32;

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
    /// CRIME CPU interface (MMIO).
    pub crime: physical::PhysicalMemory,
    /// MACE I/O engine (MMIO).
    pub mace: Mace,
    /// GBE display engine (MMIO).
    pub gbe: physical::PhysicalMemory,
}

impl MemoryMap {
    /// Create a new memory map with `ram_mb` megabytes of RAM.
    pub fn new(ram_mb: u32) -> Self {
        let ram_size = ram_mb.min(ip32::MAX_MEMORY / (1024 * 1024)) * 1024 * 1024;
        Self {
            ram: physical::PhysicalMemory::new(ram_size as usize),
            rom: physical::PhysicalMemory::new(ip32::SYSTEM_ROM_WINDOW_SIZE as usize),
            crime: physical::PhysicalMemory::new(0x1000),
            mace: Mace::new(),
            gbe: physical::PhysicalMemory::new(0x1000),
        }
    }

    /// Read a 32-bit word from the physical address space.
    pub fn read32(&mut self, addr: u32) -> u32 {
        match addr {
            a if a < ip32::PHYS_BASE_CRIME => self.ram.read32(a),
            a if a < ip32::PHYS_BASE_RENDER => self.crime.read32(a - ip32::PHYS_BASE_CRIME),
            a if a < ip32::PHYS_BASE_GBE => 0, // Render engine (unimplemented)
            a if a < ip32::PHYS_BASE_MACE => self.gbe.read32(a - ip32::PHYS_BASE_GBE),
            a if a < ip32::PHYS_SYSTEM_ROM => self.mace.read32(a - ip32::PHYS_BASE_MACE),
            a => self.rom.read32(a - ip32::PHYS_SYSTEM_ROM),
        }
    }

    /// Write a 32-bit word to the physical address space.
    pub fn write32(&mut self, addr: u32, value: u32) {
        match addr {
            a if a < ip32::PHYS_BASE_CRIME => self.ram.write32(a, value),
            a if a < ip32::PHYS_BASE_RENDER => {
                self.crime.write32(a - ip32::PHYS_BASE_CRIME, value)
            }
            a if a < ip32::PHYS_BASE_GBE => {} // Render engine (unimplemented)
            a if a < ip32::PHYS_BASE_MACE => self.gbe.write32(a - ip32::PHYS_BASE_GBE, value),
            a if a < ip32::PHYS_SYSTEM_ROM => self.mace.write32(a - ip32::PHYS_BASE_MACE, value),
            a => self.rom.write32(a - ip32::PHYS_SYSTEM_ROM, value),
        }
    }

    /// Read a 16-bit halfword from the physical address space.
    pub fn read16(&mut self, addr: u32) -> u16 {
        match addr {
            a if a < ip32::PHYS_BASE_CRIME => self.ram.read16(a),
            a if a < ip32::PHYS_BASE_RENDER => self.crime.read16(a - ip32::PHYS_BASE_CRIME),
            a if a < ip32::PHYS_BASE_GBE => 0,
            a if a < ip32::PHYS_BASE_MACE => self.gbe.read16(a - ip32::PHYS_BASE_GBE),
            a if a < ip32::PHYS_SYSTEM_ROM => self.mace.read16(a - ip32::PHYS_BASE_MACE),
            a => self.rom.read16(a - ip32::PHYS_SYSTEM_ROM),
        }
    }

    /// Write a 16-bit halfword to the physical address space.
    pub fn write16(&mut self, addr: u32, value: u16) {
        match addr {
            a if a < ip32::PHYS_BASE_CRIME => self.ram.write16(a, value),
            a if a < ip32::PHYS_BASE_RENDER => {
                self.crime.write16(a - ip32::PHYS_BASE_CRIME, value)
            }
            a if a < ip32::PHYS_BASE_GBE => {}
            a if a < ip32::PHYS_BASE_MACE => self.gbe.write16(a - ip32::PHYS_BASE_GBE, value),
            a if a < ip32::PHYS_SYSTEM_ROM => self.mace.write16(a - ip32::PHYS_BASE_MACE, value),
            a => self.rom.write16(a - ip32::PHYS_SYSTEM_ROM, value),
        }
    }

    /// Read a single byte from the physical address space.
    pub fn read8(&mut self, addr: u32) -> u8 {
        match addr {
            a if a < ip32::PHYS_BASE_CRIME => self.ram.read8(a),
            a if a < ip32::PHYS_BASE_RENDER => self.crime.read8(a - ip32::PHYS_BASE_CRIME),
            a if a < ip32::PHYS_BASE_GBE => 0,
            a if a < ip32::PHYS_BASE_MACE => self.gbe.read8(a - ip32::PHYS_BASE_GBE),
            a if a < ip32::PHYS_SYSTEM_ROM => self.mace.read8(a - ip32::PHYS_BASE_MACE),
            a => self.rom.read8(a - ip32::PHYS_SYSTEM_ROM),
        }
    }

    /// Write a single byte to the physical address space.
    pub fn write8(&mut self, addr: u32, value: u8) {
        match addr {
            a if a < ip32::PHYS_BASE_CRIME => self.ram.write8(a, value),
            a if a < ip32::PHYS_BASE_RENDER => {
                self.crime.write8(a - ip32::PHYS_BASE_CRIME, value)
            }
            a if a < ip32::PHYS_BASE_GBE => {}
            a if a < ip32::PHYS_BASE_MACE => self.gbe.write8(a - ip32::PHYS_BASE_GBE, value),
            a if a < ip32::PHYS_SYSTEM_ROM => self.mace.write8(a - ip32::PHYS_BASE_MACE, value),
            a => self.rom.write8(a - ip32::PHYS_SYSTEM_ROM, value),
        }
    }

    /// Read a 64-bit doubleword from the physical address space (big-endian).
    pub fn read64(&mut self, addr: u32) -> u64 {
        match addr {
            a if a < ip32::PHYS_BASE_CRIME => self.ram.read64(a),
            a if a < ip32::PHYS_BASE_RENDER => self.crime.read64(a - ip32::PHYS_BASE_CRIME),
            a if a < ip32::PHYS_BASE_GBE => 0,
            a if a < ip32::PHYS_BASE_MACE => self.gbe.read64(a - ip32::PHYS_BASE_GBE),
            a if a < ip32::PHYS_SYSTEM_ROM => self.mace.read64(a - ip32::PHYS_BASE_MACE),
            a => self.rom.read64(a - ip32::PHYS_SYSTEM_ROM),
        }
    }

    /// Write a 64-bit doubleword to the physical address space (big-endian).
    pub fn write64(&mut self, addr: u32, value: u64) {
        match addr {
            a if a < ip32::PHYS_BASE_CRIME => self.ram.write64(a, value),
            a if a < ip32::PHYS_BASE_RENDER => {
                self.crime.write64(a - ip32::PHYS_BASE_CRIME, value)
            }
            a if a < ip32::PHYS_BASE_GBE => {}
            a if a < ip32::PHYS_BASE_MACE => self.gbe.write64(a - ip32::PHYS_BASE_GBE, value),
            a if a < ip32::PHYS_SYSTEM_ROM => self.mace.write64(a - ip32::PHYS_BASE_MACE, value),
            a => self.rom.write64(a - ip32::PHYS_SYSTEM_ROM, value),
        }
    }
}