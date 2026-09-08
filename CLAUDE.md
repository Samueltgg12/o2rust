## Attention
The Original "docs" "samples" "assets" and md files are from the O2Emu Project, an C++ Version of this emulator.



# CLAUDE.md

Project-specific guidance for Claude Code (and other AI assistants) working in
this repository.

## What this project is

An emulator for the **SGI O2 (IP32)** workstation. **Phase 1 (Research) is complete, and
Phase 2 (Emulation) is our current phase** — every hardware subsystem has a sourced
register map under `docs/`, and the full-fledged Rust emulator is what we need to do.
We are currently in **Phase 2 (The Emulation.)**.

## Phases

- **Phase 1 — Research ✅ complete:** exhaustive, well-sourced hardware
  documentation under `docs/`. All register maps sourced.
- **Phase 2 — Emulation (current):** a full-fledged **Rust** emulator, as
  accurate to the hardware as the documentation and specs allow. CPU, memory,
  graphics, I/O, and the PROM firmware, with a GUI.
- **Phase 3 — Performance & polish (After Phase 2):** JIT compilation, optimizations,
  GUI improvements, cross-platform support, and full-featured emulator features.

See `ROADMAP.md` for the full plan.

## How to work here

1. **Start with `docs/README.md`** for the documentation index, then read the
   relevant module doc before answering or editing.
2. **Accuracy is the product.** Every emulated register, address, and
   behavior must be backed by a source in `docs/` (ASIC spec, driver, header,
   or leaked IRIX source).
3. **Every hardware claim must be sourced.** Cite the Linux/NetBSD driver,
   header, or leaked IRIX source that backs each register, address, or
   behavior. Never invent register names or addresses.
4. **Keep docs concise.** Use tables for register maps and address layouts.
5. **Update the index and sources.** Add new docs to `docs/README.md` and new
   sources to `docs/sources.md`.

## Repository layout

```
README.md            # GitHub-facing overview
AGENTS.md            # AI agent guidance (generic)
CLAUDE.md            # This file (Claude-specific)ROADMAP.md           # Full project plan (phases 1-3)docs/                # All research documentation
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

## Key facts

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

## Scope guard

Phase 1 (research) is already complete thanks to the O2Emu Project, The C++ Version of this emulator; Phase 2 (Emulation) is in scope. When hardware behavior is unclear, consult the ASIC
specs pdf's and md's in `docs/manuals-specs/` first, then the driver sources. Ask the user
before deviating from documented behavior.