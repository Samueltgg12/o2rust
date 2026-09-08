# o2-pci-digital-audio-board-installation-guide

PCI Digital Audio Board
Installation Guide
Document Number 007-3502-003

CONTRIBUTORS
Written by Judy Muchowski , Elizabeth Deeth, and Alan Stein
Illustrated by Dany Galgani and Cheri Brown
Production by Kirsten Johnson, Rena Patel, and Glen Traefald
Engineering contributions by Jeff Dinapoli, Ted Marsh, Michael Poimboeuf, Aldon Caron, Reed
Lawson, and Jeff Milo
Cover design by Sarah Bolles, Sarah Bolles Design, and Dany Galgani, SGI Technical
Publications
COPYRIGHT
(C) 2000, Silicon Graphics, Inc. All rights reserved; provided portions may be copyright in third
parties, as indicated elsewhere herein. No permission is granted to copy, distribute, or create
derivative works from the contents of this electronic documentation in any manner, in whole or
in part,without the prior written permission of Silicon Graphics, Inc.
LIMITED RIGHTS LEGEND
The electronic (software) version of this document was developed at private expense; if
acquired under an agreement with the USA government or any contractor thereto, it is acquired
as "commercial computer software" subject to the provisions of its applicable license agreement,
as specified in (a) 48 CFR 12.212 of the FAR; or, if acquired for Department of Defense units, (b)
48 CFR 227-7202 of the DoD FAR Supplement; or sections succeeding thereto.
Contractor/manufacturer is Silicon Graphics, Inc., 1600 Amphitheatre Pkwy 2E, Mountain
View, CA 94043-1351.
TRADEMARKS
Silicon Graphics, IRIX, and OCTANE are registered trademarks and SGI, the SGI logo,
GIGAchannel, O2, OCTANE2, Onyx2, Onyx 3000, Origin200, Origin2000, Origin 3000, and the
SGI 2000 and 3000 series servers are trademarks of Silicon Graphics, Inc. UNIX is a registered
trademark in the United States and other countries, licensed exclusively through X/Open
Company, Ltd. Windows NT is a trademark of Microsoft Corporation.
For regulatory and compliance information, see your workstation’s Owner’s Guide.
ADAT is a registered trademark of Alesis Corporation.
PCI Digital Audio Board Installation Guide
Document Number 007-3502-003

Contents
List of Figuresv
List of Tablesvii
Introductionix
Reader Comments    x
1. Installing the PCI Digital Audio Board1
PCI Digital Audio Board Package    3
Installing the PCI Digital Audio Board in the O2 Workstation    3
Installing the PCI Digital Audio Board in the Octane and Octane2 Workstations    4
Placing a System Upgrade Label    5
Installing the PCI Digital Audio Board in the Origin200 Server    6
Installing the PCI Digital Audio Board in Onyx 3000 Workstations and
Origin 3000 Servers    7
Supported Configurations for All Silicon Graphics Systems    8
Connecting the Cables    9
Installing the Software    10
Verifying the Hardware Installation    11
Connecting ADAT Optical Devices    11
2. Using the Audio Panel Software13
Using the Audio Panel to Control ADAT and AES    13
Displaying ADAT and AES Device Controls    14
Setting the Default Input and Output to ADAT or AES    14
Setting the Input and Output Signal Path    15
3. Technical Specifications and Product Support17
PCI Digital Audio Board Specifications    18
DB15 Connector Pinouts    19
iii

Contents
Product Support    20
Glossary21
Index23
iv

List of Figures
Figure 1-1 PCI Digital Audio Board    2
Figure 1-2 Rear Views of the Octane and Octane2 Workstations    5
Figure 1-3 The Breakout Cable    10
v

List of Tables
Table 1-1 Maximum Number of PCI Audio Boards in Origin200 Servers    6
Table 1-2 Supported Configurations    8
Table 3-1 PCI Audio Board’s Specifications    18
Table 3-2 DB15 Connector Pinout Assignments    19
vii

Introduction
This guide tells you how to locate instructions for installing the PCI Digital Audio Board
in O2, OCTANE, OCTANE2, and Onyx 3000 workstations and Origin200 and
Origin 3000 servers.
Note: The PCI Digital Audio Board can also be installed in Onyx2 workstations and in
Origin2000 servers, and SGI 2000 series servers. However, the installation must be
performed by an authorized SGIfield engineer. These systems use internal electrical
power that is hazardous if the equipment is improperly handled. Installation procedures
require specific training and technical knowledge, as provided to SGI System Support
Engineers or other personnel trained by SGI.
The following topics are covered in this guide:
• Chapter 1,“Installing the PCI Digital Audio Board”explains where tofind PCI
board installation instructions for your workstation or server, and describes how to
verify the board installation after installing the software. It also explains how to
connect cables once the board is in place.
• Chapter 2,“Using the Audio Panel Software”describes how to use the Audio Panel
®
to display the ADATand AES-3id device controls and to set the default input and
output audio devices.
• Chapter 3,“Technical Specifications and Product Support”provides technical
information and explains how to contact SGI.
ix

Introduction
Reader Comments
If you have comments about the technical accuracy, content, or organization of this
document, please send them to SGI. Be sure to include the title and document number of
the manual with your comments. (Online, the document number is located in the front
matter of the manual. In printed manuals, you canfind the document number on the
back cover.)
You can contact us in any of the following ways:
• Send e-mail to the following address:
• Use the Feedback option on the Technical Publications Library World Wide Web
page:
• Contact your customer service representative and ask that an incident befiled in the
SGI incident tracking system.
• Send mail to the following address:
Technical Publications
SGI
1600 Amphitheatre Pkwy., M/S 535
Mountain View, California 94043-1351
• Send a fax to the attention of“Technical Publications”at +1 650 932 0801.
SGI values your comments and will respond to them promptly.
x

Chapter 1
1. Installing the PCI Digital Audio Board
This chapter explains where tofind PCI board installation instructions for your
workstation or server, and describes how to verify the board installation. It also explains
how to connect cables once the board is in place.
The following topics are covered:
•“PCI Digital Audio Board Package”on page 3
•“Installing the PCI Digital Audio Board in the O2 Workstation”on page 3
•“Installing the PCI Digital Audio Board in the Octane and Octane2 Workstations”
on page 4
•“Installing the PCI Digital Audio Board in the Origin200 Server”on page 6
•“Installing the PCI Digital Audio Board in Onyx 3000 Workstations and Origin 3000
Servers”on page 7
•“Supported Configurations for All Silicon Graphics Systems”on page 8
•“Connecting the Cables”on page 9
•“Installing the Software”on page 10
•“Verifying the Hardware Installation”on page 11
•“Connecting ADAT Optical Devices”on page 11
1

Chapter 1: Installing the PCI Digital Audio Board
The PCI Digital Audio Board (Figure 1-1) is a half size PCI expansion board that provides
multichannel digital audio expansion via the PCI bus. It connects to consumer and
professional audio and video equipment via industry-standard interfaces. If your
workstation has multiple PCI slots, you can install multiple PCI Digital Audio Boards for
more channels of synchronized digital audio. To determine the maximum number of
boards you can install in your workstation or server, see the appropriate section later in
this chapter.
Coaxial digital in
Video ref loop
Coaxial digital out
Breakout cable
Video ref loop
ADAT optical out
Feature
ADAT optical in connector
Figure 1-1 PCI Digital Audio Board
2

PCI Digital Audio Board Package
PCI Digital Audio Board Package
Your package includes all of the following items. If anything is missing, contact your
sales representative.
• PCI Digital Audio Board
• breakout cable
•fiber optic cable
• feature cable (for use on media-ready Origin200 servers only)
• bag of hardware for installing the board
• antistatic wrist strap
• PCI Digital Audio Board Installation Guide(this manual)
• Compact disc (CD-ROM)
Installing the PCI Digital Audio Board in the O2 Workstation
In an O2 workstation, you install the PCI Digital Audio Board in the PCI module located
on the system module.
Note: The feature connector (see Figure 1-1) is not used in the O2 workstation.
For instructions on how to install the board, see the section on installing PCI boards in
theO2 Workstation Owner’s Guide.
• If you don’t have the printed manual handy, it is available on the Internet in the SGI
Technical Publications Library at the following location:
Choose Catalogs> Hardware Catalog > and look in the O2 section for theO2
Hardware Reference Guide.
After installing the board in the PCI tray and before replacing the system module in the
O2 chassis, connect the cables. In this guide, go to“Connecting the Cables”on page 9.
3

Chapter 1: Installing the PCI Digital Audio Board
Installing the PCI Digital Audio Board in the Octane and Octane2 Workstations
In Octane and Octane2 workstations, you install the PCI Digital Audio Board in the PCI
module, which is an optional module. You can install the board in any open PCI slot. A
maximum of three PCI Digital Audio Boards can be installed.
For instructions on installing the board, see theOCTANE PCI Module Installation Guide
that came with the optional PCI module. Or see the section on installing PCI boards in
the printedOCTANE or OCTANE2 Workstation Owner’s Guide, which is shipped with
these workstations.
If you don’t have either of the printed manuals handy, the manuals are available on the
Internet in the SGI Technical Publications Library at the following location:
Choose Catalogs> Hardware Catalog > and look in the Octane section for theOCTANE
orOCTANE2 Workstation Owner’s Guide,or theOCTANE PCI Module Installation Guide.
After installing the board in the PCI module and before reinstalling the module in the
workstation, connect the cables. In this manual, go to“Connecting the Cables”on page 9.
4

Installing the PCI Digital Audio Board in the Octane and Octane2 Workstations
Placing a System Upgrade Label
Model/CMN number
Model/CMN number
System label
System label
RL RL
In   Out In   Out
2	1	2	1
Octane Workstation Octane2 Workstation
Figure 1-2 Rear Views of the Octane and Octane2 Workstations
If you received an Octane or Octane2 system upgrade label, place it on the system label
as follows:
Face the rear of the workstation. The system label (containing the model/CMN number)
is located at the top center of the rear of the workstation. Place the label over the VCCI
and CISPR 22 information.
5

Chapter 1: Installing the PCI Digital Audio Board
Installing the PCI Digital Audio Board in the Origin200 Server
Table 1-1 lists the maximum number of boards you can install in the Origin200 and
Origin200 GIGAchannel servers and describes performance issues.
Table 1-1 Maximum Number of PCI Audio Boards in Origin200 Servers
| Server Type | Maximum Number of PCI | Where To Install the Boards for Optimum Performance Digital Audio Boards |
| --- | --- | --- |
| Origin200 | 3 per CPU module (6 max.) | Install boards starting with slot 5 in the CPU module. |
| Origin200 GIGAchannel | 6 boards maximum in the | Install boards anywhere in the GIGAchannel expansion cabinet entire system first, then in the CPU module (starting with slot 5). For instructions on installing the board, and how to connect the feature connector cable received with this package, see theOrigin200 and Origin200 GIGAchannel Maintenance Guide. Most manuals are available on the Internet in the SGI Technical Publications Library at the following location: Choose Catalogs > Hardware Catalog and look in the Origin section for the appropriate manual. After installing the board in the Origin200, return to this guide and go to“Connecting the Cables”on page 9. Note: There are some versions of the audio board that do not support media synchronization on Origin200 servers. For a list of supported and non-supported configurations (by part number), see“Supported Configurations for All Silicon Graphics Systems”on page 8. |
6

Installing the PCI Digital Audio Board in Onyx 3000 Workstations and Origin 3000 Servers
Installing the PCI Digital Audio Board in Onyx 3000 Workstations and Origin 3000
Servers
You can install a maximum of six PCI Digital Audio Boards in Onyx 3000 workstations
and Origin 3000 servers. For instructions on installing the board, see the sections on
installing PCI boards in the I-Brick and the P-Brick in the appropriate owner’s guide.
Note: The Onyx 3000 ships with one PCI Digital Audio Board installed in the I-Brick.
Most manuals are available on the Internet in the SGI Technical Publications Library at
the following location:
Choose Catalogs > Hardware Catalog and look in the Onyx or Origin section for the
appropriate manual.
After installing the board in the Onyx 3000 or Origin 3000, return to this guide and go to
“Connecting the Cables”on page 9.
Note: If you have more than one board installed and you want to reconfigure the slot
numbers for the digital audio software, call your System Administrator.
7

Chapter 1: Installing the PCI Digital Audio Board
Supported Configurations for All Silicon Graphics Systems
Warning: To prevent serious physical damage, do not use audio boards with part
numbers 030-0950-001 and 030-0950-002 with the IRIX-based Onyx 3000 series and
Origin 3000 series, or the Windows NT-based 320 and 540 Workstations.
Table 1-2 shows the audio board compatibility information for all Silicon Graphics
systems.
Table 1-2 Supported Configurations
| Board part number | Compatible platforms | Notes |
| --- | --- | --- |
| 030-0950-001 | O2, Octane, Octane2 | Do not use with Origin200 media-ready servers, Origin 3000 or Onyx 3000 |
| 030-0950-002 | O2, Octane, Octane2, Origin200 media-ready servers nonmedia-ready servers, Origin 3000, or Onyx 3000 | Do not use with Origin200 |
| 030-1649-001 | All platforms, except as noted nonmedia-ready servers | Do not use with Origin200 |
| 030-1441-001 | All platforms, except as noted | This board was designed for the Silicon Graphics 320 and 540 Windows NT systems but is compatible with all IRIX-based platforms if the required software is installed |
8

Connecting the Cables
Note:Do not install the030-0950-001audio board in Origin200 media-ready servers or the
007-1649-001 and 030-0950-002 boards in non media-ready Origin200 servers. Although
these boards work, some of the functionality is lost. For example, with these boards, you
cannot fully synchronize the audio in this module with audio in another module. For full
operation of the PCI Digital Audio Board in an Origin200 media-ready server, use the
board and the feature connector cable shipped in this package. If you install the
030-0950-001 board in a media-ready Origin200 server, the following error message
appears:
Connecting the Cables
You received three cables with your package—afiber optic cable, a feature cable (for use
on Origin200 servers only), and a breakout cable (Figure 1-3). Follow the steps below to
remove the dust caps and connect the cables. (It’s easier to remove the dust caps before
reinstalling the PCI module in the chassis.)
®
1. Remove the dust caps from the ADAT Optical interface connectors (see Figure 1-1).
2.    Attach one end of the providedfiber optic cable to one of the ADAT connectors and
the other end of the same cable to the second ADAT connector. This protects the
connectors from dust until you are ready to connect a device. At that time, remove
one end of the cable and connect it to the device. Use the cable provided with the
optical device for the second optical interface connector.
3.    Connect the breakout cable to the DB15 connector (see Figure 1-1). The breakout
cable supports video synchronization and the AES-3id digital audio standard. The
black and green video ref loop cables are interchangeable.
Note:If you are using only a video input and not looping through to another device,
you must terminate the unused video connector with a 75 ohm BNC terminator.
To verify that the system recognizes the new board, see“Verifying the Hardware
Installation”on page 11.
9

Chapter 1: Installing the PCI Digital Audio Board
Coaxial digital in (red)
Video ref loop (green)
Coaxial digital out (blue)
Video ref loop (black)
Figure 1-3 The Breakout Cable
Installing the Software
After installing the PCI Digital Audio Board, read any release notes on the CD that came
in this package. In addition, read the booklet enclosed with the software CD for
information on installing the software.
10

Verifying the Hardware Installation
Verifying the Hardware Installation
After you install the PCI Digital Audio Board and the required software, follow these
steps to verify that the system recognizes the new hardware:
®
1. Open a UNIXshell. At the prompt type
hinv
The hardware inventory list appears.
2.    Look for a line similar to this:
A line appears for each PCI Digital Audio Board installed.
If the board(s) do not appear in the hardware inventory list, turn off your
workstation and repeat each step of the installation instructions. Make sure the
board is fully seated in the slot.
Connecting ADAT Optical Devices
You can connect ADAT Optical interface-compatible devices to the optical input or
output connectors on the PCI Digital Audio Board.
Note:Some of these devices provide the option of clocking their inputs either externally
or internally. Often the default is set to internal clocking. For output to these devices to
work properly, you must configure them to use external clocking. Refer to the manual
that comes with the device for instructions on how to do this.
11

Chapter 2
2. Using the Audio Panel Software
This chapter describes how to use the Audio Panel to control ADAT Optical and AES-3id
I/O.
If you did not install the software, do so now. For information on installing the software,
read the release notes on the CD and the booklet enclosed with the CD.
Using the Audio Panel to Control ADAT and AES
You can use the Audio Panel program on your system to manage multiple audio inputs
and outputs, and to control various audio settings. Refer to the Audio Panel Help menu
for general information on using the Audio Panel.
Note: Software instructions included in this section are based on the IRIX 6.3, IRIX 6.4,
and IRIX 6.5.x operating systems. If you upgraded to a subsequent operating system,
some of the instructions may no longer be valid. If this is the case, check the latest
information by using the online documentation that came with your new software.
When a PCI Digital Audio Board is installed, you can use the Audio Panel to display the
ADAT and AES device controls, set the default input and output to ADAT or AES I/O,
and set the audio signal path to ADAT or AES I/O.
13

Chapter 2: Using the Audio Panel Software
Displaying ADAT and AES Device Controls
To display the ADAT and AES device controls:
1. Open Audio Panel:
2.    From the Toolchest, choose Desktop > Control Audio.
3.    Select the desired AES or ADAT device control panel from the view menu.
The selected device control panel appears, where you can adjust parameters such as
sample rate, input source, and output destination. For details, refer to the Audio
Panel Help menu.
If more than one PCI Digital Audio Board is installed, the system names each board using
an incremental prefix naming scheme. For example, if two PCI Digital Audio Boards are
installed in your system, one ADAT input is displayed as“RAD 1.ADAT In”and the
other as“RAD 2. ADAT In.”
Setting the Default Input and Output to ADAT or AES
Audio Panel displays device controls for ADAT In, ADAT Out, AES In, and AES Out for
each PCI Digital Audio Board installed in your system. You can designate any of these
devices as a default, which means your system uses the device as the internal audio
device unless otherwise specified by another application. To set the default input and
output, simply open Audio Panel and use the Default menu to select a default input and
output and device.
14

Using the Audio Panel to Control ADAT and AES
Setting the Input and Output Signal Path
You can select either electrical or optical signal paths for the AES input and output
devices. For ADAT input and output devices, you can select either the optical signal path
or a disabled signal path (labeled“None”).
To set the input source with Audio Panel:
1. Position the cursor over the input device panel (if the device panel you need is not
displayed, use the View menu), then hold down the right mouse button to display a
menu that contains the Input Source option.
2.    Choose the input source (for example, AES In or Optical In) that you want the input
device to use.“AES In”is the electrical input.
Selecting the optical input for AES disables the ADAT input, since the two devices
share the same optical connector. Similarly, selecting the optical input for ADAT
automatically switches the AES input to the coaxial digital electrical connector.
To set the output destination with Audio Panel:
1. Position the cursor over the output device panel (if the device panel you need is not
displayed, use the View menu), then hold down the right mouse button to display a
menu that contains the Output Destination option.
2.    Choose the output destination (for example, AES Out or Optical Out) that you want
the output device to use.“AES Out”is the coaxial digital electrical output.
Selecting the optical output for AES disables the ADAT output, since the two
devices share the same optical connector. Similarly, selecting the optical output for
ADAT automatically switches the AES output to the coaxial digital electrical
connector.
15

Chapter 3
3. Technical Specifications and Product Support
This chapter contains the following sections:
•“PCI Digital Audio Board Specifications”on page 18
•“DB15 Connector Pinouts”on page 19
•“Product Support”on page 20
17

Chapter 3: Technical Specifications and Product Support
PCI Digital Audio Board Specifications
Table 3-1 lists the PCI Digital Audio Board’s features and specifications.
Table 3-1 PCI Audio Board’s Specifications
Synchronization Locks to professional video (via video sync) and audio
equipment with professional jitter attenuation.
Accepts video reference input, generates AES11 grade 2
(10ppm) clocks
Input Sample Rates Continuous between 32 kHz and 48 kHz
Output Sample Rates 32 kHz, 44.1 KHz, and 48 KHz
Jitter Attenuation For 44.1 kHz or 48 kHz sample rates
Video Reference Loop Through 75 ohm BNC connectors, breakout cable
Coaxial Digital Audio Input AES-3id 75 ohm BNC connector, breakout cable
AES11 synchronization input (for audio clock rates)
AES3 professional 2-channel 24-bit digital
Compatible with IEC958, S/PDIF consumer 2-channel
digital (IEC958 is now officially designated IEC60958)
Coaxial Digital Audio Outputs AES-3id 75 ohm BNC connector, breakout cable
AES11 synchronization output
AES3 professional 2-channel 24-bit digital
Compatible with IEC958, S/PDIF consumer 2-channel
digital (IEC958 is now officially designated IEC60958)
Optical Digital Input 12.8 Mbps SHARP multimode plasticfiber optic
connector, PCI I/O panel
8-channel, 24-bit ADAT Optical interface
Compatible with IEC958, S/PDIF consumer 2-channel
digital (IEC958 is now officially designated IEC60958)
Optical Digital Output 12.8 Mbps SHARP multimode plasticfiber optic
connector, PCI I/O panel
8-channel, 24-bit ADAT optical
Compatible with IEC958, S/PDIF consumer 2-channel
digital (IEC958 is now officially designated IEC60958)
18

DB15 Connector Pinouts
DB15 Connector Pinouts
Table 3-2 shows the pinouts for the DB15 connector for video synchronization and
AES-3id digital audio through the breakout cable.
Table 3-2 DB15 Connector Pinout Assignments
Pin Assignment
1 COAXIAL DIGITAL_IN GND
2 COAXIAL DIGITAL_IN
3 VID_REF
4 VID_REF GND
5 VID_REF
6 VID_REF GND
7 NOT CONNECTED
8 CHASSIS GND
9 COAXIAL DIGITAL_OUT
10 NOT CONNECTED
11 COAXIAL DIGITAL_OUT GND
12 NOT CONNECTED
13 CHASSIS GND
14 CHASSIS GND
15 NOT CONNECTED
19

Chapter 3: Technical Specifications and Product Support
Product Support
SGI provides a comprehensive range of product support for its products. If you are in
North America and would like support for your SGI supported products, contact the
Technical Assistance Center at 1-800-800-4SGI or your authorized service provider. If you
are outside North America, contact the SGI subsidiary or authorized distributor in your
country.
20

Glossary
ADAT Optical interface
The ADAT Optical interface provides a unidirectional point-to-point connection of 8
tracks of 24-bit digital audio, with subcodes, at professional sampling rates, on a single
consumer-grade opticalfiber. A typical use is for a data connection between multiple
ADAT tape decks, for bouncing tracks between decks.
AES3
The AES3-1992 specification of transmission format for digital audio data. This
specification is also commonly referred to as AES/EBU. See also ANSI S4.4-1992 and
IEC958 (now officially designated“IEC 60958), Audio Engineering Society, Inc., Vol. 40,
No. 3, 1992, New York, NY 10165. For more information on this specification, see
http://www.aes.org/publications/
AES-3id
An information document describing implementation of AES3-1992 (see above
attribution) using 75-ohm coaxial cable with 1 V peak-to-peak signal levels. See
http://www.aes.org/publications/
PCI
Peripheral Component Interconnect—a bus specification. The PCI bus is a
high-performance local bus used to connect peripherals to memory and a
microprocessor. A wide range of vendors make devices that plug into the PCI bus.
PCI I/O blank panel
A metal panel thatfits on a slot in the I/O door. It must be in place if a PCI board is not
installed in the corresponding slot in the PCI module.
21

Glossary
S/PDIF
A serial interconnect standard for consumer digital audio equipment. This interface can
transfer stereo or single channel audio content at sample resolutions up to 20 bits (or 24
bits using an extension to the specification) and sample rates of 32 kHz, 44.1 kHz, and 48
kHz. Physical connectors are typically RCA jacks or optical connectors. For a complete
description of S/PDIF, see the IEC 958 specification (now officially designated“IEC
60958).
22

Index
A G
ADAT Optical Devices, connecting,11 GIGAchannel
See Origin200 server,6
Audio Panel
controlling ADAT and AES,13
setting I/O defaults,14
setting the I/O signal path,15 H
help,20
C
cable I
breakout,9
feature,9 installing
fiber optic,9 software,10, 13
configurations installing the board
supported,8 in O2,3
in Octane and Octane2,4
connector pinouts,19
in Onyx 3000 and Origin 3000,7
controlling ADAT and AES,13
in Origin200,6
D
L
DB15 connector pinouts,19
label
attaching,5
F
O
features,18
O2
installing in,3
23

Index
Octane and Octane2 specifications,18
installing in,4 support,20
maximum number of boards,4
system label
Onyx 3000 attaching,5
installing in,7
maximum number of boards,7
optical devices, connecting,11 T
Origin200
installing in technical specifications,18
,6
maximum number of boards,6
performance issues,6
V
Origin 3000
installing in,7
verifying the hardware installation,11
maximum number of boards,7
W
P
workstations,7
PCI audio board
connectors,1
description,2
features,18
installing in O2,3
installing in Octane and Octane2,4
installing in Origin200,6
installing in Origin 3000,7
performance
maximizing in an Origin200 server,6
pinouts, DB15 connector,19
platforms
compatibility,8
S
servers,7
setting I/O defaults,14
setting the I/O signal path,15
software, installing,10, 13
24
