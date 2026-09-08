# O2 (IP32) Register Maps — from Linux/NetBSD drivers and IRIX source

Register-level details mined from open-source drivers and the leaked IRIX 6.5.17
source tree. These are the ground truth for emulator implementation.

## Memory Map (IRIX `irix/kern/sys/mace.h`, `irix/kern/sys/crime.h`)

| Base | Device |
|------|--------|
| `0x14000000` | CRIME CPU Interface (`CRM_BASEADDR`) |
| `0x15000000` | CRIME Render Engine (`CRM_RE_BASE_ADDRESS`) |
| `0x16000000` | GBE Display Engine (`GBECHIP_ADDR`) |
| `0x1f000000` | MACE (`MACE_BASE`) |
| `0x1fc00000` | System ROM / PROM |

### MACE sub-offsets (relative to `0x1f000000`, IRIX `mace.h`)

| Offset | Device |
|--------|--------|
| `0x080000` | PCI (`MACE_PCI`) |
| `0x100000` | Video In 1 (`MACE_VIN1`) |
| `0x180000` | Video In 2 (`MACE_VIN2`) |
| `0x200000` | Video Out (`MACE_VOUT`) |
| `0x280000` | Ethernet (`MACE_ENET`) |
| `0x300000` | Peripheral (`MACE_PERIF`) |
| `0x380000` | ISA External (`MACE_ISA_EXT`) |

### Peripheral sub-offsets (relative to `MACE_PERIF`)

| Offset | Device |
|--------|--------|
| `0x00000` | Audio (`MACE_AUDIO`) |
| `0x10000` | ISA (`MACE_ISA`) |
| `0x20000` | Keyboard/Mouse (`MACE_KBDMS`) |
| `0x30000` | I2C (`MACE_I2C`) |
| `0x40000` | UST/MSC (`MACE_UST_MSC`) |

### ISA External sub-offsets (relative to `MACE_ISA_EXT`)

| Offset | Device |
|--------|--------|
| `0x00000` | EPP (`ISA_EPP_BASE`) |
| `0x08000` | ECP (`ISA_ECP_BASE`) |
| `0x10000` | Serial 1 (`ISA_SER1_BASE`) |
| `0x18000` | Serial 2 (`ISA_SER2_BASE`) |
| `0x20000` | RTC (`ISA_RTC_BASE`) |
| `0x30000` | Game port (`ISA_GAME_BASE`) |

## CRIME ASIC (IRIX `irix/kern/sys/crime.h`, `stand/arcs/IP32prom/include/sys/crimereg.h`, `crimedef.h`, `crimechip.h`)

CRIME is the memory controller / system controller ASIC. It contains:
- CPU Interface (memory controller, interrupt controller, timers)
- Render Engine (RE) — 3D pipeline, TLB, pixel pipe, MTE
- GBE Display Engine interface

### CRIME CPU Interface registers (base `0x14000000`, IRIX `crime.h`)

| Register | Offset | Mask | Description |
|----------|--------|------|-------------|
| `CRM_ID` | `0x00` | `0xff` | Chip ID/revision |
| `CRM_CONTROL` | `0x08` | `0x3fff` | Control register |
| `CRM_INTSTAT` | `0x10` | `0xffffffff` | Interrupt status |
| `CRM_INTMASK` | `0x18` | `0xffffffff` | Interrupt mask |
| `CRM_SOFTINT` | `0x20` | `0xffffffff` | Software interrupt |
| `CRM_HARDINT` | `0x28` | `0xf0ffffff` | Hardware interrupt |
| `CRM_DOG` | `0x30` | `0x1fffff` | Watchdog timer |
| `CRM_TIME` | `0x38` | `0xffffffffffff` | Crime timer (66.6 MHz) |
| `CRM_CPU_ERROR_ADDR` | `0x40` | `0x3ffffffff` | CPU error address |
| `CRM_CPU_ERROR_STAT` | `0x48` | `0x7` | CPU error status |
| `CRM_CPU_ERROR_ENA` | `0x50` | `0x7` | CPU error enable |
| `CRM_VICE_ERROR_ADDR` | `0x58` | `0x3fffffff` | VICE (ICE) error address |
| `CRM_MEM_CONTROL` | `0x200` | `0x3` | Memory control (ECC) |
| `CRM_MEM_BANK_CTRL(x)` | `0x208 + x*8` | `0x11f` | Bank control (8 banks) |
| `CRM_MEM_REFRESH_CNTR` | `0x248` | `0x7ff` | Refresh counter |
| `CRM_MEM_ERROR_STAT` | `0x250` | `0x0ff7ffff` | Memory error status |
| `CRM_MEM_ERROR_ADDR` | `0x258` | `0x3fffffff` | Memory error address |
| `CRM_MEM_ERROR_ECC_SYN` | `0x260` | `0xffffffff` | ECC syndrome |
| `CRM_MEM_ERROR_ECC_CHK` | `0x268` | `0xffffffff` | ECC check bits |
| `CRM_MEM_ERROR_ECC_REPL` | `0x270` | `0xffffffff` | ECC replacement bits |

### CRIME Control register bits (IRIX `crime.h`)

```text
CRM_CONTROL_TRITON_SYSADC       0x2000
CRM_CONTROL_CRIME_SYSADC        0x1000
CRM_CONTROL_HARD_RESET          0x0800
CRM_CONTROL_SOFT_RESET          0x0400
CRM_CONTROL_DOG_ENA             0x0200
CRM_CONTROL_ENDIANESS           0x0100
CRM_CONTROL_ENDIAN_BIG          0x0100
CRM_CONTROL_ENDIAN_LITTLE       0x0000
CRM_CONTROL_CQUEUE_HWM          0x000f   (command queue high-water mark)
CRM_CONTROL_CQUEUE_SHFT         0
CRM_CONTROL_WBUF_HWM            0x00f0   (write buffer high-water mark)
CRM_CONTROL_WBUF_SHFT           8
```

### CRIME interrupt bits (IRIX `crime.h`)

```text
CRM_INT_VICE        0x80000000  (VICE/ICE interrupt)
CRM_INT_SOFT2       0x40000000
CRM_INT_SOFT1       0x20000000
CRM_INT_SOFT0       0x10000000
CRM_INT_RE5         0x08000000
CRM_INT_RE4         0x04000000
CRM_INT_RE3         0x02000000
CRM_INT_RE2         0x01000000
CRM_INT_RE1         0x00800000
CRM_INT_RE0         0x00400000
CRM_INT_MEMERR      0x00200000
CRM_INT_CRMERR      0x00100000
CRM_INT_GBE3        0x00080000
CRM_INT_GBE2        0x00040000
CRM_INT_GBE1        0x00020000
CRM_INT_GBE0        0x00010000
CRM_INT_GBEx        (CRM_INT_GBE0|CRM_INT_GBE1|CRM_INT_GBE2|CRM_INT_GBE3)
CRM_INT_MACE(i)     (1 << i)   (i = 0..15)
```

### CRIME CPU error status bits (IRIX `crime.h`)

```text
CRM_CPU_ERROR_CPU_ILL_ADDR      0x4
CRM_CPU_ERROR_VICE_WRT_PRTY     0x2
CRM_CPU_ERROR_CPU_WRT_PRTY      0x1
```

### CRIME Memory error status bits (IRIX `crime.h`)

```text
CRM_MEM_ERROR_MACE_ID           0x0000007f
CRM_MEM_ERROR_MACE_ACCESS       0x00000080
CRM_MEM_ERROR_RE_ID             0x00007f00
CRM_MEM_ERROR_RE_ACCESS         0x00008000
CRM_MEM_ERROR_GBE_ACCESS        0x00010000
CRM_MEM_ERROR_VICE_ACCESS       0x00020000
CRM_MEM_ERROR_CPU_ACCESS        0x00040000
CRM_MEM_ERROR_SOFT_ERR          0x00100000
CRM_MEM_ERROR_HARD_ERR          0x00200000
CRM_MEM_ERROR_MULTIPLE          0x00400000
CRM_MEM_ERROR_MEM_ECC_RD        0x00800000
CRM_MEM_ERROR_MEM_ECC_RMW       0x01000000
CRM_MEM_ERROR_INV_MEM_ADDR_RD   0x02000000
CRM_MEM_ERROR_INV_MEM_ADDR_WR   0x04000000
CRM_MEM_ERROR_INV_MEM_ADDR_RMW  0x08000000
```

## CRIME Render Engine (RE) registers (base `0x15000000`, IRIX `crimereg.h`, `crimedef.h`, `crimechip.h`)

The RE is organized in 4KB pages:

| Page | Base Offset | Contents |
|------|-------------|----------|
| 0 | `0x0000` | Interface Buffer (`CrmIntfBufReg`) |
| 1 | `0x1000` | TLB (`CrmTlbReg`) |
| 2 | `0x2000` | Pixel Pipe / Draw registers |
| 3 | `0x3000` | MTE (Memory Transfer Engine) |
| 4 | `0x4000` | Status / SetStartPtr |

### Interface Buffer (page 0, `CRM_INTFBUF_BASE = 0x0`)

| Register | Offset | Description |
|----------|--------|-------------|
| `data[64]` | `0x000` | FIFO data (2 words each) |
| `addr[64]` | `0x200` | FIFO address |
| `ctl` | `0x400` | Control (full/empty/stall levels) |
| `reset` | `0x408` | Reset |

### TLB (page 1, `CRM_TLB_BASE = 0x1000`)

| TLB | Offset | Entries |
|-----|--------|---------|
| FB A | `0x000` | 64 |
| FB B | `0x200` | 64 |
| FB C | `0x400` | 64 |
| Texture | `0x600` | 28 |
| CID | `0x6e0` | 4 |
| Linear A | `0x700` | 16 |
| Linear B | `0x780` | 16 |

Each TLB entry is 8 bytes (2×32-bit).

### Pixel Pipe / Draw registers (page 2, `CRM_PIXPIPE_BASE = 0x2000`)

| Register | Offset | Description |
|----------|--------|-------------|
| `BufMode` (src) | `0x000` | Source buffer mode |
| `BufMode` (dst) | `0x008` | Destination buffer mode |
| `ClipMode` | `0x010` | Clip mode |
| `DrawMode` | `0x018` | Draw mode |
| `ScrMask[5]` | `0x020`–`0x040` | Screen masks |
| `Scissor` | `0x048` | Scissor rectangle |
| `WinOffset` (src) | `0x050` | Window offset source |
| `WinOffset` (dst) | `0x058` | Window offset dest |
| `Primitive` | `0x060` | Primitive type/width |
| `Vertex X[3]` | `0x070`–`0x078` | Vertex X coordinates |
| `Vertex GL[3]` | `0x080`–`0x094` | Vertex GL (x,y) |
| `StartSetup` | `0x098` | Start setup |
| `PixelXfer` (src) | `0x0a0` | Pixel transfer source |
| `PixelXfer` (dst) | `0x0b0` | Pixel transfer dest |
| `Stipple` | `0x0c0` | Stipple mode/pattern |
| `Shade` | `0x0d0` | Shade registers (12) |
| `Texture` | `0x110` | Texture registers (23) |
| `Fog` | `0x170` | Fog registers |
| `Antialias` | `0x190` | Antialias line/coverage |
| `AlphaTest` | `0x198` | Alpha test |
| `Blend` | `0x1a0` | Blend constant/function |
| `LogicOp` | `0x1b0` | Logic operation |
| `ColorMask` | `0x1b8` | Color mask |
| `Depth` | `0x1c0` | Depth func/z0/dzdx/dzdy |
| `Stencil` | `0x1e0` | Stencil mode/mask |
| `PixPipeNull` | `0x1f0` | Null register |
| `PixPipeFlush` | `0x1f8` | Flush pipeline |

### MTE (page 3, `CRM_MTE_BASE = 0x3000`)

| Register | Offset | Description |
|----------|--------|-------------|
| `MTE_MODE` | `0x00` | Mode (clear/copy, stipple, depth) |
| `MTE_BYTEMASK` | `0x08` | Byte mask |
| `MTE_STIPPLEMASK` | `0x10` | Stipple mask |
| `MTE_FGVALUE` | `0x18` | Foreground value |
| `MTE_SRC0` | `0x20` | Source 0 |
| `MTE_SRC1` | `0x28` | Source 1 |
| `MTE_DST0` | `0x30` | Destination 0 |
| `MTE_DST1` | `0x38` | Destination 1 |
| `MTE_SRCYSTEP` | `0x40` | Source Y step |
| `MTE_DSTYSTEP` | `0x48` | Destination Y step |
| `MTE_NULL` | `0x70` | Null |
| `MTE_FLUSH` | `0x78` | Flush |

### Status (page 4, `CRM_STATUS_BASE = 0x4000`)

| Register | Offset | Description |
|----------|--------|-------------|
| `Status` | `0x00` | Status register |
| `SetStartPtr` | `0x08` | Set start pointer |

### Buffer mode bitfields (IRIX `crimedef.h`)

```text
BM_DOUBLE_PIX_SEL     BIT(0)
BM_DOUBLE_PIX         BIT(1)
BM_PIX_DEPTH_SHIFT    2   (mask 3<<2)
BM_PIX_TYPE_SHIFT     4   (mask 3<<4)
BM_BUF_DEPTH_SHIFT    8   (mask 3<<8)
BM_BUF_TYPE_SHIFT     10  (mask 7<<10)
```

Buffer types: `FB_A=0`, `FB_B=1`, `FB_C=2`, `TEX=3`, `LINEAR_A=4`, `LINEAR_B=5`, `CID=6`

### Draw mode bitfields (IRIX `crimedef.h`)

```text
DM_ENNOCONFLICT     BIT(23)
DM_ENGL             BIT(22)
DM_ENPIXXFER        BIT(21)
DM_ENSCISSORTEST    BIT(20)
DM_ENLINESTIPPLE    BIT(19)
DM_ENPOLYSTIPPLE    BIT(18)
DM_ENOPAQSTIPPLE    BIT(17)
DM_ENSMOOTHSHADE    BIT(16)
DM_ENTEXTURE        BIT(15)
DM_ENFOG            BIT(14)
DM_ENCOVERAGE       BIT(13)
DM_ENANTILINE       BIT(12)
DM_ENALPHATEST      BIT(11)
DM_ENBLEND          BIT(10)
DM_ENLOGICOP        BIT(9)
DM_ENDITHER         BIT(8)
DM_ENCOLORMASK      BIT(7)
DM_ENCOLORBYTEMASK  BIT(3..6)
DM_ENDEPTHTEST      BIT(2)
DM_ENDEPTHMASK      BIT(1)
DM_ENSTENCILTEST    BIT(0)
```

## GBE (Display Engine) register map (IRIX `stand/arcs/IP32prom/include/sys/crime_gbe.h`, `gbedefs.h`)

GBE base: `0x16000000` (`GBECHIP_ADDR`). Register block is ~512KB.

### Control / clock / ID block (offset `0x00000`)

| Register | Offset | Description |
|----------|--------|-------------|
| `ctrlstat` | `0x00000` | Control/status, CHIPID 3:0 |
| `dotclock` | `0x00004` | Dot clock PLL (M 7:0, N 13:8, P 15:14, RUN 20) |
| `i2c` | `0x00008` | I2C interface |
| `sysclk` | `0x0000c` | System clock PLL |
| `i2cfp` | `0x00010` | I2C flat panel |
| `id` | `0x00014` | Device ID/revision |

### Video timing block (offset `0x10000`)

| Register | Offset | Description |
|----------|--------|-------------|
| `vt_xy` | `0x10000` | Current dot coords (X 11:0, Y 23:12, FREEZE 31) |
| `vt_xymax` | `0x10004` | Max dot coords (MAXX 11:0, MAXY 23:12) |
| `vt_vsync` | `0x10008` | VSync on/off |
| `vt_hsync` | `0x1000c` | HSync on/off |
| `vt_vblank` | `0x10010` | VBlank on/off |
| `vt_hblank` | `0x10014` | HBlank on/off |
| `vt_flags` | `0x10018` | Polarity flags |
| `vt_f2rf_lock` | `0x1001c` | Frame-to-raster lock |
| `vt_intr01` | `0x10020` | Interrupt 0/1 Y coords |
| `vt_intr23` | `0x10024` | Interrupt 2/3 Y coords |
| `fp_hdrv` | `0x10028` | Flat panel HDRV |
| `fp_vdrv` | `0x1002c` | Flat panel VDRV |
| `fp_de` | `0x10030` | Flat panel DE |
| `vt_hpixen` | `0x10034` | Horiz pixel enable |
| `vt_vpixen` | `0x10038` | Vert pixel enable |
| `vt_hcmap` | `0x1003c` | Horiz cmap write enable |
| `vt_vcmap` | `0x10040` | Vert cmap write enable |
| `did_start_xy` | `0x10044` | DID start X/Y |
| `crs_start_xy` | `0x10048` | Cursor start X/Y |
| `vc_start_xy` | `0x1004c` | Video capture start X/Y |

### Overlay plane (offset `0x20000`)

| Register | Offset | Description |
|----------|--------|-------------|
| `ovr_width_tile` | `0x20000` | Overlay width in tiles |
| `ovr_control` | `0x20004` | Tile list ptr + DMA enable |

### Framebuffer plane (offset `0x30000`)

| Register | Offset | Description |
|----------|--------|-------------|
| `frm_size_tile` | `0x30000` | FRM_RHS 4:0, FRM_WIDTH_TILE 12:5, FRM_DEPTH 14:13 |
| `frm_size_pixel` | `0x30004` | FB_HEIGHT_PIX 31:16 |
| `frm_control` | `0x30008` | Tile list ptr + DMA enable |

### DID control (offset `0x40000`)

| Register | Offset | Description |
|----------|--------|-------------|
| `did_control` | `0x40000` | DID table ptr + DMA enable |

### WID table (offset `0x48000`)

| Register | Offset | Description |
|----------|--------|-------------|
| `mode_regs[32]` | `0x48000` | WID mode registers |

### Color / gamma map (offset `0x50000`)

| Register | Offset | Description |
|----------|--------|-------------|
| `cmap[4608]` | `0x50000` | Color palette |
| `cm_fifo` | `0x58000` | Cmap FIFO status |
| `gmap[256]` | `0x60000` | Gamma ramp |

### Cursor (offset `0x70000`)

| Register | Offset | Description |
|----------|--------|-------------|
| `crs_pos` | `0x70000` | Cursor position (X 11:0, Y 27:16) |
| `crs_ctl` | `0x70004` | Cursor enable, crosshair |
| `crs_cmap[3]` | `0x70008` | Cursor colors 1-3 |
| `crs_glyph[64]` | `0x78000` | Cursor glyph |

### Video capture (offset `0x80000`)

| Register | Offset | Description |
|----------|--------|-------------|
| `vc_lr` | `0x80000` | Capture window X coords |
| `vc_tb` | `0x80004` | Capture window Y coords |
| `vc_filters` | `0x80008` | Capture filters |
| `vc_control` | `0x8000c` | Capture control |

### GBE mode register bitfields (IRIX `gbedefs.h`)

```text
GBE_WID_BUF_MASK      0x3
GBE_WID_TYPE_MASK     0x1C  (shift 2)
GBE_WID_CM_MASK       0x3E0 (shift 5)
GBE_WID_GM_MASK       0x400 (shift 10)

GBE_CMODE_I8          0
GBE_CMODE_I12         1
GBE_CMODE_RG3B2       2
GBE_CMODE_RGB4        3
GBE_CMODE_RGB5        4
GBE_CMODE_RGB8        5
```

### Video timing tables (IRIX `crm_timing.h`)

Standard timings: 640×480@60, 800×600@60/72, 1024×768@60/70/75, 1280×1024@48/50/60/72/75, 1280×492@120 (stereo), 1600×1024@50.

## MACE ASIC (IRIX `irix/kern/sys/mace.h`)

MACE = Multimedia, Audio and Communications Engine. Base: `0x1f000000`.

### PCI host bridge (offset `MACE_PCI = 0x080000`)

| Register | Offset | Description |
|----------|--------|-------------|
| `PCI_ERROR_ADDR` | `0x00` | Error address |
| `PCI_ERROR_FLAGS` | `0x04` | Error flags |
| `PCI_CONTROL` | `0x08` | PCI control |
| `PCI_REV_INFO_R` | `0x0c` | Revision info (read) |
| `PCI_FLUSH_W` | `0x0c` | Flush (write) |
| `PCI_CONFIG_ADDR` | `0xcf8` | Config address |
| `PCI_CONFIG_DATA` | `0xcfc` | Config data |

### PCI Error Flags bits (IRIX `mace.h`)

```text
PERR_MASTER_ABORT           0x80000000
PERR_TARGET_ABORT           0x40000000
PERR_DATA_PARITY_ERR        0x20000000
PERR_RETRY_ERR              0x10000000
PERR_ILLEGAL_CMD            0x08000000
PERR_SYSTEM_ERR             0x04000000
PERR_INTERRUPT_TEST         0x02000000
PERR_PARITY_ERR             0x01000000
PERR_OVERRUN                0x00800000
PERR_MASTER_ABORT_ADDR_VALID 0x00080000
PERR_TARGET_ABORT_ADDR_VALID 0x00040000
PERR_DATA_PARITY_ADDR_VALID 0x00020000
PERR_RETRY_ADDR_VALID       0x00010000
```

Additional error bits from Linux `asm/ip32/mace.h`: `MEMORY_ADDR` bit 21,
`CONFIG_ADDR` bit 20 (address-type flags for the error address), `SIG_TABORT`
bit 4, `DEVSEL` timing bits 7:6 (fast/medium/slow), `FBB` bit 1, `66MHZ`
bit 0.

### PCI control register bits (Linux `asm/ip32/mace.h`)

| Bits | Field | Description |
|------|-------|-------------|
| 7:0 | `INT(x)` | Interrupt enables (per-slot) |
| 8 | `SERR_ENA` | System error enable |
| 9 | `ARB_N6` | Arbiter |
| 10 | `PARITY_ERR` | Parity error |
| 11 | `MRMRA_ENA` | Memory-read-multiple/read-ahead enable |
| 12–14 | `ARB_N3/N4/N5` | Arbiter |
| 15 | `PARK_LIU` | Park last-in-use |
| 23:16 | `INV_INT(x)` | Inverted interrupt bits |
| 24 | `OVERRUN_INT` | Overrun interrupt |
| 25 | `PARITY_INT` | Parity interrupt |
| 26 | `SERR_INT` | System error interrupt |
| 27 | `IT_INT` | Interrupt test |
| 28 | `RE_INT` | Retry error interrupt |
| 29 | `DPED_INT` | Data parity error interrupt |
| 30 | `TAR_INT` | Target abort interrupt |
| 31 | `MAR_INT` | Master abort interrupt |

Linux init writes `control = 0xff008500` after clearing errors.

### PCI configuration space access (Linux `arch/mips/pci/ops-mace.c`)

Config cycles go through `PCI_CONFIG_ADDR` (`0xcf8`) / `PCI_CONFIG_DATA`
(`0xcfc`):

- `config_addr = (bus << 16) | (devfn << 8) | (reg & 0xfc)`
- Byte/word accesses use big-endian lane steering:
  byte `b[(reg & 3) ^ 3]`, word `w[((reg >> 1) & 1) ^ 1]`
- Master-abort interrupts are masked during config reads; the master-abort
  error bit is cleared afterwards
- **Quirk:** reads of config register `0x40` (dword) from the two onboard
  SCSI devices (devfn 1<<3 and 2<<3) have bit 12 (`0x1000`, Ultra SCSI)
  forced on — the firmware never set the Ultra bit, so Linux fakes it

### PCI address windows (Linux `asm/ip32/mace.h`)

| Constant | Value | Purpose |
|----------|-------|---------|
| `MACEPCI_LOW_MEMORY` | `0x1a000000` | Low PCI memory window (32 MB, to `0x1bffffff`) |
| `MACEPCI_LOW_IO` | `0x18000000` | Low PCI I/O window |
| `MACEPCI_SWAPPED_VIEW` | `0x00000000` | Byte-swapped view offset |
| `MACEPCI_NATIVE_VIEW` | `0x40000000` | Native (unswapped) view offset |
| `MACEPCI_IO` | `0x80000000` | I/O space flag |
| `MACEPCI_HI_MEMORY` | `0x280000000` | High PCI memory window (64-bit kernels) |
| `MACEPCI_HI_IO` | `0x100000000` | High PCI I/O window |

On 32-bit kernels the PCI memory offset is `MACEPCI_LOW_MEMORY - 0x80000000`;
on 64-bit kernels it is `0x200000000`.

### Ethernet (offset `MACE_ENET = 0x280000`)

The MACE Ethernet block is a **MAC110** core. Register map from NetBSD
`sys/arch/sgimips/mace/if_mecreg.h` and Linux `drivers/net/ethernet/sgi/meth.h`.
All registers are 64-bit, accessed at 8-byte offsets from `0x1f280000`.

| Offset | Register | Description |
|--------|----------|-------------|
| `0x00` | `MEC_MAC_CONTROL` | MAC control |
| `0x08` | `MEC_INT_STATUS` | Interrupt status |
| `0x10` | `MEC_DMA_CONTROL` | DMA control |
| `0x18` | `MEC_TIMER` | Timer |
| `0x20` | `MEC_TX_ALIAS` | TX alias (int enable) |
| `0x28` | `MEC_RX_ALIAS` | RX alias (int enable/threshold) |
| `0x30` | `MEC_TX_RING_PTR` | TX ring read/write pointers |
| `0x38` | `MEC_TX_RING_PTR_ALIAS` | TX ring pointer alias |
| `0x40` | `MEC_RX_FIFO` | RX FIFO status |
| `0x48` | `MEC_RX_FIFO_ALIAS1` | RX FIFO alias 1 |
| `0x50` | `MEC_RX_FIFO_ALIAS2` | RX FIFO alias 2 |
| `0x58` | `MEC_TX_VECTOR` / `MEC_IRQ_VECTOR` | TX/IRQ vector |
| `0x60` | `MEC_PHY_DATA` | PHY data (MDIO) |
| `0x68` | `MEC_PHY_ADDRESS` | PHY address |
| `0x70` | `MEC_PHY_READ_INITIATE` | PHY read initiate |
| `0x78` | `MEC_PHY_BACKOFF` | PHY backoff |
| `0xa0` | `MEC_STATION` | Station address (48-bit MAC) |
| `0xa8` | `MEC_STATION_ALT` | Station address (alternate) |
| `0xb0` | `MEC_MULTICAST` | Multicast hash filter |
| `0xb8` | `MEC_TX_RING_BASE` | TX ring base address |
| `0xc0` | `MEC_TX_PKT1_CMD_1` | TX packet 1 command |
| `0xc8`–`0xd8` | `MEC_TX_PKT1_BUFFER_1..3` | TX packet 1 concat buffers |
| `0xe0` | `MEC_TX_PKT2_CMD_1` | TX packet 2 command |
| `0xe8`–`0xf8` | `MEC_TX_PKT2_BUFFER_1..3` | TX packet 2 concat buffers |
| `0x100` | `MEC_MCL_RX_FIFO` | MCL RX FIFO |

#### MAC_CONTROL bits

| Bits | Field | Description |
|------|-------|-------------|
| 0 | `CORE_RESET` | Global reset to MAC110 core |
| 1 | `FULL_DUPLEX` | 1 = full duplex |
| 2 | `INT_LOOPBACK` | 1 = loop internal MII bus |
| 3 | `SPEED_SELECT` | 0 = 10 Mbit, 1 = 100 Mbit |
| 4 | `MII_SELECT` | 0 = MII, 1 = SIA (collision report in loopback) |
| 6:5 | `FILTER` | 00 = station only, 01 = +matched multicast, 10 = +all multicast, 11 = promiscuous |
| 7 | `LINK_FAILURE` | 1 = PHY link-failure detection enabled |
| 14:8 | `IPGT` | Inter-packet gap (80 ns/unit @100M, 800 ns @10M) |
| 21:15 | `IPGR1` | IPGR1 |
| 28:22 | `IPGR2` | IPGR2 |
| 31:29 | `REVISION` | 000 = initial, 001 = improved TX concatenation |

Default IPG: `IPGT=21, IPGR1=17, IPGR2=11` (`MEC_MAC_IPG_DEFAULT`).

#### INT_STATUS bits (write-1-to-clear)

| Bit | Field | Description |
|-----|-------|-------------|
| 0 | `TX_EMPTY` | TX ring buffer empty |
| 1 | `TX_PACKET_SENT` | TX packet with INT request sent |
| 2 | `TX_LINK_FAIL` | PHY link failure |
| 3 | `TX_MEM_ERROR` | DMA memory error (fatal) |
| 4 | `TX_ABORT` | TX aborted (fatal) |
| 5 | `RX_THRESHOLD` | RX threshold condition |
| 6 | `RX_FIFO_UNDERFLOW` | RX FIFO empty, packet dropped |
| 7 | `RX_DMA_UNDERFLOW` | RX DMA FIFO overflow (fatal) |
| 12:8 | `RX_MCL_FIFO_ALIAS` | RX MCL FIFO read pointer alias |
| 24:16 | `TX_RING_BUFFER_ALIAS` | TX ring read pointer alias |
| 29:25 | `RX_SEQUENCE_NUMBER` | RX sequence number of queue head |
| 30 | `MCAST_HASH_OUTPUT` | Multicast hash select latch |

#### DMA_CONTROL bits

| Bits | Field | Description |
|------|-------|-------------|
| 0 | `TX_INT_ENABLE` | TX buffer-empty interrupt enable |
| 1 | `TX_DMA_ENABLE` | TX DMA enable |
| 3:2 | `TX_RING_SIZE` | TX ring size |
| 8:4 | `RX_INT_THRESHOLD` | RX FIFO depth threshold for interrupt |
| 9 | `RX_INT_ENABLE` | RX packet interrupt enable |
| 10 | `RX_RUNT` | Receive runt packets |
| 11 | `RX_PACKET_GATHER` | RX packet gather |
| 14:12 | `RX_DMA_OFFSET` | RX packet data start offset |
| 15 | `RX_DMA_ENABLE` | RX DMA enable |

#### TX packet format (128 bytes per ring entry)

Each TX ring entry is a 128-byte `tx_packet`: an 8-byte header followed by
either up to 3 concatenation buffer pointers or up to 120 bytes of inline data.

Header (`tx_packet_hdr`):

| Bits | Field | Description |
|------|-------|-------------|
| 63:28 | pad | zero |
| 27 | `cat_ptr3_valid` | Concat pointer 3 valid |
| 26 | `cat_ptr2_valid` | Concat pointer 2 valid |
| 25 | `cat_ptr1_valid` | Concat pointer 1 valid |
| 24 | `tx_int_flag` | Generate TX interrupt when sent |
| 23 | `term_dma_flag` | Terminate DMA on abort |
| 22:16 | `data_offset` | Start offset in ring data block |
| 15:0 | `data_len` | Data length in bytes − 1 |

Concat pointer (`tx_cat_ptr`): bits 63:48 pad, 47:32 length−1, 60:32 start
address (physical), 2:0 pad.

On DMA completion the header is overwritten with a TX status vector:
bit 63 = sent, bits 44:36 flags, 35:32 collision retry count, 15:0 length.
TX status flags: `SUCCESS` bit 23, `TOOLONG` 24, `UNDERRUN` 25, `EXCCOLL` 26,
`DEFER` 27, `LATECOLL` 28.

#### RX status vector (per received packet)

| Bits | Field | Description |
|------|-------|-------------|
| 63 | valid | Entry valid |
| 47:32 | `ip_chk_sum` | IP checksum |
| 31:27 | `seq_num` | Sequence number |
| 26 | `mac_addr_match` | Station address match |
| 25 | `mcast_addr_match` | Multicast filter match |
| 24 | `carrier_event_seen` | Carrier event |
| 23 | `bad_packet` | Bad packet |
| 22 | `long_event_seen` | Long event |
| 21 | `invalid_preamble` | Invalid preamble |
| 20 | `broadcast` | Broadcast packet |
| 19 | `multicast` | Multicast packet |
| 18 | `crc_error` | CRC error |
| 16 | `rx_code_violation` | RX code violation |
| 15:0 | `rx_len` | Received length |

RX ring: 16 entries, each a 4 KB buffer (`METH_RX_BUFF_SIZE`), 34-byte header
(status vector + 3 quad pad + 2-byte zero pad) before packet data.

#### PHY (MDIO) interface

- `MEC_PHY_DATA` (`0x60`): bit 16 = busy, bits 15:0 = data
- `MEC_PHY_ADDRESS` (`0x68`): bits 4:0 = register, bits 9:5 = device
- `MEC_PHY_READ_INITIATE` (`0x70`): write to start read
- Known PHYs: Quality QS6612X (`0x0181441`), ICS1889/ICS1890, National DP83840

The PROM exercises the MAC control register at offset `0x04` (low 32-bit lane),
receive FIFO at `0x104`, and byte registers for write pointer (`0x45`), read
pointer (`0x46`), and depth (`0x47`).

### Audio (offset `MACE_AUDIO = 0x300000`)

Register layout from Linux `arch/mips/include/asm/ip32/mace.h`
(`struct mace_audio`) and `sound/mips/sgio2audio.c`. All registers are 64-bit.

| Offset | Register | Description |
|--------|----------|-------------|
| `0x00` | `control` | Audio control |
| `0x08` | `codec_control` | Codec control/status |
| `0x10` | `codec_mask` | Codec status input mask |
| `0x18` | `codec_read` | Codec read data |
| `0x20` | `chan[0].control` | Channel 0 (ADC) control |
| `0x28` | `chan[0].read_ptr` | Channel 0 read pointer |
| `0x30` | `chan[0].write_ptr` | Channel 0 write pointer |
| `0x38` | `chan[0].depth` | Channel 0 ring depth |
| `0x40` | `chan[1].control` | Channel 1 (DAC1) control |
| `0x48` | `chan[1].read_ptr` | Channel 1 read pointer |
| `0x50` | `chan[1].write_ptr` | Channel 1 write pointer |
| `0x58` | `chan[1].depth` | Channel 1 ring depth |
| `0x60` | `chan[2].control` | Channel 2 (DAC2) control |
| `0x68` | `chan[2].read_ptr` | Channel 2 read pointer |
| `0x70` | `chan[2].write_ptr` | Channel 2 write pointer |
| `0x78` | `chan[2].depth` | Channel 2 ring depth |

Audio `control` bits: bit 0 = `RESET` (reset audio interface), bit 1 =
`CODEC_PRESENT` (codec detected).

`codec_control`: bits 15:0 = codec control word, bit 16 = `READ` (initiate
codec register read), bits 24:17 = codec register address. After writing a
read command, poll/wait ~200 µs then read the result from `codec_read`.

Channel `control` bits: bit 10 = `RESET` (reset channel), bit 9 = `DMA_ENABLE`,
bits 7:5 = interrupt threshold (0 = disabled, 1 = >25% full, 2 = >50%,
3 = >75%, 4 = empty, 5 = not empty, 6 = full, 7 = not full).

Each channel has a 4 KB ring buffer (`CHANNEL_RING_SIZE = 1 << 12`). Sample
packing: left channel at bit 40, right channel at bit 8 of each 64-bit ring
entry. DMA channel assignments: ADC = channel 0, DAC1 = channel 1,
DAC2 = channel 2 (IRIX `ad1843.h`).

Audio interrupts appear in the MACE ISA interrupt status/mask registers
(`MACEISA_AUDIO_SW_INT` bit 0, `MACEISA_AUDIO_SC_INT` bit 1,
`MACEISA_AUDIO1_DMAT_INT` bit 2, `MACEISA_AUDIO1_OF_INT` bit 3,
`MACEISA_AUDIO2_DMAT_INT` bit 4, `MACEISA_AUDIO2_MERR_INT` bit 5,
`MACEISA_AUDIO3_DMAT_INT` bit 6, `MACEISA_AUDIO3_MERR_INT` bit 7).

PROM uses channel 2: ring control `0x40`, read ptr `0x48`, write ptr `0x50`,
depth `0x58`. Ring control and write pointer are 64-bit.

#### AD1843 codec registers (IRIX `irix/kern/sys/ad1843.h`, AD1843 datasheet)

The MACE audio block talks to an Analog Devices AD1843 SoundComm codec over a
serial control interface. The AD1843 has 32 16-bit registers:

| Reg | Name | Function |
|-----|------|----------|
| 0 | `AD1843_STAT_REV` | Status/revision (bit 15 `INIT`, bit 14 `PDNO`, bits 3:0 rev) |
| 1 | `AD1843_CH_STAT` | Channel status (DAC1/2 underrun, ADC L/R overrange) |
| 2 | `AD1843_ADC_SRC_GATTN` | ADC source select + gain/attenuation (L/R) |
| 3 | `AD1843_DAC2_MIX` | DAC2 to main mix (mute + gain L/R) |
| 4 | `AD1843_AUX1_MIX` | AUX1 to mix (mute + gain L/R) |
| 5 | `AD1843_AUX2_MIX` | AUX2 to mix (mute + gain L/R) |
| 6 | `AD1843_AUX3_MIX` | AUX3 to DAC1 mix |
| 7 | `AD1843_MIC_MIX` | MIC to mix |
| 8 | `AD1843_MONO_MIX_MISC` | Mono in select + misc mixing |
| 9 | `AD1843_DAC1_GATTN` | DAC1 gain/attenuation |
| 10 | `AD1843_DAC2_GATTN` | DAC2 gain/attenuation |
| 11 | `AD1843_DAC1_DIGITAL_ATTEN` | DAC1 digital-only attenuation |
| 12 | `AD1843_DAC2_DIGITAL_ATTEN` | DAC2 digital-only attenuation |
| 13 | `AD1843_ADC_DAC1_MIX` | ADC→DAC1 digital mixing |
| 14 | `AD1843_ADC_DAC2_MIX` | ADC→DAC2 digital mixing |
| 15 | `AD1843_CLK_SRC_SELECT` | Rate clock source select |
| 16 | `AD1843_CLK_GEN1_MODE` | Clock gen 1 mode (video lock, PLL gain) |
| 17 | `AD1843_CLK_GEN1_RATE` | Clock gen 1 sample rate |
| 18 | `AD1843_CLK_GEN1_PHASE` | Clock gen 1 phase shift |
| 19 | `AD1843_CLK_GEN2_MODE` | Clock gen 2 mode |
| 20 | `AD1843_CLK_GEN2_RATE` | Clock gen 2 sample rate |
| 21 | `AD1843_CLK_GEN2_PHASE` | Clock gen 2 phase shift |
| 22 | `AD1843_CLK_GEN3_MODE` | Clock gen 3 mode |
| 23 | `AD1843_CLK_GEN3_RATE` | Clock gen 3 sample rate |
| 24 | `AD1843_CLK_GEN3_PHASE` | Clock gen 3 phase shift |
| 25 | `AD1843_FILTER_MODE` | Digital filter mode |
| 26 | `AD1843_SERIAL` | Serial interface + sample format |
| 27 | `AD1843_CH_POWERDOWN` | Channel power down |
| 28 | `AD1843_CONFIG` | Converter power down, clock out enables |
| 29–31 | reserved | — |

ADC source select (reg 2): line, mic, aux1, aux2, aux3, mono, DAC1, DAC2 per
channel; mic gain enable bits 12 (L) and 4 (R); 4-bit gain/attenuation fields.
Mixer registers use bit 15/7 = mute, 5-bit gain fields (`01000` = 0.0 dB).

DMA channel assignments (IRIX `ad1843.h`): ADC = channel 0, DAC1 = channel 1,
DAC2 = channel 2. PROM audio test (`hello_tune.c`) uses these with the MACE
audio ring registers above.

### I2C (offset `MACE_I2C = 0x330000`)

| Register | Offset | Description |
|----------|--------|-------------|
| `MACE_I2C_CONFIG` | `0x00` | Config |
| `MACE_I2C_STATUS` | `0x10` | Status |
| `MACE_I2C_DATA` | `0x18` | Data |

### PS/2 Keyboard/Mouse (offset `MACE_KBDMS = 0x320000`)

| Register | Offset | Description |
|----------|--------|-------------|
| `MACE_KEYBOARD_TX_BUF` | `0x00` | Keyboard TX |
| `MACE_KEYBOARD_RX_BUF` | `0x08` | Keyboard RX |
| `MACE_KEYBOARD_CONTROL` | `0x10` | Keyboard control |
| `MACE_KEYBOARD_STATUS` | `0x18` | Keyboard status |
| `MACE_MOUSE_TX_BUF` | `0x20` | Mouse TX |
| `MACE_MOUSE_RX_BUF` | `0x28` | Mouse RX |
| `MACE_MOUSE_CONTROL` | `0x30` | Mouse control |
| `MACE_MOUSE_STATUS` | `0x38` | Mouse status |

### ISA interface (offset `MACE_ISA = 0x310000`)

| Register | Offset | Description |
|----------|--------|-------------|
| `ISA_RINGBASE` | `0x00` | Ring base + reset |
| `ISA_FLASH_NIC_REG` | `0x08` | Flash/NIC/LED/DP-RAM |
| `ISA_INT_STS_REG` | `0x10` | Interrupt status |
| `ISA_INT_MSK_REG` | `0x18` | Interrupt mask |

ISA misc control bits: `FLASH_WE=0x01`, `PWD_CLEAR=0x02`, `NIC_DEASSERT=0x04`,
`NIC_DATA=0x08`, `LED_RED=0x10`, `LED_GREEN=0x20`, `DP_RAM_ENABLE=0x40`.

### UST/MSC (offset `MACE_UST_MSC = 0x340000`)

| Register | Offset | Description |
|----------|--------|-------------|
| `MACE_UST` | `0x00` | Universal System Time (64-bit) |
| `MACE_COMPARE1` | `0x08` | Compare 1 |
| `MACE_COMPARE2` | `0x10` | Compare 2 |
| `MACE_COMPARE3` | `0x18` | Compare 3 |
| `MACE_AIN_MSC_UST` | `0x20` | Audio in MSC/UST |
| `MACE_AOUT1_MSC_UST` | `0x28` | Audio out 1 MSC/UST |
| `MACE_AOUT2_MSC_UST` | `0x30` | Audio out 2 MSC/UST |
| `MACE_VIN1_MSC_UST` | `0x38` | Video in 1 MSC/UST |
| `MACE_VIN2_MSC_UST` | `0x40` | Video in 2 MSC/UST |
| `MACE_VOUT_MSC_UST` | `0x48` | Video out MSC/UST |

UST period: 960 ns.

### MACE interrupt assignments (IRIX `mace.h`)

```text
MACE_VID_IN_1         0
MACE_VID_IN_2         1
MACE_VID_OUT          2
MACE_ETHERNET         3
MACE_PERIPH_SERIAL    4
MACE_PERIPH_PARALLEL  4
MACE_PERIPH_MISC      5
MACE_PERIPH_AUDIO     6
MACE_PCI_BRIDGE       7
MACE_PCI_SCSI0        8
MACE_PCI_SCSI1        9
MACE_PCI_SLOT0        10
MACE_PCI_SLOT1        11
MACE_PCI_SLOT2        12
MACE_PCI_SHARED0      13
MACE_PCI_SHARED1      14
MACE_PCI_SHARED2      15
```

## VICE / ICE (Image Compression Engine)

The VICE (Video Image Compression Engine) is the ICE ASIC. Source: **VICE
Design Specification 099-0123-003 v1.0** (`docs/manuals-specs/o2-VICE-spec.md`).
It is referenced in IRIX as:
- Interrupt: `VICE_CPU_INTR = 31` (IRIX `IP32.h`)
- Error address: `CRM_VICE_ERROR_ADDR = 0x14000058` (IRIX `crime.h`)
- Error status bits in `CRM_CPU_ERROR_STAT` and `CRM_MEM_ERROR_STAT`

VICE contains a Media Signal Processor (MSP), a Bitstream Processor (BSP), a
DMA unit with its own TLB, and a host interface. All registers sit on
double-word (8-byte) boundaries regardless of width.

### VICE system address map (VICE_ID pins = 00)

The VICE chip decodes SysAD address bits (21:20) against its VICE_ID pins.
With VICE_ID = 00 the chip occupies `0x17000000`–`0x170FFFFC`:

| System address | VICE offset | Size | Function |
|----------------|-------------|------|----------|
| `0x17000000` | `0x0000` | 8 B | "Safe" register address 0 (watchpoint) |
| `0x17000008` | `0x0008` | 4 K | Chip registers (mode, status, control, interrupts) |
| `0x17001000` | `0x1000` | 4 K | Chip DMA descriptor set registers (COP3) |
| `0x17002000` | `0x2000` | 4 K | MSP instruction RAM |
| `0x17003000` | `0x3000` | 4 K | MSP instruction RAM (reserved, not implemented) |
| `0x17004000` | `0x4000` | 4 K | BSP instruction RAM (1K x 16) |
| `0x17005000` | `0x5000` | 8 K | BSP table memory (1280K x 22) |
| `0x17007000` | `0x7000` | 4 K | BSP input/output buffers (IN `0x7800`, OUT `0x7000`) |
| `0x17008000` | `0x8000` | 2 K | Data RAM bank A |
| `0x17008800` | `0x8800` | 2 K | Data RAM bank B |
| `0x17009000` | `0x9000` | 2 K | Data RAM bank C |
| `0x17009800` | `0x9800` | 10 K | Data RAM (unused) |
| `0x1700C000` | `0xC000` | 8 K | Unused |
| `0x1700E000` | `0xE000` | 4 K | Kernel restricted registers (protected) |
| `0x1700F000` | `0xF000` | 4 K | VICE TLB (64-entry map) |

VICE-accessible system memory (via the VICE TLB, 64K pages):
- `0x00800000`–`0x00BFFFFC` — 64K linear pages (4 MB)
- `0x10800000`–`0x10BFFFFC` — 64K frame-buffer tiles (4 MB)

### VICE chip registers (offset from `0x17000000`)

| Offset | MSP access | Register | Function | r/w | Reset | Bits |
|--------|-----------|----------|----------|-----|-------|------|
| `0x0008` | — | `VICE_ID` | Chip rev/ID | r | `0xE1` | 8 |
| `0x0020`/`0xE000` | — | `VICE_CFG` | General config | r/w | `0x00` | 16 |
| `0xE008` | — | `VICE_INT_RESET` | Interrupt reset (w1c) | w | `0x00` | 9 |
| `0xE010` | — | `VICE_INT_EN` | Interrupt enable | r/w | `0x00` | 9 |
| `0x0028` | — | `HST_BSP_IN_BOX` | Host copy of BSP/MSP in mailbox | r | `0x00` | 16 |
| `0x0030` | — | `HST_BSP_OUT_BOX` | Host copy of BSP/MSP out mailbox | r | `0x00` | 16 |
| `0x0040` | — | `MSP_CTL_STAT` | MSP control/status | r/w | `0x00` | 32 |
| `0x0048` | — | `MSP_ExcpFlag` | MSP exception flag | r/w | `0x00` | 32 |
| `0x0050` | — | `MSP_PC` | MSP program counter | r/w | — | 32 |
| `0x0058` | — | `MSP_BadAddr` | MSP bad address | r | — | 32 |
| `0x0060` | — | `MSP_WatchPoint` | MSP watchpoint | r/w | `0x00` | 32 |
| `0x0068` | — | `MSP_EPC` | MSP exception PC | r | — | 32 |
| `0x0070` | — | `MSP_CAUSE` | MSP exception cause | r | — | 32 |
| `0x0078` | — | `BSP_RPAGE` | BSP R page | r/w | — | 16 |
| `0x0080` | — | `BSP_SW_INT` | BSP software interrupt | w | — | 0 |
| `0x0100` | CTC1 `$0` | `MSP_D_RAM` | MSP data RAM arbitration | r/w | `0x00` | 32 |
| `0x0108` | CFC1 `$1` | `VICEMSP_COUNT` | MSP free-running counter | r | — | 32 |
| `0x0110` | CTC1 `$2` | `BSP_CTL_STAT` | BSP control/status | r/w | `0x00` | 16 |
| `0x0118` | CTC1 `$3` | `BSP_WatchPoint` | BSP watchpoint | r/w | `0x00` | 16 |
| `0x0120` | CFC1 `$4` | `BSP_IN_COUNT` | BSP decoded bits counter | r | — | 24 |
| `0x0128` | CFC1 `$5` | `BSP_OUT_COUNT` | BSP encoded bits counter | r | — | 24 |
| `0x0140` | CTC1 `$8` | `BSP_PC` | BSP program counter | r/w | `0x00` | 16 |
| `0x0148` | CTC1 `$9` | `BSP_EPC` | BSP exception PC | r | `0x00` | 16 |
| `0x0150` | CTC1 `$10` | `BSP_HALT_RESET` | BSP halt/reset control | r | `0x00` | 2 |
| `0x0158` | CTC1 `$11` | `BSP_CAUSE` | BSP exception cause | r | `0x00` | 16 |
| `0x0160` | CTC1 `$12` | `VICE_INT` | Interrupt status | r | `0x00` | 9 |
| `0x0168` | CTC1 `$13` | `BSP_FIFO_CTL_STAT` | BSP FIFO control/status | r/w | `0x05` | 6 |
| `0x0170` | CTC1 `$14` | `BSP_AVALID_BITS` | BSP A FIFO valid bits (decode) | r/w | `0x00` | — |
| `0x0178` | CTC1 `$15` | `BSP_FVALID_BITS` | BSP F FIFO valid bits (encode) | r/w | `0x00` | — |
| `0x0180` | CTC1 `$16` | `DMA_CTL_CH1` | DMA ch1 control | r/w | `0x10` | 16 |
| `0x0188` | CFC1 `$17` | `DMA_STAT_CH1` | DMA ch1 status | r | `0x10` | 16 |
| `0x0190` | CTC1 `$18` | `DMA_DATA_CH1` | DMA ch1 data fill | r/w | `0x00` | 16 |
| `0x0198` | CFC1 `$19` | `DMA_MEM_PT_CH1` | DMA ch1 system pointer | r | `0x00` | 32 |
| `0x01A0` | CFC1 `$20` | `DMA_VICE_PT_CH1` | DMA ch1 VICE pointer | r | `0x00` | 16 |
| `0x01A8` | CFC1 `$21` | `DMA_COUNT_CH1` | DMA ch1 remaining count | r | `0x00` | 16 |
| `0x01B8` | CFC1 `$23` | `MSP_SW_INT` | MSP software interrupt | w | — | 0 |
| `0x01C0` | CTC1 `$24` | `DMA_CTL_CH2` | DMA ch2 control | r/w | `0x10` | 16 |
| `0x01C8` | CFC1 `$25` | `DMA_STAT_CH2` | DMA ch2 status | r | `0x10` | 16 |
| `0x01D0` | CTC1 `$26` | `DMA_DATA_CH2` | DMA ch2 data fill | r/w | `0x00` | 16 |
| `0x01D8` | CFC1 `$27` | `DMA_MEM_PT_CH2` | DMA ch2 system pointer | r | `0x00` | 32 |
| `0x01E0` | CFC1 `$28` | `DMA_VICE_PT_CH2` | DMA ch2 VICE pointer | r | `0x00` | 16 |
| `0x01E8` | CFC1 `$29` | `DMA_COUNT_CH2` | DMA ch2 remaining count | r | `0x00` | 16 |
| `0x01F0` | CFC1 `$30` | `BSP_IN_BOX` | BSP/MSP in mailbox | r | `0x00` | 16 |
| `0x01F8` | CTC1 `$31` | `BSP_OUT_BOX` | BSP/MSP out mailbox | r/w | `0x00` | 16 |

### VICE DMA descriptor registers

Two DMA channels, each with 4 descriptor sets (D1–D4). Channel 1 descriptors
at system offset `0x1000`+ (MSP `MTC3`), channel 2 at `0x1100`+ (MSP `CTC3`).
Each descriptor set is 8 registers spaced 8 bytes apart:

| Descriptor offset | Register | Function |
|-------------------|----------|----------|
| `+0x00` | `DMA_CTL_CHx_Dy` | Descriptor control |
| `+0x08` | `DMA_SMEM_HI_CHx_Dy` | Upper address pointer |
| `+0x10` | `DMA_SMEM_LO_CHx_Dy` | Lower address pointer |
| `+0x18` | `DMA_WIDTH_CHx_Dy` | Width in bytes of line |
| `+0x20` | `DMA_STRIDE_CHx_Dy` | Bytes to skip |
| `+0x28` | `DMA_LINES_CHx_Dy` | Number of lines |
| `+0x30` | `DMA_VMEM_Y_CHx_Dy` | VICE pointer Y component |
| `+0x38` | `DMA_VMEM_C_CHx_Dy` | VICE pointer C component |

Channel 1 descriptors D1–D4 base offsets: `0x1000`, `0x1040`, `0x1080`,
`0x10C0`. Channel 2 descriptors D1–D4 base offsets: `0x1100`, `0x1140`,
`0x1180`, `0x11C0`.

### VICE_ID register

| Bits | Function |
|------|----------|
| 3:0 | VICE revision: `0001` = VICE-A (099-0123-001), `0010` = VICE-B "DX" (099-0123-002), `0011` = VICE-C "TRE" (099-0123-003) |
| 7:4 | VICE ID value |

### VICE_CFG register

| Bits | Function |
|------|----------|
| 0 | `check_data_sysad` — check data on SysAD when VICE is external agent |
| 1 | MSP TLB bypass enable |
| 31:2 | reserved |

### VICE_INT / VICE_INT_RESET / VICE_INT_EN bits

| Bit | Function |
|-----|----------|
| 0 | DMA complete interrupt channel 1 |
| 1 | DMA error interrupt channel 1 |
| 2 | MSP software interrupt to Unix processor |
| 3 | MSP exception interrupt to Unix processor |
| 4 | BSP software interrupt to Unix processor |
| 5 | BSP exception interrupt to Unix processor |
| 6 | SysAD erroneous data received |
| 7 | DMA complete interrupt channel 2 |
| 8 | DMA error interrupt channel 2 |

`VICE_INT` is read-only status; `VICE_INT_RESET` is write-1-to-clear;
`VICE_INT_EN` masks the corresponding interrupt sources.

## PCI device map (Linux `arch/mips/pci/fixup-ip32.c`)

O2 has up to 5 PCI devices connected into the MACE bridge:
```
0  aic7xxx 0   (SCSI)
1  aic7xxx 1   (SCSI)
2  expansion slot
3  N/C
4  N/C
```

IRQ routing:
```
SCSI0 = MACEPCI_SCSI0_IRQ
SCSI1 = MACEPCI_SCSI1_IRQ
INTA0 = MACEPCI_SLOT0_IRQ
INTA1 = MACEPCI_SLOT1_IRQ
INTA2 = MACEPCI_SLOT2_IRQ
INTB  = MACEPCI_SHARED0_IRQ
INTC  = MACEPCI_SHARED1_IRQ
INTD  = MACEPCI_SHARED2_IRQ
```

## UART registers (16550-style, decompiled PROM `definitions.h`)

UARTs at `0x1f390000` (UART1) and `0x1f398000` (UART2). Byte-addressed with
`UART_REG(x) = (x << 8) + 7`:

| Register | `UART_REG` |
|----------|-----------|
| `UART_DATA` | `0x07` |
| `UART_IER` | `0x107` |
| `UART_IIR` | `0x207` |
| `UART_LCR` | `0x307` |
| `UART_MCR` | `0x407` |
| `UART_LSR` | `0x507` |
| `UART_MSR` | `0x607` |
| `UART_SCR` | `0x707` |

## RTC registers (MC146818-style, decompiled PROM `definitions.h`)

RTC at `0x1f3a0000`. Byte-addressed with `RTC_REG(x) = x << 8`:

| Register | `RTC_REG` |
|----------|-----------|
| `RTC_SECONDS` | `0x00` |
| `RTC_SECONDS_ALARM` | `0x100` |
| `RTC_MINUTES` | `0x200` |
| `RTC_MINUTES_ALARM` | `0x300` |
| `RTC_HOURS` | `0x400` |
| `RTC_HOURS_ALARM` | `0x500` |
| `RTC_DAY_OF_WEEK` | `0x600` |
| `RTC_DAY_OF_MONTH` | `0x700` |
| `RTC_MONTH` | `0x800` |
| `RTC_YEAR` | `0x900` |
| `RTC_CTRL_A/B/C/D` | `0xa00`–`0xd00` |
| `RTC_CRC` | `0x4700` |
| `RTC_CENTURY` | `0x4800` |
| `RTC_DATE_ALARM` | `0x4900` |
| `RTC_EXT_CTRL_4A/4B` | `0x4a00`/`0x4b00` |
| `RTC_NVRAM(x)` | `(0x0e + x) << 8` |

## TODO / Open questions

- [x] ICE/VICE ASIC register map (VICE Design Spec 099-0123-003)
- [x] Complete Ethernet packet/descriptor register map (NetBSD `if_mecreg.h`,
      Linux `meth.h`)
- [x] Complete mavb codec and audio register semantics (IRIX `ad1843.h` +
      AD1843 datasheet)
- [x] DIMM SPD address, EEPROM layout, and probe behavior — no SPD; PROM
      bank probing (see cpu-memory.md)
- [x] PCI configuration space details (Linux `ops-mace.c`, `pci-ip32.c`,
      `fixup-ip32.c`; config mechanism, windows, control/error bits, Ultra
      SCSI quirk)
| `vt_vblank` | `0x10010` | vertical blank |
| `vt_hblank` | `0x10014` | horizontal blank |
| `vt_flags` | `0x10018` | `VDRV_INVERT` 0, `VDRV_LOW` 1, `HDRV_INVERT` 2, `HDRV_LOW` 3, `SYNC_HIGH` 4, `SYNC_LOW` 5, `F2RF_HIGH` 6 |
| `vt_f2rf_lock` | `0x1001c` | frame-to-raster lock |
| `vt_intr01` | `0x10020` | interrupt 0/1 |
| `vt_intr23` | `0x10024` | interrupt 2/3 |
| `fp_hdrv` | `0x10028` | flat-panel hdrv |
| `fp_vdrv` | `0x1002c` | flat-panel vdrv |
| `fp_de` | `0x10030` | flat-panel data enable |
| `vt_hpixen` | `0x10034` | horizontal pixel enable |
| `vt_vpixen` | `0x10038` | vertical pixel enable |
| `vt_hcmap` | `0x1003c` | horizontal colormap |
| `vt_vcmap` | `0x10040` | vertical colormap |
| `did_start_xy` | `0x10044` | `DID_STARTX` 11:0, `DID_STARTY` 23:12 |
| `crs_start_xy` | `0x10048` | `CRS_STARTX` 11:0, `CRS_STARTY` 23:12 |
| `vc_start_xy` | `0x1004c` | `VC_STARTX` 11:0, `VC_STARTY` 23:12 |

**Overlay plane** (offset `0x20000`):

| Register | Offset | Fields |
|----------|--------|--------|
| `ovr_width_tile` | `0x20000` | `OVR_FIFO_RESET` 13 |
| `ovr_inhwctrl` | `0x20004` | `OVR_DMA_ENABLE` 0 |
| `ovr_control` | `0x20008` | `OVR_DMA_ENABLE` 0 |

**Normal framebuffer plane** (offset `0x30000`):

| Register | Offset | Fields |
|----------|--------|--------|
| `frm_size_tile` | `0x30000` | `FRM_RHS` 4:0, `FRM_WIDTH_TILE` 12:5, `FRM_DEPTH` 14:13, `FRM_FIFO_RESET` 15 |
| `frm_size_pixel` | `0x30004` | `FB_HEIGHT_PIX` 31:16 |
| `frm_inhwctrl` | `0x30008` | `FRM_DMA_ENABLE` 0 |
| `frm_control` | `0x3000c` | `FRM_DMA_ENABLE` 0, `FRM_LINEAR` 1, `FRM_TILE_PTR` 31:9 |

**DID control** (offset `0x40000`):

| Register | Offset | Fields |
|----------|--------|--------|
| `did_inhwctrl` | `0x40000` | `DID_DMA_ENABLE` 0 |
| `did_control` | `0x40004` | `DID_DMA_ENABLE` 0 |

**WID table** (offset `0x50000`): `mode_regs[32]`, fields `BUF` 1:0, `TYP` 4:2,
`CM` 9:5, `GAMMA` 10, `AUX` 12:11.

**Color / gamma map** (offset `0x60000`):

| Register | Offset |
|----------|--------|
| `cmap[6144]` | `0x60000` |
| `cm_fifo` | `0x78000` |
| `gmap[256]` | `0x80000` |
| `gmap10[1024]` | `0x90000` |

**Cursor** (offset `0xa0000`):

| Register | Offset |
|----------|--------|
| `crs_pos` | `0xa0000` |
| `crs_ctl` | `0xa0004` |
| `crs_cmap[3]` | `0xa0008` |
| `crs_glyph[64]` | `0xa0014` |

**Video capture** (offset `0xb0000`): `vc_0` … `vc_8`.

**Color-mode constants** (`GBE_CMODE_*`): `I8` 0, `I12` 1, `RG3B2` 2, `RGB4` 3,
`ARGB5` 4, `RGB8` 5, `RGBA5` 6, `RGB10` 7.

**Driver init sequence** (`gbefb.c`): read `gbe_revision = ctrlstat & 15`; build
tile list; init WID table; `vt_intr01`/`vt_intr23 = 0xffffffff`;
`did_control`/`ovr_width_tile`/`crs_ctl = 0`; init gamma map; init color map +
`gbe_loadcmap()`; program `vt_xymax`/`vsync`/`hsync`/`vblank`/`hblank`;
`vc_start_xy` with `VC_STARTX = hblank_end - 4`; `vt_hpixen`/`vt_vpixen`;
`did_start_xy` with `DID_STARTX = hblank_end - 20`; `crs_start_xy` with
`CRS_STARTY = temp + 1`, `CRS_STARTX = hblank_end - GBE_CRS_MAGIC`;
`fp_de`/`hdrv`/`vdrv`; `frm_size_pixel` with `FB_HEIGHT_PIX`;
`frm_size_tile` with `FRM_WIDTH_TILE`/`FRM_RHS`/`FRM_DEPTH`; then
`ctrlstat`/`dotclock`/`sysclk`/`i2c`/`i2cfp`/`id`/`config`/`bist`.

**Timing info** (`struct gbe_timing_info`): `flags`, `width`, `height`,
`fields_sec`, `cfreq`, `htotal`, `hblank_start`/`hblank_end`,
`hsync_start`/`hsync_end`, `vtotal`, `vblank_start`/`vblank_end`,
`vsync_start`/`vsync_end`, `pll_m`/`pll_n`/`pll_p`. Flags: `GBE_VOF_UNKNOWNMON`
1, `STEREO` 2, `DO_GENSYNC` 4, `SYNC_ON_GREEN` 8, `FLATPANEL` 0x1000,
`MAGICKEY` 0x2000.

### CRIME Control register bits

```text
CRIME_CONTROL_TRITON_SYSADC  0x2000
CRIME_CONTROL_CRIME_SYSADC   0x1000
CRIME_CONTROL_HARD_RESET     0x0800
CRIME_CONTROL_SOFT_RESET     0x0400
CRIME_CONTROL_DOG_ENA        0x0200
CRIME_CONTROL_ENDIANESS      0x0100
CRIME_CONTROL_ENDIAN_BIG     0x0100
CRIME_CONTROL_ENDIAN_LITTLE  0x0000
CRIME_CONTROL_CQUEUE_HWM     0x000f   (command queue high-water mark)
CRIME_CONTROL_CQUEUE_SHFT    0
CRIME_CONTROL_WBUF_HWM       0x00f0   (write buffer high-water mark)
CRIME_CONTROL_WBUF_SHFT      8
```

### CRIME interrupt registers
```
istat      (interrupt status)
imask      (interrupt mask)
soft_int   (software interrupt)
hard_int   (hardware interrupt)
```

### CRIME interrupt bits (hard_int)
```
MACE_VID_IN1_INT        BIT(0)
MACE_VID_IN2_INT        BIT(1)
MACE_VID_OUT_INT        BIT(2)
MACE_ETHERNET_INT       BIT(3)
MACE_SUPERIO_INT        BIT(4)
MACE_MISC_INT           BIT(5)
MACE_AUDIO_INT          BIT(6)
MACE_PCI_BRIDGE_INT     BIT(7)
MACEPCI_SCSI0_INT       BIT(8)
MACEPCI_SCSI1_INT       BIT(9)
MACEPCI_SLOT0_INT       BIT(10)
MACEPCI_SLOT1_INT       BIT(11)
MACEPCI_SLOT2_INT       BIT(12)
MACEPCI_SHARED0_INT     BIT(13)
MACEPCI_SHARED1_INT     BIT(14)
MACEPCI_SHARED2_INT     BIT(15)
CRIME_GBE0_INT          BIT(16)
CRIME_GBE1_INT          BIT(17)
CRIME_GBE2_INT          BIT(18)
CRIME_GBE3_INT          BIT(19)
CRIME_CPUERR_INT        BIT(20)
CRIME_MEMERR_INT        BIT(21)
```

### CRIME interrupt classes (from `ip32-irq.c`)
- **Level-triggered:** `CRIME_CPUERR_IRQ`, `CRIME_MEMERR_IRQ`
- **Edge-triggered:** `CRIME_GBE0..GBE3`, `CRIME_RE_EMPTY_E..RE_IDLE_E`,
  `CRIME_SOFT0..SOFT2`, `CRIME_VICE_IRQ`

## MACE ASIC (Linux `arch/mips/include/asm/ip32/mace.h`)

MACE = Multimedia, Audio and Communications Engine.

### MACE PCI interface
```
struct mace_pci {
    error_addr;
    error;
    MACEPCI_ERROR_MASTER_ABORT      BIT(31)
    MACEPCI_ERROR_TARGET_ABORT      BIT(30)
    MACEPCI_ERROR_DATA_PARITY_ERR   BIT(29)
    MACEPCI_ERROR_RETRY_ERR         BIT(28)
    MACEPCI_ERROR_ILLEGAL_CMD       BIT(27)
    MACEPCI_ERROR_SYSTEM_ERR        BIT(26)
}
```

## Interrupt map (Linux `arch/mips/include/asm/ip32/ip32_ints.h`)

### CRIME interrupts
```
CRIME_VICE_IRQ
```

### MACEISA interrupts
```
MACEISA_AUDIO_SW_IRQ
MACEISA_AUDIO_SC_IRQ
MACEISA_AUDIO1_DMAT_IRQ
MACEISA_AUDIO1_OF_IRQ
MACEISA_AUDIO2_DMAT_IRQ
MACEISA_AUDIO2_MERR_IRQ
MACEISA_AUDIO3_DMAT_IRQ
MACEISA_AUDIO3_MERR_IRQ
MACEISA_RTC_IRQ
MACEISA_KEYB_IRQ
MACEISA_MOUSE_IRQ
MACEISA_TIMER0_IRQ
MACEISA_TIMER1_IRQ
MACEISA_TIMER2_IRQ
MACEISA_PARALLEL_IRQ
MACEISA_PAR_CTXA_IRQ
MACEISA_PAR_CTXB_IRQ
MACEISA_PAR_MERR_IRQ
MACEISA_SERIAL1_IRQ
MACEISA_SERIAL1_TDMAT_IRQ
MACEISA_SERIAL1_TDMAPR_IRQ
MACEISA_SERIAL2_RDMAOR_IRQ
...
IP32_IRQ_MAX = MACEISA_SERIAL2_RDMAOR_IRQ
```

### MACEISA interrupt groups (from `ip32-irq.c`)
```
MACEISA_AUDIO_INT = AUDIO_SW | AUDIO_SC | AUDIO1_DMAT | AUDIO1_OF |
                    AUDIO2_DMAT | AUDIO2_MERR | AUDIO3_DMAT | AUDIO3_MERR
MACEISA_MISC_INT  = RTC | KEYB | KEYB_POLL | MOUSE | MOUSE_POLL |
                    TIMER0 | TIMER1 | TIMER2
```

## PCI device map (Linux `arch/mips/pci/fixup-ip32.c`)

O2 has up to 5 PCI devices connected into the MACE bridge:
```
0  aic7xxx 0   (SCSI)
1  aic7xxx 1   (SCSI)
2  expansion slot
3  N/C
4  N/C
```

IRQ routing:
```
SCSI0 = MACEPCI_SCSI0_IRQ
SCSI1 = MACEPCI_SCSI1_IRQ
INTA0 = MACEPCI_SLOT0_IRQ
INTA1 = MACEPCI_SLOT1_IRQ
INTA2 = MACEPCI_SLOT2_IRQ
INTB  = MACEPCI_SHARED0_IRQ
INTC  = MACEPCI_SHARED1_IRQ
INTD  = MACEPCI_SHARED2_IRQ
```

## Linux IP32 kernel files (`arch/mips/sgi-ip32/`)

From `Makefile`:
```
obj-y += ip32-berr.o ip32-irq.o ip32-platform.o ip32-setup.o ip32-reset.o \
         crime.o ip32-memory.o ip32-dma.o
```

- `ip32-berr.c` — bus error handling
- `ip32-irq.c` — interrupt controller setup
- `ip32-platform.c` — platform devices
- `ip32-setup.c` — machine setup
- `ip32-reset.c` — reset/power
- `crime.c` — CRIME driver
- `ip32-memory.c` — memory setup
- `ip32-dma.c` — DMA

`flush_crime_bus()` does a PIO read to ensure no pending PIO writes.

## NetBSD sgimips files

- `sys/arch/sgimips/mace/mace.c` — MACE driver
- `sys/arch/sgimips/dev/crime.c` — CRIME driver
- `sys/arch/sgimips/dev/crmfb.c` + `crmfbreg.h` — framebuffer
- `sys/arch/sgimips/dev/imc.c` + `imcreg.h` — IMC (memory controller)
- `sys/arch/sgimips/conf/GENERIC32_IP3x` — device tree
- `sys/arch/sgimips/hpc/hpcreg.h` — HPC3 registers (Indy/Indigo2, NOT O2)

## Physical address map (decompiled PROM `definitions.h`)

The decompiled PROM (`samples/decompiled-prom/rev4.18/definitions.h`) is the
authoritative source for the IP32 address map:

| Base | Device |
|------|--------|
| `0x14000000` | CRIME (`PHYS_BASE_CRIME`) |
| `0x15000000` | Render engine (`PHYS_BASE_RENDER`) |
| `0x1f000000` | MACE (`PHYS_BASE_MACE`) |
| `0x1fc00000` | System ROM / PROM (`PHYS_SYSTEM_ROM`) |

MACE sub-offsets (relative to `0x1f000000`):

| Offset | Device |
|--------|--------|
| `0x080000` | PCI |
| `0x280000` | Ethernet |
| `0x300000` | Peripheral (Audio `0x00000`, ISA `0x10000`, KBD/MS `0x20000`, I2C `0x30000`, UST `0x40000`) |
| `0x380000` | ISA external (UART1 `0x10000`, UART2 `0x18000`, RTC `0x20000`) |

## CRIME registers (decompiled PROM `definitions.h`)

Offsets relative to `0x14000000`:

| Register | Offset |
|----------|--------|
| `CRIME_ID_OFFSET` | `0x0000` |
| `CRIME_CONTROL_OFFSET` | `0x0008` |
| `CRIME_INTSTAT_OFFSET` | `0x0010` |
| `CRIME_INTMASK_OFFSET` | `0x0018` |
| `CRIME_SOFT_INT_OFFSET` | `0x0020` |
| `CRIME_HARD_INT_OFFSET` | `0x0028` |
| `CRIME_WATCHDOG_OFFSET` | `0x0030` |
| `CRIME_TIMER_OFFSET` | `0x0038` |
| `CRIME_CPU_ERROR_ADDR` | `0x0040` |
| `CRIME_CPU_ERROR_STAT` | `0x0048` |
| `CRIME_CPU_ERROR_ENA` | `0x0050` |
| `CRIME_MC_STATUS_CTRL` | `0x0200` |
| `CRIME_BANK_0_CTRL` … `CRIME_BANK_7_CTRL` | `0x0208` … `0x0240` |
| `CRIME_REFRESH_COUNTER` | `0x0248` |
| `CRIME_ERROR_STATUS` | `0x0250` |
| `CRIME_ERROR_ADDR` | `0x0258` |
| `CRIME_SYNDROME_BITS` | `0x0260` |
| `CRIME_GENERATED_CHECK_BITS` | `0x0268` |
| `CRIME_REPLACEMENT_CHECK_BITS` | `0x0270` |

CRIME control bits: `TRITON_SYSADC` `0x2000`, `CRIME_SYSADC` `0x1000`,
`HARD_RESET` `0x0800`, `SOFT_RESET` `0x0400`.

### CRIME rendering engine registers

Offsets relative to the render base (`0x15000000`):

| Register | Offset |
|----------|--------|
| `RENDER_INTERFACE_CTRL` | `0x0400` |
| `CRIME_RE_TLB_A/B/C` | `0x1000`/`0x1200`/`0x1400` |
| `CRIME_DE_MODE_SRC` | `0x2000` |
| `CRIME_DE_MODE_DST` | `0x2008` |
| `CRIME_DE_DRAWMODE` | `0x2018` |
| `CRIME_DE_SCRMASK0-4` | `0x2020`–`0x2040` |
| `CRIME_DE_SCISSOR` | `0x2048` |
| `CRIME_DE_WINOFFSET_SRC/DST` | `0x2050`/`0x2058` |
| `CRIME_DE_PRIMITIVE` | `0x2060` |
| `CRIME_DE_X_VERTEX_0/1` | `0x2070`/`0x2074` |
| `CRIME_DE_XFER_STEP_X/Y` | `0x20a8`/`0x20ac` |
| `CRIME_DE_STIPPLE_MODE/PAT` | `0x20c0`/`0x20c4` |
| `CRIME_DE_FG` | `0x20d0` |
| `CRIME_DE_ROP` | `0x21b0` |
| `CRIME_DE_PLANEMASK` | `0x21b8` |
| `CRIME_DE_NULL` | `0x21f0` |
| `CRIME_DE_FLUSH` | `0x21f8` |
| `MTE_MODE` | `0x3000` |
| `MTE_BYTE_MASK` | `0x3008` |
| `MTE_STIPPLE_MASK` | `0x3010` |
| `MTE_FG_VALUE` | `0x3018` |
| `MTE_SRC0/1` | `0x3020`/`0x3028` |
| `MTE_DST0/1` | `0x3030`/`0x3038` |
| `MTE_SRC_Y_STEP` | `0x3040` |
| `MTE_DST_Y_STEP` | `0x3048` |
| `MTE_NULL` | `0x3070` |
| `MTE_FLUSH` | `0x3078` |
| `CRIME_DE_STATUS` | `0x4000` |
| `CRIME_DE_START` | `0x0800` |

## MACE registers (decompiled PROM `definitions.h`)

### PCI host bridge

| Register | Offset |
|----------|--------|
| `MACE_PCI_ERROR_ADDR` | `0x00` |
| `MACE_PCI_ERROR_FLAGS` | `0x04` |
| `MACE_PCI_CONTROL` | `0x08` |
| `MACE_PCI_CONFIG_ADDR` | `0xcf8` |
| `MACE_PCI_CONFIG_DATA` | `0xcfc` |

### Ethernet

| Register | Offset |
|----------|--------|
| `MACE_ETH_MAC_CONTROL` | `0x00` |
| `MACE_ETH_INTR_STATUS` | `0x08` |
| `MACE_ETH_RX_MCL_WR_PTR` | `0x45` |
| `MACE_ETH_RX_MCL_RD_PTR` | `0x46` |
| `MACE_ETH_RX_MCL_DEPTH` | `0x47` |
| `MACE_ETH_MCL_RECEIVE_FIFO(x)` | `x + 0x100` |

PROM observations (`samples/decompiled-prom/rev4.18/firmware.S`): the MAC
control register is accessed through the low 32-bit lane at `0x04` on the
big-endian bus; the receive FIFO is accessed at `0x104`; and the write pointer,
read pointer, and depth are byte registers at `0x45`, `0x46`, and `0x47`.
The PROM diagnostic exercises 16 receive-FIFO positions but does not expose a
complete packet descriptor or transmit-register map.

### Audio

| Register | Offset |
|----------|--------|
| `MACE_AUDIO_STATUS` | `0x00` |
| `MACE_AUDIO_CODEC_STATUS` | `0x08` |
| `MACE_AUDIO_CODEC_INPUT_MASK` | `0x10` |
| `MACE_AUDIO_CODEC_INPUT` | `0x18` |
| `MACE_AUDIO_RING_CTRL_CHAN(x)` | `0x20*x` |
| `MACE_AUDIO_RD_PTR_CHAN(x)` | `0x20*x + 0x8` |
| `MACE_AUDIO_WR_PTR_CHAN(x)` | `0x20*x + 0x10` |
| `MACE_AUDIO_RING_DEPTH_CHAN(x)` | `0x20*x + 0x18` |

PROM observations (`firmware.S`): channel 2 uses ring control `0x40`, read
pointer `0x48`, write pointer `0x50`, and depth `0x58`; ring control and write
pointer are accessed as 64-bit registers. The PROM setup writes control values
`0x1000` and `0x200`, then resets the write pointer. Codec status is polled at
`0x08` and codec input is read/written at `0x18`; codec bit definitions are not
present in the recovered PROM header.

### I2C

| Register | Offset |
|----------|--------|
| `MACE_I2C_CONFIG` | `0x00` |
| `MACE_I2C_STATUS` | `0x10` |
| `MACE_I2C_DATA` | `0x18` |

### PS/2 (keyboard + mouse)

| Register | Offset |
|----------|--------|
| `MACE_KEYBOARD_TX_BUF` | `0x00` |
| `MACE_KEYBOARD_RX_BUF` | `0x08` |
| `MACE_KEYBOARD_CONTROL` | `0x10` |
| `MACE_KEYBOARD_STATUS` | `0x18` |
| `MACE_MOUSE_TX_BUF` | `0x20` |
| `MACE_MOUSE_RX_BUF` | `0x28` |
| `MACE_MOUSE_CONTROL` | `0x30` |
| `MACE_MOUSE_STATUS` | `0x38` |

### ISA interface

| Register | Offset |
|----------|--------|
| `ISA_RING_BASE_AND_RESET` | `0x00` |
| `ISA_MISC_CONTROL` | `0x08` |

`ISA_MISC_CONTROL` bits: `ISA_RESET` `0x01`, `ISA_FLASH_ROM_WRITE_ENABLE` `1<<0`,
`ISA_RED_LED` `1<<4`, `ISA_GREEN_LED` `1<<5`.

## UART registers (16550-style, decompiled PROM `definitions.h`)

UARTs are at `0x1f390000` (UART1) and `0x1f398000` (UART2). Registers are
byte-addressed with `UART_REG(x) = (x << 8) + 7`:

| Register | `UART_REG` |
|----------|-----------|
| `UART_DATA` | `0x07` |
| `UART_IER` | `0x107` |
| `UART_IIR` | `0x207` |
| `UART_LCR` | `0x307` |
| `UART_MCR` | `0x407` |
| `UART_LSR` | `0x507` |
| `UART_MSR` | `0x607` |
| `UART_SCR` | `0x707` |

## RTC registers (MC146818-style, decompiled PROM `definitions.h`)

RTC is at `0x1f3a0000`. Registers are byte-addressed with `RTC_REG(x) = x << 8`:

| Register | `RTC_REG` |
|----------|-----------|
| `RTC_SECONDS` | `0x00` |
| `RTC_SECONDS_ALARM` | `0x100` |
| `RTC_MINUTES` | `0x200` |
| `RTC_MINUTES_ALARM` | `0x300` |
| `RTC_HOURS` | `0x400` |
| `RTC_HOURS_ALARM` | `0x500` |
| `RTC_DAY_OF_WEEK` | `0x600` |
| `RTC_DAY_OF_MONTH` | `0x700` |
| `RTC_MONTH` | `0x800` |
| `RTC_YEAR` | `0x900` |
| `RTC_CTRL_A/B/C/D` | `0xa00`–`0xd00` |
| `RTC_CRC` | `0x4700` |
| `RTC_CENTURY` | `0x4800` |
| `RTC_DATE_ALARM` | `0x4900` |
| `RTC_EXT_CTRL_4A/4B` | `0x4a00`/`0x4b00` |
| `RTC_NVRAM(x)` | `(0x0e + x) << 8` |

## TODO / Open questions

- [ ] ICE ASIC register map (no authoritative register header located yet)
- [x] PROM-visible MRE/RE/DE/MTE register map (`definitions.h`, base
    `0x15000000`)
- [x] Display Engine / GBE register map (`include/video/gbe.h`)
- [ ] Cross-check the GBE map against the leaked IRIX `crmGfxState.c`
- [x] PROM-level MACE Ethernet register map
- [x] PROM-level MACE audio ring/codec register map
- [ ] Complete Ethernet packet/descriptor register map
- [ ] Complete mavb codec and audio register semantics