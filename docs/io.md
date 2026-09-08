# O2 (IP32) I/O

## MACE (I/O Engine ASIC)

MACE provides the bulk of the O2's I/O:

- **64-bit PCI bus** (single expansion slot)
- **ISA bus** (used solely for the Super I/O chip)
- **2× PS/2 ports** (keyboard + mouse)
- **10/100 Base-T Ethernet**
- Audio (MACEISA audio)

### MACE registers

- `MACE_BASE = 0x1f000000` (physical)
- See [register-maps.md](register-maps.md) for the full register map

MACE sub-blocks (from the decompiled PROM `definitions.h`):

| Offset | Device |
|--------|--------|
| `0x080000` | PCI host bridge |
| `0x280000` | Ethernet |
| `0x300000` | Peripheral (Audio, ISA, KBD/MS, I2C, UST) |
| `0x380000` | ISA external (UART1, UART2, RTC) |

## PCI bus

- 64-bit, provided by MACE
- Single expansion slot
- PCI devices (from `fixup-ip32.c`):
  - aic7xxx SCSI controller (2 devices: SCSI0, SCSI1)
  - Expansion slot
  - N/C, N/C
- IRQ routing: SCSI0/SCSI1/SLOT0-2/SHARED0-2

## SCSI

- UltraWide SCSI
- Adaptec AIC-7880 controller
- R5000/RM7000 units: 2 drive sleds
- R10000/R12000 units: 1 drive sled

## ISA bus

- Used solely for the Super I/O chip
- Provides serial + parallel ports

## Ethernet

- 10/100 Base-T, provided by MACE.
- Ethernet block base: `0x1f280000` (`BASE_MEC`), from the decompiled PROM
  `definitions.h`.
- The PROM accesses the MAC control register as a 64-bit register and uses its
  low 32-bit lane at offset `0x04` on the big-endian bus. It writes zero then
  one during the controller reset/start sequence.
- PROM diagnostics write and read the receive FIFO at `0x104` and inspect the
  byte-sized receive FIFO pointers at `0x45` (write), `0x46` (read), and `0x47`
  (depth). The diagnostic exercises 16 FIFO positions and returns failure bits
  for control, FIFO, base-register, and final-state checks.
- These are PROM-observed behaviors, not a complete packet TX/RX programming
  model. The controller descriptor and interrupt semantics remain open.

## Audio

- MACEISA audio block base: `0x1f300000` (`BASE_AUDIO`), from the decompiled
  PROM `definitions.h`.
- The PROM initializes audio channel 2's ring control at offset `0x40` and its
  write pointer at offset `0x50` using 64-bit accesses. It sets ring-control
  values `0x1000` and `0x200` during setup, resets the write pointer to zero,
  and advances it while copying data into the ring.
- Codec status/input registers are at offsets `0x08` and `0x18`; the PROM polls
  codec status while transferring codec input data. Bit meanings and complete
  channel behavior still require the MACE audio source or hardware reference.

## UART (serial)

- Two 16550-style UARTs at `0x1f390000` (UART1) and `0x1f398000` (UART2)
- Registers byte-addressed: `UART_REG(x) = (x << 8) + 7`
- See [register-maps.md](register-maps.md) for the register list

## RTC

- MC146818-style RTC at `0x1f3a0000`
- Registers byte-addressed: `RTC_REG(x) = x << 8`
- Includes NVRAM (`RTC_NVRAM(x) = (0x0e + x) << 8`)
- See [register-maps.md](register-maps.md) for the register list

## PS/2

- Keyboard + mouse ports at `0x1f320000` (MACE peripheral KBD/MS)
- TX/RX buffers, control, and status registers for each
- See [register-maps.md](register-maps.md) for the register list

## I2C

- I2C controller at `0x1f330000` (MACE peripheral I2C)
- Config, status, and data registers
- Used to read monitor EDID
- See [register-maps.md](register-maps.md) for the register list

## NetBSD driver

- `sys/arch/sgimips/dev/mace.c` — MACE driver
- `sys/arch/sgimips/dev/hpcreg.h` — HPC (?) register definitions

## TODO / Open questions

- [x] PROM-level MACE Ethernet register map and reset/FIFO behavior
- [x] PROM-level MACE audio ring/codec register map
- [x] Complete MACEISA audio register details (Linux `struct mace_audio` +
      `sgio2audio.c`; AD1843 codec register map from IRIX `ad1843.h` and the
      AD1843 datasheet)
- [x] PCI configuration space details (Linux `ops-mace.c`/`pci-ip32.c`/
      `fixup-ip32.c`)
- [x] Complete Ethernet controller register details (NetBSD `if_mecreg.h`,
      Linux `meth.h` — full MAC110 register map, TX/RX descriptor formats)