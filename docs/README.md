# Attention
The "docs" (this folder), "samples" and other folder/md files come from the O2Emu Project (https://github.com/samueltgg12/o2emu) made by myself. Use this folder for all documentation and when implementing all components of the emulator.



# SGI O2 Emulator — Hardware Documentation

**Phase 1 (Research) and Phase 2 (Emulation) are complete.** Every hardware
subsystem has a sourced register map, and the full C++ emulator is
implemented. These docs are the authoritative reference for the emulator.

See [ROADMAP.md](../ROADMAP.md) for the full project plan (Phase 3 =
JIT/optimizations/GUI polish/cross-platform).

## Machine Overview

- **Codename:** "Moosehead", SGI IP32
- **Architecture:** Proprietary high-bandwidth **Unified Memory Architecture (UMA)**.
  PCI bus bridged onto UMA with a single expansion slot.
- **CPU options:**
  - MIPS R5000 (180–350 MHz) or RM7000 (low-end)
  - MIPS R10000 (150–400 MHz) or R12000 (high-end)
  - 200 MHz R5000 + 1 MB L2 is faster than 180 MHz + 512 KB
  - Hobbyist retrofit of 600 MHz RM7xxx exists (sgidepot.co.uk/o2cpumod.html)
- **Memory:** 8 DIMM slots, up to 1 GB using proprietary 239-pin SDRAM DIMMs,
  up to 128 MiB/stick (reverse engineered — forums.irixnet.org thread-4794).
  Memory & Rendering Engine (MRE) ASIC contains the memory controller.
  133 MHz 144-bit bus (128-bit data + ECC), buffered to a 66 MHz 256-bit memory system.
- **I/O:** IO Engine (MACE) ASIC provides 64-bit PCI bus, ISA bus, 2 PS/2 ports,
  10/100 Base-T Ethernet. ISA bus is used solely for the Super I/O chip
  (serial/parallel ports).
- **Disks:** UltraWide SCSI (Adaptec AIC-7880). R5000/RM7000 units have 2 drive
  sleds; R10000/R12000 units have 1 (heat constraints). Toshiba SCSI CD/DVD-ROM.
- **Graphics:** CRM chipset (Microprocessor + ICE, MRE, Display ASICs).
  - Microprocessor: display list/vertex processing + MRE control
  - ICE (Imaging & Compression Engine): pixel packaging/unpacking
  - MRE: rasterization + texture mapping
  - Display Engine: analog video generation
  - UMA means textures/framebuffer come from main memory
- **Operating systems:** IRIX 6.3/6.5.x (native), Linux (Debian/Gentoo/T2),
  OpenBSD (3.7–6.9), NetBSD (since 2.0, first open-source port).

## Documentation Index

| Topic | File |
|-------|------|
| System architecture | [architecture.md](architecture.md) |
| CPU & memory | [cpu-memory.md](cpu-memory.md) |
| Graphics (CRM/ICE/MRE/Display) | [graphics.md](graphics.md) |
| I/O (MACE, PCI, SCSI, Ethernet) | [io.md](io.md) |
| Register maps (from drivers) | [register-maps.md](register-maps.md) |
| PROM firmware | [prom.md](prom.md) |
| Research sources | [sources.md](sources.md) |
| Phase 1 checklist | [phase1-checklist.md](phase1-checklist.md) |
| SGI ASIC specs (CRIME/MACE/GBE/VICE) | [manuals-specs/](manuals-specs/) |
| Datasheets (CPU, AD1843, O2 video option) | [datasheets/](datasheets/) |

## Decompiled PROM

The PROM has been decompiled into MIPS assembly under
[`samples/decompiled-prom/`](../samples/decompiled-prom/). The auto-generated
`definitions.h` is the authoritative source for the IP32 address map and the
CRIME/MACE/UART/RTC register maps (see [register-maps.md](register-maps.md)).