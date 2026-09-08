# AD1843JST

Serial-Port 16-Bit
a
SoundComm Codec
AD1843
FEATURES GENERAL PRODUCT DESCRIPTION
Single Chip Integrated Speech, Audio, Fax and Modem The AD1843 SoundComm™ Codec is a complete analog front
Codec end for high performance DSP-based telephony and audio ap-
Highly Configurable Stereo∑∆ADCs and Quad∑∆DACs plications.  The device integrates the real-world analog I/O re-
Supports V.34, V.32bis, and Fallback Modem Standards quirements for many popular functions thereby reducing size,
As Well As Voice Over Data power consumption, and system complexity. The AD1843
Dual Digital Resamplers with Programmable Input and SoundComm is the world’s first codec which can support four
Output Phase and Frequency different sample rates simultaneously, without any beat fre-
Three On-Chip Phase Lock Loops for Synchronization to quency noise issues. This is essential for highly integrated audio/
External Signals, Including Video modem/fax products since the sample rates associated with au-
Thirteen Analog Inputs and Seven Analog Outputs dio are very much distinct from the sample rates associated with
Advanced Analog and Digital Signal Mixing and Digital- telephony-oriented data communication. It is also the first codec
to-Digital Sample Rate Conversion to offer on-chip digital phase lock loops for sample rate synchro-
Programmable Gain, Attenuation and Mute nization to external clock signals. This sample rate flexibility is
On-Chip Signal Filters enabled through Analog Devices’ Continuous Time Oversampling
Digital Interpolation and Decimation (CTO) technology.
Analog Output Low Pass
The main elements of the AD1843 are its extensive input and mix-
1 Hz Resolution Programmable Sample Rates from 4 kHz
ing section, its two channels of sigma-delta (∑∆) analog-to-digital
to 54 kHz Derived from a Single Clock Input
conversion, its four channels of∑∆digital-to-analog conversion, its
80-Lead PQFP and 100-Lead TQFP Packages
digital filters, and the clock and control circuitry for implementing
Operation from +5 V or Mixed +5 V/+3 V Supplies
the device’s different modes. The AD1843 permits flexible sample-
FIFO-Buffered Serial Digital Interface Compatible with
rate selection through programming and external synchronization,
ADSP-21xx Fixed-Point DSPs
many input and output options, and many mixing options.
Advanced Power Management
(continued on page 11)
VHDL Model of Serial Port Available; Evaluation Board
SoundComm is a trademark of Analog Devices, Inc.
and MAFE Board Available
SIMPLIFIED FUNCTIONAL BLOCK DIAGRAM
SYNC    XTAL   CONV BIT
3	2	3	3
4
LIN
AD1843	CLOCK GENERATION	CLKOUT
20 dB
S
2 CONTROL
MIC E
| 2 | L | REGISTERS |
| --- | --- | --- |
| AUX1 | ∑∆ 2 E PGA E | S |
| AUX2 | C | ADC 2 L AUX3 T |
| 1 | O | S E μ/A SCLK MIN ADC |
| R | E | C LAW L T D SDFS E O |
I
C	R	SDI
ATTN G
MUTE T
I
GAM GAM GAM GAM GAM	O	SDO
MOUT T
R MUTE
A
MUTE
BM
L
M
2	∑∆	U	μ/A
∑ CS
LOUT1	∑	∑	∑	∑	∑	GAM	∑	T	ATTN	FIFO	DAC1
DAC LAW
E
| LEFT AND | I | TSO |
| --- | --- | --- |
| DRIVER | RIGHT CHANNELS T TSI | MUTE N |
| MUTE | GAM | ATTN E 2 |
| MUTE | R | XCTL [1:0] GAM = GAIN |
F
HPOUTL	ATTENUATION	PDMNFT
A
| MUTE | MUTE HPOUTC C | MUTE |
| --- | --- | --- |
| LEFT AND | E | RESET HPOUTR M |
| 4 | RIGHT CHANNELS | ∑∆ |
| ∑ | U | μ/A |
| LOUT2 | GAM | ATTN ∑ FIFO DAC2 |
| DAC | T | LAW PWRDWN SUM E 2 VOLTAGE REFERENCE |
| 4 | 3 | 89 |
| V | GNDA | VCC GNDD VDD |
| REV. 0 | REF | CMOUT AAFILTL   AAFILTR FILTL FILTR Information furnished by Analog Devices is believed to be accurate and © Analog Devices, Inc., 1996 reliable. However, no responsibility is assumed by Analog Devices for its use, nor for any infringements of patents or other rights of third parties which  may  result  from  its  use.  No  license  is  granted  by  implication  or One Technology Way, P.O. Box 9106, Norwood, MA 02062-9106,  U.S.A. |
| otherwise under any patent or patent rights of Analog Devices. | Tel: 617/329-4700 | Fax: 617/326-8703 |

AD1843–SPECIFICATIONS
STANDARD TEST CONDITIONS UNLESS OTHERWISE NOTED
| Temperature | 25 | °C | DAC Conditions | ADC Input Conditions |
| --- | --- | --- | --- | --- |
| Digital Supply (VDD) | 5.0 | V | Autocalibrated | Mic 20 dB Gain Disabled |
| Analog Supply (VCC) | 5.0 | V | 0 dB Attenuation | LIN Single-Ended |
| Sample Rate (FS) | 48 | kHz | 0 dB Relative to Full Scale | (LINLSD & LINRSD = 0) |
| Input Signal | 1008 | Hz | 16-Bit Linear Mode | Autocalibrated |
| Analog Output Passband | 20 Hz to 20 kHz | No Output Load | 0 dB PGA Gain |  |
| ADC FFT Size | 2048 | Mute Off | –1.0 dB Relative to Full Scale |  |
| DAC FFT Size | 8192 | DAC1 Single-Ended | Line Input |  |
| VIH | 2.0 | V | DAC2 Differential | 16-Bit Linear Mode |
| VIL | 0.8 | V |  |  |
| VOH | 2.4 | V |  |  |
| VOL | 0.4 | V |  |  |
| IOH | –2 IOL 2mA ANALOG INPUT | mA |  |  |
| Min | Typ | Max | Units Full-Scale Input Voltage (RMS Values Assume Sine Wave Input) |  |
| All Inputs with ADRFLT & ADLFLT = 0 and LINLSD & LINRSD = 0 | 1 | V rms |  |  |
| (LINLP, LINRP, AUX1L, AUX1R, AUX2L, | 2.55 | 2.828 AUX2R, AUX3L, AUX3R, MIN) | 3.1 | V p-p |
| All Inputs with ADRFLT & ADLFLT = 0 and LINLSD & LINRSD = 1 | 2 | V rms |  |  |
| (LINLP & LINLN, LINRP & LINRN) | 5.1 | 5.656 | 6.2 | V p-p |
| All Inputs with ADRFLT & ADLFLT = 1 and LINLSD & LINRSD = 0 | 1.127 | V rms |  |  |
| (LINLP, LINRP, AUX1L, AUX1R, AUX2L, AUX2R, AUX3L, | 2.8 | 3.156 AUX3R, MIN) | 3.5 | V p-p |
| All Inputs with ADRFLT & ADLFLT = 1 and LINLSD & LINRSD = 1 | 2.254 | V rms |  |  |
| (LINLP & LINLN, LINRP & LINRN) | 5.6 | 6.312 MIC with +20 dB Gain (LMGE & RMGE  = 1 | 7.0 | V p-p |
| and ADRFLT & ADLFLT = 0) | 0.1 | V rms |  |  |
| (MICL, MICR) | 0.25 | 0.2828 MIC with 0 dB Gain (LMGE & RMGE = 0 | 0.31 | V p-p |
| and ADRFLT & ADLFLT = 0) | 1 | V rms |  |  |
| (MICL, MICR) | 2.55 | 2.828 | 3.1 | V p-p |
| AUX, SUM and MIN Input Impedance* | 10K (AUX1L, AUX1R, AUX2L, AUX2R, AUX3L, AUX3R, SUML, SUMR, MIN) | Ω |  |  |
| LIN Input Impedance* (LINLP, LINLN, LINRP, LINRN) | 40K | Ω |  |  |
| MIC Input Impedance* (MICL, MICR) | 20K | Ω |  |  |
| Input Capacitance* (All Inputs) | 15 PROGRAMMABLE GAIN AMPLIFIER–ADC | pF |  |  |
| Min | Typ | Max | Units |  |
| Step Size (0 dB to 22.5 dB) (All Steps Tested) | 1.3 | 1.5 | 1.7 | dB |
| PGA Gain Range Span* | 21.5 INPUT (AUX1, AUX2, AUX3, MIN, MIC) ANALOG AMPLIFIERS/ATTENUATORS | 22.5 | 23.5 | dB |
| Min | Typ | Max | Units |  |
| Step Size (+12.0 dB to –30 dB) (All Steps Tested) | 1.25 | 1.5 | 1.75 | dB |
| Step Size (–31.5 dB to –34.5 dB) (All Steps Tested) | 1.1 | 1.5 | 1.9 | dB |
| Input Gain/Attenuation Range* | 45.5 | 46.5 | 47.5 | dB |
| Mute Attenuation* | –80.0 –2– REV. 0 | dB |  |  |

AD1843
DIGITAL DECIMATION AND INTERPOLATION FILTERS–AUDIO MODE*
| Min | Max | Units |
| --- | --- | --- |
| Passband | 0 | 0.40×FS Hz |
| Passband Ripple | 0 | –0.016 dB |
| Transition Band | 0.4×FS | 0.6×FS Hz |
1
| Stopband | 0.6×FS | Hz |
| --- | --- | --- |
| Stopband Rejection | 91.8 | dB |
| Group Delay | 15/FS | s |
| Group Delay Variation Over Passband | 0.0 DIGITAL DECIMATION AND INTERPOLATION FILTERS–MODEM MODE* | μs |
| Min | Max | Units |
| Passband | 0 | 0.442×FS Hz |
| Passband Ripple | 0 | –0.220 dB |
| Transition Band | 0.442×FS | 0.542×FS Hz |
2
| Stopband | 0.542×FS | Hz |
| --- | --- | --- |
| Stopband Rejection | 75.7 | dB |
| Group Delay | 19/FS | s |
| Group Delay Variation Over Passband | 0.0 | μs |
| Sample Rate | 24 | kHz DIGITAL DECIMATION AND INTERPOLATION FILTERS–RESAMPLER MODE* |
| Min | Max | Units |
| Passband | 0 | 0.4×FS Hz |
| Passband Ripple | 0 | –0.035 dB |
| Transition Band | 0.4×FS | 0.5×FS Hz |
3
| Stopband | 0.5×FS | Hz |
| --- | --- | --- |
| Stopband Rejection | 92.2 | dB |
| Group Delay | 25/FS | s |
| Group Delay Variation Over Passband | 0.0 ANALOG-TO-DIGITAL CONVERTERS | μs |
| Min | Typ | Max Units Audio Dynamic Range (–60 dB Input, THD+N Referenced to Full Scale, |
| A-Weighted, ADRFLT & ADLFLT = 0) | 80 | 85 dB Modem Dynamic Range (–60 dB Input, THD+N Referenced to Full Scale, 300 Hz to 4 kHz Analog Output Passband, LINRSD & LINLSD = 1, |
| ADRFLT & ADLFLT = 1, FS= 12.8 kHz) | 87 | 90 dB |
| Audio THD+N (Referenced to Full Scale) | 0.03 | % |
| –74 | –70 Modem THD+N (–3.0 dB Referenced to Full Scale, 300 Hz to 4 kHz Analog Output Passband, LINRSD & LINLSD = 1, | dB |
| ADRFLT & ADLFLT = 1, FS= 12.8 kHz) | 0.02 | % |
| –78.5 | –74 | dB |
| Audio Signal-to-Intermodulation Distortion* (CCIF Method) | –94 | –80 dB ADC Crosstalk* |
| LIN Inputs (Input L, Ground R, Read R; Input R, Ground L, Read L) | –80 | dB |
| Line to MIC (Input LIN, Ground and Select MIC, Read Both Channels) | –80 | dB |
| Line to AUX1, AUX2, AUX3, MIN | –80 | dB |
| Interchannel Gain Mismatch (Difference of Gain Errors) | ±0.5 | dB |
| ADC Offset Error | 10 | 50 mV REV. 0 –3– |

AD1843
DAC1 DIGITAL-TO-ANALOG CONVERTERS
| Min | Typ | Max | Units Audio Dynamic Range (–60 dB Input, THD+N Referenced to Full Scale, |
| --- | --- | --- | --- |
| A-Weighted, DA1FLT = 0) | 77 | 80 | dB |
| Audio THD+N (Referenced to Full Scale, DA1FLT = 0) | 0.03 | % |  |
| –74 | –70 | dB |  |
| Audio Signal-to-Intermodulation Distortion* (CCIF Method) | –92 | –80 | dB |
| Interchannel Gain Mismatch (Difference of Gain Errors) | ±0.5 DAC Crosstalk* (Input L, Zero R, Measure LOUT1R; Input R, Zero L, | dB |  |
| Measure LOUT1L) | –77 Total Out-of-Band Energy* | dB |  |
| (Measured from 0.6×FSto 100 kHz in Audio Mode) | –60 Audible Out-of-Band Energy* (Measured from 0.6×FSto 22 kHz in Audio Mode, | dB |  |
| Tested at FS= 8.0 kHz) | –72 DAC2 DIGITAL-TO-ANALOG CONVERTERS | dB |  |
| Min | Typ | Max | Units Audio Dynamic Range (–60 dB Input, THD+N Referenced to Full Scale, |
| A-Weighted, DA2FLT = 0) | 78 Modem Dynamic Range (–60 dB Input, THD+N Referenced to Full Scale, 300 Hz to 4 kHz Analog Output Passband,  DA2FLT = 1, RDA2G5:0 | 80 | dB |
| & LDA2G5:0 = 000101 [4.5 dB], FS= 12.8 kHz) | 87 | 90 | dB |
| Audio THD+N (Referenced to Full Scale, DA2FLT = 0) | 0.03 | % |  |
| –77 | –70 Modem THD+N (–3.0 dB Referenced to Full Scale, 300 Hz to 4 kHz Analog Output Passband, DA2FLT = 1, RDA2G5:0 | dB |  |
| & LDA2G5:0 = 000101 [4.5 dB], FS= 12.8 kHz) | 0.016 | % |  |
| –81 | –76 | dB |  |
| Audio Signal-to-Intermodulation Distortion* (CCIF Method) | –86 | –80 | dB |
| Interchannel Gain Mismatch (Difference of Gain Errors) | ±0.5 DAC Crosstalk* (Input L, Zero R, Measure LOUT2R; Input R, Zero L, | dB |  |
| Measure LOUT2L) | –80 Total Out-of-Band Energy* | dB |  |
| (Measured from 0.6×FSto 100 kHz in Audio Mode) | –60 Audible Out-of-Band Energy* (Measured from 0.6×FSto 22 kHz in Audio Mode, | dB |  |
| Tested at FS= 8.0 kHz) | –72 DC Offset 525mV DAC1 AND DAC2 ANALOG AMPLIFIERS/ATTENUATORS | dB |  |
| Min | Typ | Max | Units |
| Step Size (+12.0 dB to –30.0 dB) (All Steps Tested) | 1.25 | 1.5 | 1.75 dB |
| Step Size (–31.5 dB to –34.5 dB) (All Steps Tested) | 1.1 | 1.5 | 1.9 dB |
| Step Size (–36.0 dB to –82.5 dB)* | 1.3 | 1.5 | 1.7 dB |
| Output Attenuation Span* | 81.5 | 82.5 | 83.5 dB |
| Mute Attenuation* | –80 DIGITAL MIX ATTENUATORS | dB |  |
| Min | Typ | Max | Units |
| Step Size (0 dB to –94.5 dB)* (All Steps Tested) | 1.3 | 1.5 | 1.7 dB |
| Output Attenuation Span* | 93.5 | 94.5 | 95.5 dB |
| Mute Attenuation* | –90 –4– REV. 0 | dB |  |

AD1843
ANALOG OUTPUT
| Min | Typ | Max | Units |
| --- | --- | --- | --- |
| LOUT1 Full-Scale Output Voltage | 0.707 | V rms |  |
| (RMS Values Assume Sine Wave Input) | 1.8 | 2.0 | 2.2 V p-p |
| LOUT2 Full-Scale Single-Ended Output Voltage | 0.707 | V rms |  |
| (RMS Values Assume Sine Wave Input) | 1.8 | 2.0 | 2.2 V p-p |
| LOUT2 Full-Scale Differential Output Voltage | 1.414 | V rms |  |
| (RMS Values Assume Sine Wave Input) | 3.6 | 4.0 | 4.4 V p-p |
| LOUT1 Output Impedance* | 600 | Ω |  |
| LOUT2 Output Impedance* | 1 | Ω |  |
| LOUT1 External Load Impedance* | 10 | kΩ |  |
| LOUT2 External Load Impedance* | 2 | kΩ |  |
| MOUT External Load Impedance* | 10 | kΩ |  |
| HPOUT External Load Impedance* | 16 | 32 | Ω |
| HPOUT THD+N (Referenced to Full Scale, 32ΩExternal Load Impedance) | 0.10 –60 dB | % |  |
| Output Capacitance* | 15 | pF |  |
| External Load Capacitance* | 100 | pF |  |
| CMOUT | 2.10 | 2.25 | 2.40 V |
| External CMOUT Load Current* | 10 | μA |  |
| CMOUT Output Impedance* | 4 | kΩ Mute Click* (Muted Output Minus Unmuted Midscale DAC1 and DAC2 Outputs) ±5mV SYSTEM SPECIFICATIONS Max Units |  |
| System Frequency Response Ripple* (Line-In to Line-Out) | 1.0 | dB |  |
| Differential Nonlinearity* | ±1 | Bit |  |
| Phase Linearity Deviation* | 5 | Degrees STATIC DIGITAL SPECIFICATIONS |  |
| Min | Max | Units High-Level Input Voltage (VIH) |  |
| Digital Inputs, Except SCLK | 2.0 | VDD+ 0.3 | V |
| XTALI and SCLK | 2.4 | VDD+ 0.3 | V |
| Low-Level Input Voltage (VIL) | –0.3 | 0.8 | V |
| High-Level Output Voltage (VOH) | 2.4 | V |  |
| Low-Level Output Voltage (VOL) | 0.4 | V |  |
| Input Leakage Current (GO/NOGO Tested) | –10 | 10 | μA |
| Output Leakage Current (GO/NOGO Tested) | –10 TIMING PARAMETERS (GUARANTEED OVER OPERATING TEMPERATURE AND DIGITAL SUPPLY RANGE) | 10 | μA |
| Min | Typ | Max | Units Serial Data Frame Sync [SDFS] Period (t1) |
| (Master Mode, FRS = 1 [16 Slots per Frame], SCF = 0 [SCLK = 12.288 MHz]) | 20.833 Frame Sync [SDFS] HI Pulse Width (t2)80ns Clock [SCLK] to Frame Sync [SDFS] Propagation Delay (tPD1)15ns Data [SDI] Input Setup Time to SCLK (tS)10ns Data [SDI] Input Hold Time from SCLK (tH)10  ns Clock [SCLK] to Output Data [SDO] Valid (tDV)15ns Clock [SCLK] to Output Data [SDO] Three-State [High-Z] (tHZ)15ns Clock [SCLK] to Time Slot Output [TSO] Propagation Delay (tPD2)15ns | μs |  |
| RESETandPWRDWNLO Pulse Width (tRPWL) | 100 | ns |  |
SCLK
SCLK
t2 t1
SDFS SDFS
| tPD1 | t | t | tPD1 S H |
| --- | --- | --- | --- |
| SDI | BIT 15 | BIT 14 | BIT 0 SDI OR SDO 15 14 13 3  2   1   0 15 14 13 15 1413 |
| tDV | tHZ | LAST |  |
VALID
SDO	BIT 15	BIT 14	BIT 0	TIME SLOT
TSO
tPD2
tRPWL
RESET
PWRDWN
Figure 1.  Timing Diagrams
REV. 0 –5–

AD1843
POWER SUPPLY (33ΩHPOUT LOAD)
| Min | Typ | Max | Units |
| --- | --- | --- | --- |
| Power Supply Range—Analog VCC | 4.75 | 5.25 | V |
| Power Supply Range—Digital VDD | 2.85 Total Power Supply Current—5.0 VCCand VDDOperating | 5.25 | V |
| (5.0 VCCand VDDSupplies) | 210 Total Power Supply Current—5.0 VCC/3.0 VDDOperating* | 250 | mA |
| (5.0 VCCAnalog/3.0 VDDDigital Supplies) | 150 | 175 | mA |
| Analog Supply Current—5.0 VCCOperating | 60 | 75 | mA |
| Digital Supply Current—5.0 VDDOperating | 150 | 175 | mA |
| Digital Supply Current—3.0 VDDOperating* | 90 | 100 | mA |
| Digital Power Supply Current—VDDPower Down (PWRDWNLO) | 1 | mA |  |
| Analog Power Supply Current—VCCPower Down (PWRDWNLO) | 0.5 | mA |  |
| Power Dissipation—5.0 VCCand VDDOperating (Current×Nominal Supply) | 1250 | mW |  |
| Power Dissipation—5.0 VCC/3.0 VDDOperating* (Current×Nominal Supply) | 875 Power Dissipation—5.0 VCCand VDDPower Down (PWRDWNLO) | mW |  |
| (Current×Nominal Supply) | 7.5 Power Dissipation—5.0 VCC/3.0 VDDPower Down* (PWRDWNLO) (Current×Nominal Supply) 5mW | mW |  |
| Power Supply Rejection (100 mV p-p Signal @ 1 kHz)* | 40 (At Both Analog and Digital Supply Pins, for ADC, DAC1 and DAC2) CLOCK SPECIFICATIONS* | dB |  |
| Min | Typ | Max | Units |
| Input Crystal/Clock Frequency | 24.576 | MHz |  |
| Input Clock Duty Cycle (When an External Clock Is Used Instead of a Crystal) | 25/75 | 75/25 | % |
| Initialization Sample Rate Change Time | 0 | ms PACKAGE CHARACTERISTICS Typ Units |  |
| PQFPθJA(Thermal Resistance [Junction-to-Ambient]) | 96 | °C/W |  |
| PQFPθJC(Thermal Resistance [Junction-to-Case]) | 8.75 | °C/W |  |
| TQFPθJA(Thermal Resistance [Junction-to-Ambient]) | 30.6 | °C/W |  |
| TQFPθJC(Thermal Resistance [Junction-to-Case]) | 4.6 | °C/W |  |
NOTES
1
The stopband repeats itself at multiples of 64×FS, where FSis the sampling frequency. Thus the audio mode digital filter will attenuate to –91.8 dB or better across
the frequency spectrum except for a range of±0.6×FSwide at multiples of 64×FS.
2
The stopband repeats itself at multiples of 64×FS, where FSis the sampling frequency. Thus the modem mode digital filter will attenuate to –75.7 dB or better across
the frequency spectrum except for a range of±0.542×FSwide at multiples of 64×FS.
3
The stopband repeats itself at multiples of 64×FS, where FSis the sampling frequency. Thus the resampler mode digital filter will attenuate to –92.2 dB or better
across the frequency spectrum except for a range of±0.5×FSwide at multiples of 64×FS.
*Guaranteed, not tested.
Specifications subject to change without notice.
ABSOLUTE MAXIMUM RATINGS* *Stresses greater than those listed under “Absolute Maximum Ratings” may cause
permanent  damage  to  the  device.  This  is  a  stress  rating  only  and  functional
Min	Max	Units
operation of the device at these or any other conditions above those indicated in the
Power Supplies operational  section  of  this  specification  is  not  implied.    Exposure  to  absolute
maximum rating conditions for extended periods may affect device reliability.
Digital (V	)	–0.3	6.0	V
DD
Analog (V	)	–0.3	6.0	V
CC
ORDERING INFORMATION
Input Current
| (Except Supply Pins) | ±10.0 | mA |
| --- | --- | --- |
| Temperature | Package | Package |
| Analog Input Voltage (Signal Pins)  –0.3 | V | + 0.3 V |
CC
Model	Range	Description	Option
Digital Input Voltage (Signal Pins)   –0.3	V	+ 0.3	V
DD
| Ambient Temperature (Operating)  0 | +70 | °C |
| --- | --- | --- |
| AD1843JS | 0°C to +70°C | 80-Lead PQFP S-80 |
| Storage Temperature | –65 | +150 °C |
| AD1843JST | 0°C to +70°C | 100-Lead TQFP   ST-100 |
| ESD Tolerance (Human Body | 1000 Model per Method 3015.2 of MIL-STD-883B) | V |
CAUTION
ESD (electrostatic discharge) sensitive device. Electrostatic charges as high as 4000 V readily
accumulate  on  the  human  body  and  test  equipment  and  can  discharge  without  detection. WARNING!
Although the AD1843 features proprietary ESD protection circuitry, permanent damage may
occur  on  devices  subjected  to  high  energy  electrostatic  discharges.  Therefore,  proper  ESD
ESD SENSITIVE DEVICE
precautions are recommended to avoid performance degradation or loss of functionality.
–6– REV. 0

AD1843
PIN CONFIGURATIONS
80-Lead PQFP
| DD | DD | DD | DDDD |
| --- | --- | --- | --- |
| SDISCLKGNDDV | CLKOUTCONV3BIT3GNDDV CONV2BIT2GNDDV CONV1BIT1GNDDV V 80 79 78 77 76 75 74 73 72 71 70 69 68 67 66 65 64 63 62 61 SDO 1 60GNDD SDFS 2 59XCTL1 GNDD 3 58XCTL0 VDD 4 57 SYNC3 TSI 5 56SYNC2 TSO 6 55SYNC1 GNDD 7 54 GNDD VDD 8 53 VDD CS 9 52RESET | XTALOXTALI |  |
| BM 10 | AD1843 | 51PWRDWN TOP VIEW AUX3R 11 50 VDD (Not to Scale) AUX3L 12 49PDMNFT AUX2R 13 48 GNDA AUX2L 14 47HPOUTL AUX1R 15 46HPOUTC AUX1L 16 45HPOUTR MICR 17 44 VCC MICL 18 43SUML MIN19 42SUMR VCC 20 41 VCC 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 |  |
REF
V
GNDA FILTR FILTLLINRPLINRNLINLPLINLN	MOUT GNDA	GNDA
CMOUT
LOUT1RLOUT1L
AAFILTRAAFILTL LOUT2LPLOUT2LN
LOUT2RPLOUT2RN
PIN DESCRIPTION
Serial Interface
| Pin Name | PQFP | TQFP | I/O | Description |
| --- | --- | --- | --- | --- |
| SCLK | 79 | 99 | I/O | Serial Clock.  SCLK is a bidirectional signal that supplies the clock as an output to the serial bus when the Bus Master (BM) pin is driven HI and accepts the clock as an input when the BM pin is driven LO. When the AD1843 is configured in master mode, the SCLKfrequency may be set to either 12.288 MHz or 16.384 MHz with the SCF bit in Control Register Address 26. |
| SDFS | 2 | 2 | I/O | Serial Data Frame Sync.  SDFS is a bidirectional signal that supplies the frame synchronization signal as an output to the serial bus when the Bus Master (BM) pin is driven HI and accepts the frame synchronization signal as an input when the BM pin is driven LO. |
| SDI | 80 | 100 | I | Serial Data Input.  SDI is used by peripheral devices such as the host CPU or a DSP to supply control and playback data information to the AD1843.  All control and playback transfers are 16 bits long, MSB first. |
| SDO | 1 | 1 | O | Serial Data Output.  SDO is used to supply status/control register readback and capture data information to peripheral devices such as the host CPU or a DSP. All status/control register readback and capture data transfers are 16 bits long, MSB first.  A three-state output driver is used on this pin. |
| BM | 10 | 12 | I | Bus Master.  When BM is tied HI the AD1843 is the serial bus master.  The AD1843 will then supply the serial clock (SCLK) and the frame sync (SDFS) signals for the serial bus.  No more than one device (AD1843/CPU/DSP) should be configured as the serial bus master.  When BM is tied LO, the AD1843 is con- figured as a bus slave, and will accept the SCLK and SDFS signals as inputs. The logic level on this pin must not be changed onceRESETis deasserted (driven HI). REV. 0 –7– |

AD1843
PIN CONFIGURATIONS
100-Lead TQFP
DD	DD	DD	DDDD
SDISCLKNCGNDDV	CLKOUTCONV3NCBIT3GNDDV	CONV2NCBIT2GNDDV CONV1NCBIT1GNDDV V	NCXTALOXTALI
100 99 98 97 96 95 94 93 92 91 90 89 88 87 86 85 84 83 82 81 80 79 78 77 76
SDO 1 75 GNDD
SDFS 2 74 XCTL1
NC 3 73 NC
GNDD 4 72 XCTL0
VDD 5 71 SYNC3
TSI 6 70 SYNC2
TSO 7 69 SYNC1
NC 8 68 NC
GNDD 9 67 GNDD
VDD 10 66 VDD
CS 11 65 RESET
BM 12 64 PWRDWN
AD1843
NC 13	TOP VIEW	63 NC
(Not to Scale)
NC 14 62 VDD
AUX3R 15 61 PDMNFT
AUX3L 16 60 NC
AUX2R 17 59 GNDA
AUX2L 18 58 HPOUTL
AUX1R 19 57 HPOUTC
AUX1L 20 56 HPOUTR
MICL 21 55 VCC
MICR 22 54 SUML
MIN 23 53 SUMR
NC 24 52 NC
VCC 25 51 VCC
26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50
NC	NC	NC	NC	REF	NC
V
GNDA FILTR FILTL	LINRPLINRNLINLPLINLN	MOUT GNDA	GNDA
LOUT1RLOUT1LCMOUT
AAFILTRAAFILTL LOUT2LP
LOUT2RPLOUT2RNLOUT2LN
NC = NO CONNECT
Serial Interface (Continued)
| Pin Name | PQFP | TQFP | I/O | Description |
| --- | --- | --- | --- | --- |
| CS | 9 | 11 | I | Chip Select.When CS is set HI, the serial interface I/O pins will be in their normal active states. When CS is reset LO, SCLK, SDFS, and SDO are three- stated; SCLK, SDFS and SDI inputs are ignored; and TSO drives out the logic level received on TSI. |
| TSO | 6 | 7 | O | Time Slot Output.  TSO is asserted HI by the AD1843 simultaneously with the LSB of the last time slot used by the AD1843.  It is used to daisy-chain multiple AD1843s on a common TDM serial bus.  If the power-down (PWRDWN) pin is asserted or if the chip select pin (CS) is deasserted, TSO is set to the logic level on the TSI pin, allowing powered-down or unselected AD1843s on a daisy-chain to be skipped. |
| TSI | 5 | 6 | I | Time Slot Input.  Asserting TSI HI indicates to the AD1843 that it should use the next six time slots beginning on the next SCLK period.  It also enables TSO to be asserted at the end of these six time slots.  TSI is ignored (but should be tied LO) when the AD1843 is the bus master since the bus master uses the first time slots in a TDM frame. |
| XCTL[1:0] | 59, 58 | 72, 74 | I/O | External Control.  These signals reflect the status of bits (Data 8 and 9) in Control Register Address 28 of the AD1843.  They may be used for signaling or controlling external logic. –8– REV. 0 |

AD1843
Analog Signals
| Pin Name | PQFP | TQFP | I/O | Description |
| --- | --- | --- | --- | --- |
| LINLP | 28 | 35 | I | Line Input Left Channel Positive Differential Signal. |
| LINLN | 29 | 36 | I | Line Input Left Channel Negative Differential Signal. |
| LINRP | 26 | 33 | I | Line Input Right Channel Positive Differential Signal. |
| LINRN | 27 | 34 | I | Line Input Right Channel Negative Differential Signal. |
| MICL | 18 | 21 | I | Microphone Input Left Channel.  Microphone input for the left channel. This signal can be either line level or –20 dB from line level. |
| MICR | 17 | 22 | I | Microphone Input Right Channel.  Microphone input for the right channel. This signal can be either line level or –20 dB from line level. |
| AUX1L | 16 | 20 | I | Auxiliary #1 Left Channel Line Input. |
| AUX1R | 15 | 19 | I | Auxiliary #1 Right Channel Line Input. |
| AUX2L | 14 | 18 | I | Auxiliary #2 Left Channel Line Input. |
| AUX2R | 13 | 17 | I | Auxiliary #2 Right Channel Line Input. |
| AUX3L | 12 | 16 | I | Auxiliary #3 Left Channel Line Input. |
| AUX3R | 11 | 15 | I | Auxiliary #3 Right Channel Line Input. |
| MIN | 19 | 23 | I | Monaural (Mono) Line Input. |
| MOUT | 35 | 44 | O | Monaural (Mono) Line Output. |
| LOUT1L | 36 | 45 | O | Line Output #1 Left Channel. |
| LOUT1R | 34 | 43 | O | Line Output #1 Right Channel. |
| HPOUTL | 47 | 58 | O | Headphone Output Left Channel. |
| HPOUTC | 46 | 57 | Headphone Common Return. |  |
| HPOUTR | 45 | 56 | O | Headphone Output Right Channel. |
| LOUT2LP | 32 | 40 | O | Line Output #2 Left Channel Positive Differential Signal. |
| LOUT2LN | 33 | 41 | O | Line Output #2 Left Channel Negative Differential Signal. |
| LOUT2RP | 30 | 38 | O | Line Output #2 Right Channel Positive Differential Signal. |
| LOUT2RN | 31 | 39 | O | Line Output #2 Right Channel Negative Differential Signal. |
| SUML | 43 | 54 | I | Mixer Line Input Left Channel. |
| SUMR | 42 | 53 | I | Mixer Line Input Right Channel. Clocks |
| Pin Name | PQFP | TQFP | I/O | Description |
| CLKOUT | 76 | 95 | O | Clock Output.  This signal is a buffered version of XTALO (with a duty cycle restored to at least 60%/40%), the crystal clock output. This pin is enabled by default but can be three-stated by programming a bit in Control Register Address 28. The CLKOUT frequency is 24.576 MHz. |
| SYNC[3:1] | 57, 56, 55 | 71, 70, 69 | I | Sync Inputs.  These SYNC signals are used as the clock source inputs to three receptive PLLs in the AD1843.  These pins accept a clock at, or at a multiple of, the desired sample rate for A-to-D and D-to-A conversions.  These inputs are ignored if a sample rate is programmed directly, but should never be left floating. |
| CONV[3:1]    75, 71, 67 | 94, 89, 84 | O | ConversionClock Outputs. These output clocks have an average period equal to (or 128 times) the internal sample rates of the AD1843. These clock outputs are three-stated by default but can be enabled by programming bits in ControlRegister Address 28. |  |
| BIT[3:1] | 74, 70, 66 | 92, 87, 82 | O | Bit Clock Outputs.  These output clocks can be individually programmed to multiples of the sample rates.  Support for V.34 or V.32 bit rates is available. These clock outputs are three-stated by default but can be enabled by programming bits in Control Register Address 28. REV. 0 –9– |

AD1843
Miscellaneous
| Pin Name | PQFP | TQFP | I/O | Description |
| --- | --- | --- | --- | --- |
| XTALI | 61 | 76 | I | 24.576 MHz Crystal Input.  When using a crystal as the clock source, the crystal should be connected between the XTALI and XTALO pins.  This crystal should be 24.576 MHz for the normal sampling rate range, i.e., 4 kHz to 54 kHz.  A clock input (perhaps the CLKOUT of another AD1843) may be driven into XTALI in place of a crystal. The external clock input must be greater than or equal to 512 times the maximum desired AD1843 sampling frequency. |
| XTALO | 62 | 77 | O | 24.576 MHz Crystal Output.  When using a crystal as the clock source, the crystal should be connected between the XTALI and XTALO pins.  If a clock is driven directly into XTALI, then XTALO should be left unconnected. |
| PWRDWN | 51 | 64 | I | Power Down.PWRDWNis active LO. The assertion of this signal will initialize the on-chip Control Registers to their default values, and will completely and quietly power down the AD1843. If a crystal is not connected between XTALI and XTALO, there must be a 24.576 MHz clock input on XTALI for at least 5 ms after this signal is asserted LO for proper operation. The AD1843 will not be completely powered down until after this 5 ms period elapses. The AD1843 always finishes an in-progress power-up sequence before initiating a power-down sequence, and vice versa. If thePWRDWNpin is asserted while a power-up sequence is in progress, the 24.576 MHz clock signal on XTALI must persist for a worst case maximum of 479 ms (power up = 470 ms, autocalibration = 4 ms, power down = 5 ms) afterPWRDWNis asserted. When INIT (Control Register Address 0, Bit 15) is set to a “1,” the power-down sequence is complete. See the “Power Management” section for important additional details. |
| RESET | 52 | 65 | I | Reset.RESETis active LO.  The assertion of this signal will initialize the on-chip registers to their default values, and will completely power down the AD1843. RESETis similar toPWRDWN, except that whenPWRDWNis asserted, power down is “quiet” and performed synchronously to the internal clocks. WhenRESET is asserted, power down is “noisy” and performed asynchronously to the internal clocks. |
| PDMNFT     49 | 61 | I | Power-Down Mono Feedthrough.  When the AD1843 mixer is powered down, and PDMNFT is asserted HI, the Mono Input (MIN, PQFP Pin 19) is routed to the Mono Output (MOUT, PQFP Pin 35), and the signal applied to MIN will feedthrough to MOUT.  When the AD1843 mixer is powered down and PDMNFT is deasserted LO, the feedthrough of MIN to MOUT will be muted. When the AD1843 mixer is not powered down, and MIN to MOUT feedthrough is desired, the Mono Input Mix Mute (Control Register Address 8, Bit 15) and the Mono Output Mute (Control Register Address 8, Bit 6) must be unmuted. During power-down feedthrough, the signal applied to the MIN input appears only at the MOUT output.  During normal operation, the signal applied to the MIN input appears at both the MOUT and the LOUT1 outputs.  The state of the PDMNFT pin should be changed when the AD1843 mixer is powered up.  If the state of PDMNFT is changed when the AD1843 is in total power-down, audible pops and clicks will likely result. |  |
| CMOUT | 38 | 47 | O | Common-Mode Voltage Output.  Nominal 2.25 volt reference available externally for dc-coupling and level-shifting.  CMOUT should not be used where it will sink or source current. |
| VREF | 39 | 48 | I | Voltage Reference Filter.  Voltage reference filter point for external bypassing only. |
| FILTL | 25 | 31 | I | Left Channel Filter.  This pin requires a 1.0μF capacitor to analog ground for proper operation. |
| FILTR | 23 | 29 | I | Right Channel Filter.  This pin requires a 1.0μF capacitor to analog ground for proper operation. |
| AAFILTL      24 | 30 | I | Left Channel Antialias Filter.  This pin requires a 1000 pF capacitor to analog ground for proper operation. |  |
| AAFILTR      22 | 28 | I | Right Channel Antialias Filter.  This pin requires a 1000 pF capacitor to analog ground for proper operation. –10– REV. 0 |  |

AD1843
POWER SUPPLIES
| Pin Name  PQFP | TQFP | I/O | Description |
| --- | --- | --- | --- |
| VCC | 20, 41, 44 | 25, 51, 55 | I Analog Supply Voltage (+5 V). |
| GNDA | 21, 37, 40, 48 | 27, 46, 49, 59   O | Analog Ground. |
| VDD | 4, 8, 50, 53, | 5, 10, 62, 66,    I | Digital Supply Voltage (+5/3 V). 63, 64, 68, 72, 79, 80, 85, 90, 77 96 |
| GNDD | 3, 7, 54, 60, | 4, 9, 67, 75, | I Digital Ground. 65, 69, 73, 78 81, 86, 91, 97 |
| NC | 3, 8, 13, 14, | NoConnect. May be left floating. 24, 26, 32, 37, 42, 50, 52, 60, 63, 68, 73, 78, 83, 88, 93, 98 (continued from page 1) differential stereo output for connection to a DAA. The line and The versatility of the device is shown by the following examples differential outputs are looped back to the ADC input selector. of functions it can perform: The AD1843’s mixing and routing capabilities are extensive. • Stereo audio input and/or quad output, simultaneously at dif- The digital data from both DAC channels after interpolation ferent sample rates can be routed back to the ADC decimators, to support digital- • Stereo audio output with simultaneous full duplex modem or to-digital sample rate conversion (digital resampling).  Digital fax operation with frequency and phase resampling data from the ADC can also be routed to the two stereo DAC pairs, for a digital loopback mode which is helpful for device- • Mono audio input and stereo audio output with simultaneous level and board-level test.  Digital data from either stereo DAC modem receive and transmit for simultaneous voice and data can be mixed with the digital data feeding the other DAC, and communications the analog signal from DAC2 can be mixed with the analog out- • Dual independent audio inputs with audio output for echo- put from DAC1. cancelling speakerphones Sample rates are independently programmable in the range of Audio Functional Description 4 kHz to 54 kHz to a 1 Hz resolution or sample rates can be The AD1843 SoundComm codec provides a complete audio so- synchronized to an external source.  Up to three different signals lution with very few external components required.  Dynamic can be applied to the device’s three digital phase lock loop range of the device exceeds 80 dB over the 20 kHz audio band SYNC inputs for external synchronization. and sample rates from 4 kHz to 49 kHz are supported (up to 54 kHz for a single channel if other channels are powered These SYNC inputs can also be used in a special mode for au- down). The audio functionality of this device is a superset of dio/video synchronization.  In this mode, an NTSC or PAL de- ® that found in the Analog Devices AD1848 SoundPortdevice rived clock signal (approximately 15 kHz) is applied to the which has set the business audio standard throughout the com- SYNC inputs and the device produces one of a variety of stan- puter industry. dard audio sample rates (32 kHz, 44.056 kHz, 44.1 kHz and 48 kHz, and most of these divided by the integers 1 through 8). Inputs to the device include a stereo microphone pair, a stereo In this manner, video and audio sample rates which are math- line pair, a stereo CD input pair (AUX1), a stereo synthesized ematically unrelated can be locked together. music input pair (AUX2), a dual phone line input (AUX3), a Data Communications/Telephony Functional Description mono input, and a stereo input from an FM synthesizer (SUM). The AD1843 includes all data conversion, filtering, and clock All of these inputs (except SUM) are multiplexed to the two∑∆ generation circuitry needed to implement an echo-cancelling A/D converters and are mixable directly as analog signals with modem with a companion digital signal processor.  Software- the outputs of the D/A converters.  All analog input signals (ex- programmable sample rates and clocking modes support all cept SUM) can be amplified, attenuated or muted before mix- established modem standards including those for the V.34 ing with the outputs of the D/A converters. standard. The device has two pairs of∑∆DACs which accept 8- or 16-bit The AD1843 utilizes advanced∑∆technology to move the digital data from the serial port.  Each DAC pair’s independent entire echo-cancelling modem implementation into the digital sampling rate can either be programmed by Control Register domain.  The device maintains 90 dB typical dynamic range (with 1 Hz resolution) or synchronized to an external input. throughout all filtering and data conversion across a 9.6 kHz The second pair of DACs can be used to replace the music syn- passband.  Purely DSP-based echo cancellation algorithms can thesis DAC pair found on many audio products for PCs.  Out- maintain robust bit error rates under worst-case signal attenua- puts from the AD1843 include a line output, a mono output, a tion and echo amplitude conditions.  The AD1843’s on-chip stereo headphone output with its own current return path, and a interpolation filter resamples (both frequency and phase) the re- ceived signal after echo cancellation in the DSP, freeing the pro- SoundPort is a registered trademark of Analog Devices, Inc. cessor for other voice or data communications tasks. REV. 0 –11– |  |

AD1843
CLKOUT SCLKSDFSSDISDOBM CS TSOTSI XCTL [1:0]PDMNFTRESETPWRDWN
2
DIGITAL INTERFACE
| ADC | DAC1 BIT1 CONTROLREGISTERS | DAC2 |
| --- | --- | --- |
| μ/A | μ/A | μ/A |
| LAW | LAW BIT2 DD | LAW |
V
FIFO FIFO
BIT3 8 9
LEFTRIGHT
GNDD
∑
ATTN MUTE ∑
∑
CC
ATTNMUTE∑ V
AD1843
ATTN	ATTN	43
GNDA
CLOCK GENERATION	MUTE	MUTE
LEFTRIGHT
SELECT	R	∑	∑
O
FILTR
∑	MUTE	MUTE	∑
SELECTOR
FILTL
∑∆ADC
XTALI    XTALO   SYNC3   SYNC2   SYNC1   CONV3   CONV2   CONV1
∑∆DAC ∑∆DAC
PGA
GN/AT CMOUT
LEFT
SELECTOR RIGHT
MUTE
REF
V
∑ VOLTAGE REFERENCE
GN/ATMUTE ∑ MUTEGN/AT
∑
GN/AT
GN/ATMUTE ∑
∑ MUTE
AAFILTR
GN/ATMUTE ∑
∑
20 dB
GN/ATMUTE ∑	GN/AT = GAIN/ATTENUATION	AAFILTL
∑
GN/ATMUTE ∑ MUTE
MUTEMUTE
MUTE
DRIVER
MIN
MICLMICR SUML
LINLPLINLNLINRPLINRN	AUX1LAUX2LAUX3L	MOUT	SUMR
AUX1RAUX2RAUX3R
LOUT1LLOUT1R HPOUTL
HPOUTCHPOUTR
LOUT2LPLOUT2LNLOUT2RPLOUT2RN
Figure 2.  Detailed Functional Block Diagram
–12– REV. 0

AD1843
On-chip bit and baud clock generation circuitry allows either noise are removed from the DACs’ analog stereo output by on-
synchronous or asynchronous operation of the transmit (DAC) chip switched-capacitor and continuous-time filters.  All of the
and receive (ADC) paths.  Each path features independent analog inputs (except the stereo line input) can be mixed with
phase advance and retard adjustments via software control.  The the DAC1 output in the analog domain.  The DAC2 output can
AD1843 can also synchronize modem operation to an external also be mixed with the DAC1 output in the analog domain.
terminal band clock.  Because the device has multiple input and The DAC1 and DAC2 digital data can be fed back to the digital
output channels and converters, it is well suited for telephony half of the ADC to enable digital resampling operation.  DAC1
applications requiring multiple channels for voice and modem. and DAC2 can be run at different sample rates and with differ-
ent digital filter functions, without any beat frequency problems.
A detailed block diagram of the AD1843 is shown in Figure 2.
FUNCTIONAL DESCRIPTION
DETAILED PRODUCT DESCRIPTION
This section overviews the functionality of the AD1843 and is
The Serial-Port AD1843 SoundComm Codec integrates the key
intended as a general introduction to the capabilities of the de-
audio and PSTN data conversion and control functions into a
vice.  As much as possible, detailed reference information has
single integrated circuit.  The AD1843 is intended to provide a
been placed in “Control Registers” and other sections.  The
complete, single-chip audio and fax/modem solution for PC
user is not expected to refer repeatedly to this section.
multimedia applications.
Analog Inputs
External circuit requirements are limited to a minimal number
The AD1843 SoundComm Codec accepts stereo line-level and
of low cost support components.  Dynamic range exceeds 80 dB
mic-level inputs.  The mono MIN analog signal input, and LIN
over the 20 kHz audio band. Sample rates from 4 kHz to
(differential), MIC, AUX1, AUX2, AUX3 and post-mixed
54 kHz with 1 Hz resolution are supported from a single exter-
DAC output analog stereo signals are multiplexed to the inter-
nal crystal or clock source.
nal programmable gain amplifier stage (PGA).
The AD1843 SoundComm Codec is intended to be interfaced
The PGA following the input multiplexer allows left and right
through a DSP chip or an ASIC to a host bus such as ISA,
independent selectable gains for each channel from 0 dB to
EISA or PCI. A general system architecture is shown in
22.5 dB in +1.5 dB steps.  The Codec can operate either in a
Figure 3.
global stereo mode or in a global mono mode with left-channel
inputs appearing at both channel outputs.
S
Y
Analog Mixing
S	ASIC	AD1843	ANALOG I/O
T The MIN analog mono signal, and the MIC, AUX1, AUX2,
E
M AUX3 and SUM analog stereo signals can be mixed in the ana-
log domain with the DAC1 output.  Each channel of each auxil-
B ADSP-21xx
U iary analog input can be independently gained/attenuated from
S
+12 dB to –34.5 dB in 1.5 dB steps or completely muted.  The
Figure 3.  AD1843 System Diagram mixer output is available on LOUT1 externally and as an input
to the ADCs.  Even if the AD1843 is not playing back data from
The SoundComm codec includes a stereo pair of∑∆analog-to-
its DACs, the analog mix function can still be active.
digital converters and two stereo pairs of∑∆digital-to-analog
converters.  Inputs to the ADC can be selected from eight MIN allows the analog signal intended for the PC speaker to be
sources of analog signals: stereo line (LIN), stereo microphone passed through, attenuated or mixed in the AD1843’s analog
(MIC), stereo auxiliary line #1 (AUX1), stereo auxiliary line #2 domain.  MIN can be used to accept other mono input sources.
(AUX2), stereo auxiliary line #3 (AUX3), mono line (MIN), A digital control signal pin PDMNFT (Power Down Mono
mixer output, and DAC2 output.  A mono output and a stereo Feed Through) enables the mono input signal to be fed through
headphone driver are included on-chip.  A stereo line level input to the mono output when the AD1843 mixer is powered down.
(SUM) can be mixed into the output summer.  A software-con-
Analog-to-Digital Datapath
trolled programmable gain stage allows independent gain for
The AD1843∑∆ADCs incorporate a fourth-order modulator.
each ADC channel.  The ADCs’ output can be digitally mixed
A single pole of passive filtering is all that is required for
with both the DAC1 and DAC2 inputs.  The left and right
antialiasing the analog input because of the ADC’s high over-
ADC channels can be configured for different sample rates and
sampling ratio.  The ADCs include linear-phase digital decima-
digital filter function (audio, modem or resampling).
tion filters that low-pass filter the input.  ADC input overrange
The pair of 16-bit outputs from the ADCs is available over a se- conditions will cause bits to be set that can be read.
rial interface that also supports 16-bit digital input to the DACs
Each channel of the mic inputs can be amplified in the analog
and control/status information. The AD1843 can accept and
domain by +20 dB to compensate for the voltage swing differ-
generate 16-bit twos-complement PCM linear digital data, 8-bit
ence between line levels and typical condenser microphone levels.
unsigned magnitude PCM linear data, and 8-bitμ-law or A-law
Digital-to-Analog Datapath
companded digital data. The data format is defined indepen-
The∑∆DACs are preceded by a programmable attenuator and
dently for each conversion resource on the AD1843.
a low-pass digital interpolation filter. The anti-imaging interpo-
The∑∆DACs are preceded by a four sample deep FIFO buffer
lation filter oversamples and digitally filters the higher frequency
and a digital interpolation filter.  The DAC1 and DAC2 outputs
images.  The attenuator allows independent control of each
can be mixed in the digital domain.  Digital and analog attenua-
DAC channel from +12.0 dB to –82.5 dB in 1.5 dB steps plus
tors provide independent user volume control (plus mute) over
full mute.  The DACs’∑∆noise shapers oversample and con-
each DAC channel.  Nyquist images and shaped quantized
vert the signal to a single-bit stream.  The DAC outputs arethen
REV. 0 –13–

AD1843
filtered in the analog domain by a combination ofswitched-capaci- representations in all four formats correspond to equivalent full-
tor and continuous-time filters.  They remove the very high fre- scale signals.  The eight least-significant bit positions of 8-bit
quency components of the DAC bitstream output.  No external data in 16-bit fields are ignored on input and zeroed on output.
components are required.  Phase linearity at the analog output is
The 16-bit PCM data format is capable of representing 96 dB of
achieved by internally compensating for the group delay varia-
dynamic range.  Eight-bit PCM can represent 48 dB of dynamic
tion of the analog output filters.
range.  Compandedμ-law and A-law data formats use nonlinear
Changes in DAC output attenuation may be programmed to coding with less precision for large-amplitude signals.  The loss
take effect immediately, or only on zero crossings of the digital of precision is compensated for by an increase in dynamic range
signal, thereby eliminating “zipper” noise on playback.  Each to 64 dB and 72 dB, respectively.
channel has its own independent zero-crossing detector and at- On input, 8-bit companded data is expanded to an internal lin-
tenuator change control circuitry.  A timer guarantees that re- ear representation, according to whetherμ-law or A-law was
quested volume changes will occur even in the absence of an specified in the Codec’s internal registers.  Note that whenμ-
input signal that changes sign.  The time-outperiod is 8milli- law compressed data is expanded to a linear format, it requires
seconds at a 48 kHz sampling rate and 48milliseconds at an 14 bits.  A-law data expanded requires 13 bits.
8 kHz sampling rate.  (Time-out [ms]≈384÷FS[kHz]).
| 15 | 8   7 Digital Mixing COMPRESSED MSB LSB INPUT DATA Stereo digital output from the ADCs can be mixed digitally with the input to the DACs.  Digital output from the ADCs going | 0 |
| --- | --- | --- |
| 15 | 3/2     2/1 out of the serial port is unaffected by this digital mix.  Along the EXPANSION MSB LSB digital mix datapath, the 16-bit linear output from the ADCs is attenuated by an amount specified with Control Register bits. | 0 |
| 15 | 3/2    2/1 The level of attenuation applied to the left and right channels is DAC INPUT MSB LSB 000/00 independently programmable. (Note that internally the AD1843 always works with 16-bit PCM linear data, digital mixing in- Figure 4.μ-Law or A-Law Expansion cluded; format conversions take place at the input and output.) When 8-bit companding is specified, the ADCs’ linear output is Sixty-foursteps of –1.5 dB attenuation are supported to –94.5 dB. compressed to the format specified. The digital mix datapath can also be completely muted, pre- | 0 |
| venting any mixing of the analog input with the analog output. | 15 ADC OUTPUT MSB LSB Note that the level of the mixed signal is also a function of the input PGA settings, since they affect the ADCs’ output.  The | 0 |
| sample rate of the ADCs and the selected DAC pair must be the | 15 | 3/2    2/1 0 |
| same for the digital mix function to operate properly. | TRUNCATION MSB . The attenuated digital mix data is digitally summed with the | LSB |
| 15 | 8   7 DAC input data prior to the DACs’ datapath attenuators.  The | 0 |
| COMPRESSION MSB | LSB | 00000000 digital sum of digital mix data and DAC input data is clipped at plus or minus full scale and does not wrap around.  Because Figure 5.μ-Law or A-Law Compression both stereo signals are mixed before the output attenuators, Note that all format conversions take place at input or output. mix data is attenuated a second time by the DACs’ datapath attenuators. In case the AD1843 is playing back data but input Power Supplies and Voltage Reference |
| digital DAC data fails to arrive in time (“DAC underrun”), then | The AD1843 operates from either +5.0 V analog (V | ) and |
CC
a midscale zero will be added to the digital mix data in place of	digital (V	) power supplies or +5.0 V analog and +3.0 V digi-
DD
the unavailable DAC data. tal supplies.  Independent analog and digital supplies are recom-
Analog Outputs mended for optimal performance though excellent results can be
The two mixer line-level outputs are available at external pins. obtained in single-supply systems.  A voltage reference is included
Each output channel can be independently muted.  When on the Codec and its +2.25 V buffered output is available on an
muted, the outputs will settle to a dc value near CMOUT, the external pin (CMOUT).  The reference output can be used for
midscale reference voltage.  The two DAC2 stereo outputs are biasing op amps used in single supply systems.  The internal ref-
available at external pins differentially.  The full-scale level on erence is externally bypassed to analog ground at the VREFpin.
these pins is established by programming bits in a Control Reg- Clocks and Sample Rates
ister.  In addition, there is stereo headphone output (with a cur- The AD1843 operates from a single external clock or crystal
rent return), and a mono output.  Both the headphone output source. From a single clock, a wide range of sample rates can be
and the mono output have a single mute control. generated. When supplied with a single 24.576 MHz clock, the
Digital Data Types AD1843 can be programmed to generate any sample frequency
The AD1843 supports four data types: 16-bit twos-complement between 4 kHz and 54 kHz with 1 Hz resolution. For modem
linear PCM, 8-bitunsigned linear PCM, 8-bit compandedμ-law, sample rate support, the frequency programmed can also be in-
and 8-bit companded A-law, as specified by control register bits. creased by 8/7 using a control bit. All sample rate changes can
The data type is independently assignable for each conversion be made “on the fly.”
resource (i.e., ADCL, ADCR, DAC1 and DAC2).   Data in all The AD1843’s SYNC inputs can be used to synchronize the
four formats is always transferred MSB first. Eight-bit data is al- sampling activity of the four on-chip conversion resources to ex-
ways left-justified in 16-bit fields; said in other words, the MSBs ternal clock signals, such as video HSYNC or an ISDN network
of all data types are always aligned; in yet other words, full-scale clock. The SYNC inputs are used by three on-chip digital phase
–14– REV. 0

AD1843
lock loops, which can be arbitrarily assigned to the conversion four conversion channels; clock generator 1; clock generator 2;
resources. The lock range of these digital PLLs is 4 kHz to clock generator 3; conversion clock outputs 1 through 3; bit clock
54 kHz, which is the same range supported by the register- outputs 1 through 3; and the nominal 24.576 MHz clock output.
controlled clock generators. Refer to the descriptions of Control Register Address 27 and 28 for
further information.
If a SYNC input stops after its associated phase lock loop has
had a chance to initially lock, the AD1843 will continue to gen- For proper operation, the AD1843 must be calibrated following
erate a sample clock (as well as BIT clock and CONV clock) power-up. This initial calibration occurs automatically without any
very similar to the initial frequency, but off by at most±1%. user intervention or programming. Subsequent to this initial
The three SYNC inputs feed three on-chip Digital Phase Lock power-up autocalibration, there is no requirement to recalibrate the
Loops (DPLLs) which utilize a first-order loop filter with a SoundComm codec following software power-down sequences.
20 Hz corner frequency. Jitter frequencies above 20 Hz are The entire AD1843 or selected portions of the device may be
attenuated, and jitter frequencies below 20 Hz are interpreted as powered down, allowed to idle indefinitely, then powered up
time base drift, and are tracked. The DPLL provides 12 dB per and used immediately, without the need for repeated auto-
octave of jitter rejection. The DPLLs have been designed to tol- calibration. The digital information obtained during the initial
erate at least 2% Unit Interval (UI) of SYNC clock jitter. The power-up calibration is retained and valid unless theRESETor
DPLLs are critically damped at all input frequencies. PWRDWNpin is asserted, forcing a hardware reset. (If desired,
the user can specify that a calibration cycle occur when leaving
Power Management
the software power-down state by setting ACEN (Control Reg-
The AD1843 SoundComm codec has extensive power manage-
ister Address 28, Bit 14) to ”1.”) A hardware reset or power-
ment capabilities.  Hardware power down is performed using the
down clears the calibration information, and therefore a fresh
PWRDWNpin.  Software power management is programmed us-
autocalibration cycle is performed by the AD1843 following this
ing Control Register Address 27 and 28.  Several elements of the
event. Autocalibration takes approximately 4 ms to complete.
AD1843 can be powered down on a selective basis.  These blocks
include: the DAC2 to DAC1 analog mixer; the entire DAC1 con- The following table provides an indication of the power savings
version channel; the entire DAC2 conversion channel; the analog associated with powering-down the various resources in the
half of the ADC, DAC1 and DAC2; the headphone driver; the en- AD1843. Note that the power savings is somewhat order-
tire analog mixer; the right ADC channel; the left ADC channel; all
Table I.  AD1843 Power-Down Savings
+5 V Digital, +5 V Analog Supplies Total Active Operation Current:  200 mA
Average, Typical Average, Typical
Absolute IDD+ Normalized
| Software Power Down | Control Register Bit(s) | ICCCurrent | Power Savings |
| --- | --- | --- | --- |
| CLKOUT Output | ENCLKO Bit = 0 All Bit Clocks and ENBT3, ENBT2, ENBT1 Bits = 0 | 8 mA | 4% |
| All Conversion Clocks | ENCV3, ENCV2, ENCV1 Bits = 0 | 2 mA | 1% |
| Clock Generator 1 | C1EN Bit = 0 | 6 mA | 3% |
| Clock Generator 2 | C2EN Bit = 0 | 6 mA | 3% |
| Clock Generator 3 | C3EN Bit = 0 | 6 mA | 3% |
| All Clock Generators | C1EN, C2EN, C3EN Bits = 0 | 20 mA | 10% |
| Headphone Driver | HPEN Bit =  0 | 8 mA | 4% |
| DAC2 to DAC1 Mix | DDMEN Bit = 0 | 2 mA | 1% |
| Analog Input to Analog Output Mix | AAMEN Bit = 0 | 8 mA | 4% |
| ADC Left Channel | ADLEN Bit = 0 | 8 mA | 4% |
| ADC Right Channel | ADREN Bit = 0 | 8 mA | 4% |
| ADC Left and Right Channels | ADLEN, ADREN Bits = 0 | 38 mA | 17% |
| DAC2 (Left and Right Channels) | DA2EN Bit = 0 | 30 mA | 15% |
| DAC1 (Left and Right Channels) | DA1EN Bit = 0 | 24 mA | 12% |
| DAC2 AND DAC1 (Left and Right Chs) | DA2EN, DA1EN Bits = 0 ADC and DAC2 and DAC1 ADLEN, ADREN, | 60 mA | 30% |
| DA2EN, DA1EN Bits = 0 | 108 mA | 54% |  |
| Analog Channel | ANAEN Bit = 0 All Control Register 27 HPEN, DDMEN, AAMEN, ADLEN, | 54 mA | 27% |
| ADREN, DA2EN, DA1EN, ANAEN Bits = 0 | 134 mA | 67% |  |
| Converter | PDNI Bit = 1 All of the Above (Register 27 and ENCLKO, ENBT3, ENBT2, ENBT1, ENCV3, Clocks and PDNI) ENCV2, ENCV1, C1EN, C2EN, C3EN, HPEN, DDMEN, AAMEN, ADLEN, ADREN, DA2EN, | 140 mA | 70% |
| DA1EN, ANAEN Bits = 0, PDNI Bit = 1 | 176 mA REV. 0 –15– | 88% |  |

AD1843
dependent; depending upon the sequence in which the hardware sion resources, as long as the DFREE bit (Control Register Ad-
resources are powered down, the savings may be more or less dress 27) is asserted (i.e., set to “1”). If DFREE is not asserted,
than the typical numbers given. then the maximum sampling rate for the remaining conversion
resources is 49 kHz.
Mode Changing
In general, there are very few restrictions with respect to chang- Digital Filter Selection
ing the operating mode of the AD1843. Because of the advanced The operative digital filter modes for the four conversion re-
ContinuousTime Oversampling technology, the waiting period sources on the AD1843 SoundComm are programmed using
associated with changes to the sample rate of the data converters Control Register Address 25. ADLFLT (Bit 0) selects the digi-
(“Mode Change Enable” resynchronization delay) is eliminated. tal filter mode for the ADC left channel and ADRFLT (Bit 1)
The only waiting periods associated with the AD1843 occur at selects the digital filter mode for the ADC right channel. Note
start-up, and are documented in the “START-UP SEQUENCE” that these bits also establish the full-scale input voltage range for
section below. Following the start-up sequence, the sample rate these channels as well. DA1FLT (Bit 8) selects the DAC1 digi-
of the four data conversionresources on the AD1843 may be tal filter mode, and DA2FLT (Bit 9) selects the DAC2 digital
changed at any time, on-the-fly (presuming that they are filter mode. Note that these bits also establish the full-scale out-
enabled). All gain, mute and attenuation settings of enabled put voltage for these channels as well.
resources may also be changed at any time.
The three digital filter modes are audio, modem and resampler.
Channel Synchronization The specifications for these modes are given in the description
If multiple AD1843s are used in a daisy-chained system, and it of Control Register Address 25, as well as in the “SPECIFICA-
is desired to synchronize data conversion activity among the TIONS” section of this data sheet. The specifications have been
multiple codecs, the clock generator blocks of the AD1843s made to satisfy the demands of the applications which the
must be enabled on the same frame (see step 5 in the “START- AD1843 can serve. The audio mode provides decimation and
UP SEQUENCE” section below). interpolation  characteristics  sufficient  for  high  quality  cap-
ture and playback of material from 20 Hz to 20 kHz. The mo-
A DAC channel does not actually start processing samples until
dem mode provides characteristics sufficient for modulation
the first rising edge of the conversion clock (CONV pin) after
standards up to V.34 quality. The resampling mode provides
the sixth rising edge of frame sync (SDFS pin) after the channel
optimal characteristics for high quality sample rate conversion.
is enabled (via a write to DA1EN or DA2EN in Control Regis-
While in the resampling mode, all images in the resampled data
ter Address 27). The wait until the sixth rising edge of frame
stream (including those in the transition band) are attenuated to
sync is necessary to allow the four deep DAC FIFO to be filled
below the quantization noise floor. Note that the maximum
before conversion commences. The subsequent wait until the
sample rate for modem mode is 24 kHz.
rising edge of the conversion clock is necessary to synchronize
the serial interface based DAC channel enable command with a Digital Resampling
conversion clock that is potentially already running (which is Digital resampling is best achieved by routing the digital output
particularly likely if the SYNC pin inputs and lock mode are of one of the DACs back to the digital input of one of the
in use). ADCs. This bypasses the analog portion of the DAC and ADC,
eliminating their noise and signal delay contributions. This fea-
The ADC channels behave very similarly to the DAC channels.
ture is enabled by bits DAADR1:0 (Digital ADC Right Channel
An ADC channel does not actually start taking samples until the
Source Select) and DAADL1:0 (Digital ADC Left Channel
first rising edge of the conversion clock (CONV pin) after the
Source Select) in Control Register Address 25.
sixth rising edge of frame sync (SDFS pin) after the channel is
enabled (via a write to ADLEN or ADREN in Control Register If the “Digital Resampler Filter Mode” (DRSFLT bit = “1,”
Address 27). The wait until the sixth rising edge of frame sync is Control Register Address 25) is enabled, the DAC2 pair is sacri-
present so that the ADC startup is similar to that of the DAC ficed, but the remaining four channels (ADC left and right,
startup, as well as to allow some time for stale ADC data inside DAC1 left and right) can still be used in any way they could
the AD1843 to be cleared. The subsequent wait until the rising have been when not in “Digital Resampler Filter Mode.” When
edge of the conversion clock is necessary to synchronize the in this mode, internal AD1843 hardware normally devoted to
serial interface based ADC channel enable command with a DAC2 is reallocated to the other four channels, allowing these
conversion clock that it potentially already running (which is par- channels to realize superior digital filtering. Note that the
ticularly likely if the SYNC pin inputs and lock mode are in use). AD1843 DOES NOT actually have to be in digital resampler
filter mode to perform digital resampling, however the superior
Supported Conversion Rates
digital filters in this mode allow for a much higher quality digital
With all conversion channels operating (i.e., ADC left, ADC
resampling.
right, DAC1 and DAC2), the AD1843 is able to support sam-
pling rates up to 49 kHz, which 2.1% higher than the nominal Using the AD1843 in a Modem Application
maximum audio standard of 48 kHz, to accommodate timebase The AD1843 analog performance is sufficient to support the
drift while configured in slave mode. If either one DAC (i.e., modem Analog Front End (AFE) function, for data modulation
either DAC1 or DAC2) or both ADC channels (i.e., ADC left standards up to and including the 28.8 kbps V.34 ITU stan-
and ADC right) are shut down, then the AD1843 can support dard. The data pump function is performed in a companion
sampling up to 54 kHz on all channels of the remaining conver- DSP, such as the ADSP-2181, for which several V.34 algo-
rithms (from third party Independent Algorithm Vendors) exist.
–16– REV. 0

AD1843
PC ATTENTION
“BEEPER” SIGNAL
EXTERNAL
WAVEFORM AD1843 SOUNDCOMM
SYNTHESIZER CODEC
NTSC
LINE IN	SYNC2	HORIZONTAL
SYNC SIGNAL
MIC IN RIGHT
MIC IN LEFT CONV1
AUDIO FROM
AUX1 IN
AUDIO FROM	CD-ROM	BIT1
EXTERNAL MPEG AUX2 IN
DECODER
AUX3 IN
ADSP-21xx
MONO IN
DSP OR ASIC
DAA HOST BUS
SUM IN ISA OR PCI
SERIAL SERIAL
INTERFACE INTERFACE
PSTN LINE1 OUT
HEADPHONE OUT
IDMA PORT
LINE2 OUT RIGHT OR PARALLEL
PORT
LINE2 OUT LEFT
MONO OUT
PC SPEAKER
AUDIO FROM DAT
OR CASSETTE
EXTERNAL POWERED
MULTIMEDIA SPEAKERS
SPEAKERPHONE
Figure 6.  Typical Configurations
SAMPLE
SAMPLE PERIOD N+1 PERIOD N+3
| SAMPLE PERIOD N | SLOT 16 | SLOT 31 | SAMPLE PERIOD N+2 | SLOT 16 |
| --- | --- | --- | --- | --- |
| SLOT 0 | SLOT 15 | SLOT 0 | SLOT 15 | 256 BITS 256 BITS 256 BITS 256 BITS |
| SDI OR SDO | 321015141312 | 321015141312 | 321015141312 | 321015141312 |
| MSB | MSB | MSB | MSB |  |
SCLK
512 BITS 512 BITS
FRAME M
FRAME M+1
SDFS
FRS = 0     [DEFAULT 32 SLOTS PER FRAME, 2 SAMPLES PER FRAME SYNC]
MASTER MODE
NOTE THAT AD1843 FRAME RATE IS NOT RELATED TO SAMPLE RATES
Figure 7.  FRS = 0, Master Mode Timing
SAMPLE
SAMPLE PERIOD N+1 PERIOD N+3
| SAMPLE PERIOD N | SLOT 0 | SLOT 15 | SAMPLE PERIOD N+2 | SLOT 0 |
| --- | --- | --- | --- | --- |
| SLOT 0 | SLOT 15 | SLOT 0 | SLOT 15 |  |
| SDI OR SDO | 321015141312 | 321015141312 | 321015141312 | 321015141312 |
| MSB | MSB | MSB | MSB |  |
SCLK
256 BITS	256 BITS	256 BITS
FRAME M FRAME M+2
FRAME M+1 FRAME M+3
SDFS
FRS = 1     [16 SLOTS PER FRAME, 1 SAMPLE PER FRAME SYNC]
MASTER MODE
NOTE THAT SCLK CAN BE PROGRAMMED FOR EITHER 12.288 MHz
OR 16.384 MHz WHEN IN MASTER  MODE
Figure 8.  FRS = 1, Master Mode Timing
REV. 0 –17–

AD1843
SAMPLE PERIOD N+1
| SAMPLE PERIOD N | SLOT 16 | SLOT 31 | SAMPLE PERIOD N+2 |
| --- | --- | --- | --- |
| SLOT 0 | SLOT 15 256 BITS | SLOT 0 | SLOT 15 |
| GAP | 256 BITS | GAP | 256 BITS |
| SDI OR SDO | 15   14   13   12 | 321015141312 | 3210 15141312 321015 |
| MSB | MSB | MSB | MSB |
SCLK
512 BITS 512 BITS
FRAME M
FRAME M+1
TSI
FRS = 0     [DEFAULT 32 SLOTS PER FRAME, 2 SAMPLES PER FRAME SYNC]
EXAMPLE SHOWING GAPS BETWEEN FRAMES
SLAVE MODE
Figure 9a.  FRS = 0, Slave Mode Timing
SAMPLE PERIOD N+1
| SAMPLE PERIOD N | SLOT 16 | SLOT 31 | SAMPLE PERIOD N+2 |
| --- | --- | --- | --- |
| SLOT 0 | SLOT 15 256 BITS 256 BITS 256 BITS | SLOT 0 | SLOT 15 |
| SDI OR SDO | 321015141312 | 321015141312 | 321015141312 32101514 |
| MSB | MSB | MSB | MSB |
SCLK
512 BITS 512 BITS
FRAME M
FRAME M+1
TSI
FRS = 0     [DEFAULT 32 SLOTS PER FRAME, 2 SAMPLES PER FRAME SYNC]
EXAMPLE SHOWING NO GAPS BETWEEN FRAMES
SLAVE MODE
Figure 9b.  FRS = 0, Slave Mode Timing
SAMPLE PERIOD N SAMPLE PERIOD N+1
| SLOT 0 | SLOT 15 | SLOT 0 | SLOT 15 |
| --- | --- | --- | --- |
| GAP | GAP | GAP |  |
| SDI OR SDO | 15   14   13   12 | 3210 | 15   14   13   12 3210 MSB MSB |
SCLK
256 BITS 256 BITS
FRAME M
FRAME M+1
TSI
FRS = 1     [16 SLOTS PER FRAME, 1 SAMPLES PER FRAME SYNC]
EXAMPLE SHOWING GAPS BETWEEN FRAMES
SLAVE MODE
Figure 10a.  FRS = 1, Slave Mode Timing
SAMPLE PERIOD N+1
| SAMPLE PERIOD N | SLOT 0 | SLOT 15 SLOT 0 SLOT 15 |
| --- | --- | --- |
| SDI OR SDO | 321015   14   13   12 | 321015   14   13   12 321015   14   13   12 |
| MSB | MSB | MSB |
SCLK
256 BITS 256 BITS
FRAME M
FRAME M+1
TSI
FRS = 1     [16 SLOTS PER FRAME, 1 SAMPLES PER FRAME SYNC]
EXAMPLE SHOWING NO GAPS BETWEEN FRAMES
SLAVE MODE
Figure 10b.  FRS = 1, Slave Mode Timing
–18– REV. 0

AD1843
Modem Data Access Arrangement (DAA) devices are generally Use the ADREN bit in Control Register Address 27 for the
differential on the transmit side, and single-ended on the receive ADC right channel, the DAC1EN bit in Control Register
side. The DAA transmit input (generally differential) should be Address 27 for DAC1, and the DAC2EN bit in Control Regis-
connected to the DAC2 output, pins LOUT2LP and LOUT2LN, ter Address 27 for DAC2.
or LOUT2RP and LOUT2RN. The DAA receive output
Typical Configurations
(generally single-ended) should be connected to one of the
Figure 6 below illustrates example connections between the
ADC line inputs, LINLP or LINRP. See the “APPLICATION
AD1843 SoundComm codec and other system resources. The
CIRCUITS” section below for more detail on the electrical
rich analog input and output connectivity of the AD1843 allows
connections. There are several software driver steps that are re-
a wide variety of configuration possibilities. Note that the level
quired to configure the SoundComm codec for use as a modem
of modem, speakerphone and external speaker concurrency is
AFE.
application and DSP resource dependent.
Configure DAC2
1.  Set the DA2FLT bit (Control Register Address 25, Bit 9) to SERIAL INTERFACE
“1,” to select the digital modem filter mode. The DAC2 out- The AD1843 SoundComm Codec transmits and receives both
puts can be used either as differential outputs or single-ended data and control/status information through its serial port.
outputs depending on how the pins are connected electrically;
The AD1843 can be configured as either master or slave of the
no Control Register writes are required to configure the DAC2
serial interface.  This is selected by using the BM pin.  When
outputs as either differential or single-ended.
BM is tied HI, the AD1843 serves as bus master and supplies
2.  Program LDA2G5:0 (Control Register Address 10, Bits 8 the frame sync and the serial clock.  When BM is tied LO, the
through 13) to “00 0101” (i.e., +4.5 dB) or RDA2G5:0 AD1843 serves as bus slave and receives the frame sync and the
(Control Register 10, Bits 0 through 5) to “00 0101” (i.e., serial clock.  The level on BM should not be altered unless the
+4.5 dB), depending on whether the DAA transmit input is reset pin (RESET) is asserted.
connected to the left channel DAC2 output (useLDA2G5:0)
The AD1843 has six pins devoted to the serial interface: SDI,
or the right channel DAC2 output (use RDA2G5:0). This
SDO, SCLK, SDFS, TSI and TSO.  The SDI pin is for serial
code establishes the DAC2 nominal analog output swing at
data input to the AD1843 and the SDO pin is for serial data
3.156 V p-p single-ended, or 6.312 V p-p differentially. The
output from the AD1843.  The SCLK pin is the serial interface
3.156 V p-p level is equivalent to 3.17 dBm.
clock.  Communication in and out of the AD1843 requires bits
Configure ADC of data to be transmitted after a rising edge of SCLK, and
1.  Set the ADLFLT bit (Control Register Address 25, Bit 0) to sampled on a falling edge of SCLK.  When the AD1843 is bus
“1,” or the ADRFLT bit (Control Register Address 25, Bit 1) master (BM pin tied HI), the SCLK frequency driven by the
to “1,” to select the digital modem filter mode. Set ADLFLT AD1843 will be 12.288 MHz by default, but this can be in-
if the DAA receive output is connected to the AD1843 creased to 16.384 MHz by setting the SCF bit in Control Regis-
LINLP input; set ADRFLT if the DAA receive output is ter 26.  When the AD1843 is bus slave (BM pin tied LO), the
connected to the AD1843 LINRP input. Set the LINLSD bit SCLK frequency driven to the AD1843 may be as high as
(Control Register Address 28, Bit 0) to “1” if the DAA is 24.576 MHz, but must not be any higher than the frequency on
connected to the AD1843 LINLP input; set the LINRSD bit the XTALI pin.
(Control Register Address 28, Bit 1) to “1” if the DAA is
The SDFS pin is for the serial interface frame sync.  When bus
connected to the AD1843 LINRP input.
master, new frames are marked by a HI pulse driven out on
2.  Program LIG3:0 (Control Register Address 2, Bits 8 through SDFS one serial clock period before the frame begins.  When
11) to “0000” (i.e., 0.0 dB) or RIG3:0 (Control Register bus slave, new frames must be marked by a LO to HI transition
Address 2, Bits 0 through 3) to “0000” (i.e., 0.0 dB) de- driven in on SDFS one serial clock period before the frame be-
pending on whether the left or right ADC input channel is gins, but the transition back from HI to LO may occur at any
being used for the modem function. This code maps an ana- time provided the HI and LO times of SDFS are at least one
log input swing of 3.156 V p-p to the full dynamic range of SCLK period in duration each.
15
the 16-bit digital sample (i.e.,±2 ). The 3.156 V p-p level is
When the AD1843 is bus master, frame size is controlled by the
equivalent to 3.17 dBm.
FRS bit in Control Register 26.  When FRS is set to “1,” each
Note that if the AD1843 is to be reconfigured dynamically, the frame is divided into 16 slots of 16 bits.  When FRS is reset to
affected converter must be powered down before its associated “0,” each frame is divided into 32 slots of 16 bits.  In 32 slot
digital filter can be changed. In other words, if the digital filter configuration, the second 16 slots of a frame must have slot as-
for the ADC left channel is being changed from audio mode to signments that are identical to the first 16 slots of the frame; 32
modem mode, the ADC left channel must be powered down slot configuration is essentially 16 slot configuration with every
first (using the ADLEN bit in Control Register Address 27). other SDFS pulse missing.  Although these are the frame sizes
REV. 0 –19–

AD1843
produced by an AD1843 serving as bus master, an AD1843 for Control Register write data input and read data output.  The
serving as bus slave does not actually require these frame sizes. remaining slots are used for playback (DAC) data input and
When FRS is set to “1,” a slave will operate correctly with any capture (ADC) data output, where each channel has an assigned
number or fraction of slots, provided there are enough slots for slot.  Table II and Figure 11 illustrate these slot assignments.
it to complete its necessary communication (see below).  When
Since the conversion channels of the AD1843 can be pro-
FRS is reset to “0,” a slave can also operate correctly with a
grammed to run at different sample rates, a communication
wide range in the number of slots per frame, however it will au-
mechanism indicates when playback channels request data,
tomatically retake ownership of the serial interface bus 16 slots
when playback data is actually sent to the AD1843, and when
after it is first given ownership of the bus in a frame.
transmission of capture data from the AD1843 becomes neces-
The nominal minimum number of slots when the AD1843 is sary.  This is facilitated by the Control and Status Words lo-
configured in slave mode is six. The codec must be supplied cated in the first slot.  The Control Word indicates which slots
with at least 6×16 = 96 SCLK periods (both rising and falling in the current frame contain valid playback data.  The Status
edges); SCLK may be gated (i.e., no need to be continuous) Word indicates if playback data can be sent to the AD1843 dur-
between valid slots. ing the next frame, and which slots in the current frame contain
valid capture data.  See the descriptions of the Control Word
While SDFS marks the beginning of frames, AD1843 bus own-
and the Status Word below for additional detail.
ership during a frame is controlled by the TSI (Time Slot In)
and TSO (Time Slot Out) pins.  When bus slave, a level HI on Four word FIFO buffers are used on the inputs of each of the
TSI grants the AD1843 bus ownership beginning with the next DACs to allow data to be transferred in small bursts.  This re-
SCLK period.  The TSI pin is monitored only when anAD1843 duces the required response time to playback data requests, and
does not already own the bus; once an AD1843 is given owner- also buffers differences between the frame sync rate and the
ship of the bus, the level on TSI is ignored until one SCLK channel sample rate.  The Status Word indicates that playback
period before bus ownership is relinquished.  Bus ownership will data can be sent if there is any room in the buffers, thus tending
last for six slots.  Coincident with the final SCLK period of the to keep the input buffers full.  Underrun flags are available in
final slot owned, the AD1843 asserts TSO HI.  This allows Control Register 1, which indicate if an input buffer ran out of
chaining of AD1843s onto a common serial bus by connecting data.  If an underrun occurs, a zero is used in place of the un-
the TSO pin of one AD1843 to the TSI pin on the next later available data. To ensure underruns do not occur, playback data
AD1843 in a chain. In single codec systems where the must be sent to the AD1843 within two sample periods after the
SoundComm isconfigured as bus slave, connect the AD1843 status word indicates that the DAC FIFO is not full.
SDFS and TSI signals together. When an AD1843 is bus mas-
Note that the DAC Not Full status bits (DA2RQ and DA1RQ
ter, its function is identical to that just described for the slave,
in the Status Word Output) are updated immediately (i.e., in
except a bus master always owns the first six slots and its TSI
the same frame as a valid write to the DAC FIFOs). If the DAC
pin is ignored (but should be tied LO).
Input Valid Flags (DA2V and DA1V in the Control Word
Whenever an AD1843 does not own the bus, its SDO pin will Input)are set (i.e., DAC data is valid) and only one location in
be three-stated and its SDI pin is ignored.  Figures 7 through 10 the DAC1 and DAC2 input FIFOs is available, then the
illustrate the signal, slot, sample and frame relationships for the DA2RQ and DA1RQ status bits will reflect this valid write, and
four basic operating modes of the AD1843 serial interface. will be reset to “0.” This is possible because the DA2V and
DA1V bits are in the most significant bits of the Control Word
The AD1843 requires slots of communication each time it takes
and the DA2RQ and DA1RQ bits are in the least significant bits
ownership of the serial bus. The first slot is used for a Control
of the Status Word, and the AD1843 uses this intervening time
Word input and a Status Word output.  The second slot is used
Table II.  AD1843 Slot Assignment
32 Slot Mode (FRS Reset to “0”)
| Slot | SDI Pin | SDO Pin |
| --- | --- | --- |
| 0 & 16 | Control Word Input | Status Word Output |
| 1 & 17 | Control Register Data Input | Control Register Data Output |
| 2 & 18 | Playback Data Input—DAC1 Left | Capture Data Output—ADC Left |
| 3 & 19 | Playback Data Input—DAC1 Right | Capture Data Output—ADC Right |
| 4 & 20 | Playback Data Input—DAC2 Left | Reserved (Unused) |
| 5 & 21 | Playback Data Input—DAC2 Right 16 Slot Mode (FRS Set to “1”) | Reserved (Unused) |
| Slot | SDI Pin | SDO Pin |
| 0 | Control Word Input | Status Word Output |
| 1 | Control Register Data Input | Control Register Data Output |
| 2 | Playback Data Input—DAC1 Left | Capture Data Output—ADC Left |
| 3 | Playback Data Input—DAC1 Right | Capture Data Output—ADC Right |
| 4 | Playback Data Input—DAC2 Left | Reserved (Unused) |
| 5 | Playback Data Input—DAC2 Right –20– REV. 0 | Reserved (Unused) |

AD1843
interval to update the DAC Not Full status bits. Therefore mode, or half as high as the sample rate in 32 slot per frame
driver software does not have to make provision for frame-to- mode.  When the AD1843 is bus master, the SCLK frequency
frame delays between control and status information; the infor- is either 12.288 MHz or 16.384 MHz, which allows for up to
mation in each frame is always up to date. 48 kHz or 64 kHz sampling rates, respectively.
The AD1843 supports locking to an external clock which may The AD1843 Control Registers are read and written by trans-
result in a sample rate that is marginally higher than the nominal mitting a read/write request bit along with the Control Register
audio standard maximum sample rate of 48 kHz.  This is neces- address in the slot 0 Control Word.  When a read is requested,
sary since two crystal-based clock sources are neverperfectly the contents of the Control Register addressed is transmitted
matched to one another.  One is always at a slightly higher fre- out during slot 1 of the following frame.  When a write is re-
quency.  The AD1843 conversion channels have been designed quested, data to be written must be transmitted to the AD1843
to support sample rates that are up to 2.1% higher than 48 kHz in slot 1, and the former contents of the control register are
(i.e., 49 kHz), referenced to the AD1843’s clock input on transmitted out during slot 1 of the following frame. Unless oth-
XTALI, when all conversion channels are simultaneously run- erwise noted, Control Register writes do not take effect until the
ning.  Even higher rates can be supported when all channels are current round of six communication TDM time slots concludes.
not running simultaneously.  See the “Conversion Rates” sec- Equivalently, unless otherwise noted, Control Register writes do
tion above for additional details.  The serial interface must also not take effect until the subsequent falling edge of the TSO signal.
allow for this higher sample rate with a frame sync (SDFS) rate
The following sections describe the bit assignments for all time
that is at least as high as the sample rate in 16 slot per frame
slots.
TIME	TIME	TIME	TIME	TIME	TIME
SLOT 0	SLOT 1	SLOT 2	SLOT 3	SLOT 4	SLOT 5
SCLK
16 BITS
SDFS
SDI	IGNORED	CONTROL WORD REGISTER DATA	DAC1  LEFT	DAC1 RIGHT	DAC2 LEFT	DAC2 RIGHT	IGNORED
16-BIT
STEREO	SDO	3-STATED	STATUS WORD	REGISTER DATA	ADC LEFT	ADC  RIGHT	0s	0s	3-STATED
SDI	IGNORED	CONTROL WORD REGISTER DATA	DAC1	DON'T CARE	DAC2	DON'T CARE	IGNORED
16-BIT
MONO
SDO	3-STATED	STATUS WORD	REGISTER DATA	ADC LEFT	ADC  RIGHT	0s	0s	3-STATED
DAC1, DAC2
| DON'T | DON'T | DON'T | DON'T |
| --- | --- | --- | --- |
| SDI | IGNORED | CONTROL WORD REGISTER DATA DAC1L | DAC1R DAC2L DAC2R IGNORED |
| 8-BIT | CARE | CARE | CARE CARE |
STEREO
SDO	3-STATED	STATUS WORD	REGISTER DATA	ADCL	0s	ADCR	0s	0s	0s	3-STATED
DON'T DON'T
8-BIT	SDI	IGNORED	CONTROL WORD REGISTER DATA	DAC1	DON'T CARE	DAC2	DON'T CARE	IGNORED
CARE CARE
MONO
DAC1, DAC2 SDO	3-STATED	STATUS WORD	REGISTER DATA	ADCL	0s	ADCR	0s	0s	0s	3-STATED
THE DIAGRAM ABOVE IS INTENDED TO BE ILLUSTRATIVE OF THE MORE COMMONLY USED CONFIGURATIONS. ADC LEFT, ADC RIGHT, DAC1
AND DAC2 CAN BE INDIVIDUALLY ASSIGNED TO 8-BIT OR 16-BIT SAMPLE WIDTH, AND DAC1 AND DAC2 CAN BE INDIVIDUALLY ASSIGNED
TO STEREO OR MONO MODE. EACH AD1843 CONSUMES 6 TMD SLOTS (REQUIRING 96 SCLK PERIODS), LEAVING 10 TDM SLOTS
UNUSED IN A 16 SLOT FRAME.
NOTE THAT BECAUSE THE SERIAL INTERFACE AND THE ADC AND DACS ARE IN GENERAL ASYNCHRONOUS, NOT EVERY CAPTURE OR
PLAYBACK TIME SLOT WILL CONTAIN VALID DATA. THE HOST PROCESSOR MUST POLL THE STATUS WORD TO DETERMINE WHETHER
THE ADC DATA IS VALID AND WHETHER THE DAC IS REQUESTING ADDITIONAL SAMPLES.
Figure 11.  AD1843 Slot Assignments
REV. 0 –21–

AD1843
SERIAL INTERFACE INPUT
Note that the references to slot numbers are valid only when the AD1843 is configured in master mode. For slave mode, bus owner-
ship does not necessarily start on Slot 0.
Control Word Input (Slot 0 or 16)
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| res | res | res | res | res | res | DA2V | DA1V |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| R/W | res DA2V DAC2 Input Valid Flag.  When in DAC2 stereo mode, setting this bit to “1” indicates that slots 4 and 5 contain valid playback data for the left and right channels of DAC2, respectively.  When in DAC2 mono mode, setting this bit to “1” indicates that slot 4 contains valid playback data for both left and right channels of DAC2.  Slot 5 is ignored in DAC2 mono mode.  When this bit is reset to “0,” data in slots 4 and 5 is ignored.  This bit is ignored if the AD1843 did not request data for DAC2 in the last frame (see the DA2RQ bit in the Status Word Output). DA1V DAC1 Input Valid Flag.  When in DAC1 stereo mode, setting this bit to “1” indicates that slots 2 and 3 contain valid playback data for the left and right channels of DAC1, respectively.  When in DAC1 mono mode, setting this bit to “1” indicates that slot 2 contains valid playback data for both left and right channels of DAC1.  Slot 3 is ignored in DAC1 mono mode.  When this bit is reset to “0,” data in slots 2 and 3 is ignored.  This bit is ignored if the AD1843 did not request data for DAC1 in the last frame (see the DA1RQ bit in the Status Word Output). R/W Read/Write Request.  Either a read from or a write to a Control Register occurs every frame.  Setting this bit to “1” indicates a Control Register read while resetting this bit to “0” initiates a Control Register write.  Bits IA4:0 define the Control Register address.  When reading, the contents of the Control Register addressed are transmitted during slot 1 of the following frame.  When writing, the data to be written is taken from slot 1 and the former contents of the Control Register are transmitted during slot 1 of the following frame. IA4:0 Control Register address for read or write. res Reserved for future expansion.  To ensure future compatibility, write “0” to all reserved bits. Control Register Write Data Input (Slot 1 or 17) | res | IA4 | IA3 | IA2 | IA1 | IA0 |
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| DATA15 | DATA14 | DATA13 | DATA12 | DATA11 | DATA10 | DATA9 | DATA8 |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| DATA7 | DATA6 Format for the input data to written to the addressed Control Register. MSB is first. DAC1 Left Sample Input (Slot 2 or 18) | DATA5 | DATA4 | DATA3 | DATA2 | DATA1 | DATA0 |
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| DATA15 | DATA14 | DATA13 | DATA12 | DATA11 | DATA10 | DATA9 | DATA8 |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| DATA7 | DATA6 Data format may be 8-bit unsigned linear PCM, 16-bit signed linear PCM, 8-bitμ-Law companded, or 8-bit A-Law companded. MSB is first. DATA7:0 are ignored in 8-bit linear or 8-bit companded modes. –22– REV. 0 | DATA5 | DATA4 | DATA3 | DATA2 | DATA1 | DATA0 |

AD1843
DAC1 Right Sample Input (Slot 3 or 19)
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| DATA15 | DATA14 | DATA13 | DATA12 | DATA11 | DATA10 | DATA9 | DATA8 |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| DATA7 | DATA6 Data format may be 8-bit unsigned linear PCM, 16-bit signed linear PCM, 8-bitμ-Law companded, or 8-bit A-Law companded. MSB is first. DATA7:0 are ignored in 8-bit linear or 8-bit companded modes. DAC2 Left Sample Input (Slot 4 or 20) | DATA5 | DATA4 | DATA3 | DATA2 | DATA1 | DATA0 |
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| DATA15 | DATA14 | DATA13 | DATA12 | DATA11 | DATA10 | DATA9 | DATA8 |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| DATA7 | DATA6 Data format may be 8-bit unsigned linear PCM, 16-bit signed linear PCM, 8-bitμ-Law companded, or 8-bit A-Law companded. MSB is first. DATA7:0 are ignored in 8-bit linear or 8-bit companded modes. DAC2 Right Sample Input (Slot 5 or 21) | DATA5 | DATA4 | DATA3 | DATA2 | DATA1 | DATA0 |
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| DATA15 | DATA14 | DATA13 | DATA12 | DATA11 | DATA10 | DATA9 | DATA8 |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| DATA7 | DATA6 Data format may be 8-bit unsigned linear PCM, 16-bit signed linear PCM, 8-bitμ-Law companded, or 8-bit A-Law companded. MSB is first. DATA7:0 are ignored in 8-bit linear or 8-bit companded modes. REV. 0 –23– | DATA5 | DATA4 | DATA3 | DATA2 | DATA1 | DATA0 |

AD1843
SERIAL INTERFACE OUTPUT
Status Word Output (Slot 0 or 16)
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| res | res | res | res | res | res | ADRV | ADLV |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| res | res | res | res | res | res | DA2RQ ADRV Right ADC Channel Output Valid Flag.  Set to “1” if slot 3 (or 19) contains valid right ADC capture data. ADLV Left ADC Channel Output Valid Flag.  Set to “1” if slot 2 (or 18) contains valid left ADC capture data. DA2RQ DAC2 Input Request Flag.  Set to “1” if DAC2 is enabled and its four word stereo input buffer is not full. DA1RQ DAC1 Input Request Flag.  Set to “1” if DAC1 is enabled and its four word stereo input buffer is not full. res Reserved for future expansion.  Read back as “0.”  Should be ignored to ensure future compatibility. Control Register Data Output (Slots 1 or 17) | DA1RQ |
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| DATA15 | DATA14 | DATA13 | DATA12 | DATA11 | DATA10 | DATA9 | DATA8 |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| DATA7 | DATA6 Contents of Control Register addressed in previous frame. ADC Left Sample Output (Slots 2 or 18) | DATA5 | DATA4 | DATA3 | DATA2 | DATA1 | DATA0 |
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| DATA15 | DATA14 | DATA13 | DATA12 | DATA11 | DATA10 | DATA9 | DATA8 |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| DATA7 | DATA6 Data format may be 8-bit unsigned linear PCM, 16-bit signed linear PCM, 8-bitμ-Law companded, or 8-bit A-Law companded. MSB is first. DATA7:0 are ignored in 8-bit linear or 8-bit companded modes. ADC Right Sample Output (Slots 3 or 19) | DATA5 | DATA4 | DATA3 | DATA2 | DATA1 | DATA0 |
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| DATA15 | DATA14 | DATA13 | DATA12 | DATA11 | DATA10 | DATA9 | DATA8 |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| DATA7 | DATA6 Data format may be 8-bit unsigned linear PCM, 16-bit signed linear PCM, 8-bitμ-Law companded, or 8-bit A-Law companded. MSB is first. DATA7:0 are ignored in 8-bit linear or 8-bit companded modes. –24– REV. 0 | DATA5 | DATA4 | DATA3 | DATA2 | DATA1 | DATA0 |

AD1843
Reserved Output (Slots 4 or 20)
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| res | res | res | res | res | res | res | res |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| res | res res Reserved for future expansion.  Read back as “0.” Should be ignored to ensure future compatibility. Reserved Output (Slots 5 or 21) | res | res | res | res | res | res |
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| res | res | res | res | res | res | res | res |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| res | res res Reserved for future expansion.  Read back as “0.” Should be ignored to ensure future compatibility. CONTROL REGISTERS Control Register Architecture All Control Registers are cleared to their default state when either theRESETpin or thePWRDWNpin is asserted.  All conversion related Control Registers (Addresses 1–15, 25 and 27) are cleared to defaults when the conversion resources of the AD1843 are pow- ered down through the PDNI bit in Control Register Address 28.  Control Registers which manage analog DAC gain/attenuation (Addresses 3–10) are cleared to defaults whenever the resource they manage is powered down.  Finally, the three clock generator sample phase shift Control Registers (Addresses 18, 21 and 24) are cleared to defaults whenever their associated clock generator is powered down.  Writes to Control Registers are blocked until a clearing condition no longer exists.  Writes to Control Registers are also blocked immediately after theRESETpin or thePWRDWNpin is asserted until the AD1843’s internal clocks have settled, which is indicated by the INIT bit in Control Register Address 0.  Control Registers may always be read. Figures 12 and 13 associate the Control Registers to the AD1843 Block Diagram. REV. 0 –25– | res | res | res | res | res | res |

AD1843
LINLP
LINLN
LINRP
LINRN
MICL AD1843
CRA2
MICR
20 dB S
ECRA2
AUX1L
L
AUX1R E
∑∆
PGA
AUX2L	C	ADC
T
AUX2R
O
AUX3L R
AUX3R
MIN
| MUTE | GN/AT | GN/AT | GN/AT | GN/AT | GN/AT |
| --- | --- | --- | --- | --- | --- |
| MUTE | MUTE MOUT CRA8 | MUTE | MUTE | MUTE |  |
| CRA8 | CRA6 | CRA5 | CRA4 | CRA7 | CRAX = CONTROL REGISTER ADDRESS “X” |
MUTE
LOUT1L	∑	∑	∑	∑	∑	GN/AT
∑∆
MUTE
DAC
LOUT1R	∑	∑	∑	∑	∑	CRA9
DRIVER
CRA8	MUTE	GN/AT
CRA8 MUTE
MUTE GN/AT = GAIN/
CRA3
ATTENUATION
HPOUTL
HPOUTC
HPOUTR
LOUT2LP LEFT
GN/AT
LOUT2LN ∑∆
MUTE
LOUT2RP	RIGHT	DAC
CRA10
LOUT2RN
SUML
SUMR VOLTAGE REFERENCE
4
AAFILTL AAFILTR	VREF CMOUT	FILTL	FILTR	GNDA
Figure 12.  AD1843 Control Registers Associated with Block Diagram
Address Register Description
0 Codec Status and Revision Identification (Read Only)
1 Channel Status Flags
2 Input Control—ADC Source and Gain/Attenuation
3 Mix Control—DAC2 to Mixer
4 Mix Control—Auxiliary 1 to Mixer
5 Mix Control—Auxiliary 2 to Mixer
6 Mix Control—Auxiliary 3 to Mixer
7 Mix Control—Mic to Mixer
8 Mix/Misc. Control—Mono In to Mixer and Miscellaneous Settings
9 Output Control—DAC1 Analog Gain/Attenuation
10 Output Control—DAC2 Analog Gain/Attenuation
11 Output Control—DAC1 Digital Attenuation
12 Output Control—DAC2 Digital Attenuation
13 Digital Mix Control—ADC to DAC1
14 Digital Mix Control—ADC to DAC2
15 Codec Configuration—Channel Sample Rate Source Select
–26– REV. 0

AD1843
XTALI XTALOSYNC3   SYNC2   SYNC1   CONV3   CONV2   CONV1	BIT3	BIT2	BIT1
CLOCK GENERATION
CRA15, CRA16, CRA17, CRA18, CRA19, CLKOUT
CRA20, CRA21, CRA22, CRA23, CRA24
CRAX = CONTROL REGISTER ADDRESS “X” CONTROL AND STATUS REGISTERS
CRA0, CRA1, CRA25, CRA26,
CRA27, CRA28
AD1843
S
E
L LEFT
E μ/A
S	RIGHT	ADC
C LAW
E
T
L
O
E
CRA25	R	CRA26
C SCLK
T CRA25
O SDFS
R
D SDI
ATTN I
MUTE G
SDO
CRA13 I
T
BM
A
L
LEFT CS
∑ ∑
μ/A
ATTN	FIFO	DAC1	TSO
RIGHT LAW
∑ ∑
TSI
CRA11 I
CRA26 2
N
CRA25	MUTE	XCTL [1:0]
T
ATTN E
MUTE	R	PDMNFT
CRA14 F
A RESET
C
CRA25	MUTE	E	PWRDWN
∑ ∑
μ/A
ATTN	FIFO	DAC2
LAW
∑ ∑
CRA12
CRA26
3 8 9
VCC	GNDD	VDD
Figure 13.  AD1843 Control Registers Associated with Block Diagram
Address Register Description
16 Clock Generator 1 Control—Mode
17 Clock Generator 1 Control—Sample Rate
18 Clock Generator 1 Control—Sample Phase Shift
19 Clock Generator 2 Control—Mode
20 Clock Generator 2 Control—Sample Rate
21 Clock Generator 2 Control—Sample Phase Shift
22 Clock Generator 3 Control—Mode
23 Clock Generator 3 Control—Sample Rate
24 Clock Generator 3 Control—Sample Phase Shift
25 Codec Configuration—Digital Filter and Mode Select
26 Codec Configuration—Serial Interface
27 Codec Configuration—Channel Power Down
28 Codec Configuration—Fundamental Settings
29 Reserved for Future Expansion
30 Reserved for Future Expansion
31 Reserved  for  Future  Expansion
REV. 0 –27–

AD1843
Address 0 Codec Status and Revision Identification
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| INIT | PDNO | res | res | res | res | res | res |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| res | res This register is read only. INIT Clock Initialization Flag.  This bit is set to “1” if the AD1843’s internal clocks generated from the crystal input pin XTALI have not yet stabilized.  When set to “1,” Control Registers may be read if the AD1843 is configured as a Bus Slave (see the definition for the BM pin), but Control Register writes will be blocked.  This bit is set to a “1” after the RESETpin is asserted, or when the power-down sequence (initiated by asserting thePWRDWNpin) has completed. The bit is reset to “0” usually within 400 to 800μsec after theRESETor thePWRDWNpin is deasserted, depending upon parasitic capacitance on the XTALI and XTALO pins outside the AD1843.  Because these parasitics are a func- tion of the board design and layout, the exact amount of time required for the crystal to start to oscillate, and the in- ternal clocks to stabilize cannot be known exactly in advance. PDNO Converter Power-Down Flag.  This bit is set to “1” if the AD1843 conversion resources are powered down, or are in the process of entering or exiting power down.  Conversion resources are all resources in the AD1843 with the excep- tion of the three clock generators and the serial interface.Conversion power-down is entered if either the power-down pin (PWRDWN) is driven LO, or if the PDNI (Converter Power Down) bit in Control Register Address 28 is as- serted.  Power down is exited only if both pin (PWRDWN) and bit (PDNI) are deasserted.  When this bit is set to “1,” Control Registers may still be read, but only Control Registers 16–24, 26 and 28 which do not manage conver- sion resources may be written.  See the “Power Management” section for further details.  This bit is set to “1” imme- diately after a reset since the PDNI bit is initially asserted after a reset. ID3:0 Revision Identification.  These bits define the revision level of the AD1843.  The first version of the AD1843 is “0001.” res Reserved for future expansion.  To ensure future compatibility, write “0” to all reserved bits. Initial default state after reset: 1100 0000 0000 0001 (C001 hex).  Cleared to default and cannot be written to when: theRESETpin is asserted LO; or when thePWRDWNpin is asserted LO. Address 1 Channel Status Flags | res | res | ID3 | ID2 | ID1 | ID0 |
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| res | res | res | res | res | res | SU2 | SU1 |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| res | res | res | res | OVR1 This register is cleared to all “0” after any read or write access. SU2 DAC2 Sample Underrun.  When set to “1,” this bit indicates that the four word stereo input buffer for DAC pair 2 ran out of samples. If this occurs, zeros are used in place of the unavailable data. This bit is “sticky” and is cleared after any write to this register.  It is also cleared by powering down DAC2 (see the DA2EN bit in Control Register Address 27). SU1 DAC1 Sample Underrun.  When set to “1,” this bit indicates that the four word stereo input buffer for DAC pair 1 ran out of samples. If this occurs, zeros are used in place of the unavailable data. This bit is “sticky” and is cleared after any write to this register.  It is also cleared by powering down DAC1 (see the DA1EN bit in Control Register Address 27). OVR1:0 ADC Right Overrange Detect.  These bits record the largest output magnitude on the ADC right channel and are cleared to “00” after any write to this register.  The peak amplitude as recorded by these bits is “sticky,” i.e., the larg- est output magnitude recorded by these bits will persist until these bits are explicitly cleared.  They are also cleared by powering down the ADC right channel (see the ADREN bit in Control Register Address 27). 00= Greater than –1.0 dB underrange 01 = Between –1.0 dB and 0 dB underrange 10 = Between 0 dB and 1 dB overrange 11 = Greater than 1.0 dB overrange –28– REV. 0 | OVR0 | OVL1 | OVL0 |

AD1843
OVL1:0 ADC Left Overrange Detect.  These bits record the largest output magnitude on the ADC left channel and are
cleared to “00” after any write to this register.  The peak amplitude as recorded by these bits is “sticky,” i.e., the larg-
est output magnitude recorded by these bits will persist until these bits are explicitly cleared.  They are also cleared by
powering down the ADC left channel (see the ADLEN bit in Control Register Address 27).
00= Greater than –1.0 dB underrange
01 = Between –1.0 dB and 0 dB underrange
10 = Between 0 dB and 1 dB overrange
11 = Greater than 1.0 dB overrange
res Reserved for future expansion.  To ensure future compatibility, write “0” to all reserved bits.
Initial default state after reset: 0000 0000 0000 0000 (0000 hex).  Cleared to default when: theRESETpin is asserted
LO; when thePWRDWNpin is asserted LO; or when the PDNO bit in Control Register Address 0 is set to “1” (all
conversions disabled).
Address 2 Input Control—ADC Source and Gain/Attenuation
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| LSS2 | LSS1 | LSS0 | LMGE | LIG3 | LIG2 | LIG1 | LIG0 |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| RSS2 | RSS1 LSS2:0 Left ADC Source Select 000= Left Line Input 001 = Left Mic Input 010 = Left Auxiliary 1 Input 011 = Left Auxiliary 2 Input 100 = Left Auxiliary 3 Input 101 = Mono Input 110 = Left DAC1 Output 111 = Left DAC2 Output LMGE Left ADC Microphone Gain Enable 0= 0 dB Gain 1 = +20 dB Gain LIG3:0 Left ADC Input Gain.  Least significant bit represents +1.5 dB. 0000= 0.0 dB Gain 1111 = +22.5 dB Gain RSS2:0 Right ADC Source Select 000= Right Line Input 001 = Right Mic Input 010 = Right Auxiliary 1 Input 011 = Right Auxiliary 2 Input 100 = Right Auxiliary 3 Input 101 = Mono Input 110 = Right DAC1 Output 111 = Right DAC2 Output RMGE Right ADC Microphone Gain Enable 0= 0 dB Gain 1 = +20 dB Gain RIG3:0 Right ADC Input Gain.  Least significant bit represents +1.5 dB. 0000= 0.0 dB Gain 1111 = +22.5 dB Gain Initial Default State after Reset: 0000 0000 0000 0000 (0000 hex). Cleared to default and cannot be written to when: theRESETpin is asserted LO; when thePWRDWNpin is asserted LO; or when the PDNO bit in Control Register 0 is set to “1” (all conversions disabled). REV. 0 –29– | RSS0 | RMGE | RIG3 | RIG2 | RIG1 | RIG0 |

AD1843
Address 3 Mix Control—DAC2 to Mixer
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| LD2MM | res | res | LD2M4 | LD2M3 | LD2M2 | LD2M1 | LD2M0 |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| RD2MM | res | res | RD2M4 LD2MM Left DAC2 Mix Mute 0 = Mix Enabled 1= Mix Muted LD2M4:0 Left DAC2 Mix Gain/Attenuation Select.  Least significant bit represents –1.5 dB.  Referred to 2.0 V p-p DAC2 output level. 00000 = +12.0 dB Gain 01000= 0.0 dB 11111 = –34.5 dB Attenuation RD2MM Right DAC2 Mix Mute 0 = Mix Enabled 1= Mix Muted RD2M4:0 Right DAC2 Mix Gain/Attenuation Select.  Least significant bit represents –1.5 dB. Referred to 2.0 V p-p DAC2 output level. 00000 = +12.0 dB Gain 01000= 0.0 dB 11111 = –34.5 dB Attenuation res Reserved for future expansion.  To ensure future compatibility, write “0” to all reserved bits. Initial default state after reset: 1000 1000 1000 1000 (8888 hex).  Cleared to default and cannot be written to when: theRESETpin is asserted LO; when thePWRDWNpin is asserted LO; when the PDNO bit in Control Register Address 0 is set to “1” (all conversions disabled); or when the DDMEN bit (DAC2 to DAC1 analog mix disabled), the DA2EN bit (DAC2 powered down), or the ANAEN bit (analog channels powered down) in Control Register Ad- dress 27 is reset to “0.” Address 4 Mix Control—Auxiliary 1 to DAC1 | RD2M3 | RD2M2 | RD2M1 | RD2M0 |
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| LX1MM | res | res | LX1M4 | LX1M3 | LX1M2 | LX1M1 | LX1M0 |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| RX1MM | res | res | RX1M4 LX1MM Left Auxiliary 1 Mix Mute 0 = Mix Enabled 1= Mix Muted LX1M4:0 Left Auxiliary 1 Mix Gain/Attenuation Select.  Least significant bit represents –1.5 dB. Referred to 2.0 V p-p DAC1 output level. 00000 = +12.0 dB Gain 01000= 0.0 dB 11111 = –34.5 dB Attenuation RX1MM Right Auxiliary 1 Mix Mute 0 = Mix Enabled 1= Mix Muted RX1M4:0 Right Auxiliary 1 Mix Gain/Attenuation Select.  Least significant bit represents –1.5 dB. Referred to 2.0 V p-p DAC1 output level. 00000 = +12.0 dB Gain 01000= 0.0 dB 11111 = –34.5 dB Attenuation –30– REV. 0 | RX1M3 | RX1M2 | RX1M1 | RX1M0 |

AD1843
res Reserved for future expansion.  To ensure future compatibility, write “0” to all reserved bits.
Initial default state after reset: 1000 1000 1000 1000 (8888 hex).  Cleared to default and cannot be written
to when: theRESETpin is asserted LO; when thePWRDWNpin is asserted LO; when the PDNO bit in
Control Register Address 0 is set to “1” (all conversions disabled); or when the AAMEN bit (analog input to
analog mix disabled), or the ANAEN bit (analog channels powered down) in Control Register Address 27 is
reset to “0.”
Address 5 Mix Control—Auxiliary 2 to Mixer
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| LX2MM | res | res | LX2M4 | LX2M3 | LX2M2 | LX2M1 | LX2M0 |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| RX2MM | res | res | RX2M4 LX2MM Left Auxiliary 2 Mix Mute 0 = Mix Enabled 1= Mix Muted LX2M4:0 Left Auxiliary 2 Mix Gain/Attenuation Select.  Least significant bit represents –1.5 dB. Referred to 2.0 V p-p DAC1 output level. 00000 = +12.0 dB Gain 01000= 0.0 dB 11111 = –34.5 dB Attenuation RX2MM Right Auxiliary 2 Mix Mute 0 = Mix Enabled 1= Mix Muted RX2M4:0 Right Auxiliary 2 Mix Gain/Attenuation Select.  Least significant bit represents –1.5 dB. Referred to 2.0 V p-p DAC1 output level. 00000 = +12.0 dB Gain 01000= 0.0 dB 11111 = –34.5 dB Attenuation res Reserved for future expansion.  To ensure future compatibility, write “0” to all reserved bits. Initial default state after reset: 1000 1000 1000 1000 (8888 hex).  Cleared to default and cannot be written to when: theRESETpin is asserted LO; when thePWRDWNpin is asserted LO; when the PDNO bit in Control Register Ad- dress 0 is set to “1” (all conversions disabled); or when the AAMEN bit (analog input to analog mix disabled), or the ANAEN bit (analog channels powered down) in Control Register Address 27 is reset to “0.” Address 6 Mix Control—Auxiliary 3 to Mixer | RX2M3 | RX2M2 | RX2M1 | RX2M0 |
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| LX3MM | res | res | LX3M4 | LX3M3 | LX3M2 | LX3M1 | LX3M0 |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| RX3MM | res | res | RX3M4 LX3MM Left Auxiliary 3 Mix Mute 0 = Mix Enabled 1= Mix Muted LX3M4:0 Left Auxiliary 3 Mix Gain/Attenuation Select.  Least significant bit represents –1.5 dB. Referred to 2.0 V p-p DAC1 output level. 00000 = +12.0 dB Gain 01000= 0.0 dB 11111 = –34.5 dB Attenuation REV. 0 –31– | RX3M3 | RX3M2 | RX3M1 | RX3M0 |

AD1843
RX3MM Right Auxiliary 3 Mix Mute
0 = Mix Enabled
1= Mix Muted
RX3M4:0 Right Auxiliary 3 Mix Gain/Attenuation Select.  Least significant bit represents –1.5 dB. Referred to
2.0 V p-p DAC1 output level.
00000 = +12.0 dB Gain
01000= 0.0 dB
11111 = –34.5 dB Attenuation
res Reserved for future expansion.  To ensure future compatibility, write “0” to all reserved bits.
Initial default state after reset: 1000 1000 1000 1000 (8888 hex).  Cleared to default and cannot be written to when:
theRESETpin is asserted LO; when thePWRDWNpin is asserted LO; when the PDNO bit in Control Register
Address 0 is set to “1” (all conversions disabled); or when the AAMEN bit (analog input to analog mix disabled), or
the ANAEN bit (analog channels powered down) in Control Register Address 27 is reset to “0.”
Address 7 Mix Control—Mic to Mixer
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| LMCMM | res | res | LMCM4 | LMCM3 | LMCM2 | LMCM1 | LMCM0 |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| RMCMM | res | res | RMCM4 LMCMM Left Microphone Mix Mute 0 = Mix Enabled 1= Mix Muted LMCM4:0 Left Microphone Mix Gain/Attenuation Select.  Least significant bit represents –1.5 dB. Referred to 2.0 V p-p DAC1 output level. 00000 = +12.0 dB Gain 01000= 0.0 dB 11111 = –34.5 dB Attenuation RMCMM Right Microphone Mix Mute 0 = Mix Enabled 1= Mix Muted RMCM4:0 Right Microphone Mix Gain/Attenuation Select.  Least significant bit represents –1.5 dB. Referred to 2.0 V p-p DAC1 output level. 00000 = +12.0 dB Gain 01000= 0.0 dB 11111 = –34.5 dB Attenuation res Reserved for future expansion.  To ensure future compatibility, write “0” to all reserved bits. Initial default state after reset: 1000 1000 1000 1000 (8888 hex).  Cleared to default and cannot be written to when: theRESETpin is asserted LO; when thePWRDWNpin is asserted LO; when the PDNO bit in Control Register Address 0 is set to “1” (all conversions disabled); or when the AAMEN bit (analog input to analog mix disabled), or the ANAEN bit (analog channels powered down) in Control Register Address 27 is reset to “0.” Address 8 Mix/Miscellaneous Control—Mono In to Mixer and Miscellaneous Settings | RMCM3 | RMCM2 | RMCM1 | RMCM0 |
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| MNMM | res | res | MNM4 | MNM3 | MNM2 | MNM1 | MNM0 |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| ALLMM | MNOM | HPOM | HPOS | SUMM | res | DAC2T MNMM Mono Input Mix Mute 0 = Mix Enabled 1= Mix Muted –32– REV. 0 | DAC1T |

AD1843
MNM4:0 Left and Right Mono Mix Gain/Attenuation Select.  Least significant bit represents –1.5 dB.
00000 = +12.0 dB Gain
01000= 0.0 dB
11111 = –34.5 dB Attenuation
ALLMM All Mix Mute.  Mutes all mixing (MIC, AUX1, AUX2, AUX3, and DAC2) to left and right DAC1, overriding the
independent mute control bits in Control Register Addresses 3 through 8.
0= Mix to DAC1 Enabled
1 = Mix to DAC1 Muted
MNOM Mono Output Mute
0 = Mono Output Enabled
1= Mono Output Muted
HPOM Headphone Output Mute (Left and Right)
0 = Headphone Output Enabled
1= Headphone Output Muted
HPOS Headphone Output Voltage Swing (Left and Right)
0= 2 volts peak-to-peak
1 = 4 volts peak-to-peak
SUMM Sum Left and Right Mute.  Mutes mixing from SUML and SUMR pins to DAC1.
0 = SUML and SUMR Mix to DAC1 Enabled
1= SUML and SUMR Mix to DAC1 Muted
DAC2T DAC2 Gain/Attenuation Change Timing.  This bit controls when changes to the DAC2 gain/attenuation setting
(Control Register 10) take effect.  When set to “1,” changes take effect immediately.  When reset to “0,” changes are
delayed until either the output level on DAC2 crosses zero (midscale), or until after a 10 to 12 ms time-out period is
reached.  Delaying gain/attenuation changes until zero crossings reduces instantaneous output voltage changes, which
reduces audible “clicks.”
0= Gain/Attenuation Changes Applied on Signal Zero Crossing or After a 10-12 ms Time-Out
1 = Gain/Attenuation Changes Applied Immediately
DAC1T DAC1 Gain/Attenuation Change Timing.  This bit controls when changes to the DAC1 gain/attenuation setting
(Control Register 9) take effect.  When set to “1,” changes take effect immediately.  When reset to “0,” changes are
delayed until either the output level on DAC1 crosses zero (midscale), or until after a 10 to 12 ms time-out period is
reached.  Delaying gain/attenuation changes until zero crossings reduces instantaneous output voltage changes, which
reduces audible “clicks.”
0= Gain/Attenuation Changes Applied on Signal Zero Crossing or After a 10-12 ms Time-Out
1 = Gain/Attenuation Changes Applied Immediately
res Reserved for future expansion.  To ensure future compatibility, write “0” to all reserved bits.
Initial default state after reset: 1000 1000 0110 1000 (8868 hex).  Cleared to default and cannot be written to when:
theRESETpin is asserted LO; when thePWRDWNpin is asserted LO; or when the PDNO bit in Control Register
Address 0 is set to “1” (all conversions powered down). The MSB half (Data 15 through Data 8) of this Control Reg-
ister is cleared to default and cannot be written to also when: the AAMEN bit (analog input to analog mix disabled),
or the ANAEN bit (analog channels powered down) in Control Register 27 is reset to “0.”
Address 9 Output Control—DAC1 Analog Gain/Attenuation
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| LDA1GM | res | LDA1G5 | LDA1G4 | LDA1G3 | LDA1G2 | LDA1G1 | LDA1G0 |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| RDA1GM | res | RDA1G5 LDA1GM Left DAC1 Analog Mute 0 = Left DAC1 Enabled 1= Left DAC1 Muted REV. 0 –33– | RDA1G4 | RDA1G3 | RDA1G2 | RDA1G1 | RDA1G0 |

AD1843
LDA1G5:0 Left DAC1 Analog/Digital Gain/Attenuation Select.  Least significant bit represents –1.5 dB. Note that the
implementation of the attenuation is mixed analog and digital.
| 0 00000 = | +12.0 dB: | +12.0 dB Analog, | +0.0 dB Digital |
| --- | --- | --- | --- |
| 0 01000= | 0.0 dB: | +0.0 dB Analog, | +0.0 dB Digital |
| 0 11111 = | –34.5 dB: | –34.5 dB Analog, | +0.0 dB Digital |
| 1 00000 = | –36.0 dB: | –34.5 dB Analog, | –1.5 dB Digital |
| 1 11111 = | –82.5 dB: | –34.5 dB Analog, RDA1GM Right DAC1 Analog Mute 0 = Right DAC1 Enabled 1= Right DAC1 Muted RDA1G5:0 Right DAC1 Analog/Digital Gain/Attenuation Select.  Least significant bit represents –1.5 dB. Note that the implementation of the attenuation is mixed analog and digital. | –48.0 dB Digital |
| 0 00000 = | +12.0 dB: | +12.0 dB Analog, | +0.0 dB Digital |
| 0 01000= | 0.0 dB: | +0.0 dB Analog, | +0.0 dB Digital |
| 0 11111 = | –34.5 dB: | –34.5 dB Analog, | +0.0 dB Digital |
| 1 00000 = | –36.0 dB: | –34.5 dB Analog, | –1.5 dB Digital |
| 1 11111 = | –82.5 dB: | –34.5 dB Analog, res Reserved for future expansion.  To ensure future compatibility, write “0” to all reserved bits. Initial default state after reset: 1000 1000 1000 1000 (8888 hex).  Cleared to default and cannot be written to when: theRESETpin is asserted LO; when thePWRDWNpin is asserted LO; when the PDNO bit in Control Register Address 0 is set to “1” (all conversions disabled); when the ANAEN bit in Control Register Address 27 is reset to “0” (analog channels powered down); or when the DA1EN bit in Control Register Address 27 is reset to “0” (DAC1 disabled). Address 10 Output Control—DAC2 Analog Gain/Attenuation | –48.0 dB Digital |
| Data 15 | Data 14 | Data 13 | Data 12 Data 11 Data 10 Data 9 Data 8 |
| LDA2GM | res | LDA2G5 | LDA2G4 LDA2G3 LDA2G2 LDA2G1 LDA2G0 |
| Data 7 | Data 6 | Data 5 | Data 4 Data 3 Data 2 Data 1 Data 0 |
| RDA2GM | res | RDA2G5 | RDA2G4 RDA2G3 RDA2G2 RDA2G1 RDA2G0 LDA2GM Left DAC2 Analog Mute 0 = Left DAC2 Enabled 1= Left DAC2 Muted LDA2G5:0 Left DAC2 Analog/Digital Gain/Attenuation Select.  Least significant bit represents –1.5 dB. Note that the imple- mentation of the attenuation is mixed analog and digital. |
| 0 00000 = | +12.0 dB: | +12.0 dB Analog, | +0.0 dB Digital |
| 0 01000= | 0.0 dB: | +0.0 dB Analog, | +0.0 dB Digital |
| 0 11111 = | –34.5 dB: | –34.5 dB Analog, | +0.0 dB Digital |
| 1 00000 = | –36.0 dB: | –34.5 dB Analog, | –1.5 dB Digital |
| 1 11111 = | –82.5 dB: | –34.5 dB Analog, RDA2GM Right DAC2 Analog Mute 0 = Right DAC2 Enabled 1= Right DAC2 Muted RDA2G5:0 Right DAC2 Analog/Digital Gain/Attenuation Select.  Least significant bit represents –1.5 dB. Note that the imple- mentation of the attenuation is mixed analog and digital. | –48.0 dB Digital |
| 0 00000 = | +12.0 dB: | +12.0 dB Analog, | +0.0 dB Digital |
| 0 01000= | 0.0 dB: | +0.0 dB Analog, | +0.0 dB Digital |
| 0 11111 = | –34.5 dB: | –34.5 dB Analog, | +0.0 dB Digital |
| 1 00000 = | –36.0 dB: | –34.5 dB Analog, | –1.5 dB Digital |
| 1 11111 = | –82.5 dB: | –34.5 dB Analog, res Reserved for future expansion.  To ensure future compatibility, write “0” to all reserved bits. Initial default state after reset: 1000 1000 1000 1000 (8888 hex).  Cleared to default and cannot be written to when: theRESETpin is asserted LO; when thePWRDWNpin is asserted LO; when the PDNO bit in Control Register Address 0 is set to “1” (all conversions disabled); when the ANAEN bit in Control Register Address 27 is reset to “0” (analog channels powered down); or when the DA2EN bit in Control Register Address 27 is reset to “0” (DAC2 disabled). –34– REV. 0 | –48.0 dB Digital |

AD1843
Address 11 Output Control—DAC1 Digital Attenuation
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| LDA1AM | res | LDA1A5 | LDA1A4 | LDA1A3 | LDA1A2 | LDA1A1 | LDA1A0 |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| RDA1AM | res | RDA1A5 LDA1AM Left DAC1 Digital Mute 0 = Left DAC1 Enabled 1= Left DAC1 Muted LDA1A5:0 Left DAC1 Digital Attenuation Select.  Least significant bit represents –1.5 dB. 000000= +0.0 dB Attenuation 111111 = –94.5 dB Attenuation RDA1AM Right DAC1 Digital Mute 0 = Right DAC1 Enabled 1= Right DAC1 Muted RDA1A5:0 Right DAC1 Digital Attenuation Select.  Least significant bit represents –1.5 dB. 000000= +0.0 dB Attenuation 111111 = –94.5 dB Attenuation res Reserved for future expansion.  To ensure future compatibility, write “0” to all reserved bits. Initial default state after reset: 0000 0000 0000 0000 (0000 hex).  Cleared to default and cannot be written to when: theRESETpin is asserted LO; when thePWRDWNpin is asserted LO; or when the PDNO bit in Control Register Address 0 is set to “1” (all conversions disabled). Address 12 Output Control—DAC2 Digital Attenuation | RDA1A4 | RDA1A3 | RDA1A2 | RDA1A1 | RDA1A0 |
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| LDA2AM | res | LDA2A5 | LDA2A4 | LDA2A3 | LDA2A2 | LDA2A1 | LDA2A0 |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| RDA2AM | res | RDA2A5 LDA2AM Left DAC2 Digital Mute 0 = Left DAC2 Enabled 1= Left DAC2 Muted LDA2A5:0 Left DAC2 Digital Attenuation Select.  Least significant bit represents –1.5 dB. 000000= +0.0 dB Attenuation 111111 = –94.5 dB Attenuation RDA2AM Right DAC2 Digital Mute 0 = Right DAC2 Enabled 1= Right DAC2 Muted RDA2A5:0 Right DAC2 Digital Attenuation Select.  Least significant bit represents –1.5 dB. 000000= +0.0 dB Attenuation 111111 = –94.5 dB Attenuation res Reserved for future expansion.  To ensure future compatibility, write “0” to all reserved bits. Initial default state after reset: 0000 0000 0000 0000 (0000 hex).  Cleared to default and cannot be written to when: theRESETpin is asserted LO; when thePWRDWNpin is asserted LO; or when the PDNO bit in Control Register Address 0 is set to “1” (all conversions disabled). REV. 0 –35– | RDA2A4 | RDA2A3 | RDA2A2 | RDA2A1 | RDA2A0 |

AD1843
Address 13 Digital Mix Control—ADC to DAC1
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| LAD1MM | res | LAD1M5 | LAD1M4 | LAD1M3 | LAD1M2 | LAD1M1 | LAD1M0 |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| RAD1MM | res | RAD1M5 Restrictions: ADC and DAC channel mixed must receive conversion rate from same Clock Generator. Serial interface must be run- ning at a rate greater than or equal to the ADC/DAC conversion rate.  For bus master: SDFS frequency≥ADC/DAC conversion rate.  For bus slave: TSI frequency≥ADC/DAC conversion rate. LAD1MM Digital Mix of Left ADC Output with Left DAC1 Input Mute. 0 = Mix Enabled 1= Mix Muted LAD1M5:0 Digital Mix of Left ADC Output with Left DAC1 Input Attenuation Select.  Least significant bit represents –1.5 dB. 000000= +0.0 dB Attenuation 111110 = –93.0 dB Attenuation 111111 = Full Mute RAD1MM Digital Mix of Right ADC Output with Right DAC1 Input Mute. 0 = Mix Enabled 1= Mix Muted RAD1M5:0 Digital Mix of Right ADC Output with Right DAC1 Input Attenuation Select.Least significant bit represents –1.5 dB. 000000= +0.0 dB Attenuation 111110 = –93.0 dB Attenuation 111111 = Full Mute res Reserved for future expansion.  To ensure future compatibility, write “0” to all reserved bits. Initial default state after reset: 1000 0000 1000 0000 (8080 hex).  Cleared to default and cannot be written to when: theRESETpin is asserted LO; when thePWRDWNpin is asserted LO; or when the PDNO bit in Control Register Address 0 is set to “1” (all conversions disabled). Address 14 Digital Mix Control—ADC to DAC2 | RAD1M4 | RAD1M3 | RAD1M2 | RAD1M1 | RAD1M0 |
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| LAD2MM | res | LAD2M5 | LAD2M4 | LAD2M3 | LAD2M2 | LAD2M1 | LAD2M0 |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| RAD2MM | res | RAD2M5 Restrictions: ADC and DAC channel mixed must receive conversion rate from same Clock Generator. Serial interface must be run- ning at a rate greater than or equal to the ADC/DAC conversion rate. For bus master: SDFS frequency≥ADC/DAC conversion rate.  For bus slave: TSI frequency≥ADC/DAC conversion rate. LAD2MM Digital Mix of Left ADC Output with Left DAC2 Input Mute. 0 = Mix Enabled 1= Mix Muted LAD2M5:0 Digital Mix of Left ADC Output with Left DAC2 Input Attenuation Select.  Least significant bit represents –1.5 dB. 000000= +0.0 dB Attenuation 111110 = –93.0 dB Attenuation 111111 = Full Mute RAD2MM Digital Mix of Right ADC Output with Right DAC2 Input Mute. 0 = Mix Enabled 1= Mix Muted RAD2M5:0 Digital Mix of Right ADC Output with Right DAC2 Input Attenuation Select.Least significant bit represents –1.5 dB. 000000= +0.0 dB Attenuation 111110 = –93.0 dB Attenuation 111111 = Full Mute res Reserved for future expansion.  To ensure future compatibility, write “0” to all reserved bits. Initial default state after reset: 1000 0000 1000 0000 (8080 hex).  Cleared to default and cannot be written to when: theRESETpin is asserted LO; when thePWRDWNpin is asserted LO; or when the PDNO bit in Control Register Address 0 is set to “1” (all conversions disabled). –36– REV. 0 | RAD2M4 | RAD2M3 | RAD2M2 | RAD2M1 | RAD2M0 |

AD1843
Address 15 Codec Configuration—Channel Sample Rate Source Select
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| res | res | res | res | DA2C1 | DA2C0 | DA1C1 | DA1C0 |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| res | res | res | res | ADRC1 DA2C1:0 DAC2 Sample Rate Source.  Selects the sample rate clock source for left and right channels of DAC2. 00= Conversion Sample Rate is 48.0 kHz 01 = Conversion Sample Rate is from Clock Generator 1 10 = Conversion Sample Rate is from Clock Generator 2 11 = Conversion Sample Rate is from Clock Generator 3 DA1C1:0 DAC1 Sample Rate Source.  Selects the sample rate clock source for left and right channels of DAC1. 00= Conversion Sample Rate is 48.0 kHz 01 = Conversion Sample Rate is from Clock Generator 1 10 = Conversion Sample Rate is from Clock Generator 2 11 = Conversion Sample Rate is from Clock Generator 3 ADRC1:0 ADC Right Sample Rate Source.  Selects the sample rate clock source for the right ADC channel. 00= Conversion Sample Rate is 48.0 kHz 01 = Conversion Sample Rate is from Clock Generator 1 01 = Conversion Sample Rate is from Clock Generator 2 11 = Conversion Sample Rate is from Clock Generator 3 ADLC1:0 ADC Left Sample Rate Source.  Selects the sample rate clock source for the left ADC channel. 00= Conversion Sample Rate is 48.0 kHz 01 = Conversion Sample Rate is from Clock Generator 1 10 = Conversion Sample Rate is from Clock Generator 2 11 = Conversion Sample Rate is from Clock Generator 3 res Reserved for future expansion.  To ensure future compatibility, write “0” to all reserved bits. Initial default state after reset: 0000 0000 0000 0000 (0000 hex).  Cleared to default and cannot be written to when: theRESETpin is asserted LO; when thePWRDWNpin is asserted LO; or when the PDNO bit in Control Register Address 0 is set to “1” (all conversions disabled). Address 16 Clock Generator 1 Control—Mode | ADRC0 | ADLC1 | ADLC0 |
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| C1REF | C1VID | C1PLLG | C1P200 | C1X8/7 | C1C128 | res | res |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| C1M7 | C1M6 C1REF Clock Generator 1 Reference Select.  Selects the fundamental clock reference used by Clock Generator 1 to synthesize its “Conversion” (sample) and “Bit” clock rates. 0=  Clocks are referenced to the input on pin XTALI (crystal or master clock input). Sample clock frequency is defined by Control Register Address 17 and Bit C1X8/7. Sample clock phase may be shifted by Control Register Address 18. Bit clock frequency is defined by bits C1M7:0 and C1P200. Bit C1VID is ignored. 1 =  Clocks are referenced to the input on pin SYNC1 (Sync 1 Clock Input). Sample clock frequency is defined by C1VID and C1M7:0. Sample clock phase is locked to SYNC1 and cannot be shifted. Bit clock frequency is defined by bits C1M7:0 and C1P200 unless in Video Lock Mode  (C1VID set to “1”) where the Bit clocks are not produced. Control Register Addresses 17, 18 and the C1X8/7 bit are ignored. REV. 0 –37– | C1M5 | C1M4 | C1M3 | C1M2 | C1M1 | C1M0 |

AD1843
C1VID Clock Generator 1 Video Lock Mode.  This bit is used to select between lock modes when the Clock Generator 1 is
referenced to SYNC1 (C1REF set to “1”). This bit should be reset to “0” if C1REF is reset to “0.” When reset to
“0,” Clock Generator 1 is innormallock mode where the Conversion clock will be frequency and phase locked to
SYNC1, and the Bit clock frequency is chosen using bits C1M7:0 and C1P200.  When set to “1,” Clock Generator 1
is invideolock mode, where the Conversion clock frequency is selected using bits C1M7:0, and a Bit clock is not pro-
duced.
C1PLLG Clock Generator 1 PLL Loop Gain Select.  If reset to “0,” this bit selects finite PLL loop gain, and if set to “1,” this
bit selects infinite PLL loop gain.  This bit should nominally be reset to “0.”  Setting it to “1” may enhance the PLL’s
ability to lock to certain SYNC1 inputs, but it may also increase conversion noise.
C1P200 Clock Generator 1 Bit Clock +200 Frequency Modifier.  When set to “1,” the Bit clock driven out of pin BIT1 will
have a frequency that is 200 Hertz greater than the frequency selected through bit C1M7:0.  This bit is ignored when
in Video Lock Mode (C1VID set to “1”). C1P200 only modifies the bit clock driven on the BIT1 pin.
C1X8/7 Clock Generator 1 Conversion Clock 8/7 Frequency Modifier.  When set to “1,” the Conversion clock frequency gen-
erated will be 8/7 times the value programmed in Control Register Address 17.  This bit is ignored when clocks are
referenced to SYNC1 (C1REF set to “1”).
C1C128 Clock Generator 1 Conversion Clock Pin (CONV1) Frequency Select.  When set to “1,” the frequency driven on to
the CONV1 pin will be 128 times the conversion rate.  When reset to “0,” the frequency driven on to the CONV1 pin
will be the same as the conversion rate. C1C128 only modifies the clock frequency driven on the CONV1 pin.
res Reserved for future expansion.  To ensure future compatibility, write “0” to all reserved bits.
C1M7:0 Clock Generator 1 Clock Rate Modifiers.
When not in Video Lock Mode (C1REF and C1VID are not both set to “1”):
Bits C1M7:0 select the Bit clock rate which will be driven out on pin BIT1.  Using the following table, the least sig-
nificant four bits (C1M3:0) are programmed to the desired Bit clock rate, and the most significant four bits
(C1M7:4) must be programmed to the Conversion clock rate established using Control Register Address 17 and the
C1X8/7 bit.  If the actual Conversion clock differs from the value selected by C1M7:4, then the resultant Bit clock
will be different from the rate selected by the ratio of the C1M7:4 selected rate to the Control Register Address 17
plus the C1X8/7 bit actual rate.
| C1M3:0 | Bit Clock Frequency* | C1M7:4 | Conversion (Sample) Rate |
| --- | --- | --- | --- |
| 0000 | 2,400 | Hertz | 0000 7,200 Hertz |
| 0001 | 4,800 | 0001 | 8,400 |
| 0010 | 7,200 | 0010 | 9,000 |
| 0011 | 9,600 | 0011 | 9,600 |
| 0100 | 12,000 | 0100 | 11,200 |
| 0101 | 14,400 | 0101 | 12,000 |
| 0110 | 16,800 | 0110 | 12,800 |
| 0111 | 19,200 | 0111 | 7,200×8/7 |
| 1000 | 21,600 | 1000 | 9,000×8/7 |
| 1001 | 24,000 | 1001 | 9,600×8/7 |
| 1010 | 26,400 | 1010 | 12,000×8/7 |
| 1011 | 28,800 | 1011 | Reserved |
| 1100 | Reserved | 1100 | Reserved |
| 1101 | Reserved | 1101 | Reserved |
| 1110 | Reserved | 1110 | Reserved |
| 1111 | See Below *Bit clock frequencies listed will be increased by 200 Hz if C1P200 is set to “1.” When C1M7:4 is programmed to “1111” and C1M3:0 is programmed to “1111,” the Bit clock rate will be 128 times the Conversion rate. When in Video Lock Mode (C1REF and C1VID are both set to “1”): Bits C1M7:0 select the Conversion clock rate.  The most significant bit (C1M7) must be set to indicate the type of video lock, either NTSC or PAL.  For an NTSC lock, C1M7 must be reset to “0,” and the SYNC1 pin must receive the NTSC sync frequency (525 lines/frame×30 Hz×1000/1001 frame rate≈15.734 kHz).  For a PAL lock, C1M7 must be set to “1,” and the SYNC1 pin must receive the PAL sync frequency (625 lines/frame×25 Hz frame rate≈15.625 kHz).  The next three most significant bits (C1M6:4) select a desiredbaseConversion clock rate, and the least significant four bits (C1M3:0) select adivisor.  The Conversion clock created by Clock Generator 1 will be thebasedivided by thedivisor.  The following tables list the possible choices for base and divisor. –38– REV. 0 | 1111 | See Below |

AD1843
| NTSC | (C1M7 = “0”): | Base Frequency In Hz (C1M6:4) |
| --- | --- | --- |
| 48,000 | 32,000 | 44,100 44,100×2/3 48,000* 44,056 |
| Divisor | (C1M3:0) | (000) (001) (010) (011) (100) (101) |
| 1 | (0000) | Yes Yes Yes Yes Yes Yes |
| 2 | (0001) | Yes Yes Yes Yes Yes Yes |
| 3 | (0010) | Yes No Yes No Yes Yes |
| 4 | (0011) | Yes Yes Yes Yes Yes Yes |
| 5 | (0100) | Yes Yes Yes Yes Yes Yes |
| 6 | (0101) | Yes No Yes No Yes Yes |
| 7 | (0110) | No No Yes No Yes Yes |
| 8 | (0111) | Yes No Yes No Yes Yes *When C1M6:4 = “100,” base frequency is 48,000 Hz only if NTSC sync rate is increased by 1001/1000, or is exactly 15.750 kHz. |
| PAL | (C1M7 = “1”): | Base Frequency In Hz (C1M6:4) |
| 48,000 | 32,000 | 44,100 44,100×2/3 |
| Divisor | (C1M3:0) | (000) (001) (010) (011) |
| 1 | (0000) | Yes Yes Yes Yes |
| 2 | (0001) | Yes Yes Yes Yes |
| 3 | (0010) | Yes No Yes No |
| 4 | (0011) | Yes Yes Yes Yes |
| 5 | (0100) | Yes Yes Yes Yes |
| 6 | (0101) | Yes No Yes No |
| 7 | (0110) | Yes No Yes No |
| 8 | (0111) | Yes No Yes No Initial default state after reset: 0000 0000 1111 1111 (00FF hex).  Cleared to default and cannot be written to when: theRESETpin is asserted LO; or when thePWRDWNpin is asserted LO. Address 17 Clock Generator 1 Control—Sample Rate |
| Data 15 | Data 14 | Data 13 Data 12 Data 11 Data 10 Data 9 Data 8 |
| C1C15 | C1C14 | C1C13 C1C12 C1C11 C1C10 C1C9 C1C8 |
| Data 7 | Data 6 | Data 5 Data 4 Data 3 Data 2 Data 1 Data 0 |
| C1C7 | C1C6 | C1C5 C1C4 C1C3 C1C2 C1C1 C1C0 C1C15:0 Clock Generator 1 Conversion (Sample) Rate Select.  Defines the conversion rate produced by Clock Generator 1 when not referenced to the SYNC1 pin (Control Register Address 16 Bit 15 [C1REF]).  One LSB represents exactly one Hertz, assuming a 24.576 MHz clock input on the XTALI pin.  Usable range is 4 kHz (0x0FA0) to 54 kHz (0xD2F0). Initial default state after reset: 1011 1011 1000 0000 (BB80 hex), which is 48 kHz, assuming a 24.576 MHz clock in- put on the XTALI pin.  Cleared to default and cannot be written to when: theRESETpin is asserted LO; or when thePWRDWNpin is asserted LO. Address 18 Clock Generator 1 Control—Sample Phase Shift |
| Data 15 | Data 14 | Data 13 Data 12 Data 11 Data 10 Data 9 Data 8 |
| res | res | res res res res res C1PD |
| Data 7 | Data 6 | Data 5 Data 4 Data 3 Data 2 Data 1 Data 0 |
| C1P7 | C1P6 | C1P5 C1P4 C1P3 C1P2 C1P1 C1P0 C1PD Clock Generator 1 Phase Shift Direction.  This bit controls the direction of sample clock phase shift. 0= Phase Advance 1 = Phase Retard C1P7:0 Clock Generator 1 Phase Shift Magnitude.  These bits control the magnitude of sample clock phase shift.  One LSB represents exactly 0.12 degrees.  LSBs are processed and decremented at a rate of 3.072 MHz (assuming a 24.576 MHz clock input on the XTALI pin).  When this register is read, it indicates any phase advance/retard remaining to be processed as of the beginning of slot 0 if bus master, or when TSI was received if bus slave.  This register may be REV. 0 –39– |

AD1843
overwritten even if all previously programmed phase advance/retard has not been processed.  When written, the con-
tents of this register (just prior to the write) are transmitted during slot 1 of the following frame (as with all Control
Register writes).
res Reserved for future expansion.  To ensure future compatibility, write “0” to all reserved bits.
Initial default state after reset: 0000 0000 0000 0000 (0000 hex).  Cleared to default and cannot be written to when:
theRESETpin is asserted LO; when thePWRDWNpin is asserted LO; or when the G1EN bit in Control Register
Address 28 is reset to “0” (clock generator 1 disabled).
Address 19 Clock Generator 2 Control—Mode
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| C2REF | C2VID | C2PLLG | C2P200 | C2X8/7 | C2C128 | res | res |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| C2M7 | C2M6 C2REF Clock Generator 2 Reference Select.  Selects the fundamental clock reference used by Clock Generator 2 to synthesize its “Conversion” (sample) and “Bit” clock rates. 0=  Clocks are referenced to the input on pin XTALI (crystal or master clock input). Sample clock frequency is defined by Control Register Address 20 and Bit C2X8/7. Sample clock phase may be shifted by Control Register Address 21. Bit clock frequency is defined by bits C2M7:0 and C2P200. Bit C2VID is ignored. 1 = Clocks are referenced to the input on pin SYNC2 (Sync 2 Clock Input). Sample clock frequency is defined by C2VID and C2M7:0. Sample clock phase is locked to SYNC2 and cannot be shifted. Bit clock frequency is defined by bits C2M7:0 and C2P200 unless in Video Lock Mode (C2VID set to “1”) where the Bit clocks are not produced. Control Register Addresses 17, 18 and the C2X8/7 bit are ignored. C2VID Clock Generator 2 Video Lock Mode.  This bit is used to select between lock modes when the Clock Generator 2 is referenced to SYNC2 (C2REF set to “1”).  This bit should be reset to “0” if C2REF is reset to “0.” When reset to “0,” Clock Generator 2 is innormallock mode where the Conversion clock will be frequency and phase locked to SYNC2, and the Bit clock frequency is chosen using bits C2M7:0 and C2P200.  When set to “1,” Clock Generator 2 is invideolock mode, where the Conversion clock frequency is selected using bits C2M7:0, and a Bit clock is not produced. C2PLLG Clock Generator 2 PLL Loop Gain Select.  If reset to “0,” this bit selects finite PLL loop gain, and if set to “1,” this bit selects infinite PLL loop gain.  This bit should nominally be reset to “0.”  Setting it to “1” may enhance the PLL’s ability to lock to certain SYNC2 inputs, but it may also increase conversion noise. C2P200 Clock Generator 2 Bit Clock +200 Frequency Modifier.  When set to “1,” the Bit clock driven out of pin BIT2 will have a frequency that is 200 Hertz greater than the frequency selected through bit C2M7:0.  This bit is ignored when in Video Lock Mode (C2VID set to “1”). C2P200 only modifies the bit clock driven on the BIT2 pin. C2X8/7 Clock Generator 2 Conversion Clock 8/7 Frequency Modifier.  When set to “1,” the Conversion clock frequency gen- erated will be 8/7 times the value programmed in Control Register Address 20.  This bit is ignored when clocks are referenced to SYNC2 (C2REF set to “1”). C2C128 Clock Generator 2 Conversion Clock Pin (CONV2) Frequency Select.  When set to “1,” the frequency driven on to the CONV2 pin will be 128 times the conversion rate.  When reset to “0,” the frequency driven on to the CONV2 pin will be the same as the conversion rate. C2C128 only modifies the clock frequency driven on the CONV2 pin. res Reserved for future expansion.  To ensure future compatibility, write “0” to all reserved bits. C2M7:0 Clock Generator 2 Clock Rate Modifiers. When not in Video Lock Mode (C2REF and C2VID are not both set to “1”): Bits C2M7:0 select the Bit clock rate which will be driven out on pin BIT2.  Using the following table, the least sig- nificant four bits (C2M3:0) are programmed to the desired Bit clock rate, and the most significant four bits –40– REV. 0 | C2M5 | C2M4 | C2M3 | C2M2 | C2M1 | C2M0 |

AD1843
(C2M7:4) must be programmed to the Conversion clock rate established using Control Register Address 20 and the
C2X8/7 bit. If the actual Conversion clock differs from the value selected by C2M7:4, then the resultant Bit clock will
be different from the rate selected by the ratio of the C2M7:4 selected rate to the Control Register Address 20 plus
the C2X8/7 bit actual rate.
| C2M3:0 | Bit Clock Frequency* | C2M7:4 | Conversion (Sample) Rate |
| --- | --- | --- | --- |
| 0000 | 2,400 | Hertz | 0000 7,200 Hertz |
| 0001 | 4,800 | 0001 | 8,400 |
| 0010 | 7,200 | 0010 | 9,000 |
| 0011 | 9,600 | 0011 | 9,600 |
| 0100 | 12,000 | 0100 | 11,200 |
| 0101 | 14,400 | 0101 | 12,000 |
| 0110 | 16,800 | 0110 | 12,800 |
| 0111 | 19,200 | 0111 | 7,200×8/7 |
| 1000 | 21,600 | 1000 | 9,000×8/7 |
| 1001 | 24,000 | 1001 | 9,600×8/7 |
| 1010 | 26,400 | 1010 | 12,000×8/7 |
| 1011 | 28,800 | 1011 | Reserved |
| 1100 | Reserved | 1100 | Reserved |
| 1101 | Reserved | 1101 | Reserved |
| 1110 | Reserved | 1110 | Reserved |
| 1111 | See Below *Bit clock frequencies listed will be increased by 200 Hz if C2P200 is set to “1.” When C2M7:4 is programmed to “1111” and C2M3:0 is programmed to “1111,” the Bit clock rate will be 128 times the Conversion rate. When in Video Lock Mode (C2REF and C2VID are both set to “1”): Bits C2M7:0 select the Conversion clock rate.  The most significant bit (C2M7) must be set to indicate the type of video lock, either NTSC or PAL.  For an NTSC lock, C2M7 must be reset to “0,” and the SYNC2 pin must receive the NTSC sync frequency (525 lines/frame×30 Hz×1000/1001 frame rate≈15.734 kHz).  For a PAL lock, C2M7 must be set to “1,” and the SYNC2 pin must receive the PAL sync frequency (625 lines/frame×25 Hz frame rate≈ 15.625 kHz).  The next three most significant bits (C2M6:4) select a desiredbaseConversion clock rate, and the least significant four bits (C2M3:0) select adivisor.  The Conversion clock created by Clock Generator 2 will be thebase divided by thedivisor.  The following tables list the possible choices for base and divisor. | 1111 | See Below |
| NTSC | (C2M7 = “0”): | Base Frequency In Hz (C2M6:4) |  |
| 48,000 | 32,000 | 44,100 | 44,100×2/3 48,000* 44,056 |
| Divisor | (C2M3:0) | (000) | (001) (010) (011) (100) (101) |
| 1 | (0000) | Yes | Yes Yes Yes Yes Yes |
| 2 | (0001) | Yes | Yes Yes Yes Yes Yes |
| 3 | (0010) | Yes | No Yes No Yes Yes |
| 4 | (0011) | Yes | Yes Yes Yes Yes Yes |
| 5 | (0100) | Yes | Yes Yes Yes Yes Yes |
| 6 | (0101) | Yes | No Yes No Yes Yes |
| 7 | (0110) | No | No Yes No Yes Yes |
| 8 | (0111) | Yes | No Yes No Yes Yes *When C2M6:4 = “100,” base frequency is 48,000 Hz only if NTSC sync rate is increased by 1001/1000, or is exactly 15.750 kHz. |
| PAL | (C2M7 = “1”): | Base Frequency In Hz (C2M6:4) |  |
| 48,000 | 32,000 | 44,100 | 44,100×2/3 |
| Divisor | (C2M3:0) | (000) | (001) (010) (011) |
| 1 | (0000) | Yes | Yes Yes Yes |
| 2 | (0001) | Yes | Yes Yes Yes |
| 3 | (0010) | Yes | No Yes No |
| 4 | (0011) | Yes | Yes Yes Yes |
| 5 | (0100) | Yes | Yes Yes Yes |
| 6 | (0101) | Yes | No Yes No |
| 7 | (0110) | Yes | No Yes No |
| 8 | (0111) | Yes | No Yes No Initial default state after reset: 0000 0000 1111 1111 (00FF hex).  Cleared to default and cannot be written to when: theRESETpin is asserted LO; or when thePWRDWNpin is asserted LO. REV. 0 –41– |

AD1843
Address 20 Clock Generator 2 Control—Sample Rate
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| C2C15 | C2C14 | C2C13 | C2C12 | C2C11 | C2C10 | C2C9 | C2C8 |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| C2C7 | C2C6 C2C15:0 Clock Generator 2 Conversion (Sample) Rate Select.  Defines the conversion rate produced by Clock Generator 2 when not referenced to the SYNC2 pin (Control Register Address 19 Bit 15 [C2REF]).  One LSB represents exactly one Hertz, assuming a 24.576 MHz clock input on the XTALI pin.  Usable range is 4 kHz (0x0FA0) to 54 kHz (0xD2F0). Initial default state after reset: 1011 1011 1000 0000 (BB80 hex), which is 48 kHz, assuming a 24.576 MHz clock in- put on the XTALI pin.  Cleared to default and cannot be written to when: theRESETpin is asserted LO; or when thePWRDWNpin is asserted LO. Address 21 Clock Generator 2 Control—Sample Phase Shift | C2C5 | C2C4 | C2C3 | C2C2 | C2C1 | C2C0 |
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| res | res | res | res | res | res | res | C2PD |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| C2P7 | C2P6 C2PD Clock Generator 2 Phase Shift Direction.  This bit controls the direction of sample clock phase shift. 0= Phase Advance 1 = Phase Retard C2P7:0 Clock Generator 2 Phase Shift Magnitude.  These bits control the magnitude of sample clock phase shift.  One LSB repre- sents exactly 0.12 degrees.  LSBs are processed and decremented at a rate of 3.072 MHz (assuming a 24.576 MHz clock input on the XTALI pin).  When this register is read, it indicates any phase advance/retard remaining to be processed as of the beginning of slot 0 if bus master, or when TSI was received if bus slave. This register may be overwritten even if all pre- viously programmed phase advance/retard has not been processed.  When written, the contents of this register (just prior to the write) are transmitted during slot 1 of the following frame (as with all Control Register writes). res Reserved for future expansion.  To ensure future compatibility, write “0” to all reserved bits. Initial default state after reset: 0000 0000 0000 0000 (0000 hex).  Cleared to default and cannot be written to when: theRESETpin is asserted LO; when thePWRDWNpin is asserted LO; or when the G2EN bit in Control Register Address 28 is reset to “0” (clock generator 2 disabled). Address 22 Clock Generator 3 Control—Mode | C2P5 | C2P4 | C2P3 | C2P2 | C2P1 | C2P0 |
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| C3REF | C3VID | C3PLLG | C3P200 | C3X8/7 | C3C128 | res | res |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| C3M7 | C3M6 C3REF Clock Generator 3 Reference Select.  Selects the fundamental clock reference used by Clock Generator 3 to synthe- size its “Conversion” (sample) and “Bit” clock rates. 0=  Clocks are referenced to the input on pin XTALI (crystal or master clock input). Sample clock frequency is defined by Control Register Address 23 and Bit C3X8/7. Sample clock phase may be shifted by Control Register Address 24. Bit clock frequency is defined by bits C3M7:0 and C3P200. Bit C3VID is ignored. –42– REV. 0 | C3M5 | C3M4 | C3M3 | C3M2 | C3M1 | C3M0 |

AD1843
1 =  Clocks are referenced to the input on pin SYNC3 (Sync 3 Clock Input).
Sample clock frequency is defined by C3VID and C3M7:0.
Sample clock phase is locked to SYNC3 and cannot be shifted.
Bit clock frequency is defined by bits C3M7:0 and C3P200 unless in Video Lock Mode
(C3VID set to “1”) where the Bit clocks are not produced.
Control Register Addresses 23, 24 and the C3X8/7 bits are ignored.
C3VID Clock Generator 3 Video Lock Mode.  This bit is used to select between lock modes when the Clock Generator 3 is
referenced to SYNC3 (C3REF set to “1”).  This bit should be reset to “0” if C3REF is reset to “0.” When reset to
“0,” Clock Generator 3 is innormallock mode where the Conversion clock will be frequency and phase locked to
SYNC3, and the Bit clock frequency is chosen using bits C3M7:0 and C3P200.  When set to “1,” Clock Generator 3
is invideolock mode, where the Conversion clock frequency is selected using bits C3M7:0, and a Bit clock is not produced.
C3PLLG Clock Generator 3 PLL Loop Gain Select.  If reset to “0,” this bit selects finite PLL loop gain, and if set to “1,” this
bit selects infinite PLL loop gain.  This bit should nominally be reset to “0.”  Setting it to “1” may enhance the PLL’s
ability to lock to certain SYNC3 inputs, but it may also increase conversion noise.
C3P200 Clock Generator 3 Bit Clock +200 Frequency Modifier.  When set to “1,” the Bit clock driven out of pin BIT3 will
have a frequency that is 200 Hertz greater than the frequency selected through bit C3M7:0.  This bit is ignored when
in Video Lock Mode (C3VID set to “1”). C3P200 only modifies the bit clock driven on the BIT3 pin.
C3X8/7 Clock Generator 3 Conversion Clock 8/7 Frequency Modifier.  When set to “1,” the Conversion clock frequency gen-
erated will be 8/7 times the value programmed in Control Register Address 23.  This bit is ignored when clocks are
referenced to SYNC3 (C3REF set to “1”).
C3C128 Clock Generator 3 Conversion Clock Pin (CONV3) Frequency Select.  When set to “1,” the frequency driven on to
the CONV3 pin will be 128 times the conversion rate.  When reset to “0,” the frequency driven on to the CONV3 pin
will be the same as the conversion rate. C3C128 only modifies the clock frequency driven on the CONV3 pin.
res Reserved for future expansion.  To ensure future compatibility, write “0” to all reserved bits.
C3M7:0 Clock Generator 3 Clock Rate Modifiers.
When not in Video Lock Mode (C3REF and C3VID are not both set to “1”):
Bits C3M7:0 select the Bit clock rate which will be driven out on pin BIT3.  Using the following table, the least sig-
nificant four bits (C3M3:0) are programmed to the desired Bit clock rate, and the most significant four bits
(C3M7:4) must be programmed to the Conversion clock rate established using Control Register Address 23 and the
C3X8/7 bit.  If the actual Conversion clock differs from the value selected by C3M7:4, then the resultant Bit clock
will be different from the rate selected by the ratio of the C3M7:4 selected rate to the Control Register Address 23
plus the C3X8/7 bit actual rate.
| C3M3:0 | Bit Clock Frequency* | C3M7:4 | Conversion (Sample) Rate |
| --- | --- | --- | --- |
| 0000 | 2,400 | Hertz | 0000 7,200 Hertz |
| 0001 | 4,800 | 0001 | 8,400 |
| 0010 | 7,200 | 0010 | 9,000 |
| 0011 | 9,600 | 0011 | 9,600 |
| 0100 | 12,000 | 0100 | 11,200 |
| 0101 | 14,400 | 0101 | 12,000 |
| 0110 | 16,800 | 0110 | 12,800 |
| 0111 | 19,200 | 0111 | 7,200×8/7 |
| 1000 | 21,600 | 1000 | 9,000×8/7 |
| 1001 | 24,000 | 1001 | 9,600×8/7 |
| 1010 | 26,400 | 1010 | 12,000×8/7 |
| 1011 | 28,800 | 1011 | Reserved |
| 1100 | Reserved | 1100 | Reserved |
| 1101 | Reserved | 1101 | Reserved |
| 1110 | Reserved | 1110 | Reserved |
| 1111 | See Below *Bit clock frequencies listed will be increased by 200 Hz if C3P200 is set to “1.” When C3M7:4 is programmed to “1111” and C3M3:0 is programmed to “1111,” the Bit clock rate will be 128 times the Conversion rate. REV. 0 –43– | 1111 | See Below |

AD1843
When in Video Lock Mode (C3REF and C3VID are both set to “1”):
Bits C3M7:0 select the Conversion clock rate.  The most significant bit (C3M7) must be set to indicate the type of
video lock, either NTSC or PAL.  For an NTSC lock, C3M7 must be reset to “0,” and the SYNC3 pin must
receive the NTSC sync frequency (525 lines/frame×30 Hz×1000/1001 frame rate≈15.734 kHz).  For a PAL lock,
C3M7 must be set to “1,” and the SYNC3 pin must receive the PAL sync frequency (625 lines/frame×25 Hz frame
rate≈15.625 kHz).  The next three most significant bits (C3M6:4) select a desiredbaseConversion clock rate, and
the least significant four bits (C3M3:0) select adivisor.  The Conversion clock created by Clock Generator 3 will be
thebasedivided by thedivisor.  The following tables list the possible choices for base and divisor.
| NTSC | (C3M7 = “0”): | Base Frequency In Hz (C3M6:4) |
| --- | --- | --- |
| 48,000 | 32,000 | 44,100 44,100×2/3 48,000* 44,056 |
| Divisor | (C3M3:0) | (000) (001) (010) (011) (100) (101) |
| 1 | (0000) | Yes Yes Yes Yes Yes Yes |
| 2 | (0001) | Yes Yes Yes Yes Yes Yes |
| 3 | (0010) | Yes No Yes No Yes Yes |
| 4 | (0011) | Yes Yes Yes Yes Yes Yes |
| 5 | (0100) | Yes Yes Yes Yes Yes Yes |
| 6 | (0101) | Yes No Yes No Yes Yes |
| 7 | (0110) | No No Yes No Yes Yes |
| 8 | (0111) | Yes No Yes No Yes Yes *When C3M6:4 = “100,” base frequency is 48,000 Hz only if NTSC sync rate is increased by 1001/1000, or is exactly 15.570 kHz. |
| PAL | (C3M7 = “1”): | Base Frequency In Hz (C3M6:4) |
| 48,000 | 32,000 | 44,100 44,100×2/3 |
| Divisor | (C3M3:0) | (000) (001) (010) (011) |
| 1 | (0000) | Yes Yes Yes Yes |
| 2 | (0001) | Yes Yes Yes Yes |
| 3 | (0010) | Yes No Yes No |
| 4 | (0011) | Yes Yes Yes Yes |
| 5 | (0100) | Yes Yes Yes Yes |
| 6 | (0101) | Yes No Yes No |
| 7 | (0110) | Yes No Yes No |
| 8 | (0111) | Yes No Yes No Initial default state after reset: 0000 0000 1111 1111 (00FF hex).  Cleared to default and cannot be written to when: theRESETpin is asserted LO; or when thePWRDWNpin is asserted LO. Address 23 Clock Generator 3 Control—Sample Rate |
| Data 15 | Data 14 | Data 13 Data 12 Data 11 Data 10 Data 9 Data 8 |
| C3C15 | C3C14 | C3C13 C3C12 C3C11 C3C10 C3C9 C3C8 |
| Data 7 | Data 6 | Data 5 Data 4 Data 3 Data 2 Data 1 Data 0 |
| C3C7 | C3C6 | C3C5 C3C4 C3C3 C3C2 C3C1 C3C0 C3C15:0 Clock Generator 3 Conversion (Sample) Rate Select.  Defines the conversion rate produced by Clock Generator 3 when not referenced to the SYNC3 pin (Control Register Address 22 Bit 15 [C3REF]).  One LSB represents exactly one Hertz, assuming a 24.576 MHz clock input on the XTALI pin.  Usable range is 4 kHz (0x0FA0) to 54 kHz (0xD2F0). Initial default state after reset: 1011 1011 1000 0000 (BB80 hex), which is 48 kHz, assuming a 24.576 MHz clock in- put on the XTALI pin.  Cleared to default and cannot be written to when: theRESETpin is asserted LO; or when thePWRDWNpin is asserted LO. –44– REV. 0 |

AD1843
Address 24 Clock Generator 3 Control—Sample Phase Shift
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| res | res | res | res | res | res | res | C3PD |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| C3P7 | C3P6 C3PD Clock Generator 3 Phase Shift Direction.  This bit controls the direction of sample clock phase shift. 0= Phase Advance 1 = Phase Retard C3P7:0 Clock Generator 3 Phase Shift Magnitude.  These bits control the magnitude of sample clock phase shift.  One LSB represents exactly 0.12 degrees.  LSBs are processed and decremented at a rate of 3.072 MHz (assuming a 24.576 MHz clock input on the XTALI pin).  When this register is read, it indicates any phase advance/retard remaining to be processed as of the beginning of slot 0 if bus master, or when TSI was received if bus slave.  This register may be overwritten even if all previously programmed phase advance/retard has not been processed.  When written, the con- tents of this register (just prior to the write) are transmitted during slot 1 of the following frame (as with all Control Register writes). res Reserved for future expansion.  To ensure future compatibility, write “0” to all reserved bits. Initial default state after reset: 0000 0000 0000 0000 (0000 hex).  Cleared to default and cannot be written to when: theRESETpin is asserted LO; when thePWRDWNpin is asserted LO; or when the G3EN bit in Control Register Address 28 is reset to “0” (clock generator 3 disabled). Address 25 Codec Configuration—Digital Filter and Mode Select | C3P5 | C3P4 | C3P3 | C3P2 | C3P1 | C3P0 |
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| DRSFLT | DAMIX | res | res | res | res | DA2FLT | DA1FLT |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| DAADR1 | DAADR0 Restrictions: Modem filter should not be selected when channel sample rate is above 24 kHz. DRSFLT Digital Resampler Filter Mode.  DAC2 is sacrificed so that the remaining four channels (left ADC, right ADC, left and right DAC1) have sufficient resources to realize the more stringent resampling filter requirements. See below for filter specifications. (Note: Digital resampling can be done without using the resampling filters but performance is not as optimal.)  This bit can be altered only if all channels (DAC1, DAC2, ADL, and ADR) are powered down. Note that this bit enables the digital resampler filters, but does not enable the digital resampler data pathways.  Use the DAADR1:0 and DAADL1:0 bits to enable the resampler data pathways. 0= Audio/Modem Filter Modes (Defined by DA1FLT, DA2FLT, ADLFLT and ADRFLT) 1 = Digital Resampler Filter Mode (DA1FLT, DA2FLT, ADLFLT and ADRFLT filter selections are ignored). DAMIX DAC Digital Mix Enable.  When set to “1,” DAC1 and DAC2 are mixed at the output of the digital interpolation fil- ter.  Left and right channels are still separated, but DAC1 and DAC2 outputs are identical.  This bit can be altered only if both DAC1 and DAC2 are powered down. 0= Digital Mix Disabled 1 = Digital Mix Enabled DA2FLT DAC2 (Left and Right Channels) Digital Filter Select and Analog Output Swing Select.  Digital filter selected is over- ridden when DRSFLT is set to “1.”  See below for filter specifications.  This bit can be altered only if DAC2 is powered down. 0= Digital Audio Filter.  Nominal analog swing is 2.000 V p-p when output level is +0.0 dB (see Control Register Address 10). 1 = Digital Modem Filter.  Nominal analog swing is 3.156 V p-p when output level is +4.5 dB (see Control Register Address 10). REV. 0 –45– | DAADL1 | DAADL0 | res | res | ADRFLT | ADLFLT |

AD1843
DA1FLT DAC1 (Left and Right Channels) Digital Filter Select.  This bit is overridden when DRSFLT is set to “1.”  See be-
low for filter specifications.  This bit can be altered only if DAC1 is powered down.  Unlike DA2FLT, this bit does
not control the analog output swing of DAC1, which is fixed at 2.000 V p-p when the output level is set to 0 dB by
Control Register Address 9.
0= Digital Audio Filter
1 = Digital Modem Filter
DAADR1:0 Digital ADC Right Channel Source Select.
00= Input from Analog: ADC Right Channel (Normal Operation)
01 = Reserved
10 = Input from Digital: DAC1 Right Channel (Digital Resampler)
11 = Input from Digital: DAC2 Right Channel (Digital Resampler)
DAADL1:0 Digital ADC Left Channel Source Select.
00= Input from Analog: ADC Left Channel (Normal Operation)
01 = Reserved
10 = Input from Digital: DAC1 Left Channel (Digital Resampler)
11 = Input from Digital: DAC2 Left Channel (Digital Resampler)
ADRFLT ADC Right Channel Digital Filter Select and Analog Input Full Scale Mapping Select.  Digital filter selected is over-
ridden when DRSFLT is set to “1.”  See below for filter specifications.  This bit can be altered only if the ADC right
channel is powered down.
15
0= Digital Audio Filter.  2.800 V p-p analog input maps to±2 when gain is set to 0.0 dB (see Control Register
Address 2).
15
1 = Digital Modem Filter.  3.156 V p-p analog input maps to±2 when gain is set to 0.0 dB (see Control Register
Address 2).
ADLFLT ADC Left Channel Digital Filter Select and Analog Input Full Scale Mapping Select.  Digital filter selected is over-
ridden when DRSFLT is set to “1.”  See below for filter specifications.  This bit can be altered only if the ADC left
channel is powered down.
15
0= Digital Audio Filter.  2.800 V p-p analog input maps to±2 when gain is set to 0.0 dB (see Control Register
Address 2).
15
1 = Digital Modem Filter.  3.156 V p-p analog input maps to±2 when gain is set to 0.0 dB (see Control Register
Address 2).
res Reserved for future expansion.  To ensure future compatibility, write “0” to all reserved bits.
Initial default state after reset: 0000 0000 0000 0000 (0000 hex).  Cleared to default and cannot be written to when:
theRESETpin is asserted LO; when thePWRDWNpin is asserted LO; or when the PDNO bit in Control Register
Address 0 is set to “1” (all conversions disabled).
The table below lists the single path (DAC or ADC only) filter characteristics of the “Audio,” “Modem,” and “Resampler” digital
filters.  Passband ripple and group delay must be doubled if the DAC is routed back through the ADC for digital resampling.  For
further information, see the “Filter Selection” section and the filter frequency response plots, Figures 34 through 36.
| Passband | Stopband | Linear |
| --- | --- | --- |
| Filter | Ripple | Edge Ripple Edge Group Delay |
| Audio | –0.016 dB | 0.400×FS < –91.8 dB 0.600×FS < 15/FS |
| Modem | –0.220 dB | 0.442×FS < –75.7 dB 0.542×FS < 19/FS |
| Resampler | –0.035 dB | 0.400×FS < –92.2 dB 0.500×FS < 25/FS FSis the conversion (sample) rate. Address 26 Codec Configuration—Serial Interface |
| Data 15 | Data 14 | Data 13 Data 12 Data 11 Data 10 Data 9 Data 8 |
| DA2SM | DA1SM | res res DA2F1 DA2F0 DA1F1 DA1F0 |
| Data 7 | Data 6 | Data 5 Data 4 Data 3 Data 2 Data 1 Data 0 |
| SCF | FRS | FRST ADTLK ADRF1 ADRF0 ADLF1 ADLF0 –46– REV. 0 |

AD1843
DA2SM DAC2 Stereo/Mono Mode Select.  When in stereo mode, data transmitted to the AD1843 during slot 4 is
used by DAC2 left and data transmitted to the AD1843 during slot 5 is used by DAC2 right.  When in
mono mode, data transmitted to the AD1843 during slot 4 is used by both DAC2 left and right, and data
transmitted to the AD1843 during slot 5 is ignored. (Note: slot 0 is assumed to be the first slot.)
0= Stereo Mode
1 = Mono Mode
DA1SM DAC1 Stereo/Mono Mode Select.  When in stereo mode, data transmitted to the AD1843 during slot 2 is
used by DAC1 left and data transmitted to the AD1843 during slot 3 is used by DAC1 right.  When in
mono mode, data transmitted to the AD1843 during slot 2 is used by both DAC1 left and right, and data
transmitted to the AD1843 during slot 3 is ignored.  (Note: slot 0 is assumed to be the first slot.)
0= Stereo Mode
1 = Mono Mode
DA2F1:0 DAC2 (Left and Right Channels) Data Format Select.
00= 8-bit Unsigned Linear PCM
01 = 16-bit Signed Linear PCM
10 = 8-bitμ-Law Companded
11 = 8-bit A-Law Companded
DA1F1:0 DAC1 (Left and Right Channels) Data Format Select.
00= 8-bit Unsigned Linear PCM
01 = 16-bit Signed Linear PCM
10 = 8-bitμ-Law Companded
11 = 8-bit A-Law Companded
SCF SCLK Frequency Select.  Changes to this bit do not take effect until shortly after the sixth slot (the final slot
owned) is completed. Relevant when the AD1843 is in Master Mode only.
0= 12.288 MHz
1 = 16.384 MHz
FRS Frame Size Select.  Selects the number of slots per frame.  Changes to this bit do not take effect until the
time specified by the FRST bit.
0= 32 Slots per Frame
1 = 16 Slots per Frame
FRST Frame Size Change Timing.  Selects the point in time when FRS takes effect.
0= Frame Size Changes Once the Current Frame is Complete
1 = Frame Size Changes Immediately, Beginning with the Current Frame
ADTLK ADC Transmit Lock Mode Select. When this bit is set to “1,” ADC transmit lock mode is entered. In this
mode, left and right ADC samples are transmitted during a frame only if both left and right samples are
ready to be transmitted. When not in ADC transmit lock mode, left and right samples are transmitted as
they individually become available. Note that even if left and right ADCs are programmed to the same
sample rate, unless the AD1843 is in transmit lock mode, ADC samples will not necessarily be transmitted
paired together in a TDM frame. This bit should be set to “1” only if both ADCs are powered down and
only if both ADCs will be programmed to the sample rate once they are enabled. This bit may be reset to
“0” at any time. While in ADC transmit lock mode, both ADCs must be enabled and disabled simulta-
neously (write to Control Register Address 27 must set ADLEN and ADREN both to “0” or to “1”).
0= ADC Transmit Lock Mode Disabled
1 = ADC Transmit Lock Mode Enabled
ADRF1:0 ADC Right Channel Data Format Select.
00= 8-bit Unsigned Linear PCM
01 = 16-bit Signed Linear PCM
10 = 8-bitμ-Law Companded
11 = 8-bit A-Law Companded
ADLF1:0 ADC Left Channel Data Format Select.
00= 8-bit Unsigned Linear PCM
01 = 16-bit Signed Linear PCM
10 = 8-bitμ-Law Companded
11 = 8-bit A-Law Companded
res Reserved for future expansion.  To ensure future compatibility, write “0” to all reserved bits.
Initial default state after reset: 0000 0000 0000 0000 (0000 hex).  Cleared to default and cannot be written
to when: theRESETpin is asserted LO; or when thePWRDWNpin is asserted LO.
REV. 0 –47–

AD1843
Address 27 Codec Configuration—Channel Power Down
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| DFREE | res | res | DDMEN | res | res | DA2EN | DA1EN |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| ANAEN | HPEN | res | AAMEN DFREE Digital Resource Free.  When set to “1,” this bit reduces the level of power down normally achieved by asserting the power down bits in this Control Register.  However, it allows the digital resources of channels powered down to be partially transferred to the remaining enabled channels and thus permits conversion rates higher than the 48 kHz nominal maximum.  See the “Supported Conversion Rates” section. 0= Power Down of Digital Resources is Complete 1 = Power Down of Digital Resources is Incomplete, Freeing Transferable Resources to Enabled Channels DDMEN DAC2 to DAC1 Mix Enable/Power Down.  When reset to “0,” Control Register Address 3 is cleared to its default and cannot be written. 0= Mix is Powered Down 1 = Mix is Enabled DA2EN DAC2 (Left and Right Channels) Enable/Power Down.  When this bit is reset to “0,” DAC2 is powered down and serial interface requests for samples (see slot 0) will cease immediately.  When this bit is set to “1,” DAC2 is enabled and requests for samples resume on the next frame.  Conversions will not actually begin until the first rising edge of the conversion clock (CONV pin) after the sixth rising edge of frame sync (SDFS pin) after this bit is set to “1.”  This delay allows the four word stereo input buffer for DAC2 to be filled before conversions begin and allows the serial in- terface to synchronize with the conversion clock, which is potentially already running.  This delay also allows DACs on multiple AD1843s sharing the same frame sync to begin conversions synchronously, providing they are enabled at any time during the same frame.  When this bit is reset to “0,” Control Register Address 10, which is the Output Level Control Register for DAC2, is cleared to default and cannot be written. 0= DAC2 is Powered Down, Control Register Address 10 (DAC2 Output Level Control) is Cleared 1 = DAC2 is Enabled DA1EN DAC1 (Left and Right Channels) Enable/Power Down.  When this bit is reset to “0,” DAC1 is powered down and serial interface requests for samples (see slot 0) will cease immediately.  When this bit is set to “1,” DAC1 is enabled and requests for samples resume on the next frame.  Conversions will not actually begin until the first rising edge of the conversion clock (CONV pin) after the sixth rising edge of frame sync (SDFS pin) after this bit is set to “1.”  This delay allows the four word stereo input buffer for DAC1 to be filled before conversions begin and allows the serial in- terface to synchronize with the conversion clock, which is potentially already running.  This delay also allows DACs on multiple AD1843s sharing the same frame sync to begin conversions synchronously, providing they are enabled at any time during the same frame.  When this bit is reset to “0,” Control Register Address 9, which is the Output Level Control Register for DAC1, is cleared to default and cannot be written. 0= DAC1 is Powered Down, Control Register Address 9 (DAC1 Output Level Control) is Cleared 1 = DAC1 is Enabled ANAEN Analog Channel Enable/Power Down.  If this bit is reset to “0,” all analog conversion circuitry (related to bits DA1EN, DA2EN, ADLEN, ADREN, DDMEN and AAMEN) will be powered down.  When reset to “0,” the only conversion channels usable (but must be enabled separately) are the digital resampling paths.  Since resetting this bit to “0” powers down the analog ADC, DAC1, DAC2 and analog mixers, their related output level Control Register Addresses 4 through 7, half of 8, 9 and 10 are cleared and cannot be written while ANAEN remains “0.” 0 = Analog Channels are Powered Down, Control Register Addresses 4 through 7, half of 8, 9 and 10 are Cleared to Default  (Digital to Digital Still Possible) 1=  Analog DAC and ADC Paths are Controlled by DA1EN, DA2EN, ADLEN, ADREN, DDMEN and AAMEN HPEN Headphone Enable/Power Down. 0 = Headphone Output is Powered Down 1= Headphone Output is Enabled AAMEN Analog Input to Analog Output Mix Enable/Power Down.  When reset to “0,” Control Register Addresses 4 - 7 and half of Control Register Address 8 are cleared and cannot be written. 0= Mix is Powered Down 1 = Mix is Enabled –48– REV. 0 | res | res | ADREN | ADLEN |

AD1843
ADREN ADC Right Channel Enable/Power Down.  When this bit is reset to “0,” the right ADC channel is powered down and
serial interface sample output will cease after the current frame.  When this bit is set to “1,” the right ADC channel is
enabled and sampling of analog input will begin on the first rising edge of the conversion clock (CONV pin) after the
sixth rising edge of frame sync (SDFS pin).  This delay allows the ADC startup to be similar to the DAC startup, and
allows some time for stale ADC data inside the AD1843 to be cleared. It also allows the serial interface to synchronize
with the conversion clock, which is potentially already running. Multiple ADCs on multiple AD1843s which share the
same frame sync will be synchronously started if they are enabled at any time during the same frame.
0= ADC Right Channel is Powered Down
1 = ADC Right Channel is Enabled
ADLEN ADC Left Channel Enable/Power Down.  When this bit is reset to “0,” the left ADC channel is powered down and
serial interface sample output will cease after the current frame.  When this bit is set to “1,” the left ADC channel is
enabled and sampling of analog input will begin on the first rising edge of the conversion clock (CONV pin) after the
sixth rising edge of frame sync (SDFS pin).  This delay allows the ADC startup to be similar to the DAC startup, and
allows some time for stale ADC data inside the AD1843 to be cleared. It also allows the serial interface to synchronize
with the conversion clock, which is potentially already running. Multiple ADCs on multiple AD1843s which share the
same frame sync will be synchronously started if they are enabled at any time during the same frame.
0= ADC Left Channel is Powered Down
1 = ADC Left Channel is Enabled
res Reserved for future expansion.  To ensure future compatibility, write “0” to all reserved bits.
Initial default state after reset: 0000 0000 1100 0000 (00C0 hex).  Cleared to default and cannot be written to when:
theRESETpin is asserted LO; when thePWRDWNpin is asserted LO; or when the PDNO bit in Control Register
Address 0 is set to “1” (all conversions disabled).
Address 28 Codec Configuration—Fundamental Settings
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| PDNI | ACEN | C3EN | C2EN | C1EN | ENCLKO | XCTL1 | XCTL0 |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| ENCV3 | ENBT3 | ENCV2 | ENBT2 | ENCV1 | ENBT1 | LINRSD PDNI Converter Power Down.  When set to “1,” this bit initiates the process of powering down all conversion channels, overriding the settings in Control Register Address 27.  Unlike the individual power down bits in Control Register Address 27, asserting this bit allows the DAC and VREFoutput pins to slowly decay to ground.  Setting this bit to “1” results in lower power consumption than results from powering down all channels individually through Control Regis- ter Address 27, and also reduces speaker “click” if asserted before the power supply is removed.  Power down is not complete until approximately 5 ms after this bit is set to “1.”  During this time, the AD1843 still requires a clock in- put from XTALI for proper operation.  When reset to “0,” this bit initiates the process of preparing the AD1843 for conversions after power down.  Approximately 470 ms are necessary to exit power down.  The power up/down status of the AD1843 may be monitored through the PDNO bit in Control Register Address 0.  Asserting the power down pin (PWRDWN) will result in an even greater degree of power down as it also powers down the serial interface and crystal oscillator. Once a power up or power down sequence has been initiated, its completion cannot be interrupted. Changes made to the state of PDNI (i.e., writes to PDNI) are ignored until a previously pending PDNI state change has been completely processed. 0 = Normal (Non-Power Down) Operation 1= Initiate Power Down of All Conversion Channels ACEN Autocalibration Enable.  When set to “1,” autocalibration will occur each time power down is exited (PDNI is changed from a “1” to a “0”).  Autocalibration increases the time required to exit power down by 4 ms (from 470 ms to 474 ms).  When reset to “0,” autocalibration is disabled.  This bit is set to “1” initially after reset and can- not be overwritten until after the AD1843 has come out of power down (PDNO bit in Control Register Address 0 reset to “0”) at least once. Note that an autocalibration cycle is always performed following a hardware reset (i.e., RESETpin asserted) or a hardware power down (i.e.,PWRDWNpin asserted), regardless of the state of ACEN. 0 = Autocalibration Disabled 1= Autocalibration Enabled, Initiated upon Exiting Power Down C3EN Clock Generator 3 Enable/Power Down. 0= Clock Generator 3 Powered Down 1 = Clock Generator 3 Enabled REV. 0 –49– | LINLSD |

AD1843
C2EN Clock Generator 2 Enable/Power Down.
0= Clock Generator 2 Powered Down
1 = Clock Generator 2 Enabled
C1EN Clock Generator 1 Enable/Power Down.
0= Clock Generator 1 Powered Down
1 = Clock Generator 1 Enabled
ENCLKO CLKOUT Pin Enable.
0 = Clock Output is Three-stated (Powered Down)
1= Clock Output is Enabled
XCTL1:0 External Control.  The state of these independent bits is reflected on the respective XCTL1 and XCTL0 output pins.
0= TTL Logic Level LO on XCTL1/XCTL0 pin
1 = TTL Logic Level HI on XCTL1/XCTL0 pin
ENCV3 CONV3 Pin Enable.
0= CONV3 is Three-stated (Powered Down)
1 = CONV3 is Enabled
ENBT3 BIT3 Pin Enable.
0= BIT3 is Three-stated (Powered Down)
1 = BIT3 is Enabled
ENCV2 CONV2 Pin Enable.
0= CONV2 is Three-stated (Powered Down)
1 = CONV2 is Enabled
ENBT2 BIT2 Pin Enable.
0= BIT2 is Three-stated (Powered Down)
1 = BIT2 is Enabled
ENCV1 CONV1 Pin Enable.
0= CONV1 is Three-stated (Powered Down)
1 = CONV1 is Enabled
ENBT1 BIT1 Pin Enable.
0= BIT1 is Three-stated (Powered Down)
1 = BIT1 is Enabled
LINRSD Line Input Right Channel Single-Ended or Differential Configuration.
0= Right Channel Line Input is Single-Ended
1 = Right Channel Line Input is Differential
LINLSD Line Input Left Channel Single-Ended or Differential Configuration.
0= Left Channel Line Input is Single-Ended
1 = Left Channel Line Input is Differential
res Reserved for future expansion.  To ensure future compatibility, write “0” to all reserved bits.
Initial default state after reset: 1100 0100 0000 0000 (C400 hex).  Cleared to default and cannot be written to when:
theRESETpin is asserted LO; or when thePWRDWNpin is asserted LO.
Address 29 Reserved for Future Expansion
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| res | res | res | res | res | res | res | res |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| res | res res Reserved for future expansion.  To ensure future compatibility, write “0” to all reserved bits. Initial default state after reset: 0000 0000 0000 0000 (0000 hex).  Cleared to default and cannot be written to when: theRESETpin is asserted LO; or when thePWRDWNpin is asserted LO. –50– REV. 0 | res | res | res | res | res | res |

AD1843
Address 30 Reserved for Future Expansion
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| res | res | res | res | res | res | res | res |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| res | res res Reserved for future expansion.  To ensure future compatibility, write “0” to all reserved bits. Initial default state after reset: 0000 0000 0000 0000 (0000 hex).  Cleared to default and cannot be written to when: theRESETpin is asserted LO; or when thePWRDWNpin is asserted LO. Address 31 Reserved for Future Expansion | res | res | res | res | res | res |
| Data 15 | Data 14 | Data 13 | Data 12 | Data 11 | Data 10 | Data 9 | Data 8 |
| res | res | res | res | res | res | res | res |
| Data 7 | Data 6 | Data 5 | Data 4 | Data 3 | Data 2 | Data 1 | Data 0 |
| res | res res Reserved for future expansion.  To ensure future compatibility, write “0” to all reserved bits. Initial default state after reset: 0000 0000 0000 0000 (0000 hex).  Cleared to default and cannot be written to when: theRESETpin is asserted LO; or when thePWRDWNpin is asserted LO. BIT AND REGISTER MAPS A map of all TDM time slot bit assignments and Control Register contents is summarized for reference as follows in Figure 14 and Figure 15: | res | res | res | res | res | res |
| INPUT SLOT | DATA 15DATA 14 DATA 13DATA 12 DATA 11DATA 10 DATA 9 | DATA 8 | DATA 7 | DATA 6 | DATA 5 | DATA 4 | DATA 3 DATA 2 DATA 1 DATA 0 |
| 0 OR 16 | RES | RES | RES | RES | RES | RES | DA2V DA1V R/W RES RES IA4 IA3 IA2 IA1 IA0 |
| 1 OR 17 | DATA 15DATA 14 DATA 13DATA 12 DATA 11DATA 10 DATA 9 | DATA 8 | DATA 7 | DATA 6 | DATA 5 | DATA 4 | DATA 3 DATA 2 DATA 1 DATA 0 |
| 2 OR 18 | DATA 15DATA 14 DATA 13DATA 12 DATA 11DATA 10 DATA 9 | DATA 8 | DATA 7 | DATA 6 | DATA 5 | DATA 4 | DATA 3 DATA 2 DATA 1 DATA 0 |
| 3 OR 19 | DATA 15DATA 14 DATA 13DATA 12 DATA 11DATA 10 DATA 9 | DATA 8 | DATA 7 | DATA 6 | DATA 5 | DATA 4 | DATA 3 DATA 2 DATA 1 DATA 0 |
| 4 OR 20 | DATA 15DATA 14 DATA 13DATA 12 DATA 11DATA 10 DATA 9 | DATA 8 | DATA 7 | DATA 6 | DATA 5 | DATA 4 | DATA 3 DATA 2 DATA 1 DATA 0 |
| 5 OR 21 | DATA 15DATA 14 DATA 13DATA 12 DATA 11DATA 10 DATA 9 | DATA 8 | DATA 7 | DATA 6 | DATA 5 | DATA 4 | DATA 3 DATA 2 DATA 1 DATA 0 |
| OUTPUT SLOT DATA 15DATA 14 DATA 13DATA 12 DATA 11DATA 10 DATA 9 | DATA 8 | DATA 7 | DATA 6 | DATA 5 | DATA 4 | DATA 3 | DATA 2 DATA 1 DATA 0 |
| 0 OR 16 | RES | RES | RES | RES | RES | RES | ADRV ADLV RES RES RES RES RES RES DA2RQ DA1RQ |
| 1 OR 17 | DATA 15DATA 14 DATA 13DATA 12 DATA 11DATA 10 DATA 9 | DATA 8 | DATA 7 | DATA 6 | DATA 5 | DATA 4 | DATA 3 DATA 2 DATA 1 DATA 0 |
| 2 OR 18 | DATA 15DATA 14 DATA 13DATA 12 DATA 11DATA 10 DATA 9 | DATA 8 | DATA 7 | DATA 6 | DATA 5 | DATA 4 | DATA 3 DATA 2 DATA 1 DATA 0 |
| 3 OR 19 | DATA 15DATA 14 DATA 13DATA 12 DATA 11DATA 10 DATA 9 | DATA 8 | DATA 7 | DATA 6 | DATA 5 | DATA 4 | DATA 3 DATA 2 DATA 1 DATA 0 |
| 4 OR 20 | RES | RES | RES | RES | RES | RES | RES RES RES RES RES RES RES RES RES RES |
| 5 OR 21 | RES | RES | RES | RES | RES | RES | RES RES RES RES RES RES RES RES RES RES Figure 14.  AD1843 TDM Time Slot Bit Assignment Map REV. 0 –51– |

AD1843
| CONTROL REG DATA 15 DATA 14DATA 13DATA 12 DATA 11DATA 10 DATA 9 | DATA 8 | DATA 7 | DATA 6 DATA 5 DATA 4 | DATA 3 | DATA 2 | DATA 1 | DATA 0 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| ADDRESS 0 | INIT | PDNO | RES | RES | RES | RES | RES RES RES RES RES RES ID3 ID2 ID1 ID0 |
| ADDRESS 1 | RES | RES | RES | RES | RES | RES | SU2 SU1 RES RES RES RES OVR1 OVR0 OVL1 OVL0 |
| ADDRESS 2 | LSS2 | LSS1 | LSS0 | LMGE | LIG3 | LIG2 | LIG1 LIG0 RSS2 RSS1 RSS0 RMGE RIG3 RIG2 RIG1 RIG0 |
| ADDRESS 3 | LD1MM | RES | RES | LD1M4 | LD1M3 | LD1M2 | LD1M1 LD1M0 RD1MM RES RES RD1M4 RD1M3 RD1M2 RD1M1 RD1M0 |
| ADDRESS 4 | LX1MM | RES | RES | LX1M4 | LX1M3 | LX1M2 | LX1M1 LX1M0 RX1MM RES RES RX1M4 RX1M3 RX1M2 RX1M1 RX1M0 |
| ADDRESS 5 | LX2MM | RES | RES | LX2M4 | LX2M3 | LX2M2 | LX2M1 LX2M0 RX2MM RES RES RX2M4 RX2M3 RX2M2 RX2M1 RX2M0 |
| ADDRESS 6 | LX3MM | RES | RES | LX3M4 | LX3M3 | LX3M2 | LX3M1 LX3M0 RX3MM RES RES RX3M4 RX3M3 RX3M2 RX3M1 RX3M0 |
| ADDRESS 7 | LMCMM RES | RES | LMCM4 | LMCM3 | LMCM2 | LMCM1 | LMCM0 RMCMM RES RES RMCM4 RMCM3 RMCM2 RMCM1 RMCM0 |
| ADDRESS 8 | MNMM | RES | RES | MNM4 | MNM3 | MNM2 | MNM1 MNM0 ALLMM MNOM HPOM HPOS SUMM RES DAC2T DAC1T |
| ADDRESS 9 | LDA1GM RES | LDA1G5 LDA1G4 LDA1G3 LDA1G2 LDA1G1 LDA1G0 RDA1GM RES | RDA1G5 RDA1G4 RDA1G3 RDA1G2 RDA1G1 RDA1G0 |  |  |  |  |
| ADDRESS 10 LDA2GM RES | LDA2G5 LDA2G4 LDA2G3 LDA2G2 LDA2G1 LDA2G0 RDA2GM RES | RDA2G5 RDA2G4 RDA2G3 RDA2G2 RDA2G1 RDA2G0 |  |  |  |  |  |
| ADDRESS 11 LDA1AM RES | LDA1A5 LDA1A4 LDA1A3 LDA1A2 | LDA1A1 LDA1A0 RDA1AM RES | RDA1A5 RDA1A4 RDA1A3 RDA1A2 RDA1A1 RDA1A0 |  |  |  |  |
| ADDRESS 12 | LDA2AM RES | LDA2A5 LDA2A4 LDA2A3 LDA2A2 | LDA2A1 LDA2A0 RDA2AM RES | RDA2A5 RDA2A4 RDA2A3 RDA2A2 RDA2A1 RDA2A0 |  |  |  |
| ADDRESS 13 | LAD1MMRES | LAD1M5 LAD1M4 LAD1M3 LAD1M2 LAD1M1 LAD1M0 RAD1MMRES | RAD1M5RAD1M4 RAD1M3 RAD1M2 RAD1M1 RAD1M0 |  |  |  |  |
| ADDRESS 14 | LAD2MMRES | LAD2M5 LAD2M4 LAD2M3 LAD2M2 LAD2M1 LAD2M0 RAD2MMRES | RAD2M5RAD2M4 RAD2M3 RAD2M2 RAD2M1 RAD2M0 |  |  |  |  |
| ADDRESS 15 | RES | RES | RES | RES | DA2C1 | DA2C0 | DA1C1 DA1C0 RES RES RES RES ADRC1 ADRC0 ADLC1 ADLC0 |
| ADDRESS 16 | C1REF | C1VID | C1PLLG C1P200 | C1X8/7 | C1C128 | RES | RES C1M7 C1M6 C1M5 C1M4 C1M3 C1M2 C1M1 C1M0 |
| ADDRESS 17 | C1C15 | C1C14 | C1C13 | C1C12 | C1C11 | C1C10 | C1C9 C1C8 C1C7 C1C6 C1C5 C1C4 C1C3 C1C2 C1C1 C1C0 |
| ADDRESS 18 | RES | RES | RES | RES | RES | RES | RES C1PD C1P7 C1P6 C1P5 C1P4 C1P3 C1P2 C1P1 C1P0 |
| ADDRESS 19 | C2REF | C2VID | C2PLLG C2P200 | C2X8/7 | C2C128 | RES | RES C2M7 C2M6 C2M5 C2M4 C2M3 C2M2 C2M1 C2M0 |
| ADDRESS 20 | C2C15 | C2C14 | C2C13 | C2C12 | C2C11 | C2C10 | C2C9 C2C8 C2C7 C2C6 C2C5 C2C4 C2C3 C2C2 C2C1 C2C0 |
| ADDRESS 21 | RES | RES | RES | RES | RES | RES | RES C2PD C2P7 C2P6 C2P5 C2P4 C2P3 C2P2 C2P1 C2P0 |
| ADDRESS 22 | C3REF | C3VID | C3PLLG C3P200 | C3X8/7 | C3C128 | RES | RES C3M7 C3M6 C3M5 C3M4 C3M3 C3M2 C3M1 C3M0 |
| ADDRESS 23 | C3C15 | C3C14 | C3C13 | C3C12 | C3C11 | C3C10 | C3C9 C3C8 C3C7 C3C6 C3C5 C3C4 C3C3 C3C2 C3C1 C3C0 |
| ADDRESS 24 | RES | RES | RES | RES | RES | RES | RES C3PD C3P7 C3P6 C3P5 C3P4 C3P3 C3P2 C3P1 C3P0 |
| ADDRESS 25 | DRSFLT DAMIX | RES | RES | RES | RES | DA2FLT DA1FLT DAADR1 DAADR0DAADL1 DAADL0 RES | RES ADRFLT ADLFLT |
| ADDRESS 26 | DA2SM | DA1SM | RES | RES | DA2F1 | DA2F0 | DA1F1 DA1F0 SCF FRS FRST ADTLK ADRF1 ADRF0 ADLF1 ADLF0 |
| ADDRESS 27 | DFREE | RES | RES | DDMEN RES | RES | DA2EN | DA1EN ANAEN HPEN RES AAMEN RES RES ADREN ADLEN |
| ADDRESS 28 | PDNI | ACEN | C3EN | C2EN | C1EN | ENCLKO XCTL1 | XCTL0 ENCV3 ENBT3 ENCV2 ENBT2 ENCV1 ENBT1 LINRSD LINLSD |
| ADDRESS 29 | RES | RES | RES | RES | RES | RES | RES RES RES RES RES RES RES RES RES RES |
| ADDRESS 30 | RES | RES | RES | RES | RES | RES | RES RES RES RES RES RES RES RES RES RES |
| ADDRESS 31 | RES | RES | RES | RES | RES | RES | RES RES RES RES RES RES RES RES RES RES Figure 15.  AD1843 Control Register Map Figure 16 shows an annotated version of the AD1843 detailed Register readback (on the following frame) matches the data block diagram, indicating the specific Control Register bit fields written.  Writes to these Control Registers will fail until the which configures the various features of the SoundComm codec. AD1843 internal clocks are stabilized.  If the system has been START-UP SEQUENCE designed for 16 slots per frame, it is suggested that this first The following paragraphs describe a typical, generic start-up “polling write/readback” is to Control Register Address 26, sequence, for the purpose of helping hardware, systems and with FRS (Bit 6) set to “1” and FRST (Bit 5) set to “1,” software driver engineers understand some of the considerations so that the AD1843 will support the 16 slot per frame mode involved in bringing up a system which includes the AD1843. immediately. Note that it does not exhaustively outline all of the flexible con- 4.Put the conversion resources into standby.  With the ex- figurations and features available in the AD1843. ception of the serial interface, the AD1843 is still completely 1.System power supplies stabilize.Ideally, the AD1843 powered down before this step.  The PDNI bit (Control Reg- RESETpin should be held LO during this period.  If ister Address 28, Bit 15) should now be reset to “0” to ini- RESETis not asserted, the AD1843 will come up in an un- tiate the process of taking the AD1843 conversion resources known state. out of power down and into standby.  This requires approxi- mately 470 ms.  An autocalibration cycle always occurs 2.Assert theRESETsignal (PLCC Pin 52, TQFP Pin 65). (automatically, without user programming or intervention) Once the power supplies have stabilized, the AD1843RE- following the deassertion of theRESETpin or the SETmust be asserted LO for at least 100 ns with BM PWRDWNpin (i.e., hardware reset or hardware power (PLCC Pin 10, TQFP Pin 12) tied to the appropriate level to down), adding another approximately 4 ms, for a total of establish whether the codec is serial bus master or slave. approximately 474 ms. Subsequent software power-down 3.Deassert theRESETsignal and enter a wait period to cycles, programmed using Control Register bits, do not ordi- allow the AD1843 internal clocks and the external crys- narily require additional calibration cycles. The original cali- tal oscillator to stabilize. The wait period duration will be bration information is retained during the power down typically 400μs to 800μs afterRESETis deasserted, but sig- sequence. However, the user can force an autocalibration to nificantly longer time may be necessary depending on occur following a software power down by setting ACEN XTALI and XTALO pin parasitics.  If the AD1843 is serial (Control Register Address 28, Bit 14) to “1.” The host CPU bus master, the host CPU or DSP can poll the INIT bit or DSP can poll the PDNO bit (Control Register Address 0, (Control Register Address 0, Bit 15) to determine when the Bit 14) to determine when the conversion resources are out AD1843 is ready to proceed.  Alternatively, the host CPU or of power down.  Alternatively, the host CPU or DSP can DSP can write to Control Register 16 through 24, or 26 or write to Control Register Address 27 (using a data pattern 28 (using a data pattern different from the initial reset default different from the initial reset default), and proceed when the for that Control Register), and proceed when the Control –52– REV. 0 |

AD1843
CLKOUT SCLKSDFSSDISDOBMCS TSOTSIXCTL [1:0]RESETPWRDWN
2
DIGITAL INTERFACE
ADC	DAC1	DAC2
BIT1
CONTROL
AND STATUSREGISTERS
μ/ALAW	μ/ALAW	μ/ALAW
BIT2 DD
V
FIFO FIFO
BIT3 8 9
LEFTRIGHT	ATTN	MUTE	∑
GNDD
LAD2M[14]RAD2M[14]LAD2MM[14]RAD2MM[14]
∑
∑
ATTN	MUTE	∑
LEFTRIGHT
LAD1M[13]RAD1M[13]LAD1MM[13]RAD1MM[13]
ATTN ATTN
LDA1A[11]RDA1A[11]	LDA2A[12]RDA2A[12]	3
CC
V
AD1843
CLOCK GENERATION
MUTE MUTE
LDA1AM[11]RDA1AM[11] LDA2AM[12]RDA2AM[12]
(0)SELECTORDAADL[25]DAADR[25]
∑ ∑
4
(2)SELECTORDAADL[25]DAADR[25](3)	∑	MUTE	MUTE	∑	GNDA
DAMIX[25] DAMIX[25]
[ ] INDICATES CONTROL REGISTER ADDRESS( ) INDICATES CONTROL REGISTER DATA
LEFTRIGHT
XTALI    XTALO   SYNC3   SYNC2   SYNC1   CONV3   CONV2   CONV1
FILTR
DAC DAC
∑∆ ∑∆
ADC	DA1EN[27]	DA2EN[27]
∑∆ FILTL
ADLEN[27]ADREN[27]	LEFT	RIGHT
GN/AT
LDA1G[9]RDA1G[9]
PGALIG[2]
RIG[2]
SELECTOR CMOUT
GN/AT = GAIN/ATTENUATION
MUTE
(0) (1)
LSS[2](2)RSS[2](3)	(4) (5) (6) (7)	LDA1GM[9]RDA1GM[9]
REF
V
∑ VOLTAGE REFERENCE
GN/ATMUTE MUTE
LMCM[7]RMCM[7]LMCMM[7]RMCMM[7]ALLMM[8]∑ GN/AT
LD2M[3]RD2M[3]
LD2MM[3]RD2MM[3]ALLMM[8]
∑
GN/ATMUTE
LX1M[4]RX1M[4]LX1MM[4]RX1MM[4]ALLMM[8]∑
GN/AT
∑ LDA2G[10]
RDA2G[10]
GN/ATLX2M[5]RX2M[5]MUTE
LX2MM[5]RX2MM[5]ALLMM[8]∑
20dB ∑
GN/ATLX3M[6]RX3M[6]MUTE AAFILTR
LX3MM[6]RX3MM[6]ALLMM[8]∑ MUTE
∑ LDA2GM[10]RDA2GM[10]
GN/ATMNM[8]MUTE MUTE
LMGE[2]RMGE[2] MNMM[8]ALLMM[8]∑
SUMM[8] AAFILTL
MUTEMUTE
MNOM[8]
MUTE
DRIVER HPOM[8]
HPEN[27]
MIN
MICLMICR SUML
LINLPLINLNLINRPLINRN	AUX1LAUX2LAUX3L	MOUT	SUMR
AUX1RAUX2RAUX3R
LOUT1LLOUT1R HPOUTLHPOUTCHPOUTR
PDMNFT LOUT2LPLOUT2LNLOUT2RPLOUT2RN
Figure 16.  AD1843 Annotated Detailed Block Diagram with Control Register Address and Data Values
REV. 0 –53–

AD1843
WRITABLE
| AD1843 STATE | ACTIVITY LEVEL RESET= LO | REGISTERS* |
| --- | --- | --- |
| ALL DOWN | INTERFACE: DOWN | NONE |
| PWRDWN= LO | INIT = 1 | PWRDWN= HI,RESET= HI CLOCK GENERATORS: DOWN PDNO =1 CONVERSIONS: DOWN |
| INITIALIZING | INTERFACE: READS ONLY IF SLAVE, INIT = 1 DOWN IF MASTER | NONE |
| PWRDWN= LO | PDNO = 1 | CLOCK GENERATORS: DOWN TIME: 800μs? CONVERSIONS: DOWN CONVERSION PDNI = 1 INTERFACE: ALIVE 16-24, 26, 28 |
RESOURCES
CLOCK GENERATORS: ENABLEABLE
POWER DOWN
| PWRDWN= HI | INIT = 0 | PDNI = 0 | PDNI = 0 | (CONTROL REGISTER ADDRESS 28) PDNO = 1 CONVERSIONS: DOWN CONVERSION RESOURCES CONVERSION RESOURCES ENTERING POWER DOWN EXITING POWER DOWN INIT = 0 INIT = 0 |
| --- | --- | --- | --- | --- |
| PDNO = 1 | PDNO = 1 | INTERFACE: ALIVE ACEN = 1 | 16-24, 26, 28 |  |
| TIME: < 5ms | TIME: < 470ms | CLOCK GENERATORS: ENABLEABLE (CONTROL REGISTER ADDRESS 28) AUTOCAL CONVERSIONS: DOWN |  |  |
| PDNI = 1 OR | ACEN = 0 PWRDWN= LO PDNO = 1 TIME: < 4ms CONVERSION RESOURCES POWERED UP | INIT = 0 |  |  |
| INIT = 0 | INTERFACE: ALIVE PDNO = 0 CLOCK GENERATORS: ENABLEABLE DEPENDING ON | POTENTIALLY ALL, |  |  |
| ADCL | ADCR (CONTROL REGISTER ADDRESS 28) ENABLED | DAC1 | DAC2 |  |
| STBY | STBY | STBY | STBY | CONVERSIONS: ENABLEABLE RESOURCES (CONTROL REGISTER ADDRESS 27) (SEE TEXT) |
| EN | EN = STATE = TRANSMISSION (WILL NOT BE INTERRUPTED ONCE STARTED UNLESSRESET= LO) *REGISTERS ARE ALWAYS READABLE, UNLESSRESET= LO Figure 17.  AD1843  Start-Up Sequence State Diagram Control Register readback (on the following frame) matches digital-to-digital sample rate conversion and DAC-to-DAC the data written.  Writes to Control Register Address 27 will digital mixing.  Note that, with the exception of filter mode fail unless the conversion resources are out of power down. and DAC-to-DAC digital mixing, all of these configuration selections may also be changed while a conversion resource is 5.Power up the clock generators and enable clock output out of standby and enabled. pins.When the PDNI bit (Control Register Address 28, Bit 15) is reset to “0” in Step 4 above, the C3EN, C2EN, 7.Enable conversion resources.  Conversion resources are C1EN, CONV[3:1], and BIT[3:1] bits, which are also lo- taken out of standby and enabled by writes to Control Regis- cated in Control Register Address 28, may be set to “1” to ter Address 27.  Control Register Address 27 contains indi- power up any clock generators which will be used, and to vidual enable bits for the conversion resources which are enable any conversion clock and bit clock outputs needed. grouped as follows: DAC1 (stereo pair); DAC2 (stereo pair); While waiting for the conversion resources to enter standby, ADC left channel; ADC right channel; DAC2 to DAC1 mix; it is convenient to write to the clock generator Control Regis- and ADC input to DAC1 output mix. ters (16 through 24) to select sample rates, bit clock frequen- If a DAC is enabled, it will begin requesting playback cies, external/internal sample rate lock mode, etc.  This is samples on the next TDM frame (i.e., the DA2RQ and/or also a convenient time to write to Control Register Address the DA1RQ flags in the Status Register will be set to “1”). 26 (if it was not already written in Step 3) to select the serial Conversions will not actually begin until the first rising edge interface configuration such as data format, SCLK fre- of the conversion clock (assigned to the DAC) after the sixth quency, etc. rising edge of the frame sync (SDFS) signal.  This delay al- 6.Configure conversion resources while they are in lows the four word deep stereo FIFOs to be filled before con- standby.Once in standby, the conversion resources may be versions begin.  (Six frames are required to fill a four word configured.  They are forced to their defaults whenever pow- deep FIFO because on the first frame, the DAC playback is ered down.  Control Register Address 2 may be written to se- enabled; on the second frame, the DA2RQ and/or DA1RQ lect between ADC sources and gain levels.  Control Register flags are set to “1”; then four more frames are required to ac- Address 15 may be written to select the assignments of tually fill the FIFO.) sample clock sources to conversion resource.  Control Regis- If an ADC is enabled, sampling of the analog input will be- ter Addresses 13 and 14 may be written to enable digital data gin on the first rising edge of the conversion clock (assigned flow paths which mix ADC output back into the DAC input. to the ADC) after the sixth rising edge of the frame sync Control Register Address 25 may be written to select conver- (SDFS) signal.  The data valid flags (ADRV and/or ADLV) sion resource filter mode (audio, modem or resampler), and in the Status Word will be set to a “1,” indicating a sample is to select digital data flow paths which may be used to achieve being transmitted shortly thereafter.  The exact TDM frame –54– REV. 0 | EN | EN |  |

AD1843
in which the valid flag is first asserted, and continues to be A circuit for 2 V rms line-level inputs is shown in Figure 18.
asserted in future frames, depends on how the AD1843 is Note that this is a divide-by-two resistive divider, and that the
configured, i.e., the sample rates selected, the frame size line input is being used in a single-ended configuration.  The
selected, and how the shared digital resources within the 1μF ac-coupling capacitor may be of any type (tantalum is a
AD1843 are internally allocated to process all conversion popular choice).
channels.  Note that because of this, two ADCs which are
1μF
5.1k
enabled during the same frame and share the same sample
LINRP
rate clock, will not necessarily transmit data during the same
1nF 5.1k
frames thereafter, even though their analog inputs will always
be sampled at the same instant in time. The two ADCs will 1μF
5.1k
transmit data during the same frame if ADTLK (Control LINLP
| Register Address 26, Bit 4) is set HI. See the description of | 1nF | 5.1k the ADTLK bit for restrictions. If mixing is enabled, either a mix from DAC2 to DAC1 or a Figure 18.  AD1843 2 V rms Single-Ended Line-Level mix from an ADC input to a DAC output, then the mix will Input Circuit begin during the same TDM frame that it is enabled. If line-level inputs are already at the 1 V rms levels expected by 8.Configure conversion resources while they are enabled. the AD1843, the circuit shown in Figure 19 below should be Once enabled, DAC output gain/attenuation levels may be used. Note that when single-ended line inputs are desired, only changed using Control Register Addresses 3 through 10. the LINRP and LINLP pins are used. DO NOT use the These registers are forced to their default of full attenuation LINRN or LINLN pins if the line input is single-ended. whenever the resource they control is either in standby or 1μF completely powered down.  Note that while a conversion re- LINRP source is enabled, all configuration selections related to it 1μF may be changed except those outlined in Step 6. LINLP The AD1843 Start-Up Sequence state diagram is shown in Figure 19.  AD1843 1 V rms Single-Ended Line-Level Figure 17. Input Circuit APPLICATIONS CIRCUITS The three auxiliary inputs, the SUM inputs, and the mono in- The AD1843 SoundComm Codec has been designed to require put of the AD1843 present an input impedance which is lower a minimum of external circuitry.  The recommended circuits are than the stereo line input.  The circuit shown in Figure 20 shown in Figures 18 through 34.  Analog Devices estimates that should be used with 2 V rms swings on these inputs.  With 1 V the total cost of all the components shown in these Figures (in- rms swings, the circuit in Figure 19 above should be used. cluding the crystal but not including the DAA) to be less than |
| --- | --- | --- |
| 1μF | 3.3k 3.3k AUX1R $6 in 10,000 piece quantities. AUX1L AUX2L AUX2R | 1μF |
| Industry-standard compact disc “line-levels” are 2 Vrms cen- | 1nF | 4.1k AUX3L 1nF 4.1k AUX3R SUML SUMR tered around analog ground.  (For other audio equipment, “line level” is much more loosely defined.) The AD1843SoundComm 1μF 3.3k Codec is a +5 V analog supply powered device.  Nominal line MIN |
| level voltage swings for the AD1843 are defined to be 1 V rms | 1nF | 4.1k (ADRFLT and ADLFLT = 0) for a sine wave ADC input and 0.707 V rms for a sine wave DAC output (DA2FLT = 0). Thus, 2 V rms input analog signals must be attenuated and either cen- Figure 20.  AD1843 2 V rms Aux, SUM, and Mono Input tered around the reference voltage intermediate between 0 V Circuits and +5 V or ac-coupled.  The CMOUT pin will be at this inter- For optimal performance, the AD1843 includes provision for a mediate voltage, nominally 2.25 V.  It has limited drive but can differential configuration of the LIN inputs.  Figure 21 illus- be used as a voltage datum to an op amp input.  CMOUT load- trates a simple single-ended to differential converter.  For the ing should be minimized to limit any audible clicks and pops. best noise immunity, the circuitry to the left of the dotted line Note that dc-coupled inputs are not recommended, as they pro- should be located as close to the driving signal source as possible. vide no performance benefits with the AD1843 architecture. The 0.9K resistor and the 1000 pF capacitor are manda- Furthermore, dc offsets on inputs create the potential for clicks tory when using the LINRN and/or the LINLN inputs. when changing the input mix gain/attenuation/mute. Furthermore, the 0.9K resistor and the 1000 pF NPO capacitor TheRESETpin must be asserted at or shortly after power up in should be located as close to the AD1843 as possible.  If the order to initialize the AD1843 Control Registers to their default output range of the source does not match that of the AD1843 states.  If the AD1843 will not be used immediately, and if it is input, the op amp gain setting resistor values should be scaled desired to utilize the mono input to mono output feedthrough to match the source voltage range to the AD1843 input voltage feature, it is essential that the mono output is ac-coupled to pre- range. vent audible pops and clicks.  It is recommended that all outputs Inexpensive feedback capacitors can be added (as shown in the are ac coupled, as standing current in output loads or in the Figure 21 with dotted lines) for additional noise filtering (6 dB voltage reference will contribute to audible pops and clicks. per octave).  Without the additional filter capacitor, there is a single antialiasing pole at approximately 160 kHz.  There is REV. 0 –55– |

AD1843
1μF
“room” in the frequency domain for a second pole, especially in LOUT1L
modem applications, where the signal bandwidth is 4.2 kHz.  A LOUT2LP
47k
suggested capacitor value is 120 pF when using the 10K op amp
feedback resistor as shown, for a –3 dB point of approximately
1μF
133 kHz.  Note that the combined effect of both filters causes a LOUT1R
LOUT2RP
gain error of approximately 0.1% at 4.2 kHz.
47k
CLOSE TO SOURCE CLOSE TO AD1843
Figure 23.  AD1843 (Single-Ended) Line Output
1μF 1μF
10k	10k	0.9k
Connections
SOURCE LINRN
LINLN
1000pF
For optimal performance, the AD1843 DAC2 output has provi-
| 10k | 10k sion for a differential configuration.  A simple differential-to- CMBUF 1μF single-ended conversion circuit is shown in Figure 24.  The 1/2 LINLP | NPO |
| --- | --- | --- |
| SSM2135 | CMBUF | LINRP noise rejection of this circuit is limited by the match of the Ri 1/2 SSM2135 and Rfresistors.  The resistors should be 1% tolerance compo- nents.  The equation that determines the output voltage is as Figure 21. Single-Ended to Differential Converter follows: The AD1843 Codec contains an optional +20 dB gain block to VO= (Rf/Ri)(V+) – (Rf/Ri)(V–) accommodate condenser microphones.  Particular system re- For maximum noise immunity, this circuit should be located in quirements will depend upon the characteristics of the intended close proximity to where VOis processed.  Another differential microphone.  Figure 22 illustrates one example of how an elec- receiver option is the SSM2141, which provides better matching tret condenser mic requiring phantom power could be con- than a discrete implementation. nected to the AD1843.  CMOUT is shown buffered by an op 10pF 10pF amp; the current drawn from CMOUT should be as minimal as |
| possible.  Note that if a battery-powered microphone is used, | LOUT2LN | LOUT2RN |
| (V–) | R | R (V–) Ri Rf i f the buffer and R2s are not needed.  The values of R1, R2, and |
| C should be chosen in light of the mic characteristics and in- | LOUT2LP VO VO | LOUT2RP |
| (V+) | Ri | (V+) i |
R
tended gain.  Typical values for these might be R1 = 20 kΩ,
1/2 1/2
R2 = 2 kΩ, and C = 220 pF.  Figure 22 shows a voltage follower	SSM2135	SSM2135
10pF 10pF
Rf Rf
buffer on the CMOUT signal. The output of this buffer, CMBUF
CMBUF
CMBUF, can be used in any circuit which needs the common-
mode bias point from the AD1843’s on-chip voltage reference.
1/2 1/2
The AD820 is a JFET input op amp whose very high imped-	SSM2135	SSM2135
ance inputs present essentially no load on the CMOUT output
from the AD1843. Figure 24.  AD1843 Differential-to-Single-Ended Converter
C Line Output Connections
1μF A circuit for headphone drive is illustrated in Figure 25.  The
5k R1
AD1843 headphone output is designed to drive loads of 32
0.33μF
LEFT ELECTRET
CONDENSER ohms or smaller (i.e., higher impedance).  If larger loads are
MICL
MICROPHONE
used (e.g., 16 ohms or 8 ohms), the analog output will be dis-
INPUT	1/2 AD820	1/2 SSM2135
R2 OR AD820
torted for large output signals because of current limiting.
CMOUT
“Walkman”-type headphone impedances are generally around
1/2 SSM2135 32 ohms. Telephone handset impedances are typically 150 ohms.
R2
CMBUF OR AD820
RIGHT ELECTRET HPOUTL
CONDENSER 1μF
5k MICR
MICROPHONE HPOUTR
INPUT 0.33μF
R1
C HPOUTC
Figure 22.  AD1843 “Phantom-Powered” Microphone Figure 25.  AD1843 Headphone Drive Connections
Input Circuit
Figure 26 shows an example circuit for the mono output
Connect unused analog inputs to CMBUF or leave them (MOUT) from the AD1843. The OP279 is a single +5 V supply
unconnected; do not connect unused analog inputs to op amp which can drive a small 8 ohm or 16 ohm speaker to
either analog power or ground! modest levels.
Figure 23 shows ac-coupled line outputs.  The resistors are used
to center the output signals around analog ground.  If dc-
coupling is desired, CMOUT could be used with op amps as
mentioned above, if desired.
–56– REV. 0

AD1843
10k
required and that external clock sources can be used in place of
10k the AD1843’s internal oscillator.  If using an external clock
MOUT	16	470μF
source, apply it to the crystal input pin (XTALI) while leaving
TO PC
CMBUF	SPEAKER	the crystal output pin (XTALO) unconnected.  Attention
1/2 OPTIONAL
OP279
XTALI XTALO
Figure 26.  AD1843 Mono Output Circuit
24.576 MHz
Figure 27 illustrates reference bypassing.  V	should only be	20–64pF	20–64pF
REF
connected to its bypass capacitors.  The 10μF capacitor should
be tantalum, and the 0.1μF capacitor should be ceramic. Figure
Figure 30.  AD1843 Crystal Connections
27 shows an optional voltage follower buffer on the CMOUT
should be paid to providing a low-jitter external input clock.
signal. The output of this buffer, CMBUF, can be used in any
Ideally, there should be no greater than 5 ns rms of random
circuit which needs the common-mode bias point from the
(white) phase jitter to ensure that it does not significantly de-
AD1843’s on-chip voltage reference. The AD820 is a JFET in-
grade the SNR of the AD1843.
put op amp whose very high impedance inputs present essen-
tially no load on the CMOUT output from the AD1843. Figure 31 and Figure 32 show example circuits for a Data
Access Arrangement (DAA).  These circuits are shown as
OPTIONAL BUFFER
examples only;DAA circuits are subject to regulatory approval
VREF	CMOUT	since they connect directly to the Public Switched Telephone
CMBUF
10μF	0.1μF	10μF	0.1μF	Network (PSTN).
1/2
AD820 LOUT2LN
FROM AD1843
LOUT2LP
SSI 73M9001
| Figure 27.  AD1843 Voltage Reference Bypassing | 23 TXA RXA | 21 | TO |
| --- | --- | --- | --- |
| 22 | 12 | AD1843 LINLP |  |
| 220Ω | TXA/ | SPK |  |
| Figure 28 illustrates the signal-path filtering capacitors, FILTL | 15 | 11 | 220μF |
| MONSUM | SPK/ | TO |  |
| and FILTR, connections to analog ground.  The 1.0μF capaci- | 29 | 1 | AD1843 MIN TIP TRAN1 2 30 |
| tors required by the AD1843 can be of any type. | RING | TRAN2 |  |
| TO AD1843 | 8 OH/ RI/ | 6 |  |
| FILTL | FILTR RLY2/ REOUT1 9 +5VA | XCTL0 | 7 19 |
14
| LO | AG0 | REOUT2 |
| --- | --- | --- |
| 1.0μF | 1.0μF | 17 13 V |
| HI | AG1 | VCCA CC 18 16 |
| LO | POWER/ | VCCD VCC |
10
AGND 56k
Figure 28.  AD1843 External Filter Capacitor Connections 20
RING DGND
INDICATION
Figure 29 illustrates the antialias filtering capacitors, AAFILTL
TO HOST
INTERRUPT
and AAFILTR, connections to analog ground.  The 1000 pF
capacitors must be NPO types. J1
1
AAFILTL AAFILTR
2 COIL
3
1000pF	1000pF	PHONE
4
NPO	NPO	COIL
5
6
Figure 29.  AD1843 Antialias Filter Capacitor Connections RJ11
SINGLE
The 24.576 MHz crystal shown in the crystal connection cir-
cuitry of Figure 30 should be fundamental-mode and parallel- Figure 31.  Silicon Systems DAA Example Circuit
tuned.  Note that using the exact data sheet frequency is not
DVDD
J1
C4 100nF RJ-11 JACK
F1 6
CURRENT L1
| C3 100nF | V | LIMIT | INDUCTOR | TIP (GREEN) | 5 |
| --- | --- | --- | --- | --- | --- |
| 8 | CC | 1 |  |  |  |
| LOUT2LP | XMIT+ L2 RING (RED) | TIP | 4 |  |  |
| C2 100nF | INDUCTOR 6 2 | 3 |  |  |  |
| LOUT2LN | XMIT– | RING | 2 |  |  |
| VR1 | VR3 | F2 | C6, 1nF C1 100nF | C5, 1nF | 1 |
| 5 | CERMETEK | MOV | MOVCURRENT | 1.5kV | 1.5kV |
| LINLP | RCV CH1837 | LIMIT |  |  |  |
| U3B | U3A | DAA | VR2 |  |  |
| 3 | 4 | 1 | 2 | 3 | MOV XCTL0 OFFHK SN74HC14SN74HC14 EARTH |
| 4 | GROUND RT 1. VR1,2,3. OVER VOLTAGE AND LIGHTNING PROTECTION | NOTES: |  |  |  |
GND
AD1843 MOV SIDACTORS. AVAILABLE AS A SINGLE UNIT FROM
| DVDD | 9 | TECCOR, PART NUMBER P3203AB OR P3203AA. 2. C5,6. EMI/RFI SUPPRESSION. HIGH VOLTAGE DISK R1 CERAMIC CAPACITORS, 1nF 1.5kV. 100k TO DSP OR HOST 3. F1,2. CURRENT LIMITING DEVICES. RAYCHEM POLY FUSE, |
| --- | --- | --- |
| U3D | U3C | CPU INTERRUPT |
| 9 | 8 | 5 6 PART NUMBER TB600-45; LITTLE FUSE TYPE 251.250 OR IRQ BUSSMAN TYPE MCR1/4. FOR NON UL APPLICATIONS, +C7 A 10 OHM, 1/8W CARBON FILM RESISTOR MAY BE USED. 4.7μF TANTSN74HC14SN74HC14 (OPTIONAL) 4. L1,2. EMI/RFI SUPPRESSION. FERRITE BEADS, FAIR-RITE PART NUMBER 2643666611 OR 2943666661. 5. J1. FCC APPROVED RJ-11 JACK. REF FCC PUBLIC NOTICE #42269 Figure 32.  Cermetek DAA Example Circuit REV. 0 –57– |

AD1843
Good, standard engineering practices should be applied for Analog Devices recommends a split ground plane as shown in
power-supply decoupling.  Decoupling capacitors should be Figure 35a (PQFP) and Figure 35b (TQFP).  The analog plane
placed as close as possible to package pins.  If a separate analog and the digital plane are connected directly under the AD1843.
power supply is not available, we recommend the circuit shown Splitting the ground plane directly under the SoundComm
in Figure 33 for using a single +5 V supply.  This circuitry Codec is optimal because analog pins will be located above the
should be as close to the supply pins as is practical. analog ground plane and digital pins will be located directly
above the digital ground plane for the best isolation.  The digital
FERRITE OR
ground and analog ground should be tied together in the vicin-
SUITABLE
INDUCTOR ity of the AD1843.  Other schemes may also yield satisfactory
+5V
SUPPLY results.  If the split ground plane recommended here is not pos-
| 0.1μF | 0.1μF 0.1μF 1μF sible, the AD1843 should be entirely over the analog ground plane. | 0.1μF | 0.1μF |
| --- | --- | --- | --- |
| VDD | VDD | VDD | VDD |
| 0.1μF | 0.1μF | 0.1μF | 0.1μF 0.1μF 50 49 |
| VDD | VDD | VDD | VDD VDD FERRITE OR DIGITAL ANALOG |
SUITABLE
GROUND	AD1843	PQFP	GROUND
INDUCTOR
1.6Ω
PLANE PLANE
0.1μF	1μF	1μF	0.1μF
VCC	VCC VCC	10	11
Figure 33.  AD1843 Recommended Power Supply
Bypassing
Figure 35a.  AD1843 Recommended PQFP Ground Plane
Figure 34 illustrates an optional circuit for generating the +5 V
analog supply for the AD1843.  The +12 V rail available on the
ISA bus can be regulated down to +5 V using an ordinary 7805
three-terminal regulator and the bypassing and decoupling
62 61
capacitors shown. The digital supply should still be decoupled
and bypassed as shown above in Figure 33.
FERRITE 7805
DIGITAL ANALOG
BEAD
| GROUND | AD1843 | TQFP | GROUND |
| --- | --- | --- | --- |
| +12V | IN | OUT | +5VA PLANE PLANE |
| 10μF | 220nF | 10μF | 220nF 1μF 220nF |
GND
PC 0 V
13 14
DGND AGND
Figure 34.  AD1843 Optional Analog Supply Regulation
Figure 35b.  AD1843 Recommended TQFP Ground Plane
–58– REV. 0

Frequency Response Plots–AD1843
0
–10
–20 0.0000
–30
–40
–50
–0.0050
dB
dB
–60
–70
–80
–0.0100
–90
–100
–110
04 8121620	24 28	32 36	40 44	024  6  8 1012141618
kHz kHz
a.  ADC Audio Full c.  ADC Audio Passband
0 0.008
0.006
–20
0.004
–40 0.002
0.000
–60
–0.002
–80
–0.004
–100 dB–0.006
dB
–0.008
–120
–0.010
–140 –0.012
–0.014
–160
–0.016
–180
–0.018
06432	96  128 160192 224 256 288 320   352	0182	4	6	8	10	12	14	16
kHz kHz
b.  DAC Audio 0–384 kHz d.  DAC Audio Passband
Figure 36.  AD1843Audio Mode Frequency ResponsePlots (Full-Scale Line-Level Inputs, 0 dB  Gain)
Based on 48 kHz sample rate; other sample rates will show slightly different behavior.  See the
”Specifications” section for performance limits.  The DAC plots do not reflect the additional benefi-
cial roll-off of the AD1843 analog filters.  Out-of-band images will be attenuated by an additional
31.4 dB at 100 kHz.
REV. 0 –59–

AD1843
0.040
0.020
0
0.000
–10
–0.020
–20
–0.040
–30
–0.060
–40
–0.080
–50
–0.100
dB
dB
–60 –0.120
–70 –0.140
–80 –0.160
–90 –0.180
–100 –0.200
–0.220
–110
–0.240
0.2	0.6	1.0	1.4	1.8	2.2	2.6	3.0
0	0.8	1.6	2.4	3.2	4.0	4.8	5.6	6.4	7.2
kHz
kHz
c.  ADC Modem Passband
a.  ADC Modem Full
0.080
0
0.040
–20
0.000
–40
–60 –0.040
–80
–0.080
dB
dB–100
–0.120
–120
–0.160
–140
–160 –0.200
–180
–0.240
0	4.8 9.614.419.224.028.833.638.443.248.0  52.8	0.2	0.6	1.0	1.4	1.8	2.2	2.6	3.0
kHz kHz
b.  DAC Modem 0–57.6 kHz d.  DAC Modem Passband
Figure 37.  AD1843 Modem Mode Frequency Response Plots (Full-Scale Line-Level Inputs, 0 dB Gain)
Based on 7.2 kHz sample rate; other sample rates will show slightly different behavior. See the
”Specifications” section for performance limits. The DAC plots do not reflect the additional beneficial
roll-off of the AD1843 analog filters.  Out-of-band images will be attenuated by an additional 31.4 dB
at 100 kHz.
–60– REV. 0

AD1843
0
0.005
–10
0.000
–20
–30 –0.005
–40
–0.010
–50
–0.015
dB dB
–60
–0.020
–70
–80 –0.025
–90
–0.030
–100
–0.035
–110
04812	16 20 2428 32 36	40	44 48	0246	81410	12	18
kHz kHz
a.  ADC Resampler Full c.  ADC Resampler Passband
0
0.008
–20
0.004
–40 0.000
–0.004
–60
–0.008
–80
–0.012
–100 dB
dB –0.016
–120 –0.020
–0.024
–140
–0.028
–160
–0.032
–180
–0.036
06432	96    128 160192 224256 288320   352	0	246148	10	12	16	18
kHz kHz
b.  DAC Resampler 0–384 kHz d.  DAC Resampler Passband
Figure 38.  AD1843 Resampler Mode Frequency Response Plots (Full-Scale Line-Level Inputs, 0 dB Gain)
Based on 48 kHz sample rate; other sample rates will show slightly different behavior.  See the ”Specifica-
tions”section for performance limits. The DAC plots do not reflect the additional beneficial roll-off of
the AD1843 analog filters.  Out-of-band images will be attenuated by an additional 31.4 dB at 100 kHz.
REV. 0 –61–

AD1843
TABLE OF CONTENTS
FEATURES   . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .  1
GENERAL PRODUCT DESCRIPTION   . . . . . . . . . . . . .    1
SPECIFICATIONS   . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .    2
ORDERING INFORMATION   . . . . . . . . . . . . . . . . . . . . 6
PIN DESCRIPTION   . . . . . . . . . . . . . . . . . . . . . . . . . . . . .    7
PIN CONFIGURATIONS  . . . . . . . . . . . . . . . . . . . . . . .    7, 8
DETAILED FUNCTIONAL BLOCK DIAGRAM  . . . . .    12
DETAILED PRODUCT DESCRIPTION  . . . . . . . . . . . .    13
FUNCTIONAL DESCRIPTION  . . . . . . . . . . . . . . . . . . .    13
SERIAL INTERFACE  . . . . . . . . . . . . . . . . . . . . . . . . . . .    19
SERIAL INTERFACE INPUT   . . . . . . . . . . . . . . . . . . . .    22
SERIAL INTERFACE OUTPUT   . . . . . . . . . . . . . . . . . .    24
CONTROL REGISTERS  . . . . . . . . . . . . . . . . . . . . . . . . .    25
BIT AND REGISTER MAPS  . . . . . . . . . . . . . . . . . . . . . .    51
START-UP SEQUENCE  . . . . . . . . . . . . . . . . . . . . . . . . .    52
APPLICATIONS CIRCUITS   . . . . . . . . . . . . . . . . . . . . .    55
FREQUENCY RESPONSE PLOTS   . . . . . . . . . . . . . . . .    59
OUTLINE DIMENSIONS
Dimensions shown in inches and (mm)
S-80 ST-100
80-Terminal Plastic Quad Flatpack (PQFP) 100-Terminal Plastic Thin Quad Flatpack (TQFP)
0.690 (17.45) 0.640 (16.25)
0.667 (16.95) SQ
0.061 (1.55) 0.620 (15.75)
| 0.096 (2.45) | 0.555 (14.10) | 0.049 (1.25) | 0.555 (14.10) |
| --- | --- | --- | --- |
| MAX | 0.547 (13.90) 0.547 (13.90) 0.486 (12.35) TYP 0.026 (0.65) 0.037 (0.95) | SQ |  |
| 0.014 (0.35) | 100 80 12° | 76 |  |
| 0.026 (0.65) | 61 | 1 | 75 |
TYP
1 60
SEATING
SEATING
PLANE
PLANE
TOP VIEW
TOP VIEW
(PINS DOWN)
(PINS DOWN)
0.555 (14.10)0.547 (13.90)0.690 (17.450.667 (16.95)
0.486 (12.35) TYP
0.004 (0.10) 0.004
20 41
| MAX | 21 | 40 | (0.102) |
| --- | --- | --- | --- |
| MAX LEAD | 25 | 51 |  |
| 0.010 (0.25) | COPLANARITY 6°±4° | 26 | 50 |
| MIN | 0.029 (0.73) 0°– 10° | 0.015 (0.38) |  |
| 0.083 (2.10) | 0.023 (0.57) | 0.009 (0.22) | 0.007 (0.177) 0.020 (0.50) 0.012 (0.20) |
| 0.075 (1.90) | 0.003 (0.077) 0.004 (0.10) –62– REV. 0 | BSC |  |

–63–

C2097–6–1/96
PRINTED IN U.S.A.
–64–

Find price and stock options from leading distributors for
AD1843JST on Findchips.com:
https://findchips.com/search/AD1843JST
Find CAD models and details for this part:
https://findchips.com/detail/ad1843jst/Analog-Devices-Inc
