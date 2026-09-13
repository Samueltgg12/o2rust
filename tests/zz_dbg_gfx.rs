use o2rust::system::Emulator;

#[test]
fn zz_dbg_gfx() {
    let (uart1_tx, uart1_rx) = std::sync::mpsc::channel();
    let (uart2_tx, _r) = std::sync::mpsc::channel();
    let mut emu = Emulator::with_ram(256, uart1_tx, uart1_rx, uart2_tx);
    let _ = emu.load_prom("samples/ip32prom.rev4.18.bin");

    let mut acc: Vec<u8> = Vec::new();
    let chunk = 1_000_000u64;
    for c in 0..3000u64 {
        emu.run(chunk);
        if c % 200 == 0 {
            eprintln!(
                "[{}] pc={:#x} acc={}B stats={:?}",
                c * chunk,
                emu.pc(),
                acc.len(),
                emu.memory.render_engine.raster_stats
            );
        }
        acc.extend(emu.drain_console_output());
        let w = emu.framebuffer_width();
        let h = emu.framebuffer_height();
        if c == 450 && (0x81006240..0x81006278).contains(&emu.pc()) {
            // step out of the delay loop and follow the caller
            let mut ra_next = 0u32;
            for _ in 0..4000u64 {
                let pc = emu.pc();
                if pc == 0x81006240 {
                    let ra = emu.cpu.state.gpr(31) as u32;
                    if ra != ra_next {
                        eprintln!("  us_delay from ra={ra:#x}");
                        ra_next = ra;
                    }
                }
                emu.step();
            }
            eprintln!("  pc now {:#x}", emu.pc());
        }
        if w != 0 || h != 0 || emu.memory.gbe.regs_dump().2 != 0 {
            eprintln!(
                "[{}] fb: {}x{} regs={:?} pc={:#x} raster={:?}",
                c * chunk,
                w,
                h,
                emu.memory.gbe.regs_dump(),
                emu.pc(),
                emu.memory.render_engine.raster_stats
            );
        }
        if acc.windows(13).any(|w| w == b"\n\rOption? ") {
            break;
        }
    }
    let out = String::from_utf8_lossy(&acc);
    eprintln!(
        "video open failure present: {}",
        out.contains("Cannot open video")
    );
    eprintln!(
        "framebuffer: {}x{}",
        emu.framebuffer_width(),
        emu.framebuffer_height()
    );
    let (w, h) = (emu.framebuffer_width(), emu.framebuffer_height());
    if w > 0 && h > 0 {
        let mut buf = vec![0u8; (w * h * 4) as usize];
        let n = emu.render_framebuffer(&mut buf);
        let nonzero = buf[..n * 4].chunks(4).filter(|p| p[..3] != [0, 0, 0]).count();
        eprintln!("rendered {} px, {} non-black", n, nonzero);
    }
    eprintln!("GBE: {:#?}", emu.memory.gbe.regs_dump());
    eprintln!("raster stats: {:?}", emu.memory.render_engine.raster_stats);
    eprintln!(
        "pixpipe writes: {:?}",
        emu.memory.render_engine.pixpipe_write_stats
    );
    // Flush the CPU D-cache: PROM text may be drawn via plain kseg0 stores.
    let fc = emu.memory.gbe.regs_dump().2;
    let Emulator { cache, memory, .. } = &mut emu;
    cache.flush_all(memory);
    let g = &memory.gbe;
    eprintln!(
        "crs: enabled={} pos=({},{}) cmap={:08x?}",
        g.crs_enabled(),
        g.crs_position().0,
        g.crs_position().1,
        g.crs_cmap()
    );
    let tl = (fc & 0xFFFF_FE00) as usize;
    eprintln!("tile-list bytes at {tl:#x}:");
    let mut nz = 0usize;
    let mut total = 0usize;
    for t in 0..16usize {
        let e = (memory.read16((tl + t * 2) as u32) as usize) << 16;
        let mut n = 0usize;
        for b in 0..65536usize {
            if memory.read8((e + b) as u32) != 0 { n += 1; }
        }
        nz += n;
        total += 65536;
        eprintln!("  tile {t} phys={e:#x} nonzero={n}");
    }
    eprintln!("nonzero pixel bytes in first 16 tiles: {nz}/{total}");

    // Dump the rendered framebuffer as a PPM for visual inspection.
    let (w, h) = (emu.framebuffer_width(), emu.framebuffer_height());
    if w > 0 && h > 0 {
        let mut buf = vec![0u8; (w * h * 4) as usize];
        let n = emu.render_framebuffer(&mut buf);
        let mut ppm = format!("P6\n{w} {h}\n255\n").into_bytes();
        for px in buf[..n * 4].chunks(4) {
            ppm.extend_from_slice(&px[..3]);
        }
        std::fs::write("/tmp/o2_screen.ppm", &ppm).unwrap();
        eprintln!("wrote /tmp/o2_screen.ppm ({w}x{h})");

        // ASCII luminance map: ~160 x ~40 chars.
        let (ac, ar) = (160usize, 40usize);
        eprintln!("ascii map ({ac}x{ar}):");
        for ry in 0..ar {
            let y0 = (ry * h as usize) / ar;
            let y1 = ((ry + 1) * h as usize) / ar;
            let mut line = String::new();
            for rx in 0..ac {
                let x0 = (rx * w as usize) / ac;
                let x1 = ((rx + 1) * w as usize) / ac;
                let mut sum = 0u32;
                let mut cnt = 0u32;
                for y in y0..y1.max(y0 + 1) {
                    for x in x0..x1.max(x0 + 1) {
                        let o = (y * w as usize + x) * 4;
                        let (r, g, b) = (buf[o] as u32, buf[o + 1] as u32, buf[o + 2] as u32);
                        sum += (r * 3 + g * 6 + b) / 10;
                        cnt += 1;
                    }
                }
                let a = if cnt == 0 { 0 } else { (sum / cnt) as u8 };
                line.push(if a > 150 {
                    '#'
                } else if a > 70 {
                    'O'
                } else if a > 20 {
                    'o'
                } else {
                    '.'
                });
            }
            eprintln!("{line}");
        }
    }

    eprintln!("--- console tail ---\n{}", &out[out.len().saturating_sub(900)..]);
}
