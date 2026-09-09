# mips-iv-isa



After executingLWL $24,2($0)”

Ι→	→	→
Ι→	→	→

A

CPU coprocessor (except 0)
Data Size	Load	Load	Store	Load	Store
Signed Unsigned
floating-point coprocessor only
Data Size	Load	Store

Mnemonic	Description	Defined in
Mnemonic	Description	Defined in

| Mnemonic | Description | Defined in |
| --- | --- | --- |
| Mnemonic | Description | Defined in |
| Mnemonic | Description | Defined in |

Mnemonic	Description	Defined in

Mnemonic	Description	Defined in
XOR Exclusive Or
Mnemonic	Description	Defined in

Mnemonic	Description	Defined in

| Mnemonic | Description | Defined in |
| --- | --- | --- |
| Mnemonic | Description | Defined in |
| Mnemonic | Description | Defined in |
| Mnemonic | Description | Defined in |
| Mnemonic | Description | Defined in |

| Mnemonic | Description | Defined in |
| --- | --- | --- |
| Mnemonic | Description | Defined in |
| Mnemonic | Description | Defined in |
| Mnemonic | Description | Defined in |

Mnemonic	Description	Defined in
Mnemonic	Description	Defined in

Mnemonic	Description	Defined in

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

←
≠
y
x
×
→ →
→ →

+1
+1

dataword←COP_SW (z, rt)
datadouble←COP_SD (z, rt)

←
← (CCA, AccessLength, pAddr, vAddr, IorD)

(CCA, AccessLength, MemElem, pAddr, vAddr)

The following pseudocode referring to the Status bit is valid for all existing
FR
MIPS 64-bit processors at the time of this writing, however this is a privileged
processor-specific mechanism and it may be different in some future
processor.
SizeFGR()   --  current size, in bits, of the CP1 general registers
size←SizeFGR()
if 32_bit_processor then
size←32
else
/* 64-bit processor */
if Status = 1 then
FR
size←64
else
size←32
endif
endif

ValueFPR()  --  Get a formatted value from an FPR.
value←ValueFPR (fpr, fmt) /* get a formatted value from an FPR */
if SizeFGR() = 64 then
case fmt of
S, W:
value←FGR[fpr]
31..0
D, L:
value←FGR[fpr]
endcase
elseif fpr= 0 then /* fpr is valid (even), 32-bit wide FGRs */
0
case fmt of
S, W:
value←FGR[fpr]
D, L:
value←FGR[fpr+1] || FGR[fpr]
endcase
else /* undefined for odd 32-bit FGRs */
UndefinedResult
endif

StoreFPR()  --  store a formatted value into an FPR.
StoreFPR(fpr, fmt, value): /* place a formatted value into an FPR */
if SizeFGR() = 64 then   /* 64-bit wide FGRs */
case fmt of
S, W:
32
FGR[fpr]←undefined || value
D, L:
FGR[fpr]←value
endcase
elseif fpr= 0 then /* fpr is valid (even), 32-bit wide FGRs */
0
case fmt of
S, W:
32
FGR[fpr+1]←undefined
FGR[fpr]←value
D, L:
FGR[fpr+1]←value
63..32
FGR[fpr]←value
31..0
endcase
else /* undefined for odd 32-bit FGRs */
UndefinedResult
endif

ADD Add Word
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 6    5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| SPECIAL | 0 | ADD |  |  |  |  |
| rs | rt | rd |  |  |  |  |
| 0 0 0 0 0 0 | 0 0 0 0 0 | 1 0 0 0 0 0 65555  6 ADD   rd, rs, rt rd←rs + rt if (NotWordValue(GPR[rs]) or NotWordValue(GPR[rt])) then UndefinedResult() endif temp←GPR[rs] + GPR[rt] if (32_bit_arithmetic_overflow) then SignalException(IntegerOverflow) else GPR[rd]←sign_extend(temp ) 31..0 endif |  |  |  |  |

Add Immediate WordADDI
31	26 25	21 20	16 15	0
ADDI
rs	rt	immediate
0 0 1 0 0 0
655 16
ADDI   rt, rs, immediate
rt←rs + immediate
if (NotWordValue(GPR[rs])) then UndefinedResult() endif
temp←GPR[rs] + sign_extend(immediate)
if (32_bit_arithmetic_overflow) then
SignalException(IntegerOverflow)
else
GPR[rt]←sign_extend(temp )
31..0
endif

ADDIU Add Immediate Unsigned Word
31	26 25	21 20	16 15	0
ADDIU
rs	rt	immediate
0 0 1 0 0 1
655 16
ADDIU   rt, rs, immediate
rt←rs + immediate
if (NotWordValue(GPR[rs])) then UndefinedResult() endif
temp ←GPR[rs] + sign_extend(immediate)
GPR[rt]←sign_extend(temp )
31..0

Add Unsigned WordADDU
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 6    5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| SPECIAL | rs | rt | rd | 0 | ADDU |  |
| 0 0 0 0 0 0 | 0 0 0 0 0 | 1 0 0 0 0 1 65555  6 ADDU   rd, rs, rt rd←rs + rt if (NotWordValue(GPR[rs]) or NotWordValue(GPR[rt])) then UndefinedResult() endif temp ←GPR[rs] + GPR[rt] GPR[rd]←sign_extend(temp ) 31..0 |  |  |  |  |

AND And
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 6    5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| SPECIAL | rs | rt | rd | 0 | AND |  |
| 0 0 0 0 0 0 | 0 0 0 0 0 | 1 0 0 1 0 0 65555  6 AND   rd, rs, rt rd←rs AND rt GPR[rd]←GPR[rs] and GPR[rt] |  |  |  |  |

And ImmediateANDI
31	26 25	21 20	16 15	0
ANDI	rs	rt	immediate
0 0 1 1 0 0
655 16
ANDI   rt, rs, immediate
rt←rs AND immediate
GPR[rt]←zero_extend(immediate) and GPR[rs]

BEQ Branch on Equal
31	26 25	21 20	16 15	0
BEQ	rs	rt	offset
0 0 0 1 0 0
655 16
BEQ   rs, rt, offset
if (rs = rt) then branch
2
: tgt_offset←sign_extend(offset || 0)
condition←(GPR[rs] = GPR[rt])
if condition then
PC←PC + tgt_offset
endif
±

Branch on Equal LikelyBEQL
31	26 25	21 20	16 15	0
BEQL	rs	rt	offset
0 1 0 1 0 0
655 16
BEQL   rs, rt, offset
if (rs = rt) then branch_likely
2
tgt_offset←sign_extend(offset || 0)
condition←(GPR[rs] = GPR[rt])
if condition then
PC←PC + tgt_offset
else
NullifyCurrentInstruction()
endif
±

BGEZ Branch on Greater Than or Equal to Zero
31	26 25	21 20	16 15	0
REGIMM	rs	BGEZ	offset
0 0 0 0 0 1 0 0 0 0 1
655 16
BGEZ   rs, offset
if (rs≥0) then branch
2
tgt_offset←sign_extend(offset || 0)
GPRLEN
condition←GPR[rs]≥0
if condition then
PC←PC + tgt_offset
endif
±

Branch on Greater Than or Equal to Zero and LinkBGEZAL
31	26 25	21 20	16 15	0
REGIMM	rs	BGEZAL	offset
0 0 0 0 0 1 1 0 0 0 1
655 16
BGEZAL   rs, offset
if (rs≥0) then procedure_call
2
tgt_offset←sign_extend(offset || 0)
GPRLEN
condition←GPR[rs]≥0
GPR[31]←PC + 8
if condition then
PC←PC + tgt_offset
endif
±

BGEZALL Branch on Greater Than or Equal to Zero and Link Likely
31	26 25	21 20	16 15	0
REGIMM	rs	BGEZALL	offset
0 0 0 0 0 1 1 0 0 1 1
655 16
BGEZALL   rs, offset
if (rs≥0) then procedure_call_likely
2
tgt_offset←sign_extend(offset || 0)
GPRLEN
condition←GPR[rs]≥0
GPR[31]←PC + 8
if condition then
PC←PC + tgt_offset
else
NullifyCurrentInstruction()
endif
±

Branch on Greater Than or Equal to Zero LikelyBGEZL
31	26 25	21 20	16 15	0
REGIMM	rs	BGEZL	offset
0 0 0 0 0 1 0 0 0 1 1
655 16
BGEZL   rs, offset
if (rs≥0) then branch_likely
2
tgt_offset←sign_extend(offset || 0)
GPRLEN
condition←GPR[rs]≥0
if condition then
PC←PC + tgt_offset
else
NullifyCurrentInstruction()
endif
±

BGTZ Branch on Greater Than Zero
31	26 25	21 20	16 15	0
BGTZ	rs	0	offset
0 0 0 1 1 1 0 0 0 0 0
655 16
BGTZ   rs, offset
if (rs > 0) then branch
2
tgt_offset←sign_extend(offset || 0)
GPRLEN
condition←GPR[rs] > 0
if condition then
PC←PC + tgt_offset
endif
±

Branch on Greater Than Zero LikelyBGTZL
31	26 25	21 20	16 15	0
BGTZL 0
rs offset
0 1 0 1 1 1 0 0 0 0 0
655 16
BGTZL   rs, offset
if (rs > 0) then branch_likely
2
tgt_offset←sign_extend(offset || 0)
GPRLEN
condition←GPR[rs] > 0
if condition then
PC←PC + tgt_offset
else
NullifyCurrentInstruction()
endif
±

BLEZ Branch on Less Than or Equal to Zero
31	26 25	21 20	16 15	0
BLEZ	rs	0	offset
0 0 0 1 1 0 0 0 0 0 0
655 16
BLEZ   rs, offset
if (rs≤0) then branch
2
tgt_offset←sign_extend(offset || 0)
GPRLEN
condition←GPR[rs]≤0
if condition then
PC←PC + tgt_offset
endif
±

Branch on Less Than or Equal to Zero LikelyBLEZL
31	26 25	21 20	16 15	0
BLEZL	rs	0	offset
0 1 0 1 1 0 0 0 0 0 0
655 16
BLEZL   rs, offset
if (rs≤0) then branch_likely
2
tgt_offset←sign_extend(offset || 0)
GPRLEN
condition←GPR[rs]≤0
if condition then
PC←PC + tgt_offset
else
NullifyCurrentInstruction()
endif
±

BLTZ Branch on Less Than Zero
31	26 25	21 20	16 15	0
REGIMM	rs	BLTZ	offset
0 0 0 0 0 1 0 0 0 0 0
655 16
BLTZ   rs, offset
if (rs < 0) then branch
2
tgt_offset←sign_extend(offset || 0)
GPRLEN
condition←GPR[rs] < 0
if condition then
PC←PC + tgt_offset
endif
±

Branch on Less Than Zero And LinkBLTZAL
31	26 25	21 20	16 15	0
REGIMM	rs	BLTZAL	offset
0 0 0 0 0 1 1 0 0 0 0
655 16
BLTZAL   rs, offset
if (rs < 0) then procedure_call
2
tgt_offset←sign_extend(offset || 0)
GPRLEN
condition←GPR[rs] < 0
GPR[31]←PC + 8
if condition then
PC←PC + tgt_offset
endif
±

BLTZALL Branch on Less Than Zero And Link Likely
31	26 25	21 20	16 15	0
REGIMM	rs	BLTZALL	offset
0 0 0 0 0 1 1 0 0 1 0
655 16
BLTZALL   rs, offset
if (rs < 0) then procedure_call_likely
2
tgt_offset←sign_extend(offset || 0)
GPRLEN
condition←GPR[rs] < 0
GPR[31]←PC + 8
if condition then
PC←PC + tgt_offset
else
NullifyCurrentInstruction()
endif
±

Branch on Less Than Zero LikelyBLTZL
31	26 25	21 20	16 15	0
REGIMM	rs	BLTZL	offset
0 0 0 0 0 1 0 0 0 1 0
655 16
BLTZ   rs, offset
if (rs < 0) then branch_likely
2
tgt_offset←sign_extend(offset || 0)
GPRLEN
condition←GPR[rs] < 0
if condition then
PC←PC + tgt_offset
else
NullifyCurrentInstruction()
endif
±

BNE Branch on Not Equal
31	26 25	21 20	16 15	0
BNE	rs	rt	offset
0 0 0 1 0 1
655 16
BNE   rs, rt, offset
if (rs≠rt) then branch
2
tgt_offset←sign_extend(offset || 0)
condition←(GPR[rs]≠GPR[rt])
if condition then
PC←PC + tgt_offset
endif
±

Branch on Not Equal LikelyBNEL
31	26 25	21 20	16 15	0
BNEL	rs	rt	offset
0 1 0 1 0 1
655 16
BNEL   rs, rt, offset
if (rs≠rt) then branch_likely
2
tgt_offset←sign_extend(offset || 0)
condition←(GPR[rs]≠GPR[rt])
if condition then
PC←PC + tgt_offset
else
NullifyCurrentInstruction()
endif
±

BREAK Breakpoint
| 31 | 26 | 25 | 6 5 | 0 |
| --- | --- | --- | --- | --- |
| SPECIAL | code | BREAK 0 0 0 0 0 0 0 0 1 1 0 1 |  |  |
| 6 | 20 | 6 |  |  |
BREAK
SignalException(Breakpoint)

Coprocessor OperationCOPz
31	2625	0
COPz cop_fun
0 1 0 0 z z
6 26
COP0   cop_fun
COP1   cop_fun
COP2   cop_fun
COP3   cop_fun
CoprocessorOperation (z, cop_fun)

DADD Doubleword Add
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 6    5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| SPECIAL | 0 | DADD |  |  |  |  |
| rs | rt | rd |  |  |  |  |
| 0 0 0 0 0 0 | 0 0 0 0 0 | 1 0 1 1 0 0 65555  6 DADD   rd, rs, rt rd←rs + rt temp←GPR[rs] + GPR[rt] if (64_bit_arithmetic_overflow) then SignalException(IntegerOverflow) else GPR[rd]←temp endif |  |  |  |  |

Doubleword Add ImmediateDADDI
31	26 25	21 20	16 15	0
DADDI
rs	rt	immediate
0 1 1 0 0 0
655 16
DADDI   rt, rs, immediate
rt←rs + immediate
temp←GPR[rs] + sign_extend(immediate)
if (64_bit_arithmetic_overflow) then
SignalException(IntegerOverflow)
else
GPR[rt]←temp
endif

DADDIU Doubleword Add Immediate Unsigned
31	26 25	21 20	16 15	0
DADDIU
rs	rt	immediate
0 1 1 0 0 1
655 16
DADDIU   rt, rs, immediate
rt←rs + immediate
GPR[rt]←GPR[rs] + sign_extend(immediate)

Doubleword Add UnsignedDADDU
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 6    5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| SPECIAL | rs | rt | rd | 0 | DADDU |  |
| 0 0 0 0 0 0 | 0 0 0 0 0 | 1 0 1 1 0 1 65555  6 DADDU   rd, rs, rt rd←rs + rt GPR[rd]←GPR[rs] + GPR[rt] |  |  |  |  |

DDIV Doubleword Divide
| 31 | 26 25 | 21 20 | 16 15 | 65 | 0 |
| --- | --- | --- | --- | --- | --- |
| SPECIAL | rs | rt | 0 | DDIV |  |
| 0 0 0 0 0 0 | 0 0   0 0 0 0  0 0 0 0 | 0 1 1 1 1 0 |  |  |  |
| 655 | 10 DDIV   rs, rt (LO, HI)←rs / rt , LO, HI ←undefined LO ←GPR[rs] div GPR[rt] HI ←GPR[rs] mod GPR[rt] | 6 |  |  |  |

Doubleword Divide UnsignedDDIVU
| 31 | 26 25 | 21 20 | 16 15 | 65 | 0 |
| --- | --- | --- | --- | --- | --- |
| SPECIAL | rs | rt | 0 | DDIVU |  |
| 0 0 0 0 0 0 | 0 0 0 0 0 0  0 0 0 0 | 0 1 1 1 1 1 |  |  |  |
| 655 | 10 DDIVU   rs, rt (LO, HI)←rs / rt , LO, HI ←undefined LO ←(0 \|\| GPR[rs]) div (0 \|\| GPR[rt]) HI ←(0 \|\| GPR[rs]) mod (0 \|\| GPR[rt]) | 6 |  |  |  |

DIV Divide Word
| 31 | 26 25 | 21 20 | 16 15 | 65 | 0 |
| --- | --- | --- | --- | --- | --- |
| SPECIAL | 0 | DIV rs rt |  |  |  |
| 0 0 0 0 0 0 | 0 0   0 0 0 0  0 0 0 0 | 0 1 1 0 1 0 |  |  |  |
| 655 | 10 DIV   rs, rt (LO, HI)←rs / rt if (NotWordValue(GPR[rs]) or NotWordValue(GPR[rt])) then UndefinedResult() endif , LO, HI ←undefined | 6 |  |  |  |
| q | ←GPR[rs] | div GPR[rt] 31..0 31..0 |  |  |  |
| LO | ←sign_extend(q 31..0 | ) |  |  |  |
| r | ←GPR[rs] | mod GPR[rt] 31..0 31..0 |  |  |  |
| HI | ←sign_extend(r 31..0 | ) |  |  |  |

Divide WordDIV

DIVU Divide Unsigned Word
| 31 | 26 25 | 21 20 | 16 15 | 65 | 0 |
| --- | --- | --- | --- | --- | --- |
| SPECIAL | 0 | DIVU rs rt |  |  |  |
| 0 0 0 0 0 0 | 0 0 0 0 0 0  0 0 0 0 | 0 1 1 0 1 1 |  |  |  |
| 655 | 10 DIVU   rs, rt (LO, HI)←rs / rt if (NotWordValue(GPR[rs]) or NotWordValue(GPR[rt])) then UndefinedResult() endif , LO, HI ←undefined | 6 |  |  |  |
| q | ←(0 \|\| GPR[rs] | ) div (0 \|\| GPR[rt] 31..0 31..0 | ) |  |  |
| LO | ←sign_extend(q 31..0 | ) |  |  |  |
| r | ←(0 \|\| GPR[rs] | ) mod (0 \|\| GPR[rt] 31..0 31..0 | ) |  |  |
| HI | ←sign_extend(r 31..0 | ) |  |  |  |

Doubleword MultiplyDMULT
| 31 | 26 25 | 21 20 | 16 15 | 65 | 0 |
| --- | --- | --- | --- | --- | --- |
| SPECIAL | rs | rt | 0 | DMULT |  |
| 0 0 0 0 0 0 | 0 0  0 0 0 0  0 0 0 0 | 0 1 1 1 0 0 |  |  |  |
| 655 | 10 DMULT   rs, rt (LO, HI)←rs×rt , LO, HI ←undefined prod ←GPR[rs] * GPR[rt] LO ←prod 63..0 H I ←prod 127..64 | 6 |  |  |  |

DMULTU Doubleword Multiply Unsigned
| 31 | 26 25 | 21 20 | 16 15 | 65 | 0 |
| --- | --- | --- | --- | --- | --- |
| SPECIAL | rs | rt | 0 | DMULTU |  |
| 0 0 0 0 0 0 | 0 0  0 0 0 0  0 0 0 0 | 0 1 1 1 0 1 |  |  |  |
| 655 | 10 DMULTU   rs, rt (LO, HI)←rs×rt , LO,HI ←undefined prod ←(0 \|\| GPR[rs]) * (0 \|\| GPR[rt]) LO ←prod 63..0 HI ←prod 127..64 | 6 |  |  |  |

Doubleword Shift Left LogicalDSLL
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 6    5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| SPECIAL | 0 | rt | rd | sa | DSLL |  |
| 0 0 0 0 0 0 | 0 0 0 0 0 | 1 1 1 0 0 0 65555  6 DSLL   rd, rt, sa  rd←rt << sa s ←0 \|\| sa s GPR[rd]←GPR[rt] \|\| 0 (63–s)..0 |  |  |  |  |

DSLL32 Doubleword Shift Left Logical Plus 32
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 6    5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| SPECIAL | 0 | rt | rd | sa | DSLL32 |  |
| 0 0 0 0 0 0 | 0 0 0 0 0 | 1 1 1 1 0 0 |  |  |  |  |
| 6555 | 5 DSLL32   rd, rt, sa  rd←rt << (sa+32) | 6 |  |  |  |  |
| s | ←1 \|\| sa | /* 32+sa */ s GPR[rd]←GPR[rt] \|\| 0 (63–s)..0 |  |  |  |  |

Doubleword Shift Left Logical VariableDSLLV
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 6    5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| SPECIAL | rs | rt | rd | 0 | DSLLV |  |
| 0 0 0 0 0 0 | 0 0 0 0 0 | 0 1 0 1 0 0 65555  6 DSLLV   rd, rt, rs rd←rt << rs s ←0 \|\| GPR[rs] 5..0 s GPR[rd]←GPR[rt] \|\| 0 (63–s)..0 |  |  |  |  |

DSRA Doubleword Shift Right Arithmetic
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 6    5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| SPECIAL | 0 | rt | rd | sa | DSRA |  |
| 0 0 0 0 0 0 | 0 0 0 0 0 | 1 1 1 0 1 1 65555  6 DSRA   rd, rt, sa  rd←rt >> sa      (arithmetic) s ←0 \|\| sa s GPR[rd]←(GPR[rt] )\|\| GPR[rt] 63 63..s |  |  |  |  |

Doubleword Shift Right Arithmetic Plus 32DSRA32
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 6    5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| SPECIAL | 0 | rt | rd | sa | DSRA32 |  |
| 0 0 0 0 0 0 | 0 0 0 0 0 | 1 1 1 1 1 1 65555  6 DSRA32   rd, rt, sa  rd←rt >> (sa+32)      (arithmetic) |  |  |  |  |
| s | ←1 \|\| sa | /* 32+sa */ s GPR[rd]←(GPR[rt] )\|\| GPR[rt] 63 63..s |  |  |  |  |

DSRAV Doubleword Shift Right Arithmetic Variable
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 6    5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| SPECIAL | rs | rt | rd | 0 | DSRAV |  |
| 0 0 0 0 0 0 | 0 0 0 0 0 | 0 1 0 1 1 1 65555  6 DSRAV   rd, rt, rs rd←rt >> rs      (arithmetic) s ←GPR[rs] 5..0 s GPR[rd]←(GPR[rt] )\|\| GPR[rt] 63 63..s |  |  |  |  |

Doubleword Shift Right LogicalDSRL
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 6    5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| SPECIAL | 0 | rt | rd | sa | DSRL |  |
| 0 0 0 0 0 0 | 0 0 0 0 0 | 1 1 1 0 1 0 65555  6 DSRL   rd, rt, sa  rd←rt >> sa      (logical) s ←0 \|\| sa s GPR[rd]←0\|\| GPR[rt] 63..s |  |  |  |  |

DSRL32 Doubleword Shift Right Logical Plus 32
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 6    5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| SPECIAL | 0 | rt | rd | sa | DSRL32 |  |
| 0 0 0 0 0 0 | 0 0 0 0 0 | 1 1 1 1 1 0 65555  6 DSRL32   rd, rt, sa  rd←rt >> (sa+32)      (logical) |  |  |  |  |
| s | ←1 \|\| sa | /* 32+sa */ s GPR[rd]←0\|\| GPR[rt] 63..s |  |  |  |  |

Doubleword Shift Right Logical VariableDSRLV
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 6    5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| SPECIAL | rs | rt | rd | 0 | DSRLV |  |
| 0 0 0 0 0 0 | 0 0 0 0 0 | 0 1 0 1 1 0 65555  6 DSRLV   rd, rt, rs rd←rt >> rs      (logical) s ←GPR[rs] 5..0 s GPR[rd]←0 \|\| GPR[rt] 63..s |  |  |  |  |

DSUB Doubleword Subtract
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 6    5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| SPECIAL | rs | rt | rd | 0 | DSUB |  |
| 0 0 0 0 0 0 | 0 0 0 0 0 | 1 0 1 1 1 0 65555  6 DSUB   rd, rs, rt rd←rs - rt temp←GPR[rs] – GPR[rt] if (64_bit_arithmetic_overflow) then SignalException(IntegerOverflow) else GPR[rd]←temp endif |  |  |  |  |

Doubleword Subtract UnsignedDSUBU
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 6    5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| SPECIAL | rs | rt | rd | 0 | DSUBU |  |
| 0 0 0 0 0 0 | 0 0 0 0 0 | 1 0 1 1 1 1 65555  6 DSUBU   rd, rs, rt rd←rs - rt GPR[rd]←GPR[rs] – GPR[rt] |  |  |  |  |

J Jump
31	26 25	0
J instr_index
0 0 0 0 1 0
6 26
J   target
2
PC←PC || instr_index || 0
GPRLEN..28

Jump And LinkJAL
31	26 25	0
JAL instr_index
0 0 0 0 1 1
6 26
JAL    target
GPR[31]←PC + 8
2
PC←PC || instr_index || 0
GPRLEN..28

JALR Jump And Link Register
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 6    5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| SPECIAL | rs | rd | JALR |  |  |  |
| 0 0 0 0 0 0 | 0 0 0 0 0 | 0 0 0 0 0 | 0 0 1 0 0 1 65555  6 JALR   rs                    (rd = 31 implied) JALR   rd, rs rd←return_addr, PC←rs temp←GPR[rs] GPR[rd]←PC + 8 PC←temp |  |  |  |

Jump RegisterJR
| 31 | 26 | 25 | 21 20 | 6 5 | 0 |
| --- | --- | --- | --- | --- | --- |
| SPECIAL | rs | JR |  |  |  |
| 0 0 0 0 0 0 | 0 0 0  0 0 0 0  0 0 0 0  0 0 0 0 | 0 0 1 0 0 0 |  |  |  |
| 6 | 515 JR   rs PC←rs temp←GPR[rs] PC←temp | 6 |  |  |  |

LB Load Byte
31	26 25	21 20	16 15	0
LB	base	rt	offset
1 0 0 0 0 0
655 16
LB   rt, offset(base)
rt←memory[base+offset]
vAddr←sign_extend(offset) + GPR[base]
(pAddr, uncached)←AddressTranslation (vAddr, DATA, LOAD)
2
pAddr←pAddr	|| (pAddr	xor ReverseEndian)
(PSIZE-1).. 2 1..0
memword←LoadMemory (uncached, BYTE, pAddr, vAddr, DATA)
2
byte←vAddr xor BigEndianCPU
1..0
GPR[rt]←sign_extend(memword )
7+8*byte..8*byte
vAddr←sign_extend(offset) + GPR[base]
(pAddr, uncached)←AddressTranslation (vAddr, DATA, LOAD)
3
pAddr←pAddr	|| (pAddr	xor ReverseEndian)
PSIZE–1..3 2..0
memdouble←LoadMemory (uncached, BYTE, pAddr, vAddr, DATA)
3
byte←vAddr xor BigEndianCPU
2..0
GPR[rt]←sign_extend(memdouble )
7+8*byte..8*byte

Load Byte UnsignedLBU
31	26 25	21 20	16 15	0
LBU	base	rt	offset
1 0 0 1 0 0
655 16
LBU   rt, offset(base)
rt←memory[base+offset]
vAddr←sign_extend(offset) + GPR[base]
(pAddr, uncached)←AddressTranslation (vAddr, DATA, LOAD)
2
pAddr←pAddr	|| (pAddr	xor ReverseEndian)
PSIZE – 1 .. 2 1..0
memword←LoadMemory (uncached, BYTE, pAddr, vAddr, DATA)
2
byte←vAddr xor BigEndianCPU
1..0
GPR[rt]←zero_extend(memword )
7+8* byte..8* byte
vAddr←sign_extend(offset) + GPR[base]
(pAddr, uncached)←AddressTranslation (vAddr, DATA, LOAD)
3
pAddr←pAddr	|| (pAddr	xor ReverseEndian)
PSIZE–1..3 2..0
memdouble←LoadMemory (uncached, BYTE, pAddr, vAddr, DATA)
3
byte←vAddr xor BigEndianCPU
2..0
GPR[rt]←zero_extend(memdouble )
7+8* byte..8* byte

LD Load Doubleword
31	26 25	21 20	16 15	0
LD	base	rt	offset
1 1 0 1 1 1
655 16
LD   rt, offset(base)
rt←memory[base+offset]
vAddr←sign_extend(offset) + GPR[base]
3
if (vAddr )≠0then SignalException(AddressError) endif
2..0
(pAddr, uncached)←AddressTranslation (vAddr, DATA, LOAD)
memdouble←LoadMemory (uncached, DOUBLEWORD, pAddr, vAddr, DATA)
GPR[rt]←memdouble

Load Doubleword to CoprocessorLDCz
31	26 25	21 20	16 15	0
LDCz	base	rt	offset
1 1 0 1 z z
655 16
LDC1   rt, offset(base)
LDC2   rt, offset(base)
rt←memory[base+offset]
vAddr←sign_extend(offset) + GPR[base]
3
if (vAddr )≠0then SignalException(AddressError) endif
2..0
(pAddr, uncached)←AddressTranslation (vAddr, DATA, LOAD)
memdouble←LoadMemory (uncached, DOUBLEWORD, pAddr, vAddr, DATA)
COP_LD (z, rt, memdouble)

LDCz Load Doubleword to Coprocessor
vAddr←sign_extend(offset) + GPR[base]
3
if (vAddr )≠0then SignalException(AddressError) endif
2..0
(pAddr, uncached)←AddressTranslation (vAddr, DATA, LOAD)
memdouble←LoadMemory (uncached, DOUBLEWORD, pAddr, vAddr, DATA)
COP_LD (z, rt, memdouble)

Load Doubleword LeftLDL
31	26 25	21 20	1615	0
LDL	base	rt	offset
0 1 1 0 1 0
655 16
LDL   rt, offset(base)
rt←rt MERGE memory[base+offset]
Doubleword at byte 2 in memory, big-endian byte order, - each mem byte contains its address
most      — significance — least
0123456789101112131415
a	b	c	d	e	f	g	h	GPR 24:  Initial contents
234567gh After executingLDL $24,2($0)
Then afterLDR $24,9($0)
23456789

LDL Load Doubleword Left
| Memory contents and byte offsets (vAddr | ) | Initial contents of 2..0 |
| --- | --- | --- |
| most | least | Destination Register |
| 01234567←big- | most | least IJKLMNOP abcdefgh 76543210←little-endian offset Destination register contents after instruction (shaded is unchanged) |
| Big-endian byte ordering | vAddr | Little-endian byte ordering 2..0 |
| IJKLMNOP | 0 | Pbcdefgh |
| JKLMNOPh | 1 | OPcdefgh |
| KLMNOPgh | 2 | NOPdefgh |
| LMNOPfgh | 3 | MNOPefgh |
| MNOPefgh | 4 | LMNOPfgh |
| NOPdefgh | 5 | KLMNOPgh |
| OPcdefgh | 6 | JKLMNOPh |
| Pbcdefgh | 7 | IJKLMNOP vAddr←sign_extend(offset) + GPR[base] (pAddr, uncached)←AddressTranslation (vAddr, DATA, LOAD) |
3
pAddr←pAddr	|| (pAddr	xor ReverseEndian)
(PSIZE-1)..3 2..0
if BigEndianMem = 0 then
3
pAddr←pAddr || 0
(PSIZE-1)..3
endif
3
byte←vAddr xor BigEndianCPU
2..0
memdouble←LoadMemory (uncached, byte, pAddr, vAddr, DATA)
GPR[rt]←memdouble || GPR[rt]
7+8*byte..0 55–8*byte..0

Load Doubleword RightLDR
31	26 25	21 20	16 15	0
LDR offset
base rt
0 1 1 0 1 1
655 16
LDR   rt, offset(base)
rt←rt MERGE  memory[base+offset]
Doubleword at byte 2 in memory, big-endian byte order, - each mem byte contains its address
most      — significance — least
0123456789101112131415
a	b	c	d	e	f	g	h	GPR 24:  Initial contents
a	b	c	d	e	f	89	After executingLDR $24,9($0)
Then afterLDL $24,2($0)
23456789

LDR Load Doubleword Right
| Memory contents and byte offsets (vAddr | ) | Initial contents of 2..0 |
| --- | --- | --- |
| most | least | Destination Register |
| 01234567←big- | most | least IJKLMNOP abcdefgh 76543210←little-endian offset Destination register contents after instruction (shaded is unchanged) |
| Big-endian byte ordering | vAddr | Little-endian byte ordering 2..0 |
| a | b | c d e f g I 0 IJKLMNOP |
| abcdefIJ | 1 | aIJKLMNO |
| abcdeIJK | 2 | abIJKLMN |
| abcdIJKL | 3 | abcIJKLM |
| abcIJKLM | 4 | abcdIJKL |
| abIJKLMN | 5 | abcdeIJK |
| aIJKLMNO | 6 | abcdefIJ |
| IJKLMNOP | 7 | abcdefgI vAddr←sign_extend(offset) + GPR[base] (pAddr, uncached)←AddressTranslation (vAddr, DATA, LOAD) |
3
pAddr←pAddr	|| (pAddr	xor ReverseEndian)
(PSIZE-1)..3 2..0
if BigEndianMem = 1 then
3
pAddr←pAddr || 0
(PSIZE-1)..3
endif
3
byte←vAddr xor BigEndianCPU
2..0
memdouble←LoadMemory (uncached, byte, pAddr, vAddr, DATA)
GPR[rt]←GPR[rt] || memdouble
63..64-8*byte 63..8*byte

Load HalfwordLH
31	26 25	21 20	16 15	0
LH	base	rt	offset
1 0 0 0 0 1
655 16
LH   rt, offset(base)
rt←memory[base+offset]
vAddr←sign_extend(offset) + GPR[base]
if (vAddr)≠0 then SignalException(AddressError) endif
0
(pAddr, uncached)←AddressTranslation (vAddr, DATA, LOAD)
pAddr←pAddr	|| (pAddr	xor (ReverseEndian || 0))
PSIZE – 1..2 1..0
memword←LoadMemory (uncached, HALFWORD, pAddr, vAddr, DATA)
byte←vAddr xor (BigEndianCPU || 0)
1..0
GPR[rt]←sign_extend(memword )
15+8*byte..8* byte
vAddr←sign_extend(offset) + GPR[base]
if (vAddr)≠0 then SignalException(AddressError) endif
0
(pAddr, uncached)←AddressTranslation (vAddr, DATA, LOAD)
pAddr←pAddr	|| (pAddr	xor (ReverseEndian || 0))
PSIZE – 1..3 2..0
memdouble←LoadMemory (uncached, HALFWORD, pAddr, vAddr, DATA)
2
byte←vAddr xor (BigEndianCPU|| 0)
2..0
GPR[rt]←sign_extend(memdouble )
15+8*byte..8* byte

LHU Load Halfword Unsigned
31	26 25	21 20	16 15	0
LHU	base	rt	offset
1 0 0 1 0 1
655 16
LHU   rt, offset(base)
rt←memory[base+offset]
vAddr←sign_extend(offset) + GPR[base]
if (vAddr)≠0 then SignalException(AddressError) endif
0
(pAddr, uncached)←AddressTranslation (vAddr, DATA, LOAD)
pAddr←pAddr	|| (pAddr	xor (ReverseEndian || 0))
PSIZE – 1..2 1..0
memword←LoadMemory (uncached, HALFWORD, pAddr, vAddr, DATA)
byte←vAddr xor (BigEndianCPU || 0)
1..0
GPR[rt]←zero_extend(memword )
15+8*byte..8*byte
vAddr←sign_extend(offset) + GPR[base]
if (vAddr)≠0 then SignalException(AddressError) endif
0
(pAddr, uncached)←AddressTranslation (vAddr, DATA, LOAD)
2
pAddr←pAddr	|| (pAddr	xor (ReverseEndian|| 0))
PSIZE – 1..3 2..0
memdouble←LoadMemory (uncached, HALFWORD, pAddr, vAddr, DATA)
2
byte←vAddr xor (BigEndianCPU|| 0)
2..0
GPR[rt]←zero_extend(memdouble )
15+8*byte..8*byte

Load Linked WordLL
31	26 25	21 20	16 15	0
LL	base	rt	offset
1 1 0 0 0 0
655 16
LL   rt, offset(base)
rt←memory[base+offset]

LL Load Linked Word
vAddr←sign_extend(offset) + GPR[base]
2
if (vAddr )≠0then SignalException(AddressError) endif
1..0
(pAddr, uncached)←AddressTranslation (vAddr, DATA, LOAD)
memword←LoadMemory (uncached, WORD, pAddr, vAddr, DATA)
GPR[rt]←memword
LLbit←1
vAddr←sign_extend(offset) + GPR[base]
2
if (vAddr )≠0then SignalException(AddressError) endif
1..0
(pAddr, uncached)←AddressTranslation (vAddr, DATA, LOAD)
2
pAddr←pAddr	|| (pAddr	xor (ReverseEndian || 0))
PSIZE-1..3 2..0
memdouble←LoadMemory (uncached, WORD, pAddr, vAddr, DATA)
2
byte←vAddr xor (BigEndianCPU || 0)
2..0
GPR[rt]←sign_extend(memdouble )
31+8*byte..8*byte
LLbit←1

Load Linked DoublewordLLD
31	26 25	21 20	16 15	0
LLD	base	rt	offset
1 1 0 1 0 0
655 16
LLD   rt, offset(base)
rt←memory[base+offset]

LLD Load Linked Doubleword
vAddr←sign_extend(offset) + GPR[base]
3
if (vAddr )≠0then SignalException(AddressError) endif
2..0
(pAddr, uncached)←AddressTranslation (vAddr, DATA, LOAD)
memdouble←LoadMemory (uncached, DOUBLEWORD, pAddr, vAddr, DATA)
GPR[rt]←memdouble
LLbit←1

Load Upper ImmediateLUI
31	26 25	21 20	16 15	0
LUI 0
rt immediate
0 0 1 1 1 1 0 0 0 0 0
655 16
LUI   rt, immediate
16
rt←immediate || 0
16
GPR[rt]←sign_extend(immediate || 0 )

LW Load Word
31	26 25	21 20	16 15	0
LW	base	rt	offset
1 0 0 0 1 1
655 16
LW   rt, offset(base)
rt←memory[base+offset]
vAddr←sign_extend(offset) + GPR[base]
2
if (vAddr )≠0then SignalException(AddressError) endif
1..0
(pAddr, uncached)←AddressTranslation (vAddr, DATA, LOAD)
memword←LoadMemory (uncached, WORD, pAddr, vAddr, DATA)
GPR[rt]←memword
vAddr←sign_extend(offset) + GPR[base]
2
if (vAddr )≠0then SignalException(AddressError) endif
1..0
(pAddr, uncached)←AddressTranslation (vAddr, DATA, LOAD)
2
pAddr←pAddr	|| (pAddr	xor (ReverseEndian || 0))
PSIZE-1..3 2..0
memdouble←LoadMemory (uncached, WORD, pAddr, vAddr, DATA)
2
byte←vAddr xor (BigEndianCPU || 0)
2..0
GPR[rt]←sign_extend(memdouble )
31+8*byte..8*byte

Load Word To CoprocessorLWCz
31	26 25	21 20	16 15	0
LWCz	base	rt	offset
1 1 0 0 z z
655 16
LWC1   rt, offset(base)
LWC2   rt, offset(base)
LWC3   rt, offset(base)
rt←memory[base+offset]
vAddr←sign_extend(offset) + GPR[base]
2
if (vAddr )≠0then SignalException(AddressError) endif
1..0
(pAddr, uncached)←AddressTranslation (vAddr, DATA, LOAD)
memword←LoadMemory (uncached, WORD, pAddr, vAddr, DATA)
COP_LW (z, rt, memword)

LWCz Load Word To Coprocessor
vAddr←sign_extend(offset) + GPR[base}
2
if (vAddr )≠0then SignalException(AddressError) endif
1..0
(pAddr, uncached)←AddressTranslation (vAddr, DATA, LOAD)
2
pAddr←pAddr	|| (pAddr	xor (ReverseEndian || 0))
PSIZE-1..3 2..0
memdouble←LoadMemory (uncached, DOUBLEWORD, pAddr, vAddr, DATA)
2
byte←vAddr xor (BigEndianCPU || 0)
2..0
memword←memdouble
31+8*byte..8*byte
COP_LW (z, rt, memdouble)

Load Word LeftLWL
| 31 | 26 25 | 21 20 | 16 15 | 0 |
| --- | --- | --- | --- | --- |
| LWL | base | rt | offset 1 0 0 0 1 0 655 16 LWL   rt, offset(base) rt←rt MERGE memory[base+offset] Word at byte 2 in memory, big-endian byte order, - each mem byte contains its address |  |
| most | - significance - 0123456789 Memoryinitial contents | least |  |  |
| e | f | g | h | 32-bitGPR 24:  Initial contents |
| a | b | c | d | e f g h 64-bitGPR 24 23gh After executingLWL $24,2($0) 23gh 2345 ThenafterLWR $24,5($0) |
2345

LWL Load Word Left
Memory contents and byte offsets Initial contents of Dest Register
0123←big-endian
IJKL	offset (vAddr	)	a	b	c	d	e	f	g	h
1..0
| 3210←little-endian | most | least |
| --- | --- | --- |
| 32-bit register | e | f g h Destination 64-bit register contents after instruction (shaded is unchanged) |
| Big-endian byte ordering | vAddr | Little-endian byte ordering 1..0 |
| IJKL | 0 | L f g h |
| JKLh | 1 | KLgh |
| KLgh | 2 | JKLh |
| L | f | g h 3 IJKL The word sign (31) is always loaded and the value is copied into bits 63..32. |
| 32-bit register | Big-endian | vAddr Little-endian 1..0 |
| IJKL | 0 | Lfgh |
| JKLh | 1 | KLgh |
| KLgh | 2 | JKLh |
| Lfgh | 3 | IJKL |

Load Word LeftLWL
vAddr←sign_extend(offset) + GPR[base]
(pAddr, uncached)←AddressTranslation (vAddr, DATA, LOAD)
2
pAddr←pAddr	|| (pAddr	xor ReverseEndian)
(PSIZE-1)..2 1..0
if BigEndianMem = 0 then
2
pAddr←pAddr || 0
(PSIZE-1)..2
endif
2
byte←vAddr xor BigEndianCPU
1..0
memword←LoadMemory (uncached, byte, pAddr, vAddr, DATA)
GPR[rt]←memword || GPR[rt]
7+8*byte..0 23–8*byte..0
vAddr←sign_extend(offset) + GPR[base]
(pAddr, uncached)←AddressTranslation (vAddr, DATA, LOAD)
3
pAddr←pAddr	|| (pAddr	xor ReverseEndian)
(PSIZE-1)..3 2..0
if BigEndianMem = 0 then
3
pAddr←pAddr || 0
(PSIZE-1)..3
endif
2
byte←0 || (vAddr xor BigEndianCPU)
1..0
word←vAddrxor BigEndianCPU
2
memdouble←LoadMemory (uncached, byte, pAddr, vAddr, DATA)
temp←memdouble || GPR[rt]
31+32*word-8*byte..32*word 23-8*byte..0
32
GPR[rt]←(temp	)	|| temp
31

LWR Load Word Right
| 31 | 26 25 | 21 20 | 16 15 | 0 |
| --- | --- | --- | --- | --- |
| LWR | base | rt | offset 1 0 0 1 1 0 655 16 LWR   rt, offset(base) rt←rt MERGE memory[base+offset] Word at byte 2 in memory, big-endian byte order, - each mem byte contains its address |  |
| most | - significance - 0123456789 Memoryinitial contents | least |  |  |
| e | f | g | h | 32-bitGPR 24:  Initial contents |
| a | b | c | d | e f g h 64-bitGPR 24 |
| e | f | 45 | After executingLWR $24,5($0) |  |
| no cng or sign ext | e | f | 45 2345 ThenafterLWL $24,2($0) |  |
2345

Load Word RightLWR
Memory contents and byte offsets Initial contents of Dest Register
0123←big-endian
IJKL	offset (vAddr	)	a	b	c	d	e	f	g	h
1..0
| 3210←little-endian | most | least |
| --- | --- | --- |
| 32-bit register | e | f g h Destination 64-bit register contents after instruction (shaded is unchanged) |
| Big-endian byte ordering | vAddr | Little-endian byte ordering 1..0 |
| e | f | g I 0 IJKL |
| e | f | IJ 1 e IJK |
| e | IJK | 2 e f IJ |
| IJKL | 3 | e f g I When the word sign bit (31) is loaded, its value is copied into bits 63..32.  When it is not loaded, the behavior is implementation specific.  Bits 63..32 are either unchanged or a the value of the unloaded bit 31 is copied into them. |
| 32-bit register | big-endian | vAddr little-endian 1..0 |
| e | f | g I 0 IJKL |
| efIJ | 1 | eIJK |
| eIJK | 2 | efIJ |
| IJKL | 3 | efgI |

LWR Load Word Right
vAddr←sign_extend(offset) + GPR[base]
(pAddr, uncached)←AddressTranslation (vAddr, DATA, LOAD)
2
pAddr←pAddr	|| (pAddr	xor ReverseEndian)
(PSIZE-1)..2 1..0
if BigEndianMem = 0 then
2
pAddr←pAddr || 0
(PSIZE-1)..2
endif
2
byte←vAddr xor BigEndianCPU
1..0
memword←LoadMemory (uncached, byte, pAddr, vAddr, DATA)
GPR[rt]←memword || GPR[rt]
31..32-8*byte 31–8*byte..0
vAddr←sign_extend(offset) + GPR[base]
(pAddr, uncached)←AddressTranslation (vAddr, DATA, LOAD)
3
pAddr←pAddr	|| (pAddr	xor ReverseEndian)
(PSIZE-1)..3 2..0
if BigEndianMem = 1 then
3
pAddr←pAddr || 0
(PSIZE-1)..3
endif
2
byte←vAddr xor BigEndianCPU
1..0
word←vAddrxor BigEndianCPU
2
memdouble←LoadMemory (uncached, 0 || byte, pAddr, vAddr, DATA)
temp←GPR[rt] || memdouble
31..32-8*byte 31+32*word..32*word+8*byte
if byte = 4 then
32 /* loaded bit 31, must sign extend */
utemp←(temp )
31
else
one of the following two behaviors:
utemp←GPR[rt]
63..32 /* leave what was there alone */
32 /* sign-extend bit 31 */
utemp←(GPR[rt] )
31
endif
GPR[rt]←utemp || temp

Load Word RightLWR

LWU Load Word Unsigned
31	26 25	21 20	16 15	0
LWU	base	rt	offset
1 0 0 1 1 1
655 16
LWU   rt, offset(base)
rt←memory[base+offset]
vAddr←sign_extend(offset) + GPR[base]
2
if (vAddr )≠0then SignalException(AddressError) endif
1..0
(pAddr, uncached)←AddressTranslation (vAddr, DATA, LOAD)
2
pAddr←pAddr	|| (pAddr	xor (ReverseEndian || 0))
PSIZE-1..3 2..0
memdouble←LoadMemory (uncached, WORD, pAddr, vAddr, DATA)
2
byte←vAddr xor (BigEndianCPU || 0)
2..0
32
GPR[rt]←0 || memdouble
31+8*byte..8*byte

Move From HI RegisterMFHI
| 31 | 26 25 | 16 15 | 11 10 | 65 | 0 |
| --- | --- | --- | --- | --- | --- |
| SPECIAL | 0 | rd | 0 | MFHI |  |
| 0 0 0 0 0 0 | 0 0  0 0 0 0  0 0 0 0 61055 6 MFHI   rd rd←HI GPR[rd]←HI | 0 0 0 0 0 | 0 1 0 0 0 0 |  |  |

MFLO Move From LO Register
| 31 | 26 25 | 16 15 | 11 10 | 65 | 0 |
| --- | --- | --- | --- | --- | --- |
| SPECIAL | 0 | rd | 0 | MFLO |  |
| 0 0 0 0 0 0 | 0 0  0 0 0 0  0 0 0 0 61055 6 MFLO   rd rd←LO GPR[rd]←LO | 0 0 0 0 0 | 0 1 0 0 1 0 |  |  |

Move Conditional on Not ZeroMOVN
| 31 | 2625 | 21 20 | 1615 | 11 10 | 65 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| SPECIAL | 0 | MOVN |  |  |  |  |
| rs | rt | rd |  |  |  |  |
| 0 0 0 0 0 0 | 0 0 0 0 0 | 0 0 1 0 1 1 |  |  |  |  |
| 655 | 55 MOVN     rd, rs, rt if (rt≠0) then rd←rs if GPR[rt]≠0 then GPR[rd]←GPR[rs] endif | 6 |  |  |  |  |

MOVZ Move Conditional on Zero
| 31 | 2625 | 21 20 | 1615 | 11 10 | 65 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| SPECIAL | 0 | MOVZ |  |  |  |  |
| rs | rt | rd |  |  |  |  |
| 0 0 0 0 0 0 | 0 0 0 0 0 | 0 0 1 0 1 0 |  |  |  |  |
| 655 | 55 MOVZ     rd, rs, rt if (rt = 0) then rd←rs if GPR[rt]=0 then GPR[rd]←GPR[rs] endif | 6 |  |  |  |  |

Move To HI RegisterMTHI
| 31 | 26 25 | 21 20 | 65 | 0 |
| --- | --- | --- | --- | --- |
| SPECIAL | rs | 0 | MTHI |  |
| 0 0 0 0 0 0 | 0   0 0 0 0   0 0 0 0   0 0 0 0   0 0 | 0 1 0 0 0 1 |  |  |
| 65 | 15 MTHI   rs HI←rs | 6 |  |  |
| MUL | r2,r4 | # start operation that will eventually write to HI,LO ... # code not containing mfhi or mflo MTHI r6 ... # code not containing mflo MFLO   r3 # this mflo would get an undefined value , HI←undefined HI←GPR[rs] |  |  |

MTLO Move To LO Register
| 31 | 26 25 | 21 20 | 65 | 0 |
| --- | --- | --- | --- | --- |
| SPECIAL | rs | 0 | MTLO |  |
| 0 0 0 0 0 0 | 0   0 0 0 0   0 0 0 0   0 0 0 0   0 0 | 0 1 0 0 1 1 |  |  |
| 65 | 15 MTLO   rs LO←rs | 6 |  |  |
| MUL | r2,r4 | # start operation that will eventually write to HI,LO ... # code not containing mfhi or mflo MTLO   r6 ... # code not containing mfhi |  |  |
| MFHI | r3 | # this mfhi would get an undefined value , LO←undefined LO←GPR[rs] |  |  |

Multiply WordMULT
| 31 | 26 25 | 21 20 | 16 15 | 65 | 0 |
| --- | --- | --- | --- | --- | --- |
| SPECIAL | rs | rt | 0 | MULT |  |
| 0 0 0 0 0 0 | 0 0  0 0 0 0  0 0 0 0 | 0 1 1 0 0 0 |  |  |  |
| 655 | 10 MULT   rs, rt (LO, HI)←rs×rt if (NotWordValue(GPR[rs]) or NotWordValue(GPR[rt])) then UndefinedResult() endif , LO, HI ←undefined | 6 |  |  |  |
| prod | ←GPR[rs] | * GPR[rt] 31..0 31..0 |  |  |  |
| LO | ←sign_extend(prod 31..0 | ) |  |  |  |
| H I | ←sign_extend(prod 63..32 | ) |  |  |  |

MULTU Multiply Unsigned Word
| 31 | 26 25 | 21 20 | 16 15 | 65 | 0 |
| --- | --- | --- | --- | --- | --- |
| SPECIAL | 0 | MULTU rs rt |  |  |  |
| 0 0 0 0 0 0 | 0 0  0 0 0 0  0 0 0 0 | 0 1 1 0 0 1 |  |  |  |
| 655 | 10 MULTU   rs, rt (LO, HI)←rs×rt if (NotWordValue(GPR[rs]) or NotWordValue(GPR[rt])) then UndefinedResult() endif , LO, HI ←undefined | 6 |  |  |  |
| prod | ←(0 \|\| GPR[rs] | ) * (0 \|\| GPR[rt] 31..0 31..0 | ) |  |  |
| LO | ←sign_extend(prod 31..0 | ) |  |  |  |
| H I | ←sign_extend(prod 63..32 | ) |  |  |  |

Not OrNOR
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 6    5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| SPECIAL | rs | rt | rd | 0 | NOR |  |
| 0 0 0 0 0 0 | 0 0 0 0 0 | 1 0 0 1 1 1 65555  6 NOR   rd, rs, rt rd←rs NOR rt GPR[rd]←GPR[rs] nor GPR[rt] |  |  |  |  |

OR Or
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 6    5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| SPECIAL | rs | rt | rd | 0 | OR |  |
| 0 0 0 0 0 0 | 0 0 0 0 0 | 1 0 0 1 0 1 65555  6 OR   rd, rs, rt rd←rs OR rt GPR[rd]←GPR[rs] or GPR[rt] |  |  |  |  |

Or ImmediateORI
31	26 25	21 20	16 15	0
ORI	rs	rt	immediate
0 0 1 1 0 1
655 16
ORI   rt, rs, immediate
rd←rs OR immediate
GPR[rt]←zero_extend(immediate) or GPR[rs]

PREF Prefetch
31	26 25	21 20	16 15	0
PREF
base	hint	offset
1 1 0 0 1 1
655 16
PREF   hint, offset(base)
prefetch_memory(base+offset)

PrefetchPREF
vAddr←GPR[base] + sign_extend(offset)
(pAddr, uncached)←AddressTranslation(vAddr, DATA, LOAD)
Prefetch(uncached, pAddr, vAddr, DATA, hint)

PREF Prefetch

Store ByteSB
31	26 25	21 20	16 15	0
SB	base	rt	offset
1 0 1 0 0 0
655 16
SB   rt, offset(base)
memory[base+offset]←rt
vAddr←sign_extend(offset) + GPR[base]
(pAddr, uncached)←AddressTranslation (vAddr, DATA, STORE)
2
pAddr←pAddr	|| (pAddr	xor ReverseEndian)
PSIZE-1..2 1..0
2
byte←vAddr xor  BigEndianCPU
1..0
8*byte
dataword←GPR[rt] || 0
31–8*byte..0
StoreMemory (uncached, BYTE, dataword, pAddr, vAddr, DATA)
vAddr←sign_extend(offset) + GPR[base]
(pAddr, uncached)←AddressTranslation (vAddr, DATA, STORE)
3
pAddr←pAddr	|| (pAddr	xor ReverseEndian)
PSIZE-1..3 2..0
3
byte←vAddr xor  BigEndianCPU
2..0
8*byte
datadouble←GPR[rt] || 0
63–8*byte..0
StoreMemory (uncached, BYTE, datadouble, pAddr, vAddr, DATA)

SC Store Conditional Word
31	26 25	21 20	16 15	0
SC	base	rt	offset
1 1 1 0 0 0
655 16
SC rt, offset(base)
if (atomic_update) then memory[base+offset]←rt, rt←1 else rt←0

Store Conditional WordSC
vAddr←sign_extend(offset) + GPR[base]
2
if (vAddr )≠0then SignalException(AddressError) endif
1..0
(pAddr, uncached)←AddressTranslation (vAddr, DATA, STORE)
dataword←GPR[rt]
if LLbit then
StoreMemory (uncached, WORD, dataword, pAddr, vAddr, DATA)
endif
31
GPR[rt]←0 ||LLbit

SC Store Conditional Word
vAddr←sign_extend(offset) + GPR[base]
2
if (vAddr )≠0then SignalException(AddressError) endif
1..0
(pAddr, uncached)←AddressTranslation (vAddr, DATA, STORE)
2
pAddr←pAddr	|| (pAddr	xor (ReverseEndian || 0))
PSIZE-1..3 2..0
2
byte←vAddr xor (BigEndianCPU || 0)
2..0
8*byte
datadouble←GPR[rt] || 0
63-8*byte..0
if LLbit then
StoreMemory (uncached, WORD, datadouble, pAddr, vAddr, DATA)
endif
63
GPR[rt]←0 ||LLbit
L1:
| LL | T1, (T0) | # load counter |
| --- | --- | --- |
| ADDI | T2, T1, 1 | # increment |
| SC | T2, (T0) | # try to store, checking for atomicity |
| BEQ | T2, 0, L1 | # if not atomic (0), try again NOP # branch-delay slot |

Store Conditional DoublewordSCD
31	26 25	21 20	16 15	0
SCD	base	rt	offset
1 1 1 1 0 0
655 16
SCD   rt, offset(base)
if (atomic_update) then memory[base+offset]←rt, rt←1 else rt←0

SCD Store Conditional Doubleword
vAddr←sign_extend(offset) + GPR[base]
3
if (vAddr )≠0then SignalException(AddressError) endif
2..0
(pAddr, uncached)←AddressTranslation (vAddr, DATA, STORE)
datadouble←GPR[rt]
if LLbit then
StoreMemory (uncached, DOUBLEWORD, datadouble, pAddr, vAddr, DATA)
endif
63
GPR[rt]←0 ||LLbit

Store Conditional DoublewordSCD
L1:
| LLD | T1, (T0) | # load counter |
| --- | --- | --- |
| ADDI | T2, T1, 1 | # increment |
| SCD | T2, (T0) | # try to store, checking for atomicity |
| BEQ | T2, 0, L1 | # if not atomic (0), try again NOP # branch-delay slot |

SD Store Doubleword
31	26 25	21 20	16 15	0
SD	base	rt	offset
1 1 1 1 1 1
655 16
SD   rt, offset(base)
memory[base+offset]←rt
vAddr←sign_extend(offset) + GPR[base]
3
if (vAddr )≠0then SignalException(AddressError) endif
2..0
(pAddr, uncached)←AddressTranslation (vAddr, DATA, STORE)
datadouble←GPR[rt]
StoreMemory (uncached, DOUBLEWORD, datadouble, pAddr, vAddr, DATA)

Store Doubleword From CoprocessorSDCz
31	26 25	21 20	16 15	0
SDCz	base	rt	offset
1 1 1 1 z z
655 16
SDC1   rt, offset(base)
SDC2   rt, offset(base)
memory[base+offset]←rt
vAddr←sign_extend(offset) + GPR[base]
3
if (vAddr )≠0then SignalException(AddressError) endif
2..0
(pAddr, uncached)←AddressTranslation (vAddr, DATA, STORE)
datadouble←COP_SD(z, rt)
StoreMemory (uncached, DOUBLEWORD, datadouble, pAddr, vAddr, DATA)

SDCz Store Doubleword From Coprocessor
vAddr←sign_extend(offset) + GPR[base]
3
if (vAddr )≠0then SignalException(AddressError) endif
2..0
(pAddr, uncached)←AddressTranslation (vAddr, DATA, STORE)
datadouble←COP_SD(z, rt)
StoreMemory (uncached, DOUBLEWORD, datadouble, pAddr, vAddr, DATA)

Store Doubleword LeftSDL
| 31 | 26 25 | 21 20 | 16 15 | 0 |
| --- | --- | --- | --- | --- |
| SDL | base | rt | offset 1 0 1 1 0 0 655 16 SDL   rt, offset(base) memory[base+offset]←Some_Bytes_From rt Doubleword at byte 2 in memory (big-endian) - each memory byte contains its address most      — significance — least |  |
| 0 | 1 | 2 | 3 | 4 5 6 7 8 9 10 11 12 13 14 15 Memory ABCDEFGHGPR 24 After executing |
| 0 | 1 | ABCDEF8910 ... Then | SDL $24,2($0) |  |
| 0 | 1 | ABCDEFGH10 ... | SDR $24,9($0) |  |

SDL Store Doubleword Left
Initial Memory contents and byte offsets Contents of
| most | least | Source Register |
| --- | --- | --- |
| 01234567←big- | most | least |
| i | j | k l m n o p ABCDE FGH 76543210←little-endian Memory contents after instruction (shaded is unchanged) |
| Big-endian byte ordering | vAddr | Little-endian byte ordering 2..0 |
| ABCDEFGH | 0 | i j k lmnoA |
| iABCDEFG | 1 | i j k lmnAB |
| i jABCDE F | 2 | i j k lmABC |
| i j kABCDE | 3 | i j k lABCD |
| i j k lABCD | 4 | i j kABCDE |
| i j k lmABC | 5 | i jABCDEF |
| i j k lmnAB | 6 | iABCDEFG |
| i j k lmnoA | 7 | ABCDE FGH vAddr←sign_extend(offset) + GPR[base] (pAddr, uncached)←AddressTranslation (vAddr, DATA, STORE) |
3
pAddr←pAddr	|| (pAddr	xor  ReverseEndian)
(PSIZE-1)..3 2..0
If BigEndianMem = 0 then
3
pAddr←pAddr || 0
(PSIZE-1)..3
endif
3
byte←vAddr xor  BigEndianCPU
2..0
56–8*byte
datadouble←0 || GPR[rt]
63..56–8*byte
StoreMemory (uncached, byte, datadouble, pAddr, vAddr, DATA)

Store Doubleword RightSDR
| 31 | 26 25 | 21 20 | 16 15 | 0 |
| --- | --- | --- | --- | --- |
| SDR | base | rt | offset 1 0 1 1 0 1 655 16 SDR   rt, offset(base) memory[base+offset]←Some_Bytes_From rt Doubleword at byte 2 in memory, big-endian byte order, - each mem byte contains its address most      — significance — least |  |
| 0 | 1 | 2 | 3 | 4 5 6 7 8 9 10 11 12 13 14 15 Memory ABCDEFGHGPR 24 After executing |
| 0 | 1 | 2 | 3 | 4 5 6 7 GH10 ... SDR $24,9($0) Then after |
| 0 | 1 | ABCDEFGH10 ... | SDL $24,2($0) |  |

SDR Store Doubleword Right
Initial Memory contents and byte offsets Contents of
| most | least | Source Register |
| --- | --- | --- |
| 01234567←big- | most | least |
| i | j | k l m n o p ABCDE FGH 76543210 ̈little-endian Memory contents after instruction (shaded is unchanged) |
| Big-endian byte ordering | vAddr | Little-endian byte ordering 2..0 |
| H | j | k l m n o p 0 ABCDE FGH |
| GHk lmnop | 1 | BCDEFGHp |
| FGHlmnop | 2 | CDEFGHop |
| EFGHmnop | 3 | DEFGHnop |
| DEFGHnop | 4 | EFGHmnop |
| CDE FGHop | 5 | FGHlmnop |
| BCDEFGHp | 6 | GHk lmnop |
| ABCDEFGH | 7 | Hj k lmnop vAddr←sign_extend(offset) + GPR[base] (pAddr, uncached)←AddressTranslation (vAddr, DATA, STORE) |
3
pAddr←pAddr	|| (pAddr	xor  ReverseEndian)
(PSIZE-1)..3 2..0
If BigEndianMem = 0 then
3
pAddr←pAddr || 0
(PSIZE-1)..3
endif
3
byte←vAddr xor BigEndianCPU
1..0
8*byte
datadouble←GPR[rt] || 0
63–8*byte
StoreMemory (uncached, DOUBLEWORD-byte, datadouble, pAddr, vAddr, DATA)

Store HalfwordSH
31	26 25	21 20	16 15	0
SH	base	rt	offset
1 0 1 0 0 1
655 16
SH   rt, offset(base)
memory[base+offset]←rt
vAddr←sign_extend(offset) + GPR[base]
if (vAddr)≠0 then SignalException(AddressError) endif
0
(pAddr, uncached)←AddressTranslation (vAddr, DATA, STORE)
pAddr←pAddr	|| (pAddr	xor (ReverseEndian || 0))
PSIZE-1..2 1..0
byte←vAddr xor (BigEndianCPU || 0)
1..0
8*byte
dataword←GPR[rt] || 0
31–8*byte..0
StoreMemory (uncached, HALFWORD, dataword, pAddr, vAddr, DATA)
vAddr←sign_extend(offset) + GPR[base]
if (vAddr)≠0 then SignalException(AddressError) endif
0
(pAddr, uncached)←AddressTranslation (vAddr, DATA, STORE)
2
pAddr←pAddr	|| (pAddr	xor (ReverseEndian|| 0))
PSIZE-1..3 2..0
2
byte←vAddr xor (BigEndianCPU|| 0)
2..0
8*byte
datadouble←GPR[rt] || 0
63–8*byte..0
StoreMemory (uncached, HALFWORD, datadouble, pAddr, vAddr, DATA)

SLL Shift Word Left Logical
31	26 25	21 20	16 15	11  10	6    5	0
0
| SPECIAL | rt | rd | sa | SLL |
| --- | --- | --- | --- | --- |
| 0 0 0 0 0 0 | 0 0 0 0 0 | 0 0 0 0 0 0 65555  6 SLL   rd, rt, sa rd←rt << sa s ←sa s |  |  |
| temp | ←GPR[rt] (31-s)..0 GPR[rd]←sign_extend(temp) | \|\| 0 |  |  |

Shift Word Left Logical VariableSLLV
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 6    5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| SPECIAL | rs | rt | rd | 0 | SLLV |  |
| 0 0 0 0 0 0 | 0 0 0 0 0 | 0 0 0 1 0 0 65555  6 SLLV   rd, rt, rs rd←rt << rs s ←GP[rs] 4..0 s |  |  |  |  |
| temp | ←GPR[rt] (31-s)..0 GPR[rd]←sign_extend(temp) | \|\| 0 |  |  |  |  |

SLT Set On Less Than
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 6    5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| SPECIAL | rs | rt | rd | 0 | SLT |  |
| 0 0 0 0 0 0 | 0 0 0 0 0 | 1 0 1 0 1 0 65555  6 SLT   rd, rs, rt rd←(rs < rt) if GPR[rs] < GPR[rt] then |  |  |  |  |
GPRLEN-1
GPR[rd]←0 || 1
else
GPRLEN
GPR[rd]←0
endif

Set on Less Than ImmediateSLTI
31	26 25	21 20	16 15	0
SLTI	rs	rt	immediate
0 0 1 0 1 0
655 16
SLTI   rt, rs, immediate
rt←(rs < immediate)
if GPR[rs] < sign_extend(immediate) then
GPRLEN-1
GPR[rd]←0 || 1
else
GPRLEN
GPR[rd]←0
endif

SLTIU Set on Less Than Immediate Unsigned
31	26 25	21 20	16 15	0
SLTIU	rs	rt	immediate
0 0 1 0 1 1
655 16
SLTIU   rt, rs, immediate
rt←(rs < immediate)
if (0 || GPR[rs]) < (0 || sign_extend(immediate)) then
GPRLEN-1
GPR[rd]←0 || 1
else
GPRLEN
GPR[rd]←0
endif

Set on Less Than UnsignedSLTU
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 6    5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| SPECIAL | rs | rt | rd | 0 | SLTU |  |
| 0 0 0 0 0 0 | 0 0 0 0 0 | 1 0 1 0 1 1 65555  6 SLTU   rd, rs, rt rd←(rs < rt) if (0 \|\| GPR[rs]) < (0 \|\| GPR[rt]) then |  |  |  |  |
GPRLEN-1
GPR[rd]←0 || 1
else
GPRLEN
GPR[rd]←0
endif

SRA Shift Word Right Arithmetic
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 6    5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| SPECIAL | 0 | rt | rd | sa | SRA |  |
| 0 0 0 0 0 0 | 0 0 0 0 0 | 0 0 0 0 1 1 65555  6 SRA   rd, rt, sa rd←rt >> sa      (arithmetic) if (NotWordValue(GPR[rt])) then UndefinedResult() endif s ←sa s |  |  |  |  |
| temp | ←(GPR[rt] | )\|\| GPR[rt] 31 31..s GPR[rd]←sign_extend(temp) |  |  |  |  |

Shift Word Right Arithmetic VariableSRAV
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 6    5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| SPECIAL | rs | rt | rd | 0 | SRAV |  |
| 0 0 0 0 0 0 | 0 0 0 0 0 | 0 0 0 1 1 1 65555  6 SRAV   rd, rt, rs rd←rt >> rs      (arithmetic) if (NotWordValue(GPR[rt])) then UndefinedResult() endif s ←GPR[rs] 4..0 s |  |  |  |  |
| temp | ←(GPR[rt] | )\|\| GPR[rt] 31 31..s GPR[rd]←sign_extend(temp) |  |  |  |  |

SRL Shift Word Right Logical
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 6    5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| SPECIAL | 0 | rt | rd | sa | SRL |  |
| 0 0 0 0 0 0 | 0 0 0 0 0 | 0 0 0 0 1 0 65555  6 SRL   rd, rt, sa rd←rt >> sa      (logical) if (NotWordValue(GPR[rt])) then UndefinedResult() endif s ←sa s temp ←0\|\| GPR[rt] 31..s GPR[rd]←sign_extend(temp) |  |  |  |  |

Shift Word Right Logical VariableSRLV
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 6    5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| SPECIAL | rs | rt | rd | 0 | SRLV |  |
| 0 0 0 0 0 0 | 0 0 0 0 0 | 0 0 0 1 1 0 65555  6 SRLV   rd, rt, rs rd←rt >> rs      (logical) if (NotWordValue(GPR[rt])) then UndefinedResult() endif s ←GPR[rs] 4..0 s temp ←0\|\| GPR[rt] 31..s GPR[rd]←sign_extend(temp) |  |  |  |  |

SUB Subtract Word
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 6    5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| SPECIAL | rs | rt | rd | 0 | SUB |  |
| 0 0 0 0 0 0 | 0 0 0 0 0 | 1 0 0 0 1 0 65555  6 SUB rd, rs, rt rd←rs - rt if (NotWordValue(GPR[rs]) or NotWordValue(GPR[rt])) then UndefinedResult() endif temp←GPR[rs] - GPR[rt] if (32_bit_arithmetic_overflow) then SignalException(IntegerOverflow) else GPR[rd]←temp endif |  |  |  |  |

Subtract Unsigned WordSUBU
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 6    5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| SPECIAL | rs | rt | rd | 0 | SUBU |  |
| 0 0 0 0 0 0 | 0 0 0 0 0 | 1 0 0 0 1 1 65555  6 SUBU   rd, rs, rt rd←rs - rt if (NotWordValue(GPR[rs]) or NotWordValue(GPR[rt])) then UndefinedResult() endif temp←GPR[rs] - GPR[rt] GPR[rd]←temp |  |  |  |  |

SW Store Word
31	26 25	21 20	16 15	0
SW	base	rt	offset
1 0 1 0 1 1
655 16
SW   rt, offset(base)
memory[base+offset]←rt
vAddr←sign_extend(offset) + GPR[base]
2
if (vAddr )≠0then SignalException(AddressError) endif
1..0
(pAddr, uncached)←AddressTranslation (vAddr, DATA, STORE)
dataword←GPR[rt]
StoreMemory (uncached, WORD, dataword, pAddr, vAddr, DATA)
vAddr←sign_extend(offset) + GPR[base]
2
if (vAddr )≠0then SignalException(AddressError) endif
1..0
(pAddr, uncached)←AddressTranslation (vAddr, DATA, STORE)
2
pAddr←pAddr	|| (pAddr	xor (ReverseEndian || 0)
PSIZE-1..3 2..0
2
byte←vAddr xor (BigEndianCPU || 0)
2..0
8*byte
datadouble←GPR[rt] || 0
63-8*byte
StoreMemory (uncached, WORD, datadouble, pAddr, vAddr, DATA)

Store Word From CoprocessorSWCz
31	26 25	21 20	16 15	0
SWCz	base	rt	offset
1 1 1 0 z z
655 16
SWC1   rt, offset(base)
SWC2   rt, offset(base)
SWC3   rt, offset(base)
memory[base+offset]←rt
vAddr←sign_extend(offset) + GPR[base]
2
if (vAddr )≠0then SignalException(AddressError) endif
1..0
(pAddr, uncached)←AddressTranslation (vAddr, DATA, STORE)
dataword←COP_SW (z, rt)
StoreMemory (uncached, WORD, dataword, pAddr, vAddr, DATA)

SWCz Store Word From Coprocessor
vAddr←sign_extend(offset) + GPR[base]
2
if (vAddr )≠0then SignalException(AddressError) endif
1..0
(pAddr, uncached)←AddressTranslation (vAddr, DATA, STORE)
2
pAddr←pAddr	|| (pAddr	xor (ReverseEndian || 0)
PSIZE-1..3 2..0
2
byte←vAddr xor (BigEndianCPU || 0)
2..0
dataword←COP_SW (z, rt)
32-8*byte 8*byte
datadouble←0 || dataword || 0
StoreMemory (uncached, WORD, datadouble, pAddr, vAddr DATA)

Store Word LeftSWL
| 31 | 26 25 | 21 20 | 16 15 | 0 |
| --- | --- | --- | --- | --- |
| SWL | base | rt | offset 1 0 1 0 1 0 655 16 SWL   rt, offset(base) memory[base+offset]←rt Word at byte 2 in memory, big-endian byte order, - each mem byte contains its address most      — significance —       least |  |
| 0 | 1 | 2 | 3 | 4 5 6 7 8 ... 64-bitGPR 24 ABCDEFGH 32-bitGPR 24 EFGH 0 1EF456... After executingSWL $24,2($0) |
| 0 | 1 | EFGH6... | Then afterSWR $24,5($0) |  |

SWL Store Word Left
Memory contents and byte offsets Initial contents of Dest Register
0123←big-endian
| i | j | k | l | offset  (vAddr 1..0 | ) | ABCDEFGH |
| --- | --- | --- | --- | --- | --- | --- |
| 3210←little-endian | most | least 32-bit register EFGH Memory contents after instruction (shaded is unchanged) Big-endian Little-endian vAddr 1..0 byte ordering byte ordering |  |  |  |  |
| EFGH | 0 | i j kE |  |  |  |  |
| iEFG | 1 | i jEF |  |  |  |  |
| i jEF | 2 | iEFG |  |  |  |  |
| i j kE | 3 | EFGH vAddr←sign_extend(offset) + GPR[base] (pAddr, uncached)←AddressTranslation (vAddr, DATA, STORE) |  |  |  |  |
2
pAddr←pAddr	|| (pAddr	xor  ReverseEndian)
(PSIZE-1)..2 1..0
If BigEndianMem = 0 then
2
pAddr←pAddr || 0
(PSIZE-1)..2
endif
2
byte←vAddr xor  BigEndianCPU
1..0
24–8*byte
dataword←0 || GPR[rt]
31..24–8*byte
StoreMemory (uncached, byte, dataword, pAddr, vAddr, DATA)

Store Word LeftSWL
vAddr←sign_extend(offset) + GPR[base]
(pAddr, uncached)←AddressTranslation (vAddr, DATA, STORE)
3
pAddr←pAddr	|| (pAddr	xor  ReverseEndian)
(PSIZE-1)..3 2..0
If BigEndianMem = 0 then
2
pAddr←pAddr || 0
(PSIZE-1)..2
endif
2
byte←vAddr xor  BigEndianCPU
1..0
if (vAddrxor BigEndianCPU) = 0 then
2
32 24-8*byte
datadouble←0	|| 0	|| GPR[rt]
31..24-8*byte
else
24-8*byte 32
datadouble←0	|| GPR[rt]	|| 0
31..24-8*byte
endif
StoreMemory(uncached, byte, datadouble, pAddr, vAddr, DATA)

SWR Store Word Right
| 31 | 26 25 | 21 20 | 16 15 | 0 |
| --- | --- | --- | --- | --- |
| SWR | base | rt | offset 1 0 1 1 1 0 655 16 SWR   rt, offset(base) memory[base+offset]←rt Word at byte 2 in memory, big-endian byte order, - each mem byte contains its address most      — significance —       least |  |
| 0 | 1 | 2 | 3 | 4 5 6 7 8 ... 64-bitGPR 24 ABCDEFGH 32-bitGPR 24 EFGH 0 1 2 3GH6... After executingSWR $24,5($0) |
| 0 | 1 | EFGH6... | Then afterSWL $24,2($0) |  |

Store Word RightSWR
Memory contents and byte offsets Initial contents of Dest Register
0123←big-endian
| i | j | k | l | offset  (vAddr 1..0 | ) | ABCDEFGH |
| --- | --- | --- | --- | --- | --- | --- |
| 3210←little-endian | most | least 32-bit register EFGH Memory contents after instruction (shaded is unchanged) Big-endian Little-endian vAddr 1..0 byte ordering byte ordering |  |  |  |  |
| H | j | k | l | 0 | EFGH |  |
| GHk l | 1 | FGHl |  |  |  |  |
| FGHl | 2 | GHk l |  |  |  |  |
| EFGH | 3 | Hj k l vAddr←sign_extend(offset) + GPR[base] (pAddr, uncached)←AddressTranslation (vAddr, DATA, STORE) |  |  |  |  |
2
pAddr←pAddr	|| (pAddr	xor  ReverseEndian)
(PSIZE-1)..2 1..0
BigEndianMem = 0 then
2
pAddr←pAddr || 0
(PSIZE-1)..2
endif
2
byte←vAddr xor  BigEndianCPU
1..0
8*byte
dataword←GPR[rt] || 0
31–8*byte
StoreMemory (uncached, WORD-byte, dataword, pAddr, vAddr, DATA)

SWR Store Word Right
vAddr←sign_extend(offset) + GPR[base]
(pAddr, uncached)←AddressTranslation (vAddr, DATA, STORE)
3
pAddr←pAddr	|| (pAddr	xor  ReverseEndian)
(PSIZE-1)..3 2..0
If BigEndianMem = 0 then
2
pAddr←pAddr || 0
(PSIZE-1)..2
endif
2
byte←vAddr xor  BigEndianCPU
1..0
if (vAddrxor BigEndianCPU) = 0 then
2
32 8*byte
datadouble←0	|| GPR[rt]	|| 0
31-8*byte..0
else
8*byte 32
datadouble←GPR[rt]	|| 0	|| 0
31-8*byte..0
endif
StoreMemory(uncached, WORD-byte, datadouble, pAddr, vAddr, DATA)

Synchronize Shared MemorySYNC
| 31 | 26 25 | 1110 | 65 | 0 |
| --- | --- | --- | --- | --- |
| SPECIAL | 0 | SYNC stype |  |  |
| 0 0 0 0 0 0 | 0 0   0 0 0 0   0 0 0 0   0 0 0 0   0 6155 6 SYNC                        (stype = 0 implied) | 0 0 1 1 1 1 |  |  |

SYNC Synchronize Shared Memory
SyncOperation(stype)

Synchronize Shared MemorySYNC

SYNC Synchronize Shared Memory
Processor A (writer)
# Conditions at entry:
# The value 0 has been stored in FLAG and that value is observable by B.
| SW | R1, DATA | # change shared DATA value LI R2, 1 SYNC # perform DATA store before performing FLAG store |
| --- | --- | --- |
| SW | R2, FLAG | # say that the shared DATA value is valid Processor B (reader) LI R2, 1 |
| 1: | LW | R1, FLAG # get FLAG |
| BNE | R2, R1, 1B | # if it says that DATA is not valid, poll again |
NOP
SYNC # FLAG value checked before doing DATA reads
LW	R1, DATA	# read (valid) shared DATA values

System CallSYSCALL
31	26 25	65	0
SPECIAL	Code	SYSCALL
0 0 0 0 0 0 0 0 1 1 00
6206
SYSCALL
SignalException(SystemCall)

TEQ Trap if Equal
| 31 | 26 25 | 21 20 | 16 15 | 65 | 0 |
| --- | --- | --- | --- | --- | --- |
| SPECIAL | rs | rt | code 0 0 0 0 0 0 1 1 0 1 0 0 | TEQ |  |
| 655 | 10 TEQ   rs, rt if (rs = rt) then Trap if GPR[rs] = GPR[rt] then SignalException(Trap) endif | 6 |  |  |  |

Trap if Equal ImmediateTEQI
31	26 25	21 20	16 15	0
REGIMM	rs	TEQI	immediate
0 0 0 0 0 1 0 1 1 0 0
655 16
TEQI   rs, immediate
if (rs = immediate) then Trap
if GPR[rs] = sign_extend(immediate) then
SignalException(Trap)
endif

TGE Trap if Greater or Equal
| 31 | 26 25 | 21 20 | 16 15 | 65 | 0 |
| --- | --- | --- | --- | --- | --- |
| SPECIAL | rs | rt | code 0 0 0 0 0 0 1 1 0 0 0 0 | TGE |  |
| 655 | 10 TGE   rs, rt if (rs≥rt) then Trap if GPR[rs]≥GPR[rt] then SignalException(Trap) endif | 6 |  |  |  |

Trap if Greater or Equal ImmediateTGEI
31	26 25	21 20	16 15	0
REGIMM	rs	TGEI	immediate
0 0 0 0 0 1 0 1 0 0 0
655 16
TGEI   rs, immediate
if (rs≥immediate) then Trap
if GPR[rs]≥sign_extend(immediate) then
SignalException(Trap)
endif

TGEIU Trap If Greater Or Equal Immediate Unsigned
31	26 25	21 20	16 15	0
REGIMM	rs	TGEIU	immediate
0 0 0 0 0 1 0 1 0 0 1
655 16
TGEIU   rs, immediate
if (rs≥immediate) then Trap
if (0 || GPR[rs])≥(0 || sign_extend(immediate)) then
SignalException(Trap)
endif

Trap If Greater or Equal UnsignedTGEU
| 31 | 26 25 | 21 20 | 16 15 | 65 | 0 |
| --- | --- | --- | --- | --- | --- |
| SPECIAL | rs | rt | code 0 0 0 0 0 0 1 1 0 0 0 1 | TGEU |  |
| 655 | 10 TGEU   rs, rt if (rs≥rt) then Trap if (0 \|\| GPR[rs])≥(0 \|\| GPR[rt]) then SignalException(Trap) endif | 6 |  |  |  |

TLT Trap if Less Than
| 31 | 26 25 | 21 20 | 16 15 | 65 | 0 |
| --- | --- | --- | --- | --- | --- |
| SPECIAL | rs | rt | code 0 0 0 0 0 0 1 1 0 0 1 0 | TLT |  |
| 655 | 10 TLT   rs, rt if (rs < rt) then Trap if GPR[rs] < GPR[rt] then SignalException(Trap) endif | 6 |  |  |  |

Trap if Less Than ImmediateTLTI
31	26 25	21 20	16 15	0
REGIMM	rs	TLTI	immediate
0 0 0 0 0 1 0 1 0 1 0
655 16
TLTI   rs, immediate
if (rs < immediate) then Trap
if GPR[rs] < sign_extend(immediate) then
SignalException(Trap)
endif

TLTIU Trap if Less Than Immediate Unsigned
31	26 25	21 20	16 15	0
REGIMM	rs	TLTIU	immediate
0 0 0 0 0 1 0 1 0 1 1
655 16
TLTIU   rs, immediate
if (rs < immediate) then Trap
if (0 || GPR[rs]) < (0 || sign_extend(immediate)) then
SignalException(Trap)
endif

Trap if Less Than UnsignedTLTU
| 31 | 26 25 | 21 20 | 16 15 | 65 | 0 |
| --- | --- | --- | --- | --- | --- |
| SPECIAL | rs | rt | code 0 0 0 0 0 0 1 1 0 0 1 1 | TLTU |  |
| 655 | 10 TLTU   rs, rt if (rs < rt) then Trap if (0 \|\| GPR[rs]) < (0 \|\| GPR[rt]) then SignalException(Trap) endif | 6 |  |  |  |

TNE Trap if Not Equal
| 31 | 26 25 | 21 20 | 16 15 | 65 | 0 |
| --- | --- | --- | --- | --- | --- |
| SPECIAL | rs | rt | code 0 0 0 0 0 0 1 1 0 1 1 0 | TNE |  |
| 655 | 10 TNE   rs, rt if (rs≠rt) then Trap if GPR[rs]≠GPR[rt] then SignalException(Trap) endif | 6 |  |  |  |

Trap if Not Equal ImmediateTNEI
31	26 25	21 20	16 15	0
REGIMM	rs	TNEI	immediate
0 0 0 0 0 1 0 1 1 1 0
655 16
TNEI   rs, immediate
if (rs≠immediate) then Trap
if GPR[rs]≠sign_extend(immediate) then
SignalException(Trap)
endif

XOR Exclusive OR
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 6    5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| SPECIAL | rs | rt | rd | 0 | XOR |  |
| 0 0 0 0 0 0 | 0 0 0 0 0 | 1 0 0 1 1 0 65555  6 XOR   rd, rs, rt rd←rs XOR rt GPR[rd]←GPR[rs] xor GPR[rt] |  |  |  |  |

Exclusive OR ImmediateXORI
31	26 25	21 20	16 15	0
XORI	rs	rt	immediate
0 0 1 1 1 0
655 16
XORI   rt, rs, immediate
rt←rs XOR immediate
GPR[rt]←GPR[rs] xor zero_extend(immediate)

I-Type (Immediate).
| 31 | 26 25 | 21 20 | 16 15 | 0 |
| --- | --- | --- | --- | --- |
| opcode | rs | rt | offset 655 16 J-Type (Jump). |  |
| 31 | 26 25 opcode instr_index 6 26 R-Type (Register). | 0 |  |  |
| 31 | 26 25 | 21 20 | 16 15 | 11  10 6    5 0 |
| opcode | rs | rt | rd | sa function 65555  6 |

| 31 | 26 opcode δ δ | 0 |
| --- | --- | --- |
| δ,π | δ,π | δ,π δ,π,κ ∗ |
| π | π | π,κ |
| π | π | π,κ |
| 31 | 26 opcode function | 50 |
| 31 | 26 | 20 16 0 opcode rt |

| 31 | 26 opcode δ δ | 0 |
| --- | --- | --- |
| δ,π | δ,π | δ,π δ,π,κ ∗ ρ |
| π | π | π,κ π π π,κ |
| π | π | π,κ π π π,κ |
| 31 | 26 opcode function | 50 |
| 31 | 26 | 20 16 0 opcode rt |

| 31 | 26 opcode δ δ | 0 |
| --- | --- | --- |
| δ,π | δ,π | δ,π ∗ ρ |
| π | π | ∗ π π |
| π | π | ∗ π π |
| 31 | 26 opcode function | 50 |
| 31 | 26 | 20 16 0 opcode rt |

| 31 | 26 opcode δ δ | 0 |
| --- | --- | --- |
| δ,π | δ,π | δ,π δ,π ρ |
| π | π | π π |
| π | π | ∗ π π |
| 31 | 26 opcode function δ,μ | 50 |
| 31 | 26 | 20 16 0 opcode rt |

| 31 | 26 opcode ρ ∗ | 0 |
| --- | --- | --- |
| 31 | 26 opcode function | 50 |
| 31 | 26 | 20 16 0 opcode rt |

| 31 | 26 opcode ρ | 0 |
| --- | --- | --- |
| π | π | π |
| π | π | π |
| 31 | 26 opcode function | 50 |
| 31 | 26 | 20 16 0 opcode rt |

31	26	0
opcode
31	26	50
opcode
function

31	26	20	16	0
opcode
rt

| 31 | 26 opcode δ,π | 0 |
| --- | --- | --- |
| 31 | 26 opcode function δ,μ | 50 |
| 31 | 26 | 20 16 0 opcode rt |

δ
π
κ
μ
ρ

B

+∞ ∞
parameter	Single	Double

unbiasedEf    s   b1	valuev	type of value
≠
∞
∞
≠

∞ ∞
∞
∞
∞ ∞
∞
∞
∞
∞

Format New QNaN value
7fbf ffff
7ff7 ffff ffff ffff
7fff ffff
7fff ffff ffff ffff

#
0

empty empty
empty empty
⇓LWC1 f0,0(r0) / MTC1 f0,r0 ⇓
undefined/unused
empty empty
⇓LWC1 f1,4(r0) / MTC1 f1,r4 ⇓
undefined/unused
undefined/unused
empty empty
empty empty
⇓LDC1 f0,0(r0) / DMTC1 f0,r0 ⇓
empty
⇓LDC1 f1,8(r0) / DMTC1 f1,r8 ⇓

undefined/unused
undefined/unused empty — available to hold an operand

empty — available to hold an operand

Bit   Description Default Action
∞	∞	∞	∞
×∞
∞ ∞

∞
∞
±
±
±
±

• ↔
• ↔
• ↔

| Mnemonic | Description | Defined in |
| --- | --- | --- |
| Mnemonic | Description | Defined in |
| Mnemonic | Description | Defined in |

| Mnemonic | Description | Defined in |
| --- | --- | --- |
| Mnemonic | Description | Defined in |
| Mnemonic | Description | Defined in |

Mnemonic	Description	Defined in
Mnemonic	Description	Defined in

| Mnemonic | Description | Defined in |
| --- | --- | --- |
| Mnemonic | Description | Defined in |
| Mnemonic | Description | Defined in |

Mnemonic	Description	Defined in
Mnemonic	Description	Defined in

••
••
••
••
∞
• ••
••
•••
••
••
••
∞
••
••

••
≠ ••
••
••
••
••
••
••
••
••
••
••
••
••
• −	.	−	−

ABS.fmt Floating-Point Absolute Value
| 31 | 26  25 | 21   20 | 16   15 | 11   10 | 6   5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| COP1 | fmt | 0 | fs | fd | ABS |  |
| 0 1 0 0 0 1 | 0 0 0 0 0 | 0 0 0 1 0 1 |  |  |  |  |
655556
ABS.S   fd, fs
ABS.D   fd, fs
fd←absolute(fs)
StoreFPR(fd, fmt, AbsoluteValue(ValueFPR(fs, fmt)))

Floating-Point  AddADD.fmt
31	26  25	21   20	16   15	11   10	6   5	0
COP1	fmt	ft	fs	fd	ADD
0 1 0 0 0 1 0 0 0 0 0 0
655556
ADD.S   fd, fs, ft
ADD.D   fd, fs, ft
fd←fs + ft
StoreFPR (fd, fmt, ValueFPR(fs, fmt) + ValueFPR(ft, fmt))

BC1F Branch on FP False
| 31 | 26 25 | 21  201817 1615 | 0 |
| --- | --- | --- | --- |
| COP1 | BC offset cc | tf |  |
| 0 1 0 0 0 1 | 0 1 0 0 0 | 0 0 |  |
| 6 | 5 | 3 | 1 1 16 BC1F   offset                    (cc = 0 implied) BC1F   cc, offset if (cc = 0) then branch |

Branch on FP FalseBC1F
MIPS I
condition←COC[1] = tf
GPRLEN-(16+2) 2
target_offset←(offset	)	|| offset || 0
15
if condition then
PC←PC + target
endif
MIPS II and MIPS III:
condition←COC[1] = tf
GPRLEN-(16+2) 2
target_offset←(offset	)	|| offset || 0
15
if condition then
PC←PC + target
else if nd then
NullifyCurrentInstruction()
endif
MIPS IV:
condition←FCC[cc] = tf
GPRLEN-(16+2) 2
target_offset←(offset	)	|| offset || 0
15
if condition then
PC←PC + target
else if nd then
NullifyCurrentInstruction()
endif
±

BC1FL Branch on FP False Likely
| 31 | 26 25 | 21  2018171615 | 0 |
| --- | --- | --- | --- |
| COP1 | BC offset cc | tf |  |
| 0 1 0 0 0 1 | 0 1 0 0 0 | 1 0 |  |
| 6 | 5 | 3 | 1 1 16 BC1FL   offset                    (cc = 0 implied) BC1FL   cc, offset if (cc = 0) then branch_likely |

Branch on FP False LikelyBC1FL
MIPS II and MIPS III:
condition←COC[1] = tf
GPRLEN-(16+2) 2
target_offset←(offset	)	|| offset || 0
15
if condition then
PC←PC + target
else if nd then
NullifyCurrentInstruction()
endif
MIPS IV:
condition←FCC[cc] = tf
GPRLEN-(16+2) 2
target_offset←(offset	)	|| offset || 0
15
if condition then
PC←PC + target
else if nd then
NullifyCurrentInstruction()
endif
±

BC1T Branch on FP True
| 31 | 26 25 | 21  201817 1615 | 0 |
| --- | --- | --- | --- |
| COP1 | BC offset cc | tf |  |
| 0 1 0 0 0 1 | 0 1 0 0 0 | 0 1 |  |
| 6 | 5 | 3 | 1 1 16 BC1T   offset                    (cc = 0 implied) BC1T   cc, offset if (cc = 1) then branch |

Branch on FP TrueBC1T
MIPS I
condition←COC[1] = tf
GPRLEN-(16+2) 2
target←(offset	)	|| offset || 0
15
if condition then
PC←PC + target
endif
MIPS II and MIPS III:
condition←COC[1] = tf
GPRLEN-(16+2) 2
target←(offset	)	|| offset || 0
15
if condition then
PC←PC + target
else if nd then
NullifyCurrentInstruction()
endif
MIPS IV:
condition←FCC[cc] = tf
GPRLEN-(16+2) 2
target←(offset	)	|| offset || 0
15
if condition then
PC←PC + target
else if nd then
NullifyCurrentInstruction()
endif
±

BC1TL Branch on FP True Likely
| 31 | 26 25 | 21  201817 1615 | 0 |
| --- | --- | --- | --- |
| COP1 | BC offset cc | tf |  |
| 0 1 0 0 0 1 | 0 1 0 0 0 | 1 1 |  |
| 6 | 5 | 3 | 1 1 16 BC1TL   offset                    (cc = 0 implied) BC1TL   cc, offset if (cc = 1) then branch_likely |

Branch on FP True LikelyBC1TL
MIPS II and MIPS III:
condition←COC[1] = tf
GPRLEN-(16+2) 2
target←(offset	)	|| offset || 0
15
if condition then
PC←PC + target
else if nd then
NullifyCurrentInstruction()
endif
MIPS IV:
condition←FCC[cc] = tf
GPRLEN-(16+2) 2
target←(offset	)	|| offset || 0
15
if condition then
PC←PC + target
else if nd then
NullifyCurrentInstruction()
endif
±

C.cond.fmt Floating-Point Compare
| 31 | 26 25 | 21 20 | 16 15 | 1110 | 8765 430 0 FC COP1 fmt |
| --- | --- | --- | --- | --- | --- |
| ft | fs | cc | cond 1 1 |  |  |
| 6555 | 3 C.cond.S      fs, ft (cc = 0 implied) C.cond.D      fs, ft (cc = 0 implied) C.cond.S      cc, fs, ft C.cond.D      cc, fs, ft cc←fscompare_condft | 2 | 2 | 4 |  |

Floating-Point CompareC.cond.fmt

C.cond.fmt Floating-Point Compare

Floating-Point CompareC.cond.fmt
if NaN(Value FPR(fs, fmt)) or NaN(ValueFPR(ft, fmt)) then
less←false
equal←false
unordered←true
if t then
SignalException(InvalidOperation)
endif
else
less←ValueFPR(fs, fmt) < ValueFPR(ft, fmt)
equal←ValueFPR(fs, fmt) = ValueFPR(ft, fmt)
unordered←false
endif
condition←(condand less) or (condand equal) or (condand unordered)
2	1	0
FCC[cc]←condition
if cc = 0 then
COC[1]←condition
endif

C.cond.fmt Floating-Point Compare
# comparisons using explicit tests for QNaN
c.eq.d   $f2,$f4  # check for equal
nop
bc1t	L2	# it is equal
c.un.d   $f2,$f4  # it is not equal, but might be unordered
bc1t ERROR# unordered goes off to an error handler
# not-equal-case code here
...
# equal-case code here
L2:
# --------------------------------------------------------------
# comparison using comparisons that signal QNaN
c.seq.d $f2,$f4  # check for equal
nop
bc1t	L2	# it is equal
nop
# it is not unordered here...
# not-equal-case code here
...
#equal-case code here
L2:

Floating-Point Ceiling Convert to Long Fixed-PointCEIL.L.fmt
| 31 | 26  25 | 21   20 | 16   15 | 11   10 | 6   5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| COP1 | fmt | 0 | fs | fd | CEIL.L |  |
| 0 1 0 0 0 1 | 0 0 0 0 0 | 0 0 1 0 1 0 65555 6 CEIL.L.S   fd, fs CEIL.L.D   fd, fs fd←convert_and_round(fs) ∞ StoreFPR(fd, L, ConvertFmt(ValueFPR(fs, fmt), fmt, L)) |  |  |  |  |

CEIL.W.fmt Floating-Point Ceiling Convert to Word Fixed-Point
| 31 | 26  25 | 21   20 | 16   15 | 11   10 | 6   5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| COP1 | fmt | 0 | fs | fd | CEIL.W |  |
| 0 1 0 0 0 1 | 0 0 0 0 0 | 0 0 1 1 1 0 |  |  |  |  |
655556
CEIL.W.S   fd, fs
CEIL.W.D   fd, fs
fd←convert_and_round(fs)
∞
StoreFPR(fd, W, ConvertFmt(ValueFPR(fs, fmt), fmt, W))

Move Control Word from Floating-PointCFC1
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 0 |
| --- | --- | --- | --- | --- | --- |
| COP1 | CF | rt | fs | 0 |  |
| 0 1 0 0 0 1 | 0 0 0 1 0 | 0 0 0  0 0 0 0  0 0 0 0 6555 11 CFC1   rt, fs rt←FP_Control[fs] temp ←FCR[fs] GPR[rt]←sign_extend(temp) temp ←FCR[fs] GPR[rt]←sign_extend(temp) |  |  |  |

CTC1 Move Control Word to Floating-Point
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 0 |
| --- | --- | --- | --- | --- | --- |
| COP1 | CT | rt | fs | 0 |  |
| 0 1 0 0 0 1 | 0 0 1 1 0 | 0 0 0  0 0 0 0  0 0 0 0 6555 11 CTC1   rt, fs FP_Control[fs]←rt temp ←GPR[rt] 31..0 FCR[fs]←temp COC[1]←FCR[31] |  |  |  |
23
temp ←GPR[rt]
31..0
FCR[fs]←temp
COC[1]←FCR[31]
23

Floating-Point Convert to Double Floating-PointCVT.D.fmt
| 31 | 26  25 | 21   20 | 16   15 | 11   10 | 6   5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| COP1 | fmt | 0 | fs | fd | CVT.D |  |
| 0 1 0 0 0 1 | 0 0 0 0 0 | 1 0 0 0 0 1 65555 6 CVT.D.S fd, fs CVT.D.W fd, fs CVT.D.L fd, fs fd←convert_and_round(fs) StoreFPR (fd, D, ConvertFmt(ValueFPR(fs, fmt), fmt, D)) |  |  |  |  |

CVT.L.fmt Floating-Point Convert to Long Fixed-Point
| 31 | 26  25 | 21   20 | 16   15 | 11   10 | 6   5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| COP1 | fmt | 0 | fs | fd | CVT.L |  |
| 0 1 0 0 0 1 | 0 0 0 0 0 | 1 0 0 1 0 1 65555 6 CVT.L.S   fd, fs CVT.L.D   fd, fs fd←convert_and_round(fs) StoreFPR (fd, L, ConvertFmt(ValueFPR(fs, fmt), fmt, L)) |  |  |  |  |

Floating-Point Convert to Single Floating-PointCVT.S.fmt
| 31 | 26  25 | 21   20 | 16   15 | 11   10 | 6   5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| COP1 | fmt | 0 | fs | fd | CVT.S |  |
| 0 1 0 0 0 1 | 0 0 0 0 0 | 1 0 0 0 0 0 65555 6 CVT.S.D fd, fs CVT.S.W fd, fs CVT.S.L fd, fs fd←convert_and_round(fs) StoreFPR(fd, S, ConvertFmt(ValueFPR(fs, fmt), fmt, S)) |  |  |  |  |

CVT.W.fmt Floating-Point Convert to Word Fixed-Point
| 31 | 26  25 | 21   20 | 16   15 | 11   10 | 6   5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| COP1 | fmt | 0 | fs | fd | CVT.W |  |
| 0 1 0 0 0 1 | 0 0 0 0 0 | 1 0 0 1 0 0 |  |  |  |  |
655556
CVT.W.S   fd, fs
CVT.W.D   fd, fs
fd←convert_and_round(fs)
StoreFPR(fd, W, ConvertFmt(ValueFPR(fs, fmt), fmt, W))

Floating-Point DivideDIV.fmt
31	26  25	21   20	16   15	11   10	6   5	0
COP1	fmt	ft	fs	fd	DIV
0 1 0 0 0 1 0 0 0 0 1 1
655556
DIV.S   fd, fs, ft
DIV.D   fd, fs, ft
fd←fs / ft
StoreFPR (fd, fmt, ValueFPR(fs, fmt) / ValueFPR(ft, fmt))

DMFC1 Doubleword Move From Floating-Point
| 31 | 26 25 | 21 20 | 16 15 | 11 10 | 0 |
| --- | --- | --- | --- | --- | --- |
| COP1 | DMF | rt | fs | 0 |  |
| 0 1 0 0 0 1 | 0 0 0 0 1 | 0 0 0   0 0 0 0  0 0 00 6555 11 DMFC1   rt, fs rt←fs if SizeFGR() = 64 then /* 64-bit wide FGRs */ data←FGR[fs] elseif fs= 0 then /* valid specifier, 32-bit wide FGRs */ |  |  |  |
0
data←FGR[fs+1] || FGR[fs]
else /* undefined for odd 32-bit FGRs */
UndefinedResult()
endif
GPR[rt]←data
if SizeFGR() = 64 then /* 64-bit wide FGRs */
data←FGR[fs]
elseif fs= 0 then /* valid specifier, 32-bit wide FGRs */
0
data←FGR[fs+1] || FGR[fs]
else /* undefined for odd 32-bit FGRs */
UndefinedResult()
endif
GPR[rt]←data

Doubleword Move To Floating-PointDMTC1
| 31 | 26 25 | 21 20 | 16 15 | 11 10 | 0 |
| --- | --- | --- | --- | --- | --- |
| COP1 | DMT | rt | fs | 0 |  |
| 0 1 0 0 0 1 | 0 0 1 0 1 | 0 0 0   0 0 0 0  0 0 00 6555 11 DMTC1   rt, fs fs←rt data←GPR[rt] if SizeFGR() = 64 then /* 64-bit wide FGRs */ FGR[fs]←data elseif fs= 0 then /* valid specifier, 32-bit wide FGRs */ |  |  |  |
0
FGR[fs+1]←data
63..32
FGR[fs]←data
31..0
else /* undefined result for odd 32-bit FGRs */
UndefinedResult()
endif
data←GPR[rt]
if SizeFGR() = 64 then /* 64-bit wide FGRs */
FGR[fs]←data
elseif fs= 0 then /* valid specifier, 32-bit wide FGRs */
0
FGR[fs+1]←data
63..32
FGR[fs]←data
31..0
else /* undefined result for odd 32-bit FGRs */
UndefinedResult()
endif

DMTC1 Doubleword Move To Floating-Point

Floating-Point Floor Convert to Long Fixed-PointFLOOR.L.fmt
| 31 | 26  25 | 21   20 | 16   15 | 11   10 | 6   5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| COP1 | fmt | 0 | fs | fd | FLOOR.L |  |
| 0 1 0 0 0 1 | 0 0 0 0 0 | 0 0 1 0 1 1 65555 6 FLOOR.L.S   fd, fs FLOOR.L.D   fd, fs fd←convert_and_round(fs) ∞ StoreFPR(fd, L, ConvertFmt(ValueFPR(fs, fmt), fmt, L)) |  |  |  |  |

FLOOR.W.fmt Floating-Point Floor Convert to Word Fixed-Point
| 31 | 26  25 | 21   20 | 16   15 | 11   10 | 6   5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| COP1 | fmt | 0 | fs | fd | FLOOR.W |  |
| 0 1 0 0 0 1 | 0 0 0 0 0 | 0 0 1 1 1 1 |  |  |  |  |
655556
FLOOR.W.S   fd, fs
FLOOR.W.D   fd, fs
fd←convert_and_round(fs)
∞
StoreFPR(fd, W, ConvertFmt(ValueFPR(fs, fmt), fmt, W))

Load Doubleword to Floating-PointLDC1
31	26 25	21 20	16 15	0
LDC1	base	ft	offset
1 1 0 1 0 1
655 16
LDC1   ft, offset(base)
ft←memory[base+offset]
≠
vAddr←sign_extend(offset) + GPR[base]
3
if vAddr ≠0then SignalException(AddressError) endif
2..0
(pAddr, uncached)←AddressTranslation (vAddr, DATA, LOAD)
data←LoadMemory(uncached, DOUBLEWORD, pAddr, vAddr, DATA)
if SizeFGR() = 64 then /* 64-bit wide FGRs */
FGR[ft]←data
elseif ft= 0 then /* valid specifier, 32-bit wide FGRs */
0
FGR[ft+1]←data
63..32
FGR[ft]←data
31..0
else /* undefined result for odd 32-bit FGRs */
UndefinedResult()
endif

LDXC1 Load Doubleword Indexed to Floating-Point
31	26 25	21 20	16 15	11  10	6    5	0
COP1X	base	index	0fdLDXC1
0 1 0 0 1 1 0 0 0 0 0 1
65555 6
LDXC1   fd, index(base)
fd←memory[base+index]
≠
≠
vAddr←GPR[base] + GPR[index]
3
if vAddr ≠0then SignalException(AddressError) endif
2..0
(pAddr, uncached)←AddressTranslation (vAddr, DATA, LOAD)
mem←LoadMemory(unchched, DOUBLEWORD, pAddr, vAddr, DATA)
if SizeFGR() = 64 then /* 64-bit wide FGRs */
FGR[fd]←data
elseif fd= 0 then /* valid specifier, 32-bit wide FGRs */
0
FGR[fd+1]←data
63..32
FGR[fd]←data
31..0
else /* undefined result for odd 32-bit FGRs */
UndefinedResult()
endif

Load Doubleword Indexed to Floating-PointLDXC1

LWC1 Load Word to Floating-Point
| 31 | 26 25 | 21 20 | 16 15 | 0 |
| --- | --- | --- | --- | --- |
| LWC1 | base | ft | offset 1 1 0 0 0 1 |  |
| 6 | 55 LWC1  ft, offset(base) ft←memory[base+offset] ≠ /* “mem” is aligned 64-bits from memory.  Pick out correct bytes. */ vAddr←sign_extend(offset) GPR[base] | 16 |  |  |
2
if vAddr ≠0then SignalException(AddressError) endif
1..0
(pAddr, uncached)←AddressTranslation (vAddr, DATA, LOAD)
mem←LoadMemory(uncached, WORD, pAddr, vAddr, DATA)
FGR[ft]←mem
/* “mem” is aligned 64-bits from memory.  Pick out correct bytes. */
vAddr←sign_extend(offset) GPR[base]
2
if vAddr ≠0then SignalException(AddressError) endif
1..0
(pAddr, uncached)←AddressTranslation (vAddr, DATA, LOAD)
2
pAddr←pAddr	|| (pAddr	xor (ReverseEndian || 0))
PSIZE-1..3 2..0
mem←LoadMemory(uncached, WORD, pAddr, vAddr, DATA)
2
bytesel←vAddr xor (BigEndianCPU || 0)
2..0
if SizeFGR() = 64 then /* 64-bit wide FGRs */
32
FGR[ft]←undefined || mem
31+8*bytesel..8*bytesel
else /* 32-bit wide FGRs */
FGR[ft]←mem
31+8*bytesel..8*bytesel
endif

Load Word to Floating-PointLWC1

LWXC1 Load Word Indexed to Floating-Point
31	26 25	21 20	16 15	11  10	6    5	0
COP1X	base	index	0fdLWXC1
0 1 0 0 1 1 0 0 0 0 0 0
65555  6
LWXC1   fd, index(base)
fd←memory[base+index]
≠
≠
vAddr←GPR[base] + GPR[index]
2
if vAddr ≠0then SignalException(AddressError) endif
1..0
(pAddr, uncached)←AddressTranslation (vAddr, DATA, LOAD)
2
pAddr←pAddr	|| (pAddr	xor (ReverseEndian || 0))
PSIZE-1..3 2..0
/* “mem” is aligned 64-bits from memory.  Pick out correct bytes. */
mem←LoadMemory(uncached, WORD, pAddr, vAddr, DATA)
2
bytesel←vAddr xor (BigEndianCPU || 0)
2..0
if SizeFGR() = 64 then /* 64-bit wide FGRs */
32
FGR[fd]←undefined || mem
31+8*bytesel..8*bytesel
else /* 32-bit wide FGRs */
FGR[fd]←mem
31+8*bytesel..8*bytesel
endif

Floating-Point Multiply AddMADD.fmt
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 6 5 | 32 | 0 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| COP1X | MADD | fmt |  |  |  |  |  |
| fr | ft 0 1 0 0 1 1 1 0 0 | fs | fd |  |  |  |  |
6555533
MADD.S   fd, fr, fs, ft
MADD.D   fd, fr, fs, ft
fd←(fs×ft) + fr
vfr ←ValueFPR(fr, fmt)
vfs ←ValueFPR(fs, fmt)
vft ←ValueFPR(ft, fmt)
StoreFPR(fd, fmt, vfr + vfs * vft)

MFC1 Move Word From Floating-Point
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 0 |
| --- | --- | --- | --- | --- | --- |
| COP1 | MF | rt | fs | 0 |  |
| 0 1 0 0 0 1 | 0 0 0 0 0 | 0 0 0  0 0 0 0  0 0 0 0 6555 11 MFC1   rt, fs rt←fs word ←FGR[fs] 31..0 GPR[rt]←sign_extend(word) word ←FGR[fs] 31..0 GPR[rt]←sign_extend(word) |  |  |  |

Floating-Point MoveMOV.fmt
| 31 | 26  25 | 21   20 | 16   15 | 11   10 | 6   5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| COP1 | fmt | 0 | fs | fd | MOV |  |
| 0 1 0 0 0 1 | 0 0 0 0 0 | 0 0 0 1 1 0 |  |  |  |  |
655556
MOV.S   fd, fs
MOV.D   fd, fs
fd←fs
StoreFPR(fd, fmt, ValueFPR(fs, fmt))

MOVF Move Conditional on FP False
| 31 | 2625 | 21 20  18  171615 | 11 10 | 65 | 0 |
| --- | --- | --- | --- | --- | --- |
| SPECIAL | 0 | tf | 0 | MOVCI |  |
| rs | cc 0 0 0 0 0 0 0 0 0 0 0 1 | rd |  |  |  |
| 0 | 0 | 0 0 0 0 0 |  |  |  |
| 65 | 5113 MOVF     rd, rs, cc if (cc = 0) then rd←rs active←FCC[cc]=tf if active then GPR[rd]←GPR[rs] endif | 5 | 5 | 6 |  |

Floating-Point Move Conditional on FP FalseMOVF.fmt
| 31 | 2625 | 21 20  18  171615 | 11 10 | 65 | 0 |
| --- | --- | --- | --- | --- | --- |
| COP1 | 0 | tf | MOVCF |  |  |
| fmt | cc 0 1 0 0 0 1 0 1 0 0 0 1 0 0 | fs | fd |  |  |
| 655115 | 3 MOVF.S     fd, fs, cc MOVF.D     fd, fs, cc if (cc = 0) then fd←fs if FCC[cc]=tf then StoreFPR(fd, fmt, ValueFPR(fs, fmt)) else StoreFPR(fd, fmt, ValueFPR(fd, fmt)) endif | 5 | 5 | 6 |  |

MOVN.fmt Floating-Point Move Conditional on Not Zero
| 31 | 2625 | 21 20 COP1 MOVN | 1615 | 11 10 | 65 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| fmt | rt 0 1 0 0 0 1 0 1 0 0 1 1 | fs | fd |  |  |  |
| 655 | 55 MOVN.S     fd, fs, rt MOVN.D     fd, fs, rt if (rt≠0) then fd←fs if GPR[rt]≠0 then StoreFPR(fd, fmt, ValueFPR(fs, fmt)) else StoreFPR(fd, fmt, ValueFPR(fd, fmt)) endif | 6 |  |  |  |  |

Move Conditional on FP TrueMOVT
| 31 | 2625 | 21 20  18  171615 | 11 10 | 65 | 0 |
| --- | --- | --- | --- | --- | --- |
| SPECIAL | 0 | tf | 0 | MOVCI |  |
| rs | cc 0 0 0 0 0 0 0 0 0 0 0 1 | rd |  |  |  |
| 0 | 1 | 0 0 0 0 0 |  |  |  |
| 6 | 5113 MOVT     rd, rs, cc if (cc = 1) then rd←rs if FCC[cc]=tf then GPR[rd]←GPR[rs] endif | 5 | 5 | 6 |  |

MOVT.fmt Floating-Point Move Conditional on FP True
| 31 | 2625 | 21 20  18  171615 | 11 10 | 65 | 0 |
| --- | --- | --- | --- | --- | --- |
| COP1 | 0 | tf | MOVCF |  |  |
| fmt | cc 0 1 0 0 0 1 0 1 0 0 0 1 0 1 | fs | fd |  |  |
| 6 | 5113 MOVT.S   fd, fs, cc MOVT.D   fd, fs, cc if (cc = 1) then fd←fs if FCC[cc]=tf then StoreFPR(fd, fmt, ValueFPR(fs, fmt)) else StoreFPR(fd, fmt, ValueFPR(fd, fmt)) endif | 5 | 5 | 6 |  |

Floating-Point Move Conditional on ZeroMOVZ.fmt
| 31 | 2625 | 21 20 COP1 MOVZ | 1615 | 11 10 | 65 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| fmt | rt 0 1 0 0 0 1 0 1 0 0 1 0 | fs | fd |  |  |  |
| 655 | 55 MOVZ.S   fd, fs, rt MOVZ.D   fd, fs, rt if (rt = 0) then fd←fs if GPR[rt]=0 then StoreFPR(fd, fmt, ValueFPR(fs, fmt)) else StoreFPR(fd, fmt, ValueFPR(fd, fmt)) endif | 6 |  |  |  |  |

MSUB.fmt Floating-Point Multiply Subtract
31	26 25	21 20	16 15	11  10	6 5	32	0
COP1X	fr	ft	fs	fd	MSUB	fmt
0 1 0 0 1 1 1 0 1
6555533
MSUB.S   fd, fr, fs, ft
MSUB.D   fd, fr, fs, ft
fd←(fs×ft) - fr
vfr ←ValueFPR(fr, fmt)
vfs ←ValueFPR(fs, fmt)
vft ←ValueFPR(ft, fmt)
StoreFPR(fd, fmt, (vfs * vft) - vfr)

Move Word to Floating-PointMTC1
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 0 |
| --- | --- | --- | --- | --- | --- |
| COP1 | MT | rt | fs | 0 |  |
| 0 1 0 0 0 1 | 0 0 1 0 0 | 0 0 0  0 0 0 0  0 0 0 0 6555 11 MTC1   rt, fs fs←rt data←GPR[rt] 31..0 if SizeFGR() = 64 then /* 64-bit wide FGRs */ |  |  |  |
32
FGR[fs]←undefined || data
else /* 32-bit wide FGRs */
FGR[fs]←data
endif
data←GPR[rt]
31..0
if SizeFGR() = 64 then /* 64-bit wide FGRs */
32
FGR[fs]←undefined || data
else /* 32-bit wide FGRs */
FGR[fs]←data
endif

MUL.fmt Floating-Point Multiply
31	26  25	21   20	16   15	11   10	6   5	0
COP1	fmt	ft	fs	fd	MUL
0 1 0 0 0 1 0 0 0 0 1 0
655556
MUL.S   fd, fs, ft
MUL.D   fd, fs, ft
fd←fs×ft
StoreFPR (fd, fmt, ValueFPR(fs, fmt) * ValueFPR(ft, fmt))

Floating-Point NegateNEG.fmt
| 31 | 26  25 | 21   20 | 16   15 | 11   10 | 6   5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| COP1 | fmt | 0 | fs | fd | NEG |  |
| 0 1 0 0 0 1 | 0 0 0 0 0 | 0 0 0 1 1 1 |  |  |  |  |
655556
NEG.S   fd, fs
NEG.D   fd, fs
fd←- (fs)
StoreFPR(fd, fmt, Negate(ValueFPR(fs, fmt)))

NMADD.fmt Floating-Point Negative Multiply Add
31	26 25	21 20	16 15	11  10	6 5	32	0
COP1X	fr	ft	fs	fd	NMADD	fmt
0 1 0 0 1 1 1 1 0
6555533
NMADD.S   fd, fr, fs, ft
NMADD.D   fd, fr, fs, ft
fd←- ((fs×ft) + fr)
vfr ←ValueFPR(fr, fmt)
vfs ←ValueFPR(fs, fmt)
vft ←ValueFPR(ft, fmt)
StoreFPR(fd, fmt, -(vfr + vfs * vft))

Floating-Point Negative Multiply SubtractNMSUB.fmt
31	26 25	21 20	16 15	11  10	6 5	32	0
COP1X	fr	ft	fs	fd	NMSUB	fmt
0 1 0 0 1 1 1 1 1
6555533
NMSUB.S   fd, fr, fs, ft
NMSUB.D   fd, fr, fs, ft
fd←- ((fs×ft) - fr)
vfr ←ValueFPR(fr, fmt)
vfs ←ValueFPR(fs, fmt)
vft ←ValueFPR(ft, fmt)
StoreFPR(fd, fmt, -((vfs * vft) - vfr))

PREFX Prefetch Indexed
31	26 25	21 20	16 15	11  10	6    5	0
0
COP1X PREFX
base	index	hint
0 1 0 0 1 1	0 0 0 0 0	0 0 1 1 1 1
65555  6
PREFX   hint, index(base)
prefetch_memory[base+index]

Prefetch IndexedPREFX
≠
vAddr←GPR[base] + GPR[index]
(pAddr, uncached)←AddressTranslation(vAddr, DATA, LOAD)
Prefetch(uncached, pAddr, vAddr, DATA, hint)

PREFX Prefetch Indexed

Reciprocal ApproximationRECIP.fmt
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 6    5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| COP1 | 0 | RECIP |  |  |  |  |
| fmt | fs | fd |  |  |  |  |
| 0 1 0 0 0 1 | 0 0 0 0 0 | 0 1 0 1 0 1 65555  6 RECIP.S   fd, fs RECIP.D   fd, fs fd←1.0 / fs StoreFPR(fd, fmt, 1.0 / valueFPR(fs, fmt)) |  |  |  |  |

ROUND.L.fmt Floating-Point Round to Long Fixed-Point
| 31 | 26  25 | 21   20 | 16   15 | 11   10 | 6   5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| COP1 | fmt | 0 | fs | fd | ROUND.L |  |
| 0 1 0 0 0 1 | 0 0 0 0 0 | 0 0 1 0 0 0 65555 6 ROUND.L.S   fd, fs ROUND.L.D   fd, fs fd←convert_and_round(fs) StoreFPR(fd, L, ConvertFmt(ValueFPR(fs, fmt), fmt, L)) |  |  |  |  |

Floating-Point Round to Word Fixed-PointROUND.W.fmt
| 31 | 26  25 | 21   20 | 16   15 | 11   10 | 6   5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| COP1 | fmt | 0 | fs | fd | ROUND.W |  |
| 0 1 0 0 0 1 | 0 0 0 0 0 | 0 0 1 1 0 0 |  |  |  |  |
655556
ROUND.W.S   fd, fs
ROUND.W.D   fd, fs
fd←convert_and_round(fs)
StoreFPR(fd, W, ConvertFmt(ValueFPR(fs, fmt), fmt, W))

RSQRT.fmt Reciprocal Square Root Approximation
| 31 | 26 25 | 21 20 | 16 15 | 11  10 | 6    5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| COP1 | 0 | RSQRT |  |  |  |  |
| fmt | fs | fd |  |  |  |  |
| 0 1 0 0 0 1 | 0 0 0 0 0 | 0 1 0 1 1 0 65555  6 RSQRT.S   fd, fs RSQRT.D   fd, fs fd←1.0 / sqrt(fs) StoreFPR(fd, fmt, 1.0 / SquareRoot(valueFPR(fs, fmt))) |  |  |  |  |

Store Doubleword from Floating-PointSDC1
31	26 25	21 20	16 15	0
SDC1	base	ft	offset
1 1 1 1 0 1
655 16
SDC1   ft, offset(base)
memory[base+offset]←ft
≠
vAddr←sign_extend(offset) + GPR[base]
3
if vAddr ≠0then SignalException(AddressError) endif
2..0
(pAddr, uncached)←AddressTranslation(vAddr, DATA, STORE)
if SizeFGR() = 64 then /* 64-bit wide FGRs */
data←FGR[ft]
elseif ft= 0 then /* valid specifier, 32-bit wide FGRs */
0
data←FGR[ft+1] || FGR[ft]
else /* undefined for odd 32-bit FGRs */
UndefinedResult()
endif
StoreMemory(uncached, DOUBLEWORD, data, pAddr, vAddr, DATA)

SDXC1 Store Doubleword Indexed from Floating-Point
31	26 25	21 20	16 15	11  10	6    5	0
COP1X	base	index	fs	0	SDXC1
0 1 0 0 1 1 0 0 1 0 0 1
65555  6
SDXC1   fs, index(base)
memory[base+index]←fs
≠
≠
vAddr←GPR[base] + GPR[index]
3
if vAddr ≠0then SignalException(AddressError) endif
2..0
(pAddr, uncached)←AddressTranslation(vAddr, DATA, STORE)
if SizeFGR() = 64 then /* 64-bit wide FGRs */
data←FGR[fs]
elseif fs= 0 then /* valid specifier, 32-bit wide FGRs */
0
data←FGR[fs+1] || FGR[fs]
else /* undefined for odd 32-bit FGRs */
UndefinedResult()
endif
StoreMemory(uncached, DOUBLEWORD, data, pAddr, vAddr, DATA)

Store Doubleword Indexed from Floating-PointSDXC1

SQRT.fmt Floating-Point Square Root
| 31 | 26  25 | 21   20 | 16   15 | 11   10 | 6   5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| COP1 | fmt | 0 | fs | fd | SQRT |  |
| 0 1 0 0 0 1 | 0 0 0 0 0 | 0 0 0 1 0 0 |  |  |  |  |
655556
SQRT.S   fd, fs
SQRT.D   fd, fs
fd←SQRT(fs)
StoreFPR(fd, fmt, SquareRoot(ValueFPR(fs, fmt)))

Floating-Point SubtractSUB.fmt
31	26  25	21   20	16   15	11   10	6   5	0
COP1	fmt	ft	fs	fd	SUB
0 1 0 0 0 1 0 0 0 0 0 1
655556
SUB.S   fd, fs, ft
SUB.D   fd, fs, ft
fd←fs - ft
StoreFPR (fd, fmt, ValueFPR(fs, fmt) – ValueFPR(ft, fmt))

SWC1 Store Word from Floating-Point
31	26 25	21 20	16 15	0
SWC1	base	ft	offset
1 1 1 0 0 1
655 16
SWC1   ft, offset(base)
memory[base+offset]←ft
≠
vAddr←sign_extend(offset) + GPR[base]
2
if vAddr ≠0then SignalException(AddressError) endif
1..0
(pAddr, uncached)←AddressTranslation (vAddr, DATA, STORE)
data←FGR[ft]
StoreMemory (uncached, WORD, data, pAddr, vAddr, DATA)
vAddr←sign_extend(offset) + GPR[base]
2
if vAddr ≠0then SignalException(AddressError) endif
1..0
(pAddr, uncached)←AddressTranslation (vAddr, DATA, STORE)
2
pAddr←pAddr	(pAddr	xor (ReverseEndian || 0))
PSIZE-1..3 2..0
2
bytesel←vAddr xor (BigEndianCPU || 0)
2..0
/* the bytes of the word are moved into the correct byte lanes */
if SizeFGR() = 64 then /* 64-bit wide FGRs */
32-8*bytesel 8*bytesel
| data←0 | \|\| FGR[ft] | \|\| 0 | /* top or bottom wd of 64-bit data */ 31..0 else /* 32-bit wide FGRs */ 32-8*bytesel 8*bytesel |
| --- | --- | --- | --- |
| data←0 | \|\| FGR[ft] \|\| 0 | /* top or bottom wd of 64-bit data */ endif StoreMemory (uncached, WORD, data, pAddr, vAddr, DATA) |  |

Store Word Indexed from Floating-PointSWXC1
31	26 25	21 20	16 15	11  10	6    5	0
COP1X	base	index	fs	0	SWXC1
0 1 0 0 1 1 0 0 1 0 0 0
65555  6
SWXC1   fs, index(base)
memory[base+index]←fs
≠
≠
vAddr←GPR[base] + GPR[index]
2
if vAddr ≠0then SignalException(AddressError) endif
1..0
(pAddr, uncached)←AddressTranslation(vAddr, DATA, STORE)
2
pAddr←pAddr	|| (pAddr	xor (ReverseEndian || 0))
PSIZE-1..3 2..0
2
bytesel←vAddr xor (BigEndianCPU || 0)
2..0
/* the bytes of the word are moved into the correct byte lanes */
if SizeFGR() = 64 then /* 64-bit wide FGRs */
32-8*bytesel 8*bytesel
| data←0 | \|\| FGR[fs] | \|\| 0 | /* top or bottom wd of 64-bit data */ 31..0 else /* 32-bit wide FGRs */ 32-8*bytesel 8*bytesel |
| --- | --- | --- | --- |
| data←0 | \|\| FGR[fs] \|\| 0 | /* top or bottom wd of 64-bit data */ endif StoreMemory (uncached, WORD, data, pAddr, vAddr, DATA) |  |

TRUNC.L.fmt Floating-Point Truncate to Long Fixed-Point
| 31 | 26  25 | 21   20 | 16   15 | 11   10 | 6   5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| COP1 | fmt | 0 | fs | fd | TRUNC.L |  |
| 0 1 0 0 0 1 | 0 0 0 0 0 | 0 0 1 0 01 65555 6 TRUNC.L.S   fd, fs TRUNC.L.D   fd, fs fd←convert_and_round(fs) StoreFPR(fd, L, ConvertFmt(ValueFPR(fs, fmt), fmt, L)) |  |  |  |  |

Floating-Point Truncate to Word Fixed-PointTRUNC.W.fmt
| 31 | 26  25 | 21   20 | 16   15 | 11   10 | 6   5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| COP1 | fmt | 0 | fs | fd | TRUNC.W |  |
| 0 1 0 0 0 1 | 0 0 0 0 0 | 0 0 1 1 0 1 |  |  |  |  |
655556
TRUNC.W.S   fd, fs
TRUNC.W.D   fd, fs
fd←convert_and_round(fs)
StoreFPR(fd, W, ConvertFmt(ValueFPR(fs, fmt), fmt, W))

| 31 | 26 | 25 | 21   20 | 16 | 15 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| opcode | base | ft | offset 655 16 Register:  2-register and 3-register formatted arithmetic operations. |  |  |  |
| 31 | 26  25 | 21   20 | 16   15 | 11   10 | 6 | 5 0 |
| COP1 | fmt | ft | fs | fd | function |  |
655556
Register Immediate:  data transfer --  CPU↔FPU register.
| 31 | 26  25 | 21   20 | 16   15 | 11   10 | 0 |
| --- | --- | --- | --- | --- | --- |
| COP1 | sub 6555 11 | rt | fs | 0 |  |
| 31 | 26 25 | 21  2018 offset | 15 | 0 |  |
| COP1 | BC | cc | tf |  |  |
| 6 | 5 | 3 | 1 1 Register to Condition Code:  formatted FP compare. | 16 |  |
| 31 | 26  25 | 21   20 | 16   15 | 11   10 | 87 6  5 0 |
| COP1 | fmt | ft | fs | cc | 0 function 655532 4 |

Condition Code, Register FP:   FPU register move-conditional on FP cc.
| 31 | 26  25 | 21   20 1817 16 15 | 11   10 | 6 | 5 | 0 |
| --- | --- | --- | --- | --- | --- | --- |
| COP1 | fmt | cc | 0tf | fs | fd | MOVCF 655 5561 1 Register-4:  4-register formatted arithmetic operations. |
| 31 | 26  25 | 21   20 function | 16   15 | 11   10 | 6 | 5 3 2 0 |
| COP1X | fr op4 fmt3 | ft | fs | fd |  |  |
6555533
Register Index:  Load/store using register + register addressing.
31	26  25	21   20	16   15	11   10	6	5	0
COP1X	base	index	0	fd	function
655556
Register Index hint:  Prefetch using register + register addressing.
31	26  25	21   20	16   15	11   10	6	5	0
COP1X	base	index	hint	0	PREFX
655556
Condition Code, Register Integer:  CPU register move-conditional on FP cc.
31	26  25	21   20 1817 16 15	11   10	6	5	0
SPECIAL	rs	cc	0tf	rd	0	MOVCI
655 5561 1

↔

| 31 | 26 opcode δ χ | 0 |
| --- | --- | --- |
| 31 | 2625 opcode fmt δ | 21 0 |
| δ | δ | δ |
| 31 | 2625 | 21 16 0 |
| opcode | fmt f | t |

| 31 | 2625 opcode fmt function | 21 | 0 |
| --- | --- | --- | --- |
| α | α | α | α α α α α |
| α | α | α | α α α α α |
| 31 | 2625 opcode fmt function | 21 | 0 |
| α | α | α | α α α α α |
| α | α | α | α α α α α |

31	2625	21	0
opcode fmt
function

| 31 | 26 opcode δ χ | 0 |
| --- | --- | --- |
| 31 | 2625 opcode fmt δ | 21 0 |
| δ | δ | δ |
| 31 | 2625 | 21 17  16 0 |
| opcode | fmt df | nt |

| 31 | 2625 opcode fmt function | 21 | 0 |
| --- | --- | --- | --- |
| α | α | α | α α α α α |
| α | α | α | α α α α α |
| 31 | 2625 opcode fmt function | 21 | 0 |
| α | α | α | α α α α α |
| α | α | α | α α α α α |

31	2625	21	0
opcode fmt
function

| 31 | 26 opcode δ χ | 0 |
| --- | --- | --- |
| 31 | 2625 opcode fmt δ | 21 0 |
| δ | δ | δ δ |
| 31 | 2625 | 21 17  16 0 |
| opcode | fmt df | nt |

| 31 | 2625 opcode fmt function | 21 | 0 |
| --- | --- | --- | --- |
| α | α | α | α α α α α |
| α | α | α | α α α α α |
| 31 | 2625 opcode fmt function | 21 | 0 |
| α | α | α | α α α α α |
| α | α | α | α α α α α |

31	2625	21	0
opcode fmt
function

| 31 | 26 opcode δ, β δ δ,λ χ | 0 |
| --- | --- | --- |
| 31 | 2625 opcode fmt δ | 21 0 |
| δ | δ | δ δ |
| 31 | 2625 | 21 17  16 0 |
| opcode | fmt df | nt |

| 31 | 2625 opcode fmt function δ | 21 | 0 |
| --- | --- | --- | --- |
| α | α | α | α α α α α |
| α | α | α | α α α α α |
| 31 | 2625 opcode fmt function δ | 21 | 0 |
| α | α | α | α α α α α |
| α | α | α | α α α α α |

| 31 | 2625 opcode fmt function | 21 | 0 |
| --- | --- | --- | --- |
| 31 | 26 opcode function | 50 |  |
| 31 | 2625 | 21 | 16 50 |
| opcode | fmt | t | function = f |

| 31 | 26 opcode function = δ χ | 50 |
| --- | --- | --- |
| 31 | 26 | 16 50 |
| opcode | t | function = f |

| 31 | 26 opcode β4 1,2,3,4 χ | 0 |
| --- | --- | --- |
| 31 | 2625 opcode fmt | 21 0 |
| 1,2,3,4 | 1,2,3,4 | 1,2,3,4 3,4 |
| 31 | 2625 | 21 17  16 0 |
| opcode | fmt df | nt |

31	2625	21	0
opcode fmt
function
31	2625	21	0
opcode fmt
function

| 31 | 2625 opcode fmt function | 21 | 0 |
| --- | --- | --- | --- |
| 31 | 26 opcode function | 50 |  |
| 31 | 2625 | 21 | 16 50 |
| opcode | fmt | t | function f |

| 31 | 26 opcode function χ | 50 |
| --- | --- | --- |
| 31 | 26 | 16 50 |
| opcode | t | function f |

| 31 | 26 opcode | 0 |
| --- | --- | --- |
| 31 | 2625 opcode fmt | 21 0 |
| 31 | 2625 | 21 17  16 0 |
| opcode | fmt df | nt |

31	2625	21	0
opcode fmt
function
31	2625	21	0
opcode fmt
function

31	2625	21	0
opcode fmt
function

| 31 | 26 opcode | 0 |
| --- | --- | --- |
| 31 | 2625 opcode fmt δ | 21 0 |
| 31 | 2625 | 21 17  16 0 |
| opcode | fmt df | nt |

31	2625	21	0
opcode fmt
function
31	2625	21	0
opcode fmt
function

31	2625	21	0
opcode fmt
function

| 31 | 26 opcode δ | 0 |
| --- | --- | --- |
| 31 | 2625 opcode fmt | 21 0 |
| 31 | 2625 | 21 17  16 0 |
| opcode | fmt df | nt |

| 31 | 2625 opcode fmt function δ | 21 | 0 |
| --- | --- | --- | --- |
| 31 | 2625 opcode fmt function δ | 21 | 0 |
| 31 | 2625 opcode fmt function | 21 | 0 |

| 31 | 26 opcode function | 50 |
| --- | --- | --- |
| 31 | 2625 | 21 16 50 |
| opcode | fmt | t function f |
| 31 | 26 opcode function δ χ | 50 |

31	26	16	50
opcode	t	function
f

α
β
δ
λ
χ
( )
