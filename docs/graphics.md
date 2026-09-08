# O2 (IP32) Graphics — CRM Chipset

The O2 graphics subsystem is the **CRM chipset** (Cobalt/RM? — the graphics
chipset). It consists of four ASICs:

| ASIC | Role |
|------|------|
| **Microprocessor** | Display list / vertex processing + MRE control |
| **ICE** | Imaging & Compression Engine — pixel packaging/unpacking |
| **MRE** | Memory & Rendering Engine — rasterization + texture mapping |
| **Display Engine** | Analog video generation |

## Unified Memory Architecture

The O2 has **no separate VRAM**. The framebuffer and textures live in main
memory, accessed over the UMA bus. This is the key architectural difference
from the earlier SGI Indy (which had a separate graphics board).

## Microprocessor

- Display list / vertex processing
- Controls the MRE

## ICE (Imaging & Compression Engine)

- Pixel packaging / unpacking
- Handles image data transfer between the framebuffer and main memory

## MRE (Memory & Rendering Engine)

- Rasterization
- Texture mapping
- Contains the memory controller

### PROM-visible render-engine registers

The decompiled PROM exposes the MRE/CRIME render block at physical base
`0x15000000` (`BASE_RENDER` in `samples/decompiled-prom/rev4.18/definitions.h`).
The recovered offsets are documented in [register-maps.md](register-maps.md)
under “CRIME rendering engine registers”. They cover the render interface,
render TLB, display engine (DE), memory-transfer engine (MTE), status, and
start/flush controls.

No separate ICE register block is named by the PROM definitions or the Linux
and NetBSD sources currently collected. ICE remains an open research item;
pixel-format behavior should not be inferred from the GBE color-mode table.

## Display Engine

- Analog video generation
- Drives the video output

## Linux driver

The O2 framebuffer is the **GBE** (Graphics Back End). In the modern Linux
kernel the driver and register definitions live under `drivers/video/fbdev/`
and `include/video/`:

- `drivers/video/fbdev/gbefb.c` — framebuffer driver
- `include/video/gbe.h` — GBE register definitions (`struct sgi_gbe` + all
  `GBE_*` bit-field macros)

> Note: older references (and this repo's earlier notes) cite
> `drivers/video/crmfb.c` + `crmfbreg.h`. In the current kernel the GBE
> framebuffer is `gbefb.c`/`gbe.h`; the `crmfb` path is stale.

## NetBSD driver

- `sys/arch/sgimips/dev/crmfb.c` + `crmfbreg.h`

## IRIX source (leaked) — real GBE register definitions

The leaked IRIX source (`stand/arcs/ide/IP32/graphics/crmGfxState.c`) contains
the **real O2 framebuffer (GBE) register definitions** used by the PROM's
graphics init:

- `GBE_FRM_DEPTH_8` / `GBE_FRM_DEPTH_16` / `GBE_FRM_DEPTH_32` — framebuffer depth
- `GBE_FRM_DEPTH_SHIFT` / `GBE_FRM_DEPTH_MASK` — depth field in `outputFrm0`
- `GBE_FRM_HEIGHT_PIX_SHIFT` / `GBE_FRM_HEIGHT_PIX_MASK` — height in `outputFrm1`
- `IDE_FB_TILE_BASE` — framebuffer tile base address
- `IDE_FB_DLIST1_BASE` — display list base address

### Tile-based framebuffer layout

The O2 framebuffer is **tile-based** (not a linear scanline buffer):

- Normal tiles: 512 px wide (divided by depth bytes), 128 px tall
- Overlay tiles: 512 px wide, 128 px tall
- Overlay descriptor list is built in `crmGfxState.c`:
  `crmOverlayPtr[i] = (IDE_FB_TILE_BASE + (0x00010000 * (i + maxNormalNumTiles + 1))) >> 16`
- Supported depths: 8, 16, 32 bits
- 1600SW flat panel mode: 1600x1024 @ 50 Hz, 32-bit depth

### CRIME graphics init sequence (`stand/arcs/lib/libsk/graphics/CRIME/crm_init.c`)

`initGraphics()` performs, in order:
1. `crmInitGraphicsBase()`
2. `crmGetRev()` — reads CRIME revision (`0x000000a1`)
3. `initDisplay()`
4. `initCrime()`
5. `initGammaMap()`
6. `turnOnGbe()`
7. `initFramebuffer()`
8. `initTiming()`
9. `initCursor()`

## GBE register map (`include/video/gbe.h`)

The Linux kernel exposes the GBE as `struct sgi_gbe` — a flat array of 32-bit
registers. Offsets are relative to `GBE_BASE = 0x16000000` (SGI O2). The
register block is 1 MiB (`0x100000`), so each register is at
`GBE_BASE + offset`.

### Control / clock / ID block

| Offset | Field | Bits | Meaning |
|--------|-------|------|---------|
| `0x00000` | `ctrlstat` | — | General control / status |
| | `CHIPID` | 3:0 | Chip revision (driver: `gbe_revision = ctrlstat & 15`) |
| | `SENSE_N` | 4 | Monitor sense (active low) |
| | `PCLKSEL` | 29:28 | Pixel clock select |
| `0x00004` | `dotclock` | — | Dot clock PLL control |
| | `M` | 7:0 | PLL multiplier |
| | `N` | 13:8 | PLL divider |
| | `P` | 15:14 | PLL post-divider |
| | `RUN` | 20 | PLL run |
| `0x00008` | `i2c` | — | CRT I2C control |
| `0x0000c` | `sysclk` | — | System clock PLL control |
| `0x00010` | `i2cfp` | — | Flat panel I2C control |
| `0x00014` | `id` | — | Device id / chip revision |
| `0x00018` | `config` | — | Power-on configuration |
| `0x0001c` | `bist` | — | Internal BIST status |

### Video timing block

| Offset | Field | Bits | Meaning |
|--------|-------|------|---------|
| `0x10000` | `vt_xy` | — | Current dot coords |
| | `X` | 11:0 | Current X |
| | `Y` | 23:12 | Current Y |
| | `FREEZE` | 31 | Freeze scan |
| `0x10004` | `vt_xymax` | — | Maximum dot coords |
| | `MAXX` | 11:0 | Max X |
| | `MAXY` | 23:12 | Max Y |
| `0x10008` | `vt_vsync` | — | VSYNC on/off |
| | `VSYNC_OFF` | 11:0 | VSYNC off Y |
| | `VSYNC_ON` | 23:12 | VSYNC on Y |
| `0x1000c` | `vt_hsync` | — | HSYNC on/off |
| `0x10010` | `vt_vblank` | — | VBLANK on/off |
| `0x10014` | `vt_hblank` | — | HBLANK on/off |
| `0x10018` | `vt_flags` | — | Polarity of VT signals |
| | `VDRV_INVERT` | 0 | Invert VDRV |
| | `VDRV_LOW` | 1 | VDRV active low |
| | `HDRV_INVERT` | 2 | Invert HDRV |
| | `HDRV_LOW` | 3 | HDRV active low |
| | `SYNC_HIGH` | 4 | Sync active high |
| | `SYNC_LOW` | 5 | Sync active low |
| | `F2RF_HIGH` | 6 | F2RF high |
| `0x1001c` | `vt_f2rf_lock` | — | F2RF & framelock Y coord |
| `0x10020` | `vt_intr01` | — | Interrupt 0,1 Y coords |
| `0x10024` | `vt_intr23` | — | Interrupt 2,3 Y coords |
| `0x10028` | `fp_hdrv` | — | Flat panel HDRV on/off |
| `0x1002c` | `fp_vdrv` | — | Flat panel VDRV on/off |
| `0x10030` | `fp_de` | — | Flat panel DE on/off |
| `0x10034` | `vt_hpixen` | — | Internal horiz pixel on/off |
| `0x10038` | `vt_vpixen` | — | Internal vert pixel on/off |
| `0x1003c` | `vt_hcmap` | — | Cmap write horiz |
| `0x10040` | `vt_vcmap` | — | Cmap write vert |
| `0x10044` | `did_start_xy` | — | EOL/F DID/XY reset val |
| | `DID_STARTX` | 11:0 | DID start X |
| | `DID_STARTY` | 23:12 | DID start Y |
| `0x10048` | `crs_start_xy` | — | EOL/F CRS/XY reset val |
| | `CRS_STARTX` | 11:0 | Cursor start X |
| | `CRS_STARTY` | 23:12 | Cursor start Y |
| `0x1004c` | `vc_start_xy` | — | EOL/F VC/XY reset val |
| | `VC_STARTX` | 11:0 | Video capture start X |
| | `VC_STARTY` | 23:12 | Video capture start Y |

### Overlay plane

| Offset | Field | Bits | Meaning |
|--------|-------|------|---------|
| `0x20000` | `ovr_width_tile` | — | Overlay plane ctrl 0 |
| | `OVR_FIFO_RESET` | 13 | Overlay FIFO reset |
| `0x20004` | `ovr_inhwctrl` | — | Overlay plane ctrl 1 |
| | `OVR_DMA_ENABLE` | 0 | Overlay DMA enable |
| `0x20008` | `ovr_control` | — | Overlay plane ctrl 1 |
| | `OVR_DMA_ENABLE` | 0 | Overlay DMA enable |

### Normal (framebuffer) plane

| Offset | Field | Bits | Meaning |
|--------|-------|------|---------|
| `0x30000` | `frm_size_tile` | — | Normal plane ctrl 0 |
| | `FRM_RHS` | 4:0 | Right-hand side tile |
| | `FRM_WIDTH_TILE` | 12:5 | Width in tiles |
| | `FRM_DEPTH` | 14:13 | Depth (`GBE_FRM_DEPTH_8/16/32`) |
| | `FRM_FIFO_RESET` | 15 | Framebuffer FIFO reset |
| `0x30004` | `frm_size_pixel` | — | Normal plane ctrl 1 |
| | `FB_HEIGHT_PIX` | 31:16 | Height in pixels |
| `0x30008` | `frm_inhwctrl` | — | Normal plane ctrl 2 |
| | `FRM_DMA_ENABLE` | 0 | Framebuffer DMA enable |
| `0x3000c` | `frm_control` | — | Normal plane ctrl 3 |
| | `FRM_DMA_ENABLE` | 0 | Framebuffer DMA enable |
| | `FRM_LINEAR` | 1 | Linear (non-tiled) mode |
| | `FRM_TILE_PTR` | 31:9 | Tile pointer |

### DID (display list) control

| Offset | Field | Bits | Meaning |
|--------|-------|------|---------|
| `0x40000` | `did_inhwctrl` | — | DID control |
| | `DID_DMA_ENABLE` | 0 | DID DMA enable |
| `0x40004` | `did_control` | — | DID shadow |
| | `DID_DMA_ENABLE` | 0 | DID DMA enable |

### WID (window ID) table

| Offset | Field | Bits | Meaning |
|--------|-------|------|---------|
| `0x50000` | `mode_regs[32]` | — | WID table (32 entries) |
| | `BUF` | 1:0 | Buffer select |
| | `TYP` | 4:2 | Type |
| | `CM` | 9:5 | Color mode |
| | `GAMMA` | 10 | Gamma map select |
| | `AUX` | 12:11 | Auxiliary |

### Color map / gamma map

| Offset | Field | Bits | Meaning |
|--------|-------|------|---------|
| `0x60000` | `cmap[6144]` | — | Color map (6144 entries) |
| `0x78000` | `cm_fifo` | — | Color map FIFO status |
| `0x80000` | `gmap[256]` | — | Gamma map (256 entries) |
| `0x90000` | `gmap10[1024]` | — | Gamma map, 10-bit (1024 entries) |

### Cursor

| Offset | Field | Bits | Meaning |
|--------|-------|------|---------|
| `0xa0000` | `crs_pos` | — | Cursor control 0 |
| `0xa0004` | `crs_ctl` | — | Cursor control 1 |
| `0xa0008` | `crs_cmap[3]` | — | Cursor color map |
| `0xa0014` | `crs_glyph[64]` | — | Cursor glyph (64 entries) |

### Video capture

| Offset | Field | Bits | Meaning |
|--------|-------|------|---------|
| `0xb0000` | `vc_0` … `vc_8` | — | Video capture control 0–8 |

### Color-mode constants

| Constant | Value | Meaning |
|----------|-------|---------|
| `GBE_CMODE_I8` | 0 | 8-bit indexed |
| `GBE_CMODE_I12` | 1 | 12-bit indexed |
| `GBE_CMODE_RG3B2` | 2 | 3-3-2 RGB |
| `GBE_CMODE_RGB4` | 3 | 4-4-4 RGB |
| `GBE_CMODE_ARGB5` | 4 | 1-5-5-5 ARGB |
| `GBE_CMODE_RGB8` | 5 | 8-8-8 RGB |
| `GBE_CMODE_RGBA5` | 6 | 5-5-5-1 RGBA |
| `GBE_CMODE_RGB10` | 7 | 10-10-10 RGB |

### Driver init sequence (`drivers/video/fbdev/gbefb.c`)

`gbe_turn_on()` / `gbe_reset()` / `compute_gbe_timing()` perform, in order:

1. `gbe_revision = gbe->ctrlstat & 15` — read chip revision
2. Build tile list: `gbe_tiles.cpu[i] = (gbe_mem_phys >> TILE_SHIFT) + i`
3. Init WID table: `gbe->mode_regs[i] = val` for `i` in 0..31
4. Disable interrupts: `gbe->vt_intr01 = 0xffffffff; gbe->vt_intr23 = 0xffffffff`
5. `gbe->did_control = 0; gbe->ovr_width_tile = 0; gbe->crs_ctl = 0`
6. Init gamma map: `gbe->gmap[i] = (i << 24) | (i << 16) | (i << 8)`
7. Init color map: `gbe_cmap[i] = (i << 8) | (i << 16) | (i << 24)`; `gbe_loadcmap()`
8. Set `vt_xymax`, `vt_vsync`/`vt_hsync`/`vt_vblank`/`vt_hblank`
9. `vc_start_xy` with `VC_STARTY` and `VC_STARTX = hblank_end - 4`
10. `vt_hpixen`/`vt_vpixen` setup
11. `did_start_xy` with `DID_STARTY` and `DID_STARTX = hblank_end - 20`
12. `crs_start_xy` with `CRS_STARTY = temp + 1` and `CRS_STARTX = hblank_end - GBE_CRS_MAGIC`
13. `fp_de`/`fp_hdrv`/`fp_vdrv` setup
14. `frm_size_pixel` with `FB_HEIGHT_PIX`
15. `frm_size_tile` with `FRM_WIDTH_TILE`, `FRM_RHS`, `FRM_DEPTH`
16. `ctrlstat`, `dotclock`, `sysclk`, `i2c`, `i2cfp`, `id`, `config`, `bist`

### Timing info (`struct gbe_timing_info`)

| Field | Meaning |
|-------|---------|
| `flags` | Mode flags (see below) |
| `width` / `height` | Visible resolution |
| `fields_sec` | Field rate |
| `cfreq` | Pixel clock |
| `htotal` / `vtotal` | Total lines |
| `hblank_start/end`, `hsync_start/end` | Horizontal blank/sync |
| `vblank_start/end`, `vsync_start/end` | Vertical blank/sync |
| `pll_m`, `pll_n`, `pll_p` | PLL values |

Timing flags: `GBE_VOF_UNKNOWNMON 1`, `GBE_VOF_STEREO 2`,
`GBE_VOF_DO_GENSYNC 4`, `GBE_VOF_SYNC_ON_GREEN 8`, `GBE_VOF_FLATPANEL 0x1000`,
`GBE_VOF_MAGICKEY 0x2000`.

## PROM video timing tables (IRIX `stand/arcs/include/crm_timing.h`)

The PROM's `crimeVTimings[]` table (`struct crime_timing_info`) defines every
video mode the firmware can set. Fields per entry: flags, width, height,
fields/sec (mHz), pixel clock (kHz), htotal, hblank_start/end, hsync_start/end,
vtotal, vblank_start/end, vsync_start/end, pll_m, pll_n, pll_p.

| Mode | Width×Height | Refresh (mHz) | Clock (kHz) | htotal | hblank | hsync | vtotal | vblank | vsync | PLL m/n/p |
|------|--------------|---------------|-------------|--------|--------|-------|--------|--------|-------|-----------|
| `VT_640_480_60` | 640×480 | 59940 | 25175 | 800 | 640–800 | 656–752 | 525 | 480–525 | 490–492 | 5/1/2 |
| `VT_800_600_60` | 800×600 | 60317 | 40000 | 1056 | 800–1056 | 840–968 | 628 | 600–628 | 601–605 | 8/1/2 |
| `VT_800_600_72` | 800×600 | 72188 | 50000 | 1040 | 800–1040 | 856–976 | 666 | 600–666 | 637–643 | 5/1/1 |
| `VT_1024_768_60` | 1024×768 | 60004 | 63546 | 1344 | 1024–1344 | 1048–1184 | 806 | 768–806 | 771–777 | 13/2/1 |
| `VT_1024_768_70` | 1024×768 | 70069 | 75000 | 1328 | 1024–1328 | 1048–1184 | 806 | 768–806 | 771–777 | 15/2/1 |
| `VT_1024_768_75` | 1024×768 | 75029 | 78750 | 1312 | 1024–1312 | 1040–1136 | 800 | 768–800 | 769–772 | 39/5/1 |
| `VT_1280_1024_48` | 1280×1024 | 48000 | 85882 | 1680 | 1280–1680 | 1360–1480 | 1065 | 1024–1065 | 1028–1031 | 30/7/0 |
| `VT_1280_1024_50` | 1280×1024 | 50000 | 89544 | 1680 | 1280–1680 | 1360–1480 | 1065 | 1024–1065 | 1027–1030 | 9/1/1 |
| `VT_1280_1024_60` | 1280×1024 | 59940 | 107245 | 1680 | 1280–1680 | 1310–1430 | 1065 | 1024–1065 | 1027–1030 | 16/3/0 |
| `VT_1280_1024_72` | 1280×1024 | 72000 | 128701 | 1690 | 1280–1690 | 1310–1450 | 1064 | 1024–1064 | 1027–1030 | 13/2/0 |
| `VT_1280_1024_75` | 1280×1024 | 75025 | 135000 | 1688 | 1280–1688 | 1296–1440 | 1066 | 1024–1066 | 1025–1028 | 27/4/0 |
| `VT_1280_492_120` | 1280×492 | 119880 | 107072 | 1680 | 1280–1680 | 1310–1430 | 532 | 492–532 | 495–498 | 16/3/0 |
| `VT_1600_1024_50` | 1600×1024 | 50000 | 101270 | 1900 | 1600–1900 | 1630–1730 | 1066 | 1024–1066 | 1027–1030 | 5/1/0 |

Notes:
- `VT_1280_492_120` is the field-sequential stereo mode
  (`CRIME_VOF_FS_STEREO` flag) for shutter glasses.
- `VT_1600_1024_50` is the 1600SW flat-panel mode.
- The GBE video timing counters are two 12-bit counters (`vt_x`, `vt_y`);
  `vt_x` counts 0→`vt_maxx` per line (GBE spec §video timing).
- The dot clock PLL is programmed via the `dotclock` register with the
  m/n/p values above.

## TODO / Open questions

- [x] Full GBE (Display Engine) register map — `include/video/gbe.h`
- [x] Full ICE register map — VICE Design Spec 099-0123-003 (see
  [register-maps.md](register-maps.md) "VICE / ICE")
- [x] PROM-visible MRE/RE/DE/MTE register map — `definitions.h`
- [x] Video modes / timing details — PROM `crm_timing.h` table above