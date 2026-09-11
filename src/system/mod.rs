//! System-level emulation: the system bus and the top-level [`Emulator`].
//!
//! The [`Emulator`] owns the CPU, memory map, and PROM, and drives execution.
//! It is the primary entry point used by the `cli` and `gui` front-ends.

pub mod bus;

use crate::cpu::{CpuModel, R5000};
use crate::io::scsi::{ScsiBus, SCSI_TARGET_CDROM, SCSI_TARGET_DISK};
use crate::memory::MemoryMap;
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

impl Emulator {
    /// Create a new emulator with `ram_mb` megabytes of RAM.
    pub fn new() -> Self {
        let (uart1_tx, uart1_rx) = std::sync::mpsc::channel();
        let (uart2_tx, uart2_rx) = std::sync::mpsc::channel();
        Self {
            cpu: R5000::new(),
            memory: MemoryMap::new(256, uart1_tx, uart1_rx, uart2_tx),
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
        Ok(())
    }

    /// Reset the emulator (CPU + memory) to its power-on state.
    pub fn reset(&mut self) {
        self.cpu.reset(ip32::PROM_RESET_VECTOR);
        self.running = false;
        log::info_msg("Emulator reset");
    }

    /// Execute a single instruction.
    pub fn step(&mut self) {
        let mut bus = bus::SystemBus { memory: &mut self.memory };
        self.cpu.step(&mut bus);
    }

    /// Run for `n` cycles.
    pub fn run(&mut self, n: u64) {
        let mut bus = bus::SystemBus { memory: &mut self.memory };
        self.cpu.run(&mut bus, n);
    }

    /// Run until the program counter reaches `target_pc`.
    pub fn run_until(&mut self, target_pc: u32) {
        let mut bus = bus::SystemBus { memory: &mut self.memory };
        self.cpu.run_until(&mut bus, target_pc);
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
        let kbdms = &mut self.memory.mace.perif.kbdms;
        kbdms.keyboard_input.clear();
        kbdms.mouse_input.clear();
    }
}