//! Physical memory backing store.
//!
//! A simple byte-addressable RAM region with big-endian word access, matching
//! the MIPS big-endian byte order of the O2.

use super::AddressSpace;

/// A contiguous block of byte-addressable memory.
pub struct PhysicalMemory {
    data: Vec<u8>,
}

impl PhysicalMemory {
    /// Create a new zero-initialized memory region of `size` bytes.
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
        }
    }

    /// The size of this region in bytes.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Whether this region is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Load a byte slice into memory at `offset`.
    pub fn load(&mut self, offset: usize, bytes: &[u8]) {
        let end = offset + bytes.len();
        if end > self.data.len() {
            // Truncate to fit.
            let n = self.data.len() - offset;
            self.data[offset..].copy_from_slice(&bytes[..n]);
        } else {
            self.data[offset..end].copy_from_slice(bytes);
        }
    }

    /// Get a raw byte slice view of the memory.
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    /// Get a mutable raw byte slice view of the memory.
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data
    }
}

impl AddressSpace for PhysicalMemory {
    fn read8(&mut self, addr: u32) -> u8 {
        self.data
            .get(addr as usize)
            .copied()
            .unwrap_or(0)
    }

    fn read16(&mut self, addr: u32) -> u16 {
        let a = addr as usize;
        let hi = self.data.get(a).copied().unwrap_or(0) as u16;
        let lo = self.data.get(a + 1).copied().unwrap_or(0) as u16;
        (hi << 8) | lo
    }

    fn read32(&mut self, addr: u32) -> u32 {
        let a = addr as usize;
        let b0 = self.data.get(a).copied().unwrap_or(0) as u32;
        let b1 = self.data.get(a + 1).copied().unwrap_or(0) as u32;
        let b2 = self.data.get(a + 2).copied().unwrap_or(0) as u32;
        let b3 = self.data.get(a + 3).copied().unwrap_or(0) as u32;
        (b0 << 24) | (b1 << 16) | (b2 << 8) | b3
    }

    fn write8(&mut self, addr: u32, value: u8) {
        if let Some(byte) = self.data.get_mut(addr as usize) {
            *byte = value;
        }
    }

    fn write16(&mut self, addr: u32, value: u16) {
        let a = addr as usize;
        if let Some(byte) = self.data.get_mut(a) {
            *byte = (value >> 8) as u8;
        }
        if let Some(byte) = self.data.get_mut(a + 1) {
            *byte = value as u8;
        }
    }

    fn write32(&mut self, addr: u32, value: u32) {
        let a = addr as usize;
        if let Some(byte) = self.data.get_mut(a) {
            *byte = (value >> 24) as u8;
        }
        if let Some(byte) = self.data.get_mut(a + 1) {
            *byte = (value >> 16) as u8;
        }
        if let Some(byte) = self.data.get_mut(a + 2) {
            *byte = (value >> 8) as u8;
        }
        if let Some(byte) = self.data.get_mut(a + 3) {
            *byte = value as u8;
        }
    }

    fn contains(&self, addr: u32) -> bool {
        (addr as usize) < self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn big_endian_word_access() {
        let mut mem = PhysicalMemory::new(16);
        mem.write32(0, 0x1234_5678);
        assert_eq!(mem.read32(0), 0x1234_5678);
        assert_eq!(mem.read8(0), 0x12);
        assert_eq!(mem.read8(1), 0x34);
        assert_eq!(mem.read8(2), 0x56);
        assert_eq!(mem.read8(3), 0x78);
    }

    #[test]
    fn out_of_bounds_reads_zero() {
        let mut mem = PhysicalMemory::new(4);
        assert_eq!(mem.read32(0x1000), 0);
        mem.write32(0x1000, 0xdead_beef); // no-op
        assert_eq!(mem.read32(0x1000), 0);
    }
}