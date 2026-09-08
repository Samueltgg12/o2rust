/* Section: sloader */
/* Version: 1.0 */
/* Type: SECTION_TYPE_CODE */
/* Length: 0x4000 (16384 bytes) */

#include "definitions.h"
#include "macros.inc"

.set noreorder
.set noat


/* Function F_0xbfc00000 [0xbfc00000 - 0xbfc0073c)
 */
F_0xbfc00000: /* 0xbfc00000 */
	b	L_0xbfc00048
	 nop

	section_start sloader
	shdr sloader, 1.0, SECTION_TYPE_CODE

	sentinel 0x00000000

L_0xbfc00048: /* 0xbfc00048 */
	b	start_me_up
	 nop

	.org	0x100

/* Function exception_uncached_exception [0xbfc00100 - 0xbfc0073c)
 */
exception_uncached_exception: /* 0xbfc00100 */
	mtc0	$a0, $CP0_LLADDR
	lui	$t8, HI(BASE_CRIME + CRIME_REPLACEMENT_CHECK_BITS)
	sd	$a0, LO(BASE_CRIME + CRIME_REPLACEMENT_CHECK_BITS)($t8)
	lui	$t8, HI(BASE_CRIME + CRIME_MC_STATUS_CTRL)
	addiu	$t8, $t8, LO(BASE_CRIME + CRIME_MC_STATUS_CTRL)
	b	L_0xbfc004ec
	 sd	$zero, 0($t8)		# CRIME_MC_STATUS_CTRL; disable ECC

	.org	0x200

/* Function exception_TLB_refill [0xbfc00200 - 0xbfc003a8)
 */
exception_TLB_refill: /* 0xbfc00200 */
	beqz	$fp, crash
	 nop
	jr	$fp
	 li	$a0, 0
flash_check: /* 0xbfc00210 */
	.int	0xffffffff
	.int	0x00000000
	.int	0x55555555
	.int	0xaaaaaaaa

	.org	0x280

/* Function exception_XTLB_refill [0xbfc00280 - 0xbfc003a8)
 */
exception_XTLB_refill: /* 0xbfc00280 */
	beqz	$fp, crash
	 nop
	jr	$fp
	 li	$a0, 1

	.org	0x300

/* Function exception_cache_error [0xbfc00300 - 0xbfc003a8)
 */
exception_cache_error: /* 0xbfc00300 */
	beqz	$fp, crash
	 nop
	jr	$fp
	 li	$a0, 2

	.org	0x380

/* Function exception_all_others [0xbfc00380 - 0xbfc003a8)
 */
exception_all_others: /* 0xbfc00380 */
	beqz	$fp, crash
	 nop
	jr	$fp
	 li	$a0, 3
crash: /* 0xbfc00390 */
	mfc0	$t0, $CP0_STATUS
	mfc0	$t1, $CP0_CAUSE
	mfc0	$t2, $CP0_CACHEERR
	mfc0	$t3, $CP0_EPC
dead_loop: /* 0xbfc003a0 */
	b	dead_loop
	 nop
start_me_up: /* 0xbfc003a8 */
	move	$at, $zero
	move	$v0, $zero
	move	$v1, $zero
	move	$a0, $zero
	move	$a1, $zero		# Branch to warm_start if (ST0_NMI|ST0_SR) set
	move	$a2, $zero
	move	$a3, $zero
	move	$t0, $zero
	move	$t1, $zero
	move	$t2, $zero
	move	$t3, $zero
	move	$t4, $zero		# Branch to warm_start if (CRIME_CONTROL_TRITON_SYSADC|CRIME_CONTROL_CRIME_SYSADC) set
	move	$t5, $zero
	move	$t6, $zero		# Initialize GPRs to 0
	move	$t7, $zero
	move	$s0, $zero
	move	$s1, $zero
	move	$s2, $zero
	move	$s3, $zero
	move	$s4, $zero
	move	$s5, $zero
	move	$s6, $zero
	move	$s7, $zero
	move	$t8, $zero
	move	$t9, $zero
	move	$k0, $zero
	move	$k1, $zero
	move	$gp, $zero
	move	$sp, $zero
	move	$fp, $zero
	move	$ra, $zero
	mfc0	$v1, $CP0_STATUS
	nop
	nop
	lui	$a1, 0x2400
	mtc0	$a1, $CP0_STATUS
	nop
	nop
	nop
	mtc1	$zero, $f0
	mtc1	$zero, $f1
	mtc1	$zero, $f2
	mtc1	$zero, $f3
	mtc1	$zero, $f4
	mtc1	$zero, $f5		# Save status register
	mtc1	$zero, $f6
	mtc1	$zero, $f7
	mtc1	$zero, $f8
	mtc1	$zero, $f9		# Enable floating-point unit
	mtc1	$zero, $f10
	mtc1	$zero, $f11
	mtc1	$zero, $f12
	mtc1	$zero, $f13		# Initialize FPRs to 0.0
	mtc1	$zero, $f14
	mtc1	$zero, $f15
	mtc1	$zero, $f16
	mtc1	$zero, $f17
	mtc1	$zero, $f18
	mtc1	$zero, $f19
	mtc1	$zero, $f20
	mtc1	$zero, $f21
	mtc1	$zero, $f22
	mtc1	$zero, $f23
	mtc1	$zero, $f24
	mtc1	$zero, $f25
	mtc1	$zero, $f26
	mtc1	$zero, $f27
	mtc1	$zero, $f28
	mtc1	$zero, $f29
	mtc1	$zero, $f30
	mtc1	$zero, $f31
	nop
	nop
	ctc1	$zero, $CP1_FCSR
	nop
	nop
	ctc1	$zero, $CP1_FCSR
	nop
	nop
	mtc0	$v1, $CP0_STATUS
	nop
L_0xbfc004ec: /* 0xbfc004ec */
	lui	$t8, HI(BASE_RTC + RTC_NVRAM(0x29) + BYTE_OFFSET)
	sb	$t9, LO(BASE_RTC + RTC_NVRAM(0x29) + BYTE_OFFSET)($t8)
	srl	$t9, $t9, 8
	sb	$t9, LO(BASE_RTC + RTC_NVRAM(0x2a) + BYTE_OFFSET)($t8)
	srl	$t9, $t9, 8
	sb	$t9, LO(BASE_RTC + RTC_NVRAM(0x2b) + BYTE_OFFSET)($t8)		# Initialize FPU control register
	srl	$t9, $t9, 8
	sb	$t9, LO(BASE_RTC + RTC_NVRAM(0x2c) + BYTE_OFFSET)($t8)
	srl	$t9, $t9, 8		# ... Twice?
	sb	$t9, LO(BASE_RTC + RTC_NVRAM(0x2d) + BYTE_OFFSET)($t8)
	srl	$t9, $t9, 8
	sb	$t9, LO(BASE_RTC + RTC_NVRAM(0x2e) + BYTE_OFFSET)($t8)		# Restore status register
	srl	$t9, $t9, 8
	sb	$t9, LO(BASE_RTC + RTC_NVRAM(0x2f) + BYTE_OFFSET)($t8)
	srl	$t9, $t9, 8		# *0xbf3a3707 = (t9 >>  0) & 0xff
	addiu	$t8, $t8, LO(BASE_RTC + BYTE_OFFSET)
	sb	$t9, LO(BASE_RTC + RTC_NVRAM(0x30))($t8)		# *0xbf3a3807 = (t9 >>  8) & 0xff
	lui	$t8, HI(BASE_ISA + ISA_MISC_CONTROL)
	ld	$t9, LO(BASE_ISA + ISA_MISC_CONTROL)($t8)		# *0xbf3a3907 = (t9 >> 16) & 0xff
	addiu	$t8, $t8, LO(BASE_ISA + ISA_MISC_CONTROL)
	li	$t8, 0x30		# *0xbf3a3a07   (t9 >> 24) & 0xff
	not	$t8, $t8
	and	$t9, $t9, $t8		# *0xbf3a3b07   (t9 >> 32) & 0xff
	lui	$t8, HI(BASE_ISA + ISA_MISC_CONTROL)
	addiu	$t8, $t8, LO(BASE_ISA + ISA_MISC_CONTROL)		# *0xbf3a3d07   (t9 >> 48) & 0xff
	sd	$t9, 0($t8)
	mfc0	$t8, $CP0_STATUS
	lui	$t9, 0x40
	and	$t8, $t8, $t9
	bnez	$t8, L_0xbfc005bc		# *0xbf3a3e07   (t9 >> 56) & 0xff
	 nop
	lui	$t8, 0x40
	mtc0	$t8, $CP0_STATUS
	li	$t9, 2
	beq	$a0, $t9, L_0xbfc005f4
	 li	$t8, 3		# Turn on Green and Red LEDs
	beq	$a0, $t8, L_0xbfc005f4
	 li	$t9, 4
	beq	$a0, $t9, L_0xbfc005f4
	 li	$t8, 1
	beq	$a0, $t8, L_0xbfc00704
	 li	$t9, 5
	beq	$a0, $t9, L_0xbfc005f4		# Branch to 0x5f0 if in bootstrap
	 li	$t8, 0
	beq	$a0, $t8, L_0xbfc00720
	 li	$t9, 7		# Set bootstrap mode
	beq	$a0, $t9, L_0xbfc005f4
	 li	$t8, 6		# Branch if a0 is 2, 3, 4, 5, 7, 6
	beq	$a0, $t8, L_0xbfc005f4
	 nop
	b	L_0xbfc00720
	 nop
L_0xbfc005bc: /* 0xbfc005bc */
	lui	$t8, HI(BASE_RTC + RTC_NVRAM(0x31) + BYTE_OFFSET)
	lb	$t9, LO(BASE_RTC + RTC_NVRAM(0x31) + BYTE_OFFSET)($t8)		# Branch if a0 is 1
	addiu	$t8, $t8, LO(BASE_RTC + BYTE_OFFSET)
	addi	$t9, $t9, 1
	sb	$t9, LO(BASE_RTC + RTC_SECONDS)($t8)
	mfc0	$t8, $CP0_STATUS		# Branch if a0 is 0
	lui	$t9, 0x10
	and	$t8, $t8, $t9
	bnez	$t8, L_0xbfc006d4
	 nop
	li	$a0, 0
	mtc0	$a0, $CP0_LLADDR
	b	L_0xbfc005f4
initialize_0: /* 0xbfc005f0 */
	 nop
L_0xbfc005f4: /* 0xbfc005f4 */
	move	$s7, $a0
	lui	$v0, HI_UNSIGNED(0x34410080)
	ori	$v0, $v0, LO_UNSIGNED(0x34410080)
	mtc0	$v0, $CP0_STATUS
	nop
	mfc0	$t0, $CP0_CONFIG
	li	$v0, 7
	not	$v0, $v0
	and	$t0, $t0, $v0
	ori	$t0, $t0, 3
	mtc0	$t0, $CP0_CONFIG
	mtc0	$zero, $CP0_CAUSE
	mtc0	$zero, $CP0_PAGEMASK
	mtc0	$zero, $CP0_WATCHLO
	mtc0	$zero, $CP0_WATCHHI
	move	$fp, $zero
	lui	$t8, HI(BASE_UART_1)
	li	$t9, 6
	addiu	$t8, $t8, LO(BASE_UART_1)
	sb	$t9, LO(BASE_UART_1 + UART_IIR)($t8)
	lui	$t8, HI(BASE_UART_2)
	addiu	$t8, $t8, LO(BASE_UART_2)
	sb	$t9, UART_IIR($t8)
	lui	$t9, HI(BASE_ISA + ISA_RING_BASE_AND_RESET)		# Set KSEG0 to cacheable, noncoherent
	addiu	$t9, $t9, LO(BASE_ISA + ISA_RING_BASE_AND_RESET)
	li	$t8, 0x400
	addiu	$at, $t9, 0x7fff
	sd	$t8, 1($at)
	addiu	$at, $t9, 0x7fff
	sd	$t8, 0x21($at)
	addiu	$at, $t9, 0x7fff
	sd	$t8, 0x4001($at)
	addiu	$at, $t9, 0x7fff
	sd	$t8, 0x4021($at)
	lui	$t8, HI(BASE_ISA + ISA_RING_BASE_AND_RESET)
	li	$t9, 1
	sd	$t9, LO(BASE_ISA + ISA_RING_BASE_AND_RESET)($t8)
	addiu	$t8, $t8, LO(BASE_ISA + ISA_RING_BASE_AND_RESET)
	li	$t9, 0x100
L_0xbfc00690: /* 0xbfc00690 */
	addi	$t9, $t9, -1
	bnez	$t9, L_0xbfc00690		# *(long *)0xbf318000 = 0x400
	 nop
	sd	$t9, LO(BASE_ISA + ISA_RING_BASE_AND_RESET)($t8)		# *(long *)0xbf318020 = 0x400
	jal	F_0xbfc01810
	 nop		# *(long *)0xbf31c000 = 0x400
	lui	$s0, HI(KSEG0)
	jal	F_0xbfc007c0		# *(long *)0xbf31c020 = 0x400
	 addi	$sp, $s0, 0x1000
	lui	$t0, HI(BASE_RTC + RTC_NVRAM(0x31) + BYTE_OFFSET)
	lb	$a1, LO(BASE_RTC + RTC_NVRAM(0x31) + BYTE_OFFSET)($t0)		# Reset ISA controller
	move	$a0, $s7
	addiu	$t0, $t0, LO(BASE_RTC + BYTE_OFFSET)
isa_reset_delay_loop: /* 0xbfc006c4 */
	jal	F_0xbfc008e0
	 addi	$sp, $sp, -8		# Delay... count down from 256
	b	F_0xbfc00000
	 nop		# ISA_RING_BASE_AND_RESET; take out of reset state
L_0xbfc006d4: /* 0xbfc006d4 */
	lui	$t8, HI(BASE_RTC + RTC_NVRAM(0x31) + BYTE_OFFSET)
	lbu	$t9, LO(BASE_RTC + RTC_NVRAM(0x31) + BYTE_OFFSET)($t8)
	addiu	$t8, $t8, LO(BASE_RTC + BYTE_OFFSET)
	li	$t8, 1
	beq	$t8, $t9, L_0xbfc006f8		# $sp = 0x80001000
	 nop
	b	L_0xbfc005f4
	 nop
	li	$a0, 0		# unreachable
L_0xbfc006f8: /* 0xbfc006f8 */
	b	L_0xbfc005f4
	 nop
	li	$a0, 1		# unreachable
L_0xbfc00704: /* 0xbfc00704 */
	lui	$a0, HI(BASE_CRIME + CRIME_CONTROL_OFFSET)
	li	$a1, 0x400
	addiu	$a0, $a0, LO(BASE_CRIME + CRIME_CONTROL_OFFSET)
	bal	F_0xbfc0073c
	 sd	$a1, 0($a0)
	b	L_0xbfc00704
	 nop
L_0xbfc00720: /* 0xbfc00720 */
	lui	$a0, HI(BASE_CRIME + CRIME_CONTROL_OFFSET)
	li	$a1, 0x800
	addiu	$a0, $a0, LO(BASE_CRIME + CRIME_CONTROL_OFFSET)
	bal	F_0xbfc0073c
	 sd	$a1, 0($a0)
	b	L_0xbfc00720
soft_reset: /* 0xbfc00738 */
	 nop

/* Function F_0xbfc0073c [0xbfc0073c - 0xbfc0079c)
 */
F_0xbfc0073c: /* 0xbfc0073c */
	lui	$a0, HI(BASE_ISA + ISA_MISC_CONTROL)
	ld	$a1, LO(BASE_ISA + ISA_MISC_CONTROL)($a0)
	lui	$a2, HI(ROM_SIZE)
	addiu	$a0, $a0, LO(BASE_ISA + ISA_MISC_CONTROL)
	xori	$a1, $a1, 0x30
	sd	$a1, 0($a0)
hard_reset: /* 0xbfc00754 */
	dsll32	$a2, $a2, 0
L_0xbfc00758: /* 0xbfc00758 */
	bgtz	$a2, L_0xbfc00758
	 addi	$a2, $a2, -1
	jr	$ra
	 nop
	b	F_0xbfc0079c		# unreachable
	 nop		# unreachable
wait_for_reset: /* 0xbfc00770 */
	move	$t0, $s0		# unreachable
	addi	$t0, $t0, 4		# unreachable
	.word	0x17a8fffe	/* bne $sp, $t0, 0xbfc00774 */		# unreachable
	 sw	$a0, -4($t0)		# unreachable
	move	$t0, $s0		# unreachable
	lw	$t1, 0($t0)		# unreachable; ISA_MISC_CONTROL
	bne	$t1, $a0, F_0xbfc007a4		# unreachable
	 nop		# unreachable
	addi	$t0, $t0, 4		# unreachable; Delay... count down from 8<<48
	.word	0x151dfffb	/* bne $t0, $sp, 0xbfc00784 */		# unreachable
	 nop		# unreachable

/* Function F_0xbfc0079c [0xbfc0079c - 0xbfc007a4)
 */
F_0xbfc0079c: /* 0xbfc0079c */
	jr	$ra
	 nop

/* Function F_0xbfc007a4 [0xbfc007a4 - 0xbfc007c0)
 */
F_0xbfc007a4: /* 0xbfc007a4 */
	beqz	$fp, crash
	 nop
	b	F_0xbfc007a4
	 nop
	nop		# unreachable
	nop		# unreachable
	nop		# unreachable

/* Function F_0xbfc007c0 [0xbfc007c0 - 0xbfc008e0)
 */
F_0xbfc007c0: /* 0xbfc007c0 */
	mfc0	$t0, $CP0_PRID
	andi	$t0, $t0, 0xff00
	li	$t1, 9
	srl	$t0, $t0, 8
	bne	$t0, $t1, L_0xbfc007e0
	 nop
	b	L_0xbfc0081c
	 nop
L_0xbfc007e0: /* 0xbfc007e0 */
	mtc0	$zero, $CP0_TAGHI
	move	$t0, $s0
warm_start: /* 0xbfc007e8 */
	sll	$at, $t0, 3
	srl	$at, $at, 0xf
	srl	$at, $at, 8
	ori	$at, $at, 0xc0
	mtc0	$at, $CP0_TAGLO
	move	$t1, $t0
	addiu	$t0, $t0, 0x20
	cache	(CACHE_TYPE_L1D|INDEX_STORE_TAG), 0($t1)
	sltu	$at, $sp, $t0
	beqz	$at, warm_start
	 nop
	b	L_0xbfc008b4
	 nop
L_0xbfc0081c: /* 0xbfc0081c */
	mfc0	$t1, $CP0_CONFIG
	mtc0	$zero, $CP0_TAGHI
	andi	$t1, $t1, 0x2000
	bgtz	$t1, L_0xbfc00834
	 li	$t0, 0x80
	li	$t0, 0x40
L_0xbfc00834: /* 0xbfc00834 */
	move	$t1, $s0
L_0xbfc00838: /* 0xbfc00838 */
	sll	$at, $t1, 3
	srl	$at, $at, 0x15
	sll	$at, $at, 0xe
	andi	$t2, $t1, 0x3000
	ori	$at, $at, 0xc00
	srl	$t2, $t2, 5
	or	$t2, $t2, $at
	mtc0	$t2, $CP0_TAGLO
	nop
	addu	$t3, $t1, $t0
	cache	(CACHE_TYPE_L2|INDEX_STORE_TAG), 0($t1)
	sltu	$at, $sp, $t3
	beqz	$at, L_0xbfc00838
	 move	$t1, $t3
	lui	$t0, 0x4000
	mtc0	$t0, $CP0_TAGHI
	move	$t1, $s0
L_0xbfc0087c: /* 0xbfc0087c */
	sll	$at, $t1, 3
	srl	$at, $at, 0xf
	sll	$at, $at, 8
	ori	$at, $at, 0xc8
	mtc0	$at, $CP0_TAGLO
	nop
	addiu	$t2, $t1, 0x20
	sltu	$at, $sp, $t2
	cache	(CACHE_TYPE_L1D|INDEX_STORE_TAG), 0($t1)
	nop
	bnez	$at, L_0xbfc0087c
	 move	$t1, $t2
	b	L_0xbfc008b4
	 nop
L_0xbfc008b4: /* 0xbfc008b4 */
	addiu	$t0, $sp, -8
	nop
L_0xbfc008bc: /* 0xbfc008bc */
	not	$t1, $t0
	addiu	$t2, $t0, -8
	sd	$t1, 0($t0)
	bne	$t2, $s0, L_0xbfc008bc
	 move	$t0, $t2
	jr	$ra
	 nop
	nop		# unreachable
	nop		# unreachable

/* Function F_0xbfc008e0 [0xbfc008e0 - 0xbfc009d0)
 */
F_0xbfc008e0: /* 0xbfc008e0 */
	addiu	$sp, $sp, -0x20
	sw	$a0, 0x20($sp)
	sw	$a1, 0x24($sp)
	sw	$ra, 0x14($sp)
cache_init: /* 0xbfc008f0 */
	lui	$a1, %hi(data_0xbfc01b50)
	lui	$a0, %hi(data_0xbfc01be0)
	addiu	$a0, $a0, %lo(data_0xbfc01be0)
	addiu	$a1, $a1, %lo(data_0xbfc01b50)
	jal	F_0xbfc015b0
	 move	$a2, $zero
	lui	$a0, %hi(data_0xbfc01be0)
	lui	$a1, %hi(data_0xbfc01b58)
	addiu	$a1, $a1, %lo(data_0xbfc01b58)
	addiu	$a0, $a0, %lo(data_0xbfc01be0)
	move	$a2, $zero
	jal	F_0xbfc01530
	 sw	$v0, 0x1c($sp)
cache_init_rm7k_loop: /* 0xbfc00924 */
	lui	$a3, HI_UNSIGNED(BASE_ISA | ISA_MISC_CONTROL)
	ori	$a3, $a3, LO_UNSIGNED(BASE_ISA | ISA_MISC_CONTROL)
	lw	$t6, 0($a3)
	lw	$t7, 4($a3)
	li	$at, -1
	lw	$v1, 0x1c($sp)
	and	$t8, $t6, $at
	li	$at, -0x31
	sw	$t8, 0($a3)
	and	$t9, $t7, $at
	move	$a1, $v0
	beqz	$v1, L_0xbfc00974
	 sw	$t9, 4($a3)
	beqz	$v0, L_0xbfc00974
	 lw	$a0, 0x20($sp)
	jalr	$v1
	 lw	$a2, 0x24($sp)
	lui	$a3, HI_UNSIGNED(BASE_ISA | ISA_MISC_CONTROL)
	beqz	$v0, L_0xbfc00994
	 ori	$a3, $a3, LO_UNSIGNED(BASE_ISA | ISA_MISC_CONTROL)
L_0xbfc00974: /* 0xbfc00974 */
	lw	$t0, 0($a3)
	lw	$t1, 4($a3)
	li	$at, -1
	and	$t2, $t0, $at
cache_init_r5k: /* 0xbfc00984 */
	li	$at, -0x31
	sw	$t2, 0($a3)
cache_init_r5k_loop: /* 0xbfc0098c */
	and	$t3, $t1, $at
	sw	$t3, 4($a3)
L_0xbfc00994: /* 0xbfc00994 */
	jal	F_0xbfc009d0
	 nop
	b	L_0xbfc00994
	 nop
	nop		# unreachable
	nop		# unreachable
	nop		# unreachable
	nop		# unreachable
	nop		# unreachable
	nop		# unreachable
	nop		# unreachable
cache_init_r10k: /* 0xbfc009c0 */
	lw	$ra, 0x14($sp)		# unreachable
	addiu	$sp, $sp, 0x20		# unreachable
	jr	$ra		# unreachable
	 nop		# unreachable

/* Function F_0xbfc009d0 [0xbfc009d0 - 0xbfc00aa8)
 */
F_0xbfc009d0: /* 0xbfc009d0 */
	addiu	$sp, $sp, -0x1a0
	sw	$ra, 0x1c($sp)
	jal	F_0xbfc00fc8
cache_init_r10k_l2_loop: /* 0xbfc009dc */
	 sw	$s0, 0x18($sp)
	sw	$v0, 0x19c($sp)
	sw	$zero, 0x128($sp)
	sw	$zero, 0x12c($sp)
	sw	$zero, 0x130($sp)
	sw	$zero, 0x24($sp)
	sw	$zero, 0x194($sp)
	sw	$zero, 0x198($sp)
	jal	F_0xbfc00ad8
	 li	$a0, 1
	addiu	$s0, $sp, 0x24
	jal	F_0xbfc00b2c
	 move	$a0, $s0
	bnez	$v0, L_0xbfc00a44
	 move	$v1, $v0
L_0xbfc00a18: /* 0xbfc00a18 */
	jal	F_0xbfc00eb4
	 move	$a0, $s0
cache_init_r10k_l1d_loop: /* 0xbfc00a20 */
	bnez	$v0, L_0xbfc00a44
	 move	$v1, $v0
	lw	$t6, 0x128($sp)
	move	$a0, $s0
	addiu	$t7, $t6, 1
	jal	F_0xbfc00b2c
	 sw	$t7, 0x128($sp)
	beqz	$v0, L_0xbfc00a18
	 move	$v1, $v0
L_0xbfc00a44: /* 0xbfc00a44 */
	move	$a0, $s0
	move	$a1, $zero
	jal	F_0xbfc00d68
	 sw	$v1, 0x20($sp)
	jal	F_0xbfc00ad8
	 move	$a0, $zero
	lw	$v1, 0x20($sp)
	sra	$a0, $v1, 8
	jal	F_0xbfc01268
	 andi	$a0, $a0, 0xff
	jal	F_0xbfc01300
	 lw	$a0, 0x128($sp)
	jal	F_0xbfc01268
	 li	$a0, 0x2d
	jal	F_0xbfc01300

/* Function F_0xbfc00a80 [0xbfc00a80 - 0xbfc00aa8)
 *
 * $a0 = initialization number
 * $a1
 */
F_0xbfc00a80: /* 0xbfc00a80 */
	 lw	$a0, 0x130($sp)
	jal	F_0xbfc01268
	 li	$a0, 0xa
	jal	F_0xbfc0138c
	 nop
	lw	$ra, 0x1c($sp)
	lw	$s0, 0x18($sp)		# Pointer to 0xbfc01e70 containing struct { start=0xbfc00000, end=0xbfc80000, alignment=0x100, length=0x00080000, 0xbfc00c48, nop1, isa_reset }
	addiu	$sp, $sp, 0x1a0		# Pointer to 0xbfc01de0 containing "post1"
	jr	$ra		# Call find_valid_section_by_name(flash_info, "post1", 0)
	 nop

/* Function F_0xbfc00aa8 [0xbfc00aa8 - 0xbfc00ad0)
 */
F_0xbfc00aa8: /* 0xbfc00aa8 */
	sw	$a1, 4($sp)
	lbu	$t6, 4($sp)
	sb	$t6, 0($a0)		# Pointer to 0xbfc01de8 containing "firmware"
	lbu	$t7, 5($sp)		# Pointer to 0xbfc01e70 containing struct { start=0xbfc00000, end=0xbfc80000, alignment=0x100, length=0x00080000, 0xbfc00c48, nop1, isa_reset }
	sb	$t7, 1($a0)
	lbu	$t8, 6($sp)		# Call find_section_by_name(flash_info, "firmware", 0)
	sb	$t8, 2($a0)		# Save address of post1 section
	lbu	$t9, 7($sp)
	jr	$ra
	 sb	$t9, 3($a0)		# ISA_MISC_CONTROL

/* Function F_0xbfc00ad0 [0xbfc00ad0 - 0xbfc00ad8)
 */
F_0xbfc00ad0: /* 0xbfc00ad0 */
	jr	$ra		# ISA_MISC_CONTROL + 0x04
	 nop

/* Function F_0xbfc00ad8 [0xbfc00ad8 - 0xbfc00b2c)
 */
F_0xbfc00ad8: /* 0xbfc00ad8 */
	beqz	$a0, L_0xbfc00b00		# $v1 = address of post1 section
	 lui	$v0, HI_UNSIGNED(BASE_ISA | ISA_MISC_CONTROL)
	lui	$v0, HI_UNSIGNED(BASE_ISA | ISA_MISC_CONTROL)
	ori	$v0, $v0, LO_UNSIGNED(BASE_ISA | ISA_MISC_CONTROL)		# ISA_MISC_CONTROL
	lw	$t7, 4($v0)
	lw	$t6, 0($v0)		# $a1 = address of firmware section
	ori	$t9, $t7, 1
	sw	$t9, 4($v0)
	jr	$ra
	 sw	$t6, 0($v0)
L_0xbfc00b00: /* 0xbfc00b00 */
	ori	$v0, $v0, LO_UNSIGNED(BASE_ISA | ISA_MISC_CONTROL)		# Call post1($a0=$a0, $a1=address of firmware section, $a2=$a1)
	lw	$t0, 0($v0)
	lw	$t1, 4($v0)
	li	$at, -1
	and	$t2, $t0, $at
	li	$at, -2		# ISA_MISC_CONTROL
	and	$t3, $t1, $at		# ISA_MISC_CONTROL + 0x04
	sw	$t3, 4($v0)
	sw	$t2, 0($v0)
	jr	$ra
	 nop

/* Function F_0xbfc00b2c [0xbfc00b2c - 0xbfc00d68)
 */
F_0xbfc00b2c: /* 0xbfc00b2c */
	addiu	$sp, $sp, -0x58
	sw	$fp, 0x38($sp)
	sw	$s7, 0x34($sp)
	sw	$s6, 0x30($sp)
	sw	$s5, 0x2c($sp)
	sw	$s4, 0x28($sp)
	sw	$s3, 0x24($sp)
	sw	$s2, 0x20($sp)
	sw	$s1, 0x1c($sp)
	sw	$s0, 0x18($sp)
	sw	$ra, 0x3c($sp)
	lui	$s5, %hi(data_0xbfc01b6c)
	lui	$s7, %hi(data_0xbfc01b64)
	move	$s0, $a0
	move	$s3, $zero
	addiu	$s7, $s7, %lo(data_0xbfc01b64)
	addiu	$s5, $s5, %lo(data_0xbfc01b6c)
	lw	$v0, 0x178($a0)
	move	$s1, $zero
	lw	$s2, 0x44($sp)
	li	$s4, 0x3a
	li	$s6, 4
	li	$fp, 1
L_0xbfc00b88: /* 0xbfc00b88 */
	bne	$fp, $v0, L_0xbfc00ba0
	 nop
	jal	F_0xbfc01300
	 lw	$a0, 0x104($s0)
	jal	F_0xbfc01268
	 li	$a0, 0x3e
L_0xbfc00ba0: /* 0xbfc00ba0 */
	jal	F_0xbfc01358
	 move	$a0, $fp
L_0xbfc00ba8: /* 0xbfc00ba8 */
	jal	F_0xbfc011f8
	 nop
	andi	$t6, $v0, 0xff00
	beqz	$t6, L_0xbfc00bc8
	 nop
	sra	$s3, $v0, 8
	b	L_0xbfc00ce0
	 li	$s1, 8
L_0xbfc00bc8: /* 0xbfc00bc8 */
	bnez	$s1, L_0xbfc00be8
	 sll	$t7, $s1, 3
	bnel	$v0, $s4, L_0xbfc00ce4
	 slti	$at, $s1, 7
	move	$s1, $fp
	sw	$zero, 0x110($s0)
	b	L_0xbfc00ce0
	 sw	$zero, 0x114($s0)
L_0xbfc00be8: /* 0xbfc00be8 */
	lw	$v1, 0x110($s0)
	addu	$a1, $s5, $t7
	lh	$t0, 0($a1)
	bgez	$v1, L_0xbfc00c04
	 sra	$t8, $v1, 1
	addiu	$at, $v1, 1
	sra	$t8, $at, 1
L_0xbfc00c04: /* 0xbfc00c04 */
	addu	$t9, $t8, $s0
	slti	$at, $v0, 0x30
	addiu	$t1, $v1, 1
	sw	$t1, 0x110($s0)
	bnez	$at, L_0xbfc00c30
	 addu	$a0, $t9, $t0
	slti	$at, $v0, 0x3a
	beqzl	$at, L_0xbfc00c34
	 slti	$at, $v0, 0x41
	b	L_0xbfc00c64
	 addiu	$s2, $v0, -0x30
L_0xbfc00c30: /* 0xbfc00c30 */
	slti	$at, $v0, 0x41
L_0xbfc00c34: /* 0xbfc00c34 */
	bnez	$at, L_0xbfc00c4c
	 slti	$at, $v0, 0x47
	beqzl	$at, L_0xbfc00c50
	 slti	$at, $v0, 0x61
	b	L_0xbfc00c64
	 addiu	$s2, $v0, -0x37		# void ...(out $a0, in $a1)
L_0xbfc00c4c: /* 0xbfc00c4c */
	slti	$at, $v0, 0x61
L_0xbfc00c50: /* 0xbfc00c50 */
	bnez	$at, L_0xbfc00c64
	 slti	$at, $v0, 0x67
	beqzl	$at, L_0xbfc00c68
	 lw	$t2, 0x110($s0)
	addiu	$s2, $v0, -0x57
L_0xbfc00c64: /* 0xbfc00c64 */
	lw	$t2, 0x110($s0)
L_0xbfc00c68: /* 0xbfc00c68 */
	sll	$t4, $s2, 4
	andi	$t3, $t2, 1
nop1: /* 0xbfc00c70 */
	beqzl	$t3, L_0xbfc00c84
	 lbu	$t5, 0($a0)

/* Function isa_reset [0xbfc00c78 - 0xbfc00d68)
 */
isa_reset: /* 0xbfc00c78 */
	b	L_0xbfc00c9c
	 sb	$t4, 0($a0)
	lbu	$t5, 0($a0)		# unreachable
L_0xbfc00c84: /* 0xbfc00c84 */
	or	$t8, $t5, $s2
	sb	$t8, 0($a0)		# ISA_MISC_CONTROL + 0x04
	lw	$t7, 0x114($s0)		# ISA_MISC_CONTROL
	sll	$t9, $t8, 0x18
	addu	$t0, $t7, $t9		# ISA_MISC_CONTROL + 0x04
	sw	$t0, 0x114($s0)
L_0xbfc00c9c: /* 0xbfc00c9c */
	lh	$v0, 2($a1)		# ISA_MISC_CONTROL
	bnezl	$v0, L_0xbfc00cb4
	 lw	$t1, 0x110($s0)		# ISA_MISC_CONTROL
	lbu	$v0, 0x11c($s0)		# ISA_MISC_CONTROL + 0x04
	sll	$v0, $v0, 1
	lw	$t1, 0x110($s0)
L_0xbfc00cb4: /* 0xbfc00cb4 */
	slt	$at, $t1, $v0
	bnezl	$at, L_0xbfc00ce4
	 slti	$at, $s1, 7		# ISA_MISC_CONTROL + 0x04
	bne	$s1, $s6, L_0xbfc00cdc		# ISA_MISC_CONTROL
	 sw	$zero, 0x110($s0)
	lbu	$t2, 0x11d($s0)
	andi	$t3, $t2, 7
	addu	$t4, $s7, $t3
	b	L_0xbfc00ce0
	 lbu	$s1, 0($t4)
L_0xbfc00cdc: /* 0xbfc00cdc */
	lw	$s1, 4($a1)
L_0xbfc00ce0: /* 0xbfc00ce0 */
	slti	$at, $s1, 7
L_0xbfc00ce4: /* 0xbfc00ce4 */
	bnez	$at, L_0xbfc00ba8
	 nop
	jal	F_0xbfc01358
	 move	$a0, $zero
	lw	$t5, 0x114($s0)
	beqzl	$t5, L_0xbfc00d08
	 lw	$v0, 0x178($s0)
	li	$s3, 0x5800
	lw	$v0, 0x178($s0)
L_0xbfc00d08: /* 0xbfc00d08 */
	bne	$fp, $v0, L_0xbfc00d20
	 nop
	bnez	$s3, L_0xbfc00d20
	 nop
	b	L_0xbfc00b88
	 move	$s1, $zero
L_0xbfc00d20: /* 0xbfc00d20 */
	bnez	$s3, L_0xbfc00d38
	 lw	$s1, 0x1c($sp)
	lbu	$t6, 0x11d($s0)
	bnel	$fp, $t6, L_0xbfc00d3c
	 lw	$ra, 0x3c($sp)
	li	$s3, 0x2400
L_0xbfc00d38: /* 0xbfc00d38 */
	lw	$ra, 0x3c($sp)
L_0xbfc00d3c: /* 0xbfc00d3c */
	move	$v0, $s3
	lw	$s3, 0x24($sp)
	lw	$s0, 0x18($sp)
	lw	$s2, 0x20($sp)
	lw	$s4, 0x28($sp)
	lw	$s5, 0x2c($sp)
	lw	$s6, 0x30($sp)
	lw	$s7, 0x34($sp)
	lw	$fp, 0x38($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x58

/* Function F_0xbfc00d68 [0xbfc00d68 - 0xbfc00eb4)
 */
F_0xbfc00d68: /* 0xbfc00d68 */
	addiu	$sp, $sp, -0x28
	lui	$at, HI_UNSIGNED(0xbfc04000)
	ori	$at, $at, LO_UNSIGNED(0xbfc04000)
	sw	$s2, 0x1c($sp)
	sltu	$at, $a1, $at
	move	$s2, $a0
	sw	$ra, 0x24($sp)
	sw	$s3, 0x20($sp)
	sw	$s1, 0x18($sp)
	sw	$s0, 0x14($sp)
	bnez	$at, L_0xbfc00da8
	 move	$a2, $a1
	lui	$at, HI(ROM_END)
	sltu	$at, $a1, $at
	bnezl	$at, L_0xbfc00db4
	 sw	$a2, 0x2c($sp)
L_0xbfc00da8: /* 0xbfc00da8 */
	b	L_0xbfc00e98
	 li	$v0, 0x4100
	sw	$a2, 0x2c($sp)		# unreachable
L_0xbfc00db4: /* 0xbfc00db4 */
	lw	$t6, 0($s2)
	move	$s0, $zero
	move	$s1, $s2
	beqz	$t6, L_0xbfc00e24
	 li	$s3, 0x100
	sw	$a2, 0x2c($sp)
L_0xbfc00dcc: /* 0xbfc00dcc */
	lw	$t7, 0($s2)
	lw	$a1, 4($s1)
	jal	F_0xbfc00aa8
	 addu	$a0, $t7, $s0
	addiu	$s0, $s0, 4
	bne	$s0, $s3, L_0xbfc00dcc
	 addiu	$s1, $s1, 4
	jal	F_0xbfc00ad0
	 nop
	lw	$t8, 0x178($s2)
	li	$at, 1
	beql	$t8, $at, L_0xbfc00e28
	 lw	$a2, 0x2c($sp)
	lw	$t9, 0x108($s2)
	addiu	$t0, $t9, 1
	andi	$t2, $t0, 0x3f
	bnez	$t2, L_0xbfc00e1c
	 sw	$t0, 0x108($s2)
	jal	F_0xbfc01268
	 li	$a0, 0xa
L_0xbfc00e1c: /* 0xbfc00e1c */
	jal	F_0xbfc01268
	 li	$a0, 0x2e
L_0xbfc00e24: /* 0xbfc00e24 */
	lw	$a2, 0x2c($sp)
L_0xbfc00e28: /* 0xbfc00e28 */
	li	$at, -0x100
	move	$v0, $zero
	beqz	$a2, L_0xbfc00e94
	 addiu	$t3, $a2, 0xff
	and	$t4, $t3, $at
	sw	$t4, 0($s2)
	move	$v1, $s2
	li	$a0, 0x40
L_0xbfc00e48: /* 0xbfc00e48 */
	lw	$t5, 0($s2)
	addiu	$v1, $v1, 4
	addu	$t6, $t5, $v0
	lbu	$t7, 0($t6)
	sb	$t7, 0($v1)
	lw	$t8, 0($s2)
	addu	$t9, $t8, $v0
	lbu	$t0, 1($t9)
	sb	$t0, 1($v1)
	lw	$t1, 0($s2)
	addu	$t2, $t1, $v0
	lbu	$t3, 2($t2)
	sb	$t3, 2($v1)
	lw	$t4, 0($s2)
	addu	$t5, $t4, $v0
	lbu	$t6, 3($t5)
	addiu	$v0, $v0, 4
	bne	$v0, $a0, L_0xbfc00e48
	 sb	$t6, 3($v1)
L_0xbfc00e94: /* 0xbfc00e94 */
	move	$v0, $zero
L_0xbfc00e98: /* 0xbfc00e98 */
	lw	$ra, 0x24($sp)
	lw	$s0, 0x14($sp)
	lw	$s1, 0x18($sp)
	lw	$s2, 0x1c($sp)
	lw	$s3, 0x20($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x28

/* Function F_0xbfc00eb4 [0xbfc00eb4 - 0xbfc00fc8)
 */
F_0xbfc00eb4: /* 0xbfc00eb4 */
	addiu	$sp, $sp, -0x20
	sw	$ra, 0x1c($sp)
	sw	$s1, 0x18($sp)
	sw	$s0, 0x14($sp)
	lbu	$v0, 0x11d($a0)
	move	$s0, $a0
	move	$v1, $zero
	beqz	$v0, L_0xbfc00ef0
	 li	$at, 2
	beq	$v0, $at, L_0xbfc00f94
	 li	$at, 4
	beql	$v0, $at, L_0xbfc00fa8
	 lhu	$t6, 0x11a($s0)
	b	L_0xbfc00fb4
	 move	$v0, $v1
L_0xbfc00ef0: /* 0xbfc00ef0 */
	lbu	$t6, 0x11c($s0)
	move	$s1, $zero
	blezl	$t6, L_0xbfc00fb4
	 move	$v0, $v1
	lw	$t7, 0x170($s0)
L_0xbfc00f04: /* 0xbfc00f04 */
	lw	$t8, 0x174($s0)
	lhu	$t0, 0x118($s0)
	lw	$t2, 0($s0)
	addu	$t9, $t7, $t8
	addu	$t1, $t9, $t0
	addu	$a1, $t1, $s1
	subu	$v0, $a1, $t2
	bltz	$v0, L_0xbfc00f30
	 slti	$at, $v0, 0x100
	bnezl	$at, L_0xbfc00f6c
	 addu	$t0, $s0, $s1
L_0xbfc00f30: /* 0xbfc00f30 */
	jal	F_0xbfc00d68
	 move	$a0, $s0
	beqz	$v0, L_0xbfc00f48
	 move	$v1, $v0
	b	L_0xbfc00fb8
	 lw	$ra, 0x1c($sp)
L_0xbfc00f48: /* 0xbfc00f48 */
	lw	$t3, 0x170($s0)
	lw	$t4, 0x174($s0)
	lhu	$t6, 0x118($s0)
	lw	$t9, 0($s0)
	addu	$t5, $t3, $t4
	addu	$t7, $t5, $t6
	addu	$t8, $t7, $s1
	subu	$v0, $t8, $t9
	addu	$t0, $s0, $s1
L_0xbfc00f6c: /* 0xbfc00f6c */
	lbu	$t1, 0x11f($t0)
	addu	$t2, $s0, $v0
	addiu	$s1, $s1, 1
	sb	$t1, 4($t2)
	lbu	$t3, 0x11c($s0)
	slt	$at, $s1, $t3
	bnezl	$at, L_0xbfc00f04
	 lw	$t7, 0x170($s0)
	b	L_0xbfc00fb4
	 move	$v0, $v1
L_0xbfc00f94: /* 0xbfc00f94 */
	lhu	$t4, 0x11a($s0)
	sll	$t5, $t4, 4
	b	L_0xbfc00fb0
	 sw	$t5, 0x170($s0)
	lhu	$t6, 0x11a($s0)		# unreachable
L_0xbfc00fa8: /* 0xbfc00fa8 */
	sll	$t7, $t6, 0x10
	sw	$t7, 0x174($s0)
L_0xbfc00fb0: /* 0xbfc00fb0 */
	move	$v0, $v1
L_0xbfc00fb4: /* 0xbfc00fb4 */
	lw	$ra, 0x1c($sp)
L_0xbfc00fb8: /* 0xbfc00fb8 */
	lw	$s0, 0x14($sp)
	lw	$s1, 0x18($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x20

/* Function F_0xbfc00fc8 [0xbfc00fc8 - 0xbfc011f8)
 */
F_0xbfc00fc8: /* 0xbfc00fc8 */
	addiu	$sp, $sp, -0x40
	sw	$s5, 0x2c($sp)
	sw	$s4, 0x28($sp)
	sw	$fp, 0x38($sp)
	sw	$s7, 0x34($sp)
	sw	$s6, 0x30($sp)
	sw	$s3, 0x24($sp)
	sw	$s2, 0x20($sp)
	sw	$s1, 0x1c($sp)
	sw	$s0, 0x18($sp)
	lui	$s4, %hi(tlb_r10k_write_tlb_loop)
	lui	$s5, HI_UNSIGNED(0x001c2000)
	sw	$ra, 0x3c($sp)
	li	$v0, 1
	ori	$s5, $s5, LO_UNSIGNED(0x001c2000)
	addiu	$s4, $s4, %lo(tlb_r10k_write_tlb_loop)
	move	$s0, $zero
	move	$s1, $zero
	li	$s2, 0x3e00
	li	$s3, 3
	li	$s6, 0x80
	li	$s7, 0x1b
	li	$fp, 3
L_0xbfc01024: /* 0xbfc01024 */
	lw	$t6, 0($s4)
L_0xbfc01028: /* 0xbfc01028 */
	lui	$t7, %hi(data_0xbfc01c08)
	bnez	$t6, L_0xbfc01040
	 nop
	lui	$s4, %hi(data_0xbfc01bc0)
	addiu	$s4, $s4, %lo(data_0xbfc01bc0)
	move	$v0, $zero
L_0xbfc01040: /* 0xbfc01040 */
	lw	$t7, %lo(data_0xbfc01c08)($t7)
	lui	$t2, %hi(data_0xbfc01bfc)
	lui	$a0, %hi(data_0xbfc01bb4)
	sb	$s6, 0($t7)
	lw	$t8, 0($s4)
	lw	$t2, %lo(data_0xbfc01bfc)($t2)
	addiu	$a0, $a0, %lo(data_0xbfc01bb4)
	sll	$t9, $t8, 4
	div	$zero, $s5, $t9
	mflo	$t0
	move	$t1, $t0
	sb	$t1, 0($t2)
	lw	$t3, 0($s4)
	lui	$t8, %hi(data_0xbfc01c00)
	lw	$t8, %lo(data_0xbfc01c00)($t8)
	sll	$t4, $t3, 4
	div	$zero, $s5, $t4
	mflo	$t5
	sra	$t7, $t5, 8
	bnez	$t9, L_0xbfc01098
	 nop
	break	7
L_0xbfc01098: /* 0xbfc01098 */
	li	$at, -1
	bne	$t9, $at, L_0xbfc010b0
	 lui	$at, HI(KSEG0)
	bne	$s5, $at, L_0xbfc010b0
	 nop
	break	6
L_0xbfc010b0: /* 0xbfc010b0 */
	lui	$t9, %hi(data_0xbfc01c08)
	sb	$t7, 0($t8)
	lw	$t9, %lo(data_0xbfc01c08)($t9)
	lui	$t0, %hi(data_0xbfc01c00)
	lui	$t1, %hi(data_0xbfc01c0c)
	sb	$s7, 0($t9)
	lw	$t0, %lo(data_0xbfc01c00)($t0)
	lui	$t3, %hi(data_0xbfc01c04)
	bnez	$t4, L_0xbfc010dc
	 nop
	break	7
L_0xbfc010dc: /* 0xbfc010dc */
	li	$at, -1
	bne	$t4, $at, L_0xbfc010f4
	 lui	$at, HI(KSEG0)
	bne	$s5, $at, L_0xbfc010f4
	 nop
	break	6
L_0xbfc010f4: /* 0xbfc010f4 */
	sb	$zero, 0($t0)
	lw	$t1, %lo(data_0xbfc01c0c)($t1)
	li	$t2, 7
	sb	$fp, 0($t1)
	lw	$t3, %lo(data_0xbfc01c04)($t3)
	jal	F_0xbfc012b4
	 sb	$t2, 0($t3)
	jal	F_0xbfc01300
	 lw	$a0, 0($s4)
	lui	$a0, %hi(tlb_r10k_init)
	jal	F_0xbfc012b4
	 addiu	$a0, $a0, %lo(tlb_r10k_init)
	jal	F_0xbfc01268
	 li	$a0, 0x3e
	jal	F_0xbfc01358
	 li	$a0, 1
L_0xbfc01134: /* 0xbfc01134 */
	jal	F_0xbfc011f8
	 nop
	bne	$v0, $s2, L_0xbfc01154
	 move	$v1, $v0
	addiu	$s4, $s4, 4
	move	$s1, $zero
	b	L_0xbfc01024
	 move	$s0, $zero
L_0xbfc01154: /* 0xbfc01154 */
	andi	$t4, $v0, 0xff00
	beqz	$t4, L_0xbfc0117c
	 sll	$t5, $s1, 8
	sra	$a0, $v1, 8
	andi	$a0, $a0, 0xff
	move	$s1, $zero
	jal	F_0xbfc01268
	 move	$s0, $zero
	b	L_0xbfc01028
	 lw	$t6, 0($s4)
L_0xbfc0117c: /* 0xbfc0117c */
	addiu	$s0, $s0, 1
	bne	$s0, $s3, L_0xbfc01134
	 or	$s1, $t5, $v0
	jal	F_0xbfc01358
	 move	$a0, $zero
	lui	$at, HI_UNSIGNED(0x00534c44)
	ori	$at, $at, LO_UNSIGNED(0x00534c44)
	bne	$s1, $at, L_0xbfc011a8
	 move	$s0, $zero
	b	L_0xbfc011c8
	 move	$v0, $zero
L_0xbfc011a8: /* 0xbfc011a8 */
	lui	$at, HI_UNSIGNED(0x00534c48)
	ori	$at, $at, LO_UNSIGNED(0x00534c48)
	bne	$s1, $at, L_0xbfc011c0
	 nop
	b	L_0xbfc011c8
	 li	$v0, 1
L_0xbfc011c0: /* 0xbfc011c0 */
	b	L_0xbfc01024
	 move	$s1, $zero
L_0xbfc011c8: /* 0xbfc011c8 */
	lw	$ra, 0x3c($sp)
	lw	$s0, 0x18($sp)
	lw	$s1, 0x1c($sp)
	lw	$s2, 0x20($sp)
	lw	$s3, 0x24($sp)
	lw	$s4, 0x28($sp)
	lw	$s5, 0x2c($sp)
	lw	$s6, 0x30($sp)
	lw	$s7, 0x34($sp)
	lw	$fp, 0x38($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x40

/* Function F_0xbfc011f8 [0xbfc011f8 - 0xbfc01268)
 */
F_0xbfc011f8: /* 0xbfc011f8 */
	lui	$v1, %hi(data_0xbfc01c10)
	lw	$v1, %lo(data_0xbfc01c10)($v1)
	lui	$t1, %hi(data_0xbfc01bfc)
	lbu	$v0, 0($v1)
	andi	$t6, $v0, 0x10
L_0xbfc0120c: /* 0xbfc0120c */
	beqz	$t6, L_0xbfc0121c
	 andi	$t7, $v0, 0xe
	jr	$ra
	 li	$v0, 0x3e00
L_0xbfc0121c: /* 0xbfc0121c */
	beqz	$t7, L_0xbfc01250
	 andi	$t0, $v0, 1
	andi	$t8, $v0, 8
	beqz	$t8, L_0xbfc01238
	 andi	$t9, $v0, 4
	jr	$ra
	 li	$v0, 0x4600
L_0xbfc01238: /* 0xbfc01238 */
	beqz	$t9, L_0xbfc01248
	 nop
	jr	$ra
	 li	$v0, 0x5000
L_0xbfc01248: /* 0xbfc01248 */
	jr	$ra
	 li	$v0, 0x4f00
L_0xbfc01250: /* 0xbfc01250 */
	beqzl	$t0, L_0xbfc0120c
	 andi	$t6, $v0, 0x10
	lw	$t1, %lo(data_0xbfc01bfc)($t1)
	lbu	$v0, 0($t1)
	jr	$ra
	 nop

/* Function F_0xbfc01268 [0xbfc01268 - 0xbfc012b4)
 */
F_0xbfc01268: /* 0xbfc01268 */
	lui	$v1, %hi(data_0xbfc01bfc)
	addiu	$v1, $v1, %lo(data_0xbfc01bfc)
	lw	$t6, 0x14($v1)
	sw	$a0, 0($sp)
	lui	$t9, %hi(data_0xbfc01bfc)
	lbu	$t7, 0($t6)
	andi	$a0, $a0, 0xff
	addiu	$t9, $t9, %lo(data_0xbfc01bfc)
	andi	$t8, $t7, 0x20
	bnez	$t8, L_0xbfc012a8
	 lui	$t1, %hi(data_0xbfc01bfc)
	lw	$t0, 0x14($t9)
	lbu	$v0, 0($t0)		# Load address from table (0xbf390407)
	andi	$v0, $v0, 0x20
L_0xbfc012a0: /* 0xbfc012a0 */
	beqz	$v0, L_0xbfc012a0
	 nop
L_0xbfc012a8: /* 0xbfc012a8 */
	lw	$t1, %lo(data_0xbfc01bfc)($t1)
	jr	$ra
	 sb	$a0, 0($t1)

/* Function F_0xbfc012b4 [0xbfc012b4 - 0xbfc01300)
 */
F_0xbfc012b4: /* 0xbfc012b4 */
	addiu	$sp, $sp, -0x20
	sw	$ra, 0x1c($sp)
	sw	$s1, 0x18($sp)
	sw	$s0, 0x14($sp)
	lbu	$s1, 0($a0)
	addiu	$s0, $a0, 1
	beqzl	$s1, L_0xbfc012f0
	 lw	$ra, 0x1c($sp)
L_0xbfc012d4: /* 0xbfc012d4 */
	jal	F_0xbfc01268
	 andi	$a0, $s1, 0xff
	lbu	$s1, 0($s0)
	addiu	$s0, $s0, 1
	bnez	$s1, L_0xbfc012d4
	 nop
	lw	$ra, 0x1c($sp)
L_0xbfc012f0: /* 0xbfc012f0 */
	lw	$s0, 0x14($sp)
	lw	$s1, 0x18($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x20

/* Function F_0xbfc01300 [0xbfc01300 - 0xbfc01358)
 */
F_0xbfc01300: /* 0xbfc01300 */
	addiu	$sp, $sp, -0x18
	sltiu	$at, $a0, 0xa
	sw	$ra, 0x14($sp)
	bnez	$at, L_0xbfc01330
	 move	$a1, $a0
	li	$at, 0xa
	divu	$zero, $a0, $at
	mflo	$a0
	sw	$a1, 0x18($sp)
	jal	F_0xbfc01300
	 nop
	lw	$a1, 0x18($sp)
L_0xbfc01330: /* 0xbfc01330 */
	li	$at, 0xa
	divu	$zero, $a1, $at
	mfhi	$a0
	addiu	$a0, $a0, 0x30
	jal	F_0xbfc01268
	 andi	$a0, $a0, 0xff
	lw	$ra, 0x14($sp)
	addiu	$sp, $sp, 0x18
	jr	$ra
	 nop

/* Function F_0xbfc01358 [0xbfc01358 - 0xbfc0138c)
 */
F_0xbfc01358: /* 0xbfc01358 */
	lui	$t6, %hi(data_0xbfc01c0c)
	lw	$t6, %lo(data_0xbfc01c0c)($t6)
	lui	$t9, %hi(data_0xbfc01c0c)
	beqz	$a0, L_0xbfc01378
	 lbu	$v0, 0($t6)
	ori	$t7, $v0, 1
	jr	$ra
	 sb	$t7, 0($t6)
L_0xbfc01378: /* 0xbfc01378 */
	lw	$t9, %lo(data_0xbfc01c0c)($t9)
	andi	$t8, $v0, 0xfffe
	sb	$t8, 0($t9)
	jr	$ra
	 nop

/* Function F_0xbfc0138c [0xbfc0138c - 0xbfc01394)
 */
F_0xbfc0138c: /* 0xbfc0138c */
	jr	$ra
	 nop

/* Function F_0xbfc01394 [0xbfc01394 - 0xbfc013c4)
 */
F_0xbfc01394: /* 0xbfc01394 */
	lbu	$v0, 0($a0)
	move	$v1, $zero
	addiu	$a0, $a0, 1
	beqz	$v0, L_0xbfc013bc
	 nop
	lbu	$v0, 0($a0)
L_0xbfc013ac: /* 0xbfc013ac */
	addiu	$a0, $a0, 1
	addiu	$v1, $v1, 1
	bnezl	$v0, L_0xbfc013ac
	 lbu	$v0, 0($a0)
L_0xbfc013bc: /* 0xbfc013bc */
	jr	$ra
	 move	$v0, $v1

/* Function F_0xbfc013c4 [0xbfc013c4 - 0xbfc01530)
 */
F_0xbfc013c4: /* 0xbfc013c4 */
	addiu	$v0, $a2, -1
	bltz	$v0, L_0xbfc01424
	 move	$a3, $a1
	lbu	$a1, 0($a0)
	lbu	$t6, 0($a3)
	addiu	$a3, $a3, 1
	xor	$v1, $a1, $t6
	sltiu	$v1, $v1, 1
	beqz	$v1, L_0xbfc01424
	 sltiu	$v1, $a1, 1
L_0xbfc013ec: /* 0xbfc013ec */
	beqz	$v1, L_0xbfc013fc
	 addiu	$a0, $a0, 1
	jr	$ra
	 move	$v0, $zero
L_0xbfc013fc: /* 0xbfc013fc */
	addiu	$v0, $v0, -1
	bltz	$v0, L_0xbfc01424
	 nop
	lbu	$a1, 0($a0)
	lbu	$t7, 0($a3)
	addiu	$a3, $a3, 1
	xor	$v1, $a1, $t7
	sltiu	$v1, $v1, 1
	bnezl	$v1, L_0xbfc013ec
	 sltiu	$v1, $a1, 1
L_0xbfc01424: /* 0xbfc01424 */
	bgezl	$v0, L_0xbfc01438
	 lbu	$t8, 0($a0)
	b	L_0xbfc01444
	 move	$v1, $zero
	lbu	$t8, 0($a0)		# unreachable
L_0xbfc01438: /* 0xbfc01438 */
	lbu	$t9, -1($a3)
	addiu	$a3, $a3, -1
	subu	$v1, $t8, $t9
L_0xbfc01444: /* 0xbfc01444 */
	move	$v0, $v1
	jr	$ra
	 nop
	addiu	$sp, $sp, -0x18		# unreachable
	lui	$t9, HI(ROM_START)		# unreachable
	sw	$ra, 0x14($sp)		# unreachable
	ori	$t9, $t9, 0x100		# unreachable
	li	$a0, 2		# unreachable
	jalr	$t9		# unreachable
	 move	$a1, $zero		# unreachable
	lw	$ra, 0x14($sp)		# unreachable
	addiu	$sp, $sp, 0x18		# unreachable
	jr	$ra		# unreachable
	 nop		# unreachable
	addiu	$sp, $sp, -0x18		# unreachable
	lui	$t9, HI(ROM_START)		# unreachable
	sw	$ra, 0x14($sp)		# unreachable
	ori	$t9, $t9, 0x100		# unreachable
	li	$a0, 3		# unreachable
	jalr	$t9		# unreachable
	 move	$a1, $zero		# unreachable
	lw	$ra, 0x14($sp)		# unreachable
	addiu	$sp, $sp, 0x18		# unreachable
	jr	$ra		# unreachable
	 nop		# unreachable
	addiu	$sp, $sp, -0x18		# unreachable
	lui	$t9, HI(ROM_START)		# unreachable
	sw	$ra, 0x14($sp)		# unreachable
	ori	$t9, $t9, 0x100		# unreachable
	li	$a0, 4		# unreachable
	jalr	$t9		# unreachable
	 move	$a1, $zero		# unreachable
	lw	$ra, 0x14($sp)		# unreachable
	addiu	$sp, $sp, 0x18		# unreachable
	jr	$ra		# unreachable
	 nop		# unreachable
	addiu	$sp, $sp, -0x18		# unreachable
	lui	$t9, HI(ROM_START)		# unreachable
	sw	$ra, 0x14($sp)		# unreachable
	ori	$t9, $t9, 0x100		# unreachable
	li	$a0, 5		# unreachable
	jalr	$t9		# unreachable
	 move	$a1, $zero		# unreachable
	lw	$ra, 0x14($sp)		# unreachable
	addiu	$sp, $sp, 0x18		# unreachable
	jr	$ra		# unreachable
	 nop		# unreachable; Load address from table (0xbf390407)
	addiu	$sp, $sp, -0x18		# unreachable
	lui	$t9, HI(ROM_START)		# unreachable
	sw	$ra, 0x14($sp)		# unreachable; $v0 = *(char *)0xbf390407
	ori	$t9, $t9, 0x100		# unreachable
	li	$a0, 6		# unreachable
	jalr	$t9		# unreachable; *(char *)0xbf390407 |= 1;
	 move	$a1, $zero		# unreachable
	lw	$ra, 0x14($sp)		# unreachable
	addiu	$sp, $sp, 0x18		# unreachable; *(char *)0xbf390407 &= ~1;
	jr	$ra		# unreachable
	 nop		# unreachable
nop2: /* 0xbfc0152c */
	nop		# unreachable

/* Function F_0xbfc01530 [0xbfc01530 - 0xbfc015b0)
 */
F_0xbfc01530: /* 0xbfc01530 */
	addiu	$sp, $sp, -0x28
strlen: /* 0xbfc01534 */
	sw	$s1, 0x1c($sp)
	move	$s1, $a1
	sw	$ra, 0x24($sp)
	sw	$s2, 0x20($sp)
	move	$s2, $a0
	sw	$s0, 0x18($sp)
	jal	F_0xbfc01618
	 move	$a1, $a2
	beqz	$v0, L_0xbfc01594
	 move	$s0, $v0
L_0xbfc0155c: /* 0xbfc0155c */
	jal	F_0xbfc01394
	 move	$a0, $s1

/* Function strncmp [0xbfc01564 - 0xbfc015b0)
 *
 * $a0 = string1; $a1 = string2; $a2 = len
 */
strncmp: /* 0xbfc01564 */
	lbu	$a2, 0x10($s0)
	move	$a0, $s1
	bnel	$v0, $a2, L_0xbfc01584
	 move	$a0, $s2
	jal	F_0xbfc013c4
	 addiu	$a1, $s0, 0x14
	beqz	$v0, L_0xbfc01594
	 move	$a0, $s2
L_0xbfc01584: /* 0xbfc01584 */
	jal	F_0xbfc01618
	 move	$a1, $s0
	bnez	$v0, L_0xbfc0155c
	 move	$s0, $v0
L_0xbfc01594: /* 0xbfc01594 */
	lw	$ra, 0x24($sp)
	move	$v0, $s0
	lw	$s0, 0x18($sp)
	lw	$s1, 0x1c($sp)
	lw	$s2, 0x20($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x28

/* Function F_0xbfc015b0 [0xbfc015b0 - 0xbfc01618)
 */
F_0xbfc015b0: /* 0xbfc015b0 */
	addiu	$sp, $sp, -0x28
	sw	$ra, 0x24($sp)
	sw	$s2, 0x20($sp)
	sw	$s1, 0x1c($sp)
	move	$s1, $a0
	move	$s2, $a1
	jal	F_0xbfc01530
	 sw	$s0, 0x18($sp)
	beqz	$v0, L_0xbfc015fc
	 move	$s0, $v0
L_0xbfc015d8: /* 0xbfc015d8 */
	jal	F_0xbfc016d4
	 move	$a0, $s0
	bnez	$v0, L_0xbfc015fc
	 move	$a0, $s1
	move	$a1, $s2
	jal	F_0xbfc01530

/* Function enter_interactive_mode [0xbfc015f0 - 0xbfc01618)
 */
enter_interactive_mode: /* 0xbfc015f0 */
	 move	$a2, $s0		# Calls exception_uncached_exception with $a0=2
	bnez	$v0, L_0xbfc015d8
	 move	$s0, $v0
L_0xbfc015fc: /* 0xbfc015fc */
	lw	$ra, 0x24($sp)
	move	$v0, $s0
	lw	$s0, 0x18($sp)
	lw	$s1, 0x1c($sp)
	lw	$s2, 0x20($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x28

/* Function F_0xbfc01618 [0xbfc01618 - 0xbfc0161c)
 */
F_0xbfc01618: /* 0xbfc01618 */
	addiu	$sp, $sp, -0x20

/* Function halt [0xbfc0161c - 0xbfc016d4)
 */
halt: /* 0xbfc0161c */
	sw	$s1, 0x18($sp)		# Calls exception_uncached_exception with $a0=3
	sw	$s0, 0x14($sp)
	move	$s1, $a0
	sw	$ra, 0x1c($sp)
	move	$s0, $a1
L_0xbfc01630: /* 0xbfc01630 */
	bnez	$s0, L_0xbfc01640
	 move	$a0, $s1
	b	L_0xbfc0167c
	 lw	$s0, 0($s1)
L_0xbfc01640: /* 0xbfc01640 */
	jal	F_0xbfc01754
	 move	$a1, $s0

/* Function powerdown [0xbfc01648 - 0xbfc016d4)
 */
powerdown: /* 0xbfc01648 */
	bnezl	$v0, L_0xbfc01660		# Calls exception_uncached_exception with $a0=4
	 lw	$v1, 8($s1)
	lw	$t6, 8($s1)
	b	L_0xbfc0167c
	 addu	$s0, $s0, $t6
	lw	$v1, 8($s1)		# unreachable
L_0xbfc01660: /* 0xbfc01660 */
	lw	$t7, 0xc($s0)
	addiu	$t9, $v1, -1
	addu	$t8, $s0, $t7
	addu	$v0, $t8, $v1
	addiu	$v0, $v0, -1

/* Function restart [0xbfc01674 - 0xbfc016d4)
 */
restart: /* 0xbfc01674 */
	not	$t0, $t9		# Calls exception_uncached_exception with $a0=5
	and	$s0, $v0, $t0
L_0xbfc0167c: /* 0xbfc0167c */
	lw	$v0, 4($s1)
	move	$a0, $s1
	sltu	$at, $s0, $v0
	beqzl	$at, L_0xbfc016a8
	 sltu	$at, $s0, $v0
	jal	F_0xbfc01754
	 move	$a1, $s0
	beqz	$v0, L_0xbfc01630
	 nop

/* Function reboot [0xbfc016a0 - 0xbfc016d4)
 */
reboot: /* 0xbfc016a0 */
	lw	$v0, 4($s1)		# Calls exception_uncached_exception with $a0=6
	sltu	$at, $s0, $v0
L_0xbfc016a8: /* 0xbfc016a8 */
	beqz	$at, L_0xbfc016bc
	 lw	$s1, 0x18($sp)
	lbu	$t1, 0x10($s0)
	bnezl	$t1, L_0xbfc016c4
	 lw	$ra, 0x1c($sp)
L_0xbfc016bc: /* 0xbfc016bc */
	move	$s0, $zero
	lw	$ra, 0x1c($sp)
L_0xbfc016c4: /* 0xbfc016c4 */
	move	$v0, $s0
	lw	$s0, 0x14($sp)
	jr	$ra
find_section_by_name: /* 0xbfc016d0 */
	 addiu	$sp, $sp, 0x20

/* Function F_0xbfc016d4 [0xbfc016d4 - 0xbfc01754)
 */
F_0xbfc016d4: /* 0xbfc016d4 */
	lw	$t6, 0xc($a0)
	li	$at, -4
	addiu	$v1, $a0, 0x40
	addu	$a1, $a0, $t6
	addiu	$a1, $a1, 3
	and	$a1, $a1, $at
	addiu	$t7, $a0, 0x40
	move	$v0, $v1
	beq	$t7, $a1, L_0xbfc0174c
	 move	$a2, $zero
	subu	$a3, $a1, $v1
	andi	$a3, $a3, 0xf
	beqz	$a3, L_0xbfc01724
	 addu	$a0, $a3, $v1
L_0xbfc0170c: /* 0xbfc0170c */
	lw	$t8, 0($v0)
	addiu	$v0, $v0, 4
	bne	$a0, $v0, L_0xbfc0170c
	 addu	$a2, $a2, $t8
	beq	$v0, $a1, L_0xbfc0174c
	 nop
L_0xbfc01724: /* 0xbfc01724 */
	lw	$t9, 0($v0)
	lw	$t0, 4($v0)
	lw	$t1, 8($v0)
	addu	$a2, $a2, $t9
	lw	$t2, 0xc($v0)
	addu	$a2, $a2, $t0
	addiu	$v0, $v0, 0x10
	addu	$a2, $a2, $t1
	bne	$v0, $a1, L_0xbfc01724
	 addu	$a2, $a2, $t2
L_0xbfc0174c: /* 0xbfc0174c */
	jr	$ra
find_valid_section_by_name: /* 0xbfc01750 */
	 sltiu	$v0, $a2, 1

/* Function F_0xbfc01754 [0xbfc01754 - 0xbfc01810)
 */
F_0xbfc01754: /* 0xbfc01754 */
	lw	$t6, 8($a0)
	addiu	$v1, $a1, 0x40
	move	$v0, $v1
	addiu	$t7, $t6, -1
	and	$t8, $a1, $t7
	bnez	$t8, L_0xbfc01798
	 nop
	lw	$t9, 8($a1)
	lui	$at, HI_UNSIGNED(SHDR_MAGIC)
	ori	$at, $at, LO_UNSIGNED(SHDR_MAGIC)
	bne	$t9, $at, L_0xbfc01798
	 nop
	lw	$t1, 0xc($a1)
	move	$a0, $zero
	andi	$t2, $t1, 3
	beqz	$t2, L_0xbfc017a0
	 nop
L_0xbfc01798: /* 0xbfc01798 */
	jr	$ra
	 move	$v0, $zero
L_0xbfc017a0: /* 0xbfc017a0 */
	beq	$a1, $v1, L_0xbfc017f8
	 move	$a2, $a1
	subu	$t0, $v1, $a1
	andi	$t0, $t0, 0xf
	beqz	$t0, L_0xbfc017d0
	 addu	$a3, $t0, $a1

/* Function find_next_section [0xbfc017b8 - 0xbfc01810)
 *
 * $a0 = pointer to memory containing base address;
 * $a1 = offset? (or 0 => use address from *$a0)
 * returns pointer to section or 0
 */
find_next_section: /* 0xbfc017b8 */
	lw	$t3, 0($a2)
	addiu	$a2, $a2, 4
	bne	$a3, $a2, find_next_section
	 addu	$a0, $a0, $t3
	beql	$a2, $v1, L_0xbfc017fc
	 sltiu	$v0, $a0, 1
L_0xbfc017d0: /* 0xbfc017d0 */
	lw	$t4, 0($a2)
	lw	$t5, 4($a2)
	lw	$t6, 8($a2)
	addu	$a0, $a0, $t4
	lw	$t7, 0xc($a2)
	addu	$a0, $a0, $t5
	addiu	$a2, $a2, 0x10
	addu	$a0, $a0, $t6		# Load section alignment requirement
	bne	$a2, $v0, L_0xbfc017d0		# Load section alignment requirement
	 addu	$a0, $a0, $t7
L_0xbfc017f8: /* 0xbfc017f8 */
	sltiu	$v0, $a0, 1
L_0xbfc017fc: /* 0xbfc017fc */
	jr	$ra
	 nop
	nop		# unreachable
	nop		# unreachable
	nop		# unreachable

/* Function F_0xbfc01810 [0xbfc01810 - 0xbfc019b0)
 */
F_0xbfc01810: /* 0xbfc01810 */
	lui	$t1, %hi(L_0xbfc01828)
	lui	$t0, HI(KSEG1)
	addiu	$t1, $t1, %lo(L_0xbfc01828)
	or	$t0, $t0, $t1
	jr	$t0
	 nop		# $s0 is within the system ROM?
L_0xbfc01828: /* 0xbfc01828 */
	mtc0	$zero, $CP0_PAGEMASK
	mtc0	$zero, $CP0_TAGHI
	li	$t1, 9
	nop
	mfc0	$t0, $CP0_PRID
	andi	$t0, $t0, 0xff00
	srl	$t0, $t0, 8
	beq	$t0, $t1, L_0xbfc01924		# $s0 is within the system ROM?
	 nop
	li	$t0, 0x2f
	lui	$t1, HI(0x0fffe000)
L_0xbfc01854: /* 0xbfc01854 */
	li	$at, 0x1fff
	not	$t2, $at
	and	$t2, $t2, $t1
	lui	$at, HI(KSEG0)
	mtc0	$t0, $CP0_INDEX
	or	$t2, $t2, $at
	mtc0	$t2, $CP0_ENTRYHI
	srl	$at, $t1, 0xc
is_section_checksum_valid: /* 0xbfc01874 */
	sll	$at, $at, 6		# $t6 = Load the length of the section
	ori	$at, $at, 0x11
	mtc0	$at, $CP0_ENTRYLO0		# $v1 = address of end of SHDR
	addiu	$t2, $at, 0x40		# $a1 = address of end of section
	mtc0	$at, $CP0_ENTRYLO1
	addi	$t0, $t0, -1
	addiu	$t1, $t1, LO(0x0fffe000)
	bgtz	$t0, L_0xbfc01854		# $v0 = address of data to be checksummed
	 tlbwi
	mfc0	$t0, $CP0_CONFIG		# $a2 = checksum
	li	$at, -0x1001
	and	$t0, $t0, $at
	mtc0	$t0, $CP0_CONFIG
	mfc0	$t0, $CP0_CONFIG
checksum_remainder_loop: /* 0xbfc018ac */
	srl	$t0, $t0, 9
	li	$t1, 0x1000
	andi	$t0, $t0, 7
	sllv	$t0, $t1, $t0
	addi	$t0, $t0, -0x20
	lui	$t1, HI(KSEG0)
checksum_main_loop: /* 0xbfc018c4 */
	addu	$t2, $t0, $t1		# $t9 = word[0]
L_0xbfc018c8: /* 0xbfc018c8 */
	cache	(CACHE_TYPE_L1I|INDEX_WRITEBACK_INV), 0($t2)		# $t0 = word[1]
	addi	$t0, $t0, -0x20		# $t1 = word[2]
	bgez	$t0, L_0xbfc018c8		# checksm += word[0]
	 addu	$t2, $t0, $t1		# $t2 = word[3]
	mfc0	$t0, $CP0_CONFIG		# checksum += word[1]
	srl	$t0, $t0, 6		# word += 16
	li	$t1, 0x1000		# checksum += word[2]
	andi	$t0, $t0, 7		# branch while not at end
	sllv	$t0, $t1, $t0		# checksum += word[3]
checksum_done: /* 0xbfc018ec */
	addi	$t0, $t0, -0x20
	lui	$t1, HI(KSEG0)		# return checksum == 0
is_section_valid: /* 0xbfc018f4 */
	lui	$at, 0x1000
L_0xbfc018f8: /* 0xbfc018f8 */
	addu	$at, $at, $t0
	srl	$at, $at, 0xc
	sll	$at, $at, 8
	mtc0	$at, $CP0_TAGLO
	addu	$t2, $t0, $t1
	addi	$t0, $t0, -0x20
	cache	(CACHE_TYPE_L1D|INDEX_STORE_TAG), 0($t2)
	bgez	$t0, L_0xbfc018f8
	 lui	$at, 0x1000
	jr	$ra		# if (memcmp(&shdr->magic, "SHDR", sizeof(shdr->magic)) != 0) return 0;
	 nop
L_0xbfc01924: /* 0xbfc01924 */
	li	$t0, 0x3f
	lui	$t1, HI(0x0fffe000)
L_0xbfc0192c: /* 0xbfc0192c */
	li	$at, 0x1fff
	not	$t2, $at		# if ((shdr->section_len & 0x3) != 0) return 0;
	and	$t2, $t2, $t1
	lui	$at, HI(KSEG1)
	mtc0	$t0, $CP0_INDEX
	or	$t2, $t2, $at		# $a1 & $v1 are different and const, so this is a dead branch
	mtc0	$t2, $CP0_ENTRYHI
	srl	$at, $t1, 0xc
	sll	$at, $at, 6
	ori	$at, $at, 0x11		# This branch is always taken
	mtc0	$at, $CP0_ENTRYLO0
verify_shdr_checksum_loop: /* 0xbfc01958 */
	addiu	$t2, $at, 0x40
	mtc0	$at, $CP0_ENTRYLO1
	addi	$t0, $t0, -1
	addiu	$t1, $t1, LO(0x0fffe000)
	bgtz	$t0, L_0xbfc0192c
	 tlbwi
verify_section_checksum_loop: /* 0xbfc01970 */
	mtc0	$zero, $CP0_TAGHI
	mtc0	$zero, $CP0_TAGLO
	mtc0	$zero, $CP0_ECC
	lui	$t0, HI(KSEG0)
	li	$t1, 0x3fc0
L_0xbfc01984: /* 0xbfc01984 */
	or	$t2, $t0, $t1
	cache	(CACHE_TYPE_L1I|INDEX_STORE_TAG), 0($t2)
	addi	$t1, $t1, -0x40
	cache	(CACHE_TYPE_L1I|INDEX_STORE_TAG), 1($t2)
	bgez	$t1, L_0xbfc01984
	 nop
	mtc0	$zero, $CP0_TAGHI
	mtc0	$zero, $CP0_TAGLO
	mtc0	$zero, $CP0_ECC
	lui	$t0, HI(KSEG0)
	li	$t1, 0x3fe0

/* Function tlb_init_uncached_trampoline [0xbfc019b0 - 0xbfc019c8)
 *
 * Jump to tlb_init through uncached KSEG1
 */
tlb_init_uncached_trampoline: /* 0xbfc019b0 */
	or	$t2, $t0, $t1
	cache	(CACHE_TYPE_L1D|INDEX_STORE_TAG), 0($t2)
	addi	$t1, $t1, -0x20
	cache	(CACHE_TYPE_L1D|INDEX_STORE_TAG), 1($t2)
	bgez	$t1, tlb_init_uncached_trampoline		# Jump to (KSEG1 | tlb_init)
	 nop

/* Function tlb_init [0xbfc019c8 - 0xbfc01d98)
 */
tlb_init: /* 0xbfc019c8 */
	mfc0	$t0, $CP0_CONFIG
	lui	$t1, HI(ROM_SIZE)
	srl	$t2, $t0, 0x10
	andi	$t2, $t2, 7
L_0xbfc019d8: /* 0xbfc019d8 */
	slti	$at, $t2, 6
	beqz	$at, L_0xbfc019d8
	 nop
	sllv	$t0, $t1, $t2
	andi	$at, $t0, 0x2000
	li	$t2, 0x80
	bgtz	$at, L_0xbfc019fc
	 nop
	li	$t2, 0x40
L_0xbfc019fc: /* 0xbfc019fc */
	mtc0	$zero, $CP0_TAGHI
	mtc0	$zero, $CP0_TAGLO
	srl	$t1, $t0, 1
	mtc0	$zero, $CP0_ECC
tlb_rm7k_write_tlb_loop: /* 0xbfc01a0c */
	sub	$t1, $t1, $t2
L_0xbfc01a10: /* 0xbfc01a10 */
	lui	$t3, HI(KSEG0)
	or	$t3, $t3, $t1
	cache	(CACHE_TYPE_L2|INDEX_STORE_TAG), 0($t3)
	sub	$t1, $t1, $t2
	cache	(CACHE_TYPE_L2|INDEX_STORE_TAG), 1($t3)
	bgez	$t1, L_0xbfc01a10
	 nop
	mtc0	$zero, $CP0_ECC
	mtc0	$zero, $CP0_TAGHI
	mtc0	$zero, $CP0_TAGLO
	nop
	move	$t0, $zero
L_0xbfc01a40: /* 0xbfc01a40 */
	lui	$t1, HI(KSEG0)
	or	$t1, $t1, $t0
	cache	(CACHE_TYPE_L1D|HIT_SET_VIRTUAL), 0($t1)
	addi	$t0, $t0, 4
	li	$t2, 0x4000
	cache	(CACHE_TYPE_L1D|HIT_SET_VIRTUAL), 1($t1)
	slt	$at, $t0, $t2
	bnez	$at, L_0xbfc01a40
	 nop
	mtc0	$zero, $CP0_ECC
	mtc0	$zero, $CP0_TAGHI
	mtc0	$zero, $CP0_TAGLO
	nop
	move	$t0, $zero
L_0xbfc01a78: /* 0xbfc01a78 */
	lui	$t1, HI(KSEG0)
	or	$t1, $t1, $t0
	cache	(CACHE_TYPE_L1I|HIT_SET_VIRTUAL), 0($t1)
	addi	$t0, $t0, 4
tlb_rm7k_inv_l1i_loop: /* 0xbfc01a88 */
	li	$t2, 0x4000
	cache	(CACHE_TYPE_L1I|HIT_SET_VIRTUAL), 1($t1)
	slt	$at, $t0, $t2
	bnez	$at, L_0xbfc01a78
	 nop
	mfc0	$t0, $CP0_CONFIG
	lui	$t1, HI(ROM_SIZE)
	srl	$t2, $t0, 0x10
	andi	$t2, $t2, 7
L_0xbfc01aac: /* 0xbfc01aac */
	slti	$at, $t2, 6
	beqz	$at, L_0xbfc01aac
	 nop
tlb_rm7k_inv_l1d_loop: /* 0xbfc01ab8 */
	sllv	$t0, $t1, $t2
	mtc0	$zero, $CP0_ECC
	srl	$t2, $t0, 1
	mtc0	$zero, $CP0_TAGHI
	mtc0	$zero, $CP0_TAGLO
	move	$t0, $zero
L_0xbfc01ad0: /* 0xbfc01ad0 */
	lui	$at, HI(KSEG0)
	or	$t1, $t0, $at
	cache	(CACHE_TYPE_L2|HIT_SET_VIRTUAL), 0($t1)
	addi	$t0, $t0, 0x10
	cache	(CACHE_TYPE_L2|HIT_SET_VIRTUAL), 1($t1)
tlb_r5k_init: /* 0xbfc01ae4 */
	slt	$at, $t0, $t2
	bnez	$at, L_0xbfc01ad0
tlb_r5k_write_tlb_loop: /* 0xbfc01aec */
	 nop
	b	L_0xbfc01af8
	 nop
L_0xbfc01af8: /* 0xbfc01af8 */
	jr	$ra
	 nop
	.int	0x27bdffd0
	.int	0xafbf002c
	.int	0xafa10028
	.int	0xafa80024
	.int	0xafa90020
	.int	0xafaa001c
	.int	0xafab0018
	.int	0x0411ff3c
	.int	0xafac0014
	.int	0x8fbf002c
	.int	0x8fa10028
	.int	0x8fa80024
	.int	0x8fa90020
	.int	0x8faa001c
	.int	0x8fab0018
	.int	0x8fac0014
	.int	0x03e00008
	.int	0x27bd0030
	.fill	2, 4, 0x00000000
data_0xbfc01b50: /* 0xbfc01b50 */
	string 4, "post1"
data_0xbfc01b58: /* 0xbfc01b58 */
	.ascii	"firmware"
tlb_r5k_inv_l1i_loop: /* 0xbfc01b60 */
	.fill	4, 1, 0
data_0xbfc01b64: /* 0xbfc01b64 */
	.int	0x06050308
	.int	0x03080808
data_0xbfc01b6c: /* 0xbfc01b6c */
	.fill	2, 4, 0x00000000
	.int	0x011c0002
	.int	0x00000002
	.int	0x01180004
	.int	0x00000004
	.int	0x011a0004
	.int	0x00000005
	.int	0x011d0002
tlb_r5k_inv_l1d_loop: /* 0xbfc01b90 */
	.int	0x00000000
	.int	0x011e0002
	.int	0x00000007
	.int	0x011f0000
	.int	0x00000005
	.fill	4, 4, 0x00000000
data_0xbfc01bb4: /* 0xbfc01bb4 */
	string 4, "\n\rSL-"
tlb_r10k_init: /* 0xbfc01bbc */
	string 4, "-8E"
data_0xbfc01bc0: /* 0xbfc01bc0 */
	.int	0x000012c0
tlb_r10k_write_tlb_loop: /* 0xbfc01bc4 */
	.int	0x00002580
	.int	0x00004b00
	.int	0x00009600
	.int	0x0000e100
	.int	0x0001c200
	.fill	2, 4, 0x00000000
data_0xbfc01be0: /* 0xbfc01be0 */
	.int	F_0xbfc00000
	.int	ROM_END
	.int	0x00000100
	.int	ROM_SIZE
	.int	F_0xbfc00aa8
	.int	F_0xbfc00ad0
	.int	F_0xbfc00ad8
data_0xbfc01bfc: /* 0xbfc01bfc */
	.int	BASE_UART_1 + UART_DATA
data_0xbfc01c00: /* 0xbfc01c00 */
	.int	BASE_UART_1 + UART_IER
data_0xbfc01c04: /* 0xbfc01c04 */
	.int	BASE_UART_1 + UART_IIR
data_0xbfc01c08: /* 0xbfc01c08 */
	.int	BASE_UART_1 + UART_LCR
data_0xbfc01c0c: /* 0xbfc01c0c */
	.int	BASE_UART_1 + UART_MCR
data_0xbfc01c10: /* 0xbfc01c10 */
	.int	BASE_UART_1 + UART_LSR
	.int	BASE_UART_1 + UART_MSR
	.int	0x00000000
tlb_r10k_inv_l1i_loop: /* 0xbfc01c1c */
	.fill	11, 4, 0x00000000
tlb_r10k_inv_l1d_loop: /* 0xbfc01c48 */
	.fill	24, 4, 0x00000000
tlb_r10k_inv_l2_loop: /* 0xbfc01ca8 */
	.fill	12, 4, 0x00000000
tlb_r10k_set_virt_l1d_loop: /* 0xbfc01cd8 */
	.fill	14, 4, 0x00000000
tlb_r10k_set_virt_l1i_loop: /* 0xbfc01d10 */
	.fill	22, 4, 0x00000000
tlb_r10k_set_virt_l2_loop: /* 0xbfc01d68 */
	.fill	10, 4, 0x00000000
tlb_r10k_exit: /* 0xbfc01d90 */
	.fill	2, 4, 0x00000000

/* Function tlb_init_preserve [0xbfc01d98 - 0xbfc04000)
 */
tlb_init_preserve: /* 0xbfc01d98 */
	nop		# unused?
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop
	nop

	section_checksum
	section_end sloader
