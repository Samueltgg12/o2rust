//! Storage devices — block-device abstraction and disk-image backends.
//!
//! The O2's UltraWide SCSI bus (Adaptec AIC-7880) has its hard disk on
//! **target 1** and the CD-ROM drive on **target 6**. Guest block access goes
//! through the [`BlockDevice`] trait; this module provides the file-backed
//! implementations:
//!
//! - [`RawImage`] — raw byte-for-byte block dumps (`.raw` / `.img`), used for
//!   both hard disks (512-byte sectors) and CD-ROMs (2048-byte MODE1 sectors).
//! - CHD hard-disk images (`.chd`) are handled by the front-end
//!   (`libchdman-rs`), which exposes its own `BlockDevice`-compatible reader.
//!
//! Register/behavior references: Linux `drivers/scsi/aic7xxx/`, IRIX
//! `adp78.h`/`adp78.c`, and `docs/io.md` (SCSI section).

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

/// A block device (one SCSI target) as seen by the guest.
///
/// Access is sector based. A sector is an opaque run of bytes
/// ([`BlockDevice::sector_size`] bytes); framing (SCSI CDBs, EDC) is the
/// SCSI layer's job, not the image's.
pub trait BlockDevice: Send {
    /// Human-readable name of the backing image (usually the file name).
    fn name(&self) -> &str;

    /// Size of one sector, in bytes (512 for hard disks, 2048 for CD-ROMs).
    fn sector_size(&self) -> u32;

    /// Number of sectors on the device.
    fn sector_count(&self) -> u64;

    /// Whether the image is read-only (CD-ROMs, and non-opened-for-write raws).
    fn is_read_only(&self) -> bool;

    /// Read one sector at `lba`. `buf` must be at least `sector_size()` bytes.
    fn read_sector(&mut self, lba: u64, buf: &mut [u8]) -> io::Result<()>;

    /// Write one sector at `lba` from `data`. Fails on read-only devices.
    fn write_sector(&mut self, lba: u64, data: &[u8]) -> io::Result<()>;

    /// Convenience: read `count * sector_size()` bytes starting at `lba`.
    ///
    /// The default implementation loops [`Self::read_sector`]; images may
    /// override it with a more efficient bulk read.
    fn read_blocks(&mut self, lba: u64, count: u32, buf: &mut [u8]) -> io::Result<()> {
        let ss = self.sector_size() as usize;
        let need = count as usize * ss;
        if buf.len() < need {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "output buffer too small for read_blocks",
            ));
        }
        for i in 0..count as usize {
            self.read_sector(lba + i as u64, &mut buf[i * ss..(i + 1) * ss])?;
        }
        Ok(())
    }
}

/// A raw, byte-for-byte disk image (`.raw`, `.img`, `.iso`).
///
/// Behind a plain `std::fs::File`, so it can grow beyond RAM without being
/// copied into memory. If the file can be opened read-write, secondary writes
/// go straight back to disk; otherwise the image is read-only.
pub struct RawImage {
    file: File,
    path: PathBuf,
    sector_size: u32,
    sector_count: u64,
    read_only: bool,
}

impl RawImage {
    /// Open a raw image as a **hard disk** (512-byte sectors).
    pub fn open_disk(path: impl AsRef<Path>) -> io::Result<Self> {
        Self::open(path, 512)
    }

    /// Open a raw image as a **CD-ROM** (2048-byte MODE1 sectors).
    pub fn open_cd(path: impl AsRef<Path>) -> io::Result<Self> {
        Self::open(path, 2048)
    }

    /// Open a raw image with an explicit sector size.
    pub fn open(path: impl AsRef<Path>, sector_size: u32) -> io::Result<Self> {
        let path = path.as_ref();
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)
            .or_else(|_| File::open(path))?;
        let read_only = file
            .metadata()
            .map(|m| m.permissions().readonly())
            .unwrap_or(true);

        let len = file.metadata()?.len();
        if sector_size == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "sector size must be non-zero",
            ));
        }
        Ok(Self {
            file,
            path: path.to_path_buf(),
            sector_size,
            sector_count: len / u64::from(sector_size),
            read_only,
        })
    }

    /// The backing file path.
    pub fn path(&self) -> &Path {
        &self.path
    }

    fn offset(&self, lba: u64) -> io::Result<u64> {
        lba.checked_mul(u64::from(self.sector_size)).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "sector offset overflow",
            )
        })
    }
}

impl BlockDevice for RawImage {
    fn name(&self) -> &str {
        self.path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("raw image")
    }

    fn sector_size(&self) -> u32 {
        self.sector_size
    }

    fn sector_count(&self) -> u64 {
        self.sector_count
    }

    fn is_read_only(&self) -> bool {
        self.read_only
    }

    fn read_sector(&mut self, lba: u64, buf: &mut [u8]) -> io::Result<()> {
        if buf.len() < self.sector_size as usize {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "sector buffer too small",
            ));
        }
        if lba >= self.sector_count {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "read past end of image",
            ));
        }
        self.file.seek(SeekFrom::Start(self.offset(lba)?))?;
        self.file.read_exact(&mut buf[..self.sector_size as usize])
    }

    fn write_sector(&mut self, lba: u64, data: &[u8]) -> io::Result<()> {
        if self.read_only {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "image opened read-only",
            ));
        }
        if data.len() < self.sector_size as usize {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "sector buffer too small",
            ));
        }
        if lba >= self.sector_count {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "write past end of image",
            ));
        }
        self.file.seek(SeekFrom::Start(self.offset(lba)?))?;
        self.file.write_all(&data[..self.sector_size as usize])
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::sync::Arc;

    // A tiny in-memory `BlockDevice` used to exercise the scsi bus / mount
    // plumbing without touching the filesystem.
    pub struct MemDisk {
        pub name: String,
        pub data: Arc<std::sync::Mutex<Vec<u8>>>,
        pub sector_size: u32,
    }

    impl BlockDevice for MemDisk {
        fn name(&self) -> &str {
            &self.name
        }
        fn sector_size(&self) -> u32 {
            self.sector_size
        }
        fn sector_count(&self) -> u64 {
            (self.data.lock().unwrap().len() / self.sector_size as usize) as u64
        }
        fn is_read_only(&self) -> bool {
            false
        }
        fn read_sector(&mut self, lba: u64, buf: &mut [u8]) -> io::Result<()> {
            let data = self.data.lock().unwrap();
            let start = (lba * u64::from(self.sector_size)) as usize;
            let end = start + self.sector_size as usize;
            if end > data.len() {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "sector past end"));
            }
            buf[..self.sector_size as usize].copy_from_slice(&data[start..end]);
            Ok(())
        }
        fn write_sector(&mut self, lba: u64, data: &[u8]) -> io::Result<()> {
            let mut inner = self.data.lock().unwrap();
            let start = (lba * u64::from(self.sector_size)) as usize;
            let end = start + self.sector_size as usize;
            if end > inner.len() {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "sector past end"));
            }
            inner[start..end].copy_from_slice(&data[..self.sector_size as usize]);
            Ok(())
        }
    }

    #[test]
    fn raw_image_round_trip() {
        let dir = std::env::temp_dir();
        let path = dir.join(format!("o2rust_raw_test_{}.img", std::process::id()));
        // 4 sectors of 512 bytes.
        let mut backing = vec![0u8; 4 * 512];
        backing[0] = 0x55;
        backing[511] = 0xaa;
        std::fs::write(&path, &backing).unwrap();

        let mut img = RawImage::open(&path, 512).unwrap();
        assert_eq!(img.sector_size(), 512);
        assert_eq!(img.sector_count(), 4);

        let mut sector = vec![0u8; 512];
        img.read_sector(0, &mut sector).unwrap();
        assert_eq!(sector[0], 0x55);
        assert_eq!(sector[511], 0xaa);

        let mut new_word = b"O2RUST".to_vec();
        new_word.resize(512, 0);
        img.write_sector(1, &new_word).unwrap();
        img.read_sector(1, &mut sector).unwrap();
        assert_eq!(&sector[..6], b"O2RUST");

        // Reads past the end fail cleanly.
        assert!(img.read_sector(4, &mut sector).is_err());

        // A read-only file (no write bit) is surfaced as read-only.
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o444)).unwrap();
            let mut ro = RawImage::open(&path, 512).unwrap();
            assert!(ro.is_read_only());
            assert!(ro.write_sector(1, &new_word).is_err());
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).unwrap();
        }

        std::fs::remove_file(&path).unwrap();
    }

    #[test]
    fn raw_image_zero_sector_is_invalid() {
        let dir = std::env::temp_dir();
        let path = dir.join(format!("o2rust_raw_bad_{}.img", std::process::id()));
        std::fs::write(&path, [0u8; 512]).unwrap();
        assert!(RawImage::open(&path, 0).is_err());
        std::fs::remove_file(&path).unwrap();
    }
}