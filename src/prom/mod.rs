// src/prom/mod.rs
//! IP32 PROM firmware loading.
//!
//! The PROM image is a raw MIPS firmware blob organized into **5 sections**
//! (SHDR layout), each with a section header. Section 4 contains an embedded
//! ELF header (version info), and section 5 stores the image checksum.
//!
//! See `docs/prom.md` for the full layout and `samples/decompiled-prom/` for
//! the decompiled reference.

use crate::ip32;
use thiserror::Error;

/// Errors that can occur while loading a PROM image.
#[derive(Debug, Error)]
pub enum PromError {
    /// The image is too small to contain a valid PROM.
    #[error("PROM image too small ({0} bytes)")]
    TooSmall(usize),
    /// The image checksum does not match (two's complement).
    #[error("PROM checksum mismatch: expected 0x{expected:08x}, got 0x{actual:08x}")]
    ChecksumMismatch { expected: u32, actual: u32 },
    /// An I/O error occurred while reading the image.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// A loaded PROM image.
pub struct Prom {
    /// The raw image bytes.
    data: Vec<u8>,
}

impl Prom {
    /// Load a PROM image from a byte slice.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, PromError> {
        if bytes.len() < 0x1000 {
            return Err(PromError::TooSmall(bytes.len()));
        }
        Ok(Self {
            data: bytes.to_vec(),
        })
    }

    /// Load a PROM image from a file on disk.
    pub fn from_file(path: &str) -> Result<Self, PromError> {
        let bytes = std::fs::read(path)?;
        Self::from_bytes(&bytes)
    }

    /// The raw image bytes.
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    /// The size of the image in bytes.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Whether the image is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Compute the two's complement checksum over the image.
    ///
    /// The checksum is defined such that the sum of all 32-bit words
    /// (including the stored checksum) equals zero in two's complement
    /// arithmetic.
    pub fn checksum(&self) -> u32 {
        let mut sum: u32 = 0;
        for chunk in self.data.chunks_exact(4) {
            let word = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            sum = sum.wrapping_add(word);
        }
        // Two's complement: the stored checksum makes the total sum zero.
        sum.wrapping_neg()
    }

    /// Verify the image checksum.
    ///
    /// Returns `Ok(())` if the sum of all words (including the stored
    /// checksum) is zero, or a [`PromError::ChecksumMismatch`] otherwise.
    pub fn verify_checksum(&self) -> Result<(), PromError> {
        let mut sum: u32 = 0;
        for chunk in self.data.chunks_exact(4) {
            let word = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            sum = sum.wrapping_add(word);
        }
        if sum == 0 {
            Ok(())
        } else {
            Err(PromError::ChecksumMismatch {
                expected: 0,
                actual: sum,
            })
        }
    }

    /// The reset vector where the CPU begins execution.
    pub fn reset_vector(&self) -> u32 {
        ip32::PROM_RESET_VECTOR
    }

    /// The firmware virtual memory address (where POST copies the firmware).
    pub fn firmware_vma(&self) -> u32 {
        ip32::PROM_VMA_BASE
    }

    /// Read a 32-bit word from the image at `offset` (big-endian).
    pub fn read32(&self, offset: usize) -> u32 {
        if offset + 4 > self.data.len() {
            return 0;
        }
        u32::from_be_bytes([
            self.data[offset],
            self.data[offset + 1],
            self.data[offset + 2],
            self.data[offset + 3],
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_tiny_images() {
        assert!(Prom::from_bytes(&[0u8; 16]).is_err());
    }

    #[test]
    fn loads_valid_image() {
        let mut bytes = vec![0u8; 0x2000];
        bytes[0] = 0x10; // some instruction bytes
        let prom = Prom::from_bytes(&bytes).unwrap();
        assert_eq!(prom.len(), 0x2000);
        assert_eq!(prom.reset_vector(), 0xBFC0_0000);
        assert_eq!(prom.firmware_vma(), 0x8100_0000);
    }

    #[test]
    fn checksum_is_two_complement() {
        // A single word 0x12345678: checksum = -0x12345678.
        // Need at least 0x1000 bytes for Prom::from_bytes
        let mut bytes = vec![0u8; 0x1000];
        bytes[0] = 0x12;
        bytes[1] = 0x34;
        bytes[2] = 0x56;
        bytes[3] = 0x78;
        let prom = Prom::from_bytes(&bytes).unwrap();
        assert_eq!(prom.checksum(), 0x1234_5678u32.wrapping_neg());
    }
}