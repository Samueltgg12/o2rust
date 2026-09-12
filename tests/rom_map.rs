use o2rust::memory::MemoryMap;

#[test]
fn inspect_rom_mapping() {
    let mut mem = MemoryMap::new_without_console(256);
    mem.rom.load(0, &[0x12, 0x34, 0x56, 0x78][..]);
    for a in [0x1fc00000u32, 0xbfc00000, 0xbfc00004, 0xa0000000] {
        eprintln!("read32(0x{a:08x}) = 0x{:08x}", mem.read32(a));
    }
    eprintln!("rom len = 0x{:x}", mem.rom.len());
}