use o2rust::system::Emulator;

#[test]
fn observe_prom_boot() {
    let mut emu = Emulator::with_ram_no_console(256);
    emu.load_prom("samples/ip32prom.rev4.18.bin").unwrap();
    let mut seen = std::collections::HashSet::new();
    for i in 0..200_000 {
        emu.step();
        let pc = emu.pc();
        if i < 20 {
            eprintln!("[{i}] pc=0x{pc:08x}");
        }
        if !seen.insert(pc) {
            eprintln!("REPEAT at pc=0x{pc:08x} after {i} steps");
            break;
        }
    }
    eprintln!("final pc=0x{:08x}, seen {} distinct", emu.pc(), seen.len());
}