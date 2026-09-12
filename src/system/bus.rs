//! The system bus — bridges the CPU to the memory map.
//!
//! Implements the [`MemoryAccess`] trait required by the CPU core, routing
//! reads/writes through the [`MemoryMap`] and the write-back primary data
//! cache.
//!
//! ## Address translation
//!
//! The CPU executes against *virtual* MIPS segments: `kuseg` (`0x0–0x7fff_ffff`),
//! `kseg0` (`0x8000_0000–0x9fff_ffff`, cached) and `kseg1`
//! (`0xa000_0000–0xbfff_ffff`, uncached). The memory map is indexed by
//! *physical* address, so every access is translated here:
//!
//! - `kuseg`  → physical address unchanged.
//! - `kseg0/kseg1` → `addr & 0x1fff_ffff`.
//! - `kseg2/kseg3` → TLB-mapped; not yet implemented, passed through.
//!
//! Data accesses through **kseg0 that land in RAM** are cacheable: they go
//! through the write-back [`DCache`]. Everything else (kseg1, devices,
//! kuseg) bypasses the cache. Instruction fetches ([`MemoryAccess::fetch32`])
//! always read memory directly — the instruction stream is uncached.
//!
//! This matches the MIPS III/IV unmap-on-segment rule and the KSEG1 device
//! views the firmware uses: PROM `0xbfc0_0000`, CRIME `0xb400_0000`, MACE
//! `0xbf00_0000`.

use crate::cpu::r5000::MemoryAccess;
use crate::ip32;
use crate::memory::{DCache, MemoryMap};

/// Translate a MIPS virtual address to a physical address and report whether
/// the access is cacheable (kseg0 mapping to RAM).
fn translate(addr: u32) -> (u32, bool) {
    let high = addr & 0xe000_0000;
    // kseg0 = 0x8000_0000..0x9fff_ffff, kseg1 = 0xa000_0000..0xbfff_ffff
    // (bits 31:29 == 4, 5) alias physical 0x0000_0000..0x1fff_ffff.
    if high == 0x8000_0000 || high == 0xa000_0000 {
        let phys = addr & 0x1fff_ffff;
        // Only RAM is cacheable; device regions are never cached.
        (phys, high == 0x8000_0000 && phys < ip32::PHYS_BASE_CRIME)
    } else {
        // kuseg (< 0x8000_0000) passes through; kseg2/kseg3 (>= 0xc000_0000)
        // are TLB-mapped and left for a future MMU.
        (addr, false)
    }
}

/// A thin adapter that lets the CPU access the memory map.
pub struct SystemBus<'a> {
    /// The memory map to route accesses through.
    pub memory: &'a mut MemoryMap,
    /// The CPU's write-back primary data cache.
    pub cache: &'a mut DCache,
}

impl MemoryAccess for SystemBus<'_> {
    fn read32(&mut self, addr: u32) -> u32 {
        let (phys, cached) = translate(addr);
        if cached {
            self.cache.read(self.memory, phys, 4) as u32
        } else {
            self.memory.read32(phys)
        }
    }

    fn read16(&mut self, addr: u32) -> u16 {
        let (phys, cached) = translate(addr);
        if cached {
            self.cache.read(self.memory, phys, 2) as u16
        } else {
            self.memory.read16(phys)
        }
    }

    fn read8(&mut self, addr: u32) -> u8 {
        let (phys, cached) = translate(addr);
        if cached {
            self.cache.read(self.memory, phys, 1) as u8
        } else {
            self.memory.read8(phys)
        }
    }

    fn write32(&mut self, addr: u32, value: u32) {
        let (phys, cached) = translate(addr);
        if cached {
            self.cache.write(self.memory, phys, 4, u64::from(value));
        } else {
            self.memory.write32(phys, value);
        }
    }

    fn write16(&mut self, addr: u32, value: u16) {
        let (phys, cached) = translate(addr);
        if cached {
            self.cache.write(self.memory, phys, 2, u64::from(value));
        } else {
            self.memory.write16(phys, value);
        }
    }

    fn write8(&mut self, addr: u32, value: u8) {
        let (phys, cached) = translate(addr);
        if cached {
            self.cache.write(self.memory, phys, 1, u64::from(value));
        } else {
            self.memory.write8(phys, value);
        }
    }

    fn read64(&mut self, addr: u32) -> u64 {
        let (phys, cached) = translate(addr);
        if cached {
            self.cache.read(self.memory, phys, 8)
        } else {
            self.memory.read64(phys)
        }
    }

    fn write64(&mut self, addr: u32, value: u64) {
        let (phys, cached) = translate(addr);
        if cached {
            self.cache.write(self.memory, phys, 8, value);
        } else {
            self.memory.write64(phys, value);
        }
    }

    fn fetch32(&mut self, addr: u32) -> u32 {
        // Instruction fetch: never routed through the data cache.
        self.memory.read32(translate(addr).0)
    }

    fn cache_instruction(&mut self, op_field: u32, addr: u32) {
        self.cache
            .cache_instruction(self.memory, op_field, translate(addr).0);
    }
}