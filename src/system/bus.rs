//! The system bus — bridges the CPU to the memory map.
//!
//! Implements the [`MemoryAccess`] trait required by the CPU core, routing
//! reads/writes through the [`MemoryMap`].

use crate::cpu::r5000::MemoryAccess;
use crate::memory::MemoryMap;

/// A thin adapter that lets the CPU access the memory map.
pub struct SystemBus<'a> {
    /// The memory map to route accesses through.
    pub memory: &'a mut MemoryMap,
}

impl MemoryAccess for SystemBus<'_> {
    fn read32(&mut self, addr: u32) -> u32 {
        self.memory.read32(addr)
    }

    fn read16(&mut self, addr: u32) -> u16 {
        self.memory.read16(addr)
    }

    fn read8(&mut self, addr: u32) -> u8 {
        self.memory.read8(addr)
    }

    fn write32(&mut self, addr: u32, value: u32) {
        self.memory.write32(addr, value);
    }

    fn write16(&mut self, addr: u32, value: u16) {
        self.memory.write16(addr, value);
    }

    fn write8(&mut self, addr: u32, value: u8) {
        self.memory.write8(addr, value);
    }
}