//! SCSI bus emulation — target map for the AIC-7880 controller.
//!
//! The O2's UltraWide SCSI is provided by the Adaptec AIC-7880 on the MACE
//! PCI bus (`fixup-ip32.c` lists two aic7xxx devices: SCSI0 and SCSI1).
//! Standard SGI drive layout:
//!
//! - **target 1** — internal/first hard disk (drive sled)
//! - **target 6** — internal CD-ROM drive
//!
//! The AIC-7880 register/seleQ emulation itself is not yet implemented (see
//! `docs/register-maps.md`, `adp78.h`/`adp78.c`); this module provides the
//! target → [`BlockDevice`] map the emulated controller will serve once it
//! lands. The CLI mounts disk images here up front.

use crate::storage::BlockDevice;
use std::fmt;
use thiserror::Error;

/// Number of SCSI targets on a single bus.
pub const SCSI_TARGET_COUNT: usize = 16;

/// SCSI target ID of the (first) internal hard disk.
pub const SCSI_TARGET_DISK: u8 = 1;

/// SCSI target ID of the internal CD-ROM drive.
pub const SCSI_TARGET_CDROM: u8 = 6;

/// Errors while manipulating a SCSI target.
#[derive(Debug, Error)]
pub enum ScsiError {
    /// Target ID is out of the valid 0..15 range.
    #[error("invalid SCSI target id {0} (valid range 0..15)")]
    InvalidTarget(u8),
    /// A device is already attached at the target.
    #[error("SCSI target {0} already has a device attached")]
    TargetBusy(u8),
}

/// A single SCSI bus with up to 16 targets.
///
/// Targets are addressed by their ID (`0..15`). Per SGI convention the hard
/// disk lives at [`SCSI_TARGET_DISK`] and the CD-ROM at [`SCSI_TARGET_CDROM`].
#[derive(Default)]
pub struct ScsiBus {
    targets: [Option<Box<dyn BlockDevice>>; SCSI_TARGET_COUNT],
}

impl fmt::Debug for ScsiBus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let attached: Vec<u8> = self
            .targets
            .iter()
            .enumerate()
            .filter_map(|(i, t)| t.as_ref().map(|_| i as u8))
            .collect();
        f.debug_struct("ScsiBus")
            .field("attached_targets", &attached)
            .finish()
    }
}

impl ScsiBus {
    /// Create a new bus with nothing attached.
    pub fn new() -> Self {
        Self::default()
    }

    /// Attach a device at `target`. Fails if the target is busy or invalid.
    pub fn mount(&mut self, target: u8, device: Box<dyn BlockDevice>) -> Result<(), ScsiError> {
        let slot = self
            .targets
            .get_mut(target as usize)
            .ok_or(ScsiError::InvalidTarget(target))?;
        if slot.is_some() {
            return Err(ScsiError::TargetBusy(target));
        }
        *slot = Some(device);
        Ok(())
    }

    /// Detach whatever is at `target` (no-op for empty/invalid targets).
    pub fn unmount(&mut self, target: u8) {
        if (target as usize) < SCSI_TARGET_COUNT {
            self.targets[target as usize] = None;
        }
    }

    /// Whether a device is attached at `target`.
    pub fn is_attached(&self, target: u8) -> bool {
        (target as usize) < SCSI_TARGET_COUNT && self.targets[target as usize].is_some()
    }

    /// Mutable access to the device at `target`, if any.
    pub fn device(&mut self, target: u8) -> Option<&mut (dyn BlockDevice + '_)> {
        self.targets
            .get_mut(target as usize)?
            .as_deref_mut()
            .map(|d| d as &mut dyn BlockDevice)
    }

    /// Descriptive info for the device at `target`, if any.
    pub fn device_info(&self, target: u8) -> Option<DeviceInfo> {
        let dev = self.targets.get(target as usize)?.as_ref()?;
        Some(DeviceInfo {
            name: dev.name().to_string(),
            sector_size: dev.sector_size(),
            sector_count: dev.sector_count(),
            read_only: dev.is_read_only(),
        })
    }
}

/// Summary of an attached device (for the front-end status/log line).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceInfo {
    /// Image name.
    pub name: String,
    /// Bytes per sector.
    pub sector_size: u32,
    /// Total sectors.
    pub sector_count: u64,
    /// Whether writes are rejected.
    pub read_only: bool,
}

impl DeviceInfo {
    /// Total size in bytes.
    pub fn size_bytes(&self) -> u64 {
        self.sector_count * u64::from(self.sector_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::tests::MemDisk;
    use std::sync::{Arc, Mutex};

    #[test]
    fn mount_and_query_targets() {
        let mut bus = ScsiBus::new();
        assert!(!bus.is_attached(SCSI_TARGET_DISK));

        let dev = MemDisk {
            name: "test.img".into(),
            data: Arc::new(Mutex::new(vec![0u8; 512 * 8])),
            sector_size: 512,
        };
        bus.mount(SCSI_TARGET_DISK, Box::new(dev)).unwrap();
        assert!(bus.is_attached(SCSI_TARGET_DISK));

        let info = bus.device_info(SCSI_TARGET_DISK).unwrap();
        assert_eq!(info.name, "test.img");
        assert_eq!(info.sector_count, 8);
        assert_eq!(info.size_bytes(), 4096);

        // Can't double-mount.
        let dev2 = MemDisk {
            name: "x".into(),
            data: Arc::new(Mutex::new(vec![0u8; 512])),
            sector_size: 512,
        };
        assert!(bus.mount(SCSI_TARGET_DISK, Box::new(dev2)).is_err());

        // Out-of-range target ids are rejected at mount time.
        let dev3 = MemDisk {
            name: "y".into(),
            data: Arc::new(Mutex::new(vec![0u8; 512])),
            sector_size: 512,
        };
        assert!(bus.mount(16, Box::new(dev3)).is_err());

        // Unmount clears the slot.
        bus.unmount(SCSI_TARGET_DISK);
        assert!(!bus.is_attached(SCSI_TARGET_DISK));
    }
}