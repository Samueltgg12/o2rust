libchdman-rs

A Rust wrapper for the official MAME CHD core (chd_file).

This crate provides native support for reading, writing, and managing CHD (Compressed Hunks of Data) containers by wrapping the original MAME C++ source code. It ensures 100% feature parity with chdman by using the same core logic.
Features

    Native CHD Support: Seek, read, and write at both hunk and byte levels.
    Metadata Management: Add, read, and delete CHD metadata.
    Custom I/O: Implement the ChdIo trait to provide your own I/O backends (e.g., in-memory, network, or custom encrypted filesystems).
    Asynchronous Compression: High-level ChdCompressor API for creating compressed CHDs from custom data sources.
    chdman parity: hd, dvd, cd, and copy modules cover createhd/extracthd, createdvd/extractdvd, createcd/extractcd, and copy byte-for-byte.
    Zero Re-implementation: Directly wraps MAME's libutil/chd.cpp.

For the full API surface and a chdman-flag-by-flag mapping see docs/format-modules.md and docs/chdman-mapping.md.
Choosing between crates.io and git

libchdman-rs is published to crates.io in a slim form that contains only the Rust wrapper code. The ~900 MB MAME C++ source is not shipped in the crates.io tarball — published crates can't be that large, and most consumers don't need to compile MAME from source.
Use crates.io (recommended for most consumers)

If you don't need to compile MAME from source — and you almost never do — depend on the crates.io version and enable the prebuilt feature:

[dependencies]
libchdman-rs = { version = "0.289", features = ["prebuilt"] }

The prebuilt feature downloads a pre-built static archive matching your target triple from this crate's GitHub Releases. See the "Pre-built static archives" section below for supported triples, glibc floors, and escape hatches.

Without features = ["prebuilt"], the build script aborts with an actionable message — the crates.io tarball has no C++ source for a local build to consume.
Use the git dependency (for source builds)

If you need to compile MAME's C++ from source — debugging the wrapped C++, custom MAME patches, or a target triple not covered by the prebuilt matrix — depend on the git repo:

[dependencies]
libchdman-rs = { git = "https://github.com/danifunker/libchdman-rs", tag = "v0.289.0" }

The git dependency includes the full vendored MAME source tree (~1 GB), so cold builds are slow.
Summary
Need 	Use this
Fast CI, prebuilt target 	crates.io + prebuilt
Custom MAME modifications 	git + source build
Unusual target not in prebuilt matrix 	git + source build
Reproducible historic builds 	git, pinned by tag
Usage
Opening a CHD

use libchdman_rs::Chd;

let chd = Chd::open("game.chd", false, None).expect("Failed to open CHD");
let info = chd.info().expect("info");
println!("Version: {}, logical bytes: {}", info.version, info.logical_bytes);
println!("CD: {}, DVD: {}, HD: {}", info.is_cd, info.is_dvd, info.is_hd);

Creating a hard-disk CHD

use libchdman_rs::hd::{create_from_path, HdCreateOptions};
use std::path::Path;

create_from_path(
    Path::new("disk.img"),
    Path::new("disk.chd"),
    HdCreateOptions::default(),
    &mut |p| eprintln!("{}/{}", p.bytes_done, p.bytes_total),
    &|| false,
)?;

Creating a CD CHD from a CUE

use libchdman_rs::cd::{create_from_cue, CdCreateOptions};
use libchdman_rs::parse_codec_spec;
use std::path::Path;

create_from_cue(
    Path::new("game.cue"),
    Path::new("game.chd"),
    CdCreateOptions {
        codecs: parse_codec_spec("cdlz,cdfl,cdzl")?,
        ..CdCreateOptions::default()
    },
    &mut |_| {},
    &|| false,
)?;

Reading a CD CHD as a cooked ISO stream

For MODE1 / MODE1_RAW CD tracks, CdCookedReader exposes the 2048-byte/sector user data as a Read + Seek stream — useful for browsing the ISO9660 / UDF filesystem inside a CD CHD without extracting to a temp file first. Length is track.frames * 2048; sync, header, and ECC bytes are stripped by MAME regardless of whether the CHD stored the track raw or cooked.

use libchdman_rs::cd::CdCookedReader;
use libchdman_rs::Chd;
use std::io::{Read, Seek, SeekFrom};

let chd = Chd::open("game.chd", false, None)?;
let mut reader = CdCookedReader::open(chd)?;

// Jump to the ISO9660 Primary Volume Descriptor (LBA 16, byte 16 * 2048).
reader.seek(SeekFrom::Start(16 * 2048))?;
let mut pvd = [0u8; 2048];
reader.read_exact(&mut pvd)?;
assert_eq!(&pvd[1..6], b"CD001");

// Hand `reader` to any ISO9660 / UDF parser that wants Read + Seek.
let total_bytes = reader.len();

For multi-track CHDs (PSX / Saturn / mixed-mode PC CDs: one data track plus one or more audio tracks), use CdCookedReader::open_track(chd, track_index) to pick which track to expose. Position 0 corresponds to the start of the selected track's user data, not the start of the CHD. The track must be MODE1 / MODE1_RAW; audio and Mode 2 variants return Err(ChdError::UnsupportedFormat).

// PSX game: track 0 is data, tracks 1..N are audio.
let chd = Chd::open("psx-game.chd", false, None)?;
let data_reader = CdCookedReader::open_track(chd, 0)?;

CdCookedReader::open (no _track) returns Err(ChdError::UnsupportedFormat) for multi-track CHDs as a back-compat guard; pick a track explicitly with open_track instead. Out-of-range track_index returns Err(ChdError::InvalidData). Use Chd::info().is_cd and cd::list_tracks to inspect the TOC before opening. into_inner() recovers the owned Chd if you need it back.
Runtime hard-disk writes (MAME-style)

HdImage is a block-device view over a hard-disk CHD — the same surface MAME's harddisk_image_device exposes to a running emulated machine. Writes to a HdImage go straight back into the CHD via write_bytes, which is exactly what MAME does when an emulated guest (e.g. a Macintosh LC) writes to its CHD hard disk.

For an uncompressed CHD, open it writeable in place:

use libchdman_rs::hd::HdImage;
use std::path::Path;

let mut img = HdImage::open(Path::new("mac_lc.chd"))?;
let ss = img.sector_size() as usize;

let mut buf = vec![0u8; ss];
img.read_sector(0, &mut buf)?;       // read MBR / boot sector
buf[510] = 0x55;
buf[511] = 0xAA;
img.write_sector(0, &buf)?;          // persisted on drop

For a compressed CHD, MAME's runtime strategy is to keep the parent CHD untouched and route every write into a fresh uncompressed diff child. HdImage::open_with_diff does the same:

use libchdman_rs::hd::HdImage;
use std::path::Path;

// Parent stays read-only and untouched. Diff is uncompressed and
// linked to the parent by SHA-1.
let mut img = HdImage::open_with_diff(
    Path::new("mac_lc.chd"),       // compressed parent
    Path::new("mac_lc.diff.chd"),  // freshly created diff
)?;
img.write_sector(42, &[0xAB; 512])?;

// Later, re-attach to the existing diff:
let img = HdImage::reopen_diff(
    Path::new("mac_lc.chd"),
    Path::new("mac_lc.diff.chd"),
)?;

Chd::create_with_parent is the lower-level primitive if you want to build the diff yourself (it wraps MAME's chd_file::create(filename, logicalbytes, hunkbytes, compression, parent) overload). Pass compression = [0; 4] for an uncompressed diff, which is the only configuration that supports per-hunk runtime writes.

Caveats

    Writes only work on uncompressed CHDs (or uncompressed diffs against a compressed parent). MAME's write_hunk / write_bytes reject compressed targets — this is a CHD-format constraint, not a wrapper limitation.
    HdImage::open_with_diff fails if the diff path already exists. Use reopen_diff to re-attach to an existing diff.
    HdImage validates buf.len() == sector_size() and lba < sector_count(); both fail with ChdError::InvalidData since the underlying error enum has no InvalidParameter variant.
    The diff inherits logical size, hunk size, and unit size from the parent — you can't resize on attach.
    For diff-mode images the HdImage owns the parent Chd handle internally and drops the child first, because MAME's chd_file stores m_parent as a non-owning aliasing shared_ptr (chd.cpp L681,L841). Don't extract the inner Chd and outlive the wrapper — use as_chd() / as_chd_mut() to reach lower-level APIs while the HdImage is in scope.

Re-compressing an existing CHD

use libchdman_rs::copy::{copy, CopyOptions};
use libchdman_rs::{CHD_CODEC_LZMA, CHD_CODEC_ZLIB};
use std::path::Path;

copy(
    Path::new("old.chd"),
    Path::new("new.chd"),
    CopyOptions {
        hunk_size: None,
        codecs: [CHD_CODEC_LZMA, CHD_CODEC_ZLIB, 0, 0],
    },
    &mut |_| {},
    &|| false,
)?;

Custom I/O

use libchdman_rs::{Chd, ChdIo};
use std::io::{Read, Write, Seek, SeekFrom};

struct MyCustomIo { ... }
impl Read for MyCustomIo { ... }
impl Write for MyCustomIo { ... }
impl Seek for MyCustomIo { ... }
// ChdIo is automatically implemented for Read + Write + Seek

let io = MyCustomIo::new();
let chd = Chd::open_custom(io, false, None).expect("Failed to open");

Pre-built static archives (faster CI builds)

libchdman-rs wraps MAME's C++ core via the cc crate, which can take several minutes on cold builds. For CI and other build-time-sensitive consumers, every tagged release ships pre-built static archives that skip the C++ compile entirely.

Enable the prebuilt feature in your Cargo.toml:

libchdman-rs = { version = "0.289", features = ["prebuilt"] }

On cargo build, the build script downloads the archive matching your target triple from this repo's GitHub Releases, verifies its sha256, and links it statically.
Supported targets
Triple 	Notes
x86_64-unknown-linux-gnu 	Three glibc floors (see below)
aarch64-unknown-linux-gnu 	Three glibc floors
riscv64gc-unknown-linux-gnu 	Two glibc floors (2.35, 2.39); cross
x86_64-apple-darwin 	MACOSX_DEPLOYMENT_TARGET=10.13
aarch64-apple-darwin 	MACOSX_DEPLOYMENT_TARGET=10.13
aarch64-apple-ios 	Device; IPHONEOS_DEPLOYMENT_TARGET=13.0
aarch64-apple-ios-sim 	Simulator on Apple Silicon
x86_64-pc-windows-msvc 	
i686-pc-windows-msvc 	
aarch64-pc-windows-msvc 	Native windows-11-arm build

On iOS the archive links into an app the same way it does anywhere else — libc++ is linked automatically. The library itself is filesystem-only (no fork/exec/dlopen, nothing an App Store review objects to), and its compression worker pool is already capped at 4 threads, which suits iOS's tighter memory ceiling. File access is subject to the app sandbox, so hand it paths inside your container.

The aarch64-apple-ios device archive is validated in CI by build, required-symbol check, and a Mach-O platform check — an iOS device binary cannot be executed on a CI runner. The aarch64-apple-ios-sim archive is additionally run in a booted simulator, and both are built from the same sources with the same compiler.

Targets not in the list (musl, BSD, anything exotic) fall back to source build automatically when LIBCHDMAN_PREBUILT_FALLBACK=1 is set; otherwise the build will fail with a clear message.
Linux: picking a glibc floor

Linux archives are built against two glibc versions, and you choose which floor your binary should require:
LIBCHDMAN_GLIBC 	glibc floor 	Built on Ubuntu 	Works on (examples)
2.35 (default) 	2.35 	22.04 	Debian 12, RHEL 9, modern distros
2.39 	2.39 	24.04 	Newest distros only

If unset (or auto), the build script picks 2.35 — the modern sweet spot. macOS and Windows builds ignore this variable.

    The previous 2.31 floor (Debian 11 / RHEL 8 era) was dropped when GitHub retired the ubuntu-20.04 runner image. Consumers on older distros should fall back to source build with LIBCHDMAN_PREBUILT_FALLBACK=1.

Escape hatches
Env var 	Effect
LIBCHDMAN_FORCE_SOURCE=1 	Always compile from source, even with prebuilt on
LIBCHDMAN_PREBUILT_FALLBACK=1 	Silently fall back to source if the download fails
Caching

The downloaded archive is cached under:

    Linux: ~/.cache/libchdman-rs/<version>/ (or $XDG_CACHE_HOME/libchdman-rs/<version>/)
    macOS: ~/Library/Caches/libchdman-rs/<version>/
    Windows: %LOCALAPPDATA%\libchdman-rs\<version>\

cargo clean doesn't invalidate the cache; bumping the crate version does.
Versioning

The crate version matches the MAME release it embeds — for example, 0.289.0 embeds MAME 0.289. If a wrapper-only fix is needed against the same MAME version, the patch component is bumped (0.289.1, 0.289.2, ...).

Notable changes per release are in CHANGELOG.md.
Testing

cargo test runs the lib-only suite (round-trip parity, metadata, custom I/O). It never invokes the chdman binary and has no system dependencies beyond a C++20 toolchain.

For byte-for-byte parity verification against the upstream chdman tool, enable the chdman_compat_tests feature locally:

cargo test --features chdman_compat_tests

This is dev-local only. The feature gates tests that shell out to chdman on $PATH; CI does not install chdman, so these tests never run there. Use them on demand during development cycles when you want to confirm that the output of this crate matches what chdman produces for the same input.
Build Requirements

    Rust 1.75+
    C++20 compliant compiler (GCC 10+, Clang 10+, MSVC 2019+)

License

This crate is licensed under the BSD 3-Clause License.

The MAME project as a whole is distributed under GPL-2.0+, but that is an aggregate license. The specific CHD core files this crate vendors and compiles (src/lib/util/chd.cpp, chdcodec.cpp, cdrom.cpp, and their supporting utility files) each carry an individual // license:BSD-3-Clause SPDX header (md5.cpp is Public Domain). The bundled third-party codecs are likewise permissive (zlib, libFLAC — BSD-3, LZMA — Public Domain, Zstandard — BSD-3, utf8proc — MIT). None of the compiled code is GPL-licensed, so the wrapper and the prebuilt static archives are distributable under BSD-3-Clause terms.