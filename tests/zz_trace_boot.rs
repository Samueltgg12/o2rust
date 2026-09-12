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
        0xbfc046f8..=0xbfc04a9c => "post1_lowmem",
        0xbfc04a9c..=0xbfc05ad4 => "post1_diag",
        0xbfc05ad4..=0xbfc06000 => "post1_meminit",
        0xbfc06000..=0xbfc06500 => "post1_bcopy",
        0xa0004000..=0xa0006200 => "ramcopy@4000",
        0x81000000..=0x82000000 => "FW-RAM!",
        0xbfc00100..=0xbfc0039f => "exceptions",
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
    let mut acc = Vec::new();
    let mut stopped = "";
    let mut watching: Vec<(u32, u32)> = Vec::new();
    let mut dead_loop_reported = false;
    let mut gda_seen = false;
    let mut fc_seen = false;
    let mut loop_probed = 0;
    let mut probes = [0u32; 8];
    let mut ztrans: Vec<(u64, u32, &'static str)> = Vec::new();
    let mut wait_probed = 0;
    let mut prev_z_was_ramcopy = false;
    for i in 0..200_000_000u64 {
        let pc = emu.pc();
        if let Some((a1, n)) = watching.first().copied() {
            let a0 = emu.cpu.state.gpr(4) as u32;
            let t8 = emu.cpu.state.gpr(24) as u32;
            let t9 = emu.cpu.state.gpr(25) as u32;
            let v0 = emu.cpu.state.gpr(2) as u32;
            eprintln!("W step={i:>9} pc=0x{pc:08x} a1=0x{a1:08x} a0=0x{a0:08x} t8=0x{t8:08x} t9=0x{t9:08x} v0=0x{v0:08x}");
            watching[0].1 += 1;
            if watching[0].1 > 80 || pc == 0xbfc0199c {
                for r in 2..=15 {
                    eprintln!("  R{r:02}={:016x}", emu.cpu.state.gpr(r));
                }
                watching.clear();
            }
        }
        let z = zone(pc);
        if z != prev {
            order.push(z);
            if order.len() > 60 {
                order.remove(0);
            }
ztrans.push((i, pc, z));
        if ztrans.len() > 6000 {
            ztrans.remove(0);
        }
        }
        prev = z;
        *counts.entry(z).or_insert(0) += 1;
        last.push((i, pc, z));
        if last.len() > 40 {
            last.remove(0);
        }
        let fresh = emu.drain_console_output();
        acc.extend(fresh.iter().copied());
        if fresh.len() >= 5 {
            for w in fresh.windows(5) {
                if w == b"SL-96" || w == b"SL-48" {
                    stopped = "SL- prompt (fresh)";
                    break;
                }
            }
            if stopped != "" { break; }
        }
        if z == "FW-RAM!" {
            stopped = "FW-RAM";
            break;
        }
        if (0xbfc00b14..0xbfc00b34).contains(&pc) {
            eprintln!(
                "ERROR-PATH pc=0x{pc:08x}: v0(section2)=0x{:08x} v1(post1)=0x{:08x} a1=0x{:08x} a3=0x{:08x}",
                emu.cpu.state.gpr(2),
                emu.cpu.state.gpr(3),
                emu.cpu.state.gpr(5),
                emu.cpu.state.gpr(7),
            );
            break;
        }
        if pc == 0xbfc00a58 {
            let t0 = emu.cpu.state.gpr(8) as u64;
            let t2 = emu.cpu.state.gpr(10) as u64;
            let s0 = emu.cpu.state.gpr(16) as u64;
            let sp = emu.cpu.state.gpr(29) as u64;
            let ra = emu.cpu.state.gpr(31) as u64;
            eprintln!(
                "SCRUB t0=0x{t0:016x} t2=0x{t2:016x} s0=0x{s0:016x} sp=0x{sp:016x} ra=0x{ra:016x}",
            );
        }
        if (0xa0100020..0xa0100040).contains(&pc) && !gda_seen {
            gda_seen = true;
            eprintln!(
                "GDA-REGION entry pc=0x{pc:08x} ra=0x{:016x} sp=0x{:016x} v0=0x{:016x} a0=0x{:016x}",
                emu.cpu.state.gpr(31),
                emu.cpu.state.gpr(29),
                emu.cpu.state.gpr(2),
                emu.cpu.state.gpr(4),
            );
            eprintln!("GDA-REGION halt reason: kernel stub? pre-FW jump failed");
            eprintln!("GDA phys0x400 magic=0x{:08x} entry=0x{:08x}", emu.memory.read32(0x400), emu.memory.read32(0x408));
            for i in 0..10u32 {
                let w = emu.memory.read32(0x0010_0000 + i * 4);
                eprintln!("  0xa010{:04x}: {:08x}", i * 4, w);
            }
            eprintln!("last transitions at handoff:");
            for (i, p, z) in last.iter() {
                eprintln!("  step={i:>10} pc=0x{p:08x} {z}");
            }
            eprintln!("GDA-REGION probe done");
        }
        if (0xa010002c..0xa0100040).contains(&pc) && loop_probed < 12 {
            loop_probed += 1;
            let w2c = emu.memory.read32(0x0010_002c & 0xffffffff);
            let w30 = emu.memory.read32(0x0010_0030);
            let w34 = emu.memory.read32(0x0010_0034);
            eprintln!(
                "LOOPPROBE step={i} pc=0x{pc:08x} := [0x2c]=0x{w2c:08x} [0x30]=0x{w30:08x} [0x34]=0x{w34:08x} ra=0x{:016x}",
                emu.cpu.state.gpr(31),
            );
        }
        if pc == 0xbfc044f4 {
            eprintln!(
                "STKCHK t0(savedsp)=0x{:016x} t1(sp^0x20000000)=0x{:016x} sp=0x{:016x} [0xa0001008]=0x{:016x}",
                emu.cpu.state.gpr(8),
                emu.cpu.state.gpr(9),
                emu.cpu.state.gpr(29),
                emu.memory.read64(0xa0001008),
            );
        }
        if pc == 0xbfc04520 {
            eprintln!(
                "SEGTYPE t2(FS_Xcopy)=0x{:08x} t1(segt)=0x{:08x} t0(seg)=0x{:016x} a1(fwSeg)=0x{:08x}",
                emu.cpu.state.gpr(10) as u32,
                emu.cpu.state.gpr(9) as u32,
                emu.cpu.state.gpr(8),
                emu.cpu.state.gpr(5) as u32,
            );
        }
        if pc == 0xbfc045b8 {
            eprintln!(
                "VALIDATE v0=0x{:08x} a0(blkaddr)=0x{:08x} a1(blklen)=0x{:08x} s6=0x{:08x} s7=0x{:08x}",
                emu.cpu.state.gpr(2) as u32,
                emu.cpu.state.gpr(4) as u32,
                emu.cpu.state.gpr(5) as u32,
                emu.cpu.state.gpr(22) as u32,
                emu.cpu.state.gpr(23) as u32,
            );
        }
        if pc == 0xbfc04624 {
            eprintln!(
                "CHKSUM s1=0x{:08x} s0(blkptr)=0x{:08x} v0=0x{:08x}",
                emu.cpu.state.gpr(17) as u32,
                emu.cpu.state.gpr(16) as u32,
                emu.cpu.state.gpr(2) as u32,
            );
        }
        if pc == 0xbfc04670 {
            eprintln!(
                "POST1-EXIT v0=0x{:08x} ra=0x{:08x} sp=0x{:016x}",
                emu.cpu.state.gpr(2) as u32,
                emu.cpu.state.gpr(31) as u32,
                emu.cpu.state.gpr(29),
            );
        }
        if pc == 0xbfc04668 {
            eprintln!(
                "POST1-JUMP-FW t0(fw_entry)=0x{:016x} a0(fcode)=0x{:08x} a1(reset)=0x{:08x}",
                emu.cpu.state.gpr(8),
                emu.cpu.state.gpr(4) as u32,
                emu.cpu.state.gpr(5) as u32,
            );
        }
        if pc == 0xbfc05110 {
            probes[6] += 1;
            let sp = emu.cpu.state.gpr(29);
            eprintln!(
                "MEMTEST entry #{} a0=0x{:08x} a1=0x{:08x} ra=0x{:016x} sp=0x{:016x}",
                probes[6],
                emu.cpu.state.gpr(4) as u32,
                emu.cpu.state.gpr(5) as u32,
                emu.cpu.state.gpr(31),
                sp,
            );
        }
        if pc == 0xbfc06190 {
            let v0 = emu.cpu.state.gpr(2) as u64;
            let v1 = emu.cpu.state.gpr(3) as u64;
            let a3 = emu.cpu.state.gpr(7) as u64;
            eprintln!("COPYLOOP entry v0(src)=0x{v0:016x} v1(dst)=0x{v1:016x} a3(end)=0x{a3:016x}");
        }
        if pc == 0xbfc061ac {
            let v0 = emu.cpu.state.gpr(2) as u64;
            let v1 = emu.cpu.state.gpr(3) as u64;
            let t9 = emu.cpu.state.gpr(25) as u64;
            eprintln!("COPYDONE v0=0x{v0:016x} v1=0x{v1:016x} t9(jalr)=0x{t9:016x}");
        }
        if (0x80010000..0x80010100).contains(&pc) && !fc_seen {
            fc_seen = true;
            let a0 = emu.cpu.state.gpr(4) as u64;
            let a1 = emu.cpu.state.gpr(5) as u64;
            let a2 = emu.cpu.state.gpr(6) as u64;
            let a3 = emu.cpu.state.gpr(7) as u64;
            let ra = emu.cpu.state.gpr(31) as u64;
            eprintln!(
                "FASTCOPY@0x80010000 entry pc=0x{pc:08x} a0=0x{a0:016x} a1=0x{a1:016x} a2=0x{a2:016x} a3=0x{a3:016x} ra=0x{ra:016x}",
            );
            for i in 0..8u32 {
                eprintln!(
                    "  0x8001{:04x}: {:08x}",
                    i * 4,
                    emu.memory.read32(0x80010000 + i * 4)
                );
            }
        }
        if pc == 0xbfc04448 {
            probes[0] += 1;
            eprintln!(
                "POST1 entry #{} a0=0x{:08x} a1=0x{:08x} a2=0x{:08x} ra=0x{:08x} sp=0x{:016x}",
                probes[0],
                emu.cpu.state.gpr(4) as u32,
                emu.cpu.state.gpr(5) as u32,
                emu.cpu.state.gpr(6) as u32,
                emu.cpu.state.gpr(31) as u32,
                emu.cpu.state.gpr(29),
            );
        }
        if pc == 0xbfc060f8 {
            probes[1] += 1;
            eprintln!(
                "COPIER entry #{} a0=0x{:08x} a1=0x{:08x} a2=0x{:08x} a3=0x{:08x} ra=0x{:08x}",
                probes[1],
                emu.cpu.state.gpr(4) as u32,
                emu.cpu.state.gpr(5) as u32,
                emu.cpu.state.gpr(6) as u32,
                emu.cpu.state.gpr(7) as u32,
                emu.cpu.state.gpr(31) as u32,
            );
        }
        if pc == 0xbfc04588 && probes[2] < 3 {
            probes[2] += 1;
            eprintln!(
                "POST1 next_subsection #{} s0=0x{:016x} s2=0x{:016x} s3=0x{:016x} sp=0x{:016x}",
                probes[2],
                emu.cpu.state.gpr(16),
                emu.cpu.state.gpr(18),
                emu.cpu.state.gpr(19),
                emu.cpu.state.gpr(29),
            );
        }
        if pc == 0xbfc045d4 && probes[3] < 3 {
            probes[3] += 1;
            eprintln!(
                "POST1 copy_loop #{} a0(dst)=0x{:08x} a1(len)=0x{:08x} s0(src)=0x{:016x} s1=0x{:016x}",
                probes[3],
                emu.cpu.state.gpr(4) as u32,
                emu.cpu.state.gpr(5) as u32,
                emu.cpu.state.gpr(16),
                emu.cpu.state.gpr(17),
            );
        }
        if pc == 0xbfc0461c && probes[4] < 3 {
            probes[4] += 1;
            eprintln!(
                "POST1 verify_and_jump #{} a0=0x{:016x} s0=0x{:016x} s1=0x{:016x}",
                probes[4],
                emu.cpu.state.gpr(4),
                emu.cpu.state.gpr(16),
                emu.cpu.state.gpr(17),
            );
        }
        if pc == 0xbfc04670 && probes[5] < 3 {
            probes[5] += 1;
            eprintln!(
                "POST1 error_return #{} v0=0x{:016x} ra=0x{:016x}",
                probes[5],
                emu.cpu.state.gpr(2),
                emu.cpu.state.gpr(31),
            );
        }
        if pc == 0xbfc003a0 && !dead_loop_reported {
            let status = emu.cpu.cp0.read(o2rust::cpu::cp0::Cp0Reg::Status);
            let cause = emu.cpu.cp0.read(o2rust::cpu::cp0::Cp0Reg::Cause);
            let epc = emu.cpu.cp0.read(o2rust::cpu::cp0::Cp0Reg::Epc);
            let bva = emu.cpu.cp0.read(o2rust::cpu::cp0::Cp0Reg::BadVAddr);
            eprintln!(
                "DEAD_LOOP: status=0x{status:08x} cause=0x{cause:08x} epc=0x{epc:08x} bva=0x{bva:08x} sp=0x{:016x} ra=0x{:016x}",
                emu.cpu.state.gpr(29),
                emu.cpu.state.gpr(31),
            );
            dead_loop_reported = true;
        }
        if pc == 0xbfc018f4 && watching.is_empty() {
            let a1 = emu.cpu.state.gpr(5) as u32;
            if a1 == 0xbfc00000 {
                watching.push((a1, 0));
            }
        }
        if (0xbfc00ac4..0xbfc00b13).contains(&pc) && pc % 4 == 0 {
            // after each find returns, log the returned pointer briefly
            if pc == 0xbfc00b00 || pc == 0xbfc00b04 {
                eprintln!(
                    "AFTER-FIND pc=0x{pc:08x}: v0=0x{:08x} v1=0x{:08x}",
                    emu.cpu.state.gpr(2),
                    emu.cpu.state.gpr(3),
                );
            }
        }
        if (0xbfc00b70..0xbfc00c48).contains(&pc) {
            stopped = "monitor_tail entered";
            break;
        }
        if pc == 0xa00047d0 && wait_probed < 8 {
            wait_probed += 1;
            eprintln!(
                "USTWAIT entry #{} a0(timeout)=0x{:016x} a1=0x{:016x} ra=0x{:016x} sp=0x{:016x} UST=0x{:016x}",
                wait_probed,
                emu.cpu.state.gpr(4),
                emu.cpu.state.gpr(5),
                emu.cpu.state.gpr(31),
                emu.cpu.state.gpr(29),
                emu.memory.read64(0x1f340000),
            );
        }
        if (0xa00047d0..0xa0004810).contains(&pc) && i % 10000 == 0 {
            eprintln!(
                "USTSPIN step={i} pc=0x{pc:08x}: a0=0x{:016x} v1=0x{:016x} t1=0x{:016x} UST=0x{:016x}",
                emu.cpu.state.gpr(4),
                emu.cpu.state.gpr(3),
                emu.cpu.state.gpr(9),
                emu.memory.read64(0x1f340000),
            );
        }
        if prev_z_was_ramcopy && !(0xa0004000..0xa0005000).contains(&pc) && wait_probed < 20 {
            wait_probed = 20;
            eprintln!(
                "LEFT-RAMCOPY to pc=0x{pc:08x}: ra=0x{:016x} sp=0x{:016x} v0=0x{:016x} a0=0x{:016x} a1=0x{:016x}",
                emu.cpu.state.gpr(31),
                emu.cpu.state.gpr(29),
                emu.cpu.state.gpr(2),
                emu.cpu.state.gpr(4),
                emu.cpu.state.gpr(5),
            );
        }
        if prev_z_was_ramcopy && !(0xa0004000..0xa0006150).contains(&pc) && wait_probed < 20 {
            wait_probed = 20;
            eprintln!(
                "POST1DIAGS-RETURN to pc=0x{pc:08x}: v0=0x{:016x} ra=0x{:016x} sp=0x{:016x} s0=0x{:016x} s1=0x{:016x} a0=0x{:016x} a1=0x{:016x}",
                emu.cpu.state.gpr(2),
                emu.cpu.state.gpr(31),
                emu.cpu.state.gpr(29),
                emu.cpu.state.gpr(16),
                emu.cpu.state.gpr(17),
                emu.cpu.state.gpr(4),
                emu.cpu.state.gpr(5),
            );
        }
        if z == "ramcopy@4000" {
            // Fast-forward through ramcopy (post1diags) but still single-step.
            emu.run(100);
        } else {
            emu.step();
        }
        prev_z_was_ramcopy = (0xa0004000..0xa0006150).contains(&pc);
    }
    if stopped.is_empty() {
        stopped = "step-limit";
    }
    eprintln!("stopped: {stopped} at pc=0x{:08x} step={}", emu.pc(), emu.cpu.cycles());
    for (v, p) in [
        (0xa0004000u32, 0x0000_4000u32),
        (0xa000542c, 0x0000_542c),
        (0xa0000f00, 0x0000_0f00),
        (0xa0100000, 0x0010_0000),
        (0x81000000, 0x0100_0000),
    ] {
        eprintln!("--- RAM phys 0x{p:08x} (v {v:08x}) ---");
        for i in 0..24 {
            let w = emu.memory.read32(p + i * 4);
            eprintln!("  0x{:08x}: {w:08x}", p + i * 4);
        }
    }
    eprintln!("zone counts:");
    for (z, c) in &counts {
        eprintln!("  {z:<24} {c:>10}");
    }
    eprintln!("zone transitions (first {}):", ztrans.len());
    for (i, p, z) in ztrans.iter() {
        eprintln!("  step={i:>10} pc=0x{p:08x} {z}");
    }
    eprintln!("last transitions:");
    for (i, p, z) in last.iter() {
        eprintln!("  step={i:>10} pc=0x{p:08x} {z}");
    }
}