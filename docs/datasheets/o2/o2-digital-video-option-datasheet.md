# o2-digital-video-option-datasheet

Datasheet
® ®
Silicon Graphics	O2	Digital Video Option
Features Professional Video I/O
• Professional digital media option The Digital Video option extends the Silicon Graphics O2 visual worksta-
for Silicon Graphics O2
tion’s baseline video, audio, and compression features to professional
• Uncompressed digital video I/O
levels by providing uncompressed digital video I/O. The O2 Digital Video
• Analog audio support
board provides two independent input and two identical output streams
of CCIR601, SMPTE 259M serial digital video with PAL or NTSC timing
at 8 or 10 bits. One output channel may be selected to provide a 4:0:0
alpha signal, enabling a real-time feed to an external downstream
switcher or keyer.
Performance for Uncompressed Video
The flexibility of the O2 Unified Memory Architecture allows video to
stream directly to memory through dedicated DMA channels, enabling
uninterrupted video and audio streaming. The built-in Ultra Fast/Wide
SCSI interface enables uncompressed video capture to an external
array of disks. If an application supports compressed video formats,
the Imaging and Compression Engine provides real-time motion-JPEG
compression and decompression on the video stream.
Synchronization
The Silicon Graphics O2 synchronization capabilities add to its strengths
as a digital media content creation platform. The Unadjusted System
Time/Media Stream Counter support synchronizes recording and play-
back of audio and video signals to within ±200 microseconds (a few
video lines) of each other or to external timecode signals such as LTC or
VITC. Applications can synchronize audio and video signals to incoming
or outgoing serial or MIDI signals to within ±1 millisecond, which is use-
ful for field-accurate RS-422 video deck control and audio-, video-, or
MIDI timecode-synchronized sequencing.
Audio
Audio support includes two independent stereo analog input and output
channels with a stereo headphone output and a mono microphone input.
The system allows programmable control of gain, attenuation, mute, and
sampling rates in 1 KHz increments between 5.5 and 48 KHz.
For professional audio applications, the optional PCI Digital Audio board
from SGI offers eight channels of 24-bit ADAT optical I/O with stereo
24-bit AES3, AES11 synchronization I/O, professional jitter attenuation,
and video composite sync loop-through to lock audio and video together.

Silicon Graphics O2DigitalVideo Option
TechnicalSpecifications
System Compatibility Genlock
O2DigitalVideo is compatiblewith all O2systems and occupiesthe •Genlocktovideo inputsignal,external housereferencesignal,
digital media optionslot orinternal reference
InputFormatand Connectors GPI Input/Output
| •Two independent8 or10 bitserial CCIR601/SMPTE259Msignals | •Input | Activelow,opticallyisolated |
| --- | --- | --- |
| (75ohm BNCterminated,unbalanced) | •Output | Open collector,opticallyisolated, activelow OutputFormatand Connectors •Two identical8 or10 bitserial CCIR601/SMPTE259Msignals AudioSpecifications (75ohm BNCterminated,unbalanced) •One16-bitstereo analog line-level output(stereo3.5mm) •One16-bitstereo analog line-level input(stereo3.5mm) O2DigitalVideo Formats •Mono microphoneinput(3.5mm) •Stereo headphoneoutput(3.5mm) |
| Format | Resolution | Numberof Numberof Timing •Supports allsampleratesfrom5.5KHzto48 KHzwith1Hzresolution |
| Inputs | Outputs | •Audiosamplerates can beslavedtovideo inputs oroutputs at32,44.1, 44.056,or48 KHzsamplerates |
| 4:2:2 | 8 or10 bits | 2 2 PAL/NTSC •Sample-accuratetiming informationforprecisesynchronization |
| 4:2:2:4 | 8 or10 bits | 1 1 PAL/NTSC RegulatoryRequirements •FCCClassA 525/60 NTSC13.5MHzpixel rateand625/50 PAL13.5MHzpixel rate Marketing Code •VIDEO-601-O2 InputCharacteristics •Return loss 15dB @270 MHz OtherO2Digital Media Products •Cablelength 300 mforinput •SGI™DigitalAudio option •Silicon Graphics O2DVLinkoption OutputCharacteristics •Amplitude 800 mV+/-10% •Riseandfalltime 400to1,500 ps •Comparison rise Within500 ps andfalltime •Overshootriseandfall No morethan10% of amplitude,77mV •Clockjitter Lessthan20% ofUI,741ps •Unitinterval 3.704ns •Cablelength 300 mforoutput Corporate Office NorthAmerica1(800)800-7441 1600AmphitheatrePkwy. LatinAmerica1(650)933-4637 MountainView,CA94043 Europe(44)118.925.75.00 (650)960-1980 Japan (81)3.5488.1811 www.sgi.com Asia Pacific(65)771.0290 1999Silicon Graphics,Inc.All rights reserved.Specificationsubjectto changewithoutnotice.Silicon Graphics and O2areregisteredtrademarks,andSGI andtheSGI logo aretrademarks,of © Silicon Graphics,Inc.All othertrademarks mentioned herein arethepropertyoftheirrespectiveowners. 1570 (08/99) J10468 |
