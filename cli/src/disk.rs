//! Disk-image mounting for the CLI.
//!
//! - Hard disk (SCSI0 target 1): `.raw` / `.img` (512-byte sectors) and `.chd`
//!   (via `libchdman-rs`).
//! - CD-ROM (SCSI0 target 4): `.iso` / `.img` (2048-byte MODE1 sectors).
//!
//! IRIX install and boot media are **not** ISO9660 CD-ROMs: they are EFS disks
//! carrying a 512-byte SGI disk label / volume header (`0x0be5a941`) and use
//! 512-byte logical sectors.  [`open_cdrom`] sniffs the image head and uses
//! 512-byte sectors for such media, so the PROM can read their volume header
//! and EFS filesystems just like a hard disk.
//!
//! When no path is given on the command line, an [`rfd`] file dialog picks one.

use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use libchdman_rs::hd::HdImage;
use o2rust::storage::{BlockDevice, RawImage};

/// Endianness-independent magic of the SGI disk label / volume header
/// (`0x0be5a941` as stored at byte 0 of an EFS disk).
const SGI_LABEL_MAGIC: [u8; 4] = [0x0b, 0xe5, 0xa9, 0x41];

/// Extension (lower-cased, no dot) of `path`, if any.
pub fn extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
}

/// Open a hard-disk image (raw/img/chd).
pub fn open_hard_disk(path: &Path) -> Result<Box<dyn BlockDevice>> {
    match extension(path).as_deref() {
        Some("chd") => open_chd(path),
        _ => Ok(Box::new(
            RawImage::open_disk(path).with_context(|| format!("opening hard disk {}", path.display()))?,
        )),
    }
}

/// Open a CD-ROM image (iso/img).
///
/// EFS-based IRIX media are detected by their SGI disk-label magic and
/// presented with 512-byte sectors; plain ISO9660 images keep MODE1 2048-byte
/// sectors.
pub fn open_cdrom(path: &Path) -> Result<Box<dyn BlockDevice>> {
    let sector_size = sniff_cd_sector_size(path)?;
    Ok(Box::new(
        RawImage::open(path, sector_size)
            .with_context(|| format!("opening CD-ROM {}", path.display()))?,
    ))
}

/// Pick the sector size for a `-c` image: 512 for EFS/IRIX install media
/// (SGI disk label magic at byte 0), otherwise 2048 (ISO9660 MODE1).
fn sniff_cd_sector_size(path: &Path) -> Result<u32> {
    use std::io::Read;
    let mut file = std::fs::File::open(path)
        .with_context(|| format!("opening {}", path.display()))?;
    let mut head = [0u8; 4];
    let n = file.read(&mut head)?;
    if n == head.len() && head == SGI_LABEL_MAGIC {
        Ok(512)
    } else {
        Ok(2048)
    }
}

/// Opens a CHD hard-disk image through libchdman.
fn open_chd(path: &Path) -> Result<Box<dyn BlockDevice>> {
    let inner = HdImage::open(path).map_err(|e| anyhow!("opening CHD {}: {:?}", path.display(), e))?;
    Ok(Box::new(ChdDevice {
        inner,
        name: path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("chd image")
            .to_string(),
    }))
}

/// A CHD-backed hard disk.
struct ChdDevice {
    inner: HdImage,
    name: String,
}

// SAFETY: the CHD library is only accessed from the emulator thread; no host
// code shares the `*mut ChdFile` across threads while a device is mounted.
unsafe impl Send for ChdDevice {}

impl BlockDevice for ChdDevice {
    fn name(&self) -> &str {
        &self.name
    }

    fn sector_size(&self) -> u32 {
        self.inner.sector_size()
    }

    fn sector_count(&self) -> u64 {
        self.inner.sector_count()
    }

    fn is_read_only(&self) -> bool {
        true // writes through libchdman would rewrite the image; not wired yet
    }

    fn read_sector(&mut self, lba: u64, buf: &mut [u8]) -> std::io::Result<()> {
        self.inner
            .read_sector(lba, buf)
            .map_err(|e| std::io::Error::other(format!("CHD read at LBA {lba}: {e:?}")))
    }

    fn write_sector(&mut self, _lba: u64, _data: &[u8]) -> std::io::Result<()> {
        Err(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "CHD images are read-only in this build",
        ))
    }
}

/// Ask the user for a disk-image path via an `rfd` file dialog.
pub fn pick_image(title: &str, extensions: &[&str], file_name: &str) -> Result<Option<PathBuf>> {
    let mut dialog = rfd::FileDialog::new()
        .set_title(title)
        .set_directory(std::env::current_dir().unwrap_or_default());
    if !extensions.is_empty() {
        dialog = dialog.add_filter(title, extensions).set_file_name(file_name);
    }
    Ok(dialog.pick_file())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extension_lowercases() {
        assert_eq!(extension(Path::new("foo.CHD")), Some("chd".to_string()));
        assert_eq!(extension(Path::new("noext")), None);
    }

    #[test]
    fn efs_install_media_is_512_byte_sectors() {
        let dir = std::env::temp_dir();
        let path = dir.join(format!("o2rust_sniff_{}.img", std::process::id()));

        // SGI disk label magic at byte 0 → EFS install disk → 512-byte sectors.
        std::fs::write(&path, b"\x0b\xe5\xa9\x41").unwrap();
        assert_eq!(sniff_cd_sector_size(&path).unwrap(), 512);

        // Anything else falls back to ISO9660 MODE1 2048-byte sectors.
        std::fs::write(&path, b"\x01CD001").unwrap();
        assert_eq!(sniff_cd_sector_size(&path).unwrap(), 2048);

        std::fs::remove_file(&path).unwrap();
    }
}