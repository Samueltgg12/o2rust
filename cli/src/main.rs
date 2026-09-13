//! O2Rust command-line interface.
//!
//! Windowed mode (default): prints the ASCII logo, opens an OpenGL framebuffer
//! window (winit + glutin), streams keyboard/mouse events into the guest PS/2
//! ports, outputs emulated audio via cpal, and mounts disk images — chosen via
//! `rfd` file dialogs when not given on the command line.
//!
//! `--headless` runs the serial console front-end: the PROM's UART1 system
//! console is attached to stdin/stdout (raw terminal mode, full interaction:
//! command monitor, "System Maintenance Menu", etc.) without a window.

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
    //
    // In headless mode stdout is the emulated machine's serial console and
    // stderr is ours; the I/O register-poke warnings the PROM otherwise
    // triggers hundreds of times per boot (KBD/MS probes etc.) bury it, so
    // the default headless filter keeps only CPU-level diagnostics (which
    // signal real emulation bugs). Anything else is still available with
    // `-l`/`-v`.
    let filter = if args.headless && !args.verbose && args.log_filter_is_default() {
        "o2rust::cpu=warn".to_string()
    } else {
        args.effective_log_filter()
    };
    unsafe {
        std::env::set_var("RUST_LOG", filter);
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
    // Console channels for UART1 (the O2 system console): `stdin_*` carries
    // host→guest bytes, `console_*` carries guest→host console output.
    let (stdin_tx, stdin_rx) = mpsc::channel::<u8>();
    let (console_tx, console_rx) = mpsc::channel::<u8>();

    let mut emulator = Emulator::with_console(args.ram_mb, stdin_rx, Some(console_tx));
    emulator
        .load_prom(&args.prom)
        .with_context(|| format!("failed to load PROM '{}'", args.prom))?;
    info!(pc = format_args!("0x{:08x}", emulator.pc()), "PROM loaded, CPU reset");

    mount_disks(&mut emulator, &args)?;

    if args.headless {
        run_headless(emulator, &args, console_rx, stdin_tx)
    } else {
        // Windowed mode gets the guest console mirrored to stdout, too — the
        // GBE graphics console is not wired yet, so the serial console is the
        // only way to see what the PROM is saying.
        spawn_console_printer(console_rx);
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

/// Stream guest console bytes to stdout.
fn spawn_console_printer(console_rx: mpsc::Receiver<u8>) {
    thread::spawn(move || {
        let mut stdout = io::stdout();
        while let Ok(byte) = console_rx.recv() {
            if stdout.write_all(&[byte]).is_err() || stdout.flush().is_err() {
                break;
            }
        }
    });
}

/// Put stdin into non-canonical, no-echo mode while the guard lives, so
/// single keypresses reach the PROM console without waiting for Enter.
/// ISIG is deliberately kept: Ctrl+C still terminates the emulator.
/// Returns `None` when stdin is not a TTY (e.g. a pipe), which is fine: the
/// caller still gets bytes, just line-buffered.
struct RawModeGuard(libc::termios);

fn enter_raw_mode() -> Option<RawModeGuard> {
    unsafe {
        let mut term: libc::termios = std::mem::zeroed();
        if libc::tcgetattr(libc::STDIN_FILENO, &mut term) != 0 {
            return None; // not a terminal — keep cooked mode
        }
        let orig = term;
        // Non-canonical + no echo, but keep ISIG (Ctrl+C → SIGINT) and OPOST.
        term.c_lflag &= !(libc::ICANON | libc::ECHO);
        term.c_cc[libc::VMIN] = 1;
        term.c_cc[libc::VTIME] = 0;
        if libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &term) != 0 {
            return None;
        }
        Some(RawModeGuard(orig))
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        unsafe {
            libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &self.0);
        }
    }
}

/// Saved cooked-mode termios so the SIGINT handler can restore the terminal.
static SAVED_TERMIOS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
static mut SAVED_TERMIOS_CELL: std::mem::MaybeUninit<libc::termios> =
    std::mem::MaybeUninit::uninit();

unsafe extern "C" fn on_sigint(_sig: libc::c_int) {
    if SAVED_TERMIOS.load(std::sync::atomic::Ordering::Relaxed) != 0 {
        unsafe {
            let ptr = std::ptr::addr_of!(SAVED_TERMIOS_CELL).cast::<libc::termios>();
            libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &*ptr);
        }
    }
    std::process::exit(130);
}

/// Install a SIGINT handler that restores the terminal and exits.
fn install_sigint_handler() {
    unsafe {
        libc::signal(libc::SIGINT, on_sigint as libc::sighandler_t);
    }
}

/// Headless mode: run the PROM with the UART1 console on stdin/stdout.
fn run_headless(
    mut emulator: Emulator,
    args: &Args,
    console_rx: mpsc::Receiver<u8>,
    stdin_tx: mpsc::Sender<u8>,
) -> Result<()> {
    info!("headless mode — console on stdin/stdout (Ctrl+C to quit)");

    // Raw terminal mode so single keypresses reach the PROM immediately,
    // plus a SIGINT handler so Ctrl+C restores the terminal before exiting.
    let _raw = enter_raw_mode().inspect(|raw| {
        unsafe {
            let cell = &mut *std::ptr::addr_of_mut!(SAVED_TERMIOS_CELL);
            cell.write(raw.0);
        }
        SAVED_TERMIOS.store(1, std::sync::atomic::Ordering::Relaxed);
    });
    install_sigint_handler();

    // stdin → UART1 (console input)
    thread::spawn(move || {
        let mut stdin = io::stdin();
        let mut buf = [0u8; 64];
        loop {
            match stdin.read(&mut buf) {
                Ok(0) => break, // EOF
                Ok(n) => {
                    for &b in &buf[..n] {
                        if stdin_tx.send(b).is_err() {
                            return;
                        }
                    }
                }
                Err(_) => break,
            }
        }
    });

    // UART1 → stdout (console output)
    spawn_console_printer(console_rx);

    if args.steps > 0 {
        emulator.run(args.steps);
        info!(
            pc = format_args!("0x{:08x}", emulator.pc()),
            "executed {} instructions",
            args.steps
        );
    } else {
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