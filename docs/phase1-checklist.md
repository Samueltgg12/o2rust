# Phase 1 Research Checklist

Phase 1 is complete when every hardware subsystem has a sourced register map
and the physical memory map is documented. No emulator implementation belongs
in this phase.

## Hardware documentation

- [x] System architecture and UMA overview
- [x] CPU families, CP0, caches, TLB, and CPU identification
- [x] CRIME memory controller and interrupt registers
- [x] PROM-visible MRE/RE/DE/MTE registers
- [x] GBE / Display Engine register map
- [x] ICE register map (VICE Design Spec 099-0123-003, transcribed into
      [register-maps.md](register-maps.md) "VICE / ICE")
- [x] PROM-level MACE Ethernet register observations
- [x] PROM-level MACE audio ring and codec register observations
- [x] AD1843 codec register map (IRIX `ad1843.h` + datasheet in
      `docs/datasheets/ad1843/`)
- [x] AIC-7880 SCSI references (IRIX `adp78.h`/`adp78.c`, Linux `aic7xxx/`;
      no public datasheet exists — Adaptec ASIC docs were NDA-only)
- [x] SDRAM/DIMM bank registers and PROM probing code (CRIME bank control)
- [x] SGI ASIC specs collected: CRIME, MACE, GBE, VICE
      (`docs/manuals-specs/`)
- [x] Complete MACE Ethernet packet and descriptor semantics (NetBSD
      `if_mecreg.h`, Linux `meth.h` — full MAC110 register map)
- [x] Complete MACE audio codec and `mavb` semantics (Linux `struct
      mace_audio` + `sgio2audio.c`; AD1843 register map)
- [x] DIMM SPD address, EEPROM layout, and probing behavior — resolved: no
      SPD; PROM sizes banks by write/read probing (`post1mem.c` `SizeMEM()`)
- [x] Per-mode GBE timing tables and monitor behavior (IRIX `crm_timing.h`,
      transcribed into [graphics.md](graphics.md))
- [x] PCI configuration-space details (Linux `ops-mace.c`, `pci-ip32.c`,
      `fixup-ip32.c`)
- [x] UART, RTC, PS/2, I2C, and physical address maps
- [x] PROM layout, reset path, POST, and firmware behavior

## Source and consistency review

- [x] Cross-check every register table against PROM, Linux, NetBSD, or IRIX
      (base addresses verified: CRIME `0x14000000`, RENDER `0x15000000`,
      MACE `0x1f000000`, GBE `0x16000000`, UART/RTC under MACE ISA_EXT)
- [x] Record all newly discovered repositories, datasheets, and documents
      ([sources.md](sources.md))
- [x] Remove stale or contradictory source paths and claims
- [x] Complete documentation indexes and cross-links
      ([README.md](README.md))
- [x] Review the Phase 1 exit criteria
- [x] Freeze the research baseline before Phase 2 scaffolding

## Phase transition

- [x] Update `ROADMAP.md` to mark Phase 1 complete
- [x] Define Phase 2 emulator scope and first boot milestone (see
      `ROADMAP.md` Phase 2)
- [ ] Create the C++/CMake project only after the research baseline is frozen