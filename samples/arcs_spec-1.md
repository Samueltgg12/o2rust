# arcs_spec-1


2

Table of Contents
3

Contents
4

Contents
5

Contents
6

Contents
7

Figures
8

Tables
9

Code Samples
10

Preface
11

Preface
How This Book Is Organized
Cited References
12

Preface
What Typographic Changes and Symbols Mean
13

Introduction to the
Arc Specification 1
14

Introduction to the Arc Specification
1.1  Covered By the ARC Specification
15

Introduction to the Arc Specification
1.2  Covered By Addenda To The ARC Specification
1.3  Not Specified by the ARC Specification
1.4 Conventions Used In the ARC Specification
16

Introduction to the Arc Specification
Rationale –
Note –
1.5  Conformance
17

Part 1 — Base Specification
18

System Architecture 2
2.1  Architectural Working Statement
19

System Architecture
2.2  System States
Off
POFF
PON
POFF
POFF
Reset Program
Reset...
"boot"
Reset... Firmware
2.2.1   State Diagram
20

System Architecture
Note –
OS Loader
Utilities
Utility <-> FW
FW
FW <-> HW
HW
HW<-> Conn
21

System Architecture
Note –
App SW
App <-> OS
OS
OS <-> HAL OS <-> Driver
HAL<>Dvr
HAL Drivers
HAL<-> HW Driver <-> HW
HW
HW<-> Conn
2.3  Software Subsystems
22

System Architecture
2.3.1   Application Software
2.3.2   Operating System Software
2.3.3   Hardware Abstraction Layer (HAL) Software
23

System Architecture
Rationale –
2.3.4   Device Drivers
2.3.5   Loader, Installer, and Independent Utility Software
24

System Architecture
2.4  Hardware Subsystems
2.4.1   Processing Subsystems
2.4.2   Peripheral Attachment Subsystems
25

System Architecture
2.5  Firmware
2.6  System Interface Definitions
26

Platform Hardware 3
3.1  System Configurations
Note –
27

Platform Hardware
3.2  Server System Configuration
28

Platform Hardware
Consequences of Non-Compliance
29

Platform Hardware
3.2.1   Processor Unit
30

Platform Hardware
3.2.2   Floating Point Unit
3.2.3   Cache
3.2.4   Memory
3.2.5   Timing Function Support
31

Platform Hardware
3.2.6   Real Time Clock
3.2.7   System Timer
3.2.8   Console
3.2.9   CD-ROM
3.3  Desktop System Configuration
3.3.1   Keyboard
32

Platform Hardware
Requirements
3.3.2   Pointing Device
Requirements
33

Platform Hardware
3.3.3   Video Subsystem
Requirements
34

Platform Hardware
3.3.4   Audio
35

Platform Hardware
36

Platform Hardware
Note –
Rationale –
3.4  Optional Hardware
3.4.1   Floppy Drive
3.4.2   Serial Ports
37

Platform Hardware
3.4.3   Parallel Port
38

Platform Hardware
3.4.4   SCSI Interface
39

Platform Hardware
3.4.5   Network Interface
Ethernet
40

Platform Hardware
Token Ring
3.5  Additional Hardware
3.6  Media Formats
41

Platform Hardware
3.6.1   Media Formats for System Load
3.6.2   System Partition Formats
3.6.3   Diskettes (5 1/4-inch and 3 1/2 inch)
Note –
42

Platform Hardware
3.6.4   CD-ROM
3.6.5   Disk Storage Devices
3.6.6   Network
43

Platform Hardware
3.6.7   Data Interchange
3.7  Processing Subsystem
44

Platform Hardware
3.7.1   Related Consequences
45

Platform Hardware
3.8  Peripheral Attachment Subsystems (I/O Bus)
3.8.1   Requirements
Rationale –
3.8.2   Related Consequences
46

Platform Firmware 4
4.1  Firmware Conventions
4.1.1    Calling Procedures
47

Platform Firmware
CHAR
SHORT
LONG
UCHAR
USHORT
ULONG
Parameter Passing
Status Codes
48

Platform Firmware
4.1.2   Memory Utilization
Note –
49

Platform Firmware
4.1.3   Stack and Data Addressability
4.1.4   Object Formats
50

Platform Firmware
4.2  The Firmware Environment
4.2.1   Exception Block
Note –
4.2.2   System Parameter Block
SPB Signature 0x1000
SPB Length 0x1004
Revision	Version	0x1008
Pointer to Restart Block 0x100C
Pointer to Debug Block 0x1010
GEVector 0x1014
UTLBMiss Vector 0x1018
Firmware Vector Length 0x101C
Pointer to Firmware Vector 0x1020
Private Vector Length 0x1024
Pointer to Private Vector 0x1028
Adapter Count 0x102C
Adapter 0 Type 0x1030
Adapter 0 Vector Length 0x1034
Pointer to Adapter 0 Vector 0x1038
•
• 0x103C
Definition 4-1: SPB
•
51

Platform Firmware
Note –
52

Platform Firmware
4.2.3   Restart Block
RSTBlock Signature
RSTB Length
Revision Version
Pointer to Next RBlock
Restart Address
Boot Master ID
Processor ID
Boot Status
Checksum
Save Area Length
Saved State Area
Definition 4-2: RESTARTBLOCK
53

Platform Firmware
Restart Procedure
54

Platform Firmware
4.2.4   Environment Variables
Console Initialization Environment Variables
55

Platform Firmware
Software Loading Environment Variables
56

Platform Firmware
Time Zone Environment Variable
Firmware Search Path Environment Variable
4.2.5   System Configuration Data
Rationale –
Note –
57

Platform Firmware
| FPU | CPU | Cache | Memory |
| --- | --- | --- | --- |
| CPU | Memory | Ethernet | Serial SCSI |
| Ethernet | Serial | SCSI |  |
| FPU | Cache Disk Tape Rationale – | Disk | Tape |
58

Platform Firmware
31             16 15                0
Class 000
Type 004
Flags 008
Revision	Version	00C
Key 010
Affinity Mask 014
Configuration Data Size 018
Identifier Length 01C
Pointer to Identifier 020
Definition 4-3: COMPONENT
Component Class and Type
Definition 4-4: COMPONENT_CLASS
Definition 4-5: COMPONENT_TYPE
59

Platform Firmware
Definition 4-6: COMPONENT_TYPE (continued)
Definition 4-7: COMPONENT_TYPE (continued)
Definition 4-8: COMPONENT_TYPE (continued)
60

Platform Firmware
Definition 4-9: COMPONENT_TYPE (continued)
61

Platform Firmware
Definition 4-10: COMPONENT_TYPE (continued)
Definition 4-11: COMPONENT_TYPE (continued)
Note –
62

Platform Firmware
Component Flags
Definition 4-12: COMPONENT_FLAG
63

Platform Firmware
√
√
√
√
√
√√√
√√√
√√
√√√
√√√√√
√√√
√√√
√√
√√
√√√
√√√
√√√√√
√√ √ √ √ √√
Component Version and Revision
Component Key
64

Platform Firmware
| 31 | 23 | 15 |
| --- | --- | --- |
| 24 | 16 | 0 |
| Refill Size | Line Size | Cache Size Affinity Mask |
65

Platform Firmware
Configuration Data Size
Component Identifier
66

Platform Firmware
System Topology Constraints
4.2.6   Additional Configuration Data
67

Platform Firmware
68

Platform Firmware
69

Platform Firmware
70

Platform Firmware
71

Platform Firmware
72

Platform Firmware
4.2.7   Devices, Partitions, Files, and Path Specifications
73

Platform Firmware
Path Specifications
Note –
74

Platform Firmware
4.2.8    System Partition
75

Platform Firmware
Rationale –
Note –
76

Platform Firmware
4.3  Standard Firmware Functions
4.3.1   Program Loading
Load()
77

Platform Firmware
Invoke()
Note –
78

Platform Firmware
Execute()
Note –
79

Platform Firmware
4.3.2   Program Termination
Halt()
PowerDown()
Restart()
80

Platform Firmware
Reboot()
Rationale –
EnterInteractiveMode
4.3.3   Configuration Functions
81

Platform Firmware
GetChild(), GetParent(), GetPeer()
AddChild()
82

Platform Firmware
DeleteComponent()
GetComponent()
GetConfigurationData()
83

Platform Firmware
SaveConfiguration()
Rationale –
Note –
4.3.4   Input/Output Functions
84

Platform Firmware
Open()
Definition 4-13: OPENMODE
85

Platform Firmware
Close()
86

Platform Firmware
Read()
Rationale –
GetReadStatus()
87

Platform Firmware
Write()
88

Platform Firmware
Seek()
Definition 4-14: LARGEINTEGER;
Definition 4-15: SEEKMODE
89

Platform Firmware
Mount()
Definition 4-16: MOUNTOPERATION
90

Platform Firmware
GetFileInformation()
Definition 4-17: FILEINFORMATION
Definition 4-18: FILEATTRIBUTES;
91

Platform Firmware
SetFileInformation()
GetDisplayStatus()
Definition 4-19: DISPLAY_STATUS
92

Platform Firmware
Note –
TestUnicodeCharacter()
4.3.5   Environment Functions
SetEnvironmentVariable()
93

Platform Firmware
GetEnvironmentVariable()
4.3.6   Miscellaneous Functions
GetSystemId()
Definition 4-20: SYSTEMID
94

Platform Firmware
GetMemoryDescriptor()
Definition 4-21: MEMORYTYPE
Definition 4-22: MEMORYDESCRIPTOR
95

Platform Firmware
GetTime()
Definition 4-23: TIMEINFO
GetRelativeTime()
96

Platform Firmware
GetDirectoryEntry()
Definition 4-24: DIRECTORYENTRY
FlushAllCaches()
97

Platform Firmware
4.3.7   The Firmware Function Vector
98

Platform Firmware
4.3.8   Platform-Specific Firmware Functions
4.3.9   Adapter-Specific Firmware Functions
4.4  Loaded-Program Conventions
Note –
99

Platform Firmware
Note –
100

Platform Firmware
Rationale –
101

Platform Firmware
4.5  Interrupts and Exceptions
Invoking Exception Handlers
102

Platform Firmware
Exception Handler Routines
Loaded Program Access to Exceptions
103

System Console 5
5.1  Functionality
5.1.1   Basic Console Input
104

System Console
Note –
105

System Console
5.1.2   UNICODE Console Input
5.1.3   Basic Console Output
106

System Console
107

System Console
Note –
Note –
108

System Console
Note –
5.1.4   UNICODE Console Output
109

System Console
5.2  Operational Characteristics
Note –
110

Multiprocessor Platforms 6
6.1  MP Architecture Overview
111

Multiprocessor Platforms
6.2  Processor Subsystem and Caches
6.2.1   Processor Instruction set
6.2.2   User Application Portability Considerations
6.2.3   Symmetry and Shared Memory
6.2.4   Homogeneity of CPUs
112

Multiprocessor Platforms
6.2.5   Hardware-Enforced Cache Coherency
6.2.6   Cache Coherency During I/O Transfers
6.2.7   Atomic Writes
6.2.8   Strong Ordering
6.2.9   Processor Identification
113

Multiprocessor Platforms
6.2.10 Timer Interrupts
6.2.11 Optional Powerfail Interrupt
6.3  I/O Subsystem
6.3.1   Symmetry
6.4  Interprocessor and I/O interrupts
6.4.1   Interprocessor Interrupts
114

Multiprocessor Platforms
6.4.2   Interprocessor Interrupt Priority
6.4.3   I/O interrupt Assignment
6.5  Boot and Reset functions
6.5.1   Boot Master CPU
6.5.2   Starting CPUs
115

Multiprocessor Platforms
6.5.3   Program Termination Function Semantics for MP Machines
116

Part 2 — Developing Material
117

Network Bootstrap
Protocol Mappings 7
7.1  BOOTP/TFTP/UDP/IP/ARP Protocols
118

Network Bootstrap Protocol Mappings
7.2  Networked System Partition
7.2.1   BOOTP/TFTP Protocol References
119

Network Bootstrap Protocol Mappings
7.2.2   System Interface Mapping
Open()
120

Network Bootstrap Protocol Mappings
Read()
Write()
Close()
121

Network Bootstrap Protocol Mappings
GetReadStatus()
Mount()
Seek()
122

Network Bootstrap Protocol Mappings
GetDirectoryEntry()
7.2.3   Protocol Clarifications
Token-Ring MAC Requirements
Ethernet MAC Requirements
LLC Requirements
123

Network Bootstrap Protocol Mappings
BOOTP Request Frame Requirements
BOOTP Response Frame Requirements
TFTP RRQ Frame Requirements
124

Network Bootstrap Protocol Mappings
TFTP ERROR Frame Requirements
ICMP Frame Requirements
ARP Frame Requirements
125

Network Bootstrap Protocol Mappings
7.2.4   Server Considerations
BOOTP vs. RARP
LLC Support
Filename Support
7.3  IBM DLC RIPL/LLC Protocols
126

Network Bootstrap Protocol Mappings
7.3.1   Protocol References
7.3.2   System Interface Mapping
Open()
127

Network Bootstrap Protocol Mappings
Read()
Write()
Close()
GetReadStatus()
128

Network Bootstrap Protocol Mappings
Mount()
Seek()
7.3.3   Protocol Clarifications
129

Network Bootstrap Protocol Mappings
Token-Ring MAC Requirements
Ethernet MAC Requirements
FIND Frame Requirements
130

Network Bootstrap Protocol Mappings
131

Network Bootstrap Protocol Mappings
FOUND Frame Requirements
SEND.FILE.REQUEST Frame Requirements
132

Network Bootstrap Protocol Mappings
133

Network Bootstrap Protocol Mappings
FILE.DATA.RESPONSE Frame Requirements
LOAD.ERROR Frame Requirements
PROGRAM.ALERT Frame Requirements
134

Network Bootstrap Protocol Mappings
7.3.4    Server Considerations
135

Glossary
136

Glossary
137

Index
A
B
138

Index
C
D
139

Index
E I
F
K
L
G
M
H
140

Index
N R
O
S
P
141

Index
T
V
142
