//! SCSI bus and target emulation for the AIC-7880 controllers.
//!
//! The O2 has **two** AIC-7880 UltraWide SCSI adapters on the MACE PCI bus
//! (`fixup-ip32.c` lists aic7xxx SCSI0 and SCSI1; `sgi-o2` docs call them
//! channel A and channel B). Each keeps its own 16-target bus, and the SGI
//! guest layout spans both:
//!
//! - **SCSI0 / ahc0** (PCI device 1) — internal-drive bus: **target 1** =
//!   hard disk, **target 4** = internal CD-ROM drive.
//! - **SCSI1 / ahc1** (PCI device 2) — external bus, unpopulated by default.
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

/// SCSI target ID of the internal CD-ROM drive (on the SCSI0 bus).
pub const SCSI_TARGET_CDROM: u8 = 4;

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
/// Each AIC-7880 owns one [`ScsiBus`]. Per SGI layout the internal drives
/// share the SCSI0 (ahc0) bus: the disk at [`SCSI_TARGET_DISK`] and the
/// CD-ROM at [`SCSI_TARGET_CDROM`].
#[derive(Default)]
pub struct ScsiBus {
    /// Attached devices indexed by target ID (0..15).
    targets: [Option<Box<dyn BlockDevice>>; SCSI_TARGET_COUNT],
    /// Per-target sense and unit-attention state.
    sense: [TargetState; SCSI_TARGET_COUNT],
    /// Live initiator<->target session on this bus (one at a time).
    pub link: ScsiLink,
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

    /// Execute one CDB against the device at `target`. See
    /// [`exec_cdb_inner`] for the byte-level behavior.
    pub fn exec_cdb(&mut self, target: u8, cdb: &[u8; 16], data: &mut [u8]) -> ScsiExec {
        let Self { targets, sense, .. } = self;
        exec_cdb_inner(targets, sense, target, cdb, data)
    }

    fn set_sense_direct(&mut self, target: u8, key: u8, asc: u8, ascq: u8) {
        let st = &mut self.sense[target as usize];
        st.sense_key = key;
        st.sense_asc = asc;
        st.sense_ascq = ascq;
        st.unit_attention = false;
    }
}


/// Execute one CDB against the device at `target`.
///
/// Split from `ScsiBus` so the link engine can call it after destructuring
/// the bus into `targets`+`sense` (avoids the `link` field borrow).
fn exec_cdb_inner(
    targets: &mut [Option<Box<dyn BlockDevice>>; SCSI_TARGET_COUNT],
    sense: &mut [TargetState; SCSI_TARGET_COUNT],
    target: u8,
    cdb: &[u8; 16],
    data: &mut [u8],
) -> ScsiExec {
    if target as usize >= SCSI_TARGET_COUNT || targets[target as usize].is_none() {
        return ScsiExec::check_condition(ScsiPhase::NoData);
    }
    let is_removable = targets[target as usize]
        .as_ref()
        .map(|d| d.sector_size() != 512)
        .unwrap_or(false);
        let op = cdb[0];
        match op {
            OP_TEST_UNIT_READY => {
                let st = &mut sense[target as usize];
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
                    let st = &sense[target as usize];
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
                let Some(dev) = targets[target as usize].as_deref_mut() else {
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
                    set_direct(sense, target, SK_ILLEGAL_REQUEST, 0x20, 0x00);
                    return ScsiExec::check_condition(ScsiPhase::NoData);
                }
                let Some(dev) = targets[target as usize].as_deref_mut() else {
                    return ScsiExec::check_condition(ScsiPhase::NoData);
                };
                let bytes = match do_read(dev, lba, n, data) {
                    Ok(t) => t,
                    Err(_) => {
                        set_direct(sense, target, SK_ILLEGAL_REQUEST, 0x21, 0x00);
                        return ScsiExec::check_condition(ScsiPhase::NoData);
                    }
                };
                ScsiExec::good(ScsiPhase::DataIn { len: bytes })
            }
            OP_WRITE6 | OP_WRITE10 | OP_WRITE12 | OP_WRITE16 => {
                let (lba, n, ok) = decode_block_cdb(cdb);
                if !ok {
                    set_direct(sense, target, SK_ILLEGAL_REQUEST, 0x20, 0x00);
                    return ScsiExec::check_condition(ScsiPhase::NoData);
                }
                let Some(dev) = targets[target as usize].as_deref_mut() else {
                    return ScsiExec::check_condition(ScsiPhase::NoData);
                };
                if dev.is_read_only() {
                    set_direct(sense, target, SK_DATA_PROTECT, 0x27, 0x00);
                    return ScsiExec::check_condition(ScsiPhase::NoData);
                }
                let need = n as usize * dev.sector_size() as usize;
                if data.len() < need {
                    set_direct(sense, target, SK_ILLEGAL_REQUEST, 0x20, 0x00);
                    return ScsiExec::check_condition(ScsiPhase::NoData);
                }
                match do_write(dev, lba, n, &data[..need]) {
                    Ok(()) => ScsiExec::good(ScsiPhase::NoData),
                    Err(_) => {
                        set_direct(sense, target, SK_ILLEGAL_REQUEST, 0x21, 0x00);
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
                let Some(dev) = targets[target as usize].as_deref_mut() else {
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
                let st = &mut sense[target as usize];
                st.sense_key = SK_ILLEGAL_REQUEST;
                st.sense_asc = 0x20;
                st.sense_ascq = 0x00;
                st.unit_attention = false;
                ScsiExec::check_condition(ScsiPhase::NoData)
            }
        }
    }

// ==== Live bus-session engine (initiator <-> target link layer) ====
//
// `ScsiBus` holds only the mounted devices and per-target sense. The
// byte-at-a-time link state below models what happens *after* the
// AIC-7880 sequencer wins a selection: message out (identify/tag), command
// transfer, data in/out, status and message in, then bus free. The chip
// side drives bytes through its pin registers (SCSIDATL/SCSIBUSL) and the
// DMA FIFO; the link tracks which phase the target is in and what bytes
// it is producing/consuming.

/// Bus phase encodings (SCSISIGI CDI|IOI|MSGI, from `aic7xxx.reg`).
pub const PHASE_DATA_OUT: u8 = 0x00;
pub const PHASE_DATA_IN: u8 = 0x40;
pub const PHASE_COMMAND: u8 = 0x80;
pub const PHASE_MSG_OUT: u8 = 0xa0;
pub const PHASE_STATUS: u8 = 0xc0;
pub const PHASE_MSG_IN: u8 = 0xe0;

/// SCSI bus message codes (SCSI-2 §5.6).
pub const MSG_COMMAND_COMPLETE: u8 = 0x00;
pub const MSG_EXTENDED: u8 = 0x01;
pub const MSG_SAVE_DATA_POINTER: u8 = 0x02;
pub const MSG_RESTORE_POINTERS: u8 = 0x03;
pub const MSG_DISCONNECT: u8 = 0x04;
pub const MSG_INITIATOR_DETECTED_ERROR: u8 = 0x05;
pub const MSG_ABORT: u8 = 0x06;
pub const MSG_MESSAGE_REJECT: u8 = 0x07;
pub const MSG_NO_OPERATION: u8 = 0x08;
pub const MSG_PARITY_ERROR: u8 = 0x09;
pub const MSG_BUS_DEVICE_RESET: u8 = 0x0c;
pub const MSG_SIMPLE_Q_TAG: u8 = 0x20;
pub const MSG_HEAD_OF_Q_TAG: u8 = 0x21;
pub const MSG_ORDERED_Q_TAG: u8 = 0x22;
pub const MSG_IDENTIFY_BASE: u8 = 0x80; // | DISC(0x40) | LUN

/// Extended message codes.
pub const MSG_EXT_SDTR: u8 = 0x01;
pub const MSG_EXT_WDTR: u8 = 0x03;

/// CDB length from the group code in `cdb[0]` (SCSI-2 §5.2):
/// groups 0/5/2/4 map to 6/12/10/16, group1 -> 10, reserved -> 16 (safe).
fn cdb_expected_len(first: u8) -> usize {
    match first >> 5 {
        0 => 6,
        1 => 10,
        2 => 10,
        5 => 12,
        3 => 16,
        4 => 16,
        6 => 16,
        _ => 16,
    }
}

/// One live initiator<->target session on this bus.
///
/// The session has explicit phases the chip's ITloop can follow. Only one
/// session may be live per bus (the AIC-7880 drives one target at a time).
#[derive(Debug, Default)]
pub struct ScsiLink {
    /// The selected target id, if any.
    pub target: Option<u8>,

    // --- target-visible wire state ---
    /// The current bus phase (SCSISIGI phase bits).
    pub phase: u8,
    /// Target-asserted REQ (byte/handshake pending).
    pub req: bool,
    /// True while the link is at a free bus (post-command).
    pub free: bool,

    // --- protocol accumulators ---
    /// Bytes the initiator has put into the current message-out.
    msg_out: Vec<u8>,
    /// Decoded message is complete (bus can advance to command).
    msg_done: bool,
    /// The LUN selected by the IDENTIFY message.
    pub lun: u8,
    /// Tagged queue id from the tag message (None for untagged).
    pub itag: Option<u8>,
    /// Disconnect-enabled flag from identify.
    pub disc: bool,
    /// Collected CDB bytes.
    cdb: Vec<u8>,
    /// Expected total CDB length (from group code).
    cdb_expected: usize,
    /// Command completed/executed; status fixed.
    executed: bool,

    // --- data transfer staging ---
    /// Bytes the device produces (data-in), drained into the chip FIFO.
    data_in: std::collections::VecDeque<u8>,
    /// Total data-in bytes the device provided for this command.
    pub data_in_total: usize,
    /// Accumulated data-out bytes (WRITE) delivered by the initiator.
    data_out: Vec<u8>,
    /// Expected total data-out length for a WRITE command.
    pub data_out_expected: usize,
    /// Bytes consumed so far this phase (to compare against the chip-side
    /// STCNT when deciding PHASEMIS).
    pub phase_bytes: usize,
    /// Status for the current command once it finishes.
    status: u8,
    /// Bytes the target queues for message-in (e.g. SDTR echo, complete).
    msg_in: std::collections::VecDeque<u8>,
    /// True when the message-in/status streams ended and the target has
    /// released the bus.
    pub released: bool,
}

impl ScsiBus {
    /// Start an over-bus session with `target` (the chip completed
    /// selection). Phase becomes MSG_OUT since the initiator has ATN
    /// (the AIC-7880 auto-asserts ATN per the `identify` flow).
    pub fn link_select(&mut self, target: u8) {
        let mut link = ScsiLink::default();
        link.target = Some(target);
        link.phase = PHASE_MSG_OUT;
        link.req = true;
        self.link = link;
    }

    /// Whether an attached device exists at `target`.
    pub fn link_device_present(&self, target: u8) -> bool {
        self.is_attached(target)
    }

    /// Current link phase (SCSISIGI phase bits). When nothing is selected,
    /// report bus-free (no REQI/asserted phase).
    pub fn link_phase(&self) -> u8 {
        self.link.phase
    }

    /// Initiator PIO/DMA byte written to the bus (out direction): feeds
    /// the live protocol. Returns true when the byte was accepted.
    pub fn link_out_byte(&mut self, v: u8) -> bool {
        let Self { targets, sense, link, .. } = self;
        link.out_byte(targets, sense, v)
    }

    /// Initiator PIO read from the bus (in direction): SCSIDATL/SCSIBUSL
    /// read path; returns the byte (driven by target) or 0xff.
    pub fn link_in_byte(&mut self) -> u8 {
        let Self { targets, sense, link, .. } = self;
        link.in_byte(targets, sense)
    }

    /// Number of message-in bytes the target has queued.
    pub fn link_msg_in_len(&self) -> usize {
        self.link.msg_in.len()
    }

    /// Pop one message-in byte (called by the chip on SCSIDATL/DFDAT reads
    /// during the MSG_IN phase). Returns `None` if the queue is empty.
    pub fn link_next_msg_in(&mut self) -> Option<u8> {
        let b = self.link.msg_in.pop_front()?;
        if self.link.msg_in.is_empty() {
            // No more messages: this was the command complete.
            self.link.release();
        }
        Some(b)
    }

    /// Pull one data-in byte (target -> bus) for the chip FIFO.
    pub fn link_next_data(&mut self) -> Option<u8> {
        let l = &mut self.link;
        let b = l.data_in.pop_front()?;
        l.phase_bytes += 1;
        if l.data_in.is_empty() {
            l.enter_status();
        }
        Some(b)
    }

    /// Push one data byte from the chip into the target (data-out).
    pub fn link_push_data(&mut self, b: u8) {
        let l = &mut self.link;
        l.data_out.push(b);
        l.phase_bytes += 1;
    }

    /// Whether the target wants another out byte in this phase.
    pub fn link_wants_out_byte(&self) -> bool {
        let l = &self.link;
        match l.phase {
            PHASE_COMMAND => (l.cdb_expected == 0 || l.cdb.len() < l.cdb_expected),
            PHASE_DATA_OUT => true,
            _ => false,
        }
    }

    /// Whether the target has an in byte ready (data-in/status/msg-in).
    pub fn link_has_in_byte(&self) -> bool {
        let l = &self.link;
        match l.phase {
            PHASE_DATA_IN => !l.data_in.is_empty(),
            PHASE_MSG_IN => !l.msg_in.is_empty(),
            PHASE_STATUS => true,
            _ => false,
        }
    }

    /// Decoded data-phase remainder for the DataIn path: how many bytes the
    /// target still has to deliver this command.
    pub fn link_data_in_remaining(&self) -> usize {
        self.link.data_in.len()
    }
}

impl ScsiLink {
    fn out_byte(
        &mut self,
        targets: &mut [Option<Box<dyn BlockDevice>>; SCSI_TARGET_COUNT],
        sense: &mut [TargetState; SCSI_TARGET_COUNT],
        v: u8,
    ) -> bool {
        match self.phase {
            PHASE_MSG_OUT => {
                self.msg_out.push(v);
                self.try_consume_msg();
                true
            }
            PHASE_COMMAND => {
                if self.cdb_expected == 0 && self.cdb.is_empty() {
                    self.cdb_expected = cdb_expected_len(v);
                }
                self.cdb.push(v);
                if self.cdb.len() >= self.cdb_expected {
                    self.execute_cdb(targets, sense);
                }
                true
            }
            PHASE_DATA_OUT => {
                self.data_out.push(v);
                self.phase_bytes += 1;
                true
            }
            _ => false,
        }
    }

    fn in_byte(
        &mut self,
        targets: &mut [Option<Box<dyn BlockDevice>>; SCSI_TARGET_COUNT],
        sense: &mut [TargetState; SCSI_TARGET_COUNT],
    ) -> u8 {
        match self.phase {
            PHASE_MSG_IN => self.msg_in.pop_front().map_or(0x00, |b| {
                if self.msg_in.is_empty() {
                    self.release();
                }
                b
            }),
            PHASE_STATUS => {
                let st = self.status;
                self.phase = PHASE_MSG_IN;
                self.msg_in.push_back(MSG_COMMAND_COMPLETE);
                st
            }
            _ => 0xff,
        }
    }

    /// Decode the accumulated message-out stream; on a complete message
    /// update lun/tag, then go to COMMAND (or stay for more messages if
    /// an extended message was partially parsed).
    #[allow(clippy::too_many_lines)]
    fn try_consume_msg(&mut self) {
        let buf = std::mem::take(&mut self.msg_out);
        let mut i = 0;
        while i < buf.len() {
            let m = buf[i];
            if m & 0x80 != 0 {
                // IDENTIFY (0x80 | DISC(0x40) | lun)
                self.lun = m & 0x3f;
                self.disc = m & 0x40 != 0;
                i += 1;
                continue;
            }
            if m == MSG_EXTENDED {
                // [0x01, len, code, args...]
                if buf.len() - i < 3 {
                    break;
                }
                let len = buf[i + 1] as usize;
                if buf.len() - i < 2 + len {
                    break; // need the full extended message
                }
                let code = buf[i + 2];
                let args = &buf[i + 3..i + 2 + len];
                match code {
                    MSG_EXT_SDTR | MSG_EXT_WDTR => {
                        // Echo accepted sync/wide negotiate.
                        self.msg_in.push_back(MSG_EXTENDED);
                        self.msg_in.push_back(len as u8);
                        for &a in args {
                            self.msg_in.push_back(a);
                        }
                    }
                    _ => {
                        self.msg_in.push_back(MSG_MESSAGE_REJECT);
                    }
                }
                i += 2 + len;
                continue;
            }
            match m {
                MSG_SIMPLE_Q_TAG | MSG_HEAD_OF_Q_TAG | MSG_ORDERED_Q_TAG => {
                    if buf.len() - i < 2 {
                        break;
                    }
                    self.itag = Some(buf[i + 1]);
                    i += 2;
                }
                MSG_ABORT | MSG_BUS_DEVICE_RESET | MSG_MESSAGE_REJECT | MSG_NO_OPERATION => {
                    i += 1;
                }
                _ => {
                    i += 1;
                }
            }
        }
        // Retain any unconsumed partial tail.
        self.msg_out.extend_from_slice(&buf[i..]);
        // When the stream fully parsed, the initiator has finished its
        // message burst: advance to COMMAND.
        if self.msg_out.is_empty() {
            self.msg_done = true;
            self.enter_command();
        }
    }

    fn enter_command(&mut self) {
        self.msg_done = false;
        self.cdb.clear();
        self.cdb_expected = 0;
        self.phase = PHASE_COMMAND;
        self.phase_bytes = 0;
        self.req = true;
    }

    fn execute_cdb(
        &mut self,
        targets: &mut [Option<Box<dyn BlockDevice>>; SCSI_TARGET_COUNT],
        sense: &mut [TargetState; SCSI_TARGET_COUNT],
    ) {
        if self.executed {
            return;
        }
        self.executed = true;
        let target = self.target.unwrap_or(0) as usize;
        let mut cdb16 = [0u8; 16];
        let n = self.cdb.len().min(16);
        cdb16[..n].copy_from_slice(&self.cdb[..n]);
        // For writing: run the device once we've accumulated the payload.
        let is_write = matches!(cdb16[0], 0x0a | 0x0e | 0x2a | 0x2e | 0xaa | 0x8a | 0xae | 0x55 | 0x15);
        if is_write {
            // Ask the device how much payload the CDB wants; then go to
            // data-out until that many bytes reach us.
            let (lba, count, ok) = decode_block_cdb(&cdb16);
            let need = if ok {
                count as usize
                    * targets[target]
                        .as_deref()
                        .map(|d| d.sector_size() as usize)
                        .unwrap_or(512)
            } else if matches!(cdb16[0], 0x55) {
                // MODE_SELECT(10): payload = cdb[7..9]
                ((cdb16[7] as usize) << 8) | cdb16[8] as usize
            } else {
                // MODE_SELECT(6): payload = cdb[4]
                cdb16[4] as usize
            };
            let _ = lba;
            self.data_out_expected = need;
            if need == 0 {
                self.finish_command(targets, sense);
                return;
            }
            self.phase = PHASE_DATA_OUT;
            self.phase_bytes = 0;
            return;
        }
        // Reads/status/etc: execute now with a staging buffer, then go to
        // the DATA_IN / STATUS phases. Buffer capacity: the command's
        // nominal transfer size (or a modest cap for descriptors).
        let cap = match cdb16[0] {
            0x12 => 144,                    // INQUIRY
            0x25 => 8,                      // READ CAPACITY 10
            0x03 => 18,                     // REQ SENSE
            0x1a => cdb16[4] as usize,      // MODE SENSE 6
            0x5a => ((cdb16[7] as usize) << 8) | cdb16[8] as usize,
            _ => {
                let (lba, count, ok) = decode_block_cdb(&cdb16);
                if ok {
                    let _ = lba;
                    count as usize
                        * targets[target]
                            .as_deref()
                            .map(|d| d.sector_size() as usize)
                            .unwrap_or(512)
                } else {
                    0
                }
            }
        };
        let mut buf = vec![0u8; cap.max(1)];
        let _ = targets;
        let res = exec_cdb_inner(targets, sense, target as u8, &cdb16, &mut buf);
        self.status = res.status;
        match res.phase {
            ScsiPhase::DataIn { len } => {
                let mut q = std::collections::VecDeque::with_capacity(len);
                let len = len.min(buf.len());
                for &b in &buf[..len] {
                    q.push_back(b);
                }
                self.data_in_total = q.len();
                self.data_in = q;
                self.phase = if self.data_in.is_empty() {
                    PHASE_STATUS
                } else {
                    PHASE_DATA_IN
                };
                self.phase_bytes = 0;
            }
            _ => {
                self.phase = PHASE_STATUS;
                self.phase_bytes = 0;
            }
        }
    }

    /// Called once the data-out buffer is full enough to write back.
    pub fn finish_command(
        &mut self,
        targets: &mut [Option<Box<dyn BlockDevice>>; SCSI_TARGET_COUNT],
        sense: &mut [TargetState; SCSI_TARGET_COUNT],
    ) {
        let target = self.target.unwrap_or(0) as usize;
        let mut cdb16 = [0u8; 16];
        let n = self.cdb.len().min(16);
        cdb16[..n].copy_from_slice(&self.cdb[..n]);
        let mut buf = std::mem::take(&mut self.data_out);
        let res = exec_cdb_inner(targets, sense, target as u8, &cdb16, &mut buf);
        self.status = res.status;
        self.enter_status();
    }

    pub fn enter_status(&mut self) {
        self.phase = PHASE_STATUS;
        self.phase_bytes = 0;
        self.req = true;
        self.released = false;
    }

    fn release(&mut self) {
        self.released = true;
        self.free = true;
        self.target = None;
    }
}
fn set_direct(sense: &mut [TargetState; SCSI_TARGET_COUNT], target: u8, key: u8, asc: u8, ascq: u8) {
    let st = &mut sense[target as usize];
    st.sense_key = key;
    st.sense_asc = asc;
    st.sense_ascq = ascq;
    st.unit_attention = false;
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

        // The internal CD-ROM shares the SCSI0 bus and reports type 0x05 + RMB.
        let cd = MemDisk {
            name: "install.iso".into(),
            data: Arc::new(Mutex::new(vec![0u8; 2048 * 64])),
            sector_size: 2048,
        };
        bus.mount(SCSI_TARGET_CDROM, Box::new(cd)).unwrap();
        let mut data = [0u8; 36];
        let mut cdb = [0u8; 16];
        cdb[0] = OP_INQUIRY;
        cdb[4] = 36;
        let r = bus.exec_cdb(SCSI_TARGET_CDROM, &cdb, &mut data);
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