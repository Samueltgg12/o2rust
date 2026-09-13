// src/memory/cache.rs
//! Write-back primary data cache model.
//!
//! The O2 PROM relies on the R5000/R10000 primary data cache being a
//! **write-back** cache: execution of `simple_memtst` (uncached kseg1 writes)
//! intentionally overwrites the physical stack region, and `DupSLStack`'s
//! `Copy2MEM` loop then re-writes the stack into RAM by reading the *cached*
//! (kseg0) copy of the pristine bytes.
//!
//! The model matches the R5000 geometry — 32 KiB, 2-way set-associative,
//! 32-byte lines, write-allocate on store, write-back on eviction — with a
//! monotonic LRU stamp per line. The `cache` instruction's index-type
//! operations (`CACH_*|C_IINV`, `CACH_*|C_IST`) invalidate the addressed
//! index **without writeback**, matching how the PROM's whole-cache
//! initialization loop (`IP32processorTCI`) uses them to *discard*
//! potentially ECC-poisoned dirty data after uncached memory tests.
//! Hit-type operations (`C_HWB`, `C_HWBINV`, `C_HINV`, `C_IWBINV`) act on
//! the exact addressed line. The R5000's "faked" secondary cache aliases to
//! the primary D cache (`CACH_SD` selects the same lines).
//!
//! Only physical addresses whose backing store is RAM are ever placed in this
//! cache — the system bus decides caching per access, so uncached kseg1
//! accesses and device MMIO always bypass it.

use crate::memory::MemoryMap;

/// One 32-byte cache line.
#[derive(Clone, Copy)]
struct Line {
    valid: bool,
    dirty: bool,
    /// Full physical address of the line (low 5 bits clear).
    tag: u32,
    /// LRU clock stamp, bumped on every access.
    used: u64,
    /// Line data, big-endian byte order.
    data: [u8; 32],
}

impl Default for Line {
    fn default() -> Self {
        Self {
            valid: false,
            dirty: false,
            tag: 0,
            used: 0,
            data: [0; 32],
        }
    }
}

/// The primary data cache.
///
/// `sets` sets × 2 ways, 32-byte lines; way index is `set + SETS`.
#[derive(Clone)]
pub struct DCache {
    lines: Vec<Line>,
    clock: u64,
}

const SETS: usize = 512; // 32 KiB / 2 ways / 32-byte lines
const WAYS: usize = 2;

impl Default for DCache {
    fn default() -> Self {
        Self::new()
    }
}

impl DCache {
    /// Create a cold (all-invalid) primary data cache.
    pub fn new() -> Self {
        Self {
            lines: vec![Line::default(); SETS * WAYS],
            clock: 0,
        }
    }

    fn set_of(tag: u32) -> usize {
        ((tag >> 5) & 0x1ff) as usize
    }

    fn line_base(tag: u32) -> u32 {
        tag & !0x1f
    }

    /// Look up `phys` in its set, returning the flat line index (if resident).
    #[inline]
    fn lookup(&mut self, phys: u32) -> Option<usize> {
        let set = Self::set_of(phys);
        let tag = Self::line_base(phys);
        for w in 0..WAYS {
            let idx = set + w * SETS;
            let line = &self.lines[idx];
            if line.valid && line.tag == tag {
                self.lines[idx].used = self.clock;
                self.clock = self.clock.wrapping_add(1);
                return Some(idx);
            }
        }
        None
    }

    /// Ensure the line containing `phys` is resident, returning its index.
    fn ensure(&mut self, mem: &mut MemoryMap, phys: u32) -> usize {
        if let Some(idx) = self.lookup(phys) {
            return idx;
        }
        let set = Self::set_of(phys);
        let tag = Self::line_base(phys);

        // Pick a victim: prefer an invalid way, else the least-recently-used.
        let base = set;
        let alt = set + SETS;
        let victim = {
            let (a, b) = (&self.lines[base], &self.lines[alt]);
            match (a.valid, b.valid) {
                (false, _) => base,
                (_, false) => alt,
                _ if a.used <= b.used => base,
                _ => alt,
            }
        };

        // Write back a dirty victim before refilling.
        let v = &mut self.lines[victim];
        if v.valid && v.dirty {
            let base_addr = v.tag;
            for i in 0..32 {
                mem.write8(base_addr + i as u32, v.data[i]);
            }
        }

        // Refill the line from memory.
        let line = &mut self.lines[victim];
        line.valid = true;
        line.dirty = false;
        line.tag = tag;
        line.used = self.clock;
        self.clock = self.clock.wrapping_add(1);
        for i in 0..32 {
            line.data[i] = mem.read8(tag + i as u32);
        }
        victim
    }

    /// Read a big-endian value of `size` bytes (`1`, `2`, `4`, or `8`).
    pub fn read(&mut self, mem: &mut MemoryMap, phys: u32, size: u32) -> u64 {
        let mut v: u64 = 0;
        for i in 0..size {
            let idx = self.ensure(mem, phys.wrapping_add(i as u32));
            let byte = self.lines[idx].data[((phys + i) & 0x1f) as usize];
            v = (v << 8) | u64::from(byte);
        }
        v
    }

    /// Write a big-endian value of `size` bytes (`1`, `2`, `4`, or `8`).
    pub fn write(&mut self, mem: &mut MemoryMap, phys: u32, size: u32, value: u64) {
        for i in 0..size {
            let shift = (size - 1 - i) * 8;
            let byte = ((value >> shift) & 0xff) as u8;
            let idx = self.ensure(mem, phys.wrapping_add(i as u32));
            let line = &mut self.lines[idx];
            line.data[((phys + i) & 0x1f) as usize] = byte;
            line.dirty = true;
        }
    }

    fn writeback_line(&mut self, mem: &mut MemoryMap, idx: usize) {
        let line = &mut self.lines[idx];
        if !line.valid || !line.dirty {
            return;
        }
        let base_addr = line.tag;
        for i in 0..32 {
            mem.write8(base_addr + i as u32, line.data[i]);
        }
        line.dirty = false;
    }

    /// Index Invalidate (I-cache op 0) / Index Store Tag (op 2): the PROM's
    /// `IP32processorTCI` sweeps the whole cache with these, dropping dirty
    /// lines **without writeback** (C_IST sets the tag state to Invalid).
    /// This matters for correctness, not just accuracy: post1's
    /// `tlb_init_preserve` stores `ra` to its kseg1 stack *before* the
    /// sweep runs; writing back the stale dirty line left behind by the
    /// earlier kseg0 (cached) stack frame would clobber that fresh store
    /// and make the function return to the wrong address (observed:
    /// `jr ra` back into the middle of post1, tripping the post1 stack
    /// sanity check and falling back to the serial loader).
    fn index_invalidate(&mut self, phys: u32) {
        let set = Self::set_of(phys);
        for w in 0..WAYS {
            let idx = set + w * SETS;
            self.lines[idx].valid = false;
            self.lines[idx].dirty = false;
        }
    }

    /// Index Writeback Invalidate (D/SD-cache op 0 on the R5000): write back
    /// dirty data, then invalidate. Used by the firmware's `r5k_cache_init`
    /// (`CACH_PD|C_IWBI` sweep) to flush the cache while preserving the data
    /// the firmware just stored through kseg0 (e.g. the cache-op function
    /// pointer table in BSS).
    fn index_writeback_invalidate(&mut self, mem: &mut MemoryMap, phys: u32) {
        let set = Self::set_of(phys);
        for w in 0..WAYS {
            let idx = set + w * SETS;
            self.writeback_line(mem, idx);
            self.lines[idx].valid = false;
            self.lines[idx].dirty = false;
        }
    }

    /// Hit-type operation on the exact line containing `phys`.
    fn hit_op(&mut self, mem: &mut MemoryMap, phys: u32, writeback: bool, invalidate: bool) {
        let set = Self::set_of(phys);
        let tag = Self::line_base(phys);
        for w in 0..WAYS {
            let idx = set + w * SETS;
            if !self.lines[idx].valid || self.lines[idx].tag != tag {
                continue;
            }
            if writeback {
                self.writeback_line(mem, idx);
            }
            if invalidate {
                self.lines[idx].valid = false;
                self.lines[idx].dirty = false;
            }
            return;
        }
    }

    /// Execute a `CACHE` instruction operation.
    ///
    /// `op_field` is the 5-bit `op` field from the instruction (bits 20:16);
    /// `phys` is the translated target address (hit operations only).
    pub fn cache_instruction(&mut self, mem: &mut MemoryMap, op_field: u32, phys: u32) {
        let target = op_field & 0x3; // 0=I, 1=D, 2=SI, 3=SD
        let op = (op_field >> 2) & 0x7;
        if target == 0 {
            return; // I-cache: not modeled (instruction fetches read memory)
        }
        // D and SD target the same primary data cache (R5000 "faked" L2).
        match op {
            // D-cache op 0 is Index Writeback Invalidate on the R5000
            // (flush dirty data, then drop); Index Store Tag (op 2) drops
            // dirty data WITHOUT writeback (the PROM's IP32processorTCI
            // sweep relies on the discard — see index_invalidate).
            0 => self.index_writeback_invalidate(mem, phys),
            2 => self.index_invalidate(phys),
            // Hit Invalidate (C_HINV) — data is written back conservatively.
            4 => self.hit_op(mem, phys, true, true),
            // Hit Writeback Invalidate (C_HWBINV) / Fill.
            5 => self.hit_op(mem, phys, true, true),
            // Hit Writeback (C_HWB).
            6 => self.hit_op(mem, phys, true, false),
            // Create Dirty Exclusive (3), Hit Set Virtual (7): no functional
            // effect required for the PROM.
            _ => {}
        }
    }

    /// Flush all dirty lines to memory (used by the host when observing RAM).
    pub fn flush_all(&mut self, mem: &mut MemoryMap) {
        for idx in 0..self.lines.len() {
            self.writeback_line(mem, idx);
        }
    }
}