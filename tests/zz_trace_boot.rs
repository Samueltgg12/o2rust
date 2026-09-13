use o2rust::system::Emulator;

fn zone(pc: u32) -> &'static str {
    match pc {
        0xbfc00000..=0xbfc003a7 => "reset/branch",
        0xbfc003a8..=0xbfc007e7 => "start_me_up",
        0xbfc007e8..=0xbfc00a7f => "init_tail",
        0xbfc00a80..=0xbfc00b6f => "post1disp(F_A80)",
        0xbfc00b70..=0xbfc00c47 => "monitor_tail(F_B70)",
        0xbfc00c48..=0xbfc016cf => "serial_loader_code",
        0xbfc016d0..=0xbfc019b0 => "find_sect/is_sect_valid",
        0xbfc019b0..=0xbfc01d98 => "tlb_init",
        0xbfc01d98..=0xbfc04000 => "sloader_late",
        0xbfc04400..=0xbfc04447 => "post1_entry",
        0xbfc04448..=0xbfc045ff => "post1_main_a",
        0xbfc04600..=0xbfc046ff => "post1_copy/checksum",
        0xbfc04a9c..=0xbfc05ad4 => "post1_diag",
        0xbfc05ad4..=0xbfc06000 => "post1_meminit",
        0xbfc06000..=0xbfc06500 => "post1_bcopy",
        0xa0004000..=0xa0006200 => "ramcopy@4000",
        0x81000000..=0x82000000 => "FW-RAM!",
        _ => "?",
    }
}

#[test]
fn zz_trace_boot() {
    let (uart1_tx, uart1_rx) = std::sync::mpsc::channel();
    let (uart2_tx, _uart2_rx) = std::sync::mpsc::channel();
    let mut emu = Emulator::with_ram(256, uart1_tx, uart1_rx, uart2_tx);
    let _ = emu.load_prom("samples/ip32prom.rev4.18.bin");

    let mut order: Vec<&'static str> = Vec::new();
    let mut counts: std::collections::BTreeMap<&'static str, u64> = Default::default();
    let mut last: Vec<(u64, u32, &'static str)> = Vec::new();
    let mut prev = "";
    let mut stopped = "";

    for i in 0..200_000_000u64 {
        let pc = emu.pc();
        let z = zone(pc);
        if z != prev {
            order.push(z);
            if order.len() > 60 {
                order.remove(0);
            }
        }
        prev = z;
        *counts.entry(z).or_insert(0) += 1;
        last.push((i, pc, z));
        if last.len() > 40 {
            last.remove(0);
        }

        // ←——————— POST1 CHECKS ———————↓
        if pc == 0xbfc04670 {
            eprintln!(
                "POST1-EXIT(sloader route): v0={} ra=0x{:016x} sp=0x{:016x}",
                emu.cpu.state.gpr(2) as u32,
                emu.cpu.state.gpr(31),
                emu.cpu.state.gpr(29),
            );
            break;
        }
        if pc == 0xbfc04668 {
            eprintln!(
                "POST1-JUMP-FW t0(entry)=0x{:08x}",
                emu.cpu.state.gpr(8) as u64,
            );
            stopped = "FW-RAM";
            break;
        }
        if pc == 0xbfc044e0 {
            let sp = emu.cpu.state.gpr(29);
            let saved_sp = emu.memory.read64(0xa0001008);
            let xor_val = sp ^ 0x20000000;
            eprintln!(
                "POST1-STACKCHK: sp=0x{:016x} sp^0x20000000=0x{:016x} [0xa0001008]=0x{:016x}",
                sp,
                xor_val,
                saved_sp,
            );
        }
        // ←——————— / POST1 CHECKS ———————→

        // Fast forward through the big slice of the code we know works,
        // so we don't waste CPU on very long loops intentionally.
        if z == "ramcopy@4000" {
            emu.run(200_000);
        } else {
            emu.step();
        }

        let fresh = emu.drain_console_output();
        for _ in fresh {}
    }
    if stopped.is_empty() {
        stopped = "step-limit";
    }
    eprintln!("stopped: {stopped} at pc=0x{:08x} step={}", emu.pc(), emu.cpu.cycles());
    for (v, p) in [
        (0x80001000u32, 0x00001000u32),
        (0xa0004000u32, 0x00004000u32),
        (0x80100000u32, 0x00100000u32),
        (0x81000000u32, 0x01000000u32),
    ] {
        eprintln!("--- RAM phys {:08x} ({}) ---", p, v);
        for i in 0..8 {
            let w = emu.memory.read32(p + i * 4);
            eprintln!("  0x{:08x}: {:08x}", p + i * 4, w);
        }
    }
    eprintln!("zone counts:");
    for (z, c) in &counts {
        eprintln!("  {}: {}", z, c);
    }
    eprintln!("last transitions:");
    for (i, p, z) in last.iter() {
        eprintln!("  step={:<10} pc=0x{:08x} {}", i, p, z);
    }
}
