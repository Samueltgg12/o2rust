use o2rust::system::Emulator;

#[test]
fn zz_live_dump() {
    let (uart1_tx, uart1_rx) = std::sync::mpsc::channel();
    let (uart2_tx, _r) = std::sync::mpsc::channel();
    let mut emu = Emulator::with_ram(256, uart1_tx, uart1_rx, uart2_tx);
    let _ = emu.load_prom("samples/ip32prom.rev4.18.bin");

    let chunk = 200_000_000u64;
    let mut step = 0u64;
    loop {
        emu.run(chunk);
        step += 1;
        let inst = step * chunk;
        let (w, h) = (emu.framebuffer_width(), emu.framebuffer_height());
        eprintln!("== inst={}G fb={}x{} pc={:#x}", inst / 1_000_000_000, w, h, emu.pc());
        if w == 0 || h == 0 {
            if inst > 6_000_000_000 {
                break;
            }
            continue;
        }
        let mut buf = vec![0u8; (w * h * 4) as usize];
        let n = emu.render_framebuffer(&mut buf);
        // Left boundary scan: per row, the x where the pixel leaves black,
        // as a coarse signature to spot an hourglass/wedge.
        let mut sig = String::new();
        let rows = 16;
        for r in 0..rows {
            let y = (r * h as usize + h as usize / 2) / rows;
            let mut lo = None;
            for x in 0..w as usize {
                let o = (y * w as usize + x) * 4;
                if buf[o] > 6 || buf[o + 1] > 6 || buf[o + 2] > 6 {
                    lo = Some(x);
                    break;
                }
            }
            match lo {
                Some(x) => {
                    sig.push('|');
                    sig.push_str(&(x / (w as usize / 50)).to_string());
                }
                None => sig.push('.'),
            }
        }
        eprintln!("  sig: {sig}");
        if inst >= 6_000_000_000 {
            break;
        }
    }
}