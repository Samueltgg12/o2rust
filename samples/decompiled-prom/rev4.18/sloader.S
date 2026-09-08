/* Section: sloader */
/* Version: 1.0 */
/* Type: SECTION_TYPE_CODE */
/* Length: 0x4000 (16384 bytes) */

#include "definitions.h"
#include "macros.inc"

.set noreorder
.set noat


/* Function F_0xbfc00000 [0xbfc00000 - 0xbfc008f0)
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

/* Function exception_uncached_exception [0xbfc00100 - 0xbfc00770)
 */
exception_uncached_exception: /* 0xbfc00100 */
	mtc0	$a0, $CP0_LLADDR
	lui	$t8, HI(BASE_CRIME + CRIME_REPLACEMENT_CHECK_BITS)
	sd	$a0, LO(BASE_CRIME + CRIME_REPLACEMENT_CHECK_BITS)($t8)
	lui	$t8, HI(BASE_CRIME + CRIME_MC_STATUS_CTRL)
	addiu	$t8, $t8, LO(BASE_CRIME + CRIME_MC_STATUS_CTRL)
	b	L_0xbfc00520
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
	mfc0	$k0, $CP0_STATUS
	srl	$k0, $k0, 0x10
	andi	$k0, $k0, HI_UNSIGNED(ST0_NMI|ST0_SR)
	addi	$k0, $k0, -HI_UNSIGNED(ST0_NMI|ST0_SR)
	beqz	$k0, warm_start		# Branch to warm_start if (ST0_NMI|ST0_SR) set
	 nop
	lui	$k0, HI(BASE_CRIME + CRIME_CONTROL_OFFSET)
	addiu	$k0, $k0, LO(BASE_CRIME + CRIME_CONTROL_OFFSET)
	ld	$k0, 0($k0)
	andi	$k0, $k0, (CRIME_CONTROL_TRITON_SYSADC|CRIME_CONTROL_CRIME_SYSADC)
	addi	$k0, $k0, -(CRIME_CONTROL_TRITON_SYSADC|CRIME_CONTROL_CRIME_SYSADC)
	beqz	$k0, warm_start		# Branch to warm_start if (CRIME_CONTROL_TRITON_SYSADC|CRIME_CONTROL_CRIME_SYSADC) set
	 nop
	move	$at, $zero		# Initialize GPRs to 0
	move	$v0, $zero
	move	$v1, $zero
	move	$a0, $zero
	move	$a1, $zero
	move	$a2, $zero
	move	$a3, $zero
	move	$t0, $zero
	move	$t1, $zero
	move	$t2, $zero
	move	$t3, $zero
	move	$t4, $zero
	move	$t5, $zero
	move	$t6, $zero
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
	mfc0	$v1, $CP0_STATUS		# Save status register
	nop
	nop
	lui	$a1, HI_UNSIGNED(ST0_FR|ST0_CU1)
	mtc0	$a1, $CP0_STATUS		# Enable floating-point unit
	nop
	nop
	nop
	mtc1	$zero, $f0		# Initialize FPRs to 0.0
	mtc1	$zero, $f1
	mtc1	$zero, $f2
	mtc1	$zero, $f3
	mtc1	$zero, $f4
	mtc1	$zero, $f5
	mtc1	$zero, $f6
	mtc1	$zero, $f7
	mtc1	$zero, $f8
	mtc1	$zero, $f9
	mtc1	$zero, $f10
	mtc1	$zero, $f11
	mtc1	$zero, $f12
	mtc1	$zero, $f13
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
	ctc1	$zero, $CP1_FCSR		# Initialize FPU control register
	nop
	nop
	ctc1	$zero, $CP1_FCSR		# ... Twice?
	nop
	nop
	mtc0	$v1, $CP0_STATUS		# Restore status register
	nop
L_0xbfc00520: /* 0xbfc00520 */
	lui	$t8, HI(BASE_RTC + RTC_NVRAM(0x29) + BYTE_OFFSET)
	sb	$t9, LO(BASE_RTC + RTC_NVRAM(0x29) + BYTE_OFFSET)($t8)		# *0xbf3a3707 = (t9 >>  0) & 0xff
	srl	$t9, $t9, 8
	sb	$t9, LO(BASE_RTC + RTC_NVRAM(0x2a) + BYTE_OFFSET)($t8)		# *0xbf3a3807 = (t9 >>  8) & 0xff
	srl	$t9, $t9, 8
	sb	$t9, LO(BASE_RTC + RTC_NVRAM(0x2b) + BYTE_OFFSET)($t8)		# *0xbf3a3907 = (t9 >> 16) & 0xff
	srl	$t9, $t9, 8
	sb	$t9, LO(BASE_RTC + RTC_NVRAM(0x2c) + BYTE_OFFSET)($t8)		# *0xbf3a3a07   (t9 >> 24) & 0xff
	srl	$t9, $t9, 8
	sb	$t9, LO(BASE_RTC + RTC_NVRAM(0x2d) + BYTE_OFFSET)($t8)		# *0xbf3a3b07   (t9 >> 32) & 0xff
	srl	$t9, $t9, 8
	sb	$t9, LO(BASE_RTC + RTC_NVRAM(0x2e) + BYTE_OFFSET)($t8)		# *0xbf3a3d07   (t9 >> 48) & 0xff
	srl	$t9, $t9, 8
	sb	$t9, LO(BASE_RTC + RTC_NVRAM(0x2f) + BYTE_OFFSET)($t8)
	srl	$t9, $t9, 8
	addiu	$t8, $t8, LO(BASE_RTC + BYTE_OFFSET)
	sb	$t9, LO(BASE_RTC + RTC_NVRAM(0x30))($t8)		# *0xbf3a3e07   (t9 >> 56) & 0xff
	lui	$t8, HI(BASE_ISA + ISA_MISC_CONTROL)
	ld	$t9, LO(BASE_ISA + ISA_MISC_CONTROL)($t8)
	addiu	$t8, $t8, LO(BASE_ISA + ISA_MISC_CONTROL)
	li	$t8, ISA_RED_LED|ISA_GREEN_LED
	not	$t8, $t8
	and	$t9, $t9, $t8		# Turn on Green and Red LEDs
	lui	$t8, HI(BASE_ISA + ISA_MISC_CONTROL)
	addiu	$t8, $t8, LO(BASE_ISA + ISA_MISC_CONTROL)
	sd	$t9, 0($t8)
	mfc0	$t8, $CP0_STATUS
	lui	$t9, HI_UNSIGNED(ST0_BEV)
	and	$t8, $t8, $t9
	bnez	$t8, initialize_0		# Branch to 0x5f0 if in bootstrap
	 nop
	lui	$t8, HI_UNSIGNED(ST0_BEV)
	mtc0	$t8, $CP0_STATUS		# Set bootstrap mode
	li	$t9, 2
	beq	$a0, $t9, L_0xbfc00628		# Branch if a0 is 2, 3, 4, 5, 7, 6
	 li	$t8, 3
	beq	$a0, $t8, L_0xbfc00628
	 li	$t9, 4
	beq	$a0, $t9, L_0xbfc00628
	 li	$t8, 1
	beq	$a0, $t8, soft_reset		# Branch if a0 is 1
	 li	$t9, 5
	beq	$a0, $t9, L_0xbfc00628
	 li	$t8, 0
	beq	$a0, $t8, hard_reset		# Branch if a0 is 0
	 li	$t9, 7
	beq	$a0, $t9, L_0xbfc00628
	 li	$t8, 6
	beq	$a0, $t8, L_0xbfc00628
	 nop
	b	hard_reset
	 nop
initialize_0: /* 0xbfc005f0 */
	lui	$t8, HI(BASE_RTC + RTC_NVRAM(0x31) + BYTE_OFFSET)
	lb	$t9, LO(BASE_RTC + RTC_NVRAM(0x31) + BYTE_OFFSET)($t8)
	addiu	$t8, $t8, LO(BASE_RTC + BYTE_OFFSET)
	addi	$t9, $t9, 1
	sb	$t9, LO(BASE_RTC + RTC_NVRAM(0x31))($t8)
	mfc0	$t8, $CP0_STATUS
	lui	$t9, HI_UNSIGNED(ST0_SR)
	and	$t8, $t8, $t9
	bnez	$t8, L_0xbfc00708
	 nop
	li	$a0, 0
	mtc0	$a0, $CP0_LLADDR
	b	L_0xbfc00628
	 nop
L_0xbfc00628: /* 0xbfc00628 */
	move	$s7, $a0
	lui	$v0, HI_UNSIGNED(ST0_CU1|ST0_CU0|ST0_FR|ST0_BEV|ST0_DE|ST0_KX)
	ori	$v0, $v0, LO_UNSIGNED(ST0_CU1|ST0_CU0|ST0_FR|ST0_BEV|ST0_DE|ST0_KX)
	mtc0	$v0, $CP0_STATUS
	nop
	mfc0	$t0, $CP0_CONFIG
	li	$v0, CONF_CM_CMASK
	not	$v0, $v0
	and	$t0, $t0, $v0
	ori	$t0, $t0, CONF_CM_CACHABLE_NONCOHERENT
	mtc0	$t0, $CP0_CONFIG		# Set KSEG0 to cacheable, noncoherent
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
	lui	$t9, HI(BASE_ISA + ISA_RING_BASE_AND_RESET)
	addiu	$t9, $t9, LO(BASE_ISA + ISA_RING_BASE_AND_RESET)
	li	$t8, 0x400
	addiu	$at, $t9, 0x7fff
	sd	$t8, 1($at)		# *(long *)0xbf318000 = 0x400
	addiu	$at, $t9, 0x7fff
	sd	$t8, 0x21($at)		# *(long *)0xbf318020 = 0x400
	addiu	$at, $t9, 0x7fff
	sd	$t8, 0x4001($at)		# *(long *)0xbf31c000 = 0x400
	addiu	$at, $t9, 0x7fff
	sd	$t8, 0x4021($at)		# *(long *)0xbf31c020 = 0x400
	lui	$t8, HI(BASE_ISA + ISA_RING_BASE_AND_RESET)
	li	$t9, ISA_RESET
	sd	$t9, LO(BASE_ISA + ISA_RING_BASE_AND_RESET)($t8)		# Reset ISA controller
	addiu	$t8, $t8, LO(BASE_ISA + ISA_RING_BASE_AND_RESET)
	li	$t9, 0x100
isa_reset_delay_loop: /* 0xbfc006c4 */
	addi	$t9, $t9, -1
	bnez	$t9, isa_reset_delay_loop		# Delay... count down from 256
	 nop
	sd	$t9, LO(BASE_ISA + ISA_RING_BASE_AND_RESET)($t8)		# ISA_RING_BASE_AND_RESET; take out of reset state
	jal	tlb_init_uncached_trampoline
	 nop
	lui	$s0, HI(SLOADER_STACK)
	jal	cache_init
	 addi	$sp, $s0, LO(SLOADER_STACK)		# $sp = 0x80001000
	lui	$t0, HI(BASE_RTC + RTC_NVRAM(0x31) + BYTE_OFFSET)
	lb	$a1, LO(BASE_RTC + RTC_NVRAM(0x31) + BYTE_OFFSET)($t0)
	move	$a0, $s7
	addiu	$t0, $t0, LO(BASE_RTC + BYTE_OFFSET)
	jal	F_0xbfc00a80
	 addi	$sp, $sp, -8
	b	F_0xbfc00000
	 nop
L_0xbfc00708: /* 0xbfc00708 */
	lui	$t8, HI(BASE_RTC + RTC_NVRAM(0x31) + BYTE_OFFSET)
	lbu	$t9, LO(BASE_RTC + RTC_NVRAM(0x31) + BYTE_OFFSET)($t8)
	addiu	$t8, $t8, LO(BASE_RTC + BYTE_OFFSET)
	li	$t8, 1
	beq	$t8, $t9, L_0xbfc0072c
	 nop
	b	L_0xbfc00628
	 nop
	li	$a0, 0		# unreachable
L_0xbfc0072c: /* 0xbfc0072c */
	b	L_0xbfc00628
	 nop
	li	$a0, 1		# unreachable
soft_reset: /* 0xbfc00738 */
	lui	$a0, HI(BASE_CRIME + CRIME_CONTROL_OFFSET)
	li	$a1, CRIME_CONTROL_SOFT_RESET
	addiu	$a0, $a0, LO(BASE_CRIME + CRIME_CONTROL_OFFSET)
	bal	wait_for_reset
	 sd	$a1, 0($a0)
	b	soft_reset
	 nop
hard_reset: /* 0xbfc00754 */
	lui	$a0, HI(BASE_CRIME + CRIME_CONTROL_OFFSET)
	li	$a1, CRIME_CONTROL_HARD_RESET
	addiu	$a0, $a0, LO(BASE_CRIME + CRIME_CONTROL_OFFSET)
	bal	wait_for_reset
	 sd	$a1, 0($a0)
	b	hard_reset
	 nop

/* Function wait_for_reset [0xbfc00770 - 0xbfc0079c)
 */
wait_for_reset: /* 0xbfc00770 */
	lui	$a0, HI(BASE_ISA + ISA_MISC_CONTROL)
	ld	$a1, LO(BASE_ISA + ISA_MISC_CONTROL)($a0)
	lui	$a2, HI(ROM_SIZE)
	addiu	$a0, $a0, LO(BASE_ISA + ISA_MISC_CONTROL)
	xori	$a1, $a1, ISA_RED_LED|ISA_GREEN_LED
	sd	$a1, 0($a0)		# ISA_MISC_CONTROL
	dsll32	$a2, $a2, 0
L_0xbfc0078c: /* 0xbfc0078c */
	bgtz	$a2, L_0xbfc0078c
	 addi	$a2, $a2, -1		# Delay... count down from 8<<48
	jr	$ra
	 nop

/* Function F_0xbfc0079c [0xbfc0079c - 0xbfc007d8)
 */
F_0xbfc0079c: /* 0xbfc0079c */
	b	L_0xbfc007d0
	 nop

/* Function F_0xbfc007a4 [0xbfc007a4 - 0xbfc007e8)
 */
F_0xbfc007a4: /* 0xbfc007a4 */
	move	$t0, $s0
L_0xbfc007a8: /* 0xbfc007a8 */
	addi	$t0, $t0, 4
	bne	$sp, $t0, L_0xbfc007a8
	 sw	$a0, -4($t0)
	move	$t0, $s0
L_0xbfc007b8: /* 0xbfc007b8 */
	lw	$t1, 0($t0)
	bne	$t1, $a0, L_0xbfc007d8
	 nop
	addi	$t0, $t0, 4
	bne	$t0, $sp, L_0xbfc007b8
	 nop
L_0xbfc007d0: /* 0xbfc007d0 */
	jr	$ra
	 nop
L_0xbfc007d8: /* 0xbfc007d8 */
	beqz	$fp, crash
	 nop
	b	L_0xbfc007d8
	 nop
warm_start: /* 0xbfc007e8 */
	mfc0	$k0, $CP0_LLADDR
	addi	$k0, $k0, -WARM_START_COOKIE
	beqz	$k0, L_0xbfc00884
	 nop
	li	$k0, WARM_START_COOKIE
	mtc0	$k0, $CP0_LLADDR
	nop
	nop
	nop
	nop
	mfc0	$k1, $CP0_ERROREPC
	nop
	nop
	nop
	nop
	lui	$k0, HI_UNSIGNED(GDA_ADDR)
	ori	$k0, $k0, LO_UNSIGNED(GDA_ADDR)
	sw	$k1, 0xc($k0)
	nop
	nop
	nop
	nop
	mfc0	$k1, $CP0_STATUS
	nop
	nop
	nop
	nop
	sw	$k1, 4($k0)
	nop
	nop
	nop
	nop
	lui	$k0, HI_UNSIGNED(ST0_CU1|ST0_CU0|ST0_FR|ST0_BEV|ST0_DE|ST0_KX)
	ori	$k0, $k0, LO_UNSIGNED(ST0_CU1|ST0_CU0|ST0_FR|ST0_BEV|ST0_DE|ST0_KX)
	mtc0	$k0, $CP0_STATUS
	nop
	nop
	nop
	nop
L_0xbfc00884: /* 0xbfc00884 */
	lui	$k0, HI_UNSIGNED(BASE_CRIME | CRIME_TIMER_OFFSET)
	ori	$k0, $k0, LO_UNSIGNED(BASE_CRIME | CRIME_TIMER_OFFSET)
	ld	$k1, 0($k0)
	lui	$k0, HI_UNSIGNED(133333000)
	ori	$k0, $k0, LO_UNSIGNED(133333000)
	daddu	$k1, $k1, $k0
L_0xbfc0089c: /* 0xbfc0089c */
	lui	$k0, HI_UNSIGNED(BASE_CRIME | CRIME_TIMER_OFFSET)
	ori	$k0, $k0, LO_UNSIGNED(BASE_CRIME | CRIME_TIMER_OFFSET)
	ld	$k0, 0($k0)
	dsub	$k0, $k1, $k0
	bgtz	$k0, L_0xbfc0089c
	 nop
	lui	$k0, HI_UNSIGNED(GDA_ADDR)
	ori	$k0, $k0, LO_UNSIGNED(GDA_ADDR)
	lw	$k0, 0($k0)
	lui	$k1, HI_UNSIGNED(GDA_MAGIC)
	ori	$k1, $k1, LO_UNSIGNED(GDA_MAGIC)
	bne	$k0, $k1, hard_reset
	 nop
	lui	$k0, HI_UNSIGNED(GDA_ADDR)
	ori	$k0, $k0, LO_UNSIGNED(GDA_ADDR)
	lw	$k0, 8($k0)
	jr	$k0
	 nop
	nop		# unreachable
	nop		# unreachable
	nop		# unreachable

/* Function cache_init [0xbfc008f0 - 0xbfc00a80)
 */
cache_init: /* 0xbfc008f0 */
	mfc0	$t0, $CP0_PRID
	andi	$t0, $t0, PRID_IMP_MASK
	li	$t1, PRID_IMP_R5000
	srl	$t0, $t0, PRID_IMP_SHIFT
	beq	$t0, $t1, cache_init_r5k
	 nop
	li	$t1, PRID_IMP_NEVADA
	beq	$t0, $t1, cache_init_r5k
	 nop
	li	$t1, PRID_IMP_RM7000
	bne	$t0, $t1, cache_init_r10k
	 nop
	move	$t0, $s0
cache_init_rm7k_loop: /* 0xbfc00924 */
	sll	$at, $t0, 3
	srl	$at, $at, 0xf
	srl	$at, $at, 8
	ori	$at, $at, RM7K_TAGLO_DIRTY
	mtc0	$at, $CP0_TAGLO
	nop
	nop
	nop
	nop
	srl	$at, $at, 8
	sll	$at, $at, 8
	mtc0	$at, $CP0_TAGHI
	nop
	nop
	nop
	nop
	move	$t1, $t0
	addiu	$t0, $t0, CACHE_LINE_SIZE
	cache	(CACHE_TYPE_L1D|INDEX_STORE_TAG), 0($t1)
	sltu	$at, $sp, $t0
	beqz	$at, cache_init_rm7k_loop
	 nop
	b	L_0xbfc00a58
	 nop
cache_init_r5k: /* 0xbfc00984 */
	mtc0	$zero, $CP0_TAGHI
	move	$t0, $s0
cache_init_r5k_loop: /* 0xbfc0098c */
	sll	$at, $t0, 3
	srl	$at, $at, 0xf
	srl	$at, $at, 8
	ori	$at, $at, R5K_TAGLO_DIRTY
	mtc0	$at, $CP0_TAGLO
	move	$t1, $t0
	addiu	$t0, $t0, CACHE_LINE_SIZE
	cache	(CACHE_TYPE_L1D|INDEX_STORE_TAG), 0($t1)
	sltu	$at, $sp, $t0
	beqz	$at, cache_init_r5k_loop
	 nop
	b	L_0xbfc00a58
	 nop
cache_init_r10k: /* 0xbfc009c0 */
	mfc0	$t1, $CP0_CONFIG
	mtc0	$zero, $CP0_TAGHI
	andi	$t1, $t1, 0x2000
	bgtz	$t1, L_0xbfc009d8
	 li	$t0, 0x80
	li	$t0, 0x40
L_0xbfc009d8: /* 0xbfc009d8 */
	move	$t1, $s0
cache_init_r10k_l2_loop: /* 0xbfc009dc */
	sll	$at, $t1, 3
	srl	$at, $at, 0x15
	sll	$at, $at, 0xe
	andi	$t2, $t1, 0x3000
	ori	$at, $at, R10K_L2_TAGLO_DIRTY
	srl	$t2, $t2, 5
	or	$t2, $t2, $at
	mtc0	$t2, $CP0_TAGLO
	nop
	addu	$t3, $t1, $t0
	cache	(CACHE_TYPE_L2|INDEX_STORE_TAG), 0($t1)
	sltu	$at, $sp, $t3
	beqz	$at, cache_init_r10k_l2_loop
	 move	$t1, $t3
	lui	$t0, 0x4000
	mtc0	$t0, $CP0_TAGHI
	move	$t1, $s0
cache_init_r10k_l1d_loop: /* 0xbfc00a20 */
	sll	$at, $t1, 3
	srl	$at, $at, 0xf
	sll	$at, $at, 8
	ori	$at, $at, R10K_TAGLO_DIRTY
	mtc0	$at, $CP0_TAGLO
	nop
	addiu	$t2, $t1, R10K_L1D_BLOCK_SIZE
	sltu	$at, $sp, $t2
	cache	(CACHE_TYPE_L1D|INDEX_STORE_TAG), 0($t1)
	nop
	bnez	$at, cache_init_r10k_l1d_loop
	 move	$t1, $t2
	b	L_0xbfc00a58
	 nop
L_0xbfc00a58: /* 0xbfc00a58 */
	addiu	$t0, $sp, -8
	nop
L_0xbfc00a60: /* 0xbfc00a60 */
	not	$t1, $t0
	addiu	$t2, $t0, -8
	sd	$t1, 0($t0)
	bne	$t2, $s0, L_0xbfc00a60
	 move	$t0, $t2
	jr	$ra
	 nop
	nop		# unreachable

/* Function F_0xbfc00a80 [0xbfc00a80 - 0xbfc00b70)
 *
 * $a0 = initialization number
 * $a1
 */
F_0xbfc00a80: /* 0xbfc00a80 */
	addiu	$sp, $sp, -0x20
	sw	$a0, 0x20($sp)
	sw	$a1, 0x24($sp)
	sw	$ra, 0x14($sp)
	lui	$a1, %hi(data_0xbfc01de0)
	lui	$a0, %hi(data_0xbfc01e70)
	addiu	$a0, $a0, %lo(data_0xbfc01e70)		# Pointer to 0xbfc01e70 containing struct { start=0xbfc00000, end=0xbfc80000, alignment=0x100, length=0x00080000, 0xbfc00c48, nop1, isa_reset }
	addiu	$a1, $a1, %lo(data_0xbfc01de0)		# Pointer to 0xbfc01de0 containing "post1"
	jal	find_valid_section_by_name		# Call find_valid_section_by_name(flash_info, "post1", 0)
	 move	$a2, $zero
	lui	$a0, %hi(data_0xbfc01e70)
	lui	$a1, %hi(data_0xbfc01de8)
	addiu	$a1, $a1, %lo(data_0xbfc01de8)		# Pointer to 0xbfc01de8 containing "firmware"
	addiu	$a0, $a0, %lo(data_0xbfc01e70)		# Pointer to 0xbfc01e70 containing struct { start=0xbfc00000, end=0xbfc80000, alignment=0x100, length=0x00080000, 0xbfc00c48, nop1, isa_reset }
	move	$a2, $zero
	jal	find_section_by_name		# Call find_section_by_name(flash_info, "firmware", 0)
	 sw	$v0, 0x1c($sp)		# Save address of post1 section
	lui	$a3, HI_UNSIGNED(BASE_ISA | ISA_MISC_CONTROL)
	ori	$a3, $a3, LO_UNSIGNED(BASE_ISA | ISA_MISC_CONTROL)
	lw	$t6, 0($a3)		# ISA_MISC_CONTROL
	lw	$t7, 4($a3)		# ISA_MISC_CONTROL + 0x04
	li	$at, -1
	lw	$v1, 0x1c($sp)		# $v1 = address of post1 section
	and	$t8, $t6, $at
	li	$at, ~(ISA_RED_LED|ISA_GREEN_LED)
	sw	$t8, 0($a3)		# ISA_MISC_CONTROL
	and	$t9, $t7, $at
	move	$a1, $v0		# $a1 = address of firmware section
	beqz	$v1, L_0xbfc00b14
	 sw	$t9, 4($a3)
	beqz	$v0, L_0xbfc00b14
	 lw	$a0, 0x20($sp)
	jalr	$v1		# Call post1($a0=$a0, $a1=address of firmware section, $a2=$a1)
	 lw	$a2, 0x24($sp)
	lui	$a3, HI_UNSIGNED(BASE_ISA | ISA_MISC_CONTROL)
	beqz	$v0, L_0xbfc00b34
	 ori	$a3, $a3, LO_UNSIGNED(BASE_ISA | ISA_MISC_CONTROL)
L_0xbfc00b14: /* 0xbfc00b14 */
	lw	$t0, 0($a3)		# ISA_MISC_CONTROL
	lw	$t1, 4($a3)		# ISA_MISC_CONTROL + 0x04
	li	$at, -1
	and	$t2, $t0, $at
	li	$at, ~(ISA_RED_LED|ISA_GREEN_LED)
	sw	$t2, 0($a3)
	and	$t3, $t1, $at
	sw	$t3, 4($a3)
L_0xbfc00b34: /* 0xbfc00b34 */
	jal	F_0xbfc00b70
	 nop
	b	L_0xbfc00b34
	 nop
	nop		# unreachable
	nop		# unreachable
	nop		# unreachable
	nop		# unreachable
	nop		# unreachable
	nop		# unreachable
	nop		# unreachable
	lw	$ra, 0x14($sp)		# unreachable
	addiu	$sp, $sp, 0x20		# unreachable
	jr	$ra		# unreachable
	 nop		# unreachable

/* Function F_0xbfc00b70 [0xbfc00b70 - 0xbfc00c48)
 */
F_0xbfc00b70: /* 0xbfc00b70 */
	addiu	$sp, $sp, -0x1a0
	sw	$ra, 0x1c($sp)
	jal	F_0xbfc01168
	 sw	$s0, 0x18($sp)
	sw	$v0, 0x19c($sp)
	sw	$zero, 0x128($sp)
	sw	$zero, 0x12c($sp)
	sw	$zero, 0x130($sp)
	sw	$zero, 0x24($sp)
	sw	$zero, 0x194($sp)
	sw	$zero, 0x198($sp)
	jal	isa_reset
	 li	$a0, 1
	addiu	$s0, $sp, 0x24
	jal	F_0xbfc00ccc
	 move	$a0, $s0
	bnez	$v0, L_0xbfc00be4
	 move	$v1, $v0
L_0xbfc00bb8: /* 0xbfc00bb8 */
	jal	F_0xbfc01054
	 move	$a0, $s0
	bnez	$v0, L_0xbfc00be4
	 move	$v1, $v0
	lw	$t6, 0x128($sp)
	move	$a0, $s0
	addiu	$t7, $t6, 1
	jal	F_0xbfc00ccc
	 sw	$t7, 0x128($sp)
	beqz	$v0, L_0xbfc00bb8
	 move	$v1, $v0
L_0xbfc00be4: /* 0xbfc00be4 */
	move	$a0, $s0
	move	$a1, $zero
	jal	F_0xbfc00f08
	 sw	$v1, 0x20($sp)
	jal	isa_reset
	 move	$a0, $zero
	lw	$v1, 0x20($sp)
	sra	$a0, $v1, 8
	jal	F_0xbfc01408
	 andi	$a0, $a0, 0xff
	jal	F_0xbfc014a0
	 lw	$a0, 0x128($sp)
	jal	F_0xbfc01408
	 li	$a0, 0x2d
	jal	F_0xbfc014a0
	 lw	$a0, 0x130($sp)
	jal	F_0xbfc01408
	 li	$a0, 0xa
	jal	nop2
	 nop
	lw	$ra, 0x1c($sp)
	lw	$s0, 0x18($sp)
	addiu	$sp, $sp, 0x1a0
	jr	$ra
	 nop

/* Function F_0xbfc00c48 [0xbfc00c48 - 0xbfc00c70)
 */
F_0xbfc00c48: /* 0xbfc00c48 */
	sw	$a1, 4($sp)		# void ...(out $a0, in $a1)
	lbu	$t6, 4($sp)
	sb	$t6, 0($a0)
	lbu	$t7, 5($sp)
	sb	$t7, 1($a0)
	lbu	$t8, 6($sp)
	sb	$t8, 2($a0)
	lbu	$t9, 7($sp)
	jr	$ra
	 sb	$t9, 3($a0)

/* Function nop1 [0xbfc00c70 - 0xbfc00c78)
 */
nop1: /* 0xbfc00c70 */
	jr	$ra
	 nop

/* Function isa_reset [0xbfc00c78 - 0xbfc00ccc)
 */
isa_reset: /* 0xbfc00c78 */
	beqz	$a0, L_0xbfc00ca0
	 lui	$v0, HI_UNSIGNED(BASE_ISA | ISA_MISC_CONTROL)
	lui	$v0, HI_UNSIGNED(BASE_ISA | ISA_MISC_CONTROL)
	ori	$v0, $v0, LO_UNSIGNED(BASE_ISA | ISA_MISC_CONTROL)
	lw	$t7, 4($v0)		# ISA_MISC_CONTROL + 0x04
	lw	$t6, 0($v0)		# ISA_MISC_CONTROL
	ori	$t9, $t7, ISA_FLASH_ROM_WRITE_ENABLE
	sw	$t9, 4($v0)		# ISA_MISC_CONTROL + 0x04
	jr	$ra
	 sw	$t6, 0($v0)		# ISA_MISC_CONTROL
L_0xbfc00ca0: /* 0xbfc00ca0 */
	ori	$v0, $v0, LO_UNSIGNED(BASE_ISA | ISA_MISC_CONTROL)
	lw	$t0, 0($v0)		# ISA_MISC_CONTROL
	lw	$t1, 4($v0)		# ISA_MISC_CONTROL + 0x04
	li	$at, -1
	and	$t2, $t0, $at
	li	$at, ~ISA_FLASH_ROM_WRITE_ENABLE
	and	$t3, $t1, $at
	sw	$t3, 4($v0)		# ISA_MISC_CONTROL + 0x04
	sw	$t2, 0($v0)		# ISA_MISC_CONTROL
	jr	$ra
	 nop

/* Function F_0xbfc00ccc [0xbfc00ccc - 0xbfc00f08)
 */
F_0xbfc00ccc: /* 0xbfc00ccc */
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
	lui	$s5, %hi(data_0xbfc01dfc)
	lui	$s7, %hi(data_0xbfc01df4)
	move	$s0, $a0
	move	$s3, $zero
	addiu	$s7, $s7, %lo(data_0xbfc01df4)
	addiu	$s5, $s5, %lo(data_0xbfc01dfc)
	lw	$v0, 0x178($a0)
	move	$s1, $zero
	lw	$s2, 0x44($sp)
	li	$s4, 0x3a
	li	$s6, 4
	li	$fp, 1
L_0xbfc00d28: /* 0xbfc00d28 */
	bne	$fp, $v0, L_0xbfc00d40
	 nop
	jal	F_0xbfc014a0
	 lw	$a0, 0x104($s0)
	jal	F_0xbfc01408
	 li	$a0, 0x3e
L_0xbfc00d40: /* 0xbfc00d40 */
	jal	F_0xbfc014f8
	 move	$a0, $fp
L_0xbfc00d48: /* 0xbfc00d48 */
	jal	F_0xbfc01398
	 nop
	andi	$t6, $v0, 0xff00
	beqz	$t6, L_0xbfc00d68
	 nop
	sra	$s3, $v0, 8
	b	L_0xbfc00e80
	 li	$s1, 8
L_0xbfc00d68: /* 0xbfc00d68 */
	bnez	$s1, L_0xbfc00d88
	 sll	$t7, $s1, 3
	bnel	$v0, $s4, L_0xbfc00e84
	 slti	$at, $s1, 7
	move	$s1, $fp
	sw	$zero, 0x110($s0)
	b	L_0xbfc00e80
	 sw	$zero, 0x114($s0)
L_0xbfc00d88: /* 0xbfc00d88 */
	lw	$v1, 0x110($s0)
	addu	$a1, $s5, $t7
	lh	$t0, 0($a1)
	bgez	$v1, L_0xbfc00da4
	 sra	$t8, $v1, 1
	addiu	$at, $v1, 1
	sra	$t8, $at, 1
L_0xbfc00da4: /* 0xbfc00da4 */
	addu	$t9, $t8, $s0
	slti	$at, $v0, 0x30
	addiu	$t1, $v1, 1
	sw	$t1, 0x110($s0)
	bnez	$at, L_0xbfc00dd0
	 addu	$a0, $t9, $t0
	slti	$at, $v0, 0x3a
	beqzl	$at, L_0xbfc00dd4
	 slti	$at, $v0, 0x41
	b	L_0xbfc00e04
	 addiu	$s2, $v0, -0x30
L_0xbfc00dd0: /* 0xbfc00dd0 */
	slti	$at, $v0, 0x41
L_0xbfc00dd4: /* 0xbfc00dd4 */
	bnez	$at, L_0xbfc00dec
	 slti	$at, $v0, 0x47
	beqzl	$at, L_0xbfc00df0
	 slti	$at, $v0, 0x61
	b	L_0xbfc00e04
	 addiu	$s2, $v0, -0x37
L_0xbfc00dec: /* 0xbfc00dec */
	slti	$at, $v0, 0x61
L_0xbfc00df0: /* 0xbfc00df0 */
	bnez	$at, L_0xbfc00e04
	 slti	$at, $v0, 0x67
	beqzl	$at, L_0xbfc00e08
	 lw	$t2, 0x110($s0)
	addiu	$s2, $v0, -0x57
L_0xbfc00e04: /* 0xbfc00e04 */
	lw	$t2, 0x110($s0)
L_0xbfc00e08: /* 0xbfc00e08 */
	sll	$t4, $s2, 4
	andi	$t3, $t2, 1
	beqzl	$t3, L_0xbfc00e24
	 lbu	$t5, 0($a0)
	b	L_0xbfc00e3c
	 sb	$t4, 0($a0)
	lbu	$t5, 0($a0)		# unreachable
L_0xbfc00e24: /* 0xbfc00e24 */
	or	$t8, $t5, $s2
	sb	$t8, 0($a0)
	lw	$t7, 0x114($s0)
	sll	$t9, $t8, 0x18
	addu	$t0, $t7, $t9
	sw	$t0, 0x114($s0)
L_0xbfc00e3c: /* 0xbfc00e3c */
	lh	$v0, 2($a1)
	bnezl	$v0, L_0xbfc00e54
	 lw	$t1, 0x110($s0)
	lbu	$v0, 0x11c($s0)
	sll	$v0, $v0, 1
	lw	$t1, 0x110($s0)
L_0xbfc00e54: /* 0xbfc00e54 */
	slt	$at, $t1, $v0
	bnezl	$at, L_0xbfc00e84
	 slti	$at, $s1, 7
	bne	$s1, $s6, L_0xbfc00e7c
	 sw	$zero, 0x110($s0)
	lbu	$t2, 0x11d($s0)
	andi	$t3, $t2, 7
	addu	$t4, $s7, $t3
	b	L_0xbfc00e80
	 lbu	$s1, 0($t4)
L_0xbfc00e7c: /* 0xbfc00e7c */
	lw	$s1, 4($a1)
L_0xbfc00e80: /* 0xbfc00e80 */
	slti	$at, $s1, 7
L_0xbfc00e84: /* 0xbfc00e84 */
	bnez	$at, L_0xbfc00d48
	 nop
	jal	F_0xbfc014f8
	 move	$a0, $zero
	lw	$t5, 0x114($s0)
	beqzl	$t5, L_0xbfc00ea8
	 lw	$v0, 0x178($s0)
	li	$s3, 0x5800
	lw	$v0, 0x178($s0)
L_0xbfc00ea8: /* 0xbfc00ea8 */
	bne	$fp, $v0, L_0xbfc00ec0
	 nop
	bnez	$s3, L_0xbfc00ec0
	 nop
	b	L_0xbfc00d28
	 move	$s1, $zero
L_0xbfc00ec0: /* 0xbfc00ec0 */
	bnez	$s3, L_0xbfc00ed8
	 lw	$s1, 0x1c($sp)
	lbu	$t6, 0x11d($s0)
	bnel	$fp, $t6, L_0xbfc00edc
	 lw	$ra, 0x3c($sp)
	li	$s3, 0x2400
L_0xbfc00ed8: /* 0xbfc00ed8 */
	lw	$ra, 0x3c($sp)
L_0xbfc00edc: /* 0xbfc00edc */
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

/* Function F_0xbfc00f08 [0xbfc00f08 - 0xbfc01054)
 */
F_0xbfc00f08: /* 0xbfc00f08 */
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
	bnez	$at, L_0xbfc00f48
	 move	$a2, $a1
	lui	$at, HI(ROM_END)
	sltu	$at, $a1, $at
	bnezl	$at, L_0xbfc00f54
	 sw	$a2, 0x2c($sp)
L_0xbfc00f48: /* 0xbfc00f48 */
	b	L_0xbfc01038
	 li	$v0, 0x4100
	sw	$a2, 0x2c($sp)		# unreachable
L_0xbfc00f54: /* 0xbfc00f54 */
	lw	$t6, 0($s2)
	move	$s0, $zero
	move	$s1, $s2
	beqz	$t6, L_0xbfc00fc4
	 li	$s3, 0x100
	sw	$a2, 0x2c($sp)
L_0xbfc00f6c: /* 0xbfc00f6c */
	lw	$t7, 0($s2)
	lw	$a1, 4($s1)
	jal	F_0xbfc00c48
	 addu	$a0, $t7, $s0
	addiu	$s0, $s0, 4
	bne	$s0, $s3, L_0xbfc00f6c
	 addiu	$s1, $s1, 4
	jal	nop1
	 nop
	lw	$t8, 0x178($s2)
	li	$at, 1
	beql	$t8, $at, L_0xbfc00fc8
	 lw	$a2, 0x2c($sp)
	lw	$t9, 0x108($s2)
	addiu	$t0, $t9, 1
	andi	$t2, $t0, 0x3f
	bnez	$t2, L_0xbfc00fbc
	 sw	$t0, 0x108($s2)
	jal	F_0xbfc01408
	 li	$a0, 0xa
L_0xbfc00fbc: /* 0xbfc00fbc */
	jal	F_0xbfc01408
	 li	$a0, 0x2e
L_0xbfc00fc4: /* 0xbfc00fc4 */
	lw	$a2, 0x2c($sp)
L_0xbfc00fc8: /* 0xbfc00fc8 */
	li	$at, -0x100
	move	$v0, $zero
	beqz	$a2, L_0xbfc01034
	 addiu	$t3, $a2, 0xff
	and	$t4, $t3, $at
	sw	$t4, 0($s2)
	move	$v1, $s2
	li	$a0, 0x40
L_0xbfc00fe8: /* 0xbfc00fe8 */
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
	bne	$v0, $a0, L_0xbfc00fe8
	 sb	$t6, 3($v1)
L_0xbfc01034: /* 0xbfc01034 */
	move	$v0, $zero
L_0xbfc01038: /* 0xbfc01038 */
	lw	$ra, 0x24($sp)
	lw	$s0, 0x14($sp)
	lw	$s1, 0x18($sp)
	lw	$s2, 0x1c($sp)
	lw	$s3, 0x20($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x28

/* Function F_0xbfc01054 [0xbfc01054 - 0xbfc01168)
 */
F_0xbfc01054: /* 0xbfc01054 */
	addiu	$sp, $sp, -0x20
	sw	$ra, 0x1c($sp)
	sw	$s1, 0x18($sp)
	sw	$s0, 0x14($sp)
	lbu	$v0, 0x11d($a0)
	move	$s0, $a0
	move	$v1, $zero
	beqz	$v0, L_0xbfc01090
	 li	$at, 2
	beq	$v0, $at, L_0xbfc01134
	 li	$at, 4
	beql	$v0, $at, L_0xbfc01148
	 lhu	$t6, 0x11a($s0)
	b	L_0xbfc01154
	 move	$v0, $v1
L_0xbfc01090: /* 0xbfc01090 */
	lbu	$t6, 0x11c($s0)
	move	$s1, $zero
	blezl	$t6, L_0xbfc01154
	 move	$v0, $v1
	lw	$t7, 0x170($s0)
L_0xbfc010a4: /* 0xbfc010a4 */
	lw	$t8, 0x174($s0)
	lhu	$t0, 0x118($s0)
	lw	$t2, 0($s0)
	addu	$t9, $t7, $t8
	addu	$t1, $t9, $t0
	addu	$a1, $t1, $s1
	subu	$v0, $a1, $t2
	bltz	$v0, L_0xbfc010d0
	 slti	$at, $v0, 0x100
	bnezl	$at, L_0xbfc0110c
	 addu	$t0, $s0, $s1
L_0xbfc010d0: /* 0xbfc010d0 */
	jal	F_0xbfc00f08
	 move	$a0, $s0
	beqz	$v0, L_0xbfc010e8
	 move	$v1, $v0
	b	L_0xbfc01158
	 lw	$ra, 0x1c($sp)
L_0xbfc010e8: /* 0xbfc010e8 */
	lw	$t3, 0x170($s0)
	lw	$t4, 0x174($s0)
	lhu	$t6, 0x118($s0)
	lw	$t9, 0($s0)
	addu	$t5, $t3, $t4
	addu	$t7, $t5, $t6
	addu	$t8, $t7, $s1
	subu	$v0, $t8, $t9
	addu	$t0, $s0, $s1
L_0xbfc0110c: /* 0xbfc0110c */
	lbu	$t1, 0x11f($t0)
	addu	$t2, $s0, $v0
	addiu	$s1, $s1, 1
	sb	$t1, 4($t2)
	lbu	$t3, 0x11c($s0)
	slt	$at, $s1, $t3
	bnezl	$at, L_0xbfc010a4
	 lw	$t7, 0x170($s0)
	b	L_0xbfc01154
	 move	$v0, $v1
L_0xbfc01134: /* 0xbfc01134 */
	lhu	$t4, 0x11a($s0)
	sll	$t5, $t4, 4
	b	L_0xbfc01150
	 sw	$t5, 0x170($s0)
	lhu	$t6, 0x11a($s0)		# unreachable
L_0xbfc01148: /* 0xbfc01148 */
	sll	$t7, $t6, 0x10
	sw	$t7, 0x174($s0)
L_0xbfc01150: /* 0xbfc01150 */
	move	$v0, $v1
L_0xbfc01154: /* 0xbfc01154 */
	lw	$ra, 0x1c($sp)
L_0xbfc01158: /* 0xbfc01158 */
	lw	$s0, 0x14($sp)
	lw	$s1, 0x18($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x20

/* Function F_0xbfc01168 [0xbfc01168 - 0xbfc01398)
 */
F_0xbfc01168: /* 0xbfc01168 */
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
	lui	$s4, %hi(data_0xbfc01e54)
	lui	$s5, HI_UNSIGNED(UART_BASE_CLOCK)
	sw	$ra, 0x3c($sp)
	li	$v0, 1
	ori	$s5, $s5, LO_UNSIGNED(UART_BASE_CLOCK)
	addiu	$s4, $s4, %lo(data_0xbfc01e54)
	move	$s0, $zero
	move	$s1, $zero
	li	$s2, 0x3e00
	li	$s3, 3
	li	$s6, 0x80
	li	$s7, 0x1b
	li	$fp, 3
L_0xbfc011c4: /* 0xbfc011c4 */
	lw	$t6, 0($s4)
L_0xbfc011c8: /* 0xbfc011c8 */
	lui	$t7, %hi(data_0xbfc01e98)
	bnez	$t6, L_0xbfc011e0
	 nop
	lui	$s4, %hi(data_0xbfc01e50)
	addiu	$s4, $s4, %lo(data_0xbfc01e50)
	move	$v0, $zero
L_0xbfc011e0: /* 0xbfc011e0 */
	lw	$t7, %lo(data_0xbfc01e98)($t7)
	lui	$t2, %hi(data_0xbfc01e8c)
	lui	$a0, %hi(data_0xbfc01e44)
	sb	$s6, 0($t7)
	lw	$t8, 0($s4)
	lw	$t2, %lo(data_0xbfc01e8c)($t2)
	addiu	$a0, $a0, %lo(data_0xbfc01e44)
	sll	$t9, $t8, 4
	div	$zero, $s5, $t9
	mflo	$t0
	move	$t1, $t0
	sb	$t1, 0($t2)
	lw	$t3, 0($s4)
	lui	$t8, %hi(data_0xbfc01e90)
	lw	$t8, %lo(data_0xbfc01e90)($t8)
	sll	$t4, $t3, 4
	div	$zero, $s5, $t4
	mflo	$t5
	sra	$t7, $t5, 8
	bnez	$t9, L_0xbfc01238
	 nop
	break	7
L_0xbfc01238: /* 0xbfc01238 */
	li	$at, -1
	bne	$t9, $at, L_0xbfc01250
	 lui	$at, HI(KSEG0)
	bne	$s5, $at, L_0xbfc01250
	 nop
	break	6
L_0xbfc01250: /* 0xbfc01250 */
	lui	$t9, %hi(data_0xbfc01e98)
	sb	$t7, 0($t8)
	lw	$t9, %lo(data_0xbfc01e98)($t9)
	lui	$t0, %hi(data_0xbfc01e90)
	lui	$t1, %hi(data_0xbfc01e9c)
	sb	$s7, 0($t9)
	lw	$t0, %lo(data_0xbfc01e90)($t0)
	lui	$t3, %hi(data_0xbfc01e94)
	bnez	$t4, L_0xbfc0127c
	 nop
	break	7
L_0xbfc0127c: /* 0xbfc0127c */
	li	$at, -1
	bne	$t4, $at, L_0xbfc01294
	 lui	$at, HI(KSEG0)
	bne	$s5, $at, L_0xbfc01294
	 nop
	break	6
L_0xbfc01294: /* 0xbfc01294 */
	sb	$zero, 0($t0)
	lw	$t1, %lo(data_0xbfc01e9c)($t1)		# Load address from table (0xbf390407)
	li	$t2, 7
	sb	$fp, 0($t1)
	lw	$t3, %lo(data_0xbfc01e94)($t3)
	jal	F_0xbfc01454
	 sb	$t2, 0($t3)
	jal	F_0xbfc014a0
	 lw	$a0, 0($s4)
	lui	$a0, %hi(data_0xbfc01e4c)
	jal	F_0xbfc01454
	 addiu	$a0, $a0, %lo(data_0xbfc01e4c)
	jal	F_0xbfc01408
	 li	$a0, 0x3e
	jal	F_0xbfc014f8
	 li	$a0, 1
L_0xbfc012d4: /* 0xbfc012d4 */
	jal	F_0xbfc01398
	 nop
	bne	$v0, $s2, L_0xbfc012f4
	 move	$v1, $v0
	addiu	$s4, $s4, 4
	move	$s1, $zero
	b	L_0xbfc011c4
	 move	$s0, $zero
L_0xbfc012f4: /* 0xbfc012f4 */
	andi	$t4, $v0, 0xff00
	beqz	$t4, L_0xbfc0131c
	 sll	$t5, $s1, 8
	sra	$a0, $v1, 8
	andi	$a0, $a0, 0xff
	move	$s1, $zero
	jal	F_0xbfc01408
	 move	$s0, $zero
	b	L_0xbfc011c8
	 lw	$t6, 0($s4)
L_0xbfc0131c: /* 0xbfc0131c */
	addiu	$s0, $s0, 1
	bne	$s0, $s3, L_0xbfc012d4
	 or	$s1, $t5, $v0
	jal	F_0xbfc014f8
	 move	$a0, $zero
	lui	$at, HI_UNSIGNED(0x00534c44)
	ori	$at, $at, LO_UNSIGNED(0x00534c44)
	bne	$s1, $at, L_0xbfc01348
	 move	$s0, $zero
	b	L_0xbfc01368
	 move	$v0, $zero
L_0xbfc01348: /* 0xbfc01348 */
	lui	$at, HI_UNSIGNED(0x00534c48)
	ori	$at, $at, LO_UNSIGNED(0x00534c48)
	bne	$s1, $at, L_0xbfc01360
	 nop
	b	L_0xbfc01368
	 li	$v0, 1
L_0xbfc01360: /* 0xbfc01360 */
	b	L_0xbfc011c4
	 move	$s1, $zero
L_0xbfc01368: /* 0xbfc01368 */
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

/* Function F_0xbfc01398 [0xbfc01398 - 0xbfc01408)
 */
F_0xbfc01398: /* 0xbfc01398 */
	lui	$v1, %hi(data_0xbfc01ea0)
	lw	$v1, %lo(data_0xbfc01ea0)($v1)
	lui	$t1, %hi(data_0xbfc01e8c)
	lbu	$v0, 0($v1)
	andi	$t6, $v0, 0x10
L_0xbfc013ac: /* 0xbfc013ac */
	beqz	$t6, L_0xbfc013bc
	 andi	$t7, $v0, 0xe
	jr	$ra
	 li	$v0, 0x3e00
L_0xbfc013bc: /* 0xbfc013bc */
	beqz	$t7, L_0xbfc013f0
	 andi	$t0, $v0, 1
	andi	$t8, $v0, 8
	beqz	$t8, L_0xbfc013d8
	 andi	$t9, $v0, 4
	jr	$ra
	 li	$v0, 0x4600
L_0xbfc013d8: /* 0xbfc013d8 */
	beqz	$t9, L_0xbfc013e8
	 nop
	jr	$ra
	 li	$v0, 0x5000
L_0xbfc013e8: /* 0xbfc013e8 */
	jr	$ra
	 li	$v0, 0x4f00
L_0xbfc013f0: /* 0xbfc013f0 */
	beqzl	$t0, L_0xbfc013ac
	 andi	$t6, $v0, 0x10
	lw	$t1, %lo(data_0xbfc01e8c)($t1)
	lbu	$v0, 0($t1)
	jr	$ra
	 nop

/* Function F_0xbfc01408 [0xbfc01408 - 0xbfc01454)
 */
F_0xbfc01408: /* 0xbfc01408 */
	lui	$v1, %hi(data_0xbfc01e8c)
	addiu	$v1, $v1, %lo(data_0xbfc01e8c)
	lw	$t6, 0x14($v1)
	sw	$a0, 0($sp)
	lui	$t9, %hi(data_0xbfc01e8c)
	lbu	$t7, 0($t6)
	andi	$a0, $a0, 0xff
	addiu	$t9, $t9, %lo(data_0xbfc01e8c)
	andi	$t8, $t7, 0x20
	bnez	$t8, L_0xbfc01448
	 lui	$t1, %hi(data_0xbfc01e8c)
	lw	$t0, 0x14($t9)
	lbu	$v0, 0($t0)
	andi	$v0, $v0, 0x20
L_0xbfc01440: /* 0xbfc01440 */
	beqz	$v0, L_0xbfc01440
	 nop
L_0xbfc01448: /* 0xbfc01448 */
	lw	$t1, %lo(data_0xbfc01e8c)($t1)
	jr	$ra
	 sb	$a0, 0($t1)

/* Function F_0xbfc01454 [0xbfc01454 - 0xbfc014a0)
 */
F_0xbfc01454: /* 0xbfc01454 */
	addiu	$sp, $sp, -0x20
	sw	$ra, 0x1c($sp)
	sw	$s1, 0x18($sp)
	sw	$s0, 0x14($sp)
	lbu	$s1, 0($a0)
	addiu	$s0, $a0, 1
	beqzl	$s1, L_0xbfc01490
	 lw	$ra, 0x1c($sp)
L_0xbfc01474: /* 0xbfc01474 */
	jal	F_0xbfc01408
	 andi	$a0, $s1, 0xff
	lbu	$s1, 0($s0)
	addiu	$s0, $s0, 1
	bnez	$s1, L_0xbfc01474
	 nop
	lw	$ra, 0x1c($sp)
L_0xbfc01490: /* 0xbfc01490 */
	lw	$s0, 0x14($sp)
	lw	$s1, 0x18($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x20

/* Function F_0xbfc014a0 [0xbfc014a0 - 0xbfc014f8)
 */
F_0xbfc014a0: /* 0xbfc014a0 */
	addiu	$sp, $sp, -0x18
	sltiu	$at, $a0, 0xa
	sw	$ra, 0x14($sp)
	bnez	$at, L_0xbfc014d0
	 move	$a1, $a0
	li	$at, 0xa
	divu	$zero, $a0, $at
	mflo	$a0
	sw	$a1, 0x18($sp)
	jal	F_0xbfc014a0
	 nop
	lw	$a1, 0x18($sp)
L_0xbfc014d0: /* 0xbfc014d0 */
	li	$at, 0xa
	divu	$zero, $a1, $at
	mfhi	$a0
	addiu	$a0, $a0, 0x30
	jal	F_0xbfc01408
	 andi	$a0, $a0, 0xff
	lw	$ra, 0x14($sp)
	addiu	$sp, $sp, 0x18
	jr	$ra
	 nop

/* Function F_0xbfc014f8 [0xbfc014f8 - 0xbfc0152c)
 */
F_0xbfc014f8: /* 0xbfc014f8 */
	lui	$t6, %hi(data_0xbfc01e9c)
	lw	$t6, %lo(data_0xbfc01e9c)($t6)		# Load address from table (0xbf390407)
	lui	$t9, %hi(data_0xbfc01e9c)
	beqz	$a0, L_0xbfc01518
	 lbu	$v0, 0($t6)		# $v0 = *(char *)0xbf390407
	ori	$t7, $v0, 1
	jr	$ra
	 sb	$t7, 0($t6)		# *(char *)0xbf390407 |= 1;
L_0xbfc01518: /* 0xbfc01518 */
	lw	$t9, %lo(data_0xbfc01e9c)($t9)
	andi	$t8, $v0, 0xfffe
	sb	$t8, 0($t9)		# *(char *)0xbf390407 &= ~1;
	jr	$ra
	 nop

/* Function nop2 [0xbfc0152c - 0xbfc01534)
 */
nop2: /* 0xbfc0152c */
	jr	$ra
	 nop

/* Function strlen [0xbfc01534 - 0xbfc01564)
 */
strlen: /* 0xbfc01534 */
	lbu	$v0, 0($a0)
	move	$v1, $zero
	addiu	$a0, $a0, 1
	beqz	$v0, L_0xbfc0155c
	 nop
	lbu	$v0, 0($a0)
L_0xbfc0154c: /* 0xbfc0154c */
	addiu	$a0, $a0, 1
	addiu	$v1, $v1, 1
	bnezl	$v0, L_0xbfc0154c
	 lbu	$v0, 0($a0)
L_0xbfc0155c: /* 0xbfc0155c */
	jr	$ra
	 move	$v0, $v1

/* Function strncmp [0xbfc01564 - 0xbfc015f0)
 *
 * $a0 = string1; $a1 = string2; $a2 = len
 */
strncmp: /* 0xbfc01564 */
	addiu	$v0, $a2, -1
	bltz	$v0, L_0xbfc015c4
	 move	$a3, $a1
	lbu	$a1, 0($a0)
	lbu	$t6, 0($a3)
	addiu	$a3, $a3, 1
	xor	$v1, $a1, $t6
	sltiu	$v1, $v1, 1
	beqz	$v1, L_0xbfc015c4
	 sltiu	$v1, $a1, 1
L_0xbfc0158c: /* 0xbfc0158c */
	beqz	$v1, L_0xbfc0159c
	 addiu	$a0, $a0, 1
	jr	$ra
	 move	$v0, $zero
L_0xbfc0159c: /* 0xbfc0159c */
	addiu	$v0, $v0, -1
	bltz	$v0, L_0xbfc015c4
	 nop
	lbu	$a1, 0($a0)
	lbu	$t7, 0($a3)
	addiu	$a3, $a3, 1
	xor	$v1, $a1, $t7
	sltiu	$v1, $v1, 1
	bnezl	$v1, L_0xbfc0158c
	 sltiu	$v1, $a1, 1
L_0xbfc015c4: /* 0xbfc015c4 */
	bgezl	$v0, L_0xbfc015d8
	 lbu	$t8, 0($a0)
	b	L_0xbfc015e4
	 move	$v1, $zero
	lbu	$t8, 0($a0)		# unreachable
L_0xbfc015d8: /* 0xbfc015d8 */
	lbu	$t9, -1($a3)
	addiu	$a3, $a3, -1
	subu	$v1, $t8, $t9
L_0xbfc015e4: /* 0xbfc015e4 */
	move	$v0, $v1
	jr	$ra
	 nop

/* Function enter_interactive_mode [0xbfc015f0 - 0xbfc0161c)
 */
enter_interactive_mode: /* 0xbfc015f0 */
	addiu	$sp, $sp, -0x18		# Calls exception_uncached_exception with $a0=2
	lui	$t9, %hi(exception_uncached_exception)
	sw	$ra, 0x14($sp)
	ori	$t9, $t9, %lo(exception_uncached_exception)
	li	$a0, 2
	jalr	$t9
	 move	$a1, $zero
	lw	$ra, 0x14($sp)
	addiu	$sp, $sp, 0x18
	jr	$ra
	 nop

/* Function halt [0xbfc0161c - 0xbfc01648)
 */
halt: /* 0xbfc0161c */
	addiu	$sp, $sp, -0x18		# Calls exception_uncached_exception with $a0=3
	lui	$t9, %hi(exception_uncached_exception)
	sw	$ra, 0x14($sp)
	ori	$t9, $t9, %lo(exception_uncached_exception)
	li	$a0, 3
	jalr	$t9
	 move	$a1, $zero
	lw	$ra, 0x14($sp)
	addiu	$sp, $sp, 0x18
	jr	$ra
	 nop

/* Function powerdown [0xbfc01648 - 0xbfc01674)
 */
powerdown: /* 0xbfc01648 */
	addiu	$sp, $sp, -0x18		# Calls exception_uncached_exception with $a0=4
	lui	$t9, %hi(exception_uncached_exception)
	sw	$ra, 0x14($sp)
	ori	$t9, $t9, %lo(exception_uncached_exception)
	li	$a0, 4
	jalr	$t9
	 move	$a1, $zero
	lw	$ra, 0x14($sp)
	addiu	$sp, $sp, 0x18
	jr	$ra
	 nop

/* Function restart [0xbfc01674 - 0xbfc016a0)
 */
restart: /* 0xbfc01674 */
	addiu	$sp, $sp, -0x18		# Calls exception_uncached_exception with $a0=5
	lui	$t9, %hi(exception_uncached_exception)
	sw	$ra, 0x14($sp)
	ori	$t9, $t9, %lo(exception_uncached_exception)
	li	$a0, 5
	jalr	$t9
	 move	$a1, $zero
	lw	$ra, 0x14($sp)
	addiu	$sp, $sp, 0x18
	jr	$ra
	 nop

/* Function reboot [0xbfc016a0 - 0xbfc016d0)
 */
reboot: /* 0xbfc016a0 */
	addiu	$sp, $sp, -0x18		# Calls exception_uncached_exception with $a0=6
	lui	$t9, %hi(exception_uncached_exception)
	sw	$ra, 0x14($sp)
	ori	$t9, $t9, %lo(exception_uncached_exception)
	li	$a0, 6
	jalr	$t9
	 move	$a1, $zero
	lw	$ra, 0x14($sp)
	addiu	$sp, $sp, 0x18
	jr	$ra
	 nop
	nop		# unreachable

/* Function find_section_by_name [0xbfc016d0 - 0xbfc01750)
 */
find_section_by_name: /* 0xbfc016d0 */
	addiu	$sp, $sp, -0x28
	sw	$s1, 0x1c($sp)
	move	$s1, $a1
	sw	$ra, 0x24($sp)
	sw	$s2, 0x20($sp)
	move	$s2, $a0
	sw	$s0, 0x18($sp)
	jal	find_next_section
	 move	$a1, $a2
	beqz	$v0, L_0xbfc01734
	 move	$s0, $v0
L_0xbfc016fc: /* 0xbfc016fc */
	jal	strlen
	 move	$a0, $s1
	lbu	$a2, SHDR_OFFSET_NAME_LEN($s0)
	move	$a0, $s1
	bnel	$v0, $a2, L_0xbfc01724
	 move	$a0, $s2
	jal	strncmp
	 addiu	$a1, $s0, SHDR_OFFSET_NAME
	beqz	$v0, L_0xbfc01734
	 move	$a0, $s2
L_0xbfc01724: /* 0xbfc01724 */
	jal	find_next_section
	 move	$a1, $s0
	bnez	$v0, L_0xbfc016fc
	 move	$s0, $v0
L_0xbfc01734: /* 0xbfc01734 */
	lw	$ra, 0x24($sp)
	move	$v0, $s0
	lw	$s0, 0x18($sp)
	lw	$s1, 0x1c($sp)
	lw	$s2, 0x20($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x28

/* Function find_valid_section_by_name [0xbfc01750 - 0xbfc017b8)
 */
find_valid_section_by_name: /* 0xbfc01750 */
	addiu	$sp, $sp, -0x28
	sw	$ra, 0x24($sp)
	sw	$s2, 0x20($sp)
	sw	$s1, 0x1c($sp)
	move	$s1, $a0
	move	$s2, $a1
	jal	find_section_by_name
	 sw	$s0, 0x18($sp)
	beqz	$v0, L_0xbfc0179c
	 move	$s0, $v0
L_0xbfc01778: /* 0xbfc01778 */
	jal	is_section_checksum_valid
	 move	$a0, $s0
	bnez	$v0, L_0xbfc0179c
	 move	$a0, $s1
	move	$a1, $s2
	jal	find_section_by_name
	 move	$a2, $s0
	bnez	$v0, L_0xbfc01778
	 move	$s0, $v0
L_0xbfc0179c: /* 0xbfc0179c */
	lw	$ra, 0x24($sp)
	move	$v0, $s0
	lw	$s0, 0x18($sp)
	lw	$s1, 0x1c($sp)
	lw	$s2, 0x20($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x28

/* Function find_next_section [0xbfc017b8 - 0xbfc01874)
 *
 * $a0 = pointer to memory containing base address;
 * $a1 = offset? (or 0 => use address from *$a0)
 * returns pointer to section or 0
 */
find_next_section: /* 0xbfc017b8 */
	addiu	$sp, $sp, -0x20
	sw	$s1, 0x18($sp)
	sw	$s0, 0x14($sp)
	move	$s1, $a0
	sw	$ra, 0x1c($sp)
	move	$s0, $a1
L_0xbfc017d0: /* 0xbfc017d0 */
	bnez	$s0, L_0xbfc017e0
	 move	$a0, $s1
	b	L_0xbfc0181c
	 lw	$s0, 0($s1)
L_0xbfc017e0: /* 0xbfc017e0 */
	jal	is_section_valid
	 move	$a1, $s0
	bnezl	$v0, L_0xbfc01800
	 lw	$v1, 8($s1)		# Load section alignment requirement
	lw	$t6, 8($s1)		# Load section alignment requirement
	b	L_0xbfc0181c
	 addu	$s0, $s0, $t6
	lw	$v1, 8($s1)		# unreachable
L_0xbfc01800: /* 0xbfc01800 */
	lw	$t7, SHDR_OFFSET_SECTION_LEN($s0)
	addiu	$t9, $v1, -1
	addu	$t8, $s0, $t7
	addu	$v0, $t8, $v1
	addiu	$v0, $v0, -1
	not	$t0, $t9
	and	$s0, $v0, $t0
L_0xbfc0181c: /* 0xbfc0181c */
	lw	$v0, 4($s1)
	move	$a0, $s1
	sltu	$at, $s0, $v0		# $s0 is within the system ROM?
	beqzl	$at, L_0xbfc01848
	 sltu	$at, $s0, $v0
	jal	is_section_valid
	 move	$a1, $s0
	beqz	$v0, L_0xbfc017d0
	 nop
	lw	$v0, 4($s1)
	sltu	$at, $s0, $v0		# $s0 is within the system ROM?
L_0xbfc01848: /* 0xbfc01848 */
	beqz	$at, L_0xbfc0185c
	 lw	$s1, 0x18($sp)
	lbu	$t1, SHDR_OFFSET_NAME_LEN($s0)
	bnezl	$t1, L_0xbfc01864
	 lw	$ra, 0x1c($sp)
L_0xbfc0185c: /* 0xbfc0185c */
	move	$s0, $zero
	lw	$ra, 0x1c($sp)
L_0xbfc01864: /* 0xbfc01864 */
	move	$v0, $s0
	lw	$s0, 0x14($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x20

/* Function is_section_checksum_valid [0xbfc01874 - 0xbfc018f4)
 */
is_section_checksum_valid: /* 0xbfc01874 */
	lw	$t6, SHDR_OFFSET_SECTION_LEN($a0)		# $t6 = Load the length of the section
	li	$at, -4
	addiu	$v1, $a0, SHDR_SIZE		# $v1 = address of end of SHDR
	addu	$a1, $a0, $t6		# $a1 = address of end of section
	addiu	$a1, $a1, 3
	and	$a1, $a1, $at
	addiu	$t7, $a0, SHDR_SIZE
	move	$v0, $v1		# $v0 = address of data to be checksummed
	beq	$t7, $a1, checksum_done
	 move	$a2, $zero		# $a2 = checksum
	subu	$a3, $a1, $v1
	andi	$a3, $a3, 0xf
	beqz	$a3, checksum_main_loop
	 addu	$a0, $a3, $v1
checksum_remainder_loop: /* 0xbfc018ac */
	lw	$t8, 0($v0)
	addiu	$v0, $v0, 4
	bne	$a0, $v0, checksum_remainder_loop
	 addu	$a2, $a2, $t8
	beq	$v0, $a1, checksum_done
	 nop
checksum_main_loop: /* 0xbfc018c4 */
	lw	$t9, 0($v0)		# $t9 = word[0]
	lw	$t0, 4($v0)		# $t0 = word[1]
	lw	$t1, 8($v0)		# $t1 = word[2]
	addu	$a2, $a2, $t9		# checksm += word[0]
	lw	$t2, 0xc($v0)		# $t2 = word[3]
	addu	$a2, $a2, $t0		# checksum += word[1]
	addiu	$v0, $v0, 0x10		# word += 16
	addu	$a2, $a2, $t1		# checksum += word[2]
	bne	$v0, $a1, checksum_main_loop		# branch while not at end
	 addu	$a2, $a2, $t2		# checksum += word[3]
checksum_done: /* 0xbfc018ec */
	jr	$ra
	 sltiu	$v0, $a2, 1		# return checksum == 0

/* Function is_section_valid [0xbfc018f4 - 0xbfc019b0)
 */
is_section_valid: /* 0xbfc018f4 */
	lw	$t6, 8($a0)
	addiu	$v1, $a1, SHDR_SIZE
	move	$v0, $v1
	addiu	$t7, $t6, -1
	and	$t8, $a1, $t7
	bnez	$t8, L_0xbfc01938
	 nop
	lw	$t9, SHDR_OFFSET_MAGIC($a1)
	lui	$at, HI_UNSIGNED(SHDR_MAGIC)
	ori	$at, $at, LO_UNSIGNED(SHDR_MAGIC)
	bne	$t9, $at, L_0xbfc01938		# if (memcmp(&shdr->magic, "SHDR", sizeof(shdr->magic)) != 0) return 0;
	 nop
	lw	$t1, SHDR_OFFSET_SECTION_LEN($a1)
	move	$a0, $zero
	andi	$t2, $t1, 3
	beqz	$t2, L_0xbfc01940		# if ((shdr->section_len & 0x3) != 0) return 0;
	 nop
L_0xbfc01938: /* 0xbfc01938 */
	jr	$ra
	 move	$v0, $zero
L_0xbfc01940: /* 0xbfc01940 */
	beq	$a1, $v1, L_0xbfc01998		# $a1 & $v1 are different and const, so this is a dead branch
	 move	$a2, $a1
	subu	$t0, $v1, $a1
	andi	$t0, $t0, 0xf
	beqz	$t0, verify_section_checksum_loop		# This branch is always taken
	 addu	$a3, $t0, $a1
verify_shdr_checksum_loop: /* 0xbfc01958 */
	lw	$t3, 0($a2)
	addiu	$a2, $a2, 4
	bne	$a3, $a2, verify_shdr_checksum_loop
	 addu	$a0, $a0, $t3
	beql	$a2, $v1, L_0xbfc0199c
	 sltiu	$v0, $a0, 1
verify_section_checksum_loop: /* 0xbfc01970 */
	lw	$t4, 0($a2)
	lw	$t5, 4($a2)
	lw	$t6, 8($a2)
	addu	$a0, $a0, $t4
	lw	$t7, 0xc($a2)
	addu	$a0, $a0, $t5
	addiu	$a2, $a2, 0x10
	addu	$a0, $a0, $t6
	bne	$a2, $v0, verify_section_checksum_loop
	 addu	$a0, $a0, $t7
L_0xbfc01998: /* 0xbfc01998 */
	sltiu	$v0, $a0, 1
L_0xbfc0199c: /* 0xbfc0199c */
	jr	$ra
	 nop
	nop		# unreachable
	nop		# unreachable
	nop		# unreachable

/* Function tlb_init_uncached_trampoline [0xbfc019b0 - 0xbfc019c8)
 *
 * Jump to tlb_init through uncached KSEG1
 */
tlb_init_uncached_trampoline: /* 0xbfc019b0 */
	lui	$t1, %hi(tlb_init)
	lui	$t0, HI(KSEG1)
	addiu	$t1, $t1, %lo(tlb_init)
	or	$t0, $t0, $t1
	jr	$t0		# Jump to (KSEG1 | tlb_init)
	 nop

/* Function tlb_init [0xbfc019c8 - 0xbfc01d98)
 */
tlb_init: /* 0xbfc019c8 */
	mtc0	$zero, $CP0_PAGEMASK
	mtc0	$zero, $CP0_TAGHI
	li	$t1, PRID_IMP_R5000
	nop
	mfc0	$t0, $CP0_PRID
	andi	$t0, $t0, PRID_IMP_MASK
	srl	$t0, $t0, PRID_IMP_SHIFT
	beq	$t0, $t1, tlb_r5k_init
	 nop
	li	$t1, PRID_IMP_NEVADA
	beq	$t0, $t1, tlb_r5k_init
	 nop
	li	$t1, PRID_IMP_RM7000
	bne	$t0, $t1, tlb_r10k_init
	 nop
	li	$t0, RM7000_NUM_TLB_ENTRIES-1
	lui	$t1, HI(0x0fffe000)
tlb_rm7k_write_tlb_loop: /* 0xbfc01a0c */
	li	$at, PAGE_OFFSET_MASK
	not	$t2, $at
	and	$t2, $t2, $t1
	lui	$at, HI(KSEG0)
	mtc0	$t0, $CP0_INDEX
	or	$t2, $t2, $at
	mtc0	$t2, $CP0_ENTRYHI
	srl	$at, $t1, PAGE_SHIFT
	sll	$at, $at, ENTRYLO_PFN_SHIFT
	ori	$at, $at, (ENTRYLO_G|ENTRYLO_C_UNCACHED)
	mtc0	$at, $CP0_ENTRYLO0
	addiu	$t2, $at, 1 << ENTRYLO_PFN_SHIFT
	mtc0	$at, $CP0_ENTRYLO1
	addi	$t0, $t0, -1
	addiu	$t1, $t1, LO(0x0fffe000)
	bgtz	$t0, tlb_rm7k_write_tlb_loop
	 tlbwi
	mfc0	$t0, $CP0_CONFIG
	li	$at, ~RM7K_CONF_TE
	and	$t0, $t0, $at
	li	$at, ~CONF_CU
	and	$t0, $t0, $at
	mtc0	$t0, $CP0_CONFIG
	mfc0	$t0, $CP0_CONFIG
	srl	$t0, $t0, CONF_IC_SHIFT
	li	$t1, 0x1000
	andi	$t0, $t0, CONF_CACHE_SIZE_MASK
	sllv	$t0, $t1, $t0
	addi	$t0, $t0, -CACHE_LINE_SIZE
	lui	$t1, HI(KSEG0)
	addu	$t2, $t0, $t1
tlb_rm7k_inv_l1i_loop: /* 0xbfc01a88 */
	cache	(CACHE_TYPE_L1I|INDEX_WRITEBACK_INV), 0($t2)
	addi	$t0, $t0, -CACHE_LINE_SIZE
	bgez	$t0, tlb_rm7k_inv_l1i_loop
	 addu	$t2, $t0, $t1
	mfc0	$t0, $CP0_CONFIG
	srl	$t0, $t0, CONF_DC_SHIFT
	li	$t1, 0x1000
	andi	$t0, $t0, CONF_CACHE_SIZE_MASK
	sllv	$t0, $t1, $t0
	addi	$t0, $t0, -CACHE_LINE_SIZE
	lui	$t1, HI(KSEG0)
	lui	$at, 0x1000
tlb_rm7k_inv_l1d_loop: /* 0xbfc01ab8 */
	addu	$at, $at, $t0
	srl	$at, $at, PAGE_SHIFT
	sll	$at, $at, RM7K_TAGHI_PTAG_SHIFT
	mtc0	$at, $CP0_TAGHI
	addu	$t2, $t0, $t1
	addi	$t0, $t0, -CACHE_LINE_SIZE
	cache	(CACHE_TYPE_L1D|INDEX_STORE_TAG), 0($t2)
	bgez	$t0, tlb_rm7k_inv_l1d_loop
	 lui	$at, 0x1000
	jr	$ra
	 nop
tlb_r5k_init: /* 0xbfc01ae4 */
	li	$t0, R5000_NUM_TLB_ENTRIES-1
	lui	$t1, HI(0x0fffe000)
tlb_r5k_write_tlb_loop: /* 0xbfc01aec */
	li	$at, PAGE_OFFSET_MASK
	not	$t2, $at
	and	$t2, $t2, $t1
	lui	$at, HI(KSEG0)
	mtc0	$t0, $CP0_INDEX
	or	$t2, $t2, $at
	mtc0	$t2, $CP0_ENTRYHI
	srl	$at, $t1, PAGE_SHIFT
	sll	$at, $at, ENTRYLO_PFN_SHIFT
	ori	$at, $at, (ENTRYLO_G|ENTRYLO_C_UNCACHED)
	mtc0	$at, $CP0_ENTRYLO0
	addiu	$t2, $at, 1 << ENTRYLO_PFN_SHIFT
	mtc0	$at, $CP0_ENTRYLO1
	addi	$t0, $t0, -1
	addiu	$t1, $t1, LO(0x0fffe000)
	bgtz	$t0, tlb_r5k_write_tlb_loop
	 tlbwi
	mfc0	$t0, $CP0_CONFIG
	li	$at, ~R5K_CONF_SE
	and	$t0, $t0, $at
	mtc0	$t0, $CP0_CONFIG
	mfc0	$t0, $CP0_CONFIG
	srl	$t0, $t0, CONF_IC_SHIFT
	li	$t1, 0x1000
	andi	$t0, $t0, CONF_CACHE_SIZE_MASK
	sllv	$t0, $t1, $t0
	addi	$t0, $t0, -CACHE_LINE_SIZE
	lui	$t1, HI(KSEG0)
	addu	$t2, $t0, $t1
tlb_r5k_inv_l1i_loop: /* 0xbfc01b60 */
	cache	(CACHE_TYPE_L1I|INDEX_WRITEBACK_INV), 0($t2)
	addi	$t0, $t0, -CACHE_LINE_SIZE
	bgez	$t0, tlb_r5k_inv_l1i_loop
	 addu	$t2, $t0, $t1
	mfc0	$t0, $CP0_CONFIG
	srl	$t0, $t0, CONF_DC_SHIFT
	li	$t1, 0x1000
	andi	$t0, $t0, CONF_CACHE_SIZE_MASK
	sllv	$t0, $t1, $t0
	addi	$t0, $t0, -CACHE_LINE_SIZE
	lui	$t1, HI(KSEG0)
	lui	$at, 0x1000
tlb_r5k_inv_l1d_loop: /* 0xbfc01b90 */
	addu	$at, $at, $t0
	srl	$at, $at, PAGE_SHIFT
	sll	$at, $at, R5K_TAGLO_PTAG_SHIFT
	mtc0	$at, $CP0_TAGLO
	addu	$t2, $t0, $t1
	addi	$t0, $t0, -CACHE_LINE_SIZE
	cache	(CACHE_TYPE_L1D|INDEX_STORE_TAG), 0($t2)
	bgez	$t0, tlb_r5k_inv_l1d_loop
	 lui	$at, 0x1000
	jr	$ra
	 nop
tlb_r10k_init: /* 0xbfc01bbc */
	li	$t0, R10000_NUM_TLB_ENTRIES-1
	lui	$t1, HI(0x0fffe000)
tlb_r10k_write_tlb_loop: /* 0xbfc01bc4 */
	li	$at, PAGE_OFFSET_MASK
	not	$t2, $at
	and	$t2, $t2, $t1
	lui	$at, HI(KSEG1)
	mtc0	$t0, $CP0_INDEX
	or	$t2, $t2, $at
	mtc0	$t2, $CP0_ENTRYHI
	srl	$at, $t1, PAGE_SHIFT
	sll	$at, $at, ENTRYLO_PFN_SHIFT
	ori	$at, $at, (ENTRYLO_G|ENTRYLO_C_UNCACHED)
	mtc0	$at, $CP0_ENTRYLO0
	addiu	$t2, $at, 1 << ENTRYLO_PFN_SHIFT
	mtc0	$at, $CP0_ENTRYLO1
	addi	$t0, $t0, -1
	addiu	$t1, $t1, LO(0x0fffe000)
	bgtz	$t0, tlb_r10k_write_tlb_loop
	 tlbwi
	mtc0	$zero, $CP0_TAGHI
	mtc0	$zero, $CP0_TAGLO
	mtc0	$zero, $CP0_ECC
	lui	$t0, HI(KSEG0)
	li	$t1, 0x3fc0
tlb_r10k_inv_l1i_loop: /* 0xbfc01c1c */
	or	$t2, $t0, $t1
	cache	(CACHE_TYPE_L1I|INDEX_STORE_TAG), 0($t2)
	addi	$t1, $t1, -R10K_L1I_BLOCK_SIZE
	cache	(CACHE_TYPE_L1I|INDEX_STORE_TAG), 1($t2)
	bgez	$t1, tlb_r10k_inv_l1i_loop
	 nop
	mtc0	$zero, $CP0_TAGHI
	mtc0	$zero, $CP0_TAGLO
	mtc0	$zero, $CP0_ECC
	lui	$t0, HI(KSEG0)
	li	$t1, 0x3fe0
tlb_r10k_inv_l1d_loop: /* 0xbfc01c48 */
	or	$t2, $t0, $t1
	cache	(CACHE_TYPE_L1D|INDEX_STORE_TAG), 0($t2)
	addi	$t1, $t1, -R10K_L1D_BLOCK_SIZE
	cache	(CACHE_TYPE_L1D|INDEX_STORE_TAG), 1($t2)
	bgez	$t1, tlb_r10k_inv_l1d_loop
	 nop
	mfc0	$t0, $CP0_CONFIG
	lui	$t1, HI(ROM_SIZE)
	srl	$t2, $t0, R10K_CONF_SS_SHIFT
	andi	$t2, $t2, CONF_CACHE_SIZE_MASK
L_0xbfc01c70: /* 0xbfc01c70 */
	slti	$at, $t2, 6
	beqz	$at, L_0xbfc01c70
	 nop
	sllv	$t0, $t1, $t2
	andi	$at, $t0, 0x2000
	li	$t2, 0x80
	bgtz	$at, L_0xbfc01c94
	 nop
	li	$t2, 0x40
L_0xbfc01c94: /* 0xbfc01c94 */
	mtc0	$zero, $CP0_TAGHI
	mtc0	$zero, $CP0_TAGLO
	srl	$t1, $t0, 1
	mtc0	$zero, $CP0_ECC
	sub	$t1, $t1, $t2
tlb_r10k_inv_l2_loop: /* 0xbfc01ca8 */
	lui	$t3, HI(KSEG0)
	or	$t3, $t3, $t1
	cache	(CACHE_TYPE_L2|INDEX_STORE_TAG), 0($t3)
	sub	$t1, $t1, $t2
	cache	(CACHE_TYPE_L2|INDEX_STORE_TAG), 1($t3)
	bgez	$t1, tlb_r10k_inv_l2_loop
	 nop
	mtc0	$zero, $CP0_ECC
	mtc0	$zero, $CP0_TAGHI
	mtc0	$zero, $CP0_TAGLO
	nop
	move	$t0, $zero
tlb_r10k_set_virt_l1d_loop: /* 0xbfc01cd8 */
	lui	$t1, HI(KSEG0)
	or	$t1, $t1, $t0
	cache	(CACHE_TYPE_L1D|HIT_SET_VIRTUAL), 0($t1)
	addi	$t0, $t0, 4
	li	$t2, 0x4000
	cache	(CACHE_TYPE_L1D|HIT_SET_VIRTUAL), 1($t1)
	slt	$at, $t0, $t2
	bnez	$at, tlb_r10k_set_virt_l1d_loop
	 nop
	mtc0	$zero, $CP0_ECC
	mtc0	$zero, $CP0_TAGHI
	mtc0	$zero, $CP0_TAGLO
	nop
	move	$t0, $zero
tlb_r10k_set_virt_l1i_loop: /* 0xbfc01d10 */
	lui	$t1, HI(KSEG0)
	or	$t1, $t1, $t0
	cache	(CACHE_TYPE_L1I|HIT_SET_VIRTUAL), 0($t1)
	addi	$t0, $t0, 4
	li	$t2, 0x4000
	cache	(CACHE_TYPE_L1I|HIT_SET_VIRTUAL), 1($t1)
	slt	$at, $t0, $t2
	bnez	$at, tlb_r10k_set_virt_l1i_loop
	 nop
	mfc0	$t0, $CP0_CONFIG
	lui	$t1, HI(ROM_SIZE)
	srl	$t2, $t0, R10K_CONF_SS_SHIFT
	andi	$t2, $t2, CONF_CACHE_SIZE_MASK
L_0xbfc01d44: /* 0xbfc01d44 */
	slti	$at, $t2, 6
	beqz	$at, L_0xbfc01d44
	 nop
	sllv	$t0, $t1, $t2
	mtc0	$zero, $CP0_ECC
	srl	$t2, $t0, 1
	mtc0	$zero, $CP0_TAGHI
	mtc0	$zero, $CP0_TAGLO
	move	$t0, $zero
tlb_r10k_set_virt_l2_loop: /* 0xbfc01d68 */
	lui	$at, HI(KSEG0)
	or	$t1, $t0, $at
	cache	(CACHE_TYPE_L2|HIT_SET_VIRTUAL), 0($t1)
	addi	$t0, $t0, R10K_L2_BLOCK_SIZE
	cache	(CACHE_TYPE_L2|HIT_SET_VIRTUAL), 1($t1)
	slt	$at, $t0, $t2
	bnez	$at, tlb_r10k_set_virt_l2_loop
	 nop
	b	tlb_r10k_exit
	 nop
tlb_r10k_exit: /* 0xbfc01d90 */
	jr	$ra
	 nop

/* Function tlb_init_preserve [0xbfc01d98 - 0xbfc01de0)
 */
tlb_init_preserve: /* 0xbfc01d98 */
	addiu	$sp, $sp, -0x30		# unused?
	sw	$ra, 0x2c($sp)
	sw	$at, 0x28($sp)
	sw	$t0, 0x24($sp)
	sw	$t1, 0x20($sp)
	sw	$t2, 0x1c($sp)
	sw	$t3, 0x18($sp)
	bal	tlb_init_uncached_trampoline
	 sw	$t4, 0x14($sp)
	lw	$ra, 0x2c($sp)
	lw	$at, 0x28($sp)
	lw	$t0, 0x24($sp)
	lw	$t1, 0x20($sp)
	lw	$t2, 0x1c($sp)
	lw	$t3, 0x18($sp)
	lw	$t4, 0x14($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x30
data_0xbfc01de0: /* 0xbfc01de0 */
	string 4, "post1"
data_0xbfc01de8: /* 0xbfc01de8 */
	string 4, "firmware"
data_0xbfc01df4: /* 0xbfc01df4 */
	.int	0x06050308
	.int	0x03080808
data_0xbfc01dfc: /* 0xbfc01dfc */
	.fill	2, 4, 0x00000000
	.int	0x011c0002
	.int	0x00000002
	.int	0x01180004
	.int	0x00000004
	.int	0x011a0004
	.int	0x00000005
	.int	0x011d0002
	.int	0x00000000
	.int	0x011e0002
	.int	0x00000007
	.int	0x011f0000
	.int	0x00000005
	.fill	4, 4, 0x00000000
data_0xbfc01e44: /* 0xbfc01e44 */
	string 4, "\n\rSL-"
data_0xbfc01e4c: /* 0xbfc01e4c */
	string 4, "-8E"
data_0xbfc01e50: /* 0xbfc01e50 */
	.int	0x000012c0
data_0xbfc01e54: /* 0xbfc01e54 */
	.int	0x00002580
	.int	0x00004b00
	.int	0x00009600
	.int	0x0000e100
	.int	0x0001c200
	.fill	2, 4, 0x00000000
data_0xbfc01e70: /* 0xbfc01e70 */
	.int	F_0xbfc00000
	.int	ROM_END
	.int	ROM_ALIGN
	.int	ROM_SIZE
	.int	F_0xbfc00c48
	.int	nop1
	.int	isa_reset
data_0xbfc01e8c: /* 0xbfc01e8c */
	.int	BASE_UART_1 + UART_DATA
data_0xbfc01e90: /* 0xbfc01e90 */
	.int	BASE_UART_1 + UART_IER
data_0xbfc01e94: /* 0xbfc01e94 */
	.int	BASE_UART_1 + UART_IIR
data_0xbfc01e98: /* 0xbfc01e98 */
	.int	BASE_UART_1 + UART_LCR
data_0xbfc01e9c: /* 0xbfc01e9c */
	.int	BASE_UART_1 + UART_MCR
data_0xbfc01ea0: /* 0xbfc01ea0 */
	.int	BASE_UART_1 + UART_LSR
	.int	BASE_UART_1 + UART_MSR
	.fill	2133, 4, 0x00000000

	section_checksum
	section_end sloader
