# AGENTS.md

Guidance for AI coding agents working in this repository.

## Project overview

This is an emulator for the SGI O2 (IP32) workstation. **Phase 1 (Research) is complete thanks to the O2Emu Project from myself, A C++ version of this one.** — every hardware subsystem has a
sourced register map under `docs/`, and the full-fledged Rust emulator is
our current task. We are currently in **Phase 2 (Core Emulation.)**.

## Phases

- **Phase 1 — Research ✅ complete:** exhaustive, well-sourced hardware
  documentation under `docs/`. All register maps sourced.
- **Phase 2 — Emulation (current):** a full-fledged **Rust** emulator, as
  accurate to the hardware as the documentation and specs allow. CPU, memory,
  graphics, I/O, and the PROM firmware, with a GUI powered by egui.
- **Phase 3 — Performance & polish (After Phase 2.):** JIT compilation, optimizations,
  GUI improvements, cross-platform support, and full-featured emulator features.

See `ROADMAP.md` for the full plan.

## Repository layout

```
README.md            # GitHub-facing overview
AGENTS.md            # This file
CLAUDE.md            # Claude-specific guidanceROADMAP.md           # Full project plan (phases 1-3)docs/                # All research documentation (the current deliverable)
  README.md          #   docs index
  architecture.md    #   system architecture
  cpu-memory.md      #   CPU & memory
  graphics.md        #   CRM/ICE/MRE/Display
  io.md              #   MACE, PCI, SCSI, Ethernet
  register-maps.md   #   register maps from drivers
  prom.md            #   IP32 PROM firmware
  sources.md         #   research sources
samples/             # firmware images
  ip32prom.rev4.18.bin
  ip32prom.rev4.3.bin
```

## Conventions

- **Accuracy is the product.** Every emulated register, address, and behavior
  must be backed by a source in `docs/` (ASIC spec, driver, header, or leaked
  IRIX source). Do not invent register names, addresses, or behaviors.
- **Every hardware claim must be sourced.** Cite the driver file, header, or
  leaked IRIX source that backs each register/address/behavior.
- **Keep docs concise.** A few lines per section, not essays. Use tables for
  register maps and address layouts.
- **Update the index.** When adding a doc, add a row to `docs/README.md`.
- **Update `docs/sources.md`** when you discover a new source (repo, driver,
  datasheet, forum thread).
- **Preserve the PROM checksum.** The IP32 PROM uses a two's complement
  checksum; never modify a firmware image without recomputing it.

## Key facts to remember

- O2 = SGI IP32, codename "Moosehead".
- UMA: CPU, graphics, and I/O share main memory over a 133 MHz 144-bit bus.
- Graphics = CRM chipset (Microprocessor + ICE, MRE, Display ASICs).
- I/O = MACE ASIC (PCI, ISA, PS/2, Ethernet).
- PROM reset vector: `0xBFC00000`; firmware VMA `0x81000000`.
- PROM images in `samples/`; rev4.18 MD5 `c9725e036052cf1f3e6258eb9bc687fa`.

## License

BSD 3-Clause License. See [LICENSE](LICENSE).

## Research sources

- Linux: `arch/mips/sgi-ip32/`, `arch/mips/include/asm/ip32/`, `drivers/video/crmfb.c`
- NetBSD: `sys/arch/sgimips/`
- Leaked IRIX source: `calmsacibis995/irix-657m-src` (6.5.7m) and
  `jacklin9/IRIX-6.5.17-Src` (6.5.17) — includes the actual IP32 PROM source
  under `stand/arcs/`
- `mattst88/ip32prom-decompiler` — Rust PROM decompiler

## When in doubt

Phase 1 (research) is complete; Phase 2 (Emulation.) is in scope. When hardware behavior is unclear, consult the ASIC
specs in `docs/manuals-specs/` first, then the driver sources. Ask the user
before deviating from documented behavior.