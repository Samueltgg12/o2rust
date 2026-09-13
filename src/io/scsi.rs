//! SCSI bus and target emulation for the AIC-7880 controllers.
//!
//! The O2 has **two** AIC-7880 UltraWide SCSI adapters on the MACE PCI bus
//! (`fixup-ip32.c` lists aic7xxx SCSI0 and SCSI1; `sgi-o2` docs call them
//! channel A and channel B). Each keeps its own 16-target bus, and the SGI
//! guest layout spans both:
//!
//! - **SCSI0 / ahc0** (PCI device 1) — hard-disk bus: **target 1** = internal
//!   disk.
//! - **SCSI1 / ahc1** (PCI device 2) — CD-ROM bus: **target 6** = internal
//!   CD-ROM drive.
//!
//! This module provides the per-controller target → [`BlockDevice`] map and
//! the SCSI command engine ([`ScsiBus::exec_cdb`]) that interprets guest CDBs
//! (INQUIRY, READ CAPACITY, READ/WRITE, MODE SENSE, …) against the mounted
//! images. Host behavior references: Linux `drivers/scsi/aic7xxx/`,
//! IRIX `adp78.c`/`dksc.c`/`scsi.c` (PROM inventory uses INQUIRY(0x12) +
//! READ CAPACITY(0x25)).

use crate::storage::BlockDevice;
use std::fmt;
use thiserror::Error;

/// Number of SCSI targets on a single bus.
pub const SCSI_TARGET_COUNT: usize = 16;

/// SCSI target ID of the (first) internal hard disk (on the SCSI0 bus).
pub const SCSI_TARGET_DISK: u8 = 1;

/// SCSI target ID of the internal CD-ROM drive (on the SCSI1 bus).
pub const SCSI_TARGET_CDROM: u8 = 6;

// === SCSI-2 opcodes handled by the emulated targets ===

const OP_TEST_UNIT_READY: u8 = 0x00;
const OP_REQ_SENSE: u8 = 0x03;
const OP_INQUIRY: u8 = 0x12;
const OP_MODE_SELECT6: u8 = 0x15;
const OP_MODE_SENSE6: u8 = 0x1a;
const OP_START_STOP: u8 = 0x1b;
const OP_SEND_DIAGNOSTIC: u8 = 0x1d;
const OP_PREVENT_ALLOW: u8 = 0x1e;
const OP_READ_CAPACITY10: u8 = 0x25;
const OP_READ10: u8 = 0x28;
const OP_READ6: u8 = 0x08;
const OP_WRITE6: u8 = 0x0a;
const OP_WRITE10: u8 = 0x2a;
const OP_READ12: u8 = 0xa8;
const OP_WRITE12: u8 = 0xaa;
const OP_WRITE16: u8 = 0x8a;
const OP_READ16: u8 = 0x88;
const OP_SYNC_CACHE10: u8 = 0x35;
const OP_VERIFY: u8 = 0x2f;
const OP_MODE_SELECT10: u8 = 0x55;
const OP_MODE_SENSE10: u8 = 0x5a;
const OP_READ_TOC: u8 = 0x43;
/// SGI-proprietary "revert" command used by the IRIX PROM CD driver
/// (`stand/arcs/lib/libsk/io/cdrom.c`) to switch a dual-mode drive back.
const OP_SGI_CD_REVERT: u8 = 0xc9;

// SCSI-2 status bytes (white book §7.2.2).
pub const STATUS_GOOD: u8 = 0x00;
pub const STATUS_CHECK_CONDITION: u8 = 0x02;

// Sense keys (SCSI-2 §7.2.4).
const SK_NO_SENSE: u8 = 0x00;
const SK_NOT_READY: u8 = 0x02;
const SK_ILLEGAL_REQUEST: u8 = 0x05;
const SK_UNIT_ATTENTION: u8 = 0x06;
const SK_DATA_PROTECT: u8 = 0x07;

/// Direction of the SCSI data phase for a completed command.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScsiPhase {
    /// Command completes with no data transfer.
    NoData,
    /// Data-in phase: `len` bytes have been read from the device into the
    /// caller's buffer and must move to the host bus.
    DataIn { len: usize },
    /// Data-out phase: `len` bytes from the host already sit in the caller's
    /// buffer and must be written to the device.
    DataOut { len: usize },
}

/// Result of executing one CDB against a target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScsiExec {
    /// SCSI status byte (0 = GOOD).
    pub status: u8,
    /// Data-phase disposition for the DMA engine.
    pub phase: ScsiPhase,
}

impl ScsiExec {
    fn good(phase: ScsiPhase) -> Self {
        Self { status: STATUS_GOOD, phase }
    }
    fn check_condition(phase: ScsiPhase) -> Self {
        Self { status: STATUS_CHECK_CONDITION, phase }
    }
}

/// Per-target sense + unit-attention state.
#[derive(Default)]
struct TargetState {
    sense_key: u8,
    sense_asc: u8,
    sense_ascq: u8,
    unit_attention: bool,
}

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
/// Each AIC-7880 owns one [`ScsiBus`]. Per SGI layout the disk lives at
/// [`SCSI_TARGET_DISK`] on the SCSI0 (ahc0) bus and the CD-ROM at
/// [`SCSI_TARGET_CDROM`] on the SCSI1 (ahc1) bus.
#[derive(Default)]
pub struct ScsiBus {
    /// Attached devices indexed by target ID (0..15).
    targets: [Option<Box<dyn BlockDevice>>; SCSI_TARGET_COUNT],
    /// Per-target sense and unit-attention state.
    sense: [TargetState; SCSI_TARGET_COUNT],
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
        let st = &mut self.sense[target as usize];
        st.sense_key = SK_NO_SENSE;
        st.sense_asc = 0;
        st.sense_ascq = 0;
        // A freshly attached device reports UNIT ATTENTION to the first
        // command: "power on, reset, or bus device reset occurred" (0x29).
        st.unit_attention = true;
        *slot = Some(device);
        Ok(())
    }

    /// Detach whatever is at `target` (no-op for empty/invalid targets).
    pub fn unmount(&mut self, target: u8) {
        if (target as usize) < SCSI_TARGET_COUNT {
            self.targets[target as usize] = None;
            self.sense[target as usize] = TargetState::default();
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

    /// Execute one CDB against the device at `target`.
    ///
    /// `data` is the on-chip data buffer for this transfer: for data-out
    /// (WRITE) commands it already holds the `len` host bytes gathered by
    /// the DMA engine; for data-in commands it receives the device bytes.
    /// The returned [`ScsiPhase`] tells the DMA engine how much moved and in
    /// which direction. Sense/unit-attention is per target.
    pub fn exec_cdb(&mut self, target: u8, cdb: &[u8; 16], data: &mut [u8]) -> ScsiExec {
        if target as usize >= SCSI_TARGET_COUNT || self.targets[target as usize].is_none() {
            return ScsiExec::check_condition(ScsiPhase::NoData);
        }
        let is_removable = self
            .targets[target as usize]
            .as_ref()
            .map(|d| d.sector_size() != 512)
            .unwrap_or(false);

        let op = cdb[0];
        match op {
            OP_TEST_UNIT_READY => {
                let st = &mut self.sense[target as usize];
                let ua = st.unit_attention;
                if ua {
                    st.unit_attention = false;
                    st.sense_key = SK_UNIT_ATTENTION;
                    st.sense_asc = 0x29;
                    st.sense_ascq = 0x00;
                    ScsiExec::check_condition(ScsiPhase::NoData)
                } else {
                    ScsiExec::good(ScsiPhase::NoData)
                }
            }
            OP_REQ_SENSE => {
                let alloc = cdb[4] as usize;
                let n = alloc.min(18);
                if n > 0 {
                    // REQUEST SENSE reports the current sense; each error
                    // overwrites it, so no clearing is needed here.
                    let st = &self.sense[target as usize];
                    let key = st.sense_key;
                    let asc = st.sense_asc;
                    let ascq = st.sense_ascq;
                    data.fill(0);
                    data[0] = 0x70; // current error, fixed format
                    data[2] = key;
                    data[7] = 10; // additional sense length
                    data[12] = asc;
                    data[13] = ascq;
                    return ScsiExec::good(ScsiPhase::DataIn { len: n });
                }
                ScsiExec::good(ScsiPhase::NoData)
            }
            OP_INQUIRY => {
                let alloc = cdb[4] as usize;
                let inq = inquiry_bytes(is_removable);
                let n = alloc.min(inq.len());
                data[..n].copy_from_slice(&inq[..n]);
                ScsiExec::good(ScsiPhase::DataIn { len: n })
            }
            OP_READ_CAPACITY10 => {
                let Some(dev) = self.targets[target as usize].as_deref_mut() else {
                    return ScsiExec::check_condition(ScsiPhase::NoData);
                };
                let count = *&dev.sector_count();
                let blk = dev.sector_size();
                let last = (count.max(1) - 1) as u32;
                data[0..8].copy_from_slice(&[
                    (last >> 24) as u8,
                    (last >> 16) as u8,
                    (last >> 8) as u8,
                    last as u8,
                    (blk >> 24) as u8,
                    (blk >> 16) as u8,
                    (blk >> 8) as u8,
                    blk as u8,
                ]);
                ScsiExec::good(ScsiPhase::DataIn { len: 8 })
            }
            OP_READ6 | OP_READ10 | OP_READ12 | OP_READ16 => {
                let (lba, n, ok) = decode_block_cdb(cdb);

                if !ok {
                    self.set_sense_direct(target, SK_ILLEGAL_REQUEST, 0x20, 0x00);
                    return ScsiExec::check_condition(ScsiPhase::NoData);
                }
                let Some(dev) = self.targets[target as usize].as_deref_mut() else {
                    return ScsiExec::check_condition(ScsiPhase::NoData);
                };
                let bytes = match do_read(dev, lba, n, data) {
                    Ok(t) => t,
                    Err(_) => {
                        self.set_sense_direct(target, SK_ILLEGAL_REQUEST, 0x21, 0x00);
                        return ScsiExec::check_condition(ScsiPhase::NoData);
                    }
                };
                ScsiExec::good(ScsiPhase::DataIn { len: bytes })
            }
            OP_WRITE6 | OP_WRITE10 | OP_WRITE12 | OP_WRITE16 => {
                let (lba, n, ok) = decode_block_cdb(cdb);
                if !ok {
                    self.set_sense_direct(target, SK_ILLEGAL_REQUEST, 0x20, 0x00);
                    return ScsiExec::check_condition(ScsiPhase::NoData);
                }
                let Some(dev) = self.targets[target as usize].as_deref_mut() else {
                    return ScsiExec::check_condition(ScsiPhase::NoData);
                };
                if dev.is_read_only() {
                    self.set_sense_direct(target, SK_DATA_PROTECT, 0x27, 0x00);
                    return ScsiExec::check_condition(ScsiPhase::NoData);
                }
                let need = n as usize * dev.sector_size() as usize;
                if data.len() < need {
                    self.set_sense_direct(target, SK_ILLEGAL_REQUEST, 0x20, 0x00);
                    return ScsiExec::check_condition(ScsiPhase::NoData);
                }
                match do_write(dev, lba, n, &data[..need]) {
                    Ok(()) => ScsiExec::good(ScsiPhase::NoData),
                    Err(_) => {
                        self.set_sense_direct(target, SK_ILLEGAL_REQUEST, 0x21, 0x00);
                        ScsiExec::check_condition(ScsiPhase::NoData)
                    }
                }
            }
            OP_MODE_SENSE6 | OP_MODE_SENSE10 => {

                let alloc = if op == OP_MODE_SENSE6 {
                    cdb[4] as usize
                } else {
                    ((cdb[7] as usize) << 8) | cdb[8] as usize
                };
                let n = alloc.min(4);
                if n > 0 {
                    data[..n].fill(0);
                }
                ScsiExec::good(ScsiPhase::DataIn { len: n })
            }
            OP_MODE_SELECT6 | OP_MODE_SELECT10 | OP_START_STOP | OP_PREVENT_ALLOW
            | OP_SEND_DIAGNOSTIC | OP_SYNC_CACHE10 | OP_VERIFY | OP_SGI_CD_REVERT => {
                ScsiExec::good(ScsiPhase::NoData)
            }
            OP_READ_TOC if is_removable => {
                let Some(dev) = self.targets[target as usize].as_deref_mut() else {
                    return ScsiExec::check_condition(ScsiPhase::NoData);
                };
                // Minimal TOC (track 1 = start of data, lead-out at last LBA).
                let count = dev.sector_count();
                let last = (count.max(1) - 1) as u32;
                let mut toc = [0u8; 20];
                toc[0] = 0; // first track is 1
                toc[1] = 0;
                toc[2] = 1; // first track number
                toc[3] = 1; // last track number
                // Descriptor 0: track 1 (control 0x04 = data track).
                toc[5] = 0x14;
                toc[7] = 1;
                // Descriptor 1: lead-out 0xAA at `last`.
                toc[13] = 0x01;
                toc[14..18].copy_from_slice(&[
                    (last >> 24) as u8,
                    (last >> 16) as u8,
                    (last >> 8) as u8,
                    last as u8,
                ]);
                let n = cdb[7] as usize; // allocation length
                let n = n.min(toc.len());
                data[..n].copy_from_slice(&toc[..n]);
                ScsiExec::good(ScsiPhase::DataIn { len: n })
            }
            _ => {
                self.set_sense_direct(target, SK_ILLEGAL_REQUEST, 0x20, 0x00);
                ScsiExec::check_condition(ScsiPhase::NoData)
            }
        }
    }

    fn set_sense_direct(&mut self, target: u8, key: u8, asc: u8, ascq: u8) {
        let st = &mut self.sense[target as usize];
        st.sense_key = key;
        st.sense_asc = asc;
        st.sense_ascq = ascq;
        st.unit_attention = false;
    }
}

/// Build the standard 36-byte INQUIRY response for an emulated device.
///
/// The O2's interior drives report SGI's SCSI vendor id. These are emulated
/// devices, so the product/rev fields are synthetic; device type comes from
/// the backing image (512-byte sectors ⇒ direct access, 2048 ⇒ CD-ROM).
fn inquiry_bytes(removable: bool) -> [u8; 36] {
    let mut b = [0u8; 36];
    b[0] = if removable { 0x05 } else { 0x00 }; // CD-ROM / direct access
    if removable {
        b[1] = 0x80; // RMB: removable medium
    }
    b[2] = 0x02; // SCSI-2 command set version
    b[4] = 0x1f; // additional length ⇒ 36 total
    b[8..16].copy_from_slice(b"SGI     ");
    b[16..32].copy_from_slice(if removable {
        b"O2RUST-CDROM    "
    } else {
        b"O2RUST-DISK     "
    });
    b[32..36].copy_from_slice(b"1.00");
    b
}

/// Decode the (lba, count) of a READ/WRITE CDB. Returns `ok=false` for
/// invalid transfer lengths (0 is legal for REQ SENSE counting but not for
/// block I/O, and SCSI-2 does not define a `count == 0` transfer).
fn decode_block_cdb(cdb: &[u8; 16]) -> (u64, u32, bool) {
    let (lba, count): (u64, u32) = match cdb[0] {
        OP_READ6 | OP_WRITE6 => {
            if cdb[4] == 0 {
                return (0, 0, false);
            }
            let lba = (u64::from(cdb[1] & 0x1f) << 16)
                | (u64::from(cdb[2]) << 8)
                | u64::from(cdb[3]);
            (lba, u32::from(cdb[4]))
        }
        OP_READ10 | OP_WRITE10 => {
            if cdb[8] == 0 && cdb[7] == 0 {
                return (0, 0, false);
            }
            let lba = (u64::from(cdb[2]) << 24)
                | (u64::from(cdb[3]) << 16)
                | (u64::from(cdb[4]) << 8)
                | u64::from(cdb[5]);
            let count = (u32::from(cdb[7]) << 8) | u32::from(cdb[8]);
            (lba, count)
        }
        OP_READ12 | OP_WRITE12 => {
            if cdb[6..8] == [0, 0] {
                return (0, 0, false);
            }
            let lba = (u64::from(cdb[2]) << 24)
                | (u64::from(cdb[3]) << 16)
                | (u64::from(cdb[4]) << 8)
                | u64::from(cdb[5]);
            let count = (u32::from(cdb[6]) << 8) | u32::from(cdb[7]);
            (lba, count)
        }
        OP_READ16 | OP_WRITE16 => {
            if cdb[10..14] == [0, 0, 0, 0] {
                return (0, 0, false);
            }
            let lba = (u64::from(cdb[2]) << 56)
                | (u64::from(cdb[3]) << 48)
                | (u64::from(cdb[4]) << 40)
                | (u64::from(cdb[5]) << 32)
                | (u64::from(cdb[6]) << 24)
                | (u64::from(cdb[7]) << 16)
                | (u64::from(cdb[8]) << 8)
                | u64::from(cdb[9]);
            let count = (u32::from(cdb[10]) << 24)
                | (u32::from(cdb[11]) << 16)
                | (u32::from(cdb[12]) << 8)
                | u32::from(cdb[13]);
            (lba, count)
        }
        _ => return (0, 0, false),
    };
    (lba, count, true)
}

/// Fill `data` from the device at `lba` for `count` blocks. Returns bytes moved.
fn do_read(dev: &mut dyn BlockDevice, lba: u64, count: u32, data: &mut [u8]) -> std::io::Result<usize> {
    if lba.checked_add(u64::from(count)).unwrap_or(u64::MAX) > dev.sector_count() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "lba range past end of medium",
        ));
    }
    let ss = dev.sector_size() as usize;
    let need = count as usize * ss;
    if data.len() < need {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "data buffer too small",
        ));
    }
    dev.read_blocks(lba, count, &mut data[..need])?;
    Ok(need)
}

/// Commit `data`'s `count` blocks to the device at `lba`.
fn do_write(dev: &mut dyn BlockDevice, lba: u64, count: u32, data: &[u8]) -> std::io::Result<()> {
    if lba.checked_add(u64::from(count)).unwrap_or(u64::MAX) > dev.sector_count() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "lba range past end of medium",
        ));
    }
    let ss = dev.sector_size() as usize;
    for i in 0..count as usize {
        dev.write_sector(lba + i as u64, &data[i * ss..(i + 1) * ss])?;
    }
    Ok(())
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

    fn disk_bus(sectors: usize) -> ScsiBus {
        let mut bus = ScsiBus::new();
        let dev = MemDisk {
            name: "test.img".into(),
            data: Arc::new(Mutex::new(vec![0xab; 512 * sectors])),
            sector_size: 512,
        };
        bus.mount(SCSI_TARGET_DISK, Box::new(dev)).unwrap();
        bus
    }

    #[test]
    fn cdb_test_unit_ready_reports_unit_attention_once() {
        let mut bus = disk_bus(8);
        let mut data = [0u8; 512];
        // First TUR after attach: UA -> CHECK CONDITION.
        let cdb = [0u8; 16];
        let r = bus.exec_cdb(SCSI_TARGET_DISK, &cdb, &mut data);
        assert_eq!(r.status, STATUS_CHECK_CONDITION);
        // The guest fetches sense 0x29.
        let mut cdb = [0u8; 16];
        cdb[0] = OP_REQ_SENSE;
        cdb[4] = 18;
        let r = bus.exec_cdb(SCSI_TARGET_DISK, &cdb, &mut data);
        assert_eq!(r.status, STATUS_GOOD);
        assert_eq!(r.phase, ScsiPhase::DataIn { len: 18 });
        assert_eq!(data[0], 0x70);
        assert_eq!(data[2], 0x06); // unit attention
        assert_eq!(data[12], 0x29);
        // Second TUR succeeds.
        let cdb = [0u8; 16];
        let r = bus.exec_cdb(SCSI_TARGET_DISK, &cdb, &mut data);
        assert_eq!(r.status, STATUS_GOOD);
        assert_eq!(r.phase, ScsiPhase::NoData);
    }

    #[test]
    fn cdb_inquiry_identifies_disk_and_cd() {
        let mut bus = disk_bus(8);
        let mut data = [0u8; 36];
        let mut cdb = [0u8; 16];
        cdb[0] = OP_INQUIRY;
        cdb[4] = 36;
        let r = bus.exec_cdb(SCSI_TARGET_DISK, &cdb, &mut data);
        assert_eq!(r.status, STATUS_GOOD);
        assert_eq!(data[0], 0x00); // direct access
        assert_eq!(&data[8..16], b"SGI     ");
        assert_eq!(&data[16..32], b"O2RUST-DISK     ");

        // CD-ROM on a second bus reports type 0x05 + RMB.
        let mut cdbus = ScsiBus::new();
        let cd = MemDisk {
            name: "install.iso".into(),
            data: Arc::new(Mutex::new(vec![0u8; 2048 * 64])),
            sector_size: 2048,
        };
        cdbus.mount(SCSI_TARGET_CDROM, Box::new(cd)).unwrap();
        let mut data = [0u8; 36];
        let mut cdb = [0u8; 16];
        cdb[0] = OP_INQUIRY;
        cdb[4] = 36;
        let r = cdbus.exec_cdb(SCSI_TARGET_CDROM, &cdb, &mut data);
        assert_eq!(r.status, STATUS_GOOD);
        assert_eq!(data[0], 0x05); // CD-ROM
        assert_ne!(data[1] & 0x80, 0); // RMB
        assert_eq!(&data[16..32], b"O2RUST-CDROM    ");
    }

    #[test]
    fn cdb_read_capacity_and_read_10() {
        let mut bus = disk_bus(128);
        let mut data = [0u8; 8];
        let mut cdb = [0u8; 16];
        cdb[0] = OP_READ_CAPACITY10;
        let r = bus.exec_cdb(SCSI_TARGET_DISK, &cdb, &mut data);
        assert_eq!(r.status, STATUS_GOOD);
        assert_eq!(r.phase, ScsiPhase::DataIn { len: 8 });
        let last = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        let blk = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
        assert_eq!(last, 127);
        assert_eq!(blk, 512);

        // READ(10): 3 blocks starting at lba 4.
        let mut data = vec![0u8; 512 * 3];
        let mut cdb = [0u8; 16];
        cdb[0] = OP_READ10;
        cdb[5] = 4;
        cdb[7] = 0;
        cdb[8] = 3;
        let r = bus.exec_cdb(SCSI_TARGET_DISK, &cdb, &mut data);
        assert_eq!(r.status, STATUS_GOOD);
        assert_eq!(r.phase, ScsiPhase::DataIn { len: 512 * 3 });
        assert!(data.iter().all(|&b| b == 0xab));
    }

    #[test]
    fn cdb_write_10_round_trip() {
        let mut bus = disk_bus(8);
        // Write a full 512-byte sector at lba 5.
        let mut payload = [0u8; 512];
        payload[..6].copy_from_slice(b"O2RUST");
        let mut cdb = [0u8; 16];
        cdb[0] = OP_WRITE10;
        cdb[5] = 5;
        cdb[7] = 0;
        cdb[8] = 1;
        let r = bus.exec_cdb(SCSI_TARGET_DISK, &cdb, &mut payload);
        assert_eq!(r.status, STATUS_GOOD);
        // Read it back.
        let mut data = vec![0u8; 512];
        let mut cdb = [0u8; 16];
        cdb[0] = OP_READ10;
        cdb[5] = 5;
        cdb[8] = 1;
        bus.exec_cdb(SCSI_TARGET_DISK, &cdb, &mut data);
        assert_eq!(&data[..6], b"O2RUST");
    }

    #[test]
    fn cdb_unknown_opcode_reports_illegal_request() {
        let mut bus = disk_bus(8);
        let mut data = [0u8; 512];
        let mut cdb = [0u8; 16];
        cdb[0] = 0x9e; // unsupported opcode
        let r = bus.exec_cdb(SCSI_TARGET_DISK, &cdb, &mut data);
        assert_eq!(r.status, STATUS_CHECK_CONDITION);
        let mut cdb = [0u8; 16];
        cdb[0] = OP_REQ_SENSE;
        cdb[4] = 18;
        bus.exec_cdb(SCSI_TARGET_DISK, &cdb, &mut data);
        assert_eq!(data[2], 0x05); // illegal request
        assert_eq!(data[12], 0x20);
    }

    #[test]
    fn cdb_read_past_end_reports_lba_out_of_range() {
        let mut bus = disk_bus(8);
        let mut data = vec![0u8; 512];
        let mut cdb = [0u8; 16];
        cdb[0] = OP_READ10;
        cdb[5] = 7;
        cdb[7] = 0;
        cdb[8] = 2; // lba 7 + 2 > 8 sectors
        let r = bus.exec_cdb(SCSI_TARGET_DISK, &cdb, &mut data);
        assert_eq!(r.status, STATUS_CHECK_CONDITION);
        let mut cdb = [0u8; 16];
        cdb[0] = OP_REQ_SENSE;
        cdb[4] = 18;
        bus.exec_cdb(SCSI_TARGET_DISK, &cdb, &mut data);
        assert_eq!(data[2], 0x05);
        assert_eq!(data[12], 0x21); // LBA out of range
    }

    #[test]
    fn cdbs_absorbed_by_simple_commands() {
        let mut bus = disk_bus(8);
        let mut data = [0u8; 64];
        // MODE SENSE(6) page-0 header, MODE SENSE(10), START/STOP, MODE SELECT,
        // SYNCHRONIZE CACHE, VERIFY, PREVENT/ALLOW, SGI 0xc9 all succeed.
        for op in [0x1a, 0x1b, 0x1d, 0x1e, 0x15, 0x35, 0x2f, 0x55, 0x5a, 0xc9] {
            let mut cdb = [0u8; 16];
            cdb[0] = op;
            if op == 0x1a {
                cdb[4] = 4;
            }
            let r = bus.exec_cdb(SCSI_TARGET_DISK, &cdb, &mut data);
            assert_eq!(r.status, STATUS_GOOD, "opcode {op:#x}");
        }
    }
}