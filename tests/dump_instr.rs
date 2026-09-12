use o2rust::cpu::r5000::MemoryAccess;
use o2rust::system::bus::SystemBus;
use o2rust::system::Emulator;

#[test]
fn dump_instructions() {
    let mut emu = Emulator::with_ram_no_console(256);
    emu.load_prom("samples/ip32prom.rev4.18.bin").unwrap();
    let mut dc = o2rust::memory::DCache::new();
    for i in 0..20 {
        let pc = emu.cpu.state.pc;
        let word = {
            let mut bus = SystemBus { memory: &mut emu.memory, cache: &mut dc };
            bus.read32(pc)
        };
        eprintln!("[{i}] pc=0x{pc:08x} word=0x{word:08x}");
        emu.step();
    }
}