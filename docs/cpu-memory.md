# O2 (IP32) CPU & Memory

## CPU options

| CPU | Speed | Notes |
|-----|-------|-------|
| MIPS R5000 | 180–350 MHz | 1 MB L2 cache |
| MIPS RM7000 | low-end | |
| MIPS R10000 | 150–400 MHz | |
| MIPS R12000 | high-end | |

- 200 MHz R5000 + 1 MB L2 is faster than 180 MHz + 512 KB
- Hobbyist retrofit: 600 MHz RM7xxx (sgidepot.co.uk/o2cpumod.html)

## CPU identification (decompiled PROM `definitions.h`)

The PROM identifies the CPU from the CP0 `PRID` register (register 15). The
implementation field (`PRID_IMP_MASK = 0xff00`, shifted right 8) selects the
CPU family:

| PRID impl | CPU |
|-----------|-----|
| `0x04` | R4000 |
| `0x20` | R4600 |
| `0x21` | R4700 |
| `0x23` | R5000 |
| `0x27` | RM7000 |
| `0x28` | Nevada (R10000-family) |

> **Nevada quirk:** `get_prid` (firmware `0x81004e04`) reports a Nevada CPU
> (`PRID_IMP_NEVADA`) as an R5000 by OR-ing the R5000 impl field into the
> revision. The O2 treats Nevada as R5000-compatible for TLB/cache purposes.

### TLB entry count (per CPU)

| CPU | TLB entries |
|-----|-------------|
| R5000 | 48 |
| RM7000 | 48 |
| R10000 | 64 |

Source: `definitions.h` (`R5000_NUM_TLB_ENTRIES`, `RM7000_NUM_TLB_ENTRIES`,
`R10000_NUM_TLB_ENTRIES`).

## CPU architecture (NEC VRSeries datasheets)

The O2 CPUs are NEC VRSeries parts. The datasheets in
[`docs/datasheets/CPU/`](datasheets/CPU/) cover the VR5000/VR10000 instruction
set (DSA-446416, NEC U12754E) and the VRSeries programming guide (DSA-446414,
NEC U10710E).

### Pipeline

| CPU | Pipeline | Stages |
|-----|----------|--------|
| VR5000 | 2-way superscalar | 5 |
| VR10000 | 4-way superscalar, out-of-order | 7 |

- VR5000: 2-way superscalar, in-order, with stall/slip interlocks.
- VR10000: 4-way superscalar, out-of-order; pipeline flow is not interrupted
  by interlocks.
- One-cycle branch delay and one-cycle load delay (VR5000); the load delay is
  hidden by hardware stalling.

### Physical address space

| CPU | Space | Address width |
|-----|-------|---------------|
| VR5000 | 64 GB | 36 bits |
| VR10000 | 1 TB | 40 bits |

### Primary cache (Table 3-1, DSA-446414)

| CPU | Cache | Size | Line size | Index |
|-----|-------|------|-----------|-------|
| VR5000 | I-cache | 32 KB | 8 words | vAddr13..5 |
| VR5000 | D-cache | 32 KB | 8 words | vAddr13..5 |
| VR10000 | I-cache | 32 KB | 16 words | vAddr13..6 |
| VR10000 | D-cache | 32 KB | 8 words | vAddr13..5 |

- VR5000 primary cache line: `P` (parity), `F` (fill), `PState` (2-bit state),
  `ICDEC` (I-cache predecode), `PTag` (24-bit physical tag, bits 31..12).
- VR10000 primary cache is 2-way set-associative; bit 0 of the index selects
  the way.

### Secondary cache

- **VR5000:** on-chip secondary cache controller; external SRAM. Line format:
  `VIdx` (3-bit primary cache index, vAddr14..12), `SState` (3-bit state),
  `STag` (32-bit tag), 8×64-bit data with parity.
- **VR10000:** on-chip secondary cache controller; line format: `ECC` (7-bit
  tag ECC + 9-bit data ECC), `Tag` (26-bit), 4×128-bit data with parity.

### Cache instructions (CACHE op)

The 5-bit suboperation code `op` = `op4..2` (operation) + `op1..0` (target):

| op1..0 | Target |
|--------|--------|
| 0 | Primary I-cache |
| 1 | Primary D-cache |
| 3 | Secondary cache |

VR5000 operations (`op4..2`): `0` Index_Invalidate/Index_Writeback_Invalidate/
Flash, `1` Index_Load_Tag, `2` Index_Store_Tag, `3` Create_Dirty_Exclusive,
`4` Hit_Invalidate, `5` Fill/Hit_Writeback_Invalidate/Page_Invalidate,
`6` Hit_Writeback.

VR10000 operations (`op4..2`): `0` Index_Invalidate/Index_Writeback_Invalidate,
`1` Index_Load_Tag, `2` Index_Store_Tag, `4` Hit_Invalidate,
`5` Cache_Barrier/Hit_Writeback_Invalidate, `6` Index_Load_Data,
`7` Index_Store_Data.

### TLB

- Fully associative; each entry maps an even/odd page pair (VPN2 in EntryHi,
  PFN in EntryLo0/EntryLo1).
- Page sizes (VR5000/VR10000, Table 4-2): 4 KB, 16 KB, 64 KB, 256 KB, 1 MB,
  4 MB, 16 MB (set via PageMask MASK field).
- TLB instructions: `TLBP` (probe), `TLBR` (read), `TLBWI` (write index),
  `TLBWR` (write random).
- `Random` register decrements each instruction; range is `Wired`..max
  (47 for VR5000, 63 for VR10000). `Wired` entries are not replaced by TLBWR.

## CP0 registers (decompiled PROM `definitions.h`)

| # | Register | # | Register |
|---|----------|---|----------|
| 0 | Index | 16 | Config |
| 1 | Random | 17 | LLAddr |
| 2 | EntryLo0 | 18 | WatchLo |
| 3 | EntryLo1 | 19 | WatchHi |
| 4 | Context | 20 | XContext |
| 5 | PageMask | 21 | FrameMask |
| 6 | Wired | 22 | Diagnostic |
| 7 | Info | 23 | Debug |
| 8 | BadVAddr | 24 | DEPC |
| 9 | Count | 25 | Performance |
| 10 | EntryHi | 26 | ECC |
| 11 | Compare | 27 | CacheErr |
| 12 | Status | 28 | TagLo |
| 13 | Cause | 29 | TagHi |
| 14 | EPC | 30 | ErrorEPC |
| 15 | PRId | 31 | DESAVE |

### CP0 Status bits

`IE`(0), `EXL`(1), `KX`(7), `IM`(8..15), `DE`(16), `CH`(18), `NMI`(19),
`SR`(20), `BEV`(22), `FR`(26), `CU0`(28), `CU1`(29).

### CP0 Config bits

`CM`(0..2, cachable noncoherent=3), `CU`(3), `DB`(4), `IB`(5), `DC`(6..8),
`IC`(9..11), `SC`(17), `SB`(22..23).

CPU-specific Config bits:

| CPU | Bits |
|-----|------|
| R5000 | `SE`(12), `SS`(20..21) |
| RM7000 | `TE`(12) |
| R10000 | `SS`(16) |

### R10000 cache block sizes

`L1I` = 0x40 (64 B), `L1D` = 0x20 (32 B), `L2` = 0x10 (16 B).

## CPU init in the PROM

- `get_prid` (`0x81004e04`) — reads PRID, maps Nevada→R5000.
- `r4600_cpu_init` (`0x810041d4`) / `r4600_cpu_setup` (`0x81006170`) — R4600
  family setup.
- `F_0x8100e080`..`F_0x8100e1c0` — per-CPU WatchLo/WatchHi setup, gated on
  PRID (R4600/R4700/R5000/RM7000/Nevada).
- `F_0x81004de0`/`F_0x81004df0` — read/write CP0 Config.
- `soft_reset`/`hard_reset` (`0x81004e34`/`0x81004e50`) — write
  `CRIME_CONTROL_SOFT_RESET`/`HARD_RESET` to the CRIME control register.

## Memory

- **8 DIMM slots**, proprietary 239-pin SDRAM DIMMs
- Up to **1 GB** total
- Up to **128 MiB/stick** (reverse-engineered; forums.irixnet.org thread-4794)
- **ECC** supported

## Memory controller (CRIME)

CRIME is the memory controller / system controller ASIC.

- 133 MHz 144-bit bus (128-bit data + ECC)
- Buffered to a 66 MHz 256-bit memory system
- Provides the top-level interrupt controller
- See [register-maps.md](register-maps.md) for CRIME registers

### CRIME base addresses (decompiled PROM `definitions.h`)

- `PHYS_BASE_CRIME = 0x14000000` (CRIME CPU interface)
- `PHYS_BASE_RENDER = 0x15000000` (render engine interface)

> Note: an older IRIX `crm_stand.h` lists the render base as `0x14100000`;
> the decompiled PROM (authoritative) uses `0x15000000`.

### CRIME memory controller registers (IRIX `stand/arcs/IP32prom/include/sys/crime.h`)

Offsets are relative to `CRM_BASEADDR`:

| Register | Offset | Notes |
|----------|--------|-------|
| `CRM_MEM_CONTROL` | `0x200` | 2-bit; `ECC_ENA` (0x10), `USE_ECC_REPL` (0x20) |
| `CRM_MEM_BANK_CTRL(x)` | `0x208 + x*8` | 8 banks; `SDRAM_SIZE` (0x100), `ADDR` (0x1f) |
| `CRM_MEM_REFRESH_CNTR` | `0x248` | 11-bit refresh counter |
| `CRM_MEM_ERROR_STAT` | `0x250` | 28-bit error status |
| `CRM_MEM_ERROR_ADDR` | `0x258` | error address (bits 29:0) |
| `CRM_MEM_ERROR_ECC_SYN` | `0x260` | ECC syndrome |
| `CRM_MEM_ERROR_ECC_CHK` | `0x268` | generated ECC check bits |
| `CRM_MEM_ERROR_ECC_REPL` | `0x270` | ECC replacement bits |

- Bank address decode: `CRM_MEM_BANK_CTRL_BANK_TO_ADDR(x) = ((x & 0x1f) << 25)`
- Memory error address correction (IRIX `irix/kern/ml/error.c`): CRIME reports
  only bits 29:0. Memory below 256 MB is accessed at the 0-based alias; memory
  at/above 256 MB is accessed above 1 GB, so `if (physaddr >= 0x10000000)
  physaddr += 0x40000000`.

### Memory banks

- Up to 8 banks (`CRM_MAXBANKS = 8`)
- Each bank is either **32 MB** or **128 MB** (64 Mbit SDRAM)
- POST sizes banks by probing (IRIX `stand/arcs/IP32prom/post/post1mem.c`)

### DIMM SPD status — resolved: no SPD

**The O2 does not use SPD EEPROM for memory sizing.** The PROM sizes memory
entirely by probing (IRIX `stand/arcs/IP32prom/post/post1mem.c`, `SizeMEM()`):

1. ECC checking is disabled (`CRM_MEM_CONTROL &= ~CRM_MEM_CONTROL_ECC_ENA`).
2. For each of the 8 banks, all bank control registers from the current bank
   onward are pointed at the current probe address with the 128 MB size bit
   set (`CRM_MEM_BANK_CTRL_SDRAM_SIZE`), and a 128 MB TLB page is mapped.
3. Four 64-bit test values are written at offsets 0, 32 MB−8, 32 MB, and
   128 MB−8 from the bank base, followed by three dummy writes to clear the
   memory bus.
4. Read-back distinguishes the cases:
   - If the 32 MB aliases overwrote the 0/128 MB locations → **32 MB bank**
     (16 Mbit SDRAM; address wraps within 32 MB).
   - If all four locations read back distinctly → **128 MB bank**
     (64 Mbit SDRAM).
   - Otherwise → no DIMM in the bank.
5. Bank 0 must be populated; if not, the PROM blinks the dual-color LED amber
   and halts.
6. Second/third passes then remap all 128 MB banks to the lowest physical
   addresses, followed by the 32 MB banks (128 MB banks first so the memory
   map is contiguous with large pages low).

The decompiled PROM's MACE I2C routines access only the display/flat-panel
path; no DIMM SPD transaction exists in the PROM. The proprietary 239-pin
DIMMs carry no SPD EEPROM that the firmware reads.

## Physical memory map

| Range | Purpose |
|-------|---------|
| `0x00000000`–`0x0fffffff` | Main memory (up to 256 MB, 0-based alias) |
| `0x10000000`–`0x13ffffff` | Memory above 256 MB (aliased above 1 GB) |
| `0x14000000` | CRIME CPU interface |
| `0x15000000` | Render engine interface |
| `0x1f000000` | MACE (I/O engine) |
| `0x1fc00000` | PROM (KSEG1 view of reset vector `0xBFC00000`) |

Source: decompiled PROM `definitions.h` (`PHYS_BASE_CRIME`, `PHYS_BASE_RENDER`,
`PHYS_BASE_MACE`, `PHYS_SYSTEM_ROM`).

## Memory & Rendering Engine (MRE)

- Contains the memory controller
- Rasterization + texture mapping
- UMA: textures and framebuffer come from main memory

## Linux memory setup

- `arch/mips/sgi-ip32/ip32-memory.c` — memory setup
- `arch/mips/sgi-ip32/ip32-dma.c` — DMA setup
- `arch/mips/sgi-ip32/crime.c` — CRIME driver

## NetBSD memory setup

- `sys/arch/sgimips/dev/imc.c` + `imcreg.h` — IMC (memory controller) driver
- `sys/arch/sgimips/dev/crime.c` — CRIME driver

## TODO / Open questions

- [x] CPU architecture (R5000/R10000/R12000) — from NEC VRSeries datasheets
- [x] PROM-visible MRE/RE/DE/MTE register map — see [register-maps.md](register-maps.md)
- [ ] DIMM SPD / configuration details (EEPROM address, bytes, and probe path)
- [ ] Full CRIME timing/refresh programming sequence