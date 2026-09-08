/* Section: post1 */
/* Version: 1.0 */
/* Type: SECTION_TYPE_CODE */
/* Length: 0x4d44 (19780 bytes) */

#include "definitions.h"
#include "macros.inc"

.set noreorder
.set noat


/* Function post1_entry [0xbfc04400 - 0xbfc046a0)
 *
 * post1 entry
 */
post1_entry: /* 0xbfc04400 */
	b	post1
	 nop

	section_start post1
	shdr post1, 1.0, SECTION_TYPE_CODE

	sentinel 0x00000000

post1: /* 0xbfc04448 */
	addiu	$sp, $sp, -0x30
	sw	$ra, 0x2c($sp)
	sw	$s7, 0x24($sp)
	sw	$s6, 0x20($sp)
	sw	$s5, 0x1c($sp)
	sw	$s4, 0x18($sp)
	sw	$s3, 0x14($sp)
	sw	$s2, 0x10($sp)
	sw	$s1, 0xc($sp)
	sw	$s0, 8($sp)
	sw	$a0, 0x30($sp)
	sw	$a1, 0x34($sp)		# Save address of firmware section
	sw	$a2, 0x38($sp)
	jal	F_0xbfc05ad4
	 nop
	lui	$a0, 0x4000
	jal	F_0xbfc061f0
	 li	$a1, 0x8000
	jal	F_0xbfc060a0
	 nop
	lui	$t0, HI(POST1_STACK)
	sd	$sp, LO(POST1_STACK + 8)($t0)
	lui	$t1, HI(KSEG1)
	addiu	$t0, $t0, LO(POST1_STACK)
	jal	tlb_init_preserve
	 or	$sp, $sp, $t1
	lui	$a0, HI(0xbfc06fd0)
	lui	$a1, %hi(data_0xbfc09120)
	lui	$a2, %hi(F_0xa0004000)
	addiu	$a0, $a0, LO(0xbfc06fd0)		# 0xbfc06fd0, start of loadable subsection
	addiu	$a1, $a1, %lo(data_0xbfc09120)		# 0xbfc09120, end of loadable subsection
	addiu	$a2, $a2, %lo(F_0xa0004000)		# 0xa0004000, address of destination
	jal	F_0xbfc060f8
	 li	$a3, 0
	lui	$a0, %hi(F_0xa0004000)
	addiu	$a0, $a0, %lo(F_0xa0004000)
	jalr	$a0
	 nop
	lui	$t0, HI(POST1_STACK)
	addiu	$t0, $t0, LO(POST1_STACK)
	ld	$t0, 8($t0)
	lui	$t1, 0x2000
	xor	$t1, $t1, $sp
	bne	$t0, $t1, post1_error_return
	 nop
	lui	$t0, HI(BASE_ISA + ISA_MISC_CONTROL)
	ld	$t1, LO(BASE_ISA + ISA_MISC_CONTROL)($t0)
	li	$t2, ~(ISA_RED_LED|ISA_GREEN_LED)
	addiu	$t0, $t0, LO(BASE_ISA + ISA_MISC_CONTROL)
	and	$t1, $t1, $t2
	sd	$t1, 0($t0)		# ISA_MISC_CONTROL
	lw	$t0, 0x34($sp)		# $t0 = address of firmware section
	li	$t2, (SECTION_TYPE_CODE|SECTION_TYPE_LOADABLE)
	lbu	$t1, SHDR_OFFSET_SECTION_TYPE($t0)
	bne	$t2, $t1, post1_error_return
	 nop
	move	$s0, $t0		# $s0 = address of firmware section
	move	$s7, $sp
	addiu	$s7, $s7, 0xfff
	li	$t0, -0x1000
	and	$s7, $s7, $t0
	addu	$s6, $s7, $t0
	lui	$t0, HI_UNSIGNED(0x1fffffff)
	ori	$t0, $t0, LO_UNSIGNED(0x1fffffff)
	lui	$s4, %hi(.Llocal1)
	lui	$s5, %hi(F_0xbfc04780)
	addiu	$s4, $s4, %lo(.Llocal1)
	addiu	$s5, $s5, %lo(F_0xbfc04780)
	move	$s1, $zero
	and	$s7, $s7, $t0
	and	$s6, $s6, $t0
	and	$s5, $s5, $t0
	and	$s4, $s4, $t0
.Llocal1: /* 0xbfc0456c */
	nop
	nop
	lui	$t0, HI(BASE_CRIME + CRIME_MC_STATUS_CTRL)
	addiu	$t0, $t0, LO(BASE_CRIME + CRIME_MC_STATUS_CTRL)
	sd	$zero, 0($t0)
	li	$s2, SHDR_OFFSET_SUBSECTION_HEADER
	move	$s3, $zero
next_subsection: /* 0xbfc04588 */
	addu	$s2, $s0, $s2		# $s2 = address of subsection header
	beqz	$s3, load_subsection_len
	 lw	$a0, SUBSECTION_HEADER_OFFSET_ADDR($s2)
	addu	$s1, $s1, $a0
load_subsection_len: /* 0xbfc04598 */
	lw	$a1, SUBSECTION_HEADER_OFFSET_LEN($s2)
	beqz	$s3, copy_subsection
	 move	$s2, $zero
	addu	$s1, $s1, $a1
copy_subsection: /* 0xbfc045a8 */
	beqz	$a1, verify_checksum_and_jump
	 nop
	bal	validate_copy_dest
	 nop
	bnez	$v0, post1_error_return
	 nop
	lui	$t2, HI(KSEG1)
	addu	$t0, $s0, $s3
	or	$t1, $a0, $t2
	li	$s3, 8
	move	$t2, $a1
copy_subsection_loop: /* 0xbfc045d4 */
	lw	$t3, 0($t0)
	addu	$s1, $s1, $t3
	sw	$t3, 0($t1)
	lw	$t3, 4($t0)
	addu	$s1, $s1, $t3
	sw	$t3, 4($t1)
	lw	$t3, 8($t0)
	addu	$s1, $s1, $t3
	sw	$t3, 8($t1)
	lw	$t3, 0xc($t0)
	addu	$s1, $s1, $t3
	sw	$t3, 0xc($t1)
	addiu	$t2, $t2, -0x10
	addiu	$t0, $t0, 0x10
	bnez	$t2, copy_subsection_loop
	 addiu	$t1, $t1, 0x10
	b	next_subsection
	 move	$s0, $t0
verify_checksum_and_jump: /* 0xbfc0461c */
	lw	$t3, SUBSECTION_HEADER_SIZE($s0)		# Load section checksum
	addu	$s1, $s1, $t3
	bnez	$s1, post1_error_return		# Branch if checksum fails
	 nop
	mfc0	$t0, $CP0_CONFIG
	nop
	li	$t1, ~CONF_CM_CMASK
	nop
	and	$t1, $t0, $t1
	ori	$t1, $t1, CONF_CM_CACHABLE_NONCOHERENT
	mtc0	$t1, $CP0_CONFIG		# Set KSEG0 to cacheable, noncoherent
	nop
	nop
	nop
	nop
	jal	tlb_init_preserve
	 nop
	move	$t0, $a0
	lw	$a0, 0x30($sp)
	jr	$t0
	 lw	$a1, 0x38($sp)
post1_error_return: /* 0xbfc04670 */
	lw	$ra, 0x2c($sp)
	lw	$s7, 0x24($sp)
	lw	$s6, 0x20($sp)
	lw	$s5, 0x1c($sp)
	lw	$s4, 0x18($sp)
	lw	$s3, 0x14($sp)
	lw	$s2, 0x10($sp)
	lw	$s1, 0xc($sp)
	lw	$s0, 8($sp)
	addiu	$sp, $sp, 0x30
	jr	$ra
	 li	$v0, 1

/* Function validate_copy_dest [0xbfc046a0 - 0xbfc046f8)
 */
validate_copy_dest: /* 0xbfc046a0 */
	andi	$v0, $a0, 3
	bnez	$v0, L_0xbfc046f0
	 nop
	andi	$v0, $a1, 3
	bnez	$v0, L_0xbfc046f0
	 nop
	lui	$t2, HI_UNSIGNED(0x1fffffff)
	ori	$t2, $t2, LO_UNSIGNED(0x1fffffff)
	and	$t1, $t1, $t2
	addu	$t0, $a0, $a1
	sltu	$v0, $t1, $s6
	beqz	$v0, L_0xbfc046e0
	 and	$t0, $a0, $t2
	sltu	$v0, $s7, $t0
	bnez	$v0, L_0xbfc046f0
	 nop
L_0xbfc046e0: /* 0xbfc046e0 */
	sltu	$v0, $t1, $s4
	beqz	$v0, L_0xbfc046f0
	 nop
	sltu	$v0, $s5, $t0
L_0xbfc046f0: /* 0xbfc046f0 */
	jr	$ra
	 nop

/* Function F_0xbfc046f8 [0xbfc046f8 - 0xbfc04780)
 */
F_0xbfc046f8: /* 0xbfc046f8 */
	srl	$t0, $a0, 0x1d
	li	$at, 4
	bne	$t0, $at, L_0xbfc04778
	 nop
	mfc0	$t0, $CP0_CONFIG
	andi	$t4, $t0, CONF_DB
	beqz	$t4, L_0xbfc0471c
	 li	$t1, 0x10
	li	$t1, 0x20
L_0xbfc0471c: /* 0xbfc0471c */
	srl	$t4, $t0, 0x10
	andi	$t0, $t4, HI(CONF_SC)
	beqz	$t0, L_0xbfc04750
	 nop
	andi	$t4, $t4, HI(CONF_SB)
	srl	$t4, $t4, 6
	addi	$t4, $t4, 4
	li	$t3, 1
	sllv	$t4, $t3, $t4
	slt	$t3, $t4, $t1
	beqz	$t3, L_0xbfc04750
	 nop
	move	$t1, $t4
L_0xbfc04750: /* 0xbfc04750 */
	move	$t2, $a0
	add	$t3, $t2, $a1
	add	$t3, $t3, $t1
L_0xbfc0475c: /* 0xbfc0475c */
	cache	(CACHE_TYPE_L1D|HIT_WRITEBACK), 0($t2)
	beqz	$t0, L_0xbfc04768
	 cache	(CACHE_TYPE_L2|HIT_WRITEBACK), 0($t2)
L_0xbfc04768: /* 0xbfc04768 */
	addu	$t2, $t2, $t1
	sltu	$t4, $t2, $t3
	bnez	$t4, L_0xbfc0475c
	 nop
L_0xbfc04778: /* 0xbfc04778 */
	jr	$ra
	 nop

/* Function F_0xbfc04780 [0xbfc04780 - 0xbfc04790)
 */
F_0xbfc04780: /* 0xbfc04780 */
	jr	$ra
	 nop
	nop		# unreachable
	nop		# unreachable

/* Function F_0xbfc04790 [0xbfc04790 - 0xbfc048f8)
 */
F_0xbfc04790: /* 0xbfc04790 */
	addiu	$sp, $sp, -0x68
	sw	$s0, 0x14($sp)
	sw	$a0, 0x68($sp)
	sw	$a1, 0x6c($sp)
	ld	$s0, 0x68($sp)
	sw	$s1, 0x18($sp)
	move	$s1, $a2
	sw	$ra, 0x1c($sp)
	bnez	$s0, L_0xbfc047d0
	 sw	$a3, 0x74($sp)
	bnezl	$a2, L_0xbfc047d4
	 li	$t6, 0x30
	jal	putchar
	 li	$a0, 0x30
	b	L_0xbfc048e8
	 lw	$ra, 0x1c($sp)
L_0xbfc047d0: /* 0xbfc047d0 */
	li	$t6, 0x30
L_0xbfc047d4: /* 0xbfc047d4 */
	li	$t7, 0x30
	sb	$t7, 0x51($sp)
	sb	$t6, 0x50($sp)
	addiu	$v1, $sp, 0x52
	addiu	$a0, $sp, 0x66
	li	$v0, 0x30
L_0xbfc047ec: /* 0xbfc047ec */
	addiu	$v1, $v1, 4
	sb	$v0, -4($v1)
	sb	$v0, -3($v1)
	sb	$v0, -2($v1)
	bne	$v1, $a0, L_0xbfc047ec
	 sb	$v0, -1($v1)
	sltiu	$at, $s0, 0xb
	move	$a2, $zero
	bnez	$at, L_0xbfc04874
	 move	$a1, $s0
	lui	$t0, %hi(data_0xbfc06fb0)
	addiu	$t0, $t0, %lo(data_0xbfc06fb0)
	addiu	$s0, $sp, 0x50
	li	$a3, 0xa
L_0xbfc04824: /* 0xbfc04824 */
	li	$at, 0xa
	ddivu	$zero, $a1, $at
	addiu	$s0, $s0, 1
	mflo	$t8
	dsll32	$v0, $t8, 0
	dsra32	$v0, $v0, 0
	multu	$v0, $a3
	mflo	$t9
	dsubu	$t2, $a1, $t9
	dsll32	$v1, $t2, 0
	dsra32	$v1, $v1, 0
	addu	$t3, $t0, $v1
	lbu	$t4, 0($t3)
	move	$a1, $v0
	beqz	$v1, L_0xbfc04868
	 sb	$t4, -1($s0)
	sw	$a2, 0x3c($sp)
L_0xbfc04868: /* 0xbfc04868 */
	sltiu	$at, $a1, 0xb
	beqz	$at, L_0xbfc04824
	 addiu	$a2, $a2, 1
L_0xbfc04874: /* 0xbfc04874 */
	lui	$t0, %hi(data_0xbfc06fb0)
	dsll32	$t5, $a1, 0
	addiu	$t0, $t0, %lo(data_0xbfc06fb0)
	dsrl32	$t5, $t5, 0
	addu	$t6, $t0, $t5
	lbu	$t7, 0($t6)
	addu	$t8, $sp, $a2
	beqz	$a1, L_0xbfc0489c
	 sb	$t7, 0x50($t8)
	sw	$a2, 0x3c($sp)
L_0xbfc0489c: /* 0xbfc0489c */
	beqz	$s1, L_0xbfc048b4
	 lw	$t9, 0x74($sp)
	beqzl	$t9, L_0xbfc048b8
	 lw	$a2, 0x3c($sp)
	b	L_0xbfc048b8
	 move	$a2, $t9
L_0xbfc048b4: /* 0xbfc048b4 */
	lw	$a2, 0x3c($sp)
L_0xbfc048b8: /* 0xbfc048b8 */
	bltz	$a2, L_0xbfc048e4
	 addiu	$t1, $sp, 0x50
	addu	$s0, $a2, $t1
	addiu	$s1, $sp, 0x50
L_0xbfc048c8: /* 0xbfc048c8 */
	jal	putchar
	 lbu	$a0, 0($s0)
	li	$t2, -1
	addu	$s0, $s0, $t2
	sltu	$at, $s0, $s1
	beqz	$at, L_0xbfc048c8
	 nop
L_0xbfc048e4: /* 0xbfc048e4 */
	lw	$ra, 0x1c($sp)
L_0xbfc048e8: /* 0xbfc048e8 */
	lw	$s0, 0x14($sp)
	lw	$s1, 0x18($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x68

/* Function F_0xbfc048f8 [0xbfc048f8 - 0xbfc04a9c)
 */
F_0xbfc048f8: /* 0xbfc048f8 */
	addiu	$sp, $sp, -0x68
	sw	$s0, 0x14($sp)
	sw	$a0, 0x68($sp)
	sw	$a1, 0x6c($sp)
	ld	$s0, 0x68($sp)
	sw	$ra, 0x1c($sp)
	sw	$s1, 0x18($sp)
	sw	$a2, 0x70($sp)
	bnez	$s0, L_0xbfc04938
	 sw	$a3, 0x74($sp)
	bnezl	$a2, L_0xbfc0493c
	 li	$t7, 0x30
	jal	putchar
	 li	$a0, 0x30
	b	L_0xbfc04a8c
	 lw	$ra, 0x1c($sp)
L_0xbfc04938: /* 0xbfc04938 */
	li	$t7, 0x30
L_0xbfc0493c: /* 0xbfc0493c */
	li	$t8, 0x30
	sb	$t8, 0x51($sp)
	sb	$t7, 0x50($sp)
	addiu	$v1, $sp, 0x52
	addiu	$a0, $sp, 0x66
	li	$v0, 0x30
L_0xbfc04954: /* 0xbfc04954 */
	addiu	$v1, $v1, 4
	sb	$v0, -4($v1)
	sb	$v0, -3($v1)
	sb	$v0, -2($v1)
	bne	$v1, $a0, L_0xbfc04954
	 sb	$v0, -1($v1)
	bgez	$s0, L_0xbfc04980
	 move	$a2, $zero
	li	$s1, 0x2d
	b	L_0xbfc04988
	 dsubu	$a1, $zero, $s0
L_0xbfc04980: /* 0xbfc04980 */
	li	$s1, 0x20
	move	$a1, $s0
L_0xbfc04988: /* 0xbfc04988 */
	slti	$at, $a1, 0xb
	bnez	$at, L_0xbfc049f8
	 dsll32	$s1, $s1, 0
	addiu	$t9, $sp, 0x50
	lui	$t0, %hi(data_0xbfc06fb0)
	addiu	$t0, $t0, %lo(data_0xbfc06fb0)
	addu	$s0, $a2, $t9
	li	$a3, 0xa
L_0xbfc049a8: /* 0xbfc049a8 */
	li	$at, 0xa
	ddiv	$zero, $a1, $at
	addiu	$s0, $s0, 1
	mflo	$t1
	dsll32	$v0, $t1, 0
	dsra32	$v0, $v0, 0
	multu	$v0, $a3
	mflo	$t2
	dsubu	$t4, $a1, $t2
	dsll32	$v1, $t4, 0
	dsra32	$v1, $v1, 0
	addu	$t5, $t0, $v1
	lbu	$t6, 0($t5)
	move	$a1, $v0
	beqz	$v1, L_0xbfc049ec
	 sb	$t6, -1($s0)
	sw	$a2, 0x3c($sp)
L_0xbfc049ec: /* 0xbfc049ec */
	slti	$at, $a1, 0xb
	beqz	$at, L_0xbfc049a8
	 addiu	$a2, $a2, 1
L_0xbfc049f8: /* 0xbfc049f8 */
	lui	$t0, %hi(data_0xbfc06fb0)
	dsll32	$t7, $a1, 0
	addiu	$t0, $t0, %lo(data_0xbfc06fb0)
	dsrl32	$t7, $t7, 0
	addu	$t8, $t0, $t7
	lbu	$t9, 0($t8)
	addu	$t1, $sp, $a2
	beqz	$a1, L_0xbfc04a20
	 sb	$t9, 0x50($t1)
	sw	$a2, 0x3c($sp)
L_0xbfc04a20: /* 0xbfc04a20 */
	lw	$t2, 0x70($sp)
	lw	$t3, 0x74($sp)
	dsra32	$s1, $s1, 0
	beqz	$t2, L_0xbfc04a44
	 li	$at, 0x2d
	beqzl	$t3, L_0xbfc04a48
	 lw	$a2, 0x3c($sp)
	b	L_0xbfc04a48
	 move	$a2, $t3
L_0xbfc04a44: /* 0xbfc04a44 */
	lw	$a2, 0x3c($sp)
L_0xbfc04a48: /* 0xbfc04a48 */
	bne	$s1, $at, L_0xbfc04a5c
	 andi	$a0, $s1, 0xff
	jal	putchar
	 sw	$a2, 0x48($sp)
	lw	$a2, 0x48($sp)
L_0xbfc04a5c: /* 0xbfc04a5c */
	bltz	$a2, L_0xbfc04a88
	 addiu	$t5, $sp, 0x50
	addu	$s0, $a2, $t5
	addiu	$s1, $sp, 0x50
L_0xbfc04a6c: /* 0xbfc04a6c */
	jal	putchar
	 lbu	$a0, 0($s0)
	li	$t6, -1
	addu	$s0, $s0, $t6
	sltu	$at, $s0, $s1
	beqz	$at, L_0xbfc04a6c
	 nop
L_0xbfc04a88: /* 0xbfc04a88 */
	lw	$ra, 0x1c($sp)
L_0xbfc04a8c: /* 0xbfc04a8c */
	lw	$s0, 0x14($sp)
	lw	$s1, 0x18($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x68

/* Function F_0xbfc04a9c [0xbfc04a9c - 0xbfc04bdc)
 */
F_0xbfc04a9c: /* 0xbfc04a9c */
	addiu	$sp, $sp, -0x38
	sw	$s3, 0x24($sp)
	sw	$a0, 0x38($sp)
	sw	$a1, 0x3c($sp)
	ld	$s3, 0x38($sp)
	sw	$s6, 0x30($sp)
	sw	$s1, 0x1c($sp)
	sw	$s0, 0x18($sp)
	move	$s6, $a2
	sw	$ra, 0x34($sp)
	sw	$s5, 0x2c($sp)
	sw	$s4, 0x28($sp)
	sw	$s2, 0x20($sp)
	move	$s0, $a3
	bnez	$s3, L_0xbfc04af4
	 move	$s1, $zero
	bnez	$a2, L_0xbfc04af4
	 nop
	jal	putchar
	 li	$a0, 0x30
	b	L_0xbfc04bb8
	 lw	$ra, 0x34($sp)
L_0xbfc04af4: /* 0xbfc04af4 */
	bltz	$a3, L_0xbfc04bb4
	 li	$s5, 0x30
	lui	$s2, %hi(data_0xbfc06fb0)
	addiu	$s2, $s2, %lo(data_0xbfc06fb0)
	li	$s4, 0x58
L_0xbfc04b08: /* 0xbfc04b08 */
	bnez	$s0, L_0xbfc04b28
	 dsrlv	$t0, $s3, $s0
	andi	$t6, $s3, 0xf
	dsll32	$t7, $t6, 0
	dsrl32	$t7, $t7, 0
	addu	$t8, $s2, $t7
	b	L_0xbfc04b3c
	 lbu	$a0, 0($t8)
L_0xbfc04b28: /* 0xbfc04b28 */
	andi	$t1, $t0, 0xf
	dsll32	$t2, $t1, 0
	dsrl32	$t2, $t2, 0
	addu	$t3, $s2, $t2
	lbu	$a0, 0($t3)
L_0xbfc04b3c: /* 0xbfc04b3c */
	dsll32	$a0, $a0, 0
	dsra32	$a0, $a0, 0
	bne	$s4, $a0, L_0xbfc04b74
	 move	$v0, $a0
	slti	$at, $v0, 0x3a
	bnez	$at, L_0xbfc04b74
	 li	$t5, -0x20
	addu	$t4, $v0, $t5
	dsll32	$t4, $t4, 0
	dsrl32	$t4, $t4, 0
	andi	$a0, $t4, 0xff
	dsll32	$a0, $a0, 0
	dsra32	$a0, $a0, 0
	move	$v0, $a0
L_0xbfc04b74: /* 0xbfc04b74 */
	beq	$s5, $v0, L_0xbfc04b8c
	 nop
	jal	putchar
	 addiu	$s1, $s1, 1
	b	L_0xbfc04ba8
	 li	$t6, -4
L_0xbfc04b8c: /* 0xbfc04b8c */
	bnez	$s6, L_0xbfc04b9c
	 nop
	beqzl	$s1, L_0xbfc04ba8
	 li	$t6, -4
L_0xbfc04b9c: /* 0xbfc04b9c */
	jal	putchar
	 nop
	li	$t6, -4
L_0xbfc04ba8: /* 0xbfc04ba8 */
	addu	$s0, $s0, $t6
	bgez	$s0, L_0xbfc04b08
	 nop
L_0xbfc04bb4: /* 0xbfc04bb4 */
	lw	$ra, 0x34($sp)
L_0xbfc04bb8: /* 0xbfc04bb8 */
	lw	$s0, 0x18($sp)
	lw	$s1, 0x1c($sp)
	lw	$s2, 0x20($sp)
	lw	$s3, 0x24($sp)
	lw	$s4, 0x28($sp)
	lw	$s5, 0x2c($sp)
	lw	$s6, 0x30($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x38

/* Function F_0xbfc04bdc [0xbfc04bdc - 0xbfc04ce0)
 */
F_0xbfc04bdc: /* 0xbfc04bdc */
	addiu	$sp, $sp, -0x30
	sw	$s3, 0x20($sp)
	sw	$a0, 0x30($sp)
	sw	$a1, 0x34($sp)
	ld	$s3, 0x30($sp)
	sw	$s5, 0x28($sp)
	sw	$s1, 0x18($sp)
	sw	$s0, 0x14($sp)
	move	$s5, $a2
	sw	$ra, 0x2c($sp)
	sw	$s4, 0x24($sp)
	sw	$s2, 0x1c($sp)
	move	$s0, $a3
	bnez	$s3, L_0xbfc04c30
	 move	$s1, $zero
	bnez	$a2, L_0xbfc04c30
	 nop
	jal	putchar
	 li	$a0, 0x30
	b	L_0xbfc04cc0
	 lw	$ra, 0x2c($sp)
L_0xbfc04c30: /* 0xbfc04c30 */
	bltz	$a3, L_0xbfc04cbc
	 li	$s4, 0x30
	lui	$s2, %hi(data_0xbfc06fb0)
	addiu	$s2, $s2, %lo(data_0xbfc06fb0)
L_0xbfc04c40: /* 0xbfc04c40 */
	bnez	$s0, L_0xbfc04c60
	 dsrlv	$t0, $s3, $s0
	andi	$t6, $s3, 7
	dsll32	$t7, $t6, 0
	dsrl32	$t7, $t7, 0
	addu	$t8, $s2, $t7
	b	L_0xbfc04c74
	 lbu	$a0, 0($t8)
L_0xbfc04c60: /* 0xbfc04c60 */
	andi	$t1, $t0, 7
	dsll32	$t2, $t1, 0
	dsrl32	$t2, $t2, 0
	addu	$t3, $s2, $t2
	lbu	$a0, 0($t3)
L_0xbfc04c74: /* 0xbfc04c74 */
	dsll32	$a0, $a0, 0
	dsra32	$a0, $a0, 0
	beq	$s4, $a0, L_0xbfc04c94
	 nop
	jal	putchar
	 addiu	$s1, $s1, 1
	b	L_0xbfc04cb0
	 li	$t5, -3
L_0xbfc04c94: /* 0xbfc04c94 */
	bnez	$s5, L_0xbfc04ca4
	 nop
	beqzl	$s1, L_0xbfc04cb0
	 li	$t5, -3
L_0xbfc04ca4: /* 0xbfc04ca4 */
	jal	putchar
	 nop
	li	$t5, -3
L_0xbfc04cb0: /* 0xbfc04cb0 */
	addu	$s0, $s0, $t5
	bgez	$s0, L_0xbfc04c40
	 nop
L_0xbfc04cbc: /* 0xbfc04cbc */
	lw	$ra, 0x2c($sp)
L_0xbfc04cc0: /* 0xbfc04cc0 */
	lw	$s0, 0x14($sp)
	lw	$s1, 0x18($sp)
	lw	$s2, 0x1c($sp)
	lw	$s3, 0x20($sp)
	lw	$s4, 0x24($sp)
	lw	$s5, 0x28($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x30

/* Function F_0xbfc04ce0 [0xbfc04ce0 - 0xbfc04d74)
 */
F_0xbfc04ce0: /* 0xbfc04ce0 */
	addiu	$sp, $sp, -0x28
	sw	$s0, 0x18($sp)
	sw	$ra, 0x24($sp)
	sw	$s2, 0x20($sp)
	sw	$s1, 0x1c($sp)
	sw	$a0, 0x28($sp)
	sw	$a1, 0x2c($sp)
	bltz	$a2, L_0xbfc04d5c
	 move	$s0, $a2
	lui	$s2, %hi(data_0xbfc06fb0)
	addiu	$s2, $s2, %lo(data_0xbfc06fb0)
	ld	$s1, 0x28($sp)
L_0xbfc04d10: /* 0xbfc04d10 */
	bnez	$s0, L_0xbfc04d30
	 dsrlv	$t0, $s1, $s0
	andi	$t6, $s1, 1
	dsll32	$t7, $t6, 0
	dsrl32	$t7, $t7, 0
	addu	$t8, $s2, $t7
	b	L_0xbfc04d44
	 lbu	$a0, 0($t8)
L_0xbfc04d30: /* 0xbfc04d30 */
	andi	$t1, $t0, 1
	dsll32	$t2, $t1, 0
	dsrl32	$t2, $t2, 0
	addu	$t3, $s2, $t2
	lbu	$a0, 0($t3)
L_0xbfc04d44: /* 0xbfc04d44 */
	jal	putchar
	 nop
	li	$t4, -1
	addu	$s0, $s0, $t4
	bgez	$s0, L_0xbfc04d10
	 nop
L_0xbfc04d5c: /* 0xbfc04d5c */
	lw	$ra, 0x24($sp)
	lw	$s0, 0x18($sp)
	lw	$s1, 0x1c($sp)
	lw	$s2, 0x20($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x28

/* Function printf_disabled [0xbfc04d74 - 0xbfc04d90)
 */
printf_disabled: /* 0xbfc04d74 */
	addiu	$sp, $sp, -8
	sw	$a0, 8($sp)
	sw	$a1, 0xc($sp)
	sw	$a2, 0x10($sp)
	sw	$a3, 0x14($sp)
	jr	$ra
	 addiu	$sp, $sp, 8

/* Function delay [0xbfc04d90 - 0xbfc04dd4)
 */
delay: /* 0xbfc04d90 */
	beqz	$a0, L_0xbfc04dcc
	 move	$v0, $a0
	andi	$a1, $a0, 3
	negu	$a1, $a1
	beqz	$a1, L_0xbfc04dbc
	 addu	$v1, $a1, $a0
	li	$t6, -1
L_0xbfc04dac: /* 0xbfc04dac */
	addu	$v0, $v0, $t6
	bnel	$v1, $v0, L_0xbfc04dac
	 li	$t6, -1
	beqz	$v0, L_0xbfc04dcc
L_0xbfc04dbc: /* 0xbfc04dbc */
	 li	$t7, -4
L_0xbfc04dc0: /* 0xbfc04dc0 */
	addu	$v0, $v0, $t7
	bnezl	$v0, L_0xbfc04dc0
	 li	$t7, -4
L_0xbfc04dcc: /* 0xbfc04dcc */
	jr	$ra
	 nop

/* Function F_0xbfc04dd4 [0xbfc04dd4 - 0xbfc04df8)
 */
F_0xbfc04dd4: /* 0xbfc04dd4 */
	lui	$t6, HI(0xbfd00703)
	lbu	$t7, LO(0xbfd00703)($t6)
	dsll32	$t7, $t7, 0
	dsra32	$t7, $t7, 0
	xori	$t9, $t7, 0xde
	sltiu	$t9, $t9, 1
	dsll32	$t9, $t9, 0
	jr	$ra
	 dsrl32	$v0, $t9, 0

/* Function F_0xbfc04df8 [0xbfc04df8 - 0xbfc04e90)
 */
F_0xbfc04df8: /* 0xbfc04df8 */
	addiu	$sp, $sp, -0x18
	sw	$ra, 0x14($sp)
	li	$t6, 0x80
	lui	$t7, HI(0xbfd00380)
	sb	$t6, LO(0xbfd00380)($t7)
	jal	delay
	 li	$a0, 0x20
	lui	$t8, HI(0xbfd00100)
	sb	$zero, LO(0xbfd00100)($t8)
	jal	delay
	 li	$a0, 0x20
	li	$t9, 0x18
	lui	$t0, HI(0xbfd00018)
	sb	$t9, LO(0xbfd00018)($t0)
	jal	delay
	 li	$a0, 0x20
	li	$t1, 3
	lui	$t2, HI(0xbfd00303)
	sb	$t1, LO(0xbfd00303)($t2)
	jal	delay
	 li	$a0, 0x20
	lui	$t3, HI(0xbfd00100)
	sb	$zero, LO(0xbfd00100)($t3)
	jal	delay
	 li	$a0, 0x20
	lui	$t4, HI(0xbfd00400)
	sb	$zero, LO(0xbfd00400)($t4)
	jal	delay
	 li	$a0, 0x20
	li	$t5, 0xde
	lui	$t6, HI(0xbfd007de)
	sb	$t5, LO(0xbfd007de)($t6)
	jal	delay
	 li	$a0, 0x20
	lw	$ra, 0x14($sp)
	addiu	$sp, $sp, 0x18
	jr	$ra
	 nop

/* Function F_0xbfc04e90 [0xbfc04e90 - 0xbfc04ee4)
 */
F_0xbfc04e90: /* 0xbfc04e90 */
	lui	$v0, HI_UNSIGNED(0xbfd00503)
	ori	$v0, $v0, LO_UNSIGNED(0xbfd00503)
	lbu	$t6, 0($v0)
	lui	$a0, HI_UNSIGNED(0xbfd00003)
	li	$v1, -1
	dsll32	$t6, $t6, 0
	dsra32	$t6, $t6, 0
	andi	$t8, $t6, 1
	beqz	$t8, L_0xbfc04edc
	 ori	$a0, $a0, LO_UNSIGNED(0xbfd00003)
L_0xbfc04eb8: /* 0xbfc04eb8 */
	lbu	$t9, 0($a0)
	lbu	$t0, 0($v0)
	dsll32	$t9, $t9, 0
	dsll32	$t0, $t0, 0
	dsra32	$t0, $t0, 0
	andi	$t2, $t0, 1
	dsra32	$t9, $t9, 0
	bnez	$t2, L_0xbfc04eb8
	 andi	$v1, $t9, 0xff
L_0xbfc04edc: /* 0xbfc04edc */
	jr	$ra
	 move	$v0, $v1

/* Function F_0xbfc04ee4 [0xbfc04ee4 - 0xbfc04f40)
 */
F_0xbfc04ee4: /* 0xbfc04ee4 */
	lui	$v0, HI_UNSIGNED(0xbfd00503)
	ori	$v0, $v0, LO_UNSIGNED(0xbfd00503)
	lbu	$t6, 0($v0)
	lui	$t2, HI(0xbfd00003)
	dsll32	$t6, $t6, 0
	dsra32	$t6, $t6, 0
	andi	$t8, $t6, 1
	bnezl	$t8, L_0xbfc04f24
	 lbu	$t3, LO(0xbfd00003)($t2)
	lbu	$t9, 0($v0)
L_0xbfc04f0c: /* 0xbfc04f0c */
	dsll32	$t9, $t9, 0
	dsra32	$t9, $t9, 0
	andi	$t1, $t9, 1
	beqzl	$t1, L_0xbfc04f0c
	 lbu	$t9, 0($v0)
	lbu	$t3, LO(0xbfd00003)($t2)
L_0xbfc04f24: /* 0xbfc04f24 */
	dsll32	$t3, $t3, 0
	dsra32	$t3, $t3, 0
	andi	$t5, $t3, 0x7f
	dsll32	$t5, $t5, 0
	dsrl32	$t5, $t5, 0
	jr	$ra
	 andi	$v0, $t5, 0xff

/* Function putchar [0xbfc04f40 - 0xbfc04fc0)
 */
putchar: /* 0xbfc04f40 */
	addiu	$sp, $sp, -0x18
	lui	$v0, HI_UNSIGNED(0xbfd00503)
	ori	$v0, $v0, LO_UNSIGNED(0xbfd00503)
	sw	$ra, 0x14($sp)
	sw	$a0, 0x18($sp)
	lbu	$t6, 0($v0)
	andi	$a1, $a0, 0xff
	dsll32	$a1, $a1, 0
	dsll32	$t6, $t6, 0
	dsra32	$t6, $t6, 0
	andi	$t8, $t6, 0x20
	bnez	$t8, L_0xbfc04f8c
	 dsra32	$a1, $a1, 0
	lbu	$t9, 0($v0)
L_0xbfc04f78: /* 0xbfc04f78 */
	dsll32	$t9, $t9, 0
	dsra32	$t9, $t9, 0
	andi	$t1, $t9, 0x20
	beqzl	$t1, L_0xbfc04f78
	 lbu	$t9, 0($v0)
L_0xbfc04f8c: /* 0xbfc04f8c */
	dsll32	$t2, $a1, 0
	dsrl32	$t2, $t2, 0
	lui	$t5, 0xbfd0
	or	$t4, $t2, $t5
	dsll32	$t4, $t4, 0
	dsra32	$t4, $t4, 0
	sb	$a1, 0($t4)
	jal	delay
	 li	$a0, 0x20
	lw	$ra, 0x14($sp)
	addiu	$sp, $sp, 0x18
	jr	$ra
	 nop

/* Function F_0xbfc04fc0 [0xbfc04fc0 - 0xbfc04fe0)
 */
F_0xbfc04fc0: /* 0xbfc04fc0 */
	addiu	$a1, $a1, 8
L_0xbfc04fc4: /* 0xbfc04fc4 */
	ld	$a3, 0($a0)
	addiu	$a0, $a0, 8
	sd	$a3, 0($a2)
	bne	$a0, $a1, L_0xbfc04fc4
	 addiu	$a2, $a2, 8
	jr	$ra
.Llocal2: /* 0xbfc04fdc */
	 nop

/* Function get_CP0_ENTRYHI [0xbfc04fe0 - 0xbfc04ff0)
 */
get_CP0_ENTRYHI: /* 0xbfc04fe0 */
	mfc0	$v0, $CP0_ENTRYHI
	nop
	jr	$ra
	 nop

/* Function get_CP0_ENTRYLO1 [0xbfc04ff0 - 0xbfc05000)
 */
get_CP0_ENTRYLO1: /* 0xbfc04ff0 */
	mfc0	$v0, $CP0_ENTRYLO1
	nop
	jr	$ra
	 nop

/* Function get_CP0_ENTRYLO0 [0xbfc05000 - 0xbfc05010)
 */
get_CP0_ENTRYLO0: /* 0xbfc05000 */
	mfc0	$v0, $CP0_ENTRYLO0
	nop
	jr	$ra
	 nop

/* Function tlb_read_index [0xbfc05010 - 0xbfc05028)
 */
tlb_read_index: /* 0xbfc05010 */
	mtc0	$a0, $CP0_INDEX
	nop
	nop
	nop
	jr	$ra
	 tlbr

/* Function set_CP0_PAGEMASK [0xbfc05028 - 0xbfc05038)
 */
set_CP0_PAGEMASK: /* 0xbfc05028 */
	mtc0	$a0, $CP0_PAGEMASK
	nop
	jr	$ra
	 nop

/* Function set_CP0_ENTRY [0xbfc05038 - 0xbfc05054)
 */
set_CP0_ENTRY: /* 0xbfc05038 */
	mtc0	$a0, $CP0_ENTRYHI
	mtc0	$a1, $CP0_ENTRYLO0
	mtc0	$a2, $CP0_ENTRYLO1
	nop
	nop
	jr	$ra
	 nop

/* Function tlb_write_index [0xbfc05054 - 0xbfc05070)
 */
tlb_write_index: /* 0xbfc05054 */
	mtc0	$a0, $CP0_INDEX
	nop
	nop
	nop
	nop
	jr	$ra
	 tlbwi

/* Function writeback_dword [0xbfc05070 - 0xbfc05098)
 *
 * Copies 8 bytes from KSEG0 (cached) to KSEG1 (uncached)
 * $a0 = physical address
 */
writeback_dword: /* 0xbfc05070 */
	lui	$a1, HI(KSEG0)
	or	$a1, $a1, $a0
	ld	$a2, 0($a1)
	nop
	lui	$a1, HI(KSEG1)
	or	$a1, $a1, $a0
	sd	$a2, 0($a1)
	jr	$ra
	 nop
	nop		# unreachable

/* Function F_0xbfc05098 [0xbfc05098 - 0xbfc050ac)
 */
F_0xbfc05098: /* 0xbfc05098 */
	cache	(CACHE_TYPE_L1I|INDEX_WRITEBACK_INV), 0($a0)
	nop
	jr	$ra
	 nop
	nop		# unreachable

/* Function F_0xbfc050ac [0xbfc050ac - 0xbfc050bc)
 */
F_0xbfc050ac: /* 0xbfc050ac */
	cache	(CACHE_TYPE_L1D|HIT_WRITEBACK), 0($a0)
	nop
	jr	$ra
	 nop

/* Function F_0xbfc050bc [0xbfc050bc - 0xbfc05110)
 */
F_0xbfc050bc: /* 0xbfc050bc */
	addiu	$sp, $sp, -0x20
	sd	$at, 8($sp)
	sd	$t0, 0x10($sp)
	mfc0	$at, $CP0_PRID
	li	$t0, PRID_IMP_R5000
	andi	$at, $at, PRID_IMP_MASK
	srl	$at, $at, PRID_IMP_SHIFT
	beq	$t0, $at, L_0xbfc05100
	 nop
	li	$t0, PRID_IMP_RM7000
	beq	$t0, $at, L_0xbfc05100
	 nop
	li	$t0, PRID_IMP_NEVADA
	beq	$t0, $at, L_0xbfc05100
	 nop
	cache	(CACHE_TYPE_L1D|HIT_WRITEBACK_INV), 0($a0)
	cache	(CACHE_TYPE_L2|HIT_WRITEBACK_INV), 0($a0)
L_0xbfc05100: /* 0xbfc05100 */
	ld	$at, 8($sp)
	ld	$t0, 0x10($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x20

/* Function F_0xbfc05110 [0xbfc05110 - 0xbfc0518c)
 */
F_0xbfc05110: /* 0xbfc05110 */
	addiu	$sp, $sp, -0x80
	sd	$ra, 8($sp)
	sd	$a0, 0x10($sp)
	sd	$a1, 0x18($sp)
	sd	$t0, 0x20($sp)
	sd	$t1, 0x28($sp)
	sd	$t2, 0x30($sp)
	sd	$t3, 0x38($sp)
	sd	$t4, 0x40($sp)
	sd	$t5, 0x48($sp)
	sd	$t6, 0x50($sp)
	sd	$s0, 0x58($sp)
	sd	$s1, 0x60($sp)
	sd	$s2, 0x68($sp)
	jal	F_0xbfc0518c
	 nop
	ld	$ra, 8($sp)
	ld	$a0, 0x10($sp)
	ld	$a1, 0x18($sp)
	ld	$t0, 0x20($sp)
	ld	$t1, 0x28($sp)
	ld	$t2, 0x30($sp)
	ld	$t3, 0x38($sp)
	ld	$t4, 0x40($sp)
	ld	$t5, 0x48($sp)
	ld	$t6, 0x50($sp)
	ld	$s0, 0x58($sp)
	ld	$s1, 0x60($sp)
	ld	$s2, 0x68($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x80

/* Function F_0xbfc0518c [0xbfc0518c - 0xbfc05348)
 */
F_0xbfc0518c: /* 0xbfc0518c */
	lui	$t0, HI(KSEG1)
	addu	$t0, $t0, $a0
	addu	$t1, $t0, $a1
	lui	$t2, HI(BASE_CRIME + CRIME_ID_OFFSET)
	addiu	$t2, $t2, LO(BASE_CRIME + CRIME_ID_OFFSET)
	li	$a0, 0x20
	addi	$a0, $a0, -8
L_0xbfc051a8: /* 0xbfc051a8 */
	bltz	$a0, L_0xbfc051dc
	 addu	$a1, $t0, $a0
	li	$t3, 1
L_0xbfc051b4: /* 0xbfc051b4 */
	sd	$t3, 0($a1)
	ld	$t5, LO(BASE_CRIME + CRIME_ID_OFFSET)($t2)
	ld	$t4, 0($a1)
	li	$t5, 1
	bne	$t3, $t4, L_0xbfc05318
	 dsll	$t3, $t3, 1
	bnez	$t3, L_0xbfc051b4
	 nop
	b	L_0xbfc051a8
	 addi	$a0, $a0, -8
L_0xbfc051dc: /* 0xbfc051dc */
	li	$a0, 0x20
	addi	$a0, $a0, -8
L_0xbfc051e4: /* 0xbfc051e4 */
	bltz	$a0, L_0xbfc05220
	 addu	$a1, $t0, $a0
	li	$t3, 1
	not	$t3, $t3
L_0xbfc051f4: /* 0xbfc051f4 */
	sd	$t3, 0($a1)
	ld	$t5, LO(BASE_CRIME + CRIME_ID_OFFSET)($t2)
	ld	$t4, 0($a1)
	li	$t5, 2
	bne	$t3, $t4, L_0xbfc05318
	 not	$t6, $t3
	dsll	$t6, $t6, 1
	bnez	$t6, L_0xbfc051f4
	 not	$t3, $t6
	b	L_0xbfc051e4
	 addi	$a0, $a0, -8
L_0xbfc05220: /* 0xbfc05220 */
	nop
	lui	$t3, HI_UNSIGNED(0x5a5a5a5a)
	ori	$t3, $t3, LO_UNSIGNED(0x5a5a5a5a)
	move	$a0, $t0
	addiu	$a1, $a0, 4
L_0xbfc05234: /* 0xbfc05234 */
	sw	$t3, LO(KSEG1)($a0)
	not	$t3, $t3
	sw	$t3, 0($a1)
	lwu	$t4, LO(KSEG1)($a0)
	not	$t3, $t3
	bne	$t3, $t4, L_0xbfc05318
	 li	$t5, 3
	addiu	$a1, $a1, 4
	beq	$a1, $t1, L_0xbfc0526c
	 addiu	$a0, $a0, LO(0xa0000004)
	bne	$a0, $t1, L_0xbfc05234
	 nop
	b	L_0xbfc05274
	 nop
L_0xbfc0526c: /* 0xbfc0526c */
	b	L_0xbfc05234
	 move	$a1, $t0
L_0xbfc05274: /* 0xbfc05274 */
	move	$a0, $t0
	not	$t3, $a0
L_0xbfc0527c: /* 0xbfc0527c */
	dsll32	$t3, $t3, 0
	dsrl32	$t3, $t3, 0
	dsll32	$t5, $a0, 0
	or	$t3, $t3, $t5
	sd	$t3, LO(KSEG1)($a0)
	addiu	$a0, $a0, LO(0xa0000008)
	bne	$a0, $t1, L_0xbfc0527c
	 not	$t3, $a0
	move	$a0, $t0
	not	$t3, $a0
L_0xbfc052a4: /* 0xbfc052a4 */
	dsll32	$t3, $t3, 0
	dsrl32	$t3, $t3, 0
	dsll32	$t5, $a0, 0
	ld	$t4, LO(KSEG1)($a0)
	or	$t3, $t3, $t5
	bne	$t3, $t4, L_0xbfc05318
	 li	$t5, 4
	not	$t4, $t4
	sd	$t4, LO(KSEG1)($a0)
	addiu	$a0, $a0, LO(0xa0000008)
	bne	$a0, $t1, L_0xbfc052a4
	 not	$t3, $a0
	move	$a0, $t0
	not	$t3, $a0
L_0xbfc052dc: /* 0xbfc052dc */
	dsll32	$t3, $t3, 0
	dsrl32	$t3, $t3, 0
	dsll32	$t5, $a0, 0
	ld	$t4, LO(KSEG1)($a0)
	or	$t3, $t3, $t5
	not	$t3, $t3
	bne	$t3, $t4, L_0xbfc05318
	 li	$t5, 5
	addiu	$a0, $a0, LO(0xa0000008)
	bne	$a0, $t1, L_0xbfc052dc
	 not	$t3, $a0
	lui	$t5, HI(KSEG1)
	subu	$a0, $t0, $t5
	jr	$ra
	 subu	$a1, $t0, $t1
L_0xbfc05318: /* 0xbfc05318 */
	lui	$s0, HI(BASE_ISA + ISA_MISC_CONTROL)
	addiu	$s0, $s0, LO(BASE_ISA + ISA_MISC_CONTROL)
	ld	$s1, 0($s0)		# ISA_MISC_CONTROL
	ori	$s1, $s1, ISA_RED_LED|ISA_GREEN_LED
L_0xbfc05328: /* 0xbfc05328 */
	sd	$s1, 0($s0)		# ISA_MISC_CONTROL
	lui	$a0, HI_UNSIGNED(500000)
	ori	$a0, $a0, LO_UNSIGNED(500000)
	jal	usleep
	 nop
	ld	$s1, 0($s0)		# ISA_MISC_CONTROL
	b	L_0xbfc05328
	 xori	$s1, $s1, ISA_RED_LED|ISA_GREEN_LED

/* Function F_0xbfc05348 [0xbfc05348 - 0xbfc053a0)
 */
F_0xbfc05348: /* 0xbfc05348 */
	b	L_0xbfc05318
	 nop
data_0xbfc05350: /* 0xbfc05350 */
	string 4, "looping %d"
data_0xbfc0535c: /* 0xbfc0535c */
	string 4, "post1 memtst error\n expected=0x%0lx, obsev=0x%0lx, addr=0x%0lx"
	.int	0x00000000

/* Function F_0xbfc053a0 [0xbfc053a0 - 0xbfc05464)
 */
F_0xbfc053a0: /* 0xbfc053a0 */
	addiu	$sp, $sp, -0x30
	sw	$ra, 0x1c($sp)
	sw	$s0, 0x18($sp)
	lui	$t6, HI(BASE_ISA + ISA_RING_BASE_AND_RESET)
	jal	F_0xbfc05464
	 sd	$zero, LO(BASE_ISA + ISA_RING_BASE_AND_RESET)($t6)
	beqz	$v0, L_0xbfc053e0
	 lui	$s0, HI_UNSIGNED(BASE_ISA | ISA_MISC_CONTROL)
	ori	$s0, $s0, LO_UNSIGNED(BASE_ISA | ISA_MISC_CONTROL)
	li	$t7, ISA_GREEN_LED
	lui	$a0, HI_UNSIGNED(1000000)
	sd	$t7, 0($s0)
	jal	usleep
	 ori	$a0, $a0, LO_UNSIGNED(1000000)
	li	$t8, ISA_RED_LED|ISA_GREEN_LED
	sd	$t8, 0($s0)
L_0xbfc053e0: /* 0xbfc053e0 */
	lui	$s0, HI_UNSIGNED(BASE_ISA | ISA_MISC_CONTROL)
	ori	$s0, $s0, LO_UNSIGNED(BASE_ISA | ISA_MISC_CONTROL)
	jal	F_0xbfc05574
	 li	$a0, 1
	beqz	$v0, L_0xbfc05410
	 li	$t9, ISA_GREEN_LED
	lui	$a0, HI_UNSIGNED(1000000)
	sd	$t9, 0($s0)
	jal	usleep
	 ori	$a0, $a0, LO_UNSIGNED(1000000)
	li	$t0, ISA_RED_LED|ISA_GREEN_LED
	sd	$t0, 0($s0)
L_0xbfc05410: /* 0xbfc05410 */
	jal	F_0xbfc05574
	 li	$a0, 2
	beqz	$v0, L_0xbfc05450
	 sw	$v0, 0x20($sp)
	li	$t1, ISA_GREEN_LED
	lui	$a0, HI_UNSIGNED(1000000)
	sd	$t1, 0($s0)
	jal	usleep
	 ori	$a0, $a0, LO_UNSIGNED(1000000)
	li	$t2, ISA_GREEN_LED
	lui	$a0, HI_UNSIGNED(1000000)
	sd	$t2, 0($s0)
	jal	usleep
	 ori	$a0, $a0, LO_UNSIGNED(1000000)
	li	$t3, ISA_RED_LED|ISA_GREEN_LED
	sd	$t3, 0($s0)
L_0xbfc05450: /* 0xbfc05450 */
	lw	$ra, 0x1c($sp)
	lw	$v0, 0x20($sp)
	lw	$s0, 0x18($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x30

/* Function F_0xbfc05464 [0xbfc05464 - 0xbfc05574)
 */
F_0xbfc05464: /* 0xbfc05464 */
	lui	$a0, HI(BASE_UART_1 + UART_SCR)
	li	$a1, 0x55
	lui	$a2, HI_UNSIGNED(BASE_UART_2)
	ori	$a2, $a2, LO_UNSIGNED(BASE_UART_2)
	sb	$a1, LO(BASE_UART_1 + UART_SCR)($a0)
	lbu	$zero, LO(BASE_UART_1 + UART_SCR)($a2)
	lbu	$v0, LO(BASE_UART_1 + UART_SCR)($a0)
	li	$a3, 0x55
	move	$v1, $zero
	dsll32	$v0, $v0, 0
	dsra32	$v0, $v0, 0
	beq	$a3, $v0, L_0xbfc0549c
	 li	$t0, 0xaa
	li	$v1, 1
L_0xbfc0549c: /* 0xbfc0549c */
	sb	$t0, LO(BASE_UART_1 + UART_SCR)($a0)
	lbu	$zero, LO(BASE_UART_1 + UART_SCR)($a2)
	lbu	$v0, LO(BASE_UART_1 + UART_SCR)($a0)
	li	$t1, 0xaa
	li	$t2, 0xf0
	dsll32	$v0, $v0, 0
	dsra32	$v0, $v0, 0
	beq	$t1, $v0, L_0xbfc054c4
	 li	$t8, 0xf
	ori	$v1, $v1, 2
L_0xbfc054c4: /* 0xbfc054c4 */
	sb	$t2, LO(BASE_UART_1 + UART_SCR)($a0)
	lbu	$zero, LO(BASE_UART_1 + UART_SCR)($a2)
	lbu	$zero, LO(BASE_UART_1 + UART_SCR)($a0)
	sb	$t8, LO(BASE_UART_1 + UART_SCR)($a0)
	lbu	$zero, LO(BASE_UART_1 + UART_SCR)($a2)
	lbu	$zero, LO(BASE_UART_1 + UART_SCR)($a0)
	lbu	$zero, LO(BASE_UART_1 + UART_DATA)($a0)
	lbu	$zero, LO(BASE_UART_1 + UART_IER)($a0)
	lbu	$zero, LO(BASE_UART_1 + UART_IIR)($a0)
	lbu	$zero, LO(BASE_UART_1 + UART_LCR)($a0)
	lbu	$zero, LO(BASE_UART_1 + UART_MCR)($a0)
	lbu	$zero, LO(BASE_UART_1 + UART_LSR)($a0)
	lbu	$zero, LO(BASE_UART_1 + UART_MSR)($a0)
	sb	$a1, LO(BASE_UART_1 + UART_SCR)($a2)
	lbu	$zero, LO(BASE_UART_1 + UART_SCR)($a0)
	lbu	$v0, LO(BASE_UART_1 + UART_SCR)($a2)
	dsll32	$v0, $v0, 0
	dsra32	$v0, $v0, 0
	beql	$a3, $v0, L_0xbfc0551c
	 sb	$t0, LO(BASE_UART_1 + UART_SCR)($a2)
	ori	$v1, $v1, 4
	sb	$t0, LO(BASE_UART_1 + UART_SCR)($a2)
L_0xbfc0551c: /* 0xbfc0551c */
	lbu	$zero, LO(BASE_UART_1 + UART_SCR)($a0)
	lbu	$v0, LO(BASE_UART_1 + UART_SCR)($a2)
	dsll32	$v0, $v0, 0
	dsra32	$v0, $v0, 0
	beql	$t1, $v0, L_0xbfc0553c
	 sb	$t2, LO(BASE_UART_1 + UART_SCR)($a2)
	ori	$v1, $v1, 8
	sb	$t2, LO(BASE_UART_1 + UART_SCR)($a2)
L_0xbfc0553c: /* 0xbfc0553c */
	lbu	$zero, LO(BASE_UART_1 + UART_SCR)($a0)
	lbu	$zero, LO(BASE_UART_1 + UART_SCR)($a2)
	sb	$t2, LO(BASE_UART_1 + UART_SCR)($a2)
	lbu	$zero, LO(BASE_UART_1 + UART_SCR)($a0)
	lbu	$zero, LO(BASE_UART_1 + UART_SCR)($a2)
	lbu	$zero, LO(BASE_UART_1 + UART_DATA)($a2)
	lbu	$zero, LO(BASE_UART_1 + UART_IER)($a2)
	lbu	$zero, LO(BASE_UART_1 + UART_IIR)($a2)
	lbu	$zero, LO(BASE_UART_1 + UART_LCR)($a2)
	lbu	$zero, LO(BASE_UART_1 + UART_MCR)($a2)
	lbu	$zero, LO(BASE_UART_1 + UART_LSR)($a2)
	lbu	$zero, LO(BASE_UART_1 + UART_MSR)($a2)
	jr	$ra
	 move	$v0, $v1

/* Function F_0xbfc05574 [0xbfc05574 - 0xbfc058b0)
 */
F_0xbfc05574: /* 0xbfc05574 */
	addiu	$sp, $sp, -0x148
	li	$at, 1
	sw	$ra, 0x24($sp)
	sw	$s3, 0x20($sp)
	sw	$s2, 0x1c($sp)
	sw	$s1, 0x18($sp)
	bne	$a0, $at, L_0xbfc0559c
	 sw	$s0, 0x14($sp)
	b	L_0xbfc055b8
	 lui	$s2, HI(BASE_UART_1 + UART_LCR)
L_0xbfc0559c: /* 0xbfc0559c */
	li	$at, 2
	bne	$a0, $at, L_0xbfc055b0
	 lui	$s2, HI_UNSIGNED(BASE_UART_2)
	b	L_0xbfc055b8
	 ori	$s2, $s2, LO_UNSIGNED(BASE_UART_2)
L_0xbfc055b0: /* 0xbfc055b0 */
	b	L_0xbfc05894
	 li	$v0, 0x200
L_0xbfc055b8: /* 0xbfc055b8 */
	li	$t6, 0x80
	sb	$t6, LO(BASE_UART_1 + UART_LCR)($s2)
	li	$t7, 0x18
	sb	$t7, LO(BASE_UART_1 + UART_SCR)($s2)
	sb	$zero, LO(BASE_UART_1 + UART_IER)($s2)
	li	$t8, 0xc
	sb	$t8, LO(BASE_UART_1 + UART_DATA)($s2)
	li	$t9, 3
	sb	$t9, LO(BASE_UART_1 + UART_LCR)($s2)
	li	$t0, 1
	sb	$t0, LO(BASE_UART_1 + UART_IIR)($s2)
	sb	$zero, LO(BASE_UART_1 + UART_IER)($s2)
	li	$t1, 0x13
	sb	$t1, LO(BASE_UART_1 + UART_MCR)($s2)
	lbu	$t2, LO(BASE_UART_1 + UART_LSR)($s2)
	move	$s1, $zero
	addiu	$t5, $sp, 0x44
	dsll32	$t2, $t2, 0
	dsra32	$t2, $t2, 0
	andi	$t4, $t2, 1
	beqz	$t4, L_0xbfc05644
	 move	$s3, $zero
	addu	$s0, $s1, $t5
	addiu	$v0, $sp, 0x64
	lbu	$t6, LO(BASE_UART_1 + UART_DATA)($s2)
L_0xbfc0561c: /* 0xbfc0561c */
	addiu	$s0, $s0, 1
	sb	$t6, -1($s0)
	lbu	$t7, LO(BASE_UART_1 + UART_LSR)($s2)
	dsll32	$t7, $t7, 0
	dsra32	$t7, $t7, 0
	andi	$t9, $t7, 1
	beqzl	$t9, L_0xbfc05648
	 move	$s1, $zero
	bnel	$s0, $v0, L_0xbfc0561c
	 lbu	$t6, LO(BASE_UART_1 + UART_DATA)($s2)
L_0xbfc05644: /* 0xbfc05644 */
	move	$s1, $zero
L_0xbfc05648: /* 0xbfc05648 */
	lbu	$t0, LO(BASE_UART_1 + UART_LSR)($s2)
L_0xbfc0564c: /* 0xbfc0564c */
	li	$s0, 10000
	dsll32	$t0, $t0, 0
	dsra32	$t0, $t0, 0
	andi	$t2, $t0, 0x20
	bnezl	$t2, L_0xbfc05698
	 sb	$s3, LO(BASE_UART_1 + UART_DATA)($s2)
L_0xbfc05664: /* 0xbfc05664 */
	jal	usleep
	 li	$a0, 0x64
	lbu	$t4, LO(BASE_UART_1 + UART_LSR)($s2)
	li	$t3, -1
	addu	$s0, $s0, $t3
	dsll32	$t4, $t4, 0
	dsra32	$t4, $t4, 0
	andi	$t6, $t4, 0x20
	bnezl	$t6, L_0xbfc05698
	 sb	$s3, LO(BASE_UART_1 + UART_DATA)($s2)
	bnez	$s0, L_0xbfc05664
	 nop
	sb	$s3, LO(BASE_UART_1 + UART_DATA)($s2)
L_0xbfc05698: /* 0xbfc05698 */
	lbu	$t7, LO(BASE_UART_1 + UART_LSR)($s2)
	addiu	$s3, $s3, 1
	andi	$s3, $s3, 0xff
	dsll32	$t7, $t7, 0
	dsra32	$t7, $t7, 0
	andi	$t9, $t7, 1
	beqz	$t9, L_0xbfc056fc
	 dsll32	$s3, $s3, 0
	slti	$at, $s1, 0x100
	beqz	$at, L_0xbfc056fc
	 addiu	$t0, $sp, 0x44
	addu	$s0, $s1, $t0
	lbu	$t1, LO(BASE_UART_1 + UART_DATA)($s2)
L_0xbfc056cc: /* 0xbfc056cc */
	addiu	$s1, $s1, 1
	slti	$at, $s1, 0x100
	sb	$t1, 0($s0)
	lbu	$t2, LO(BASE_UART_1 + UART_LSR)($s2)
	addiu	$s0, $s0, 1
	dsll32	$t2, $t2, 0
	dsra32	$t2, $t2, 0
	andi	$t4, $t2, 1
	beqzl	$t4, L_0xbfc05700
	 dsra32	$s3, $s3, 0
	bnezl	$at, L_0xbfc056cc
	 lbu	$t1, LO(BASE_UART_1 + UART_DATA)($s2)
L_0xbfc056fc: /* 0xbfc056fc */
	dsra32	$s3, $s3, 0
L_0xbfc05700: /* 0xbfc05700 */
	slti	$at, $s3, 0xff
	bnezl	$at, L_0xbfc0564c
	 lbu	$t0, LO(BASE_UART_1 + UART_LSR)($s2)
	jal	usleep
	 li	$a0, 10000
	lbu	$t6, LO(BASE_UART_1 + UART_LSR)($s2)
	slti	$at, $s1, 0x100
	dsll32	$t6, $t6, 0
	dsra32	$t6, $t6, 0
	andi	$t8, $t6, 1
	beqzl	$t8, L_0xbfc0577c
	 lb	$t4, 0x44($sp)
	beqz	$at, L_0xbfc05778
	 addiu	$t9, $sp, 0x44
	addu	$s0, $s1, $t9
	addiu	$s1, $sp, 0x144
	lbu	$t0, LO(BASE_UART_1 + UART_DATA)($s2)
L_0xbfc05744: /* 0xbfc05744 */
	addiu	$s0, $s0, 1
	li	$a0, 10000
	jal	usleep
	 sb	$t0, -1($s0)
	lbu	$t1, LO(BASE_UART_1 + UART_LSR)($s2)
	sltu	$at, $s0, $s1
	dsll32	$t1, $t1, 0
	dsra32	$t1, $t1, 0
	andi	$t3, $t1, 1
	beqzl	$t3, L_0xbfc0577c
	 lb	$t4, 0x44($sp)
	bnezl	$at, L_0xbfc05744
	 lbu	$t0, LO(BASE_UART_1 + UART_DATA)($s2)
L_0xbfc05778: /* 0xbfc05778 */
	lb	$t4, 0x44($sp)
L_0xbfc0577c: /* 0xbfc0577c */
	lb	$t6, 0x45($sp)
	li	$at, 1
	beqz	$t4, L_0xbfc05794
	 nop
	b	L_0xbfc05894
	 li	$v0, 0x100
L_0xbfc05794: /* 0xbfc05794 */
	beq	$t6, $at, L_0xbfc057a4
	 lb	$t8, 0x46($sp)
	b	L_0xbfc05894
	 li	$v0, 0x101
L_0xbfc057a4: /* 0xbfc057a4 */
	li	$at, 2
	beq	$t8, $at, L_0xbfc057b8
	 li	$s1, 3
	b	L_0xbfc05894
	 li	$v0, CTYPE_TOLOWER
L_0xbfc057b8: /* 0xbfc057b8 */
	addiu	$s0, $sp, 0x47
	li	$v0, 0xff
L_0xbfc057c0: /* 0xbfc057c0 */
	lbu	$t3, 0($s0)
	dsll32	$s1, $s1, 0
	dsrl32	$s1, $s1, 0
	andi	$t1, $s1, 0xff
	dsll32	$t1, $t1, 0
	dsll32	$t3, $t3, 0
	dsra32	$t3, $t3, 0
	dsra32	$t1, $t1, 0
	beq	$t1, $t3, L_0xbfc057f0
	 addiu	$v1, $s1, 1
	b	L_0xbfc05894
	 ori	$v0, $s1, 0x100
L_0xbfc057f0: /* 0xbfc057f0 */
	lbu	$t8, 1($s0)
	dsll32	$v1, $v1, 0
	dsrl32	$v1, $v1, 0
	andi	$t6, $v1, 0xff
	dsll32	$t6, $t6, 0
	dsll32	$t8, $t8, 0
	dsra32	$t8, $t8, 0
	dsra32	$t6, $t6, 0
	beql	$t6, $t8, L_0xbfc05824
	 addiu	$v1, $s1, 2
	b	L_0xbfc05894
	 ori	$v0, $v1, 0x100
	addiu	$v1, $s1, 2		# unreachable
L_0xbfc05824: /* 0xbfc05824 */
	lbu	$t2, 2($s0)
	dsll32	$v1, $v1, 0
	dsrl32	$v1, $v1, 0
	andi	$t1, $v1, 0xff
	dsll32	$t1, $t1, 0
	dsll32	$t2, $t2, 0
	dsra32	$t2, $t2, 0
	dsra32	$t1, $t1, 0
	beql	$t1, $t2, L_0xbfc05858
	 addiu	$v1, $s1, 3
	b	L_0xbfc05894
	 ori	$v0, $v1, 0x100
	addiu	$v1, $s1, 3		# unreachable
L_0xbfc05858: /* 0xbfc05858 */
	lbu	$t7, 3($s0)
	dsll32	$v1, $v1, 0
	dsrl32	$v1, $v1, 0
	andi	$t6, $v1, 0xff
	dsll32	$t6, $t6, 0
	dsll32	$t7, $t7, 0
	dsra32	$t7, $t7, 0
	dsra32	$t6, $t6, 0
	beq	$t6, $t7, L_0xbfc05888
	 addiu	$s1, $s1, 4
	b	L_0xbfc05894
	 ori	$v0, $v1, 0x100
L_0xbfc05888: /* 0xbfc05888 */
	bne	$s1, $v0, L_0xbfc057c0
	 addiu	$s0, $s0, 4
	move	$v0, $zero
L_0xbfc05894: /* 0xbfc05894 */
	lw	$ra, 0x24($sp)
	lw	$s0, 0x14($sp)
	lw	$s1, 0x18($sp)
	lw	$s2, 0x1c($sp)
	lw	$s3, 0x20($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x148

/* Function usleep [0xbfc058b0 - 0xbfc05900)
 */
usleep: /* 0xbfc058b0 */
	lui	$v0, HI(BASE_UST)
	ld	$v1, LO(BASE_UST)($v0)
	ld	$t8, LO(BASE_UST)($v0)
	li	$at, -1
	and	$t6, $v1, $at
	daddu	$v1, $t6, $a0
	and	$t9, $t8, $at
	sltu	$at, $t9, $v1
	beqz	$at, L_0xbfc058f0
	 nop
	ld	$t0, LO(BASE_UST)($v0)
L_0xbfc058dc: /* 0xbfc058dc */
	li	$at, -1
	and	$t1, $t0, $at
	sltu	$at, $t1, $v1
	bnezl	$at, L_0xbfc058dc
	 ld	$t0, LO(BASE_UST)($v0)
L_0xbfc058f0: /* 0xbfc058f0 */
	jr	$ra
	 nop
	nop		# unreachable
	nop		# unreachable

/* Function F_0xbfc05900 [0xbfc05900 - 0xbfc05934)
 */
F_0xbfc05900: /* 0xbfc05900 */
	lui	$v0, HI(BASE_UST)
	ld	$v1, LO(BASE_UST)($v0)
	ld	$t7, LO(BASE_UST)($v0)
	daddu	$v1, $v1, $a0
	sltu	$at, $t7, $v1
	beqz	$at, L_0xbfc0592c
	 nop
	ld	$t8, LO(BASE_UST)($v0)
L_0xbfc05920: /* 0xbfc05920 */
	sltu	$at, $t8, $v1
	bnezl	$at, L_0xbfc05920
	 ld	$t8, LO(BASE_UST)($v0)
L_0xbfc0592c: /* 0xbfc0592c */
	jr	$ra
	 nop

/* Function print_bank_sizes [0xbfc05934 - 0xbfc05998)
 */
print_bank_sizes: /* 0xbfc05934 */
	addiu	$sp, $sp, -0x38
	sw	$s0, 0x30($sp)
	move	$s0, $a0
	sw	$ra, 0x34($sp)
	lw	$t6, 0xc($s0)
	lw	$a3, 8($s0)
	lw	$a2, 4($s0)
	lw	$a1, 0($s0)
	sw	$t6, 0x10($sp)
	lw	$t7, 0x10($s0)
	lui	$a0, %hi(data_0xbfc06b70)
	addiu	$a0, $a0, %lo(data_0xbfc06b70)
	sw	$t7, 0x14($sp)
	lw	$t8, 0x14($s0)
	sw	$t8, 0x18($sp)
	lw	$t9, 0x18($s0)
	sw	$t9, 0x1c($sp)
	lw	$t0, 0x1c($s0)
	jal	printf_disabled
	 sw	$t0, 0x20($sp)
	lw	$ra, 0x34($sp)
	lw	$s0, 0x30($sp)
	addiu	$sp, $sp, 0x38
	jr	$ra
	 nop

/* Function F_0xbfc05998 [0xbfc05998 - 0xbfc05a2c)
 */
F_0xbfc05998: /* 0xbfc05998 */
	addiu	$sp, $sp, -0x30
	sw	$ra, 0x2c($sp)
	sw	$s4, 0x28($sp)
	sw	$s3, 0x24($sp)
	sw	$s2, 0x20($sp)
	sw	$s1, 0x1c($sp)
	sw	$s0, 0x18($sp)
	jal	set_CP0_PAGEMASK
	 move	$a0, $zero
	move	$s2, $zero
	li	$s4, 4
	li	$s3, -3
L_0xbfc059c8: /* 0xbfc059c8 */
	jal	tlb_read_index
	 move	$a0, $s2
	jal	get_CP0_ENTRYLO0
	 nop
	jal	get_CP0_ENTRYLO1
	 move	$s0, $v0
	jal	get_CP0_ENTRYHI
	 move	$s1, $v0
	and	$a1, $s0, $s3
	and	$a2, $s1, $s3
	jal	set_CP0_ENTRY
	 move	$a0, $v0
	jal	tlb_write_index
	 move	$a0, $s2
	addiu	$s2, $s2, 1
	bne	$s2, $s4, L_0xbfc059c8
	 nop
	lw	$ra, 0x2c($sp)
	lw	$s0, 0x18($sp)
	lw	$s1, 0x1c($sp)
	lw	$s2, 0x20($sp)
	lw	$s3, 0x24($sp)
	lw	$s4, 0x28($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x30

/* Function F_0xbfc05a2c [0xbfc05a2c - 0xbfc05ad4)
 */
F_0xbfc05a2c: /* 0xbfc05a2c */
	addiu	$sp, $sp, -0x38
	sw	$s3, 0x24($sp)
	move	$s3, $a0
	sw	$ra, 0x34($sp)
	lui	$a0, HI_UNSIGNED(0x01ffe000)
	sw	$s6, 0x30($sp)
	sw	$s5, 0x2c($sp)
	sw	$s4, 0x28($sp)
	sw	$s2, 0x20($sp)
	sw	$s1, 0x1c($sp)
	sw	$s0, 0x18($sp)
	jal	set_CP0_PAGEMASK
	 ori	$a0, $a0, LO_UNSIGNED(0x01ffe000)
	lui	$at, 0x4000
	move	$s1, $s3
	addu	$s2, $s3, $at
	lui	$s3, 0x200
	move	$s0, $zero
	li	$s6, 4
	lui	$s5, 4
	li	$s4, -0x2000
L_0xbfc05a80: /* 0xbfc05a80 */
	srl	$a1, $s2, 6
	ori	$a1, $a1, 0x17
	addu	$a2, $a1, $s5
	jal	set_CP0_ENTRY
	 and	$a0, $s1, $s4
	jal	tlb_write_index
	 move	$a0, $s0
	addiu	$s0, $s0, 1
	addu	$s1, $s1, $s3
	bne	$s0, $s6, L_0xbfc05a80
	 addu	$s2, $s2, $s3
	lw	$ra, 0x34($sp)
	lw	$s0, 0x18($sp)
	lw	$s1, 0x1c($sp)
	lw	$s2, 0x20($sp)
	lw	$s3, 0x24($sp)
	lw	$s4, 0x28($sp)
	lw	$s5, 0x2c($sp)
	lw	$s6, 0x30($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x38

/* Function F_0xbfc05ad4 [0xbfc05ad4 - 0xbfc060a0)
 */
F_0xbfc05ad4: /* 0xbfc05ad4 */
	addiu	$sp, $sp, -0xa0
	sw	$ra, 0x3c($sp)
	lui	$a0, %hi(data_0xbfc06c30)
	sw	$fp, 0x38($sp)
	sw	$s7, 0x34($sp)
	sw	$s6, 0x30($sp)
	sw	$s5, 0x2c($sp)
	sw	$s4, 0x28($sp)
	sw	$s3, 0x24($sp)
	sw	$s2, 0x20($sp)
	sw	$s1, 0x1c($sp)
	sw	$s0, 0x18($sp)
	jal	printf_disabled
	 addiu	$a0, $a0, %lo(data_0xbfc06c30)
	addiu	$s2, $sp, 0x70
	addiu	$v0, $sp, 0x90
L_0xbfc05b14: /* 0xbfc05b14 */
	addiu	$s2, $s2, 4
	sltu	$at, $s2, $v0
	bnez	$at, L_0xbfc05b14
	 sw	$zero, -4($s2)
	jal	print_bank_sizes
	 addiu	$a0, $sp, 0x70
	lui	$v1, HI_UNSIGNED(BASE_CRIME | CRIME_CONTROL_OFFSET)
	ori	$v1, $v1, LO_UNSIGNED(BASE_CRIME | CRIME_CONTROL_OFFSET)
	ld	$v0, 0($v1)
	li	$t7, -2
	lui	$a0, %hi(data_0xbfc06c54)
	and	$t6, $v0, $t7
	sd	$t6, 0($v1)
	jal	printf_disabled
	 addiu	$a0, $a0, %lo(data_0xbfc06c54)
	lui	$s6, %hi(data_0xbfc06c98)
	lui	$s4, %hi(data_0xbfc06c70)
	lui	$s0, HI_UNSIGNED(BASE_CRIME | CRIME_BANK_0_CTRL)
	move	$s3, $zero
	move	$s1, $zero
	ori	$s0, $s0, LO_UNSIGNED(BASE_CRIME | CRIME_BANK_0_CTRL)
	addiu	$s4, $s4, %lo(data_0xbfc06c70)
	addiu	$s6, $s6, %lo(data_0xbfc06c98)
	addiu	$s2, $sp, 0x70
	li	$fp, 0x80
	lui	$s7, 0x800
	li	$s5, 8
L_0xbfc05b80: /* 0xbfc05b80 */
	move	$a0, $s4
	jal	printf_disabled
	 move	$a1, $s1
	jal	F_0xbfc05a2c
	 move	$a0, $s3
	slti	$at, $s1, 8
	beqz	$at, L_0xbfc05c74
	 move	$v0, $s1
	subu	$a2, $s5, $s1
	andi	$a2, $a2, 3
	beqz	$a2, L_0xbfc05be4
	 addu	$a1, $a2, $s1
	srl	$t8, $s3, 0x19
	andi	$t9, $t8, 0x1f
	ori	$v1, $t9, 0x100
L_0xbfc05bbc: /* 0xbfc05bbc */
	sll	$t1, $v0, 3
	dsll32	$t1, $t1, 0
	dsrl32	$t1, $t1, 0
	addu	$t3, $s0, $t1
	dsll32	$t3, $t3, 0
	dsra32	$t3, $t3, 0
	addiu	$v0, $v0, 1
	bne	$a1, $v0, L_0xbfc05bbc
	 sd	$v1, 0($t3)
	beq	$v0, $s5, L_0xbfc05c74
L_0xbfc05be4: /* 0xbfc05be4 */
	 srl	$t5, $s3, 0x19
	andi	$t7, $t5, 0x1f
	ori	$v1, $t7, 0x100
L_0xbfc05bf0: /* 0xbfc05bf0 */
	addiu	$t2, $v0, 1
	sll	$t8, $v0, 3
	sll	$t3, $t2, 3
	dsll32	$t8, $t8, 0
	dsll32	$t3, $t3, 0
	dsrl32	$t8, $t8, 0
	dsrl32	$t3, $t3, 0
	addu	$t0, $s0, $t8
	addu	$t5, $s0, $t3
	addiu	$t6, $v0, 2
	dsll32	$t0, $t0, 0
	sll	$t8, $t6, 3
	addiu	$t2, $v0, 3
	dsra32	$t0, $t0, 0
	sll	$t3, $t2, 3
	dsll32	$t8, $t8, 0
	dsll32	$t5, $t5, 0
	sd	$v1, 0($t0)
	dsra32	$t5, $t5, 0
	dsrl32	$t8, $t8, 0
	dsll32	$t3, $t3, 0
	dsrl32	$t3, $t3, 0
	addu	$t0, $s0, $t8
	sd	$v1, 0($t5)
	addu	$t5, $s0, $t3
	dsll32	$t0, $t0, 0
	dsra32	$t0, $t0, 0
	dsll32	$t5, $t5, 0
	dsra32	$t5, $t5, 0
	sd	$v1, 0($t0)
	addiu	$v0, $v0, 4
	bne	$v0, $s5, L_0xbfc05bf0
	 sd	$v1, 0($t5)
L_0xbfc05c74: /* 0xbfc05c74 */
	move	$a0, $s6
	jal	printf_disabled
	 move	$a1, $s3
	lui	$t6, %hi(data_0xbfc06f48)
	ld	$t6, %lo(data_0xbfc06f48)($t6)
	dsll32	$s3, $s3, 0
	dsra32	$s3, $s3, 0
	lui	$at, HI_UNSIGNED(0x01fffff8)
	ori	$at, $at, LO_UNSIGNED(0x01fffff8)
	lui	$t9, %hi(data_0xbfc06f50)
	sd	$t6, 0($s3)
	ld	$t9, %lo(data_0xbfc06f50)($t9)
	addu	$t8, $s3, $at
	dsll32	$t8, $t8, 0
	dsra32	$a0, $t8, 0
	lui	$at, 0x200
	lui	$t1, %hi(data_0xbfc06f58)
	sd	$t9, 0($a0)
	ld	$t1, %lo(data_0xbfc06f58)($t1)
	addu	$t0, $s3, $at
	dsll32	$t0, $t0, 0
	dsra32	$a1, $t0, 0
	lui	$at, HI_UNSIGNED(0x07fffff8)
	ori	$at, $at, LO_UNSIGNED(0x07fffff8)
	lui	$t3, %hi(data_0xbfc06f60)
	sd	$t1, 0($a1)
	ld	$t3, %lo(data_0xbfc06f60)($t3)
	addu	$t2, $s3, $at
	dsll32	$t2, $t2, 0
	addiu	$t4, $s3, 0x10
	dsra32	$a2, $t2, 0
	dsll32	$t4, $t4, 0
	addiu	$t7, $s3, 0x18
	dsra32	$t4, $t4, 0
	dsll32	$t7, $t7, 0
	addiu	$t8, $s3, 0x20
	sd	$t3, 0($a2)
	sd	$zero, 0($t4)
	dsra32	$t7, $t7, 0
	dsll32	$t8, $t8, 0
	sd	$zero, 0($t7)
	dsra32	$t8, $t8, 0
	sd	$zero, 0($t8)
	lui	$t0, %hi(data_0xbfc06f68)
	ld	$t0, %lo(data_0xbfc06f68)($t0)
	ld	$v1, 0($s3)
	lui	$t2, %hi(data_0xbfc06f70)
	bne	$v1, $t0, L_0xbfc05db4
	 nop
	ld	$t1, 0($a0)
	ld	$t2, %lo(data_0xbfc06f70)($t2)
	lui	$t4, %hi(data_0xbfc06f78)
	bne	$t1, $t2, L_0xbfc05db4
	 nop
	ld	$t3, 0($a1)
	ld	$t4, %lo(data_0xbfc06f78)($t4)
	lui	$t7, %hi(data_0xbfc06f80)
	bne	$t3, $t4, L_0xbfc05db4
	 nop
	ld	$t5, 0($a2)
	ld	$t7, %lo(data_0xbfc06f80)($t7)
	sll	$t8, $s1, 3
	li	$t6, 0x20
	bne	$t5, $t7, L_0xbfc05db4
	 dsll32	$t8, $t8, 0
	dsrl32	$t8, $t8, 0
	addu	$t0, $t8, $s0
	dsll32	$t0, $t0, 0
	sw	$t6, 0($s2)
	dsra32	$t0, $t0, 0
	ld	$a2, 0($t0)
	lui	$a0, %hi(data_0xbfc06cbc)
	addiu	$a0, $a0, %lo(data_0xbfc06cbc)
	dsll32	$a3, $a2, 0
	dsra32	$a3, $a3, 0
	move	$a1, $s1
	jal	printf_disabled
	 dsra32	$a2, $a2, 0
	b	L_0xbfc05edc
	 nop
L_0xbfc05db4: /* 0xbfc05db4 */
	lui	$t2, %hi(data_0xbfc06f88)
	ld	$t2, %lo(data_0xbfc06f88)($t2)
	lui	$t4, %hi(data_0xbfc06f90)
	bne	$v1, $t2, L_0xbfc05e40
	 nop
	ld	$t3, 0($a0)
	ld	$t4, %lo(data_0xbfc06f90)($t4)
	lui	$t7, %hi(data_0xbfc06f98)
	bne	$t3, $t4, L_0xbfc05e40
	 nop
	ld	$t5, 0($a1)
	ld	$t7, %lo(data_0xbfc06f98)($t7)
	lui	$t8, %hi(data_0xbfc06fa0)
	bne	$t5, $t7, L_0xbfc05e40
	 nop
	ld	$t6, 0($a2)
	ld	$t8, %lo(data_0xbfc06fa0)($t8)
	sll	$t9, $s1, 3
	dsll32	$t9, $t9, 0
	bne	$t6, $t8, L_0xbfc05e40
	 lui	$a0, %hi(data_0xbfc06ce8)
	dsrl32	$t9, $t9, 0
	addu	$t1, $t9, $s0
	dsll32	$t1, $t1, 0
	sw	$fp, 0($s2)
	dsra32	$t1, $t1, 0
	ld	$a2, 0($t1)
	addiu	$a0, $a0, %lo(data_0xbfc06ce8)
	move	$a1, $s1
	dsll32	$a3, $a2, 0
	dsra32	$a3, $a3, 0
	jal	printf_disabled
	 dsra32	$a2, $a2, 0
	b	L_0xbfc05edc
	 nop
L_0xbfc05e40: /* 0xbfc05e40 */
	bnez	$s1, L_0xbfc05ea4
	 lui	$a0, %hi(data_0xbfc06d30)
	li	$t3, 8
	subu	$a2, $t3, $zero
	andi	$a2, $a2, 3
	lui	$a0, %hi(data_0xbfc06d14)
	addiu	$a0, $a0, %lo(data_0xbfc06d14)
	move	$a1, $a2
	move	$s1, $zero
	move	$s3, $zero
	jal	printf_disabled
	 move	$v0, $zero
	lui	$s0, HI_UNSIGNED(BASE_ISA | ISA_MISC_CONTROL)
	ori	$s0, $s0, LO_UNSIGNED(BASE_ISA | ISA_MISC_CONTROL)
	ld	$t4, 0($s0)		# ISA_MISC_CONTROL
	lui	$s1, HI_UNSIGNED(500000)
	ori	$s1, $s1, LO_UNSIGNED(500000)
	ori	$t5, $t4, ISA_RED_LED|ISA_GREEN_LED
	sd	$t5, 0($s0)		# ISA_MISC_CONTROL
L_0xbfc05e8c: /* 0xbfc05e8c */
	jal	usleep
	 move	$a0, $s1
	ld	$t7, 0($s0)		# ISA_MISC_CONTROL
	xori	$t6, $t7, ISA_RED_LED|ISA_GREEN_LED
	b	L_0xbfc05e8c
	 sd	$t6, 0($s0)		# ISA_MISC_CONTROL
L_0xbfc05ea4: /* 0xbfc05ea4 */
	sll	$t8, $s1, 3
	dsll32	$t8, $t8, 0
	dsrl32	$t8, $t8, 0
	addu	$t0, $t8, $s0
	dsll32	$t0, $t0, 0
	sw	$zero, 0($s2)
	dsra32	$t0, $t0, 0
	ld	$a2, 0($t0)
	addiu	$a0, $a0, %lo(data_0xbfc06d30)
	move	$a1, $s1
	dsll32	$a3, $a2, 0
	dsra32	$a3, $a3, 0
	jal	printf_disabled
	 dsra32	$a2, $a2, 0
L_0xbfc05edc: /* 0xbfc05edc */
	jal	F_0xbfc05998
	 nop
	addiu	$s1, $s1, 1
	slti	$at, $s1, 8
	addiu	$s2, $s2, 4
	bnez	$at, L_0xbfc05b80
	 addu	$s3, $s3, $s7
	jal	print_bank_sizes
	 addiu	$a0, $sp, 0x70
	lui	$s4, %hi(data_0xbfc06d68)
	move	$s3, $zero
	addiu	$s4, $s4, %lo(data_0xbfc06d68)
	move	$s1, $zero
	addiu	$s2, $sp, 0x70
	li	$s6, -1
L_0xbfc05f18: /* 0xbfc05f18 */
	lw	$t2, 0($s2)
	srl	$t3, $s3, 0x19
	andi	$t4, $t3, 0x1f
	bne	$fp, $t2, L_0xbfc05f60
	 ori	$t5, $t4, 0x100
	sll	$t6, $s1, 3
	dsll32	$t6, $t6, 0
	dsrl32	$t6, $t6, 0
	addu	$t9, $s0, $t6
	dsll32	$t9, $t9, 0
	dsra32	$t9, $t9, 0
	sd	$t5, 0($t9)
	addu	$s3, $s3, $s7
	sw	$s6, 0($s2)
	move	$a2, $s3
	move	$a0, $s4
	jal	printf_disabled
	 move	$a1, $s1
L_0xbfc05f60: /* 0xbfc05f60 */
	addiu	$s1, $s1, 1
	slti	$at, $s1, 8
	bnez	$at, L_0xbfc05f18
	 addiu	$s2, $s2, 4
	jal	print_bank_sizes
	 addiu	$a0, $sp, 0x70
	lui	$s4, %hi(data_0xbfc06da0)
	addiu	$s4, $s4, %lo(data_0xbfc06da0)
	move	$s1, $zero
	addiu	$s2, $sp, 0x70
L_0xbfc05f88: /* 0xbfc05f88 */
	lw	$t1, 0($s2)
	li	$at, 0x20
	srl	$t2, $s3, 0x19
	bne	$t1, $at, L_0xbfc05fd4
	 andi	$t3, $t2, 0x1f
	sll	$t5, $s1, 3
	dsll32	$t5, $t5, 0
	dsrl32	$t5, $t5, 0
	addu	$t8, $s0, $t5
	dsll32	$t8, $t8, 0
	dsra32	$t8, $t8, 0
	lui	$at, 0x200
	sd	$t3, 0($t8)
	addu	$s3, $s3, $at
	sw	$s6, 0($s2)
	move	$a2, $s3
	move	$a0, $s4
	jal	printf_disabled
	 move	$a1, $s1
L_0xbfc05fd4: /* 0xbfc05fd4 */
	addiu	$s1, $s1, 1
	slti	$at, $s1, 8
	bnez	$at, L_0xbfc05f88
	 addiu	$s2, $s2, 4
	jal	print_bank_sizes
	 addiu	$a0, $sp, 0x70
	lui	$s4, HI_UNSIGNED(BASE_CRIME | CRIME_BANK_0_CTRL)
	ori	$s4, $s4, LO_UNSIGNED(BASE_CRIME | CRIME_BANK_0_CTRL)
	move	$s1, $zero
	addiu	$s2, $sp, 0x70
L_0xbfc05ffc: /* 0xbfc05ffc */
	lw	$t7, 0($s2)
	sll	$t1, $s1, 3
	dsll32	$t1, $t1, 0
	bnez	$t7, L_0xbfc06048
	 dsrl32	$t1, $t1, 0
	ld	$t0, 0($s4)
	addu	$t3, $s0, $t1
	dsll32	$t3, $t3, 0
	dsra32	$t3, $t3, 0
	sd	$t0, 0($t3)
	sw	$s6, 0($s2)
	ld	$a2, 0($s4)
	lui	$a0, %hi(data_0xbfc06dd8)
	addiu	$a0, $a0, %lo(data_0xbfc06dd8)
	dsll32	$a3, $a2, 0
	dsra32	$a3, $a3, 0
	move	$a1, $s1
	jal	printf_disabled
	 dsra32	$a2, $a2, 0
L_0xbfc06048: /* 0xbfc06048 */
	addiu	$s1, $s1, 1
	bne	$s1, $s5, L_0xbfc05ffc
	 addiu	$s2, $s2, 4
	lui	$a0, %hi(data_0xbfc06e0c)
	addiu	$a0, $a0, %lo(data_0xbfc06e0c)
	jal	printf_disabled
	 move	$a1, $s3
	jal	F_0xbfc05998
	 nop
	lw	$ra, 0x3c($sp)
	move	$v0, $s3
	lw	$s3, 0x24($sp)
	lw	$s0, 0x18($sp)
	lw	$s1, 0x1c($sp)
	lw	$s2, 0x20($sp)
	lw	$s4, 0x28($sp)
	lw	$s5, 0x2c($sp)
	lw	$s6, 0x30($sp)
	lw	$s7, 0x34($sp)
	lw	$fp, 0x38($sp)
	jr	$ra
	 addiu	$sp, $sp, 0xa0

/* Function F_0xbfc060a0 [0xbfc060a0 - 0xbfc060f8)
 */
F_0xbfc060a0: /* 0xbfc060a0 */
	addiu	$sp, $sp, -0x20
	sw	$ra, 0x1c($sp)
	sw	$s1, 0x18($sp)
	sw	$s0, 0x14($sp)
	move	$a0, $zero
	jal	F_0xbfc05110
	 li	$a1, 0x1000
	move	$s0, $zero
	li	$s1, 0x1008
	move	$a0, $s0
L_0xbfc060c8: /* 0xbfc060c8 */
	move	$a1, $zero
	jal	writeback_dword
	 move	$a2, $zero
	addiu	$s0, $s0, 8
	bnel	$s0, $s1, L_0xbfc060c8
	 move	$a0, $s0
	lw	$ra, 0x1c($sp)
	lw	$s0, 0x14($sp)
	lw	$s1, 0x18($sp)
	addiu	$sp, $sp, 0x20
	jr	$ra
	 li	$v0, 1

/* Function F_0xbfc060f8 [0xbfc060f8 - 0xbfc061f0)
 */
F_0xbfc060f8: /* 0xbfc060f8 */
	addiu	$sp, $sp, -0x60
	lui	$v1, HI_UNSIGNED(BASE_CRIME | CRIME_CONTROL_OFFSET)
	ori	$v1, $v1, LO_UNSIGNED(BASE_CRIME | CRIME_CONTROL_OFFSET)
	sw	$ra, 0x14($sp)
	sw	$a0, 0x60($sp)
	sw	$a1, 0x64($sp)
	sw	$a2, 0x68($sp)
	ld	$v0, 0($v1)
	li	$t7, -2
	lui	$a0, %hi(data_0xbfc06e38)
	and	$t6, $v0, $t7
	sd	$t6, 0($v1)
	jal	printf_disabled
	 addiu	$a0, $a0, %lo(data_0xbfc06e38)
	lui	$t0, %hi(F_0xbfc04fc0)
	lui	$a3, %hi(.Llocal2)
	addiu	$a3, $a3, %lo(.Llocal2)
	addiu	$t0, $t0, %lo(F_0xbfc04fc0)
	dsll32	$t0, $t0, 0
	dsll32	$a3, $a3, 0
	lui	$v1, 0xa001
	dsrl32	$a3, $a3, 0
	dsrl32	$t0, $t0, 0
	lui	$a0, %hi(data_0xbfc06e58)
	addiu	$a0, $a0, %lo(data_0xbfc06e58)
	move	$a1, $t0
	sw	$t0, 0x1c($sp)
	move	$a2, $a3
	sw	$a3, 0x18($sp)
	jal	printf_disabled
	 sw	$v1, 0x38($sp)
	lw	$a3, 0x18($sp)
	lw	$t0, 0x1c($sp)
	lw	$v1, 0x38($sp)
	xor	$a0, $t0, $a3
	sltu	$a0, $zero, $a0
	beqz	$a0, L_0xbfc061ac
	 addiu	$v0, $t0, 4
L_0xbfc06190: /* 0xbfc06190 */
	lwu	$t8, 0($v0)
	xor	$a0, $v0, $a3
	sltu	$a0, $zero, $a0
	addiu	$v0, $v0, 4
	addiu	$v1, $v1, 4
	bnez	$a0, L_0xbfc06190
	 sw	$t8, -4($v1)
L_0xbfc061ac: /* 0xbfc061ac */
	lui	$a0, %hi(data_0xbfc06e98)
	addiu	$a0, $a0, %lo(data_0xbfc06e98)
	lw	$a1, 0x60($sp)
	lw	$a2, 0x64($sp)
	jal	printf_disabled
	 lw	$a3, 0x68($sp)
	lui	$t9, 0x8001
	lw	$a0, 0x60($sp)
	lw	$a1, 0x64($sp)
	lw	$a2, 0x68($sp)
	jalr	$t9
	 move	$a3, $zero
	lw	$ra, 0x14($sp)
	addiu	$sp, $sp, 0x60
	li	$v0, 1
	jr	$ra
	 nop

/* Function F_0xbfc061f0 [0xbfc061f0 - 0xbfc06390)
 */
F_0xbfc061f0: /* 0xbfc061f0 */
	addiu	$sp, $sp, -0x58
	sw	$ra, 0x3c($sp)
	sw	$s3, 0x24($sp)
	sw	$s2, 0x20($sp)
	move	$s2, $a1
	move	$s3, $a0
	sw	$fp, 0x38($sp)
	sw	$s7, 0x34($sp)
	sw	$s6, 0x30($sp)
	sw	$s5, 0x2c($sp)
	sw	$s4, 0x28($sp)
	sw	$s1, 0x1c($sp)
	jal	get_crime_rev_id
	 sw	$s0, 0x18($sp)
	blezl	$v0, L_0xbfc06364
	 lw	$ra, 0x3c($sp)
	beqz	$s2, L_0xbfc06360
	 li	$fp, -1
	lui	$s5, HI_UNSIGNED(0x00020001)
	lui	$s1, %hi(data_0xbfc09130)
	addiu	$s1, $s1, %lo(data_0xbfc09130)		# Pointer to 0xbfc09130, containing 0xb5000000
	ori	$s5, $s5, LO_UNSIGNED(0x00020001)
	addiu	$s7, $sp, 0x48
	lui	$s6, 2
	lw	$s4, 0x50($sp)
	sltu	$at, $s2, $s5
L_0xbfc06258: /* 0xbfc06258 */
	bnez	$at, L_0xbfc06268
	 move	$a0, $s3
	b	L_0xbfc06274
	 move	$s0, $s6
L_0xbfc06268: /* 0xbfc06268 */
	dsll32	$s2, $s2, 0
	dsra32	$s2, $s2, 0
	move	$s0, $s2
L_0xbfc06274: /* 0xbfc06274 */
	move	$a1, $s0
	li	$a2, 0x700
	jal	F_0xbfc06490
	 move	$a3, $s7
	lw	$v1, 0x48($sp)
	or	$s4, $s4, $v0
	slt	$at, $s0, $v1
	bnez	$at, L_0xbfc0629c
	 nop
	move	$s0, $v1
L_0xbfc0629c: /* 0xbfc0629c */
	jal	F_0xbfc065f8
	 li	$a0, 5
	lw	$t6, 0($s1)
	dsll32	$s0, $s0, 0
	dsrl32	$v0, $s0, 0
	dsll32	$t6, $t6, 0
	dsrl32	$t6, $t6, 0
	addiu	$t8, $t6, MTE_DST0
	dsll32	$t8, $t8, 0
	dsra32	$t9, $t8, 0
	sw	$s3, 0($t9)
	lw	$t2, 0($s1)
	addu	$v1, $v0, $s3
	li	$t1, -1
	dsll32	$t2, $t2, 0
	dsrl32	$t2, $t2, 0
	addiu	$t4, $t2, MTE_DST1
	dsll32	$t4, $t4, 0
	dsra32	$t5, $t4, 0
	addu	$t0, $v1, $t1
	sw	$t0, 0($t5)
	lw	$t6, 0($s1)
	li	$t0, 0x11
	subu	$s2, $s2, $v0
	dsll32	$t6, $t6, 0
	dsrl32	$t6, $t6, 0
	addiu	$t8, $t6, MTE_FG_VALUE
	dsll32	$t8, $t8, 0
	dsra32	$t9, $t8, 0
	sw	$zero, 0($t9)
	lw	$t1, 0($s1)
	move	$s3, $v1
	dsll32	$t1, $t1, 0
	dsrl32	$t1, $t1, 0
	addiu	$t3, $t1, MTE_BYTE_MASK
	dsll32	$t3, $t3, 0
	dsra32	$t4, $t3, 0
	sw	$fp, 0($t4)
	lw	$t5, 0($s1)
	dsll32	$t5, $t5, 0
	dsrl32	$t5, $t5, 0
	addiu	$t7, $t5, MTE_MODE + CRIME_DE_START
	dsll32	$t7, $t7, 0
	dsra32	$t8, $t7, 0
	jal	F_0xbfc06400
	 sw	$t0, 0($t8)
	bnezl	$s2, L_0xbfc06258
	 sltu	$at, $s2, $s5
	sw	$s4, 0x50($sp)
L_0xbfc06360: /* 0xbfc06360 */
	lw	$ra, 0x3c($sp)
L_0xbfc06364: /* 0xbfc06364 */
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
	 addiu	$sp, $sp, 0x58

/* Function F_0xbfc06390 [0xbfc06390 - 0xbfc06400)
 */
F_0xbfc06390: /* 0xbfc06390 */
	addiu	$sp, $sp, -0x18
	sw	$ra, 0x14($sp)
	lui	$a0, %hi(data_0xbfc09130)
	jal	F_0xbfc06730
	 lw	$a0, %lo(data_0xbfc09130)($a0)		# Pointer to 0xbfc09130, containing 0xb5000000
	beqz	$v0, L_0xbfc063b8
	 lw	$ra, 0x14($sp)
	lui	$v0, HI_UNSIGNED(0x00ffffff)
	b	L_0xbfc063f8
	 ori	$v0, $v0, LO_UNSIGNED(0x00ffffff)
L_0xbfc063b8: /* 0xbfc063b8 */
	lui	$t6, %hi(data_0xbfc09130)
	lwu	$t6, %lo(data_0xbfc09130)($t6)		# Pointer to 0xbfc09130, containing 0xb5000000
	addiu	$t8, $t6, CRIME_DE_STATUS
	dsll32	$t8, $t8, 0
	dsra32	$t9, $t8, 0
	lwu	$v0, 0($t9)
	sll	$t0, $v0, 6
	bgez	$t0, L_0xbfc063e4
	 srl	$v1, $v0, 0x12
	b	L_0xbfc063f0
	 move	$v1, $zero
L_0xbfc063e4: /* 0xbfc063e4 */
	andi	$v1, $v1, 0x7f
	sll	$v1, $v1, 8
	addiu	$v1, $v1, 0x300
L_0xbfc063f0: /* 0xbfc063f0 */
	dsll32	$v1, $v1, 0
	dsra32	$v0, $v1, 0
L_0xbfc063f8: /* 0xbfc063f8 */
	jr	$ra
	 addiu	$sp, $sp, 0x18

/* Function F_0xbfc06400 [0xbfc06400 - 0xbfc06490)
 */
F_0xbfc06400: /* 0xbfc06400 */
	addiu	$sp, $sp, -0x20
	sw	$s0, 0x14($sp)
	lui	$s0, %hi(data_0xbfc09130)
	addiu	$s0, $s0, %lo(data_0xbfc09130)		# Pointer to 0xbfc09130, containing 0xb5000000
	lw	$t6, 0($s0)
	sw	$ra, 0x1c($sp)
	sw	$s1, 0x18($sp)
	dsll32	$t6, $t6, 0
	dsrl32	$t6, $t6, 0
	addiu	$t8, $t6, CRIME_DE_STATUS
	dsll32	$t8, $t8, 0
	dsra32	$t9, $t8, 0
	lwu	$v0, 0($t9)
	lui	$s1, 0x200
	sltiu	$v0, $v0, 1
	sll	$t0, $v0, 6
	bgezl	$t0, L_0xbfc06480
	 lw	$ra, 0x1c($sp)
L_0xbfc06448: /* 0xbfc06448 */
	jal	F_0xbfc066b0
	 li	$a0, 1
	lw	$t1, 0($s0)
	dsll32	$t1, $t1, 0
	dsrl32	$t1, $t1, 0
	addiu	$t3, $t1, CRIME_DE_STATUS
	dsll32	$t3, $t3, 0
	dsra32	$t4, $t3, 0
	lwu	$v0, 0($t4)
	sltiu	$v0, $v0, 1
	and	$t5, $v0, $s1
	bnez	$t5, L_0xbfc06448
	 nop
	lw	$ra, 0x1c($sp)
L_0xbfc06480: /* 0xbfc06480 */
	lw	$s0, 0x14($sp)
	lw	$s1, 0x18($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x20

/* Function F_0xbfc06490 [0xbfc06490 - 0xbfc065f8)
 */
F_0xbfc06490: /* 0xbfc06490 */
	addiu	$sp, $sp, -0x38
	dsll32	$a1, $a1, 0
	dsrl32	$a1, $a1, 0
	addu	$v0, $a0, $a1
	li	$t7, -1
	srl	$v1, $a0, 0xc
	sw	$ra, 0x34($sp)
	sw	$s7, 0x30($sp)
	sw	$s6, 0x2c($sp)
	sw	$s5, 0x28($sp)
	sw	$s4, 0x24($sp)
	sw	$s3, 0x20($sp)
	sw	$s2, 0x1c($sp)
	sw	$s1, 0x18($sp)
	sw	$s0, 0x14($sp)
	addu	$t0, $v0, $t7
	andi	$t8, $v1, 1
	move	$s3, $a3
	move	$s7, $a2
	move	$s4, $zero
	move	$s0, $v1
	srl	$s6, $t0, 0xc
	beqz	$t8, L_0xbfc06500
	 sw	$zero, 0($a3)
	li	$t9, -1
	li	$t3, -0x1000
	addu	$s0, $v1, $t9
	sw	$t3, 0($a3)
L_0xbfc06500: /* 0xbfc06500 */
	sll	$v0, $s0, 0xc
	subu	$t4, $t0, $v0
	lui	$at, 2
	sltu	$at, $t4, $at
	bnez	$at, L_0xbfc06524
	 lui	$at, HI_UNSIGNED(0x0001ffff)
	ori	$at, $at, LO_UNSIGNED(0x0001ffff)
	addu	$s6, $v0, $at
	srl	$s6, $s6, 0xc
L_0xbfc06524: /* 0xbfc06524 */
	subu	$t5, $s6, $s0
	srl	$t6, $t5, 1
	dsll32	$t6, $t6, 0
	jal	F_0xbfc065f8
	 dsra32	$a0, $t6, 0
	sltu	$at, $s6, $s0
	bnez	$at, L_0xbfc065c8
	 lui	$s5, %hi(data_0xbfc09130)
	addiu	$s5, $s5, %lo(data_0xbfc09130)		# Pointer to 0xbfc09130, containing 0xb5000000
	lui	$s2, HI(KSEG0)
L_0xbfc0654c: /* 0xbfc0654c */
	andi	$t7, $s0, 0x1f
	lw	$t6, 0($s5)
	srl	$t8, $t7, 1
	dsll32	$t8, $t8, 0
	dsra32	$s1, $t8, 0
	dsll32	$v0, $s1, 0
	or	$t9, $s2, $s0
	addiu	$t2, $s0, 1
	dsll32	$t6, $t6, 0
	dsrl32	$t6, $t6, 0
	or	$t4, $s2, $t2
	dsll32	$t3, $t9, 0
	dsrl32	$v0, $v0, 0
	sll	$t1, $v0, 3
	or	$a0, $t3, $t4
	addu	$t8, $t6, $s7
	addu	$a2, $t8, $t1
	dsll32	$a1, $a0, 0
	dsra32	$a1, $a1, 0
	dsra32	$a0, $a0, 0
	jal	F_0xbfc066f0
	 addiu	$a2, $a2, 0x1000
	lw	$t3, 0($s3)
	addiu	$s0, $s0, 2
	li	$t2, 1
	sltu	$at, $s6, $s0
	sllv	$t4, $t2, $s1
	addiu	$t5, $t3, 0x2000
	or	$s4, $s4, $t4
	beqz	$at, L_0xbfc0654c
	 sw	$t5, 0($s3)
L_0xbfc065c8: /* 0xbfc065c8 */
	lw	$ra, 0x34($sp)
	move	$v0, $s4
	lw	$s4, 0x24($sp)
	lw	$s0, 0x14($sp)
	lw	$s1, 0x18($sp)
	lw	$s2, 0x1c($sp)
	lw	$s3, 0x20($sp)
	lw	$s5, 0x28($sp)
	lw	$s6, 0x2c($sp)
	lw	$s7, 0x30($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x38

/* Function F_0xbfc065f8 [0xbfc065f8 - 0xbfc066b0)
 */
F_0xbfc065f8: /* 0xbfc065f8 */
	addiu	$sp, $sp, -0x28
	sw	$s2, 0x20($sp)
	lui	$s2, %hi(data_0xbfc09130)
	addiu	$s2, $s2, %lo(data_0xbfc09130)		# Pointer to 0xbfc09130, containing 0xb5000000
	lw	$t6, 0($s2)
	sw	$ra, 0x24($sp)
	sw	$s1, 0x1c($sp)
	dsll32	$t6, $t6, 0
	dsrl32	$t6, $t6, 0
	addiu	$t8, $t6, CRIME_DE_STATUS
	dsll32	$t8, $t8, 0
	sw	$s0, 0x18($sp)
	dsra32	$t9, $t8, 0
	lwu	$t0, 0($t9)
	li	$s1, 0x40
	dsll32	$a0, $a0, 0
	srl	$t1, $t0, 0x12
	andi	$t2, $t1, 0x7f
	subu	$t3, $s1, $t2
	dsrl32	$s0, $a0, 0
	sltu	$at, $t3, $s0
	beqzl	$at, L_0xbfc06694
	 lw	$ra, 0x24($sp)
L_0xbfc06654: /* 0xbfc06654 */
	jal	F_0xbfc066b0
	 li	$a0, 1
	lw	$t4, 0($s2)
	dsll32	$t4, $t4, 0
	dsrl32	$t4, $t4, 0
	addiu	$t6, $t4, CRIME_DE_STATUS
	dsll32	$t6, $t6, 0
	dsra32	$t7, $t6, 0
	lwu	$t8, 0($t7)
	srl	$t9, $t8, 0x12
	andi	$t0, $t9, 0x7f
	subu	$t1, $s1, $t0
	sltu	$at, $t1, $s0
	bnez	$at, L_0xbfc06654
	 nop
	lw	$ra, 0x24($sp)
L_0xbfc06694: /* 0xbfc06694 */
	lw	$s0, 0x18($sp)
	lw	$s1, 0x1c($sp)
	lw	$s2, 0x20($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x28
	nop		# unreachable
	nop		# unreachable

/* Function F_0xbfc066b0 [0xbfc066b0 - 0xbfc066f0)
 */
F_0xbfc066b0: /* 0xbfc066b0 */
	li	$a2, 0x42
	mult	$a0, $a2
	lui	$a1, HI_UNSIGNED(BASE_CRIME | CRIME_TIMER_OFFSET)
	ori	$a1, $a1, LO_UNSIGNED(BASE_CRIME | CRIME_TIMER_OFFSET)
	sd	$zero, 0($a1)
	mflo	$a0
	add	$a0, $a0, $zero
	nop
L_0xbfc066d0: /* 0xbfc066d0 */
	ld	$a2, 0($a1)
	dsll32	$a2, $a2, 0
	dsra32	$a2, $a2, 0
	sub	$a2, $a0, $a2
	bgtz	$a2, L_0xbfc066d0
	 nop
	jr	$ra
	 nop

/* Function F_0xbfc066f0 [0xbfc066f0 - 0xbfc06708)
 */
F_0xbfc066f0: /* 0xbfc066f0 */
	dsll32	$a1, $a1, 0
	dsrl32	$a1, $a1, 0
	dsll32	$a0, $a0, 0
	or	$a0, $a0, $a1
	jr	$ra
	 sd	$a0, 0($a2)

/* Function get_crime_rev_id [0xbfc06708 - 0xbfc06720)
 */
get_crime_rev_id: /* 0xbfc06708 */
	lui	$v0, HI(BASE_CRIME + CRIME_ID_OFFSET)
	ld	$v0, LO(BASE_CRIME + CRIME_ID_OFFSET)($v0)
	jr	$ra
	 andi	$v0, $v0, CRIME_ID_REV
	nop		# unreachable
	nop		# unreachable

/* Function F_0xbfc06720 [0xbfc06720 - 0xbfc06728)
 */
F_0xbfc06720: /* 0xbfc06720 */
	jr	$ra
	 move	$v0, $zero

/* Function F_0xbfc06728 [0xbfc06728 - 0xbfc06730)
 */
F_0xbfc06728: /* 0xbfc06728 */
	jr	$ra
	 nop

/* Function F_0xbfc06730 [0xbfc06730 - 0xbfc06740)
 */
F_0xbfc06730: /* 0xbfc06730 */
	jr	$ra
	 move	$v0, $zero
	nop		# unreachable
	nop		# unreachable

/* Function tlb_init_uncached_trampoline [0xbfc06740 - 0xbfc06758)
 *
 * Jump to tlb_init through uncached KSEG1
 */
tlb_init_uncached_trampoline: /* 0xbfc06740 */
	lui	$t1, %hi(tlb_init)
	lui	$t0, HI(KSEG1)
	addiu	$t1, $t1, %lo(tlb_init)
	or	$t0, $t0, $t1
	jr	$t0		# Jump to (KSEG1 | tlb_init)
	 nop

/* Function tlb_init [0xbfc06758 - 0xbfc06b28)
 */
tlb_init: /* 0xbfc06758 */
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
tlb_rm7k_write_tlb_loop: /* 0xbfc0679c */
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
tlb_rm7k_inv_l1i_loop: /* 0xbfc06818 */
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
tlb_rm7k_inv_l1d_loop: /* 0xbfc06848 */
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
tlb_r5k_init: /* 0xbfc06874 */
	li	$t0, R5000_NUM_TLB_ENTRIES-1
	lui	$t1, HI(0x0fffe000)
tlb_r5k_write_tlb_loop: /* 0xbfc0687c */
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
tlb_r5k_inv_l1i_loop: /* 0xbfc068f0 */
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
tlb_r5k_inv_l1d_loop: /* 0xbfc06920 */
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
tlb_r10k_init: /* 0xbfc0694c */
	li	$t0, R10000_NUM_TLB_ENTRIES-1
	lui	$t1, HI(0x0fffe000)
tlb_r10k_write_tlb_loop: /* 0xbfc06954 */
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
tlb_r10k_inv_l1i_loop: /* 0xbfc069ac */
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
tlb_r10k_inv_l1d_loop: /* 0xbfc069d8 */
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
L_0xbfc06a00: /* 0xbfc06a00 */
	slti	$at, $t2, 6
	beqz	$at, L_0xbfc06a00
	 nop
	sllv	$t0, $t1, $t2
	andi	$at, $t0, 0x2000
	li	$t2, 0x80
	bgtz	$at, L_0xbfc06a24
	 nop
	li	$t2, 0x40
L_0xbfc06a24: /* 0xbfc06a24 */
	mtc0	$zero, $CP0_TAGHI
	mtc0	$zero, $CP0_TAGLO
	srl	$t1, $t0, 1
	mtc0	$zero, $CP0_ECC
	sub	$t1, $t1, $t2
tlb_r10k_inv_l2_loop: /* 0xbfc06a38 */
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
tlb_r10k_set_virt_l1d_loop: /* 0xbfc06a68 */
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
tlb_r10k_set_virt_l1i_loop: /* 0xbfc06aa0 */
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
L_0xbfc06ad4: /* 0xbfc06ad4 */
	slti	$at, $t2, 6
	beqz	$at, L_0xbfc06ad4
	 nop
	sllv	$t0, $t1, $t2
	mtc0	$zero, $CP0_ECC
	srl	$t2, $t0, 1
	mtc0	$zero, $CP0_TAGHI
	mtc0	$zero, $CP0_TAGLO
	move	$t0, $zero
tlb_r10k_set_virt_l2_loop: /* 0xbfc06af8 */
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
tlb_r10k_exit: /* 0xbfc06b20 */
	jr	$ra
	 nop

/* Function tlb_init_preserve [0xbfc06b28 - 0xa0004000)
 */
tlb_init_preserve: /* 0xbfc06b28 */
	addiu	$sp, $sp, -0x30
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
data_0xbfc06b70: /* 0xbfc06b70 */
	string 4, "<post1> <SizeMEM>   \n        Bank0 = %dM \n        Bank1 = %dM \n        Bank2 = %dM \n        Bank3 = %dM \n        Bank4 = %dM \n        Bank5 = %dM \n        Bank6 = %dM \n        Bank7 = %dM \n"
data_0xbfc06c30: /* 0xbfc06c30 */
	string 4, "<post1> <SizeMEM> Entering routine\n"
data_0xbfc06c54: /* 0xbfc06c54 */
	string 4, "<post1> <SizeMEM> ECC off \n"
data_0xbfc06c70: /* 0xbfc06c70 */
	string 4, "<post1> <SizeMEM> Initilaize BANK%d\n"
data_0xbfc06c98: /* 0xbfc06c98 */
	string 4, "<post1> <SizeMEM> mem_addr = 0x%0x\n"
data_0xbfc06cbc: /* 0xbfc06cbc */
	string 4, "<post1> <SizeMEM> bank%d =  0x%lx (32M)\n"
data_0xbfc06ce8: /* 0xbfc06ce8 */
	string 4, "<post1> <SizeMEM> bank%d = 0x%0lx (128M)\n"
data_0xbfc06d14: /* 0xbfc06d14 */
	string 4, "Error, no SIMM in bank0\n"
data_0xbfc06d30: /* 0xbfc06d30 */
	string 4, "<post1> <SizeMEM> bank%d = 0x%0lx, no simm installed\n"
data_0xbfc06d68: /* 0xbfc06d68 */
	string 4, "<post1> <SizeMEM> bank%d = 128M mem_addr[next] = 0x%0x\n"
data_0xbfc06da0: /* 0xbfc06da0 */
	string 4, "<post1> <SizeMEM> bank%d = 32M mem_addr[next] = 0x%0x\n"
data_0xbfc06dd8: /* 0xbfc06dd8 */
	string 4, "<post1> <SizeMEM> bank%d = 0M BankControl = 0x%0lx\n"
data_0xbfc06e0c: /* 0xbfc06e0c */
	string 4, "<post1> <SizeMEM> MEM Size = 0x%0x bytes\n"
data_0xbfc06e38: /* 0xbfc06e38 */
	string 4, "<post1> <CopyPOST1> ECC off \n"
data_0xbfc06e58: /* 0xbfc06e58 */
	string 4, "<post1> <CopyPOST1>\n  pass 1: Copy bootstrap 0x%0x to 0x%0x\n"
data_0xbfc06e98: /* 0xbfc06e98 */
	string 4, "<post1> <CopyPOST1>\n        pass 1: Bootstrap POST1DIAGS\n                Starting addr: 0x%0x\n                Ending   addr: 0x%0x\n                Target   addr: 0x%0x\n"
	.int	0x00000000
data_0xbfc06f48: /* 0xbfc06f48 */
	.int	0xffffffff
	.int	0x00000000
data_0xbfc06f50: /* 0xbfc06f50 */
	.int	0xfe000007
	.int	0x01fffff8
data_0xbfc06f58: /* 0xbfc06f58 */
	.int	0xfdffffff
	.int	0x02000000
data_0xbfc06f60: /* 0xbfc06f60 */
	.int	0xf8000007
	.int	0x07fffff8
data_0xbfc06f68: /* 0xbfc06f68 */
	.int	0xfdffffff
	.int	0x02000000
data_0xbfc06f70: /* 0xbfc06f70 */
	.int	0xf8000007
	.int	0x07fffff8
data_0xbfc06f78: /* 0xbfc06f78 */
	.int	0xfdffffff
	.int	0x02000000
data_0xbfc06f80: /* 0xbfc06f80 */
	.int	0xf8000007
	.int	0x07fffff8
data_0xbfc06f88: /* 0xbfc06f88 */
	.int	0xffffffff
	.int	0x00000000
data_0xbfc06f90: /* 0xbfc06f90 */
	.int	0xfe000007
	.int	0x01fffff8
data_0xbfc06f98: /* 0xbfc06f98 */
	.int	0xfdffffff
	.int	0x02000000
data_0xbfc06fa0: /* 0xbfc06fa0 */
	.int	0xf8000007
	.int	0x07fffff8
	.fill	2, 4, 0x00000000
data_0xbfc06fb0: /* 0xbfc06fb0 */
	string 4, "0123456789abcdef"
	.fill	3, 4, 0x00000000

/* Relocated code section: executes at VMA, stored at LMA */
.section .text_ram, "ax"


/* Function F_0xa0004000 [0xa0004000 - 0xa0004080)
 */
F_0xa0004000: /* 0xa0004000 */
	addiu	$sp, $sp, -0x90
	sd	$at, 0($sp)
	sd	$ra, 8($sp)
	sd	$s0, 0x10($sp)
	sd	$s1, 0x18($sp)
	sd	$s2, 0x20($sp)
	sd	$s3, 0x28($sp)
	sd	$s4, 0x30($sp)
	sd	$s5, 0x38($sp)
	sd	$s6, 0x40($sp)
	sd	$s7, 0x48($sp)
	sd	$s7, 0x50($sp)
	lui	$s0, %hi(F_0xa000542c)
	addiu	$s0, $s0, %lo(F_0xa000542c)
	jalr	$s0
	 nop
	ld	$at, 0($sp)
	ld	$ra, 8($sp)
	ld	$s0, 0x10($sp)
	ld	$s1, 0x18($sp)
	ld	$s2, 0x20($sp)
	ld	$s3, 0x28($sp)
	ld	$s4, 0x30($sp)
	ld	$s5, 0x38($sp)
	ld	$s6, 0x40($sp)
	ld	$s7, 0x48($sp)
	ld	$s7, 0x50($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x90
	nop		# unreachable
	nop		# unreachable
	nop		# unreachable

/* Function F_0xa0004080 [0xa0004080 - 0xa00040b0)
 */
F_0xa0004080: /* 0xa0004080 */
	mfc0	$a0, $CP0_PRID
	nop
	li	$v0, PRID_IMP_R5000
	srl	$a0, $a0, PRID_IMP_SHIFT
	andi	$a0, $a0, PRID_REV_MASK
	bne	$v0, $a0, L_0xa00040a4
	 nop
	b	L_0xa00040a8
	 li	$v0, 0x30
L_0xa00040a4: /* 0xa00040a4 */
	nop
L_0xa00040a8: /* 0xa00040a8 */
	jr	$ra
	 move	$a0, $zero

/* Function F_0xa00040b0 [0xa00040b0 - 0xa00040d4)
 */
F_0xa00040b0: /* 0xa00040b0 */
	mfc0	$a0, $CP0_STATUS
	nop
	li	$a1, ST0_IE
	not	$a1, $a1
	and	$a0, $a0, $a1
	mtc0	$a0, $CP0_STATUS
	move	$a1, $zero
	jr	$ra
	 move	$a0, $zero

/* Function F_0xa00040d4 [0xa00040d4 - 0xa0004104)
 */
F_0xa00040d4: /* 0xa00040d4 */
	mfc0	$a2, $CP0_STATUS
	li	$a1, ST0_IM
	not	$a1, $a1
	and	$a1, $a1, $a2
	sll	$a0, $a0, 8
	or	$a1, $a0, $a1
	ori	$a1, $a1, ST0_IE
	mtc0	$a1, $CP0_STATUS
	move	$a0, $zero
	move	$a1, $zero
	jr	$ra
	 move	$a2, $zero

/* Function F_0xa0004104 [0xa0004104 - 0xa000411c)
 */
F_0xa0004104: /* 0xa0004104 */
	lui	$v0, HI(KSEG1)
	addiu	$v0, $v0, LO(KSEG1)
	b	F_0xa0004104
	 sd	$ra, LO(KSEG1)($v0)
	jr	$ra		# unreachable
	 nop		# unreachable

/* Function F_0xa000411c [0xa000411c - 0xa000413c)
 */
F_0xa000411c: /* 0xa000411c */
	mfc0	$v0, $CP0_CONFIG
	li	$a0, 0x1000
	nop
	srl	$v0, $v0, 9
	andi	$v0, $v0, 7
	sllv	$v0, $a0, $v0
	jr	$ra
	 move	$a0, $zero

/* Function F_0xa000413c [0xa000413c - 0xa000415c)
 */
F_0xa000413c: /* 0xa000413c */
	mfc0	$v0, $CP0_CONFIG
	li	$a0, 0x1000
	nop
	srl	$v0, $v0, 6
	andi	$v0, $v0, 7
	sllv	$v0, $a0, $v0
	jr	$ra
	 move	$a0, $zero

/* Function F_0xa000415c [0xa000415c - 0xa0004168)
 */
F_0xa000415c: /* 0xa000415c */
	mtc0	$a0, $CP0_CONFIG
	jr	$ra
	 nop

/* Function F_0xa0004168 [0xa0004168 - 0xa0004174)
 */
F_0xa0004168: /* 0xa0004168 */
	mfc0	$v0, $CP0_CONFIG
	jr	$ra
	 nop

/* Function F_0xa0004174 [0xa0004174 - 0xa0004180)
 */
F_0xa0004174: /* 0xa0004174 */
	mtc0	$a0, $CP0_EPC
	jr	$ra
	 nop

/* Function F_0xa0004180 [0xa0004180 - 0xa000418c)
 */
F_0xa0004180: /* 0xa0004180 */
	mfc0	$v0, $CP0_EPC
	jr	$ra
	 nop

/* Function F_0xa000418c [0xa000418c - 0xa0004198)
 */
F_0xa000418c: /* 0xa000418c */
	mfc0	$v0, $CP0_CACHEERR
	jr	$ra
	 nop

/* Function F_0xa0004198 [0xa0004198 - 0xa00041a4)
 */
F_0xa0004198: /* 0xa0004198 */
	mfc0	$v0, $CP0_STATUS
	jr	$ra
	 nop

/* Function F_0xa00041a4 [0xa00041a4 - 0xa00041b0)
 */
F_0xa00041a4: /* 0xa00041a4 */
	mfc0	$v0, $CP0_CAUSE
	jr	$ra
	 nop

/* Function F_0xa00041b0 [0xa00041b0 - 0xa00041c0)
 */
F_0xa00041b0: /* 0xa00041b0 */
	mtc0	$a0, $CP0_INDEX
	nop
	jr	$ra
	 nop

/* Function F_0xa00041c0 [0xa00041c0 - 0xa00041d0)
 */
F_0xa00041c0: /* 0xa00041c0 */
	mtc0	$v0, $CP0_INDEX
	nop
	jr	$ra
	 nop

/* Function F_0xa00041d0 [0xa00041d0 - 0xa00041e0)
 */
F_0xa00041d0: /* 0xa00041d0 */
	mtc0	$a0, $CP0_ENTRYLO0
	nop
	jr	$ra
	 nop

/* Function F_0xa00041e0 [0xa00041e0 - 0xa00041f0)
 */
F_0xa00041e0: /* 0xa00041e0 */
	mtc0	$a0, $CP0_ENTRYLO1
	nop
	jr	$ra
	 nop

/* Function F_0xa00041f0 [0xa00041f0 - 0xa0004200)
 */
F_0xa00041f0: /* 0xa00041f0 */
	mtc0	$a0, $CP0_ENTRYHI
	nop
	jr	$ra
	 nop

/* Function F_0xa0004200 [0xa0004200 - 0xa0004210)
 */
F_0xa0004200: /* 0xa0004200 */
	mtc0	$a0, $CP0_PAGEMASK
	nop
	jr	$ra
	 nop

/* Function F_0xa0004210 [0xa0004210 - 0xa0004218)
 */
F_0xa0004210: /* 0xa0004210 */
	jr	$ra
	 tlbp

/* Function F_0xa0004218 [0xa0004218 - 0xa0004220)
 */
F_0xa0004218: /* 0xa0004218 */
	jr	$ra
	 tlbr

/* Function F_0xa0004220 [0xa0004220 - 0xa0004228)
 */
F_0xa0004220: /* 0xa0004220 */
	jr	$ra
	 tlbwi

/* Function F_0xa0004228 [0xa0004228 - 0xa0004238)
 */
F_0xa0004228: /* 0xa0004228 */
	mtc0	$a0, $CP0_TAGLO
	nop
	jr	$ra
	 nop

/* Function F_0xa0004238 [0xa0004238 - 0xa0004248)
 */
F_0xa0004238: /* 0xa0004238 */
	mfc0	$v0, $CP0_TAGLO
	nop
	jr	$ra
	 nop

/* Function F_0xa0004248 [0xa0004248 - 0xa0004250)
 */
F_0xa0004248: /* 0xa0004248 */
	jr	$ra
	 cache	(CACHE_TYPE_L1D|HIT_INVALIDATE), 0($a0)

/* Function F_0xa0004250 [0xa0004250 - 0xa0004258)
 */
F_0xa0004250: /* 0xa0004250 */
	jr	$ra
	 cache	(CACHE_TYPE_L1D|HIT_WRITEBACK_INV), 0($a0)

/* Function F_0xa0004258 [0xa0004258 - 0xa0004260)
 */
F_0xa0004258: /* 0xa0004258 */
	jr	$ra
	 cache	(CACHE_TYPE_L1D|HIT_WRITEBACK), 0($a0)

/* Function F_0xa0004260 [0xa0004260 - 0xa0004268)
 */
F_0xa0004260: /* 0xa0004260 */
	jr	$ra
	 cache	(CACHE_TYPE_L1D|HIT_SET_VIRTUAL), 0($a0)

/* Function F_0xa0004268 [0xa0004268 - 0xa0004270)
 */
F_0xa0004268: /* 0xa0004268 */
	jr	$ra
	 cache	(CACHE_TYPE_L1D|INDEX_WRITEBACK_INV), 0($a0)

/* Function F_0xa0004270 [0xa0004270 - 0xa0004278)
 */
F_0xa0004270: /* 0xa0004270 */
	jr	$ra
	 cache	(CACHE_TYPE_L1D|INDEX_LOAD_TAG), 0($a0)

/* Function F_0xa0004278 [0xa0004278 - 0xa0004280)
 */
F_0xa0004278: /* 0xa0004278 */
	jr	$ra
	 cache	(CACHE_TYPE_L1D|INDEX_STORE_TAG), 0($a0)

/* Function F_0xa0004280 [0xa0004280 - 0xa0004288)
 */
F_0xa0004280: /* 0xa0004280 */
	jr	$ra
	 cache	(CACHE_TYPE_L1D|CREATE_DIRTY_EXCLUSIVE), 0($a0)

/* Function F_0xa0004288 [0xa0004288 - 0xa0004290)
 */
F_0xa0004288: /* 0xa0004288 */
	jr	$ra
	 cache	(CACHE_TYPE_L1I|HIT_WRITEBACK_INV), 0($a0)

/* Function F_0xa0004290 [0xa0004290 - 0xa0004298)
 */
F_0xa0004290: /* 0xa0004290 */
	jr	$ra
	 cache	(CACHE_TYPE_L1I|INDEX_WRITEBACK_INV), 0($a0)

/* Function F_0xa0004298 [0xa0004298 - 0xa00042a4)
 */
F_0xa0004298: /* 0xa0004298 */
	lui	$a0, HI(KSEG0)
	jr	$ra
	 cache	(CACHE_TYPE_L2|INDEX_WRITEBACK_INV), 0($a0)

/* Function F_0xa00042a4 [0xa00042a4 - 0xa00042ac)
 */
F_0xa00042a4: /* 0xa00042a4 */
	jr	$ra
	 cache	(CACHE_TYPE_L2|HIT_WRITEBACK_INV), 0($zero)

/* Function F_0xa00042ac [0xa00042ac - 0xa00042b4)
 */
F_0xa00042ac: /* 0xa00042ac */
	jr	$ra
	 cache	(CACHE_TYPE_L2|INDEX_LOAD_TAG), 0($a0)

/* Function F_0xa00042b4 [0xa00042b4 - 0xa00042bc)
 */
F_0xa00042b4: /* 0xa00042b4 */
	jr	$ra
	 cache	(CACHE_TYPE_L2|INDEX_STORE_TAG), 0($a0)

/* Function F_0xa00042bc [0xa00042bc - 0xa00042c0)
 */
F_0xa00042bc: /* 0xa00042bc */
	nop

/* Function F_0xa00042c0 [0xa00042c0 - 0xa0004384)
 */
F_0xa00042c0: /* 0xa00042c0 */
	addiu	$sp, $sp, -0x30
	sw	$ra, 0x1c($sp)
	sw	$s0, 0x18($sp)
	lui	$t6, HI(BASE_ISA + ISA_RING_BASE_AND_RESET)
	jal	F_0xa0004384
	 sd	$zero, LO(BASE_ISA + ISA_RING_BASE_AND_RESET)($t6)
	beqz	$v0, L_0xa0004300
	 lui	$s0, HI_UNSIGNED(BASE_ISA | ISA_MISC_CONTROL)
	ori	$s0, $s0, LO_UNSIGNED(BASE_ISA | ISA_MISC_CONTROL)
	li	$t7, 0x20
	lui	$a0, HI_UNSIGNED(1000000)
	sd	$t7, 0($s0)
	jal	F_0xa00047d0
	 ori	$a0, $a0, LO_UNSIGNED(1000000)
	li	$t8, 0x30
	sd	$t8, 0($s0)
L_0xa0004300: /* 0xa0004300 */
	lui	$s0, HI_UNSIGNED(BASE_ISA | ISA_MISC_CONTROL)
	ori	$s0, $s0, LO_UNSIGNED(BASE_ISA | ISA_MISC_CONTROL)
	jal	F_0xa0004494
	 li	$a0, 1
	beqz	$v0, L_0xa0004330
	 li	$t9, 0x20
	lui	$a0, HI_UNSIGNED(1000000)
	sd	$t9, 0($s0)
	jal	F_0xa00047d0
	 ori	$a0, $a0, LO_UNSIGNED(1000000)
	li	$t0, 0x30
	sd	$t0, 0($s0)
L_0xa0004330: /* 0xa0004330 */
	jal	F_0xa0004494
	 li	$a0, 2
	beqz	$v0, L_0xa0004370
	 sw	$v0, 0x20($sp)
	li	$t1, 0x20
	lui	$a0, HI_UNSIGNED(1000000)
	sd	$t1, 0($s0)
	jal	F_0xa00047d0
	 ori	$a0, $a0, LO_UNSIGNED(1000000)
	li	$t2, 0x20
	lui	$a0, HI_UNSIGNED(1000000)
	sd	$t2, 0($s0)
	jal	F_0xa00047d0
	 ori	$a0, $a0, LO_UNSIGNED(1000000)
	li	$t3, 0x30
	sd	$t3, 0($s0)
L_0xa0004370: /* 0xa0004370 */
	lw	$ra, 0x1c($sp)
	lw	$v0, 0x20($sp)
	lw	$s0, 0x18($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x30

/* Function F_0xa0004384 [0xa0004384 - 0xa0004494)
 */
F_0xa0004384: /* 0xa0004384 */
	lui	$a0, HI(BASE_UART_1 + UART_SCR)
	li	$a1, 0x55
	lui	$a2, HI_UNSIGNED(BASE_UART_2)
	ori	$a2, $a2, LO_UNSIGNED(BASE_UART_2)
	sb	$a1, LO(BASE_UART_1 + UART_SCR)($a0)
	lbu	$zero, LO(BASE_UART_1 + UART_SCR)($a2)
	lbu	$v0, LO(BASE_UART_1 + UART_SCR)($a0)
	li	$a3, 0x55
	move	$v1, $zero
	dsll32	$v0, $v0, 0
	dsra32	$v0, $v0, 0
	beq	$a3, $v0, L_0xa00043bc
	 li	$t0, 0xaa
	li	$v1, 1
L_0xa00043bc: /* 0xa00043bc */
	sb	$t0, LO(BASE_UART_1 + UART_SCR)($a0)
	lbu	$zero, LO(BASE_UART_1 + UART_SCR)($a2)
	lbu	$v0, LO(BASE_UART_1 + UART_SCR)($a0)
	li	$t1, 0xaa
	li	$t2, 0xf0
	dsll32	$v0, $v0, 0
	dsra32	$v0, $v0, 0
	beq	$t1, $v0, L_0xa00043e4
	 li	$t8, 0xf
	ori	$v1, $v1, 2
L_0xa00043e4: /* 0xa00043e4 */
	sb	$t2, LO(BASE_UART_1 + UART_SCR)($a0)
	lbu	$zero, LO(BASE_UART_1 + UART_SCR)($a2)
	lbu	$zero, LO(BASE_UART_1 + UART_SCR)($a0)
	sb	$t8, LO(BASE_UART_1 + UART_SCR)($a0)
	lbu	$zero, LO(BASE_UART_1 + UART_SCR)($a2)
	lbu	$zero, LO(BASE_UART_1 + UART_SCR)($a0)
	lbu	$zero, LO(BASE_UART_1 + UART_DATA)($a0)
	lbu	$zero, LO(BASE_UART_1 + UART_IER)($a0)
	lbu	$zero, LO(BASE_UART_1 + UART_IIR)($a0)
	lbu	$zero, LO(BASE_UART_1 + UART_LCR)($a0)
	lbu	$zero, LO(BASE_UART_1 + UART_MCR)($a0)
	lbu	$zero, LO(BASE_UART_1 + UART_LSR)($a0)
	lbu	$zero, LO(BASE_UART_1 + UART_MSR)($a0)
	sb	$a1, LO(BASE_UART_1 + UART_SCR)($a2)
	lbu	$zero, LO(BASE_UART_1 + UART_SCR)($a0)
	lbu	$v0, LO(BASE_UART_1 + UART_SCR)($a2)
	dsll32	$v0, $v0, 0
	dsra32	$v0, $v0, 0
	beql	$a3, $v0, L_0xa000443c
	 sb	$t0, LO(BASE_UART_1 + UART_SCR)($a2)
	ori	$v1, $v1, 4
	sb	$t0, LO(BASE_UART_1 + UART_SCR)($a2)
L_0xa000443c: /* 0xa000443c */
	lbu	$zero, LO(BASE_UART_1 + UART_SCR)($a0)
	lbu	$v0, LO(BASE_UART_1 + UART_SCR)($a2)
	dsll32	$v0, $v0, 0
	dsra32	$v0, $v0, 0
	beql	$t1, $v0, L_0xa000445c
	 sb	$t2, LO(BASE_UART_1 + UART_SCR)($a2)
	ori	$v1, $v1, 8
	sb	$t2, LO(BASE_UART_1 + UART_SCR)($a2)
L_0xa000445c: /* 0xa000445c */
	lbu	$zero, LO(BASE_UART_1 + UART_SCR)($a0)
	lbu	$zero, LO(BASE_UART_1 + UART_SCR)($a2)
	sb	$t2, LO(BASE_UART_1 + UART_SCR)($a2)
	lbu	$zero, LO(BASE_UART_1 + UART_SCR)($a0)
	lbu	$zero, LO(BASE_UART_1 + UART_SCR)($a2)
	lbu	$zero, LO(BASE_UART_1 + UART_DATA)($a2)
	lbu	$zero, LO(BASE_UART_1 + UART_IER)($a2)
	lbu	$zero, LO(BASE_UART_1 + UART_IIR)($a2)
	lbu	$zero, LO(BASE_UART_1 + UART_LCR)($a2)
	lbu	$zero, LO(BASE_UART_1 + UART_MCR)($a2)
	lbu	$zero, LO(BASE_UART_1 + UART_LSR)($a2)
	lbu	$zero, LO(BASE_UART_1 + UART_MSR)($a2)
	jr	$ra
	 move	$v0, $v1

/* Function F_0xa0004494 [0xa0004494 - 0xa00047d0)
 */
F_0xa0004494: /* 0xa0004494 */
	addiu	$sp, $sp, -0x148
	li	$at, 1
	sw	$ra, 0x24($sp)
	sw	$s3, 0x20($sp)
	sw	$s2, 0x1c($sp)
	sw	$s1, 0x18($sp)
	bne	$a0, $at, L_0xa00044bc
	 sw	$s0, 0x14($sp)
	b	L_0xa00044d8
	 lui	$s2, HI(BASE_UART_1 + UART_LCR)
L_0xa00044bc: /* 0xa00044bc */
	li	$at, 2
	bne	$a0, $at, L_0xa00044d0
	 lui	$s2, HI_UNSIGNED(BASE_UART_2)
	b	L_0xa00044d8
	 ori	$s2, $s2, LO_UNSIGNED(BASE_UART_2)
L_0xa00044d0: /* 0xa00044d0 */
	b	L_0xa00047b4
	 li	$v0, 0x200
L_0xa00044d8: /* 0xa00044d8 */
	li	$t6, 0x80
	sb	$t6, LO(BASE_UART_1 + UART_LCR)($s2)
	li	$t7, 0x18
	sb	$t7, LO(BASE_UART_1 + UART_SCR)($s2)
	sb	$zero, LO(BASE_UART_1 + UART_IER)($s2)
	li	$t8, 0xc
	sb	$t8, LO(BASE_UART_1 + UART_DATA)($s2)
	li	$t9, 3
	sb	$t9, LO(BASE_UART_1 + UART_LCR)($s2)
	li	$t0, 1
	sb	$t0, LO(BASE_UART_1 + UART_IIR)($s2)
	sb	$zero, LO(BASE_UART_1 + UART_IER)($s2)
	li	$t1, 0x13
	sb	$t1, LO(BASE_UART_1 + UART_MCR)($s2)
	lbu	$t2, LO(BASE_UART_1 + UART_LSR)($s2)
	move	$s1, $zero
	addiu	$t5, $sp, 0x44
	dsll32	$t2, $t2, 0
	dsra32	$t2, $t2, 0
	andi	$t4, $t2, 1
	beqz	$t4, L_0xa0004564
	 move	$s3, $zero
	addu	$s0, $s1, $t5
	addiu	$v0, $sp, 0x64
	lbu	$t6, LO(BASE_UART_1 + UART_DATA)($s2)
L_0xa000453c: /* 0xa000453c */
	addiu	$s0, $s0, 1
	sb	$t6, -1($s0)
	lbu	$t7, LO(BASE_UART_1 + UART_LSR)($s2)
	dsll32	$t7, $t7, 0
	dsra32	$t7, $t7, 0
	andi	$t9, $t7, 1
	beqzl	$t9, L_0xa0004568
	 move	$s1, $zero
	bnel	$s0, $v0, L_0xa000453c
	 lbu	$t6, LO(BASE_UART_1 + UART_DATA)($s2)
L_0xa0004564: /* 0xa0004564 */
	move	$s1, $zero
L_0xa0004568: /* 0xa0004568 */
	lbu	$t0, LO(BASE_UART_1 + UART_LSR)($s2)
L_0xa000456c: /* 0xa000456c */
	li	$s0, 10000
	dsll32	$t0, $t0, 0
	dsra32	$t0, $t0, 0
	andi	$t2, $t0, 0x20
	bnezl	$t2, L_0xa00045b8
	 sb	$s3, LO(BASE_UART_1 + UART_DATA)($s2)
L_0xa0004584: /* 0xa0004584 */
	jal	F_0xa00047d0
	 li	$a0, 0x64
	lbu	$t4, LO(BASE_UART_1 + UART_LSR)($s2)
	li	$t3, -1
	addu	$s0, $s0, $t3
	dsll32	$t4, $t4, 0
	dsra32	$t4, $t4, 0
	andi	$t6, $t4, 0x20
	bnezl	$t6, L_0xa00045b8
	 sb	$s3, LO(BASE_UART_1 + UART_DATA)($s2)
	bnez	$s0, L_0xa0004584
	 nop
	sb	$s3, LO(BASE_UART_1 + UART_DATA)($s2)
L_0xa00045b8: /* 0xa00045b8 */
	lbu	$t7, LO(BASE_UART_1 + UART_LSR)($s2)
	addiu	$s3, $s3, 1
	andi	$s3, $s3, 0xff
	dsll32	$t7, $t7, 0
	dsra32	$t7, $t7, 0
	andi	$t9, $t7, 1
	beqz	$t9, L_0xa000461c
	 dsll32	$s3, $s3, 0
	slti	$at, $s1, 0x100
	beqz	$at, L_0xa000461c
	 addiu	$t0, $sp, 0x44
	addu	$s0, $s1, $t0
	lbu	$t1, LO(BASE_UART_1 + UART_DATA)($s2)
L_0xa00045ec: /* 0xa00045ec */
	addiu	$s1, $s1, 1
	slti	$at, $s1, 0x100
	sb	$t1, 0($s0)
	lbu	$t2, LO(BASE_UART_1 + UART_LSR)($s2)
	addiu	$s0, $s0, 1
	dsll32	$t2, $t2, 0
	dsra32	$t2, $t2, 0
	andi	$t4, $t2, 1
	beqzl	$t4, L_0xa0004620
	 dsra32	$s3, $s3, 0
	bnezl	$at, L_0xa00045ec
	 lbu	$t1, LO(BASE_UART_1 + UART_DATA)($s2)
L_0xa000461c: /* 0xa000461c */
	dsra32	$s3, $s3, 0
L_0xa0004620: /* 0xa0004620 */
	slti	$at, $s3, 0xff
	bnezl	$at, L_0xa000456c
	 lbu	$t0, LO(BASE_UART_1 + UART_LSR)($s2)
	jal	F_0xa00047d0
	 li	$a0, 10000
	lbu	$t6, LO(BASE_UART_1 + UART_LSR)($s2)
	slti	$at, $s1, 0x100
	dsll32	$t6, $t6, 0
	dsra32	$t6, $t6, 0
	andi	$t8, $t6, 1
	beqzl	$t8, L_0xa000469c
	 lb	$t4, 0x44($sp)
	beqz	$at, L_0xa0004698
	 addiu	$t9, $sp, 0x44
	addu	$s0, $s1, $t9
	addiu	$s1, $sp, 0x144
	lbu	$t0, LO(BASE_UART_1 + UART_DATA)($s2)
L_0xa0004664: /* 0xa0004664 */
	addiu	$s0, $s0, 1
	li	$a0, 10000
	jal	F_0xa00047d0
	 sb	$t0, -1($s0)
	lbu	$t1, LO(BASE_UART_1 + UART_LSR)($s2)
	sltu	$at, $s0, $s1
	dsll32	$t1, $t1, 0
	dsra32	$t1, $t1, 0
	andi	$t3, $t1, 1
	beqzl	$t3, L_0xa000469c
	 lb	$t4, 0x44($sp)
	bnezl	$at, L_0xa0004664
	 lbu	$t0, LO(BASE_UART_1 + UART_DATA)($s2)
L_0xa0004698: /* 0xa0004698 */
	lb	$t4, 0x44($sp)
L_0xa000469c: /* 0xa000469c */
	lb	$t6, 0x45($sp)
	li	$at, 1
	beqz	$t4, L_0xa00046b4
	 nop
	b	L_0xa00047b4
	 li	$v0, 0x100
L_0xa00046b4: /* 0xa00046b4 */
	beq	$t6, $at, L_0xa00046c4
	 lb	$t8, 0x46($sp)
	b	L_0xa00047b4
	 li	$v0, 0x101
L_0xa00046c4: /* 0xa00046c4 */
	li	$at, 2
	beq	$t8, $at, L_0xa00046d8
	 li	$s1, 3
	b	L_0xa00047b4
	 li	$v0, CTYPE_TOLOWER
L_0xa00046d8: /* 0xa00046d8 */
	addiu	$s0, $sp, 0x47
	li	$v0, 0xff
L_0xa00046e0: /* 0xa00046e0 */
	lbu	$t3, 0($s0)
	dsll32	$s1, $s1, 0
	dsrl32	$s1, $s1, 0
	andi	$t1, $s1, 0xff
	dsll32	$t1, $t1, 0
	dsll32	$t3, $t3, 0
	dsra32	$t3, $t3, 0
	dsra32	$t1, $t1, 0
	beq	$t1, $t3, L_0xa0004710
	 addiu	$v1, $s1, 1
	b	L_0xa00047b4
	 ori	$v0, $s1, 0x100
L_0xa0004710: /* 0xa0004710 */
	lbu	$t8, 1($s0)
	dsll32	$v1, $v1, 0
	dsrl32	$v1, $v1, 0
	andi	$t6, $v1, 0xff
	dsll32	$t6, $t6, 0
	dsll32	$t8, $t8, 0
	dsra32	$t8, $t8, 0
	dsra32	$t6, $t6, 0
	beql	$t6, $t8, L_0xa0004744
	 addiu	$v1, $s1, 2
	b	L_0xa00047b4
	 ori	$v0, $v1, 0x100
	addiu	$v1, $s1, 2		# unreachable
L_0xa0004744: /* 0xa0004744 */
	lbu	$t2, 2($s0)
	dsll32	$v1, $v1, 0
	dsrl32	$v1, $v1, 0
	andi	$t1, $v1, 0xff
	dsll32	$t1, $t1, 0
	dsll32	$t2, $t2, 0
	dsra32	$t2, $t2, 0
	dsra32	$t1, $t1, 0
	beql	$t1, $t2, L_0xa0004778
	 addiu	$v1, $s1, 3
	b	L_0xa00047b4
	 ori	$v0, $v1, 0x100
	addiu	$v1, $s1, 3		# unreachable
L_0xa0004778: /* 0xa0004778 */
	lbu	$t7, 3($s0)
	dsll32	$v1, $v1, 0
	dsrl32	$v1, $v1, 0
	andi	$t6, $v1, 0xff
	dsll32	$t6, $t6, 0
	dsll32	$t7, $t7, 0
	dsra32	$t7, $t7, 0
	dsra32	$t6, $t6, 0
	beq	$t6, $t7, L_0xa00047a8
	 addiu	$s1, $s1, 4
	b	L_0xa00047b4
	 ori	$v0, $v1, 0x100
L_0xa00047a8: /* 0xa00047a8 */
	bne	$s1, $v0, L_0xa00046e0
	 addiu	$s0, $s0, 4
	move	$v0, $zero
L_0xa00047b4: /* 0xa00047b4 */
	lw	$ra, 0x24($sp)
	lw	$s0, 0x14($sp)
	lw	$s1, 0x18($sp)
	lw	$s2, 0x1c($sp)
	lw	$s3, 0x20($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x148

/* Function F_0xa00047d0 [0xa00047d0 - 0xa0004820)
 */
F_0xa00047d0: /* 0xa00047d0 */
	lui	$v0, HI(BASE_UST)
	ld	$v1, LO(BASE_UST)($v0)
	ld	$t8, LO(BASE_UST)($v0)
	li	$at, -1
	and	$t6, $v1, $at
	daddu	$v1, $t6, $a0
	and	$t9, $t8, $at
	sltu	$at, $t9, $v1
	beqz	$at, L_0xa0004810
	 nop
	ld	$t0, LO(BASE_UST)($v0)
L_0xa00047fc: /* 0xa00047fc */
	li	$at, -1
	and	$t1, $t0, $at
	sltu	$at, $t1, $v1
	bnezl	$at, L_0xa00047fc
	 ld	$t0, LO(BASE_UST)($v0)
L_0xa0004810: /* 0xa0004810 */
	jr	$ra
	 nop
	nop		# unreachable
	nop		# unreachable

/* Function F_0xa0004820 [0xa0004820 - 0xa0004864)
 */
F_0xa0004820: /* 0xa0004820 */
	beqz	$a0, L_0xa000485c
	 move	$v0, $a0
	andi	$a1, $a0, 3
	negu	$a1, $a1
	beqz	$a1, L_0xa000484c
	 addu	$v1, $a1, $a0
	li	$t6, -1
L_0xa000483c: /* 0xa000483c */
	addu	$v0, $v0, $t6
	bnel	$v1, $v0, L_0xa000483c
	 li	$t6, -1
	beqz	$v0, L_0xa000485c
L_0xa000484c: /* 0xa000484c */
	 li	$t7, -4
L_0xa0004850: /* 0xa0004850 */
	addu	$v0, $v0, $t7
	bnezl	$v0, L_0xa0004850
	 li	$t7, -4
L_0xa000485c: /* 0xa000485c */
	jr	$ra
	 nop

/* Function F_0xa0004864 [0xa0004864 - 0xa0004888)
 */
F_0xa0004864: /* 0xa0004864 */
	lui	$t6, HI(0xbfd00703)
	lbu	$t7, LO(0xbfd00703)($t6)
	dsll32	$t7, $t7, 0
	dsra32	$t7, $t7, 0
	xori	$t9, $t7, 0xde
	sltiu	$t9, $t9, 1
	dsll32	$t9, $t9, 0
	jr	$ra
	 dsrl32	$v0, $t9, 0

/* Function F_0xa0004888 [0xa0004888 - 0xa0004920)
 */
F_0xa0004888: /* 0xa0004888 */
	addiu	$sp, $sp, -0x18
	sw	$ra, 0x14($sp)
	li	$t6, 0x80
	lui	$t7, HI(0xbfd00380)
	sb	$t6, LO(0xbfd00380)($t7)
	jal	F_0xa0004820
	 li	$a0, 0x20
	lui	$t8, HI(0xbfd00100)
	sb	$zero, LO(0xbfd00100)($t8)
	jal	F_0xa0004820
	 li	$a0, 0x20
	li	$t9, 0x18
	lui	$t0, HI(0xbfd00018)
	sb	$t9, LO(0xbfd00018)($t0)
	jal	F_0xa0004820
	 li	$a0, 0x20
	li	$t1, 3
	lui	$t2, HI(0xbfd00303)
	sb	$t1, LO(0xbfd00303)($t2)
	jal	F_0xa0004820
	 li	$a0, 0x20
	lui	$t3, HI(0xbfd00100)
	sb	$zero, LO(0xbfd00100)($t3)
	jal	F_0xa0004820
	 li	$a0, 0x20
	lui	$t4, HI(0xbfd00400)
	sb	$zero, LO(0xbfd00400)($t4)
	jal	F_0xa0004820
	 li	$a0, 0x20
	li	$t5, 0xde
	lui	$t6, HI(0xbfd007de)
	sb	$t5, LO(0xbfd007de)($t6)
	jal	F_0xa0004820
	 li	$a0, 0x20
	lw	$ra, 0x14($sp)
	addiu	$sp, $sp, 0x18
	jr	$ra
	 nop

/* Function F_0xa0004920 [0xa0004920 - 0xa0004974)
 */
F_0xa0004920: /* 0xa0004920 */
	lui	$v0, HI_UNSIGNED(0xbfd00503)
	ori	$v0, $v0, LO_UNSIGNED(0xbfd00503)
	lbu	$t6, 0($v0)
	lui	$a0, HI_UNSIGNED(0xbfd00003)
	li	$v1, -1
	dsll32	$t6, $t6, 0
	dsra32	$t6, $t6, 0
	andi	$t8, $t6, 1
	beqz	$t8, L_0xa000496c
	 ori	$a0, $a0, LO_UNSIGNED(0xbfd00003)
L_0xa0004948: /* 0xa0004948 */
	lbu	$t9, 0($a0)
	lbu	$t0, 0($v0)
	dsll32	$t9, $t9, 0
	dsll32	$t0, $t0, 0
	dsra32	$t0, $t0, 0
	andi	$t2, $t0, 1
	dsra32	$t9, $t9, 0
	bnez	$t2, L_0xa0004948
	 andi	$v1, $t9, 0xff
L_0xa000496c: /* 0xa000496c */
	jr	$ra
	 move	$v0, $v1

/* Function F_0xa0004974 [0xa0004974 - 0xa00049d0)
 */
F_0xa0004974: /* 0xa0004974 */
	lui	$v0, HI_UNSIGNED(0xbfd00503)
	ori	$v0, $v0, LO_UNSIGNED(0xbfd00503)
	lbu	$t6, 0($v0)
	lui	$t2, HI(0xbfd00003)
	dsll32	$t6, $t6, 0
	dsra32	$t6, $t6, 0
	andi	$t8, $t6, 1
	bnezl	$t8, L_0xa00049b4
	 lbu	$t3, LO(0xbfd00003)($t2)
	lbu	$t9, 0($v0)
L_0xa000499c: /* 0xa000499c */
	dsll32	$t9, $t9, 0
	dsra32	$t9, $t9, 0
	andi	$t1, $t9, 1
	beqzl	$t1, L_0xa000499c
	 lbu	$t9, 0($v0)
	lbu	$t3, LO(0xbfd00003)($t2)
L_0xa00049b4: /* 0xa00049b4 */
	dsll32	$t3, $t3, 0
	dsra32	$t3, $t3, 0
	andi	$t5, $t3, 0x7f
	dsll32	$t5, $t5, 0
	dsrl32	$t5, $t5, 0
	jr	$ra
	 andi	$v0, $t5, 0xff

/* Function F_0xa00049d0 [0xa00049d0 - 0xa0004a50)
 */
F_0xa00049d0: /* 0xa00049d0 */
	addiu	$sp, $sp, -0x18
	lui	$v0, HI_UNSIGNED(0xbfd00503)
	ori	$v0, $v0, LO_UNSIGNED(0xbfd00503)
	sw	$ra, 0x14($sp)
	sw	$a0, 0x18($sp)
	lbu	$t6, 0($v0)
	andi	$a1, $a0, 0xff
	dsll32	$a1, $a1, 0
	dsll32	$t6, $t6, 0
	dsra32	$t6, $t6, 0
	andi	$t8, $t6, 0x20
	bnez	$t8, L_0xa0004a1c
	 dsra32	$a1, $a1, 0
	lbu	$t9, 0($v0)
L_0xa0004a08: /* 0xa0004a08 */
	dsll32	$t9, $t9, 0
	dsra32	$t9, $t9, 0
	andi	$t1, $t9, 0x20
	beqzl	$t1, L_0xa0004a08
	 lbu	$t9, 0($v0)
L_0xa0004a1c: /* 0xa0004a1c */
	dsll32	$t2, $a1, 0
	dsrl32	$t2, $t2, 0
	lui	$t5, 0xbfd0
	or	$t4, $t2, $t5
	dsll32	$t4, $t4, 0
	dsra32	$t4, $t4, 0
	sb	$a1, 0($t4)
	jal	F_0xa0004820
	 li	$a0, 0x20
	lw	$ra, 0x14($sp)
	addiu	$sp, $sp, 0x18
	jr	$ra
	 nop

/* Function F_0xa0004a50 [0xa0004a50 - 0xa0004bb8)
 */
F_0xa0004a50: /* 0xa0004a50 */
	addiu	$sp, $sp, -0x68
	sw	$s0, 0x14($sp)
	sw	$a0, 0x68($sp)
	sw	$a1, 0x6c($sp)
	ld	$s0, 0x68($sp)
	sw	$s1, 0x18($sp)
	move	$s1, $a2
	sw	$ra, 0x1c($sp)
	bnez	$s0, L_0xa0004a90
	 sw	$a3, 0x74($sp)
	bnezl	$a2, L_0xa0004a94
	 li	$t6, 0x30
	jal	F_0xa00049d0
	 li	$a0, 0x30
	b	L_0xa0004ba8
	 lw	$ra, 0x1c($sp)
L_0xa0004a90: /* 0xa0004a90 */
	li	$t6, 0x30
L_0xa0004a94: /* 0xa0004a94 */
	li	$t7, 0x30
	sb	$t7, 0x51($sp)
	sb	$t6, 0x50($sp)
	addiu	$v1, $sp, 0x52
	addiu	$a0, $sp, 0x66
	li	$v0, 0x30
L_0xa0004aac: /* 0xa0004aac */
	addiu	$v1, $v1, 4
	sb	$v0, -4($v1)
	sb	$v0, -3($v1)
	sb	$v0, -2($v1)
	bne	$v1, $a0, L_0xa0004aac
	 sb	$v0, -1($v1)
	sltiu	$at, $s0, 0xb
	move	$a2, $zero
	bnez	$at, L_0xa0004b34
	 move	$a1, $s0
	lui	$t0, %hi(data_0xa0005b40)
	addiu	$t0, $t0, %lo(data_0xa0005b40)
	addiu	$s0, $sp, 0x50
	li	$a3, 0xa
L_0xa0004ae4: /* 0xa0004ae4 */
	li	$at, 0xa
	ddivu	$zero, $a1, $at
	addiu	$s0, $s0, 1
	mflo	$t8
	dsll32	$v0, $t8, 0
	dsra32	$v0, $v0, 0
	multu	$v0, $a3
	mflo	$t9
	dsubu	$t2, $a1, $t9
	dsll32	$v1, $t2, 0
	dsra32	$v1, $v1, 0
	addu	$t3, $t0, $v1
	lbu	$t4, 0($t3)
	move	$a1, $v0
	beqz	$v1, L_0xa0004b28
	 sb	$t4, -1($s0)
	sw	$a2, 0x3c($sp)
L_0xa0004b28: /* 0xa0004b28 */
	sltiu	$at, $a1, 0xb
	beqz	$at, L_0xa0004ae4
	 addiu	$a2, $a2, 1
L_0xa0004b34: /* 0xa0004b34 */
	lui	$t0, %hi(data_0xa0005b40)
	dsll32	$t5, $a1, 0
	addiu	$t0, $t0, %lo(data_0xa0005b40)
	dsrl32	$t5, $t5, 0
	addu	$t6, $t0, $t5
	lbu	$t7, 0($t6)
	addu	$t8, $sp, $a2
	beqz	$a1, L_0xa0004b5c
	 sb	$t7, 0x50($t8)
	sw	$a2, 0x3c($sp)
L_0xa0004b5c: /* 0xa0004b5c */
	beqz	$s1, L_0xa0004b74
	 lw	$t9, 0x74($sp)
	beqzl	$t9, L_0xa0004b78
	 lw	$a2, 0x3c($sp)
	b	L_0xa0004b78
	 move	$a2, $t9
L_0xa0004b74: /* 0xa0004b74 */
	lw	$a2, 0x3c($sp)
L_0xa0004b78: /* 0xa0004b78 */
	bltz	$a2, L_0xa0004ba4
	 addiu	$t1, $sp, 0x50
	addu	$s0, $a2, $t1
	addiu	$s1, $sp, 0x50
L_0xa0004b88: /* 0xa0004b88 */
	jal	F_0xa00049d0
	 lbu	$a0, 0($s0)
	li	$t2, -1
	addu	$s0, $s0, $t2
	sltu	$at, $s0, $s1
	beqz	$at, L_0xa0004b88
	 nop
L_0xa0004ba4: /* 0xa0004ba4 */
	lw	$ra, 0x1c($sp)
L_0xa0004ba8: /* 0xa0004ba8 */
	lw	$s0, 0x14($sp)
	lw	$s1, 0x18($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x68

/* Function F_0xa0004bb8 [0xa0004bb8 - 0xa0004d5c)
 */
F_0xa0004bb8: /* 0xa0004bb8 */
	addiu	$sp, $sp, -0x68
	sw	$s0, 0x14($sp)
	sw	$a0, 0x68($sp)
	sw	$a1, 0x6c($sp)
	ld	$s0, 0x68($sp)
	sw	$ra, 0x1c($sp)
	sw	$s1, 0x18($sp)
	sw	$a2, 0x70($sp)
	bnez	$s0, L_0xa0004bf8
	 sw	$a3, 0x74($sp)
	bnezl	$a2, L_0xa0004bfc
	 li	$t7, 0x30
	jal	F_0xa00049d0
	 li	$a0, 0x30
	b	L_0xa0004d4c
	 lw	$ra, 0x1c($sp)
L_0xa0004bf8: /* 0xa0004bf8 */
	li	$t7, 0x30
L_0xa0004bfc: /* 0xa0004bfc */
	li	$t8, 0x30
	sb	$t8, 0x51($sp)
	sb	$t7, 0x50($sp)
	addiu	$v1, $sp, 0x52
	addiu	$a0, $sp, 0x66
	li	$v0, 0x30
L_0xa0004c14: /* 0xa0004c14 */
	addiu	$v1, $v1, 4
	sb	$v0, -4($v1)
	sb	$v0, -3($v1)
	sb	$v0, -2($v1)
	bne	$v1, $a0, L_0xa0004c14
	 sb	$v0, -1($v1)
	bgez	$s0, L_0xa0004c40
	 move	$a2, $zero
	li	$s1, 0x2d
	b	L_0xa0004c48
	 dsubu	$a1, $zero, $s0
L_0xa0004c40: /* 0xa0004c40 */
	li	$s1, 0x20
	move	$a1, $s0
L_0xa0004c48: /* 0xa0004c48 */
	slti	$at, $a1, 0xb
	bnez	$at, L_0xa0004cb8
	 dsll32	$s1, $s1, 0
	addiu	$t9, $sp, 0x50
	lui	$t0, %hi(data_0xa0005b40)
	addiu	$t0, $t0, %lo(data_0xa0005b40)
	addu	$s0, $a2, $t9
	li	$a3, 0xa
L_0xa0004c68: /* 0xa0004c68 */
	li	$at, 0xa
	ddiv	$zero, $a1, $at
	addiu	$s0, $s0, 1
	mflo	$t1
	dsll32	$v0, $t1, 0
	dsra32	$v0, $v0, 0
	multu	$v0, $a3
	mflo	$t2
	dsubu	$t4, $a1, $t2
	dsll32	$v1, $t4, 0
	dsra32	$v1, $v1, 0
	addu	$t5, $t0, $v1
	lbu	$t6, 0($t5)
	move	$a1, $v0
	beqz	$v1, L_0xa0004cac
	 sb	$t6, -1($s0)
	sw	$a2, 0x3c($sp)
L_0xa0004cac: /* 0xa0004cac */
	slti	$at, $a1, 0xb
	beqz	$at, L_0xa0004c68
	 addiu	$a2, $a2, 1
L_0xa0004cb8: /* 0xa0004cb8 */
	lui	$t0, %hi(data_0xa0005b40)
	dsll32	$t7, $a1, 0
	addiu	$t0, $t0, %lo(data_0xa0005b40)
	dsrl32	$t7, $t7, 0
	addu	$t8, $t0, $t7
	lbu	$t9, 0($t8)
	addu	$t1, $sp, $a2
	beqz	$a1, L_0xa0004ce0
	 sb	$t9, 0x50($t1)
	sw	$a2, 0x3c($sp)
L_0xa0004ce0: /* 0xa0004ce0 */
	lw	$t2, 0x70($sp)
	lw	$t3, 0x74($sp)
	dsra32	$s1, $s1, 0
	beqz	$t2, L_0xa0004d04
	 li	$at, 0x2d
	beqzl	$t3, L_0xa0004d08
	 lw	$a2, 0x3c($sp)
	b	L_0xa0004d08
	 move	$a2, $t3
L_0xa0004d04: /* 0xa0004d04 */
	lw	$a2, 0x3c($sp)
L_0xa0004d08: /* 0xa0004d08 */
	bne	$s1, $at, L_0xa0004d1c
	 andi	$a0, $s1, 0xff
	jal	F_0xa00049d0
	 sw	$a2, 0x48($sp)
	lw	$a2, 0x48($sp)
L_0xa0004d1c: /* 0xa0004d1c */
	bltz	$a2, L_0xa0004d48
	 addiu	$t5, $sp, 0x50
	addu	$s0, $a2, $t5
	addiu	$s1, $sp, 0x50
L_0xa0004d2c: /* 0xa0004d2c */
	jal	F_0xa00049d0
	 lbu	$a0, 0($s0)
	li	$t6, -1
	addu	$s0, $s0, $t6
	sltu	$at, $s0, $s1
	beqz	$at, L_0xa0004d2c
	 nop
L_0xa0004d48: /* 0xa0004d48 */
	lw	$ra, 0x1c($sp)
L_0xa0004d4c: /* 0xa0004d4c */
	lw	$s0, 0x14($sp)
	lw	$s1, 0x18($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x68

/* Function F_0xa0004d5c [0xa0004d5c - 0xa0004e9c)
 */
F_0xa0004d5c: /* 0xa0004d5c */
	addiu	$sp, $sp, -0x38
	sw	$s3, 0x24($sp)
	sw	$a0, 0x38($sp)
	sw	$a1, 0x3c($sp)
	ld	$s3, 0x38($sp)
	sw	$s6, 0x30($sp)
	sw	$s1, 0x1c($sp)
	sw	$s0, 0x18($sp)
	move	$s6, $a2
	sw	$ra, 0x34($sp)
	sw	$s5, 0x2c($sp)
	sw	$s4, 0x28($sp)
	sw	$s2, 0x20($sp)
	move	$s0, $a3
	bnez	$s3, L_0xa0004db4
	 move	$s1, $zero
	bnez	$a2, L_0xa0004db4
	 nop
	jal	F_0xa00049d0
	 li	$a0, 0x30
	b	L_0xa0004e78
	 lw	$ra, 0x34($sp)
L_0xa0004db4: /* 0xa0004db4 */
	bltz	$a3, L_0xa0004e74
	 li	$s5, 0x30
	lui	$s2, %hi(data_0xa0005b40)
	addiu	$s2, $s2, %lo(data_0xa0005b40)
	li	$s4, 0x58
L_0xa0004dc8: /* 0xa0004dc8 */
	bnez	$s0, L_0xa0004de8
	 dsrlv	$t0, $s3, $s0
	andi	$t6, $s3, 0xf
	dsll32	$t7, $t6, 0
	dsrl32	$t7, $t7, 0
	addu	$t8, $s2, $t7
	b	L_0xa0004dfc
	 lbu	$a0, 0($t8)
L_0xa0004de8: /* 0xa0004de8 */
	andi	$t1, $t0, 0xf
	dsll32	$t2, $t1, 0
	dsrl32	$t2, $t2, 0
	addu	$t3, $s2, $t2
	lbu	$a0, 0($t3)
L_0xa0004dfc: /* 0xa0004dfc */
	dsll32	$a0, $a0, 0
	dsra32	$a0, $a0, 0
	bne	$s4, $a0, L_0xa0004e34
	 move	$v0, $a0
	slti	$at, $v0, 0x3a
	bnez	$at, L_0xa0004e34
	 li	$t5, -0x20
	addu	$t4, $v0, $t5
	dsll32	$t4, $t4, 0
	dsrl32	$t4, $t4, 0
	andi	$a0, $t4, 0xff
	dsll32	$a0, $a0, 0
	dsra32	$a0, $a0, 0
	move	$v0, $a0
L_0xa0004e34: /* 0xa0004e34 */
	beq	$s5, $v0, L_0xa0004e4c
	 nop
	jal	F_0xa00049d0
	 addiu	$s1, $s1, 1
	b	L_0xa0004e68
	 li	$t6, -4
L_0xa0004e4c: /* 0xa0004e4c */
	bnez	$s6, L_0xa0004e5c
	 nop
	beqzl	$s1, L_0xa0004e68
	 li	$t6, -4
L_0xa0004e5c: /* 0xa0004e5c */
	jal	F_0xa00049d0
	 nop
	li	$t6, -4
L_0xa0004e68: /* 0xa0004e68 */
	addu	$s0, $s0, $t6
	bgez	$s0, L_0xa0004dc8
	 nop
L_0xa0004e74: /* 0xa0004e74 */
	lw	$ra, 0x34($sp)
L_0xa0004e78: /* 0xa0004e78 */
	lw	$s0, 0x18($sp)
	lw	$s1, 0x1c($sp)
	lw	$s2, 0x20($sp)
	lw	$s3, 0x24($sp)
	lw	$s4, 0x28($sp)
	lw	$s5, 0x2c($sp)
	lw	$s6, 0x30($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x38

/* Function F_0xa0004e9c [0xa0004e9c - 0xa0004fa0)
 */
F_0xa0004e9c: /* 0xa0004e9c */
	addiu	$sp, $sp, -0x30
	sw	$s3, 0x20($sp)
	sw	$a0, 0x30($sp)
	sw	$a1, 0x34($sp)
	ld	$s3, 0x30($sp)
	sw	$s5, 0x28($sp)
	sw	$s1, 0x18($sp)
	sw	$s0, 0x14($sp)
	move	$s5, $a2
	sw	$ra, 0x2c($sp)
	sw	$s4, 0x24($sp)
	sw	$s2, 0x1c($sp)
	move	$s0, $a3
	bnez	$s3, L_0xa0004ef0
	 move	$s1, $zero
	bnez	$a2, L_0xa0004ef0
	 nop
	jal	F_0xa00049d0
	 li	$a0, 0x30
	b	L_0xa0004f80
	 lw	$ra, 0x2c($sp)
L_0xa0004ef0: /* 0xa0004ef0 */
	bltz	$a3, L_0xa0004f7c
	 li	$s4, 0x30
	lui	$s2, %hi(data_0xa0005b40)
	addiu	$s2, $s2, %lo(data_0xa0005b40)
L_0xa0004f00: /* 0xa0004f00 */
	bnez	$s0, L_0xa0004f20
	 dsrlv	$t0, $s3, $s0
	andi	$t6, $s3, 7
	dsll32	$t7, $t6, 0
	dsrl32	$t7, $t7, 0
	addu	$t8, $s2, $t7
	b	L_0xa0004f34
	 lbu	$a0, 0($t8)
L_0xa0004f20: /* 0xa0004f20 */
	andi	$t1, $t0, 7
	dsll32	$t2, $t1, 0
	dsrl32	$t2, $t2, 0
	addu	$t3, $s2, $t2
	lbu	$a0, 0($t3)
L_0xa0004f34: /* 0xa0004f34 */
	dsll32	$a0, $a0, 0
	dsra32	$a0, $a0, 0
	beq	$s4, $a0, L_0xa0004f54
	 nop
	jal	F_0xa00049d0
	 addiu	$s1, $s1, 1
	b	L_0xa0004f70
	 li	$t5, -3
L_0xa0004f54: /* 0xa0004f54 */
	bnez	$s5, L_0xa0004f64
	 nop
	beqzl	$s1, L_0xa0004f70
	 li	$t5, -3
L_0xa0004f64: /* 0xa0004f64 */
	jal	F_0xa00049d0
	 nop
	li	$t5, -3
L_0xa0004f70: /* 0xa0004f70 */
	addu	$s0, $s0, $t5
	bgez	$s0, L_0xa0004f00
	 nop
L_0xa0004f7c: /* 0xa0004f7c */
	lw	$ra, 0x2c($sp)
L_0xa0004f80: /* 0xa0004f80 */
	lw	$s0, 0x14($sp)
	lw	$s1, 0x18($sp)
	lw	$s2, 0x1c($sp)
	lw	$s3, 0x20($sp)
	lw	$s4, 0x24($sp)
	lw	$s5, 0x28($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x30

/* Function F_0xa0004fa0 [0xa0004fa0 - 0xa0005034)
 */
F_0xa0004fa0: /* 0xa0004fa0 */
	addiu	$sp, $sp, -0x28
	sw	$s0, 0x18($sp)
	sw	$ra, 0x24($sp)
	sw	$s2, 0x20($sp)
	sw	$s1, 0x1c($sp)
	sw	$a0, 0x28($sp)
	sw	$a1, 0x2c($sp)
	bltz	$a2, L_0xa000501c
	 move	$s0, $a2
	lui	$s2, %hi(data_0xa0005b40)
	addiu	$s2, $s2, %lo(data_0xa0005b40)
	ld	$s1, 0x28($sp)
L_0xa0004fd0: /* 0xa0004fd0 */
	bnez	$s0, L_0xa0004ff0
	 dsrlv	$t0, $s1, $s0
	andi	$t6, $s1, 1
	dsll32	$t7, $t6, 0
	dsrl32	$t7, $t7, 0
	addu	$t8, $s2, $t7
	b	L_0xa0005004
	 lbu	$a0, 0($t8)
L_0xa0004ff0: /* 0xa0004ff0 */
	andi	$t1, $t0, 1
	dsll32	$t2, $t1, 0
	dsrl32	$t2, $t2, 0
	addu	$t3, $s2, $t2
	lbu	$a0, 0($t3)
L_0xa0005004: /* 0xa0005004 */
	jal	F_0xa00049d0
	 nop
	li	$t4, -1
	addu	$s0, $s0, $t4
	bgez	$s0, L_0xa0004fd0
	 nop
L_0xa000501c: /* 0xa000501c */
	lw	$ra, 0x24($sp)
	lw	$s0, 0x18($sp)
	lw	$s1, 0x1c($sp)
	lw	$s2, 0x20($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x28

/* Function F_0xa0005034 [0xa0005034 - 0xa0005050)
 */
F_0xa0005034: /* 0xa0005034 */
	addiu	$sp, $sp, -8
	sw	$a0, 8($sp)
	sw	$a1, 0xc($sp)
	sw	$a2, 0x10($sp)
	sw	$a3, 0x14($sp)
	jr	$ra
	 addiu	$sp, $sp, 8

/* Function F_0xa0005050 [0xa0005050 - 0xa0005084)
 */
F_0xa0005050: /* 0xa0005050 */
	addiu	$sp, $sp, -0x18
	sw	$ra, 0x14($sp)
	lui	$a0, %hi(data_0xa0005b70)
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa0005b70)
	lui	$a0, %hi(data_0xa0005b98)
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa0005b98)
	lw	$ra, 0x14($sp)
	addiu	$sp, $sp, 0x18
	li	$v0, 1
	jr	$ra
	 nop

/* Function F_0xa0005084 [0xa0005084 - 0xa00050b8)
 */
F_0xa0005084: /* 0xa0005084 */
	addiu	$sp, $sp, -0x18
	sw	$ra, 0x14($sp)
	lui	$a0, %hi(data_0xa0005bbc)
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa0005bbc)
	lui	$a0, %hi(data_0xa0005be4)
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa0005be4)
	lw	$ra, 0x14($sp)
	addiu	$sp, $sp, 0x18
	li	$v0, 1
	jr	$ra
	 nop

/* Function F_0xa00050b8 [0xa00050b8 - 0xa00050ec)
 */
F_0xa00050b8: /* 0xa00050b8 */
	addiu	$sp, $sp, -0x18
	sw	$ra, 0x14($sp)
	lui	$a0, %hi(data_0xa0005c08)
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa0005c08)
	lui	$a0, %hi(data_0xa0005c30)
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa0005c30)
	lw	$ra, 0x14($sp)
	addiu	$sp, $sp, 0x18
	li	$v0, 1
	jr	$ra
	 nop

/* Function F_0xa00050ec [0xa00050ec - 0xa0005120)
 */
F_0xa00050ec: /* 0xa00050ec */
	addiu	$sp, $sp, -0x18
	sw	$ra, 0x14($sp)
	lui	$a0, %hi(data_0xa0005c54)
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa0005c54)
	lui	$a0, %hi(data_0xa0005c7c)
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa0005c7c)
	lw	$ra, 0x14($sp)
	addiu	$sp, $sp, 0x18
	li	$v0, 1
	jr	$ra
	 nop

/* Function F_0xa0005120 [0xa0005120 - 0xa0005154)
 */
F_0xa0005120: /* 0xa0005120 */
	addiu	$sp, $sp, -0x18
	sw	$ra, 0x14($sp)
	lui	$a0, %hi(data_0xa0005ca0)
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa0005ca0)
	lui	$a0, %hi(data_0xa0005cc8)
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa0005cc8)
	lw	$ra, 0x14($sp)
	addiu	$sp, $sp, 0x18
	li	$v0, 1
	jr	$ra
	 nop

/* Function F_0xa0005154 [0xa0005154 - 0xa0005188)
 */
F_0xa0005154: /* 0xa0005154 */
	addiu	$sp, $sp, -0x18
	sw	$ra, 0x14($sp)
	lui	$a0, %hi(data_0xa0005cec)
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa0005cec)
	lui	$a0, %hi(data_0xa0005d14)
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa0005d14)
	lw	$ra, 0x14($sp)
	addiu	$sp, $sp, 0x18
	li	$v0, 1
	jr	$ra
	 nop

/* Function F_0xa0005188 [0xa0005188 - 0xa00051bc)
 */
F_0xa0005188: /* 0xa0005188 */
	addiu	$sp, $sp, -0x18
	sw	$ra, 0x14($sp)
	lui	$a0, %hi(data_0xa0005d38)
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa0005d38)
	lui	$a0, %hi(data_0xa0005d60)
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa0005d60)
	lw	$ra, 0x14($sp)
	addiu	$sp, $sp, 0x18
	li	$v0, 1
	jr	$ra
	 nop

/* Function F_0xa00051bc [0xa00051bc - 0xa00051f0)
 */
F_0xa00051bc: /* 0xa00051bc */
	addiu	$sp, $sp, -0x18
	sw	$ra, 0x14($sp)
	lui	$a0, %hi(data_0xa0005d84)
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa0005d84)
	lui	$a0, %hi(data_0xa0005dac)
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa0005dac)
	lw	$ra, 0x14($sp)
	addiu	$sp, $sp, 0x18
	li	$v0, 1
	jr	$ra
	 nop

/* Function F_0xa00051f0 [0xa00051f0 - 0xa0005224)
 */
F_0xa00051f0: /* 0xa00051f0 */
	addiu	$sp, $sp, -0x18
	sw	$ra, 0x14($sp)
	lui	$a0, %hi(data_0xa0005dd0)
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa0005dd0)
	lui	$a0, %hi(data_0xa0005df8)
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa0005df8)
	lw	$ra, 0x14($sp)
	addiu	$sp, $sp, 0x18
	li	$v0, 1
	jr	$ra
	 nop

/* Function F_0xa0005224 [0xa0005224 - 0xa0005258)
 */
F_0xa0005224: /* 0xa0005224 */
	addiu	$sp, $sp, -0x18
	sw	$ra, 0x14($sp)
	lui	$a0, %hi(data_0xa0005e1c)
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa0005e1c)
	lui	$a0, %hi(data_0xa0005e44)
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa0005e44)
	lw	$ra, 0x14($sp)
	addiu	$sp, $sp, 0x18
	li	$v0, 1
	jr	$ra
	 nop

/* Function F_0xa0005258 [0xa0005258 - 0xa000528c)
 */
F_0xa0005258: /* 0xa0005258 */
	addiu	$sp, $sp, -0x18
	sw	$ra, 0x14($sp)
	lui	$a0, %hi(data_0xa0005e6c)
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa0005e6c)
	lui	$a0, %hi(data_0xa0005e94)
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa0005e94)
	lw	$ra, 0x14($sp)
	addiu	$sp, $sp, 0x18
	li	$v0, 1
	jr	$ra
	 nop

/* Function F_0xa000528c [0xa000528c - 0xa0005404)
 */
F_0xa000528c: /* 0xa000528c */
	addiu	$sp, $sp, -0x28
	sw	$ra, 0x24($sp)
	sw	$s3, 0x20($sp)
	sw	$s2, 0x1c($sp)
	sw	$s1, 0x18($sp)
	sw	$s0, 0x14($sp)
	move	$a0, $zero
	move	$a1, $zero
	jal	F_0xa00040b0
	 move	$a2, $zero
	lui	$t6, HI_UNSIGNED(BASE_CRIME | CRIME_MC_STATUS_CTRL)
	ori	$t6, $t6, LO_UNSIGNED(BASE_CRIME | CRIME_MC_STATUS_CTRL)
	lui	$t7, HI_UNSIGNED(BASE_CRIME | CRIME_INTMASK_OFFSET)
	sd	$zero, 0($t6)
	ori	$t7, $t7, LO_UNSIGNED(BASE_CRIME | CRIME_INTMASK_OFFSET)
	lui	$t8, HI_UNSIGNED(BASE_CRIME | CRIME_SOFT_INT_OFFSET)
	sd	$zero, 0($t7)
	ori	$t8, $t8, LO_UNSIGNED(BASE_CRIME | CRIME_SOFT_INT_OFFSET)
	lui	$t9, HI_UNSIGNED(BASE_CRIME | CRIME_HARD_INT_OFFSET)
	sd	$zero, 0($t8)
	ori	$t9, $t9, LO_UNSIGNED(BASE_CRIME | CRIME_HARD_INT_OFFSET)
	lui	$t0, HI_UNSIGNED(BASE_CRIME | CRIME_CPU_ERROR_ENA)
	sd	$zero, 0($t9)
	ori	$t0, $t0, LO_UNSIGNED(BASE_CRIME | CRIME_CPU_ERROR_ENA)
	sd	$zero, 0($t0)
	move	$s2, $zero
	li	$s0, 0x15
	jal	F_0xa0004080
	 move	$a0, $zero
	move	$s3, $v0
	jal	F_0xa0004200
	 move	$a0, $zero
	blez	$s3, L_0xa000535c
	 move	$s1, $zero
L_0xa0005314: /* 0xa0005314 */
	dsll32	$s1, $s1, 0
	dsrl32	$s1, $s1, 0
	jal	F_0xa00041b0
	 move	$a0, $s1
	jal	F_0xa00041f0
	 move	$a0, $s2
	jal	F_0xa00041d0
	 move	$a0, $s0
	addiu	$s0, $s0, 0x40
	jal	F_0xa00041e0
	 move	$a0, $s0
	jal	F_0xa0004220
	 nop
	addiu	$s1, $s1, 1
	addiu	$s2, $s2, 0x2000
	bne	$s1, $s3, L_0xa0005314
	 addiu	$s0, $s0, 0x40
	move	$s1, $zero
L_0xa000535c: /* 0xa000535c */
	jal	F_0xa000413c
	 move	$a0, $zero
	dsll32	$v0, $v0, 0
	dsra32	$s3, $v0, 0
	blez	$s3, L_0xa00053ac
	 lui	$s2, HI(KSEG0)
	dsll32	$s1, $s1, 0
L_0xa0005378: /* 0xa0005378 */
	dsrl32	$s1, $s1, 0
	or	$s0, $s1, $s2
	jal	F_0xa0004290
	 move	$a0, $s0
	srl	$a0, $s0, 0xc
	jal	F_0xa0004228
	 sll	$a0, $a0, 8
	jal	F_0xa0004278
	 move	$a0, $s0
	addiu	$s1, $s1, 0x20
	slt	$at, $s1, $s3
	bnezl	$at, L_0xa0005378
	 dsll32	$s1, $s1, 0
L_0xa00053ac: /* 0xa00053ac */
	jal	F_0xa0004168
	 nop
	sll	$t2, $v0, 0xe
	bgez	$t2, L_0xa00053dc
	 move	$s0, $v0
	jal	F_0xa000415c
	 ori	$a0, $v0, 0x1000
	jal	F_0xa0004298
	 move	$a0, $zero
	li	$t3, -0x1001
	jal	F_0xa000415c
	 and	$a0, $s0, $t3
L_0xa00053dc: /* 0xa00053dc */
	lui	$a0, %hi(data_0xa0005ebc)
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa0005ebc)
	lw	$ra, 0x24($sp)
	lw	$s0, 0x14($sp)
	lw	$s1, 0x18($sp)
	lw	$s2, 0x1c($sp)
	lw	$s3, 0x20($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x28

/* Function F_0xa0005404 [0xa0005404 - 0xa000542c)
 */
F_0xa0005404: /* 0xa0005404 */
	addiu	$sp, $sp, -0x18
	sw	$ra, 0x14($sp)
	lui	$a0, %hi(data_0xa0005ef4)
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa0005ef4)
	lw	$ra, 0x14($sp)
	addiu	$sp, $sp, 0x18
	li	$v0, -1
	jr	$ra
	 nop

/* Function F_0xa000542c [0xa000542c - 0xa00054bc)
 */
F_0xa000542c: /* 0xa000542c */
	addiu	$sp, $sp, -0x28
	sw	$ra, 0x24($sp)
	sw	$s1, 0x18($sp)
	lui	$a0, %hi(data_0xa0005f24)
	sw	$s3, 0x20($sp)
	sw	$s2, 0x1c($sp)
	sw	$s0, 0x14($sp)
	move	$s1, $zero
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa0005f24)
	lui	$s0, %hi(F_0xa0005b60)
	lui	$s2, %hi(data_0xa0005f44)
	addiu	$s2, $s2, %lo(data_0xa0005f44)
	addiu	$s0, $s0, %lo(F_0xa0005b60)
	li	$s3, -1
L_0xa0005468: /* 0xa0005468 */
	jal	F_0xa000528c
	 nop
	move	$a0, $s2
	jal	F_0xa0005034
	 move	$a1, $s1
	lw	$t9, 0($s0)
	jalr	$t9
	 nop
	addiu	$s1, $s1, 1
	bne	$v0, $s3, L_0xa0005468
	 addiu	$s0, $s0, 4
	jal	F_0xa000528c
	 nop
	lw	$ra, 0x24($sp)
	lw	$s0, 0x14($sp)
	lw	$s1, 0x18($sp)
	lw	$s2, 0x1c($sp)
	lw	$s3, 0x20($sp)
	addiu	$sp, $sp, 0x28
	jr	$ra
	 li	$v0, 1

/* Function F_0xa00054bc [0xa00054bc - 0xa00055ec)
 */
F_0xa00054bc: /* 0xa00054bc */
	addiu	$sp, $sp, -0x30
	sw	$s1, 0x18($sp)
	move	$s1, $a0
	sw	$ra, 0x2c($sp)
	sw	$s5, 0x28($sp)
	sw	$s2, 0x1c($sp)
	lui	$a0, %hi(data_0xa0005f64)
	move	$s2, $a2
	move	$s5, $a1
	sw	$s4, 0x24($sp)
	sw	$s3, 0x20($sp)
	sw	$s0, 0x14($sp)
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa0005f64)
	move	$s0, $s1
	blez	$s5, L_0xa00055bc
	 move	$s4, $zero
	lui	$s1, HI(KSEG0)
L_0xa0005504: /* 0xa0005504 */
	srl	$a0, $s0, 0xc
	jal	F_0xa0004228
	 sll	$a0, $a0, 8
	or	$s3, $s1, $s0
	jal	F_0xa0004278
	 move	$a0, $s3
	jal	F_0xa0004280
	 move	$a0, $s3
	xor	$v1, $s0, $s2
	not	$t6, $v1
	or	$t3, $s1, $s0
	dsll32	$t3, $t3, 0
	dsll32	$t8, $t6, 0
	ori	$a0, $s0, 8
	or	$t2, $t8, $v1
	dsra32	$t3, $t3, 0
	xor	$a1, $a0, $s2
	sd	$t2, 0($t3)
	or	$t3, $s1, $a0
	not	$t5, $a1
	dsll32	$t7, $t5, 0
	dsll32	$t3, $t3, 0
	dsra32	$t3, $t3, 0
	or	$t9, $t7, $a1
	ori	$a2, $s0, 0x10
	xor	$a3, $a2, $s2
	sd	$t9, 0($t3)
	or	$t3, $s1, $a2
	not	$t4, $a3
	dsll32	$t6, $t4, 0
	dsll32	$t3, $t3, 0
	dsra32	$t3, $t3, 0
	or	$t8, $t6, $a3
	ori	$t0, $s0, 0x18
	xor	$t1, $t0, $s2
	sd	$t8, 0($t3)
	or	$t3, $s1, $t0
	not	$t2, $t1
	dsll32	$t5, $t2, 0
	dsll32	$t3, $t3, 0
	addiu	$s4, $s4, 1
	dsra32	$t3, $t3, 0
	or	$t7, $t5, $t1
	addiu	$s0, $s0, 0x20
	bne	$s4, $s5, L_0xa0005504
	 sd	$t7, 0($t3)
L_0xa00055bc: /* 0xa00055bc */
	lui	$a0, %hi(data_0xa0005f8c)
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa0005f8c)
	lw	$ra, 0x2c($sp)
	lw	$s0, 0x14($sp)
	lw	$s1, 0x18($sp)
	lw	$s2, 0x1c($sp)
	lw	$s3, 0x20($sp)
	lw	$s4, 0x24($sp)
	lw	$s5, 0x28($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x30

/* Function F_0xa00055ec [0xa00055ec - 0xa0005678)
 */
F_0xa00055ec: /* 0xa00055ec */
	addiu	$sp, $sp, -0x28
	sw	$s2, 0x1c($sp)
	move	$s2, $a0
	sw	$ra, 0x24($sp)
	sw	$s3, 0x20($sp)
	sw	$s0, 0x14($sp)
	lui	$a0, %hi(data_0xa0005fb4)
	move	$s0, $a2
	move	$s3, $a1
	sw	$s1, 0x18($sp)
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa0005fb4)
	move	$a0, $s2
	move	$a1, $s3
	jal	F_0xa00054bc
	 move	$a2, $s0
	move	$s0, $s2
	blez	$s3, L_0xa0005650
	 move	$s1, $zero
	lui	$s2, HI(KSEG0)
L_0xa000563c: /* 0xa000563c */
	jal	F_0xa0004250
	 or	$a0, $s2, $s0
	addiu	$s1, $s1, 1
	bne	$s1, $s3, L_0xa000563c
	 addiu	$s0, $s0, 0x20
L_0xa0005650: /* 0xa0005650 */
	lui	$a0, %hi(data_0xa0005fdc)
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa0005fdc)
	lw	$ra, 0x24($sp)
	lw	$s0, 0x14($sp)
	lw	$s1, 0x18($sp)
	lw	$s2, 0x1c($sp)
	lw	$s3, 0x20($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x28

/* Function F_0xa0005678 [0xa0005678 - 0xa0005794)
 */
F_0xa0005678: /* 0xa0005678 */
	addiu	$sp, $sp, -0x88
	move	$a3, $a0
	sw	$ra, 0x1c($sp)
	sw	$s0, 0x18($sp)
	lui	$a0, %hi(data_0xa0006004)
	move	$s0, $a1
	sw	$a2, 0x90($sp)
	addiu	$a0, $a0, %lo(data_0xa0006004)
	jal	F_0xa0005034
	 sw	$a3, 0x88($sp)
	lwu	$a3, 0x88($sp)
	blez	$s0, L_0xa0005774
	 move	$t3, $zero
	lwu	$t8, 0x90($sp)
	lui	$t7, HI(KSEG0)
	or	$t6, $a3, $t7
	dsll32	$t6, $t6, 0
	xor	$t5, $a3, $t8
	not	$t9, $t5
	dsra32	$v0, $t6, 0
	dsll32	$t6, $t9, 0
	or	$v1, $t6, $t5
	ori	$ra, $a3, 8
	lui	$t6, HI(KSEG0)
	xor	$t9, $ra, $t8
	sw	$t9, 0x4c($sp)
	or	$t7, $ra, $t6
	lwu	$t6, 0x4c($sp)
	dsll32	$t7, $t7, 0
	dsra32	$a0, $t7, 0
	not	$t7, $t6
	dsll32	$t7, $t7, 0
	or	$a1, $t7, $t6
	ori	$t6, $a3, 0x10
	sw	$t6, 0x3c($sp)
	lwu	$t7, 0x3c($sp)
	lui	$t6, HI(KSEG0)
	or	$t9, $t7, $t6
	dsll32	$t9, $t9, 0
	dsra32	$a2, $t9, 0
	xor	$t6, $t7, $t8
	not	$t9, $t6
	dsll32	$t9, $t9, 0
	or	$t0, $t9, $t6
	sw	$t6, 0x34($sp)
	ori	$t6, $a3, 0x18
	sw	$t6, 0x24($sp)
	lwu	$t9, 0x24($sp)
	xor	$t7, $t9, $t8
	not	$t6, $t7
	dsll32	$t6, $t6, 0
	or	$t1, $t6, $t7
	lui	$t6, HI(KSEG0)
	sw	$t7, 0x20($sp)
	or	$t7, $t9, $t6
	dsll32	$t7, $t7, 0
	dsra32	$t2, $t7, 0
L_0xa000575c: /* 0xa000575c */
	sd	$v1, 0($v0)
	sd	$a1, 0($a0)
	addiu	$t3, $t3, 1
	sd	$t0, 0($a2)
	bne	$t3, $s0, L_0xa000575c
	 sd	$t1, 0($t2)
L_0xa0005774: /* 0xa0005774 */
	lui	$a0, %hi(data_0xa0006028)
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa0006028)
	lw	$ra, 0x1c($sp)
	lw	$s0, 0x18($sp)
	addiu	$sp, $sp, 0x88
	jr	$ra
	 nop

/* Function F_0xa0005794 [0xa0005794 - 0xa00058b8)
 */
F_0xa0005794: /* 0xa0005794 */
	addiu	$sp, $sp, -0x40
	sw	$s3, 0x24($sp)
	move	$s3, $a0
	sw	$ra, 0x3c($sp)
	sw	$fp, 0x38($sp)
	sw	$s4, 0x28($sp)
	lui	$a0, %hi(data_0xa0006050)
	move	$s4, $a2
	move	$fp, $a1
	sw	$s7, 0x34($sp)
	sw	$s6, 0x30($sp)
	sw	$s5, 0x2c($sp)
	sw	$s2, 0x20($sp)
	sw	$s1, 0x1c($sp)
	sw	$s0, 0x18($sp)
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa0006050)
	blez	$fp, L_0xa000587c
	 move	$s5, $zero
	srl	$s6, $s3, 0xc
	sll	$s6, $s6, 8
	lui	$t6, HI(KSEG0)
	or	$s7, $s3, $t6
	ori	$s6, $s6, R5K_TAGLO_DIRTY
	li	$s2, 0x20
	lui	$s1, HI(KSEG0)
L_0xa00057fc: /* 0xa00057fc */
	jal	F_0xa0004270
	 move	$a0, $s7
	jal	F_0xa0004238
	 nop
	srl	$t7, $v0, 5
	sll	$t8, $t7, 5
	beql	$s6, $t8, L_0xa0005828
	 move	$s0, $zero
	jal	F_0xa0004104
	 nop
	move	$s0, $zero
L_0xa0005828: /* 0xa0005828 */
	dsll32	$s0, $s0, 0
L_0xa000582c: /* 0xa000582c */
	dsrl32	$s0, $s0, 0
	or	$v0, $s0, $s3
	or	$t0, $s1, $v0
	dsll32	$t0, $t0, 0
	dsra32	$t0, $t0, 0
	xor	$v1, $v0, $s4
	ld	$a0, 0($t0)
	not	$t2, $v1
	dsll32	$t4, $t2, 0
	or	$t6, $t4, $v1
	beql	$a0, $t6, L_0xa0005868
	 addiu	$s0, $s0, 8
	jal	F_0xa0004104
	 nop
	addiu	$s0, $s0, 8
L_0xa0005868: /* 0xa0005868 */
	bnel	$s0, $s2, L_0xa000582c
	 dsll32	$s0, $s0, 0
	addiu	$s5, $s5, 1
	bne	$s5, $fp, L_0xa00057fc
	 nop
L_0xa000587c: /* 0xa000587c */
	lui	$a0, %hi(data_0xa000607c)
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa000607c)
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

/* Function F_0xa00058b8 [0xa00058b8 - 0xa00059f4)
 */
F_0xa00058b8: /* 0xa00058b8 */
	addiu	$sp, $sp, -0x78
	sw	$s1, 0x1c($sp)
	move	$s1, $a0
	sw	$ra, 0x3c($sp)
	sw	$s7, 0x34($sp)
	sw	$s2, 0x20($sp)
	lui	$a0, %hi(data_0xa00060a8)
	move	$s2, $a2
	move	$s7, $a1
	sw	$fp, 0x38($sp)
	sw	$s6, 0x30($sp)
	sw	$s5, 0x2c($sp)
	sw	$s4, 0x28($sp)
	sw	$s3, 0x24($sp)
	sw	$s0, 0x18($sp)
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa00060a8)
	blez	$s7, L_0xa00059b8
	 move	$s5, $zero
	srl	$t7, $s1, 0x13
	sll	$t8, $t7, 0xf
	ori	$t9, $t8, 0x1000
	lui	$t6, HI(KSEG0)
	srl	$fp, $s1, 0xc
	sll	$fp, $fp, 8
	or	$s6, $s1, $t6
	sw	$t9, 0x44($sp)
	li	$s4, 0x20
	lui	$s3, HI(KSEG0)
L_0xa000592c: /* 0xa000592c */
	jal	F_0xa00042ac
	 move	$a0, $s6
	jal	F_0xa0004238
	 nop
	lwu	$t0, 0x44($sp)
	beq	$v0, $t0, L_0xa0005950
	 nop
	jal	F_0xa0004104
	 nop
L_0xa0005950: /* 0xa0005950 */
	jal	F_0xa0004228
	 move	$a0, $fp
	jal	F_0xa0004278
	 move	$a0, $s6
	move	$s0, $zero
	dsll32	$s0, $s0, 0
L_0xa0005968: /* 0xa0005968 */
	dsrl32	$s0, $s0, 0
	or	$v0, $s0, $s1
	or	$t2, $s3, $v0
	dsll32	$t2, $t2, 0
	dsra32	$t2, $t2, 0
	xor	$v1, $v0, $s2
	ld	$a0, 0($t2)
	not	$t4, $v1
	dsll32	$t6, $t4, 0
	or	$t8, $t6, $v1
	beql	$a0, $t8, L_0xa00059a4
	 addiu	$s0, $s0, 8
	jal	F_0xa0004104
	 nop
	addiu	$s0, $s0, 8
L_0xa00059a4: /* 0xa00059a4 */
	bnel	$s0, $s4, L_0xa0005968
	 dsll32	$s0, $s0, 0
	addiu	$s5, $s5, 1
	bne	$s5, $s7, L_0xa000592c
	 nop
L_0xa00059b8: /* 0xa00059b8 */
	lui	$a0, %hi(data_0xa00060d4)
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa00060d4)
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
	 addiu	$sp, $sp, 0x78

/* Function F_0xa00059f4 [0xa00059f4 - 0xa0005acc)
 */
F_0xa00059f4: /* 0xa00059f4 */
	addiu	$sp, $sp, -0x38
	sw	$s3, 0x24($sp)
	move	$s3, $a0
	sw	$ra, 0x34($sp)
	sw	$s6, 0x30($sp)
	sw	$s4, 0x28($sp)
	lui	$a0, %hi(data_0xa0006100)
	move	$s4, $a2
	move	$s6, $a1
	sw	$s5, 0x2c($sp)
	sw	$s2, 0x20($sp)
	sw	$s1, 0x1c($sp)
	sw	$s0, 0x18($sp)
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa0006100)
	blez	$s6, L_0xa0005a98
	 move	$s5, $zero
	li	$s2, 0x20
	lui	$s1, HI(KSEG0)
	move	$s0, $zero
L_0xa0005a44: /* 0xa0005a44 */
	dsll32	$s0, $s0, 0
L_0xa0005a48: /* 0xa0005a48 */
	dsrl32	$s0, $s0, 0
	or	$v0, $s0, $s3
	or	$t7, $s1, $v0
	dsll32	$t7, $t7, 0
	dsra32	$t7, $t7, 0
	xor	$v1, $v0, $s4
	ld	$a0, 0($t7)
	not	$t9, $v1
	dsll32	$t1, $t9, 0
	or	$t3, $t1, $v1
	beql	$a0, $t3, L_0xa0005a84
	 addiu	$s0, $s0, 8
	jal	F_0xa0004104
	 nop
	addiu	$s0, $s0, 8
L_0xa0005a84: /* 0xa0005a84 */
	bnel	$s0, $s2, L_0xa0005a48
	 dsll32	$s0, $s0, 0
	addiu	$s5, $s5, 1
	bnel	$s5, $s6, L_0xa0005a44
	 move	$s0, $zero
L_0xa0005a98: /* 0xa0005a98 */
	lui	$a0, %hi(data_0xa0006128)
	jal	F_0xa0005034
	 addiu	$a0, $a0, %lo(data_0xa0006128)
	lw	$ra, 0x34($sp)
	lw	$s0, 0x18($sp)
	lw	$s1, 0x1c($sp)
	lw	$s2, 0x20($sp)
	lw	$s3, 0x24($sp)
	lw	$s4, 0x28($sp)
	lw	$s5, 0x2c($sp)
	lw	$s6, 0x30($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x38
	.int	0x00000000
	.int	0x01200000
	.int	0x00000000
	.int	0xe3fffffe
	.fill	4, 4, 0x00000000
	.int	0x00007ff0
	.int	0x02080000
	.int	0x00001f00
	.int	0x06300000
	.fill	11, 4, 0x00000000
	.int	0xe3fffffe
	.fill	4, 4, 0x00000000
	.int	0x00007ff0
data_0xa0005b40: /* 0xa0005b40 */
	string 4, "0123456789abcdef"
	.fill	3, 4, 0x00000000
F_0xa0005b60: /* 0xa0005b60 */
	.int	F_0xa00042c0
	.int	F_0xa0005404
	.fill	2, 4, 0x00000000
data_0xa0005b70: /* 0xa0005b70 */
	string 4, "<post1> <mem_scache1> Entering test\n"
data_0xa0005b98: /* 0xa0005b98 */
	string 4, "<post1> <mem_scache1> Leaving test\n"
data_0xa0005bbc: /* 0xa0005bbc */
	string 4, "<post1> <mem_scache2> Entering test\n"
data_0xa0005be4: /* 0xa0005be4 */
	string 4, "<post1> <mem_scache2> Leaving test\n"
data_0xa0005c08: /* 0xa0005c08 */
	string 4, "<post1> <mem_scache3> Entering test\n"
data_0xa0005c30: /* 0xa0005c30 */
	string 4, "<post1> <mem_scache3> Leaving test\n"
data_0xa0005c54: /* 0xa0005c54 */
	string 4, "<post1> <mem_scache4> Entering test\n"
data_0xa0005c7c: /* 0xa0005c7c */
	string 4, "<post1> <mem_scache4> Leaving test\n"
data_0xa0005ca0: /* 0xa0005ca0 */
	string 4, "<post1> <mem_scache5> Entering test\n"
data_0xa0005cc8: /* 0xa0005cc8 */
	string 4, "<post1> <mem_scache5> Leaving test\n"
data_0xa0005cec: /* 0xa0005cec */
	string 4, "<post1> <mem_scache6> Entering test\n"
data_0xa0005d14: /* 0xa0005d14 */
	string 4, "<post1> <mem_scache6> Leaving test\n"
data_0xa0005d38: /* 0xa0005d38 */
	string 4, "<post1> <mem_scache7> Entering test\n"
data_0xa0005d60: /* 0xa0005d60 */
	string 4, "<post1> <mem_scache7> Leaving test\n"
data_0xa0005d84: /* 0xa0005d84 */
	string 4, "<post1> <mem_scache8> Entering test\n"
data_0xa0005dac: /* 0xa0005dac */
	string 4, "<post1> <mem_scache8> Leaving test\n"
data_0xa0005dd0: /* 0xa0005dd0 */
	string 4, "<post1> <mem_scache9> Entering test\n"
data_0xa0005df8: /* 0xa0005df8 */
	string 4, "<post1> <mem_scache9> Leaving test\n"
data_0xa0005e1c: /* 0xa0005e1c */
	string 4, "<post1> <mem_scache91> Entering test\n"
data_0xa0005e44: /* 0xa0005e44 */
	string 4, "<post1> <mem_scache91> Leaving test\n"
data_0xa0005e6c: /* 0xa0005e6c */
	string 4, "<post1> <mem_scache10> Entering test\n"
data_0xa0005e94: /* 0xa0005e94 */
	string 4, "<post1> <mem_scache10> Leaving test\n"
data_0xa0005ebc: /* 0xa0005ebc */
	string 4, "<post1> <setUPmh> setting up diags exec environment\n"
data_0xa0005ef4: /* 0xa0005ef4 */
	string 4, "<post1> <endOFdiags> This is the end of post1\n"
data_0xa0005f24: /* 0xa0005f24 */
	string 4, "<post1> <post1diags> .......\n"
data_0xa0005f44: /* 0xa0005f44 */
	string 4, "<post1> <post1diags> diags%d\n"
data_0xa0005f64: /* 0xa0005f64 */
	string 4, "<post1> <pcache_init> Enter routine.\n"
data_0xa0005f8c: /* 0xa0005f8c */
	string 4, "<post1> <pcache_init> Leaving routine.\n"
data_0xa0005fb4: /* 0xa0005fb4 */
	string 4, "<post1> <scache_init> Enter routine.\n"
data_0xa0005fdc: /* 0xa0005fdc */
	string 4, "<post1> <scache_init> Leaving routine.\n"
data_0xa0006004: /* 0xa0006004 */
	string 4, "<post1> <mem_init> Enter routine.\n"
data_0xa0006028: /* 0xa0006028 */
	string 4, "<post1> <mem_init> Leaving routine.\n"
data_0xa0006050: /* 0xa0006050 */
	string 4, "<post1> <pcache_verify> Entering routine.\n"
data_0xa000607c: /* 0xa000607c */
	string 4, "<post1> <pcache_verify> Leaving routine.\n"
data_0xa00060a8: /* 0xa00060a8 */
	string 4, "<post1> <scache_verify> Entering routine.\n"
data_0xa00060d4: /* 0xa00060d4 */
	string 4, "<post1> <scache_verify> Leaving routine.\n"
data_0xa0006100: /* 0xa0006100 */
	string 4, "<post1> <mem_verify> Entering routine.\n"
data_0xa0006128: /* 0xa0006128 */
	string 4, "<post1> <mem_verify> Leaving routine.\n"

.section .text_after_1, "ax"

data_0xbfc09120: /* 0xbfc09120 */
	.int	0x00000019
	.int	0xdeadbeef
	.fill	2, 4, 0x00000000
data_0xbfc09130: /* 0xbfc09130 */
	.int	BASE_RENDER
	.fill	3, 4, 0x00000000

	section_checksum
	section_end post1
