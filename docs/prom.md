# IP32 PROM Firmware

The O2 (IP32) PROM is the firmware that boots the machine. This document covers
the firmware images we have, their internal structure, and the tools used to
decompile them.

## Firmware images (in `samples/`)

The actual PROM binaries are already present in the workspace:

| File | Size | MD5 |
|------|------|-----|
| `samples/ip32prom.rev4.18.bin` | 524 KB (536,576 B) | `c9725e036052cf1f3e6258eb9bc687fa` |
| `samples/ip32prom.rev4.3.bin`  | 524 KB (536,576 B) | `6b86b20727a598ed15d93f27c9a3f2e8` |

- `ip32prom.rev4.18.bin` is the version the `ip32prom-decompiler` tool expects
  (its MD5 matches the tool's expected hash exactly).
- Both are raw binary images (the `file` tool misidentifies them as "TeX font
  metric data" — they are actually raw MIPS firmware, not a recognized format).

## Reset vector

- The PROM is mapped at the MIPS reset vector **`0xBFC00000`** (the standard
  MIPS uncached boot address, `0x1FC00000` in the KSEG1 view).
- On reset the CPU begins executing here, so the first bytes of the image are
  the initial jump into the PROM's real entry code.

## Image structure (5-section SHDR layout)

The image is not a single flat blob. It is organized into **5 sections**, each
with a section header. The layout is:

| Section | Purpose |
|---------|---------|
| 1 | Main firmware code/data |
| 2 | Main firmware code/data (continuation) |
| 3 | Main firmware code/data (continuation) |
| 4 | **Version section** — contains an embedded ELF header |
| 5 | **Checksum section** — stores the image checksum |

### Section checksums

Each section carries a checksum value. The known section checksum values
(hex) are:

| Section | Checksum |
|---------|----------|
| 1 | `0x00000000` |
| 2 | `0x00000000` |
| 3 | `0x00000000` |
| 4 | `0x00000000` |
| 5 | `0x00000000` |

> The exact per-section checksum values for rev 4.18 are computed by the
> decompiler tool; the above are the placeholder/zero values from the tool's
> section table. The real values are derived from the image contents.

## Firmware section (VMA `0x81000000`)

The main firmware section is loaded at virtual address **`0x81000000`** and is
further subdivided into subsections. The firmware is a MIPS executable that
implements the PROM's console, diagnostics, and boot loader.

## Version section (embedded ELF header)

Section 4 contains an **ELF header** embedded in the image. This is used to
identify the firmware version and its load layout. The decompiler parses this
to locate the firmware entry point and section layout.

## Checksum algorithm

The image uses a **two's complement checksum**:

- The checksum is computed over the image such that the sum of all bytes
  (including the stored checksum) equals zero in two's complement arithmetic.
- This is a common firmware integrity scheme: `checksum = -(sum of all other
  bytes)`.

## Decompilation tool: `mattst88/ip32prom-decompiler`

- **Repo:** `https://github.com/mattst88/ip32prom-decompiler`
- **Language:** Rust
- **Purpose:** Decompiles the IP32 PROM into a form suitable for analysis and
  modification. The author's motivation is a **900 MHz RM7900 CPU upgrade** for
  the O2 — the stock PROM does not know the RM7900, so it must be patched.
- **Expected input:** `ip32prom.rev4.18.bin` (MD5 `c9725e036052cf1f3e6258eb9bc687fa`).
- **What it does:**
  - Parses the 5-section SHDR structure.
  - Locates the firmware section at VMA `0x81000000` and its subsections.
  - Parses the embedded ELF header in the version section.
  - Recomputes/verifies the two's complement checksum.
  - Disassembles the MIPS firmware for analysis.

### Author's blog post

The author documented the reverse-engineering process in a blog post covering:
- The motivation (RM7900 CPU upgrade).
- The discovery of the 5-section SHDR layout.
- The firmware VMA `0x81000000` and subsection structure.
- The embedded ELF header in the version section.
- The two's complement checksum scheme.

## Decompiled PROM (in `samples/decompiled-prom/`)

The PROM has been decompiled into MIPS assembly. The output is in
`samples/decompiled-prom/` with separate directories for each revision:

| File | Contents |
|------|----------|
| `definitions.h` | Auto-generated constants: full IP32 address map, CRIME/MACE/UART/RTC registers, ARCS SPB/RTSB/firmware-vector structures, CP0/CP1 definitions |
| `macros.inc` | Assembly macros for SHDR headers, subsections, checksums, HI/LO address construction |
| `post1.S` | POST (Power-On Self Test) — memory sizing, subsection copy, checksum verify, TLB init |
| `sloader.S` | Secondary loader |
| `env.S` | Default environment variables |
| `firmware.S` | Main firmware (console, diagnostics, boot loader) |
| `version.S` | Version section (embedded ELF header + version string) |
| `trailing.S` | Trailing data |

### POST entry (`post1.S`)

- `post1_entry` at `0xbfc04400`; `post1` at `0xbfc04448`
- Copies loadable subsections to their destinations, verifying checksums
- Sizes memory banks by probing
- Initializes the TLB (`tlb_init_preserve`)
- Sets KSEG0 to cacheable, noncoherent via `CP0_CONFIG`
- Uses `BASE_CRIME + CRIME_MC_STATUS_CTRL` and `BASE_ISA + ISA_MISC_CONTROL`
  (LED control)

### Default environment (`env.S`)

The env section loads at `0xbfc04040` and contains the default environment:

```text
AutoLoad=Yes
console=g
 diskless=0
dbaud=9600
volume=80
sgilogo=y
monitor=h
TimeZone=PST8PDT
netaddr=192.168.1.25
crt_option=1
```

### Version string (`version.S`)

```text
VERSION 4.18
O2 R5K/R7K/R10K/R12K
IRIX 6.5.x IP32prom IP32PROM-v4
```

## PROM source code (in leaked IRIX source)

The leaked IRIX 6.5.7m source (`calmsacibis995/irix-657m-src`) contains the
**actual IP32 PROM source code** under `stand/arcs/`. This is the definitive
reference for what the PROM does.

### `stand/arcs/IP32prom/` — the PROM itself

- `include/crm_i2c.h` — EDID (Extended Display Identification Data) structure
  used to read monitor EDID over I2C.
- `debugcard/triton.sim/tst.r5k` — a simulation test script (runs `mace` and
  `cpu` simulators in `xwsh` windows) for the Triton debug card.

### `stand/arcs/ide/IP32/` — IP32 Integrated Diagnostics Environment (IDE)

The IDE is the diagnostic/test firmware that runs from the PROM:

- `graphics/crmGfxState.c` — CRM graphics state setup. Contains the real O2
  framebuffer register definitions:
  - `GBE_FRM_DEPTH_8/16/32` and `GBE_FRM_DEPTH_SHIFT/MASK` — framebuffer depth
  - `GBE_FRM_HEIGHT_PIX_SHIFT/MASK` — framebuffer height
  - `IDE_FB_TILE_BASE` — framebuffer tile base address
  - `IDE_FB_DLIST1_BASE` — display list base
  - Tile-based framebuffer layout: normal tiles 512 px wide, overlay tiles
    512 px wide, 128 px tall.
  - Supports 8/16/32-bit depths, 1600SW flat panel (1600x1024@50Hz).
- `mace/siodiag2.c` — MACE serial I/O diagnostics (16550-style UART registers:
  DLM, IIR/FCR, LCR, MCR, LSR, MSR, SCR).
- `mem/khlow_diag.s` — low-memory diagnostics in MIPS assembly (uses
  `K1_RAMBASE`, `C0_SR`, exception vectors, `bcopy`).

### `stand/arcs/lib/libsk/graphics/CRIME/crm_init.c` — CRIME graphics init

- `initGraphics()` — full graphics console init sequence:
  `initDisplay()`, `initCrime()`, `initGammaMap()`, `turnOnGbe()`,
  `initFramebuffer()`, `initTiming()`, `initCursor()`.
- `crmGetRev()` reads the CRIME revision (`0x000000a1`).
- Handles the 1600SW flat panel via `i2cfp_PanelOff()`.

## Related community resources

- `forums.sgi.sh` thread 1508 — O2 PROM / firmware discussion.
- Hackaday / Adafruit / LavX articles on the O2 CPU upgrade and PROM work.

## Relevance to the emulator

- The PROM is the first code the emulator must execute (at `0xBFC00000`).
- Understanding the 5-section layout and the embedded ELF header is needed to
  load the image correctly into emulated memory.
- The two's complement checksum must be preserved if the image is ever modified.
- The decompiler's disassembly output is a reference for what the PROM does at
  boot (console init, memory sizing, device probing, boot device selection).