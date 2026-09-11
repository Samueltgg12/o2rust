//! O2Rust command-line interface.
//!
//! Windowed mode (default): prints the ASCII logo, opens an OpenGL framebuffer
//! window (winit + glutin), streams keyboard/mouse events into the guest PS/2
//! ports, outputs emulated audio via cpal, and mounts disk images — chosen via
//! `rfd` file dialogs when not given on the command line.
//!
//! `--headless` runs the classic console front-end (stdin → UART1, UART2 →
//! stdout) without a window or disks.

mod args;
mod audio;
mod disk;
mod display;
mod input;
mod logo;

use std::io::{self, Read, Write};
use std::sync::mpsc;
use std::thread;

use anyhow::{Context, Result};
use clap::Parser;
use o2rust::system::Emulator;
use tracing::{error, info, warn};

use args::Args;

fn main() -> Result<()> {
    let args = Args::parse();

    // The tracing subscriber reads RUST_LOG (o2rust::log::init); honour the
    // `-l`/`--log-filter` argument by seeding it before init.
    unsafe {
        std::env::set_var("RUST_LOG", args.effective_log_filter());
    }
    o2rust::log::init();

    logo::print_banner();
    info!(version = %o2rust::VERSION, "starting O2Rust CLI");

    match run(args) {
        Ok(()) => Ok(()),
        Err(e) => {
            error!("{e:#}");
            Err(e)
        }
    }
}

fn run(args: Args) -> Result<()> {
    // Console channels for UART1 (console I/O) and UART2 (console output).
    let (uart1_tx, uart1_rx) = mpsc::channel::<u8>();
    let (uart2_tx, uart2_rx) = mpsc::channel::<u8>();
    // Clone for the headless stdin thread (kept until the emulator owns it).
    let uart1_tx_stdin = uart1_tx.clone();

    let mut emulator = Emulator::with_ram(args.ram_mb, uart1_tx, uart1_rx, uart2_tx);
    emulator
        .load_prom(&args.prom)
        .with_context(|| format!("failed to load PROM '{}'", args.prom))?;
    info!(pc = format_args!("0x{:08x}", emulator.pc()), "PROM loaded, CPU reset");

    mount_disks(&mut emulator, &args)?;

    if args.headless {
        run_headless(emulator, &args, uart2_rx, uart1_tx_stdin)
    } else {
        run_windowed(emulator, &args)
    }
}

/// Mount the hard disk and CD-ROM, prompting with file dialogs when no paths
/// were given and we're allowed to block on user input.
fn mount_disks(emulator: &mut Emulator, args: &Args) -> Result<()> {
    let interactive = !args.headless;

    let hd = match &args.hard_disk {
        Some(path) => disk::open_hard_disk(std::path::Path::new(path))
            .with_context(|| format!("failed to open hard disk '{path}'"))?,
        None if interactive => match disk::pick_image(
            "Select the SCSI0 hard disk image",
            &["raw", "img", "chd"],
            "hard-disk.img",
        )? {
            Some(path) => disk::open_hard_disk(&path)
                .with_context(|| format!("failed to open hard disk '{}'", path.display()))?,
            None => {
                info!("no hard disk mounted (dialog canceled)");
                return Ok(());
            }
        },
        None => {
            info!("no hard disk (none given)");
            return Ok(());
        }
    };
    emulator.mount_hard_disk(hd).context("failed to mount hard disk")?;

    let cd = match &args.cdrom {
        Some(path) => disk::open_cdrom(std::path::Path::new(path))
            .with_context(|| format!("failed to open CD-ROM '{path}'"))?,
        None if interactive => {
            match disk::pick_image("Select the SCSI6 CD-ROM image", &["iso", "img"], "install.iso")? {
                Some(path) => disk::open_cdrom(&path)
                    .with_context(|| format!("failed to open CD-ROM '{}'", path.display()))?,
                None => {
                    info!("no CD-ROM mounted (dialog canceled)");
                    return Ok(());
                }
            }
        }
        None => {
            info!("no CD-ROM (none given)");
            return Ok(());
        }
    };
    emulator.mount_cdrom(cd).context("failed to mount CD-ROM")?;

    Ok(())
}

/// Headless mode: run the PROM with UART console I/O on stdin/stdout.
fn run_headless(
    mut emulator: Emulator,
    args: &Args,
    uart2_rx: mpsc::Receiver<u8>,
    uart1_tx_stdin: mpsc::Sender<u8>,
) -> Result<()> {
    info!("headless mode — console on stdin/stdout (hang up to exit)");

    // stdin → UART1 (console input)
    thread::spawn(move || {
        let mut stdin = io::stdin();
        let mut buf = [0u8; 1];
        loop {
            match stdin.read_exact(&mut buf) {
                Ok(_) if uart1_tx_stdin.send(buf[0]).is_err() => break,
                Ok(_) => {}
                Err(_) => break,
            }
        }
    });

    // UART2 → stdout (console output)
    thread::spawn(move || {
        let mut stdout = io::stdout();
        while let Ok(byte) = uart2_rx.recv() {
            if stdout.write_all(&[byte]).is_err() || stdout.flush().is_err() {
                break;
            }
        }
    });

    if args.steps > 0 {
        emulator.run(args.steps);
        info!(
            pc = format_args!("0x{:08x}", emulator.pc()),
            "executed {} instructions",
            args.steps
        );
    } else {
        info!("running until stopped (Ctrl+C to quit)");
        loop {
            emulator.run(1_000_000);
        }
    }
    Ok(())
}

/// Windowed mode: OpenGL framebuffer + input + audio.
fn run_windowed(mut emulator: Emulator, args: &Args) -> Result<()> {
    let audio = if args.no_audio {
        None
    } else {
        match emulator.take_audio_consumer() {
            Some(consumer) => match audio::start(consumer) {
                Ok(audio) => {
                    info!("audio output started");
                    Some(audio)
                }
                Err(e) => {
                    warn!("audio unavailable, continuing silently: {e:#}");
                    None
                }
            },
            None => {
                warn!("no audio ring available from core, continuing silently");
                None
            }
        }
    };

    display::run(emulator, audio)
}