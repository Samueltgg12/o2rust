//! End-to-end exercise of the emulated MACE audio block, driving it with the
//! exact register sequence the IP32 PROM's `play_hello_tune` uses
//! (see `samples/irixsrc/.../IP32prom/fw/hello_tune.c` and the rev4.18
//! disassembly): ISA_RINGBASE -> channel-2 CHAN_RESET / CHAN_DMA_ENABLE ->
//! fill ring -> write pointer, with the codec consumption paced in emulated
//! time (`MemoryMap::audio_tick`). Captured samples must reach the host ring.

use o2rust::system::Emulator;

/// MACE audio register offsets (MACE-relative). MACE MMIO is at physical
/// 0x1f00_0000; the peripheral block (audio/ISA/...) at MACE offset 0x300000.
const MACE_BASE: u32 = 0x1f00_0000;
const MACE_PERIF: u32 = MACE_BASE + 0x300000;

const AUD_CNTRL_STAT: u32 = MACE_PERIF + 0x00;
const AUD_CODEC_REG: u32 = MACE_PERIF + 0x08;
const AUD_CH2_CNTRL: u32 = MACE_PERIF + 0x40;
const AUD_CH2_WRITE: u32 = MACE_PERIF + 0x50;

const ISA_RING_BASE: u32 = MACE_PERIF + 0x10000 + 0x00;

/// DMA ring page base (RAM, physical) and the DAC1 ring page (channel `2`
/// register set maps to DMA channel 1, page +0x1000).
const RING_PAGES: u32 = 0x0040_0000;
const RING: u32 = RING_PAGES + 0x1000;

/// AD1843 clock generator 1 rate: the boot tune uses 34000 Hz.
const CLK_GEN1_RATE: u32 = 17;

#[test]
fn prom_style_dma_ring_flows_to_host_consumer() {
    let mut emu = Emulator::with_ram_no_console(256);
    let mut consumer = emu.memory.mace.take_audio_consumer().unwrap();

    // Codec setup: CLK_GEN1_RATE = 34000 via the codec command port
    // (command = (addr << 17) | rate).
    emu.memory.write32(AUD_CODEC_REG, 0);
    emu.memory.write32(AUD_CODEC_REG + 4, (CLK_GEN1_RATE << 17) | 34000);

    // Ring base + channel 2 (DAC1) DMA: reset, then enable.
    emu.memory.write32(ISA_RING_BASE, RING_PAGES);
    emu.memory.write32(AUD_CH2_CNTRL, 0);
    emu.memory.write32(AUD_CH2_CNTRL + 4, 0x400); // CHAN_RESET
    emu.memory.write32(AUD_CH2_CNTRL, 0);
    emu.memory.write32(AUD_CH2_CNTRL + 4, 0x200); // CHAN_DMA_ENABLE

    // Pre-fill the DAC1 ring with 64 stereo frames of a rising sawtooth.
    // Each frame is <left><right> 32-bit samples, left-justified (sample << 8).
    const FRAMES: u32 = 64;
    for i in 0..FRAMES {
        let s = ((i as i32) * 1000 - 32000).wrapping_mul(256);
        let entry = ((s as u32 as u64) << 32) | (s as u32 as u64);
        emu.memory.write64(RING + i * 8, entry);
    }
    emu.memory.write32(AUD_CH2_WRITE, 0);
    emu.memory.write32(AUD_CH2_WRITE + 4, FRAMES * 8);

    // ~1 ms of emulated codec clock at 133 MHz ticks.
    emu.memory.audio_tick(133_000);

    let mut got = Vec::new();
    // First window: the codec should have consumed ~34 of the 64 frames.
    let mut fast = Vec::new();
    while let Some(s) = consumer.pop().ok() {
        fast.push(s);
    }
    assert!(
        (30..=40).contains(&fast.len()),
        "1ms should yield ~34 frames, got {}",
        fast.len()
    );
    got.extend(fast);

    // Rest of the tune drains at the same rate and never deadlocks on a
    // full ring: all 64 written frames eventually reach the host.
    emu.memory.audio_tick(133_000 * 60);
    while let Some(s) = consumer.pop().ok() {
        got.push(s);
    }
    assert_eq!(got.len(), FRAMES as usize, "all written frames must be drained");

    // First frame: (0*1000-32000) = -32000, scaled to [-1,1].
    assert!((got[0] + 0.9765625).abs() < 1e-6, "first sample got {}", got[0]);
    // Last frame: (63*1000-32000) = 31000.
    let last = got[got.len() - 1];
    assert!((last - 31000.0 / 32768.0).abs() < 1e-6, "last sample got {}", last);
}

#[test]
fn read_alias_tracks_codec_consumption() {
    let mut emu = Emulator::with_ram_no_console(256);
    let mut consumer = emu.memory.mace.take_audio_consumer().unwrap();

    emu.memory.write32(AUD_CODEC_REG, 0);
    emu.memory.write32(AUD_CODEC_REG + 4, (CLK_GEN1_RATE << 17) | 34000);
    emu.memory.write32(ISA_RING_BASE, RING_PAGES);
    emu.memory.write32(AUD_CH2_CNTRL, 0);
    emu.memory.write32(AUD_CH2_CNTRL + 4, 0x400);
    emu.memory.write32(AUD_CH2_CNTRL, 0);
    emu.memory.write32(AUD_CH2_CNTRL + 4, 0x200);

    // Single frame in the ring.
    emu.memory.write64(RING, 1024u64 << 32 | 1024);
    emu.memory.write32(AUD_CH2_WRITE, 0);
    emu.memory.write32(AUD_CH2_WRITE + 4, 8);

    // Fail-safe: fill more frames so a hung pointer still releases later.
    for i in 1..8 {
        emu.memory.write64(RING + i * 8, 1024u64 << 32 | 1024);
    }
    emu.memory.write32(AUD_CH2_WRITE, 0);
    emu.memory.write32(AUD_CH2_WRITE + 4, 64);

    // Advance past the full ring.
    emu.memory.audio_tick(133_000 * 30);
    while consumer.pop().is_ok() {}

    // GET_CH2_READ_ALIAS(cntrl_stat) = (cntrl >> 4) & 0xfe0 must reflect the
    // consumed byte offset: all 64 bytes drained, pointer parked at write_ptr.
    let lo = emu.memory.read32(AUD_CNTRL_STAT + 4);
    let alias = (lo >> 4) & 0xfe0;
    assert_eq!(alias, 64, "ring drained -> read pointer equals write pointer");
}