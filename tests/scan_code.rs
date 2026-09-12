use o2rust::cpu::r5000::MemoryAccess;
use o2rust::system::bus::SystemBus;
use o2rust::system::Emulator;

#[test]
fn scan_for_real_code() {
    let mut emu = Emulator::with_ram_no_console(256);
    emu.load_prom("samples/ip32prom.rev4.18.bin").unwrap();
    let mut dc = o2rust::memory::DCache::new();
    for i in 0..2_000_000u64 {
        let pc = emu.cpu.state.pc;
        let word = {
            let mut bus = SystemBus { memory: &mut emu.memory, cache: &mut dc };
            bus.read32(pc)
        };
        if word != 0 {
            eprintln!("[{i}] first non-zero word: pc=0x{pc:08x} word=0x{word:08x}");
            return;
        }
        emu.step();
    }
    eprintln!("no non-zero word found in 2M steps; final pc=0x{:08x}", emu.pc());
}