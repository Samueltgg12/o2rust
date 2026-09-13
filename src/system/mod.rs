//! System-level emulation: the system bus and the top-level [`Emulator`].
//!
//! The [`Emulator`] owns the CPU, memory map, and PROM, and drives execution.
//! It is the primary entry point used by the `cli` and `gui` front-ends.

pub mod bus;

use crate::cpu::{CpuModel, R5000};
use crate::io::scsi::{ScsiBus, SCSI_TARGET_CDROM, SCSI_TARGET_DISK};
use crate::memory::{DCache, MemoryMap};
use crate::prom::Prom;
use crate::storage::BlockDevice;
use crate::{ip32, log};

/// The top-level emulator.
///
/// Owns the CPU, memory, PROM, and SCSI bus, and provides a simple run loop.
pub struct Emulator {
    /// The MIPS CPU core.
    pub cpu: R5000,
    /// The physical memory map.
    pub memory: MemoryMap,
    /// The CPU's write-back primary data cache (persists across run calls).
    pub cache: DCache,
    /// The loaded PROM image (if any).
    pub prom: Option<Prom>,
    /// The CPU model being emulated.
    pub model: CpuModel,
    /// The SCSI bus (hard disk on target 1, CD-ROM on target 6).
    pub scsi: ScsiBus,
    /// Whether the emulator is running.
    running: bool,
}

impl Default for Emulator {
    fn default() -> Self {
        Self::new()
    }
}

/// KSEG1 (uncached) address of the warm-boot handoff stub in RAM.
const BOOT_STUB_VADDR: u32 = 0xa010_0000;
/// Offset (bytes) of the banner string within the stub.
const BOOT_STUB_MSG_OFFSET: u32 = 0x0034;

/// Minimal MIPS III warm-boot stub (big-endian).
///
/// Hand-assembled `bgez`-free print loop: loads the UART1 data register
/// address, `lbu`s the banner byte, `sb`s it out, then halts. Laid out at
/// `BOOT_STUB_VADDR` (offset 0x34 holds the string). PC-relative offsets in
/// the `imm` fields assume this exact placement.
const BOOT_STUB_INSTRUCTIONS: &[u32] = &[
    0x3c08bf39, // 0x00 lui $t0, 0xbf39
    0x35080007, // 0x04 ori $t0, $t0, 7        (0xbf390007 = UART1 data byte)
    0x3c09a010, // 0x08 lui $t1, 0xa010
    0x35290034, // 0x0c ori $t1, $t1, 0x34     (&banner)
    0x91240000, // 0x10 lbu $a0, 0($t1)
    0x10800005, // 0x14 beqz $a0, 0x2c        (end of string)
    0x25290001, // 0x18 addiu $t1, $t1, 1     (delay slot)
    0xa1040000, // 0x1c sb   $a0, 0($t0)
    0x1000fffb, // 0x20 b    0x10
    0x00000000, // 0x24 (padding)
    0x00000000, // 0x28 (padding)
    0x1000ffff, // 0x2c b    0x2c             (halt)
    0x00000000, // 0x30 (padding, banner at 0x34)
];

impl Emulator {
    /// Create a new emulator with `ram_mb` megabytes of RAM.
    pub fn new() -> Self {
        let (uart1_tx, uart1_rx) = std::sync::mpsc::channel();
        let (uart2_tx, uart2_rx) = std::sync::mpsc::channel();
        Self {
            cpu: R5000::new(),
            memory: MemoryMap::new(256, uart1_tx, uart1_rx, uart2_tx),
            cache: DCache::new(),
            prom: None,
            model: CpuModel::R5000,
            scsi: ScsiBus::new(),
            running: false,
        }
    }

    /// Create a new emulator with a specific amount of RAM (in MB) and console I/O channels for UARTs.
    /// UART1 is used for console input/output (bidirectional).
    /// UART2 is used for console output only (tx only).
    pub fn with_ram(
        ram_mb: u32,
        uart1_tx: std::sync::mpsc::Sender<u8>,
        uart1_rx: std::sync::mpsc::Receiver<u8>,
        uart2_tx: std::sync::mpsc::Sender<u8>,
    ) -> Self {
        Self {
            cpu: R5000::new(),
            memory: MemoryMap::new(ram_mb, uart1_tx, uart1_rx, uart2_tx),
            cache: DCache::new(),
            prom: None,
            model: CpuModel::R5000,
            scsi: ScsiBus::new(),
            running: false,
        }
    }

    /// Create a new emulator with a specific amount of RAM (in MB) without console I/O.
    pub fn with_ram_no_console(ram_mb: u32) -> Self {
        let (uart1_tx, uart1_rx) = std::sync::mpsc::channel();
        let (uart2_tx, uart2_rx) = std::sync::mpsc::channel();
        Self::with_ram(ram_mb, uart1_tx, uart1_rx, uart2_tx)
    }

    /// Create a new emulator wired for a live serial console on UART1.
    ///
    /// The O2 PROM uses UART1 as the system console (the "Option?" menu,
    /// command monitor, etc. all read/write it). `stdin_rx` carries
    /// host→guest console input bytes; `stdout_tx`, if given, receives every
    /// guest→host console byte (in addition to the internal channel drained
    /// by [`Emulator::drain_console_output`], which keeps working).
    pub fn with_console(
        ram_mb: u32,
        stdin_rx: std::sync::mpsc::Receiver<u8>,
        stdout_tx: Option<std::sync::mpsc::Sender<u8>>,
    ) -> Self {
        // UART1: guest input comes from `stdin_rx`; guest output is fanned
        // out to `stdout_tx` via `console_tx_ext`. The passthrough senders
        // have no host consumer in this configuration.
        let (uart1_tx, _unused_uart1_rx) = std::sync::mpsc::channel();
        let (uart2_tx, _unused_uart2_rx) = std::sync::mpsc::channel();
        let mut emu = Self::with_ram(ram_mb, uart1_tx, stdin_rx, uart2_tx);
        if let Some(tx) = stdout_tx {
            emu.memory.mace.isa_ext.uart1.set_console_tx_ext(tx);
        }
        emu
    }

    /// Load a PROM image from a file and map it into memory.
    pub fn load_prom(&mut self, path: &str) -> anyhow::Result<()> {
        let prom = Prom::from_file(path)?;
        log::info_msg(&format!("Loaded PROM: {path} ({} bytes)", prom.len()));

        // Map the PROM into the ROM window at the reset vector.
        let rom_base = ip32::PHYS_SYSTEM_ROM;
        self.memory.rom.load(0, prom.as_slice());

        // Reset the CPU to the reset vector.
        self.cpu.reset(ip32::PROM_RESET_VECTOR);

        self.prom = Some(prom);
        log::info_msg(&format!(
            "CPU reset to reset vector 0x{:08x}",
            ip32::PROM_RESET_VECTOR
        ));
        let _ = rom_base;

        // Install the warm-boot environment (GDA + boot stub) so the PROM's
        // `warm_start` can hand off and make boot progress on a headless O2.
        self.install_warm_boot_stub();
        Ok(())
    }

    /// Install the boot handoff environment the PROM's `warm_start` expects.
    ///
    /// On the real O2 a boot service (OS loader) fills the General Dispatch
    /// Address (GDA) in low RAM with the magic "XFER" and the entry point of
    /// the image to boot; the PROM validates the magic and `jr`s to the
    /// entry. With no disks attached, we provide a minimal stub in RAM that
    /// prints a marker to the console and halts, so the emulated boot has a
    /// defined end state. (See decompiled PROM `warm_start`, and
    /// `src/lib.rs` `ip32::GDA_*`.)
    pub fn install_warm_boot_stub(&mut self) {
        // GDA: magic at +0, jump address at +8.
        self.memory.write32(ip32::GDA_ADDR & !ip32::KSEG0, ip32::GDA_MAGIC);
        self.memory
            .write32((ip32::GDA_ADDR & !ip32::KSEG0) + ip32::GDA_ENTRY_OFFSET, BOOT_STUB_VADDR);

        // Copy the stub into RAM (physical 0x0010_0000, kseg1 0xa010_0000).
        const STUB_PHYS: u32 = 0x0010_0000;
        for (i, word) in BOOT_STUB_INSTRUCTIONS.iter().enumerate() {
            self.memory
                .write32(STUB_PHYS + (i as u32) * 4, *word);
        }
        const STUB_MSG: &[u8] = b"\r\nO2Rust: GDA boot handoff OK\r\n\0";
        for (i, b) in STUB_MSG.iter().enumerate() {
            self.memory
                .write8(STUB_PHYS + BOOT_STUB_MSG_OFFSET + (i as u32), *b);
        }
        log::info_msg(&format!("Warm-boot stub installed: GDA -> 0x{BOOT_STUB_VADDR:08x}"));
    }

    /// Reset the emulator (CPU + memory) to its power-on state.
    pub fn reset(&mut self) {
        self.cpu.reset(ip32::PROM_RESET_VECTOR);
        self.running = false;
        log::info_msg("Emulator reset");
    }

    /// Slice size for device-time advancement during long runs.
    ///
    /// Device clocks (CRIME timer, MACE UST, audio) are advanced from the
    /// CPU's cycle counter only on slice boundaries. The slice must be small
    /// enough that the PROM's short polled-delay loops (e.g. `us_delay(2)`
    /// spinning on the CRIME timer for ~130 ticks) make progress *within* a
    /// run; when the whole run was a single slice, every micro-delay cost an
    /// entire run chunk (~1M instructions) and boot effectively stalled.
    const RUN_SLICE: u64 = 8_192;

    /// Advance device clocks by the CPU cycles consumed since `before`,
    /// and service reset requests.
    fn tick_devices(&mut self, before: u64) {
        let delta = self.cpu.cycles().wrapping_sub(before);
        self.memory.crime_cpu.advance_time(delta);
        self.memory.audio_tick(delta);
        self.memory.mace_advance_ust(delta);
        self.memory.gbe.advance(delta);
        self.check_crime_reset();
    }

    /// Execute a single instruction.
    pub fn step(&mut self) {
        let before = self.cpu.cycles();
        let mut bus = bus::SystemBus { memory: &mut self.memory, cache: &mut self.cache };
        self.cpu.step(&mut bus);
        self.tick_devices(before);
    }

    /// Run for `n` cycles.
    pub fn run(&mut self, n: u64) {
        let mut remaining = n;
        while remaining > 0 {
            let slice = remaining.min(Self::RUN_SLICE);
            let before = self.cpu.cycles();
            let mut bus =
                bus::SystemBus { memory: &mut self.memory, cache: &mut self.cache };
            self.cpu.run(&mut bus, slice);
            drop(bus);
            self.tick_devices(before);
            if self.cpu.is_stopped() {
                break;
            }
            let spent = self.cpu.cycles().wrapping_sub(before);
            // Guard against a stopped/halted CPU making no progress.
            if spent == 0 {
                break;
            }
            remaining = remaining.saturating_sub(spent);
        }
    }

    /// Run until the program counter reaches `target_pc`.
    pub fn run_until(&mut self, target_pc: u32) {
        loop {
            let before = self.cpu.cycles();
            let mut bus =
                bus::SystemBus { memory: &mut self.memory, cache: &mut self.cache };
            self.cpu.run_until(&mut bus, target_pc);
            drop(bus);
            self.tick_devices(before);
            if self.pc() == target_pc || self.cpu.is_stopped() {
                break;
            }
        }
    }

    /// If CRIME requested a reset, restart the CPU at the reset vector.
    ///
    /// Both SOFT_RESET and HARD_RESET bring the CPU back through the warm
    /// path: the R5000's reset inputs latch the NMI/soft-reset flags into
    /// Status, and the PROM's `start_me_up` then takes `warm_start` and
    /// proceeds to the GDA instead of looping through the cold-boot
    /// diagnostics forever (as a real headless O2 with no boot devices
    /// would until a boot service supplies the GDA).
    fn check_crime_reset(&mut self) {
        if let Some(kind) = self.memory.crime_cpu.reset_requested() {
            self.memory.crime_cpu.clear_reset_request();
            match kind {
                crate::graphics::CrimeResetKind::Soft => {
                    log::info_msg("CRIME soft reset: CPU warm reset to reset vector")
                }
                crate::graphics::CrimeResetKind::Hard => {
                    log::info_msg("CRIME hard reset: CPU warm reset to reset vector")
                }
            }
            self.cpu.soft_reset(ip32::PROM_RESET_VECTOR);
        }
    }

    /// Whether the emulator is currently running.
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Start the emulator.
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the emulator.
    pub fn stop(&mut self) {
        self.running = false;
        self.cpu.stop();
    }

    /// The current program counter.
    pub fn pc(&self) -> u32 {
        self.cpu.state.pc
    }

    /// Drain UART1 console output (bytes written to the console UART).
    pub fn drain_console_output(&self) -> Vec<u8> {
        let mut out = Vec::new();
        if let Some(rx) = &self.memory.mace.isa_ext.uart1.console_out_rx {
            while let Ok(b) = rx.try_recv() {
                out.push(b);
            }
        }
        out
    }

    /// Render the GBE framebuffer into a linear RGBA8 buffer.
    ///
    /// `out` must be at least `width * height * 4` bytes. Returns the number
    /// of pixels written (width × height), or 0 if `out` is too small.
    pub fn render_framebuffer(&self, out: &mut [u8]) -> usize {
        self.memory.render_framebuffer(out)
    }

    /// The current framebuffer width in pixels.
    pub fn framebuffer_width(&self) -> usize {
        self.memory.gbe.width() as usize
    }

    /// The current framebuffer height in pixels.
    pub fn framebuffer_height(&self) -> usize {
        self.memory.gbe.height() as usize
    }

    // === SCSI storage ===

    /// Mount a device as the internal **hard disk** (SCSI target 1).
    pub fn mount_hard_disk(&mut self, device: Box<dyn BlockDevice>) -> anyhow::Result<()> {
        self.mount_scsi(SCSI_TARGET_DISK, device, "hard disk")
    }

    /// Mount a device as the internal **CD-ROM** (SCSI target 6).
    pub fn mount_cdrom(&mut self, device: Box<dyn BlockDevice>) -> anyhow::Result<()> {
        self.mount_scsi(SCSI_TARGET_CDROM, device, "CD-ROM")
    }

    fn mount_scsi(
        &mut self,
        target: u8,
        device: Box<dyn BlockDevice>,
        kind: &str,
    ) -> anyhow::Result<()> {
        let name = device.name().to_string();
        let sector_count = device.sector_count();
        let sector_size = device.sector_size();
        self.scsi.mount(target, device).map_err(|e| {
            anyhow::anyhow!("failed to mount {kind} image: {e}")
        })?;
        log::info_msg(&format!(
            "Mounted {kind} '{name}' on SCSI target {target} ({} bytes in {sector_count} sectors of {sector_size} B)",
            sector_count * u64::from(sector_size)
        ));
        self.scsi_unimplemented_trace();
        Ok(())
    }

    fn scsi_unimplemented_trace(&self) {
        log::debug_msg("SCSI register emulation (AIC-7880) is not yet wired; image mounted ready for future M3 work");
    }

    /// The name of the attached hard disk, if any.
    pub fn hard_disk_name(&self) -> Option<String> {
        self.scsi
            .device_info(SCSI_TARGET_DISK)
            .map(|info| info.name)
    }

    /// The name of the attached CD-ROM, if any.
    pub fn cdrom_name(&self) -> Option<String> {
        self.scsi
            .device_info(SCSI_TARGET_CDROM)
            .map(|info| info.name)
    }

    /// Take the host side of the MACE audio output ring for the front-end.
    pub fn take_audio_consumer(&mut self) -> Option<rtrb::Consumer<f32>> {
        self.memory.mace.take_audio_consumer()
    }

    // === Keyboard / mouse input ===

    /// Queue a host keyboard scan-code byte (PS/2 scan set 2) for the guest.
    pub fn push_kbd_byte(&mut self, byte: u8) {
        self.memory.mace.perif.kbdms.push_kbd_byte(byte);
    }

    /// Queue a host mouse packet byte for the guest.
    pub fn push_ms_byte(&mut self, byte: u8) {
        self.memory.mace.perif.kbdms.push_ms_byte(byte);
    }

    /// Queue a complete 3-byte PS/2 mouse packet for the guest.
    pub fn push_ms_packet(&mut self, packet: [u8; 3]) {
        for b in packet {
            self.memory.mace.perif.kbdms.push_ms_byte(b);
        }
    }

    /// Flush queued keyboard/mouse input (used by front-ends on focus loss).
    pub fn flush_input(&mut self) {
        self.memory.mace.perif.kbdms.clear_input();
    }
}