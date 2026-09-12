use o2rust::system::Emulator;

const STUB_PC: u32 = 0xa0100000;
const CRM_CONTROL_PHYS: u32 = 0x1400_0008;
const CRM_CONTROL_SOFT_RESET: u32 = 0x0400;

fn new_emu() -> Emulator {
    let (uart1_tx, uart1_rx) = std::sync::mpsc::channel();
    let (uart2_tx, _uart2_rx) = std::sync::mpsc::channel();
    let mut emu = Emulator::with_ram(256, uart1_tx, uart1_rx, uart2_tx);
    let _ = emu.load_prom("samples/ip32prom.rev4.18.bin");
    emu
}

#[test]
fn observe_prom_boot_warm_gda_stub() {
    let mut emu = new_emu();
    let chunk = 200_000u64;
    let mut step = 0u64;
    let mut saw_monitor_prompt = false;

    // Phase A: cold boot through POST into OSLOAD's scan-link monitor, which
    // blocks waiting for a keypress (real headless O2 behavior).
    let mut acc = Vec::new();
    for _ in 0..(4_000_000_000u64 / chunk) {
        let pc = emu.pc();
        if pc == 0xbfc003a0 {
            eprintln!("[{step}] dead_loop hit");
            break;
        }
        acc.extend(emu.drain_console_output());
        if acc.windows(13).any(|w| w == b"\n\rSL-9600-8E>") {
            saw_monitor_prompt = true;
            eprintln!("[{step}] scan-link monitor prompt printed, pc=0x{pc:08x}");
            break;
        }
        emu.run(chunk);
        step += chunk;
    }
    eprintln!("phase A end: prompt={saw_monitor_prompt} acc={:?}", String::from_utf8_lossy(&acc));
    assert!(saw_monitor_prompt, "PROM must reach the scan-link monitor");

    // Phase B: the boot service (host) primes the GDA + stub, then simulates
    // the reset button — the CRIME chip latches its soft-reset flag, which the
    // system loop converts into a CPU warm reset with NMI|SR latched into
    // Status. (The POST's RAM test wiped physical 0x400, so the GDA is written
    // here, right before reboot — as a real OS boot service would.)
    emu.install_warm_boot_stub();
    emu.memory.write32(CRM_CONTROL_PHYS, CRM_CONTROL_SOFT_RESET);
    emu.step();
    emu.run(chunk);

    // Phase C: the warm restart takes start_me_up's warm path, parks for 1s on
    // the CRIME timer, validates GDA magic and hands off into our boot stub.
    let mut seen_cold_boot_again = false;
    let mut final_console = Vec::new();
    for _ in 0..(4_000_000_000u64 / chunk) {
        let pc = emu.pc();
        if !seen_cold_boot_again && pc == 0xbfc00000 {
            seen_cold_boot_again = true;
            eprintln!("[{step}] ==== WARM RESTART at reset vector ====");
        }
        if pc == 0xbfc007e8 {
            eprintln!("[{step}] ==== reached warm_start 0xbfc007e8 ====");
        }
        if pc == STUB_PC {
            eprintln!("[{step}] ==== REACHED BOOT STUB 0x{STUB_PC:08x} ====");
            break;
        }
        if pc == 0xbfc003a0 {
            eprintln!("[{step}] dead_loop hit");
            break;
        }
        emu.run(chunk);
        step += chunk;
    }
    final_console.extend(emu.drain_console_output());
    let got = String::from_utf8_lossy(&final_console).into_owned();
    eprintln!("end: pc=0x{:08x} step={step}", emu.pc());
    eprintln!("console len={} tail38={:?}", got.len(), &got[got.len().saturating_sub(38)..]);
    assert!(
        (0xa010_0000..0xa010_1000).contains(&emu.pc()),
        "PC must land inside the GDA boot stub, got 0x{:08x}",
        emu.pc()
    );
    assert!(
        got.contains("O2Rust: GDA boot handoff OK"),
        "stub banner should reach the console"
    );
}

/// Reproduces `start_me_up`'s warm-start test on the R5000 Status register:
/// `srl(16) ; andi 0x18 ; addi -0x18` must yield 0 for the two flags.
#[test]
fn micro_warm_check_sequence() {
    let mut emu = Emulator::with_ram_no_console(256);
    use o2rust::cpu::cp0::Cp0Reg;
    let code = [
        0x401a6000u32, // mfc0 $k0, $12 (Status)
        0x001ad402u32, // srl  $k0, $k0, 16
        0x335a0018u32, // andi $k0, $k0, 0x18
        0x235affe8u32, // addi $k0, $k0, -0x18
    ];
    for (i, w) in code.iter().enumerate() {
        emu.memory.write32((i as u32) * 4, *w);
    }
    emu.cpu.cp0.write(Cp0Reg::Status, 0x0058_0004);
    emu.cpu.state.pc = 0x80000000;
    emu.cpu.state.next_pc = 0x80000000;
    for _ in 0..5 {
        emu.step();
    }
    assert_eq!(emu.cpu.state.gpr(26), 0, "warm check should compute k0=0 after addi");
}