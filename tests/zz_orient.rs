//! Orientation probe: draw an asymmetric pattern through the MRE path and
//! dump the rendered framebuffer region to a PBM for visual inspection.
use o2rust::memory::MemoryMap;

#[test]
fn zz_orient() {
    let (u1t, u1r) = std::sync::mpsc::channel();
    let (u2t, _u2r) = std::sync::mpsc::channel();
    let mut mem = MemoryMap::new(256, u1t, u1r, u2t);

    // Program a minimal 8bpp 1024x128 framebuffer: 2 tiles per row, 1 row.
    // Tile descriptors at phys 0x1000; tiles at phys 0x10000, 0x20000.
    use o2rust::ip32;
    let gbe_base = ip32::PHYS_BASE_GBE;
    // FRM_SIZE_TILE: width_tiles=2 (bits 12:5), rhs=0, depth=0 (8bpp)
    mem.write32(gbe_base + 0x30000, 2 << 5);
    // FRM_SIZE_PIXEL: height 128 in bits 31:16
    mem.write32(gbe_base + 0x30004, 128 << 16);
    // FRM_CONTROL: tile list at phys 0x1000, dma enable bit0
    mem.write32(gbe_base + 0x3000c, 0x1000 | 1);
    mem.write16(0x1000, 0x0001); // tile 0 -> phys 0x10000
    mem.write16(0x1002, 0x0002); // tile 1 -> phys 0x20000
    // cmap entries: 1 = white
    mem.write32(gbe_base + 0x50000 + 4, 0xff_ff_ff_00);

    let re = ip32::PHYS_BASE_RENDER;
    // fg color = index 1 (white via cmap)
    mem.write32(re + 0x20d0, 0x0000_0001); // Shade.fgColor
    mem.write32(re + 0x21b8, 0xffff_ffff); // ColorMask
    // Draw stippled 16-px line at y=10, x=100..115: pattern 0b1010_... (MSB first)
    mem.write32(re + 0x2018, (1 << 19) | 0x388); // DrawMode ENLINESTIPPLE (+COLORMASK/CBM bits like PROM)
    mem.write32(re + 0x20c0, (31 << 16) | (16 << 24)); // Stipple.mode maxIndex=31 index=16
    mem.write32(re + 0x20c4, 0b1010_1010_1010_1010); // pattern in low 16
    mem.write32(re + 0x2070, (100 << 16) | 10); // Vertex 0
    mem.write32(re + 0x2074, (115 << 16) | 10); // Vertex 1
    mem.write32(re + 0x2060, 1 << 24); // Primitive LINE
    mem.write32(re + (0x29f0), 0); // go (normal-space null)

    // And a RECT fill x=200..210, y=20..40 with color index 1
    mem.write32(re + 0x2018, 0x88); // ENCOLORMASK|ENCOLORBYTEMASK.. hmm just plain
    mem.write32(re + 0x2070, (200 << 16) | 20);
    mem.write32(re + 0x2074, (210 << 16) | 40);
    mem.write32(re + 0x2060, 3 << 24); // Primitive RECT
    mem.write32(re + (0x29f0), 0);

    // Render
    eprintln!("re stats: {:?}", mem.render_engine.raster_stats);
    eprintln!(
        "gbe: w={} h={} tlp={:#x}",
        mem.gbe.width(),
        mem.gbe.height(),
        mem.gbe.tile_list_ptr()
    );
    let w = mem.gbe.width() as usize;
    let h = mem.gbe.height() as usize;
    eprintln!("fb: {w}x{h}");
    let mut out = vec![0u8; w * h * 4];
    mem.render_framebuffer(&mut out);

    // Dump tile memory directly: line row y=10 bytes x 80..220, rect y=25 x..
    let mem_bytes = |mem: &MemoryMap, y: usize| -> String {
        let mut s = String::new();
        for x in 80..230usize {
            // tile 0 covers x 0..511; bytes BE; index = y*512 + x at phys 0x10000
            let b = mem.ram.as_slice()[0x10000 + y * 512 + x];
            s.push(if b != 0 { '#' } else { '.' });
        }
        s
    };
    for y in [9usize, 10, 11] {
        eprintln!("mem y={y}: {}", mem_bytes(&mem, y));
    }
    for y in [19usize, 25, 39, 41] {
        eprintln!("mem y={y}: {}", mem_bytes(&mem, y));
    }

    // ASCII art of rendered region x 700..1000, y 0..60 (post-scan-out flip)
    for y in 0..60 {
        let mut line = String::new();
        for x in 700..1000 {
            let o = (y * w + x) * 4;
            line.push(if out[o] > 0 || out[o + 1] > 0 || out[o + 2] > 0 { '#' } else { '.' });
        }
        eprintln!("{line}");
    }
}
