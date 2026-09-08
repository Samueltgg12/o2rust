# o2-digital-video-option-installation-guide

™
O2 Digital Video Option
Installation Guide
Document Number 007-3616-001

CONTRIBUTORS
Written by Judy Muchowski
Illustrated by Dany Galgani
Edited by Christina Cary
Production by Max Anderson
St. Peter’s Basilica image courtesy of ENEL SpA and InfoByte SpA. Disk Thrower
image courtesy of Xavier Berenguer, Animatica
© 1997, Silicon Graphics, Inc.— All Rights Reserved
The contents of this document may not be copied or duplicated in any form, in whole
or in part, without the prior written permission of Silicon Graphics, Inc.
RESTRICTED RIGHTS LEGEND
Use, duplication, or disclosure of the technical data contained in this document by
the Government is subject to restrictions as set forth in subdivision (c) (1) (ii) of the
Rights in Technical Data and Computer Software clause at DFARS 52.227-7013
and/or in similar or successor clauses in the FAR, or in the DOD or NASA FAR
Supplement. Unpublished rights reserved under the Copyright Laws of the United
States. Contractor/manufacturer is Silicon Graphics, Inc., 2011 N. Shoreline Blvd.,
Mountain View, CA 94043-1389.
Silicon Graphics and the Silicon Graphics logo are registered trademarks, and O2 and
O2Cam are trademarks of Silicon Graphics, Inc.
Adobe Premiere is a trademark of Adobe Systems Incorporated. UNIX is a registered
trademark in the United States and other countries, licensed exclusively through
X/Open Company, Ltd.
O2™ Digital Video Option Installation Guide
Document Number 007-3616-001

Contents
List of Figuresv
List of Tablesvii
Introductionix
1. Installing the O2 Digital Video Option Module1
O2 Digital Video Option Package    2
Installing the Software    2
About the O2 Digital Video Option Module    3
About the Breakout Cable    4
Installing the O2 Digital Video Option Module    5
Verifying the Hardware Installation    12
Troubleshooting the Installation    12
Connecting an SDI Video Source to the Workstation    12
Placing a System Upgrade Regulatory Label    14
Exchanging Modules    14
2. Using the Digital Media Software19
Locating Additional Software Information    20
Adjusting Video Parameters    20
Pro Menu Settings    22
Locking Audio With Video    22
Synchronizing to House Sync    24
Using Video Commands From a UNIX Shell    24
Switching Between Analog Video and Digital Video    25
iii

Contents
3. Technical Specifications, Product Support, and Regulatory Information27
Technical Specifications    27
Manufacturer’s Regulatory Declarations    30
Class A    30
Electromagnetic Emissions    31
Radio and Television Interference    32
Shielded Cables    33
Electrostatic Discharge    33
Product Support    33
Index35
iv

List of Figures
Figure 1-1 The O2 Digital Video Option Module    3
Figure 1-2 Digital Video Breakout Cable    4
Figure 1-3 Turning Off the Workstation    5
Figure 1-4 Removing the Cables    6
Figure 1-5 Removing the Analog Audio-Visual Module    7
Figure 1-6 Installing the O2 Digital Video Module    8
Figure 1-7 Locking the Digital Video Option Module in Place    9
Figure 1-8 Reconnecting the Power Cable    10
Figure 1-9 Turning On the Workstation    11
Figure 1-10 Connecting an SDI Video Source    13
Figure 1-11 Disconnecting the Cables    15
Figure 1-12 Removing the Digital Video Option Module    16
Figure 1-13 Installing the Analog Audio-Visual Module    17
Figure 1-14 Locking the Analog Audio-Visual Module in Place    18
Figure 2-1 Video Control Panel For the O2 Digital Video Option    21
Figure 2-2 Locking Digital Audio With Digital Video    23
Figure 3-1 Japan’s VCCI Class A Statement    32
v

List of Tables
Table 2-1 Output Enable Control Panel Settings    22
Table 3-1 DB9 Connector Pinout Assignments    27
Table 3-2 Serial CCIR601/SMPTE 259M Output Performance    28
Table 3-3 Serial CCIR601/SMPTE 259M Input Performance    28
Table 3-4 Genlock Performance    29
Table 3-5 GPI Input/Output    29
Table 3-6 Class A FCC Declaration of Conformance    30
vii

Introduction
This guide describes how to install the O2 digital video option module in the O2
workstation.
The following topics are covered in this manual:
• Chapter 1,“Installing the O2 Digital Video Option Module”on page 1, explains
how to remove the existing analog audio or audio-visual module and replace it with
the digital video option module. It also explains where tofind software installation
information.
• Chapter 2,“Using the Digital Media Software”on page 19, provides information on
how to get started with the digital media software.
• Chapter 3,“Technical Specifications, Product Support, and Regulatory
Information”on page 27, contains cable pinout assignments, technical
specifications for the O2 digital video option module, and regulatory statements. It
also tells you how to contact Silicon Graphics for product support information.
ix

Chapter 1
1. Installing the O2 Digital Video Option Module
This chapter explains how to install the O2 digital video option module in an O2
workstation. It also describes how to verify the hardware installation.
The O2 digital video option is a serial digital interface board that provides four 10-bit,
serial, CCIR601/SMPTE 259M channels (two in and two out). A breakout cable provides
a genlock connection for locking to analog house sync, house sync loopthrough, and GPI
(general purpose interrupt) input and output. The digital video option module also
provides an analog audio interface similar to the base O2 workstation. Digital audio is
provided by the Silicon Graphics PCI digital audio option board, a separate option. For
more information, see your service provider.
The O2 digital video option module does not support analog video or the O2Cam digital
video camera.
The following topics are covered in this chapter:
•“O2 Digital Video Option Package”on page 2
•“Installing the Software”on page 2
•“About the O2 Digital Video Option Module”on page 3
•“Installing the O2 Digital Video Option Module”on page 5
•“Verifying the Hardware Installation”on page 12
•“Connecting an SDI Video Source to the Workstation”on page 12
•“Placing a System Upgrade Regulatory Label”on page 14
•“Exchanging Modules”on page 14
1

Chapter 1: Installing the O2 Digital Video Option Module
O2 Digital Video Option Package
Your package includes all of the following items. If anything is missing, contact your
sales representative.
• O2 digital video option module
• breakout cable
• O2 Digital Video Option Module Installation Guide(this manual)
• compact disc (CD-ROM)
Installing the Software
Depending on which version of IRIX you are running, you may need to install software
for the digital video option module. Read the release notes on the CD for information on
which software you need to install.
The booklet that comes with the CD explains how to read release notes and how to install
software.
2

About the O2 Digital Video Option Module
About the O2 Digital Video Option Module
Mic in
DD11
ININ11
Stereo In
DD11 Headphones
ININ22
Stereo out
DD11
OOUUTT
Analog Genlock in (red)
DD11 GPI in (green)
OOUUTT D1
1
IN	Primary 601 in	Analog Genlock out (loopthrough)(blue)
GPI out (black)
D1
2 Secondary 601 in
IN
D1
OUT Primary 601 out
D1
OUT Secondary 601 out
Figure 1-1 The O2 Digital Video Option Module
Note: To identify the ports on the side of the O2 digital video option module, use the
icons on the module instead of those on the outer cover of the O2 workstation.
The O2 digital video option provides
• two serial, CCIR601/SMPTE 259M input channels via standard BNC connectors on
the rear panel
• two serial, CCIR601/SMPTE 259M output channels via standard BNC connectors
on the rear panel
• genlock input via the red connector on the breakout cable
• genlock output (loopthrough) via the blue connector on the breakout cable
3

Chapter 1: Installing the O2 Digital Video Option Module
• general purpose interrupt (GPI) input and output connectors via the breakout cable
• microphone in, stereo in, headphone out, and stereo out via four 3.5 mm connectors
on the side panel
About the Breakout Cable
GENLOCK in (red)
GPI in (green)
GENLOCK out, loopthrough (blue)
GPI out (black)
Figure 1-2 Digital Video Breakout Cable
The breakout cable provides a genlock circuit for locking to analog house sync. The sync
input and output is an analog composite video signal that is used as a timing reference
for the digital video outputs of the board. The board extracts the timing from the analog
video input and apply it to the digital video outputs.
The analog genlock input can be a composite video signal or a composite video
blackburst signal. The loopthrough connection allows you to daisy-chain the house sync
signal to other video devices that can be genlocked.
4

Installing the O2 Digital Video Option Module
Note: The house sync signal must be terminated with a 75 ohm BNC terminator. If you
are using a video input only and not looping through to another device, you need to
terminate the unused video out connector with a 75 ohm BNC terminator.
The GPI in and out are for synchronizing the O2 workstation with external devices.
Installing the O2 Digital Video Option Module
To install the O2 digital video option module, follow the steps below.
raphics
SiliconG
Figure 1-3 Turning Off the Workstation
1.    Turn off the workstation.
5

Chapter 1: Installing the O2 Digital Video Option Module
Figure 1-4 Removing the Cables
2.    Remove the power cable from the rear of the chassis, and any other cables from the
side of the existing analog audio or audio-visual module. The audio-visual module
is on the extreme right as you face the rear of the workstation. You don’t have to
remove the cables connected to the system module on the left.
6

Installing the O2 Digital Video Option Module
Figure 1-5 Removing the Analog Audio-Visual Module
3.    Remove the existing analog audio or audio-visual module by pulling down the
lever and sliding it out of the chassis.
7

Chapter 1: Installing the O2 Digital Video Option Module
DIDINN11
11
DIDINN11
22
DODO11
UUTT
DODO11
UUTT
Figure 1-6 Installing the O2 Digital Video Module
4.    Slide the new digital video option module into the opening on the right of the
chassis.
8

Installing the O2 Digital Video Option Module
DINDIN11
11
DINDIN11
22
DODO11
UUTT
DODO11
UUTT
Figure 1-7 Locking the Digital Video Option Module in Place
5.    Push the lever all the way up to lock the module in place.
9

Chapter 1: Installing the O2 Digital Video Option Module
DINDIN11
11
DINDIN11
22
DODO11
UUTT
DODO11
UUTT
Figure 1-8 Reconnecting the Power Cable
6.    Reconnect the power cable.
7.    Connect the digital video breakout cable and any other cables.
10

Installing the O2 Digital Video Option Module
raphics
SiliconG
Figure 1-9 Turning On the Workstation
8.    Turn on the workstation.
11

Chapter 1: Installing the O2 Digital Video Option Module
Verifying the Hardware Installation
Once you have installed the software and the digital video option module, verify that the
system recognizes the new hardware:
Open a UNIX shell. At the prompt type
hinv
Look for a line similar to this:
Note:You only see this listing if you have the video software installed. Read the release
notes on the CD for information about which software to install.
Troubleshooting the Installation
If you do not see the listing for the digital video option module:
1. Check that you have installed the correct software from the CD. Read the release
notes on the CD for information about which software to install. The booklet that
comes with the CD describes how to read release notes.
2.    If the correct software is installed, turn off the workstation, remove the power cable,
and go over each step of the hardware installation instructions again. Make sure the
digital video option module is pushed back all the way into the chassis and that the
lever is up. The module should beflush with the other modules.
Connecting an SDI Video Source to the Workstation
Follow these steps and refer to Figure 1-10 to connect the digital video source to the O2
workstation.
To record video input, you need a few additional items that are not included in your O2
Digital Video Option package.
• A professional-quality SDI (serial digital interface) video source.
• One or two cables with BNC connectors on each end.
12

Connecting an SDI Video Source to the Workstation
• One or two analog audio cables with 3.5 mm stereo jack connectors on each
end.
Note: Digital audio is provided by the Silicon Graphics PCI digital audio option board,
a separate option. For more information, see your service provider. Or go to the following
location on the Web:
http://www.sgi.com/Products/hardware/desktop/products/digital.html
To stereo in
To primary
601 In
DINDIN1111
DINDIN1122
3.5mm stereo
DODO1U1UTT
jack cable
DODO1U1UTT
To audio out
on SDI video
source
To video out
BNC to BNC cable
on SDI video source
Figure 1-10 Connecting an SDI Video Source
13

Chapter 1: Installing the O2 Digital Video Option Module
1. Attach one end of the BNC cable to the D1 In 601 (primary) or D1 In 601 (secondary)
connector on the rear of the digital video option module.
2.    Attach the other end of the BNC cable to the Video Out connector on the SDI video
source.
3.    Attach one end of the 3.5 stereo jack cable to the Stereo In connector on the side
panel of the digital video option module.
4.    Attach the other end of the 3.5 mm stereo jack cable to the analog Audio Out
connector on the SDI video source.
Placing a System Upgrade Regulatory Label
If you received a system upgrade label, tilt the O2 carefully and place it underneath the
chassis. Do not cover the original label. For regulatory statements, see“Manufacturer’s
Regulatory Declarations”on page 30.
Exchanging Modules
The O2 digital video option module and the analog audio-visual module are
interchangeable. The system recognizes either module, however, you may need tp
remove or rename your ~/.videopanelrcfile. See“Switching Between Analog Video and
Digital Video”on page 25. To remove and replace the O2 digital video option module,
follow these steps:
14

Exchanging Modules
DIDINN1111
DIDINN1122
DODO1U1UTT
DODO1U1UTT
Figure 1-11 Disconnecting the Cables
1. Disconnect the power cable from the rear of the O2 workstation, and disconnect any
cables attached to the side or rear of the digital video option module.
15

Chapter 1: Installing the O2 Digital Video Option Module
DINDIN11
11
DINDIN11
22
DODO11
UUTT
DODO11
UUTT
Figure 1-12 Removing the Digital Video Option Module
2.    Pull down the lever on the digital video module and slide the module out of the
chassis.
16

Exchanging Modules
Figure 1-13 Installing the Analog Audio-Visual Module
3.    Install the analog audio-visual module by sliding it into the chassis.
17

Chapter 1: Installing the O2 Digital Video Option Module
Figure 1-14 Locking the Analog Audio-Visual Module in Place
4.    Push the lever all the way up to lock the module in place.
5.    Reconnect any exterior cables.
6.    Turn on the workstation.
18

Chapter 2
2. Using the Digital Media Software
This chapter provides information on how to adjust Video Panel settings. For additional
information, use the Help menus on the Video Panel and digital media tools, or the O2
video man pages. If you are creating high quality digital movies, you might consider
using a third party application such as Adobe Premiere.
If you haven’t already installed the O2 digital video option software and hardware, refer
to Chapter 1,“Installing the O2 Digital Video Option Module.”
The following topics are covered in this chapter:
•“Locating Additional Software Information”on page 20
•“Adjusting Video Parameters”on page 20
•“Locking Audio With Video”on page 22
•“Synchronizing to House Sync”on page 24
•“Using Video Commands From a UNIX Shell”on page 24
•“Switching Between Analog Video and Digital Video”on page 25
19

Chapter 2: Using the Digital Media Software
Locating Additional Software Information
The instructions in this section are based on current operating system software. If you
have upgraded to a subsequent operating system, some of the instructions may no longer
be valid. If this is the case, check the latest information by using the online
documentation and the Help menus on the Video Panel and digital media tools.
• For more technical information on the O2 digital video option, refer to the O2 Video
man pages. In a shell window, enter
man 7 O2Video
• For the latest information, read the release notes on the CD.
• For detailed information on using the digital media tools such as MediaRecorder,
MediaConvert, and MovieMaker, see the onlineDigital Media Tools Guide. (From the
Toolchest choose Help > Online Books > SGI_End User > Digital Media Tools
Guide.)
• Use the online Help for digital media applications such as Media Recorder,
MediaConvert, MovieMaker, and so on.
• Browse the Silicon Graphics Technical Publications Library. Enter the following in
your Web Browser location window:
http://techpubs.sgi.com/library/
Adjusting Video Parameters
You can use the Video Panel to adjust video parameters such as input and output timing,
default input source, and output genlock source and content.
There are several ways to start the Video Panel:
• From the Toolchest, choose Find > Control Panels and double-click the videopanel
icon.
• Typevideopanelin a shell window.
•	From the Options menu of MediaRecorder, choose	.
The video panel for the O2 digital video option is shown in Figure 2-1.
20

Adjusting Video Parameters
Figure 2-1 Video Control Panel For the O2 Digital Video Option
For details on adjusting settings in the Video Panel for the O2 digital video option, use
the O2 Video man pages. In a shell window, type
man 7 O2Video
Note: There is an incorrect reference in the current O2Video man pages which will be
corrected in future releases. In the section on Output Enable, when it refers to
, this corresponds to the Primary Video Channel (channel 1). When it refers to
,  this corresponds to the Secondary Video Channel (channel 2).
21

Chapter 2: Using the Digital Media Software
Pro Menu Settings
Table 2-1 explains the settings in the Output Enable control panel of the Video
Output-Signal window in the Pro menu.
Note:Since the input and output timings and genlocks require an active input or output,
it’s best to adjust the controls while an application performing video input or output is
| running. The easiest way to do this is to use | or | in the Utilities menu of the Video Panel. Table 2-1 Output Enable Control Panel Settings |
| --- | --- | --- |
| Output Enable Settings | Channel 1 Primary 601 | Channel 2 Secondary 601 Output Output |
| Pixels/Pixels | 4:2:2 YCrCb | 4:2:2 YCrCb |
| Pixels/Alpha | 4:2:2 YCrCb | 4:0:0 A |
| Alpha/Pixels | 4:0:0 A | 4:2:2 YCrCb |
| Alpha/Alpha | 4:0:0 A | 4:0:0 A |
| Passthru/Passthru | Sends input from input source to both outputs source to both outputs without traversing without traversing memory. memory. Locking Audio With Video To lock the PCI digital audio option board with the digital video option board, you must apply house sync to both boards, as shown in Figure 2-2. For information on using analog audio with the digital video option board, see the online Digital Media Tools Guide. (From the Toolchest choose Help > Online Books > SGI_End User > Digital Media Tools Guide.) Or, use the online help for the Audio Panel and digital media applications. | Sends input from input |
22

Locking Audio With Video
To DB9 connector
on digital video option
Digital video
option cable
External
Genlock in (red) genlock
source
GPI in (green)
Genlock out (blue)
GPI out (black)
To DB15 connector
75 ohm coaxial
on PCI digital
cable
audio option
Digital audio
option cable
Coaxial digital in (red)
Video ref loop (green)
Coaxial digital out (blue)
75 ohm
Video ref loop (black) BNC
terminator
Figure 2-2 Locking Digital Audio With Digital Video
23

Chapter 2: Using the Digital Media Software
Synchronizing to House Sync
House sync is a timing signal generated to synchronize video equipment in a studio
environment. The O2 Digital Video Option can be synchronized to a house sync source
as follows.
1. Connect the house sync signal to the red Genlock In connector on the breakout
cable.
2.    In a shell window, entervideopanelto open the Video Panel.
3.    From the Pro menu, choose Video Output > Signal Controls.
4.    For Output Sync choose Genlock.
5.    For Genlock Source choose External.
Note: With current software, genlock can only be selected for Output Sync if an
application performing video output from memory is running. (This will be corrected in
future releases.)
For more information on adjusting parameters in the Video Panel, refer to its online Help,
or to the O2 video man pages. (Typeman 7 O2Video in a shell window.)
Using Video Commands From a UNIX Shell
From a UNIX shell, you can use the video tools listed below. Each of these tools has man
pages.
videoin Displays video input in a window on the graphics screen
videoout Sends graphics from a portion of the screen or the full screen to a video
output port
vintovout Receives video input and sends it to a device attached to the output port
vidtomem Saves single frames from video input to memory
memtovid Sends single frames from memory to an output port
Use the tools with the-vflag and a number to select inputs. To obtain the number, enter
vlinfoin a shell window. For example, to display input from Primary 601 In and output
it to digital, use the command
vintovout -v 1
24

Switching Between Analog Video and Digital Video
The default input is selected in the Device Controls Default Input panel in the Video
Panel. All outputs are active on the O2 digital video option.
Switching Between Analog Video and Digital Video
The O2 digital video option module and the analog audio-visual module are
interchangeable. The system recognizes either module, however, you should remove or
rename your ~/.videopanelrc settingsfile. (In future operating system releases, you
won’t have to do this.) If you plan to make this switch frequently, you may want to save
the above commands in a scriptfile to save time.
Note: The settingsfile is saved in your $HOME directory (denoted by the~ in the
examples below). You should make the change in all home directories for any user
(including root) who starts up the Video Panel.
Follow these steps:
1. If the analog audio-visual module is currently installed and you saved the video
panel settings, rename the ~/ .videopanelrcfile as follows:
In a shell window, enter
mv ~/.videopanelrc ~/.videopanelrc.analog
Or, if the digital video module is currently installed and you saved the video panel
settings, rename the ~/ .videopanelrcfile as follows:
mv ~/.videopanelrc ~/.videopanelrc.digital
2.    To switch between the two modules, link to the appropriate ~/.videopanelrc as
follows.
• To switch to the digital video module, enter
rm -f ~/.videopanelrc
ln -s ~/.videopanelrc.digital ~/.videopanelrc
• To switch to the analog video module, enter
rm -f ~/.videopanelrc
ln -s ~/.videopanelrc.analog ~/.videopanelrc
25

Chapter 3
3. Technical Specifications, Product Support, and
Regulatory Information
This chapter contains the following topics:
•“Technical Specifications”on page 27
•“Manufacturer’s Regulatory Declarations”on page 30
•“Product Support”on page 33
Technical Specifications
Table 3-1 shows the pinout assignments for the DB9 Connector on the O2 digital video
option board. Table 3-2 through Table 3-5 provide Serial CCIR601/SMPTE 259M,
Genlock, and GPI specifications.
Table 3-1 DB9 Connector Pinout Assignments
Pin Assignment
1 Analog Video Genlock In
2 Analog Video Genlock Gnd
3 GPI In
4 GPI In Gnd
5NC
6 Analog Video Genlock loopthrough
7 Analog Video Genlock Gnd
8 GPI Out
9 GPI Out Gnd
27

Chapter 3: Technical Specifications, Product Support, and Regulatory Information
Table 3-2 Serial CCIR601/SMPTE 259M Output Performance
Parameter Specification
Signal amplitude 800 mV +/- 10%
Rise time 400 to 1500 ps
Fall time 400 to 1500 ps
Comparison rise and fall time Within 500 ps
Overshoot rise No more than 10% of amplitude, 77mV
Overshoot fall No more than 10% of amplitude, 77mV
Jitter Less than 20% of UI, 741 ps
Unit interval 3.704 ns
Cable length on transmit 300 meters for output
Output configuration Two 8- or 10-bit serial CCIR601/SMPTE 259M
digital video channels, one 4:2:2 program, and
one 4:0:0 alpha. You can configure the channel
type. The same signal can be mirrored on both
outputs. For example, you can assign the 4:2:2
source to both outputs simultaneously. Or you
can assign the 4:0:0 source to both outputs
simultaneously.
Table 3-3 Serial CCIR601/SMPTE 259M Input Performance
Parameter Specification
Signal amplitude 800 mV +/- 10%
Rise Time 400 to 1500 ps
Fall time 400 to 1500 ps
Comparison rise and fall time Within 500 ps
Overshoot rise No more than 10% of amplitude, 77mV
28

Technical Specifications
Table 3-3  (continued) Serial CCIR601/SMPTE 259M Input Performance
Parameter Specification
Overshoot fall No more than 10% of amplitude, 77mV
Unit interval 3.704 ns
Cable length on transmit 300 meters
Input configuration Two independent 10-bit serial CCIR601/SMPTE
259M digital video inputs
Table 3-4 Genlock Performance
Parameter Specification
Input signal NTSC/PAL composite video, black-burst, or
composite sync signal, or either of the two serial
digital video inputs.
Jitter performance Less than 741 ps on the serial digital video
output stream when genlocked to the analog
video source. The jitter is somewhat less when
genlocking to either of the two digital video
inputs.
Table 3-5 GPI Input/Output
Parameter Specification
Output Open collector, optically isolated, active low.
Input Active low, optically isolated.
29

Chapter 3: Technical Specifications, Product Support, and Regulatory Information
Manufacturer’s Regulatory Declarations
Caution:This device has several governmental and third-party approvals, licenses, and
permits. Do not modify this product in any way that is not expressly approved by Silicon
Graphics. If you do, you may lose these approvals and your governmental agency
authority to operate this device.
The device conforms to several national and international specifications and European
Directives listed on the Manufacturer’s Declaration of Conformity. The CE insignia
displayed on each device is an indication of conformity to the European requirements.
Class A
Table 3-6 Class A FCC Declaration of Conformance
Trade Name Silicon Graphics, Inc.
Product O2 Digital Video Option Module
Model Number CMNB014A
Date of Conformance September 1997
Responsible Party Silicon Graphics, Inc.
Address 2011 North Shoreline Boulevard,
Mountain View, California 94043-1389
Telephone (650) 933-1071
This equipment complies with Part 15 of the FCC Rules. Operation is subject to the
following two conditions: (1) This device may not cause harmful interference, and (2) this
device must accept any interference received, including interference that may cause
undesired operation.
This equipment has been tested and found to comply with the limits for a Class A digital
device, pursuant to Part 15 of the FCC Rules. These limits are designed to provide
reasonable protection against harmful interference in a residential installation. This
equipment generates, uses and can radiate radio frequency energy and, if not installed
and used in accordance with the instructions, may cause harmful interference to radio
communications. However, there is no guarantee that interference will not occur in a
30

Class A
particular installation. If this equipment does cause harmful interference to radio or
television reception, which can be determined by turning the equipment off and on, the
user is encouraged to try to correct the interference by one or more of the following
measures:
• Reorient or relocate the receiving antenna.
• Increase the separation between the equipment and receiver.
• Connect the equipment into an outlet on a circuit different from that to which the
receiver is connected.
• Consult the dealer or an experienced radio/TV technician for help.
Caution: The user is cautioned that changes or modifications to the equipment not
expressly approved by the party responsible for compliance could void the user’s
authority to operate the equipment.
Electromagnetic Emissions
The following information applies to the system base configuration. Refer to the
Manufacturer’s Declaration of Conformity for your system’s specific classification.
This device complies with the Class A limits of Part 15 of the FCC Rules. Operation is
subject to the following two conditions:
• This device may not cause harmful interference.
• This device must accept any interference received, including interference that may
cause undesired operation.
This Class A digital apparatus meets all requirements of the Canadian
Interference-Causing Equipment Regulations.
Cet appareil numérique de la classe B respecte toutes les exigences du Règlement sur le
matériel brouilleur du Canada.
This device complies with Class A electromagnetic emissions limits of C.I.S.P.R.
Publication 22, Limits and Methods of Measurement of Radio Interference
Characteristics of Information Technology Equipment; Class A limits for Information
Technology Equipment; and Japan’s VCCI Class A limits.
31

Chapter 3: Technical Specifications, Product Support, and Regulatory Information
Figure 3-1 Japan’s VCCI Class A Statement
Radio and Television Interference
The equipment described in this guide generates and uses radio frequency energy. If it is
not installed and used in accordance with the instructions in this guide, it can cause radio
and television interference.
This equipment has been tested and complies with the limits for a Class A computing
device in accordance with the specifications in Part 15 of FCC rules. These specifications
are designed to provide reasonable protection against such interference in an industrial
or office installation. However, there is no guarantee that the interference will not occur
in a particular installation. This system is not certified for home use.
You can determine whether your system is causing interference by turning it off. If the
interference stops, it was probably caused by the workstation or one of the peripherals.
To tell if the interference is caused by one of the peripherals, try disconnecting one
peripheral at a time to see if the interference stops. If it does, that peripheral is the cause
of the interference.
If your workstation does cause interference to radio or television reception, try to correct
the interference by using one or more of the following suggestions:
• Turn the television or radio antenna until the interference stops.
• Move the workstation to one side or the other of the radio or television.
• Move the workstation farther away from the radio or television.
• Plug the workstation into an outlet that is on a different circuit from the radio or
television. (That is, make certain the workstation and the radio or television are on
circuits controlled by different circuit breakers or fuses.)
• For additional information, check the FCC web site at
http://www.fcc.gov/Bureaus/Compliance/WWW/tribook.html
32

Class A
Shielded Cables
The device is FCC-compliant under test conditions that include the use of shielded cables
between the workstation and its peripherals. Your workstation and any peripherals you
purchase from Silicon Graphics have shielded cables. Shielded cables reduce the
possibility of interference with radio, television, and other devices. If you use any cables
that are not from Silicon Graphics, make sure they are shielded. Telephone cables do not
need to be shielded.
Electrostatic Discharge
Silicon Graphics designs and tests its products to be immune to the effects of electrostatic
discharge (ESD). ESD is a source of electromagnetic interference and can cause problems
ranging from data errors and lockups to permanent component damage.
It is important that while you are operating your workstation you keep all the covers and
doors, including the plastics, in place. The shielded cables that came with the
workstation and its peripherals should be installed correctly, with all thumbscrews
fastened securely.
An ESD wrist strap is included with some products, such as memory and graphics
upgrades. The wrist strap is used when installing these upgrades to prevent theflow of
static electricity, and it should protect your system from ESD damage.
Product Support
Silicon Graphics provides a comprehensive range of product support for its products. If
you are in North America and would like support for your Silicon Graphics supported
products, contact the Technical Assistance Center at 1-800-800-4SGI or your authorized
service provider. If you are outside North America, contact the Silicon Graphics
subsidiary or authorized distributor in your country.
33

Index
A H
audio help,12, 33
locating digital,13 house sync, synchronizing,24
locking with video,22
I
C
identifying ports,3
cable connectors
input and output timings,22
described,4
installing
cables,using shielded,33
digital video board,5
conformance declaration,30 problems,12
connecting SDI video source,12 software,2
control panels, Pro Menu,22 interference to radios and televisions,32
D L
declaration of conformance,30 label, placement,14
digital audio, locating,13 locating
digital video board additional software information,20
described,1 software installation information,2
installing,5 locking audio with video,22
synchronizing to house sync,24
digital video source, connecting,12
M
E man pages for O2 video,21
manufacturer’s regulatory declaration,30
exchanging modules,14 modules, exchanging,14
35

Index
O T
option board, installing,5 Table,27
output and input timings,22 television interference,32
output enable settings,21 timings, input and output,22
troubleshooting,12
P
U
PCI digital audio, locating,13
ports, identified,3 UNIX video commands,24
problems,12, 33
product support,33
V
Pro Menu control panels,22
verifying the hardware installation,12
video
R
adjusting settings,20
locking with audio,22
radio interference,32
man pages,21
reference pages for O2 video,21
switching analog and digital,25
regulatory
tools from a UNIX shell,24
information,30
video board
label placement,14
Seedigital video board
video panel
adjusting settings,20
S
SDI video source, connecting,12
W
shell commands for video tools,24
shielded cables,33
workstations
software radio and television interference caused by,32
installing,2
locating additional information,20
support for the product,33
switching analog and digital video,25
synchronizing house sync,24
36

Tell Us About This Manual
As a user of Silicon Graphics products, you can help us to better understand your needs
and to improve the quality of our documentation.
Any information that you provide will be useful. Here is a list of suggested topics:
• General impression of the document
• Omission of material that you expected tofind
• Technical errors
• Relevance of the material to the job you had to do
• Quality of the printing and binding
Please send the title and part number of the document with your comments. The part
number for this document is 007-3616-001.
Thank you!
Three Ways to Reach Us
• To send your comments byelectronic mail, use either of these addresses:
– On the Internet: techpubs@sgi.com
– For UUCP mail (through any backbone site):[your_site]!sgi!techpubs
• Tofaxyour comments (or annotated copies of manual pages), use this
fax number: 650-965-0964
• To send your comments bytraditional mail, use this address:
Technical Publications
Silicon Graphics, Inc.
2011 North Shoreline Boulevard, M/S 535
Mountain View, California  94043-1389
