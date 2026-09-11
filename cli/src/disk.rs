//! Disk-image mounting for the CLI.
//!
//! - Hard disk (SCSI target 1): `.raw` / `.img` (512-byte sectors) and `.chd`
//!   (via `libchdman-rs`).
//! - CD-ROM (SCSI target 6): `.iso` / `.img` (2048-byte MODE1 sectors).
//!
//! When no path is given on the command line, an [`rfd`] file dialog picks one.

use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use libchdman_rs::hd::HdImage;
use o2rust::storage::{BlockDevice, RawImage};

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
pub fn open_cdrom(path: &Path) -> Result<Box<dyn BlockDevice>> {
    Ok(Box::new(
        RawImage::open_cd(path).with_context(|| format!("opening CD-ROM {}", path.display()))?,
    ))
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
}