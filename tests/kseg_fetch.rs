use o2rust::cpu::r5000::MemoryAccess;
use o2rust::system::bus::SystemBus;
use o2rust::system::Emulator;

#[test]
fn boot_fetches_through_kseg1() {
    let mut emu = Emulator::with_ram_no_console(256);
    let _ = emu.load_prom("samples/ip32prom.rev4.18.bin");
    let r = {
        let mut dc = o2rust::memory::DCache::new();
        let mut bus = SystemBus { memory: &mut emu.memory, cache: &mut dc };
        bus.read32(0xbfc00000)
    };
    eprintln!("KSEG1 rom[0xbfc00000] = 0x{r:08x} (was 0 before KSEG fix)");
    assert!(r != 0, "KSEG1 fetch must reach the ROM");

    let mut pcs = Vec::new();
    for _ in 0..8 {
        emu.step();
        pcs.push(emu.pc());
    }
    eprintln!("first 8 PCs: {pcs:08x?}");
    assert!(pcs.iter().any(|p| (0xbfc00000..0xbffffffc).contains(p)));
}