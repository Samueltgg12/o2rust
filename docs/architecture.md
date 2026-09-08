# O2 (IP32) System Architecture

## Unified Memory Architecture (UMA)

The O2's defining feature is its **Unified Memory Architecture**. All system
components — CPU, graphics, I/O — share a single pool of main memory through a
high-bandwidth proprietary bus. There is no separate VRAM; the framebuffer and
textures live in main memory.

```
                    +----------------------+
                    |   MIPS CPU (R5000/   |
                    |   R10000/R12000)     |
                    +----------+-----------+
                               |  (CPU bus)
                               v
   +----------------+   +------+------+   +----------------+
   |  Display ASIC  |<->|   CRIME     |<->|  MRE (Memory & |
   |  (video out)   |   |  (memory     |   |  Rendering     |
   +----------------+   |  controller) |   |  Engine)       |
                        +------+------+   +----------------+
                               |  UMA bus
                               v
                        +------+------+
                        |   MACE      |  (I/O Engine)
                        |  (IO ASIC)  |
                        +------+------+
                               |
              +----------------+----------------+
              |                |                |
        +-----+-----+    +-----+-----+    +-----+-----+
        |  PCI bus  |    |  ISA bus  |    |  PS/2     |
        | (64-bit)  |    | (Super I/O|    |  Ethernet |
        +-----+-----+    |  chip)    |    +-----------+
              |          +-----------+
        +-----+-----+
        |  SCSI     |
        |  (AIC-7880)|
        +-----------+
```

## ASIC breakdown

| ASIC | Role |
|------|------|
| **CRIME** | Memory controller / system controller. Bridges CPU to UMA. |
| **MACE** | I/O Engine. Provides PCI bus, ISA bus, PS/2, Ethernet, audio. |
| **MRE** | Memory & Rendering Engine. Rasterization + texture mapping. |
| **ICE** | Imaging & Compression Engine. Pixel packaging/unpacking. |
| **Microprocessor** | Display list/vertex processing + MRE control. |
| **Display Engine** | Analog video generation. |

## Key buses

- **CPU bus:** connects CPU to CRIME
- **UMA bus:** connects CRIME to MACE, MRE, Display
- **PCI bus:** 64-bit, provided by MACE, single expansion slot
- **ISA bus:** provided by MACE, used only for Super I/O chip

## Memory system

- 133 MHz 144-bit bus (128-bit data + ECC) from CRIME
- Buffered to a 66 MHz 256-bit memory system
- 8 DIMM slots, proprietary 239-pin SDRAM DIMMs
- Up to 1 GB (128 MiB/stick max, reverse-engineered)
- ECC supported

## Physical memory map

| Range | Purpose |
|-------|---------|
| `0x00000000`–`0x0fffffff` | Main memory (up to 256 MB, 0-based alias) |
| `0x10000000`–`0x13ffffff` | Memory above 256 MB (aliased above 1 GB) |
| `0x14000000` | CRIME CPU interface |
| `0x15000000` | Render engine interface |
| `0x1f000000` | MACE (I/O engine) |
| `0x1fc00000` | PROM (KSEG1 view of reset vector `0xBFC00000`) |

Sources: decompiled PROM `definitions.h` (`PHYS_BASE_CRIME`,
`PHYS_BASE_RENDER`, `PHYS_BASE_MACE`, `PHYS_SYSTEM_ROM`).

## Interrupt architecture

- CRIME provides the top-level interrupt controller
- MACE provides MACEISA interrupts (audio, RTC, keyboard, mouse, timers,
  parallel, serial)
- PCI interrupts are routed through MACE to CRIME
- See [register-maps.md](register-maps.md) for the full interrupt map