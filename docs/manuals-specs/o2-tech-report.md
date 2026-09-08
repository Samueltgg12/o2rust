# o2-tech-report

•
•
•
•
Memory
CPU
Memory
and Display
Display
Rendering Engine (DE)
Engine (MRE)
Imaging
and Compression
Engine (ICE)
I/O Engine
(IOE)
networks, video,
peripherals, disk drives

•
•
•
•
•
•

Memory
256 bits wide
@ 66 MHz =
2.1GB/sec
sustained
Secondary bandwidth
R5000
CPU Cache
Memory
Sys AD Bus and
64 bits (clock rate Rendering
varies with CPU type, Engine (MRE)
up to 100MHz)
Imaging
and Compression
Engine (ICE)
Memory
256 bits wide
R10000	Secondary	@ 66 MHz =
2.1GB/sec
CPU Cache
sustained
bandwidth
Sys AD Bus
Memory
CPU
Sys AD Bus and
Interface
Rendering
64 bits (clock rate
ASIC
varies with CPU Engine (MRE)
type, up to 100MHz)
Imaging
and Compression
Engine (ICE)

•
•

•
•
•
•
•
•
•
•
•
•
•
•
•
•

x144SDRAM SIMM
x144SDRAM SIMM
x144SDRAM SIMM
x144SDRAM SIMM
288+36
2133 MB/s
MUX/DEMUX
9X ALVC162280
17+8 144+18+2
Processor  Module
64+40	64+4	24+5
CPU
MRE	DE	VDAC
*
L2 1067
Flat
MB/s
Cache 32+4
Panel
Adapter
ICE
533 MB/s
Analog A/V Module
16+20+16
Serial Port 1/ Headphone
Serial Port 2 Super
IOE Microphone
Parallel	I/O	Audio	Speaker
64+40 Line In
Codec
Line Out 1
Keyboard/	RTC	ROM
Line Out 2
Mouse
Ethernet
Video Composite
Y/C
Encoder
External F20WSCSI
Video Composite
F20WSCSI Y/C
System Disk Decoder
CDROM Camera / Digital
PCI64 Slot
Data Disk Video Port
*CPU Interface ASIC; Optional, depending on configuration
R10000 195MHz only

•
•
•
•
•
•
•
•
•
•
•
•
•
•
•
•
•
•
•

288 bits wide
256 data bits
DIMM
DIMM
DIMM
DIMM
CPU
Memory
Controller
Display
__________
Engine
(DE)
Rendering
Engine
Imaging (MRE)
and
Compression
Engine (ICE)
I/O Engine
(IOE)

Audio Left In
Audio Right In
Microphone In
Headphones Out
Audio Right Out
Audio Left Out
Stereo Audio Out
•
•
•
•
•
•

•
•
•
•
•
•
•
•
•
•
•
•
•
•
•
•
•
•
•
•
•
•
•
•
•
•
•
•

•
•
•
•
•
•
Video Input Filtering Color Space Conversion
DMA
and Scaling and Pixel Packing
Channel A
Video Input
Video Input Filtering Color Space Conversion
DMA
and Scaling and Pixel Packing
Channel B
Genlock
Video Timing
Generation
Video Output Color Space Conversion
DMA
Filtering and Pixel Unpacking
Video Output

Headphone
Microphone
IOE	Audio	Speaker
Codec Line In
Line Out 1
Line Out 2
Video Composite
Encoder
Y/C
Composite
Video
Decoder Y/C
Camera / Digital
Video Port

Camera/Digital Video I/O
Microphone In
Headphones Out
Audio Right In
Audio Right Out
Audio Left In
Audio Left Out
Composite video In
S-video In
S-video Out Composite video Out
Stereo Audio Out

Mic in
DD11
ININ11
Stereo In
DD11 Headphones
IINN22
Stereo out
DD11
OOUUTT
Analog house in
DD11 GPI in
OOUUTT D1
1
IN	Primary 601 in	Anolog house out
GPI out
D1
2 Secondary 601 in
IN
D1
OUT 601 out
D1
OUT 601 out
•
•
•

•
•
•
•
•
•
•
•
•
•
•
Headphone
Microphone
Audio
IOE Speaker
Codec
Line In
Line Out
601 In #1
601 In #2
601 Out #1
601 Out #2
Genlock In
Genlock
Loopthrough
GPI In
GPI Out

Genlock in (red)
GPI in (green)
Genlock out (loopthrough)(blue)
GPI out (black)

Record button	Lens cover	Focus ring
Tilt
hinge
LED activity
indicator Directional
microphone

DMA /
Bus Arbitrator
Data RAM
16 128
64
Sys AD
32	Scalar	Vector	Bus
Unit Unit
Table
BSP
Memory MIPS
core
Instruction
Media Memory
Instruction
Memory Signal
Bistream
Processor
Processor

MRE DE
Serial Port 1/
Serial Port 2 Super
IOE A/V
Parallel I/O
Module
Keyboard/
Mouse
Ethernet
External F20WSCSI
F20WSCSI
System Disk
CDROM
PCI64 Slot
Data Disk

| 31 | 24  23 | 16  15 | 8   7 | 0 |
| --- | --- | --- | --- | --- |
| RGBA8 | R | GBA |  |  |
| 15   14 | 98 | 5 4 | 0 |  |
| A1_RGB5 | A R G | B |  |  |
| 15 | 12  11 | 8  7 | 43 | 0 |
| RGBA4 | R | G | B | A |
| 7 | 54 | 21 0 |  |  |
| RG3_B2 | R | G | B |  |
| 15 | 12 I12 I 7 0 I8 I | 11 | 0 |  |

•
•
•
•
•
•
•
•

•
•
•
•
•
•
•
•
•
