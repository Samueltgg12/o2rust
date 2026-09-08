/* Section: post1 */
/* Version: 1.0 */
/* Type: SECTION_TYPE_CODE */
/* Length: 0x4c24 (19492 bytes) */

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
	jal	F_0xbfc05ab4
	 nop
	lui	$a0, 0x4000
	jal	F_0xbfc061d0
	 li	$a1, 0x8000
	jal	F_0xbfc06080
	 nop
	lui	$t0, HI(POST1_STACK)
	sd	$sp, LO(POST1_STACK + 8)($t0)
	lui	$t1, HI(KSEG1)
	addiu	$t0, $t0, LO(POST1_STACK)
	jal	F_0xbfc06a00
	 or	$sp, $sp, $t1
	lui	$a0, %hi(data_0xbfc06eb0)
	lui	$a1, %hi(data_0xbfc09000)
	lui	$a2, HI(0xa0004000)
	addiu	$a0, $a0, %lo(data_0xbfc06eb0)		# 0xbfc06fd0, start of loadable subsection
	addiu	$a1, $a1, %lo(data_0xbfc09000)		# 0xbfc09120, end of loadable subsection
	addiu	$a2, $a2, LO(0xa0004000)		# 0xa0004000, address of destination
	jal	F_0xbfc060d8
	 li	$a3, 0
	lui	$a0, HI(0xa0004000)
	addiu	$a0, $a0, LO(0xa0004000)
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
	jal	F_0xbfc06a00
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
	lui	$t0, %hi(data_0xbfc06e90)
	addiu	$t0, $t0, %lo(data_0xbfc06e90)
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
	lui	$t0, %hi(data_0xbfc06e90)
	dsll32	$t5, $a1, 0
	addiu	$t0, $t0, %lo(data_0xbfc06e90)
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
	lui	$t0, %hi(data_0xbfc06e90)
	addiu	$t0, $t0, %lo(data_0xbfc06e90)
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
	lui	$t0, %hi(data_0xbfc06e90)
	dsll32	$t7, $a1, 0
	addiu	$t0, $t0, %lo(data_0xbfc06e90)
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
	lui	$s2, %hi(data_0xbfc06e90)
	addiu	$s2, $s2, %lo(data_0xbfc06e90)
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
	lui	$s2, %hi(data_0xbfc06e90)
	addiu	$s2, $s2, %lo(data_0xbfc06e90)
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
	lui	$s2, %hi(data_0xbfc06e90)
	addiu	$s2, $s2, %lo(data_0xbfc06e90)
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

/* Function F_0xbfc050bc [0xbfc050bc - 0xbfc050f4)
 */
F_0xbfc050bc: /* 0xbfc050bc */
	addiu	$sp, $sp, -0x20
	sd	$at, 8($sp)
	sd	$t0, 0x10($sp)
	mfc0	$at, $CP0_PRID
	li	$t0, 9
	andi	$at, $at, PRID_IMP_MASK
	srl	$at, $at, PRID_IMP_SHIFT
	bne	$t0, $at, L_0xbfc050e4
	 cache	(CACHE_TYPE_L1D|HIT_WRITEBACK_INV), 0($a0)
	cache	(CACHE_TYPE_L2|HIT_WRITEBACK_INV), 0($a0)
L_0xbfc050e4: /* 0xbfc050e4 */
	ld	$at, 8($sp)
	ld	$t0, 0x10($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x20

/* Function F_0xbfc050f4 [0xbfc050f4 - 0xbfc05170)
 */
F_0xbfc050f4: /* 0xbfc050f4 */
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
	jal	F_0xbfc05170
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

/* Function F_0xbfc05170 [0xbfc05170 - 0xbfc05348)
 */
F_0xbfc05170: /* 0xbfc05170 */
	lui	$t0, HI(KSEG1)
	addu	$t0, $t0, $a0
	addu	$t1, $t0, $a1
	lui	$t2, HI(BASE_CRIME + CRIME_ID_OFFSET)
	addiu	$t2, $t2, LO(BASE_CRIME + CRIME_ID_OFFSET)
	li	$a0, 0x20
	addi	$a0, $a0, -8
L_0xbfc0518c: /* 0xbfc0518c */
	bltz	$a0, L_0xbfc051c0
	 addu	$a1, $t0, $a0
	li	$t3, 1
L_0xbfc05198: /* 0xbfc05198 */
	sd	$t3, 0($a1)
	ld	$t5, LO(BASE_CRIME + CRIME_ID_OFFSET)($t2)
	ld	$t4, 0($a1)
	li	$t5, 1
	bne	$t3, $t4, L_0xbfc052fc
	 dsll	$t3, $t3, 1
	bnez	$t3, L_0xbfc05198
	 nop
	b	L_0xbfc0518c
	 addi	$a0, $a0, -8
L_0xbfc051c0: /* 0xbfc051c0 */
	li	$a0, 0x20
	addi	$a0, $a0, -8
L_0xbfc051c8: /* 0xbfc051c8 */
	bltz	$a0, L_0xbfc05204
	 addu	$a1, $t0, $a0
	li	$t3, 1
	not	$t3, $t3
L_0xbfc051d8: /* 0xbfc051d8 */
	sd	$t3, 0($a1)
	ld	$t5, LO(BASE_CRIME + CRIME_ID_OFFSET)($t2)
	ld	$t4, 0($a1)
	li	$t5, 2
	bne	$t3, $t4, L_0xbfc052fc
	 not	$t6, $t3
	dsll	$t6, $t6, 1
	bnez	$t6, L_0xbfc051d8
	 not	$t3, $t6
	b	L_0xbfc051c8
	 addi	$a0, $a0, -8
L_0xbfc05204: /* 0xbfc05204 */
	nop
	lui	$t3, HI_UNSIGNED(0x5a5a5a5a)
	ori	$t3, $t3, LO_UNSIGNED(0x5a5a5a5a)
	move	$a0, $t0
	addiu	$a1, $a0, 4
L_0xbfc05218: /* 0xbfc05218 */
	sw	$t3, LO(KSEG1)($a0)
	not	$t3, $t3
	sw	$t3, 0($a1)
	lwu	$t4, LO(KSEG1)($a0)
	not	$t3, $t3
	bne	$t3, $t4, L_0xbfc052fc
	 li	$t5, 3
	addiu	$a1, $a1, 4
	beq	$a1, $t1, L_0xbfc05250
	 addiu	$a0, $a0, LO(0xa0000004)
	bne	$a0, $t1, L_0xbfc05218
	 nop
	b	L_0xbfc05258
	 nop
L_0xbfc05250: /* 0xbfc05250 */
	b	L_0xbfc05218
	 move	$a1, $t0
L_0xbfc05258: /* 0xbfc05258 */
	move	$a0, $t0
	not	$t3, $a0
L_0xbfc05260: /* 0xbfc05260 */
	dsll32	$t3, $t3, 0
	dsrl32	$t3, $t3, 0
	dsll32	$t5, $a0, 0
	or	$t3, $t3, $t5
	sd	$t3, LO(KSEG1)($a0)
	addiu	$a0, $a0, LO(0xa0000008)
	bne	$a0, $t1, L_0xbfc05260
	 not	$t3, $a0
	move	$a0, $t0
	not	$t3, $a0
L_0xbfc05288: /* 0xbfc05288 */
	dsll32	$t3, $t3, 0
	dsrl32	$t3, $t3, 0
	dsll32	$t5, $a0, 0
	ld	$t4, LO(KSEG1)($a0)
	or	$t3, $t3, $t5
	bne	$t3, $t4, L_0xbfc052fc
	 li	$t5, 4
	not	$t4, $t4
	sd	$t4, LO(KSEG1)($a0)
	addiu	$a0, $a0, LO(0xa0000008)
	bne	$a0, $t1, L_0xbfc05288
	 not	$t3, $a0
	move	$a0, $t0
	not	$t3, $a0
L_0xbfc052c0: /* 0xbfc052c0 */
	dsll32	$t3, $t3, 0
	dsrl32	$t3, $t3, 0
	dsll32	$t5, $a0, 0
	ld	$t4, LO(KSEG1)($a0)
	or	$t3, $t3, $t5
	not	$t3, $t3
	bne	$t3, $t4, L_0xbfc052fc
	 li	$t5, 5
	addiu	$a0, $a0, LO(0xa0000008)
	bne	$a0, $t1, L_0xbfc052c0
	 not	$t3, $a0
	lui	$t5, HI(KSEG1)
	subu	$a0, $t0, $t5
	jr	$ra
	 subu	$a1, $t0, $t1
L_0xbfc052fc: /* 0xbfc052fc */
	lui	$s0, HI(BASE_ISA + ISA_MISC_CONTROL)
	addiu	$s0, $s0, LO(BASE_ISA + ISA_MISC_CONTROL)
	ld	$s1, 0($s0)
	ori	$s1, $s1, 0x30
L_0xbfc0530c: /* 0xbfc0530c */
	sd	$s1, 0($s0)
	lui	$a0, HI_UNSIGNED(500000)
	ori	$a0, $a0, LO_UNSIGNED(500000)
	jal	F_0xbfc05890
	 nop
	ld	$s1, 0($s0)		# ISA_MISC_CONTROL
	b	L_0xbfc0530c
	 xori	$s1, $s1, 0x30		# ISA_MISC_CONTROL
	.int	0x1000fff3
	.int	0x00000000
data_0xbfc05334: /* 0xbfc05334 */
	string 4, "looping %d"
data_0xbfc05340: /* 0xbfc05340 */
	string 4, "post1 memtst error\n expected=0x%0lx, obsev=0x%0lx, addr=0x%0lx"
	.int	0x27bdffd0
	.int	0xafbf001c
	.int	0xafb00018
	.int	0x3c0ebf31
	.int	0x0ff01511
	.int	0xfdc00000
	.int	0x10400009
	.int	0x3c10bf31

/* Function F_0xbfc053a0 [0xbfc053a0 - 0xbfc05464)
 */
F_0xbfc053a0: /* 0xbfc053a0 */
	ori	$s0, $s0, 8
	li	$t7, 0x20
	lui	$a0, HI_UNSIGNED(1000000)
	sd	$t7, 0($s0)
	jal	F_0xbfc05890
	 ori	$a0, $a0, LO_UNSIGNED(1000000)
	li	$t8, 0x30
	sd	$t8, 0($s0)
	lui	$s0, HI_UNSIGNED(BASE_ISA | ISA_MISC_CONTROL)
	ori	$s0, $s0, LO_UNSIGNED(BASE_ISA | ISA_MISC_CONTROL)
	jal	F_0xbfc05554
	 li	$a0, 1
	beqz	$v0, L_0xbfc053f0
	 li	$t9, 0x20
	lui	$a0, HI_UNSIGNED(1000000)
	sd	$t9, 0($s0)
	jal	F_0xbfc05890
	 ori	$a0, $a0, LO_UNSIGNED(1000000)
	li	$t0, 0x30
	sd	$t0, 0($s0)
L_0xbfc053f0: /* 0xbfc053f0 */
	jal	F_0xbfc05554
	 li	$a0, 2
	beqz	$v0, L_0xbfc05430
	 sw	$v0, 0x20($sp)
	li	$t1, 0x20
	lui	$a0, HI_UNSIGNED(1000000)
	sd	$t1, 0($s0)
	jal	F_0xbfc05890
	 ori	$a0, $a0, LO_UNSIGNED(1000000)
	li	$t2, 0x20
	lui	$a0, HI_UNSIGNED(1000000)
	sd	$t2, 0($s0)
	jal	F_0xbfc05890
	 ori	$a0, $a0, LO_UNSIGNED(1000000)
	li	$t3, 0x30
	sd	$t3, 0($s0)
L_0xbfc05430: /* 0xbfc05430 */
	lw	$ra, 0x1c($sp)
	lw	$v0, ISA_GREEN_LED($sp)
	lw	$s0, 0x18($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x30
	lui	$a0, HI(BASE_UART_1)		# unreachable
	li	$a1, 0x55		# unreachable
	lui	$a2, HI(BASE_UART_1)		# unreachable
	ori	$a2, $a2, 0x8000		# unreachable
	sb	$a1, 0x707($a0)		# unreachable
	lbu	$zero, 0x707($a2)		# unreachable
	lbu	$v0, 0x707($a0)		# unreachable
	li	$a3, 0x55		# unreachable

/* Function F_0xbfc05464 [0xbfc05464 - 0xbfc05554)
 */
F_0xbfc05464: /* 0xbfc05464 */
	move	$v1, $zero
	dsll32	$v0, $v0, 0
	dsra32	$v0, $v0, 0
	beq	$a3, $v0, L_0xbfc0547c
	 li	$t0, 0xaa
	li	$v1, 1
L_0xbfc0547c: /* 0xbfc0547c */
	sb	$t0, 0x707($a0)
	lbu	$zero, 0x707($a2)
	lbu	$v0, 0x707($a0)
	li	$t1, 0xaa
	li	$t2, 0xf0
	dsll32	$v0, $v0, 0
	dsra32	$v0, $v0, 0
	beq	$t1, $v0, L_0xbfc054a4
	 li	$t8, 0xf
	ori	$v1, $v1, 2
L_0xbfc054a4: /* 0xbfc054a4 */
	sb	$t2, 0x707($a0)
	lbu	$zero, 0x707($a2)
	lbu	$zero, 0x707($a0)
	sb	$t8, 0x707($a0)
	lbu	$zero, 0x707($a2)
	lbu	$zero, 0x707($a0)
	lbu	$zero, 7($a0)
	lbu	$zero, 0x107($a0)
	lbu	$zero, 0x207($a0)
	lbu	$zero, 0x307($a0)
	lbu	$zero, 0x407($a0)
	lbu	$zero, 0x507($a0)
	lbu	$zero, 0x607($a0)
	sb	$a1, 0x707($a2)
	lbu	$zero, 0x707($a0)
	lbu	$v0, 0x707($a2)
	dsll32	$v0, $v0, 0
	dsra32	$v0, $v0, 0
	beql	$a3, $v0, L_0xbfc054fc
	 sb	$t0, 0x707($a2)
	ori	$v1, $v1, 4
	sb	$t0, 0x707($a2)
L_0xbfc054fc: /* 0xbfc054fc */
	lbu	$zero, 0x707($a0)
	lbu	$v0, 0x707($a2)
	dsll32	$v0, $v0, 0
	dsra32	$v0, $v0, 0
	beql	$t1, $v0, L_0xbfc0551c
	 sb	$t2, 0x707($a2)
	ori	$v1, $v1, 8
	sb	$t2, 0x707($a2)
L_0xbfc0551c: /* 0xbfc0551c */
	lbu	$zero, 0x707($a0)
	lbu	$zero, 0x707($a2)
	sb	$t2, 0x707($a2)
	lbu	$zero, 0x707($a0)
	lbu	$zero, 0x707($a2)
	lbu	$zero, 7($a2)
	lbu	$zero, 0x107($a2)
	lbu	$zero, 0x207($a2)
	lbu	$zero, 0x307($a2)
	lbu	$zero, 0x407($a2)
	lbu	$zero, 0x507($a2)
	lbu	$zero, 0x607($a2)
	jr	$ra
	 move	$v0, $v1

/* Function F_0xbfc05554 [0xbfc05554 - 0xbfc05890)
 */
F_0xbfc05554: /* 0xbfc05554 */
	addiu	$sp, $sp, -0x148
	li	$at, 1
	sw	$ra, 0x24($sp)
	sw	$s3, 0x20($sp)
	sw	$s2, 0x1c($sp)
	sw	$s1, 0x18($sp)
	bne	$a0, $at, L_0xbfc0557c
	 sw	$s0, 0x14($sp)

/* Function F_0xbfc05574 [0xbfc05574 - 0xbfc05890)
 */
F_0xbfc05574: /* 0xbfc05574 */
	b	L_0xbfc05598
	 lui	$s2, HI(BASE_UART_1 + UART_LCR)
L_0xbfc0557c: /* 0xbfc0557c */
	li	$at, 2
	bne	$a0, $at, L_0xbfc05590
	 lui	$s2, HI_UNSIGNED(BASE_UART_2)
	b	L_0xbfc05598
	 ori	$s2, $s2, LO_UNSIGNED(BASE_UART_2)
L_0xbfc05590: /* 0xbfc05590 */
	b	L_0xbfc05874
	 li	$v0, 0x200
L_0xbfc05598: /* 0xbfc05598 */
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
	beqz	$t4, L_0xbfc05624
	 move	$s3, $zero
	addu	$s0, $s1, $t5
	addiu	$v0, $sp, 0x64
	lbu	$t6, LO(BASE_UART_1 + UART_DATA)($s2)
L_0xbfc055fc: /* 0xbfc055fc */
	addiu	$s0, $s0, 1
	sb	$t6, -1($s0)
	lbu	$t7, LO(BASE_UART_1 + UART_LSR)($s2)
	dsll32	$t7, $t7, 0
	dsra32	$t7, $t7, 0
	andi	$t9, $t7, 1
	beqzl	$t9, L_0xbfc05628
	 move	$s1, $zero
	bnel	$s0, $v0, L_0xbfc055fc
	 lbu	$t6, LO(BASE_UART_1 + UART_DATA)($s2)
L_0xbfc05624: /* 0xbfc05624 */
	move	$s1, $zero
L_0xbfc05628: /* 0xbfc05628 */
	lbu	$t0, LO(BASE_UART_1 + UART_LSR)($s2)
L_0xbfc0562c: /* 0xbfc0562c */
	li	$s0, 10000
	dsll32	$t0, $t0, 0
	dsra32	$t0, $t0, 0
	andi	$t2, $t0, 0x20
	bnezl	$t2, L_0xbfc05678
	 sb	$s3, LO(BASE_UART_1 + UART_DATA)($s2)
L_0xbfc05644: /* 0xbfc05644 */
	jal	F_0xbfc05890
	 li	$a0, 0x64
	lbu	$t4, LO(BASE_UART_1 + UART_LSR)($s2)
	li	$t3, -1
	addu	$s0, $s0, $t3
	dsll32	$t4, $t4, 0
	dsra32	$t4, $t4, 0
	andi	$t6, $t4, 0x20
	bnezl	$t6, L_0xbfc05678
	 sb	$s3, LO(BASE_UART_1 + UART_DATA)($s2)
	bnez	$s0, L_0xbfc05644
	 nop
	sb	$s3, LO(BASE_UART_1 + UART_DATA)($s2)
L_0xbfc05678: /* 0xbfc05678 */
	lbu	$t7, LO(BASE_UART_1 + UART_LSR)($s2)
	addiu	$s3, $s3, 1
	andi	$s3, $s3, 0xff
	dsll32	$t7, $t7, 0
	dsra32	$t7, $t7, 0
	andi	$t9, $t7, 1
	beqz	$t9, L_0xbfc056dc
	 dsll32	$s3, $s3, 0
	slti	$at, $s1, 0x100
	beqz	$at, L_0xbfc056dc
	 addiu	$t0, $sp, 0x44
	addu	$s0, $s1, $t0
	lbu	$t1, LO(BASE_UART_1 + UART_DATA)($s2)
L_0xbfc056ac: /* 0xbfc056ac */
	addiu	$s1, $s1, 1
	slti	$at, $s1, 0x100
	sb	$t1, 0($s0)
	lbu	$t2, LO(BASE_UART_1 + UART_LSR)($s2)
	addiu	$s0, $s0, 1
	dsll32	$t2, $t2, 0
	dsra32	$t2, $t2, 0
	andi	$t4, $t2, 1
	beqzl	$t4, L_0xbfc056e0
	 dsra32	$s3, $s3, 0
	bnezl	$at, L_0xbfc056ac
	 lbu	$t1, LO(BASE_UART_1 + UART_DATA)($s2)
L_0xbfc056dc: /* 0xbfc056dc */
	dsra32	$s3, $s3, 0
L_0xbfc056e0: /* 0xbfc056e0 */
	slti	$at, $s3, 0xff
	bnezl	$at, L_0xbfc0562c
	 lbu	$t0, LO(BASE_UART_1 + UART_LSR)($s2)
	jal	F_0xbfc05890
	 li	$a0, 10000
	lbu	$t6, LO(BASE_UART_1 + UART_LSR)($s2)
	slti	$at, $s1, 0x100
	dsll32	$t6, $t6, 0
	dsra32	$t6, $t6, 0
	andi	$t8, $t6, 1
	beqzl	$t8, L_0xbfc0575c
	 lb	$t4, 0x44($sp)
	beqz	$at, L_0xbfc05758
	 addiu	$t9, $sp, 0x44
	addu	$s0, $s1, $t9
	addiu	$s1, $sp, 0x144
	lbu	$t0, LO(BASE_UART_1 + UART_DATA)($s2)
L_0xbfc05724: /* 0xbfc05724 */
	addiu	$s0, $s0, 1
	li	$a0, 10000
	jal	F_0xbfc05890
	 sb	$t0, -1($s0)
	lbu	$t1, LO(BASE_UART_1 + UART_LSR)($s2)
	sltu	$at, $s0, $s1
	dsll32	$t1, $t1, 0
	dsra32	$t1, $t1, 0
	andi	$t3, $t1, 1
	beqzl	$t3, L_0xbfc0575c
	 lb	$t4, 0x44($sp)
	bnezl	$at, L_0xbfc05724
	 lbu	$t0, LO(BASE_UART_1 + UART_DATA)($s2)
L_0xbfc05758: /* 0xbfc05758 */
	lb	$t4, 0x44($sp)
L_0xbfc0575c: /* 0xbfc0575c */
	lb	$t6, 0x45($sp)
	li	$at, 1
	beqz	$t4, L_0xbfc05774
	 nop
	b	L_0xbfc05874
	 li	$v0, 0x100
L_0xbfc05774: /* 0xbfc05774 */
	beq	$t6, $at, L_0xbfc05784
	 lb	$t8, 0x46($sp)
	b	L_0xbfc05874
	 li	$v0, 0x101
L_0xbfc05784: /* 0xbfc05784 */
	li	$at, 2
	beq	$t8, $at, L_0xbfc05798
	 li	$s1, 3
	b	L_0xbfc05874
	 li	$v0, CTYPE_TOLOWER
L_0xbfc05798: /* 0xbfc05798 */
	addiu	$s0, $sp, 0x47
	li	$v0, 0xff
L_0xbfc057a0: /* 0xbfc057a0 */
	lbu	$t3, 0($s0)
	dsll32	$s1, $s1, 0
	dsrl32	$s1, $s1, 0
	andi	$t1, $s1, 0xff
	dsll32	$t1, $t1, 0
	dsll32	$t3, $t3, 0
	dsra32	$t3, $t3, 0
	dsra32	$t1, $t1, 0
	beq	$t1, $t3, L_0xbfc057d0
	 addiu	$v1, $s1, 1
	b	L_0xbfc05874
	 ori	$v0, $s1, 0x100
L_0xbfc057d0: /* 0xbfc057d0 */
	lbu	$t8, 1($s0)
	dsll32	$v1, $v1, 0
	dsrl32	$v1, $v1, 0
	andi	$t6, $v1, 0xff
	dsll32	$t6, $t6, 0
	dsll32	$t8, $t8, 0
	dsra32	$t8, $t8, 0
	dsra32	$t6, $t6, 0
	beql	$t6, $t8, L_0xbfc05804
	 addiu	$v1, $s1, 2
	b	L_0xbfc05874
	 ori	$v0, $v1, 0x100
	addiu	$v1, $s1, 2		# unreachable
L_0xbfc05804: /* 0xbfc05804 */
	lbu	$t2, 2($s0)
	dsll32	$v1, $v1, 0
	dsrl32	$v1, $v1, 0
	andi	$t1, $v1, 0xff
	dsll32	$t1, $t1, 0
	dsll32	$t2, $t2, 0
	dsra32	$t2, $t2, 0
	dsra32	$t1, $t1, 0
	beql	$t1, $t2, L_0xbfc05838
	 addiu	$v1, $s1, 3
	b	L_0xbfc05874
	 ori	$v0, $v1, 0x100
	addiu	$v1, $s1, 3		# unreachable
L_0xbfc05838: /* 0xbfc05838 */
	lbu	$t7, 3($s0)
	dsll32	$v1, $v1, 0
	dsrl32	$v1, $v1, 0
	andi	$t6, $v1, 0xff
	dsll32	$t6, $t6, 0
	dsll32	$t7, $t7, 0
	dsra32	$t7, $t7, 0
	dsra32	$t6, $t6, 0
	beq	$t6, $t7, L_0xbfc05868
	 addiu	$s1, $s1, 4
	b	L_0xbfc05874
	 ori	$v0, $v1, 0x100
L_0xbfc05868: /* 0xbfc05868 */
	bne	$s1, $v0, L_0xbfc057a0
	 addiu	$s0, $s0, 4
	move	$v0, $zero
L_0xbfc05874: /* 0xbfc05874 */
	lw	$ra, 0x24($sp)
	lw	$s0, 0x14($sp)
	lw	$s1, 0x18($sp)
	lw	$s2, 0x1c($sp)
	lw	$s3, 0x20($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x148

/* Function F_0xbfc05890 [0xbfc05890 - 0xbfc05900)
 */
F_0xbfc05890: /* 0xbfc05890 */
	lui	$v0, HI(BASE_UST)
	ld	$v1, LO(BASE_UST)($v0)
	ld	$t8, LO(BASE_UST)($v0)
	li	$at, -1
	and	$t6, $v1, $at
	daddu	$v1, $t6, $a0
	and	$t9, $t8, $at
	sltu	$at, $t9, $v1
usleep: /* 0xbfc058b0 */
	beqz	$at, L_0xbfc058d0
	 nop
	ld	$t0, LO(BASE_UST)($v0)
L_0xbfc058bc: /* 0xbfc058bc */
	li	$at, -1
	and	$t1, $t0, $at
	sltu	$at, $t1, $v1
	bnezl	$at, L_0xbfc058bc
	 ld	$t0, LO(BASE_UST)($v0)
L_0xbfc058d0: /* 0xbfc058d0 */
	jr	$ra
	 nop
	nop		# unreachable
	nop		# unreachable
	lui	$v0, HI(BASE_UST)		# unreachable
	ld	$v1, 0($v0)		# unreachable
	ld	$t7, 0($v0)		# unreachable
	daddu	$v1, $v1, $a0		# unreachable
	sltu	$at, $t7, $v1		# unreachable
	.word	0x10200005	/* beqz $at, 0xbfc0590c */		# unreachable
	 nop		# unreachable
	ld	$t8, 0($v0)		# unreachable

/* Function F_0xbfc05900 [0xbfc05900 - 0xbfc05914)
 */
F_0xbfc05900: /* 0xbfc05900 */
	sltu	$at, $t8, $v1
	bnezl	$at, F_0xbfc05900
	 ld	$t8, 0($v0)
	jr	$ra
	 nop

/* Function F_0xbfc05914 [0xbfc05914 - 0xbfc05978)
 */
F_0xbfc05914: /* 0xbfc05914 */
	addiu	$sp, $sp, -0x38
	sw	$s0, 0x30($sp)
	move	$s0, $a0
	sw	$ra, 0x34($sp)
	lw	$t6, 0xc($s0)
	lw	$a3, 8($s0)
	lw	$a2, 4($s0)
	lw	$a1, 0($s0)
print_bank_sizes: /* 0xbfc05934 */
	sw	$t6, 0x10($sp)
	lw	$t7, 0x10($s0)
	lui	$a0, %hi(data_0xbfc06a50)
	addiu	$a0, $a0, %lo(data_0xbfc06a50)
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

/* Function F_0xbfc05978 [0xbfc05978 - 0xbfc05a0c)
 */
F_0xbfc05978: /* 0xbfc05978 */
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
L_0xbfc059a8: /* 0xbfc059a8 */
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
	bne	$s2, $s4, L_0xbfc059a8
	 nop
	lw	$ra, 0x2c($sp)
	lw	$s0, 0x18($sp)
	lw	$s1, 0x1c($sp)
	lw	$s2, 0x20($sp)
	lw	$s3, 0x24($sp)
	lw	$s4, 0x28($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x30

/* Function F_0xbfc05a0c [0xbfc05a0c - 0xbfc05ab4)
 */
F_0xbfc05a0c: /* 0xbfc05a0c */
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
L_0xbfc05a60: /* 0xbfc05a60 */
	srl	$a1, $s2, 6
	ori	$a1, $a1, 0x17
	addu	$a2, $a1, $s5
	jal	set_CP0_ENTRY
	 and	$a0, $s1, $s4
	jal	tlb_write_index
	 move	$a0, $s0
	addiu	$s0, $s0, 1
	addu	$s1, $s1, $s3
	bne	$s0, $s6, L_0xbfc05a60
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

/* Function F_0xbfc05ab4 [0xbfc05ab4 - 0xbfc06080)
 */
F_0xbfc05ab4: /* 0xbfc05ab4 */
	addiu	$sp, $sp, -0xa0
	sw	$ra, 0x3c($sp)
	lui	$a0, %hi(data_0xbfc06b10)
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
	 addiu	$a0, $a0, %lo(data_0xbfc06b10)
	addiu	$s2, $sp, 0x70
	addiu	$v0, $sp, 0x90
L_0xbfc05af4: /* 0xbfc05af4 */
	addiu	$s2, $s2, 4
	sltu	$at, $s2, $v0
	bnez	$at, L_0xbfc05af4
	 sw	$zero, -4($s2)
	jal	F_0xbfc05914
	 addiu	$a0, $sp, 0x70
	lui	$v1, HI_UNSIGNED(BASE_CRIME | CRIME_CONTROL_OFFSET)
	ori	$v1, $v1, LO_UNSIGNED(BASE_CRIME | CRIME_CONTROL_OFFSET)
	ld	$v0, 0($v1)
	li	$t7, -0x11
	lui	$a0, %hi(data_0xbfc06b34)
	and	$t6, $v0, $t7
	sd	$t6, 0($v1)
	jal	printf_disabled
	 addiu	$a0, $a0, %lo(data_0xbfc06b34)
	lui	$s6, %hi(data_0xbfc06b78)
	lui	$s4, %hi(data_0xbfc06b50)
	lui	$s0, HI_UNSIGNED(BASE_CRIME | CRIME_BANK_0_CTRL)
	move	$s3, $zero
	move	$s1, $zero
	ori	$s0, $s0, LO_UNSIGNED(BASE_CRIME | CRIME_BANK_0_CTRL)
	addiu	$s4, $s4, %lo(data_0xbfc06b50)
	addiu	$s6, $s6, %lo(data_0xbfc06b78)
	addiu	$s2, $sp, 0x70
	li	$fp, 0x80
	lui	$s7, 0x800
	li	$s5, 8
L_0xbfc05b60: /* 0xbfc05b60 */
	move	$a0, $s4
	jal	printf_disabled
	 move	$a1, $s1
	jal	F_0xbfc05a0c
	 move	$a0, $s3
	slti	$at, $s1, 8
	beqz	$at, L_0xbfc05c54
	 move	$v0, $s1
	subu	$a2, $s5, $s1
	andi	$a2, $a2, 3
	beqz	$a2, L_0xbfc05bc4
	 addu	$a1, $a2, $s1
	srl	$t8, $s3, 0x19
	andi	$t9, $t8, 0x1f
	ori	$v1, $t9, 0x100
L_0xbfc05b9c: /* 0xbfc05b9c */
	sll	$t1, $v0, 3
	dsll32	$t1, $t1, 0
	dsrl32	$t1, $t1, 0
	addu	$t3, $s0, $t1
	dsll32	$t3, $t3, 0
	dsra32	$t3, $t3, 0
	addiu	$v0, $v0, 1
	bne	$a1, $v0, L_0xbfc05b9c
	 sd	$v1, 0($t3)
	beq	$v0, $s5, L_0xbfc05c54
L_0xbfc05bc4: /* 0xbfc05bc4 */
	 srl	$t5, $s3, 0x19
	andi	$t7, $t5, 0x1f
	ori	$v1, $t7, 0x100
L_0xbfc05bd0: /* 0xbfc05bd0 */
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
	bne	$v0, $s5, L_0xbfc05bd0
	 sd	$v1, 0($t5)
L_0xbfc05c54: /* 0xbfc05c54 */
	move	$a0, $s6
	jal	printf_disabled
	 move	$a1, $s3
	lui	$t6, %hi(data_0xbfc06e28)
	ld	$t6, %lo(data_0xbfc06e28)($t6)
	dsll32	$s3, $s3, 0
	dsra32	$s3, $s3, 0
	lui	$at, HI_UNSIGNED(0x01fffff8)
	ori	$at, $at, LO_UNSIGNED(0x01fffff8)
	lui	$t9, %hi(data_0xbfc06e30)
	sd	$t6, 0($s3)
	ld	$t9, %lo(data_0xbfc06e30)($t9)
	addu	$t8, $s3, $at
	dsll32	$t8, $t8, 0
	dsra32	$a0, $t8, 0
	lui	$at, 0x200
	lui	$t1, %hi(data_0xbfc06e38)
	sd	$t9, 0($a0)
	ld	$t1, %lo(data_0xbfc06e38)($t1)
	addu	$t0, $s3, $at
	dsll32	$t0, $t0, 0
	dsra32	$a1, $t0, 0
	lui	$at, HI_UNSIGNED(0x07fffff8)
	ori	$at, $at, LO_UNSIGNED(0x07fffff8)
	lui	$t3, %hi(data_0xbfc06e40)
	sd	$t1, 0($a1)
	ld	$t3, %lo(data_0xbfc06e40)($t3)
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
	lui	$t0, %hi(data_0xbfc06e48)
	ld	$t0, %lo(data_0xbfc06e48)($t0)
	ld	$v1, 0($s3)
	lui	$t2, %hi(data_0xbfc06e50)
	bne	$v1, $t0, L_0xbfc05d94
	 nop
	ld	$t1, 0($a0)
	ld	$t2, %lo(data_0xbfc06e50)($t2)
	lui	$t4, %hi(data_0xbfc06e58)
	bne	$t1, $t2, L_0xbfc05d94
	 nop
	ld	$t3, 0($a1)
	ld	$t4, %lo(data_0xbfc06e58)($t4)
	lui	$t7, %hi(data_0xbfc06e60)
	bne	$t3, $t4, L_0xbfc05d94
	 nop
	ld	$t5, 0($a2)
	ld	$t7, %lo(data_0xbfc06e60)($t7)
	sll	$t8, $s1, 3
	li	$t6, 0x20
	bne	$t5, $t7, L_0xbfc05d94
	 dsll32	$t8, $t8, 0
	dsrl32	$t8, $t8, 0
	addu	$t0, $t8, $s0
	dsll32	$t0, $t0, 0
	sw	$t6, 0($s2)
	dsra32	$t0, $t0, 0
	ld	$a2, 0($t0)
	lui	$a0, %hi(data_0xbfc06b9c)
	addiu	$a0, $a0, %lo(data_0xbfc06b9c)
	dsll32	$a3, $a2, 0
	dsra32	$a3, $a3, 0
	move	$a1, $s1
	jal	printf_disabled
	 dsra32	$a2, $a2, 0
	b	L_0xbfc05ebc
	 nop
L_0xbfc05d94: /* 0xbfc05d94 */
	lui	$t2, %hi(data_0xbfc06e68)
	ld	$t2, %lo(data_0xbfc06e68)($t2)
	lui	$t4, %hi(data_0xbfc06e70)
	bne	$v1, $t2, L_0xbfc05e20
	 nop
	ld	$t3, 0($a0)
	ld	$t4, %lo(data_0xbfc06e70)($t4)
	lui	$t7, %hi(data_0xbfc06e78)
	bne	$t3, $t4, L_0xbfc05e20
	 nop
	ld	$t5, 0($a1)
	ld	$t7, %lo(data_0xbfc06e78)($t7)
	lui	$t8, %hi(data_0xbfc06e80)
	bne	$t5, $t7, L_0xbfc05e20
	 nop
	ld	$t6, 0($a2)
	ld	$t8, %lo(data_0xbfc06e80)($t8)
	sll	$t9, $s1, 3
	dsll32	$t9, $t9, 0
	bne	$t6, $t8, L_0xbfc05e20
	 lui	$a0, %hi(data_0xbfc06bc8)
	dsrl32	$t9, $t9, 0
	addu	$t1, $t9, $s0
	dsll32	$t1, $t1, 0
	sw	$fp, 0($s2)
	dsra32	$t1, $t1, 0
	ld	$a2, 0($t1)
	addiu	$a0, $a0, %lo(data_0xbfc06bc8)
	move	$a1, $s1
	dsll32	$a3, $a2, 0
	dsra32	$a3, $a3, 0
	jal	printf_disabled
	 dsra32	$a2, $a2, 0
	b	L_0xbfc05ebc
	 nop
L_0xbfc05e20: /* 0xbfc05e20 */
	bnez	$s1, L_0xbfc05e84
	 lui	$a0, %hi(data_0xbfc06c10)
	li	$t3, 8
	subu	$a2, $t3, $zero
	andi	$a2, $a2, 3
	lui	$a0, %hi(data_0xbfc06bf4)
	addiu	$a0, $a0, %lo(data_0xbfc06bf4)
	move	$a1, $a2
	move	$s1, $zero
	move	$s3, $zero
	jal	printf_disabled
	 move	$v0, $zero
	lui	$s0, HI_UNSIGNED(BASE_ISA | ISA_MISC_CONTROL)
	ori	$s0, $s0, LO_UNSIGNED(BASE_ISA | ISA_MISC_CONTROL)
	ld	$t4, 0($s0)
	lui	$s1, HI_UNSIGNED(500000)
	ori	$s1, $s1, LO_UNSIGNED(500000)
	ori	$t5, $t4, 0x30
	sd	$t5, 0($s0)
L_0xbfc05e6c: /* 0xbfc05e6c */
	jal	F_0xbfc05890
	 move	$a0, $s1
	ld	$t7, 0($s0)
	xori	$t6, $t7, 0x30		# ISA_MISC_CONTROL
	b	L_0xbfc05e6c
	 sd	$t6, 0($s0)
L_0xbfc05e84: /* 0xbfc05e84 */
	sll	$t8, $s1, 3
	dsll32	$t8, $t8, 0		# ISA_MISC_CONTROL
	dsrl32	$t8, $t8, 0
	addu	$t0, $t8, $s0
	dsll32	$t0, $t0, 0		# ISA_MISC_CONTROL
	sw	$zero, 0($s2)
	dsra32	$t0, $t0, 0
	ld	$a2, 0($t0)		# ISA_MISC_CONTROL
	addiu	$a0, $a0, %lo(data_0xbfc06c10)
	move	$a1, $s1
	dsll32	$a3, $a2, 0
	dsra32	$a3, $a3, 0
	jal	printf_disabled
	 dsra32	$a2, $a2, 0
L_0xbfc05ebc: /* 0xbfc05ebc */
	jal	F_0xbfc05978
	 nop
	addiu	$s1, $s1, 1
	slti	$at, $s1, 8
	addiu	$s2, $s2, 4
	bnez	$at, L_0xbfc05b60
	 addu	$s3, $s3, $s7
	jal	F_0xbfc05914
	 addiu	$a0, $sp, 0x70
	lui	$s4, %hi(data_0xbfc06c48)
	move	$s3, $zero
	addiu	$s4, $s4, %lo(data_0xbfc06c48)
	move	$s1, $zero
	addiu	$s2, $sp, 0x70
	li	$s6, -1
L_0xbfc05ef8: /* 0xbfc05ef8 */
	lw	$t2, 0($s2)
	srl	$t3, $s3, 0x19
	andi	$t4, $t3, 0x1f
	bne	$fp, $t2, L_0xbfc05f40
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
L_0xbfc05f40: /* 0xbfc05f40 */
	addiu	$s1, $s1, 1
	slti	$at, $s1, 8
	bnez	$at, L_0xbfc05ef8
	 addiu	$s2, $s2, 4
	jal	F_0xbfc05914
	 addiu	$a0, $sp, 0x70
	lui	$s4, %hi(data_0xbfc06c80)
	addiu	$s4, $s4, %lo(data_0xbfc06c80)
	move	$s1, $zero
	addiu	$s2, $sp, 0x70
L_0xbfc05f68: /* 0xbfc05f68 */
	lw	$t1, 0($s2)
	li	$at, 0x20
	srl	$t2, $s3, 0x19
	bne	$t1, $at, L_0xbfc05fb4
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
L_0xbfc05fb4: /* 0xbfc05fb4 */
	addiu	$s1, $s1, 1
	slti	$at, $s1, 8
	bnez	$at, L_0xbfc05f68
	 addiu	$s2, $s2, 4
	jal	F_0xbfc05914
	 addiu	$a0, $sp, 0x70
	lui	$s4, HI_UNSIGNED(BASE_CRIME | CRIME_BANK_0_CTRL)
	ori	$s4, $s4, LO_UNSIGNED(BASE_CRIME | CRIME_BANK_0_CTRL)
	move	$s1, $zero
	addiu	$s2, $sp, 0x70
L_0xbfc05fdc: /* 0xbfc05fdc */
	lw	$t7, 0($s2)
	sll	$t1, $s1, 3
	dsll32	$t1, $t1, 0
	bnez	$t7, L_0xbfc06028
	 dsrl32	$t1, $t1, 0
	ld	$t0, 0($s4)
	addu	$t3, $s0, $t1
	dsll32	$t3, $t3, 0
	dsra32	$t3, $t3, 0
	sd	$t0, 0($t3)
	sw	$s6, 0($s2)
	ld	$a2, 0($s4)
	lui	$a0, %hi(data_0xbfc06cb8)
	addiu	$a0, $a0, %lo(data_0xbfc06cb8)
	dsll32	$a3, $a2, 0
	dsra32	$a3, $a3, 0
	move	$a1, $s1
	jal	printf_disabled
	 dsra32	$a2, $a2, 0
L_0xbfc06028: /* 0xbfc06028 */
	addiu	$s1, $s1, 1
	bne	$s1, $s5, L_0xbfc05fdc
	 addiu	$s2, $s2, 4
	lui	$a0, %hi(data_0xbfc06cec)
	addiu	$a0, $a0, %lo(data_0xbfc06cec)
	jal	printf_disabled
	 move	$a1, $s3
	jal	F_0xbfc05978
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

/* Function F_0xbfc06080 [0xbfc06080 - 0xbfc060d8)
 */
F_0xbfc06080: /* 0xbfc06080 */
	addiu	$sp, $sp, -0x20
	sw	$ra, 0x1c($sp)
	sw	$s1, 0x18($sp)
	sw	$s0, 0x14($sp)
	move	$a0, $zero
	jal	F_0xbfc050f4
	 li	$a1, 0x1000
	move	$s0, $zero
	li	$s1, 0x1008
	move	$a0, $s0
L_0xbfc060a8: /* 0xbfc060a8 */
	move	$a1, $zero
	jal	writeback_dword
	 move	$a2, $zero
	addiu	$s0, $s0, 8
	bnel	$s0, $s1, L_0xbfc060a8
	 move	$a0, $s0
	lw	$ra, 0x1c($sp)
	lw	$s0, 0x14($sp)
	lw	$s1, 0x18($sp)
	addiu	$sp, $sp, 0x20
	jr	$ra
	 li	$v0, 1

/* Function F_0xbfc060d8 [0xbfc060d8 - 0xbfc061d0)
 */
F_0xbfc060d8: /* 0xbfc060d8 */
	addiu	$sp, $sp, -0x60
	lui	$v1, HI_UNSIGNED(BASE_CRIME | CRIME_CONTROL_OFFSET)
	ori	$v1, $v1, LO_UNSIGNED(BASE_CRIME | CRIME_CONTROL_OFFSET)
	sw	$ra, 0x14($sp)
	sw	$a0, 0x60($sp)
	sw	$a1, 0x64($sp)
	sw	$a2, 0x68($sp)
	ld	$v0, 0($v1)
	li	$t7, -0x11
	lui	$a0, %hi(data_0xbfc06d18)
	and	$t6, $v0, $t7
	sd	$t6, 0($v1)
	jal	printf_disabled
	 addiu	$a0, $a0, %lo(data_0xbfc06d18)
	lui	$t0, %hi(F_0xbfc04fc0)
	lui	$a3, %hi(.Llocal2)
	addiu	$a3, $a3, %lo(.Llocal2)
	addiu	$t0, $t0, %lo(F_0xbfc04fc0)
	dsll32	$t0, $t0, 0
	dsll32	$a3, $a3, 0
	lui	$v1, 0xa001
	dsrl32	$a3, $a3, 0
	dsrl32	$t0, $t0, 0
	lui	$a0, %hi(data_0xbfc06d38)
	addiu	$a0, $a0, %lo(data_0xbfc06d38)
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
	beqz	$a0, L_0xbfc0618c
	 addiu	$v0, $t0, 4
L_0xbfc06170: /* 0xbfc06170 */
	lwu	$t8, 0($v0)
	xor	$a0, $v0, $a3
	sltu	$a0, $zero, $a0
	addiu	$v0, $v0, 4
	addiu	$v1, $v1, 4
	bnez	$a0, L_0xbfc06170
	 sw	$t8, -4($v1)
L_0xbfc0618c: /* 0xbfc0618c */
	lui	$a0, %hi(data_0xbfc06d78)
	addiu	$a0, $a0, %lo(data_0xbfc06d78)
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

/* Function F_0xbfc061d0 [0xbfc061d0 - 0xbfc06390)
 */
F_0xbfc061d0: /* 0xbfc061d0 */
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
	jal	F_0xbfc066e0
	 sw	$s0, 0x18($sp)
	blezl	$v0, L_0xbfc06344
	 lw	$ra, 0x3c($sp)
	beqz	$s2, L_0xbfc06340
	 li	$fp, -1
	lui	$s5, HI_UNSIGNED(0x00020001)
	lui	$s1, %hi(data_0xbfc09010)
	addiu	$s1, $s1, %lo(data_0xbfc09010)
	ori	$s5, $s5, LO_UNSIGNED(0x00020001)
	addiu	$s7, $sp, 0x48
	lui	$s6, 2
	lw	$s4, 0x50($sp)
	sltu	$at, $s2, $s5
L_0xbfc06238: /* 0xbfc06238 */
	bnez	$at, L_0xbfc06248
	 move	$a0, $s3
	b	L_0xbfc06254		# Pointer to 0xbfc09130, containing 0xb5000000
	 move	$s0, $s6
L_0xbfc06248: /* 0xbfc06248 */
	dsll32	$s2, $s2, 0
	dsra32	$s2, $s2, 0
	move	$s0, $s2
L_0xbfc06254: /* 0xbfc06254 */
	move	$a1, $s0
	li	$a2, 0x700
	jal	F_0xbfc06470
	 move	$a3, $s7
	lw	$v1, 0x48($sp)
	or	$s4, $s4, $v0
	slt	$at, $s0, $v1
	bnez	$at, L_0xbfc0627c
	 nop
	move	$s0, $v1
L_0xbfc0627c: /* 0xbfc0627c */
	jal	F_0xbfc065d8
	 li	$a0, 5
	lw	$t6, 0($s1)
	dsll32	$s0, $s0, 0
	dsrl32	$v0, $s0, 0
	dsll32	$t6, $t6, 0
	dsrl32	$t6, $t6, 0
	addiu	$t8, $t6, 0x3030
	dsll32	$t8, $t8, 0
	dsra32	$t9, $t8, 0
	sw	$s3, 0($t9)
	lw	$t2, 0($s1)
	addu	$v1, $v0, $s3
	li	$t1, -1
	dsll32	$t2, $t2, 0
	dsrl32	$t2, $t2, 0
	addiu	$t4, $t2, 0x3038
	dsll32	$t4, $t4, 0
	dsra32	$t5, $t4, 0
	addu	$t0, $v1, $t1
	sw	$t0, 0($t5)
	lw	$t6, 0($s1)
	li	$t0, 0x11
	subu	$s2, $s2, $v0
	dsll32	$t6, $t6, 0
	dsrl32	$t6, $t6, 0
	addiu	$t8, $t6, 0x3018
	dsll32	$t8, $t8, 0
	dsra32	$t9, $t8, 0
	sw	$zero, 0($t9)
	lw	$t1, 0($s1)
	move	$s3, $v1
	dsll32	$t1, $t1, 0
	dsrl32	$t1, $t1, 0
	addiu	$t3, $t1, 0x3008
	dsll32	$t3, $t3, 0
	dsra32	$t4, $t3, 0
	sw	$fp, 0($t4)
	lw	$t5, 0($s1)
	dsll32	$t5, $t5, 0
	dsrl32	$t5, $t5, 0
	addiu	$t7, $t5, 0x3800
	dsll32	$t7, $t7, 0
	dsra32	$t8, $t7, 0
	jal	F_0xbfc063e0
	 sw	$t0, 0($t8)
	bnezl	$s2, L_0xbfc06238
	 sltu	$at, $s2, $s5
	sw	$s4, 0x50($sp)
L_0xbfc06340: /* 0xbfc06340 */
	lw	$ra, 0x3c($sp)
L_0xbfc06344: /* 0xbfc06344 */
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
	addiu	$sp, $sp, -0x18		# unreachable
	sw	$ra, 0x14($sp)		# unreachable
	lui	$a0, 0xbfc1		# unreachable
	jal	0xbfc06700		# unreachable
	 lw	$a0, -0x6ff0($a0)		# unreachable
	.word	0x10400004	/* beqz $v0, 0xbfc06398 */		# unreachable
	 lw	$ra, 0x14($sp)		# unreachable
	lui	$v0, 0xff		# unreachable

/* Function F_0xbfc06390 [0xbfc06390 - 0xbfc063e0)
 */
F_0xbfc06390: /* 0xbfc06390 */
	b	L_0xbfc063d8
	 ori	$v0, $v0, 0xffff
	lui	$t6, 0xbfc1		# unreachable
	lwu	$t6, -0x6ff0($t6)		# unreachable
	addiu	$t8, $t6, 0x4000		# unreachable; Pointer to 0xbfc09130, containing 0xb5000000
	dsll32	$t8, $t8, 0		# unreachable
	dsra32	$t9, $t8, 0		# unreachable
	lwu	$v0, 0($t9)		# unreachable
	sll	$t0, $v0, 6		# unreachable
	.word	0x05010003	/* bgez $t0, 0xbfc063c4 */		# unreachable
	 srl	$v1, $v0, 0x12		# unreachable
	.word	0x10000004	/* b 0xbfc063d0 */		# unreachable; Pointer to 0xbfc09130, containing 0xb5000000
	 move	$v1, $zero		# unreachable
	andi	$v1, $v1, 0x3f		# unreachable
	sll	$v1, $v1, 8		# unreachable
	addiu	$v1, $v1, 0x300		# unreachable
	dsll32	$v1, $v1, 0		# unreachable
	dsra32	$v0, $v1, 0		# unreachable
L_0xbfc063d8: /* 0xbfc063d8 */
	jr	$ra
	 addiu	$sp, $sp, 0x18

/* Function F_0xbfc063e0 [0xbfc063e0 - 0xbfc06470)
 */
F_0xbfc063e0: /* 0xbfc063e0 */
	addiu	$sp, $sp, -0x20
	sw	$s0, 0x14($sp)
	lui	$s0, %hi(data_0xbfc09010)
	addiu	$s0, $s0, %lo(data_0xbfc09010)
	lw	$t6, 0($s0)
	sw	$ra, 0x1c($sp)
	sw	$s1, 0x18($sp)
	dsll32	$t6, $t6, 0
	dsrl32	$t6, $t6, 0
	addiu	$t8, $t6, 0x4000
	dsll32	$t8, $t8, 0
	dsra32	$t9, $t8, 0		# Pointer to 0xbfc09130, containing 0xb5000000
	lwu	$v0, 0($t9)
	lui	$s1, 0x200
	sltiu	$v0, $v0, 1
	sll	$t0, $v0, 6
	bgezl	$t0, L_0xbfc06460
	 lw	$ra, 0x1c($sp)
L_0xbfc06428: /* 0xbfc06428 */
	jal	F_0xbfc06690
	 li	$a0, 1
	lw	$t1, 0($s0)
	dsll32	$t1, $t1, 0
	dsrl32	$t1, $t1, 0
	addiu	$t3, $t1, 0x4000
	dsll32	$t3, $t3, 0
	dsra32	$t4, $t3, 0
	lwu	$v0, 0($t4)
	sltiu	$v0, $v0, 1
	and	$t5, $v0, $s1
	bnez	$t5, L_0xbfc06428
	 nop
	lw	$ra, 0x1c($sp)
L_0xbfc06460: /* 0xbfc06460 */
	lw	$s0, 0x14($sp)
	lw	$s1, 0x18($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x20

/* Function F_0xbfc06470 [0xbfc06470 - 0xbfc065d8)
 */
F_0xbfc06470: /* 0xbfc06470 */
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
	beqz	$t8, L_0xbfc064e0
	 sw	$zero, 0($a3)
	li	$t9, -1
	li	$t3, -0x1000
	addu	$s0, $v1, $t9
	sw	$t3, 0($a3)
L_0xbfc064e0: /* 0xbfc064e0 */
	sll	$v0, $s0, 0xc
	subu	$t4, $t0, $v0
	lui	$at, 2
	sltu	$at, $t4, $at
	bnez	$at, L_0xbfc06504
	 lui	$at, HI_UNSIGNED(0x0001ffff)
	ori	$at, $at, LO_UNSIGNED(0x0001ffff)
	addu	$s6, $v0, $at
	srl	$s6, $s6, 0xc
L_0xbfc06504: /* 0xbfc06504 */
	subu	$t5, $s6, $s0
	srl	$t6, $t5, 1
	dsll32	$t6, $t6, 0
	jal	F_0xbfc065d8
	 dsra32	$a0, $t6, 0
	sltu	$at, $s6, $s0
	bnez	$at, L_0xbfc065a8
	 lui	$s5, %hi(data_0xbfc09010)
	addiu	$s5, $s5, %lo(data_0xbfc09010)
	lui	$s2, HI(KSEG0)
L_0xbfc0652c: /* 0xbfc0652c */
	andi	$t7, $s0, 0x1f
	lw	$t6, 0($s5)
	srl	$t8, $t7, 1
	dsll32	$t8, $t8, 0
	dsra32	$s1, $t8, 0
	dsll32	$v0, $s1, 0
	or	$t9, $s2, $s0		# Pointer to 0xbfc09130, containing 0xb5000000
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
	jal	F_0xbfc066c8
	 addiu	$a2, $a2, 0x1000
	lw	$t3, 0($s3)
	addiu	$s0, $s0, 2
	li	$t2, 1
	sltu	$at, $s6, $s0
	sllv	$t4, $t2, $s1
	addiu	$t5, $t3, 0x2000
	or	$s4, $s4, $t4
	beqz	$at, L_0xbfc0652c
	 sw	$t5, 0($s3)
L_0xbfc065a8: /* 0xbfc065a8 */
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

/* Function F_0xbfc065d8 [0xbfc065d8 - 0xbfc06690)
 */
F_0xbfc065d8: /* 0xbfc065d8 */
	addiu	$sp, $sp, -0x28
	sw	$s2, 0x20($sp)
	lui	$s2, %hi(data_0xbfc09010)
	addiu	$s2, $s2, %lo(data_0xbfc09010)
	lw	$t6, 0($s2)
	sw	$ra, 0x24($sp)
	sw	$s1, 0x1c($sp)
	dsll32	$t6, $t6, 0
	dsrl32	$t6, $t6, 0
	addiu	$t8, $t6, 0x4000
	dsll32	$t8, $t8, 0
	sw	$s0, 0x18($sp)		# Pointer to 0xbfc09130, containing 0xb5000000
	dsra32	$t9, $t8, 0
	lwu	$t0, 0($t9)
	li	$s1, 0x40
	dsll32	$a0, $a0, 0
	srl	$t1, $t0, 0x12
	andi	$t2, $t1, 0x3f
	subu	$t3, $s1, $t2
	dsrl32	$s0, $a0, 0
	sltu	$at, $t3, $s0
	beqzl	$at, L_0xbfc06674
	 lw	$ra, 0x24($sp)
L_0xbfc06634: /* 0xbfc06634 */
	jal	F_0xbfc06690
	 li	$a0, 1
	lw	$t4, 0($s2)
	dsll32	$t4, $t4, 0
	dsrl32	$t4, $t4, 0
	addiu	$t6, $t4, 0x4000
	dsll32	$t6, $t6, 0
	dsra32	$t7, $t6, 0
	lwu	$t8, 0($t7)
	srl	$t9, $t8, 0x12
	andi	$t0, $t9, 0x3f
	subu	$t1, $s1, $t0
	sltu	$at, $t1, $s0
	bnez	$at, L_0xbfc06634
	 nop
	lw	$ra, 0x24($sp)
L_0xbfc06674: /* 0xbfc06674 */
	lw	$s0, 0x18($sp)
	lw	$s1, 0x1c($sp)
	lw	$s2, 0x20($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x28
	nop		# unreachable
	nop		# unreachable

/* Function F_0xbfc06690 [0xbfc06690 - 0xbfc066c8)
 */
F_0xbfc06690: /* 0xbfc06690 */
	li	$a2, 0x42
	mult	$a0, $a2
	lui	$a1, HI_UNSIGNED(BASE_CRIME | CRIME_TIMER_OFFSET)
	ori	$a1, $a1, LO_UNSIGNED(BASE_CRIME | CRIME_TIMER_OFFSET)
	sd	$zero, 0($a1)
	mflo	$a0
	add	$a0, $a0, $zero
	nop
L_0xbfc066b0: /* 0xbfc066b0 */
	ld	$a2, 0($a1)
	sub	$a2, $a0, $a2
	bgtz	$a2, L_0xbfc066b0
	 nop
	jr	$ra
	 nop

/* Function F_0xbfc066c8 [0xbfc066c8 - 0xbfc066e0)
 */
F_0xbfc066c8: /* 0xbfc066c8 */
	dsll32	$a1, $a1, 0
	dsrl32	$a1, $a1, 0
	dsll32	$a0, $a0, 0
	or	$a0, $a0, $a1
	jr	$ra
	 sd	$a0, 0($a2)

/* Function F_0xbfc066e0 [0xbfc066e0 - 0xbfc06710)
 */
F_0xbfc066e0: /* 0xbfc066e0 */
	lui	$v0, HI(BASE_CRIME + CRIME_ID_OFFSET)
	ld	$v0, LO(BASE_CRIME + CRIME_ID_OFFSET)($v0)
	jr	$ra
	 andi	$v0, $v0, 0xf
	jr	$ra		# unreachable
	 move	$v0, $zero		# unreachable
	jr	$ra		# unreachable
	 nop		# unreachable
	jr	$ra		# unreachable
	 move	$v0, $zero		# unreachable
get_crime_rev_id: /* 0xbfc06708 */
	nop		# unreachable
	nop		# unreachable

/* Function F_0xbfc06710 [0xbfc06710 - 0xbfc06720)
 */
F_0xbfc06710: /* 0xbfc06710 */
	lui	$t1, %hi(F_0xbfc06728)
	lui	$t0, HI(KSEG1)
	addiu	$t1, $t1, %lo(F_0xbfc06728)
	or	$t0, $t0, $t1

/* Function F_0xbfc06720 [0xbfc06720 - 0xbfc06728)
 */
F_0xbfc06720: /* 0xbfc06720 */
	jr	$t0
	 nop

/* Function F_0xbfc06728 [0xbfc06728 - 0xbfc06740)
 */
F_0xbfc06728: /* 0xbfc06728 */
	mtc0	$zero, $CP0_PAGEMASK
	mtc0	$zero, $CP0_TAGHI
	li	$t1, 9
	nop
	mfc0	$t0, $CP0_PRID
	andi	$t0, $t0, 0xff00

/* Function tlb_init_uncached_trampoline [0xbfc06740 - 0xbfc06a00)
 *
 * Jump to tlb_init through uncached KSEG1
 */
tlb_init_uncached_trampoline: /* 0xbfc06740 */
	srl	$t0, $t0, 8
	beq	$t0, $t1, L_0xbfc06824
	 nop
	li	$t0, 0x2f
	lui	$t1, HI(0x0fffe000)		# Jump to (KSEG1 | tlb_init)
L_0xbfc06754: /* 0xbfc06754 */
	li	$at, 0x1fff

/* Function tlb_init [0xbfc06758 - 0xbfc06824)
 */
tlb_init: /* 0xbfc06758 */
	not	$t2, $at
	and	$t2, $t2, $t1
	lui	$at, HI(KSEG0)
	mtc0	$t0, $CP0_INDEX
	or	$t2, $t2, $at
	mtc0	$t2, $CP0_ENTRYHI
	srl	$at, $t1, 0xc
	sll	$at, $at, 6
	ori	$at, $at, 0x11
	mtc0	$at, $CP0_ENTRYLO0
	addiu	$t2, $at, 0x40
	mtc0	$at, $CP0_ENTRYLO1
	addi	$t0, $t0, -1
	addiu	$t1, $t1, LO(0x0fffe000)
	bgtz	$t0, L_0xbfc06754
	 tlbwi
	mfc0	$t0, $CP0_CONFIG
tlb_rm7k_write_tlb_loop: /* 0xbfc0679c */
	li	$at, -0x1001
	and	$t0, $t0, $at
	mtc0	$t0, $CP0_CONFIG
	mfc0	$t0, $CP0_CONFIG
	srl	$t0, $t0, 9
	li	$t1, 0x1000
	andi	$t0, $t0, 7
	sllv	$t0, $t1, $t0
	addi	$t0, $t0, -0x20
	lui	$t1, HI(KSEG0)
	addu	$t2, $t0, $t1
L_0xbfc067c8: /* 0xbfc067c8 */
	cache	(CACHE_TYPE_L1I|INDEX_WRITEBACK_INV), 0($t2)
	addi	$t0, $t0, -0x20
	bgez	$t0, L_0xbfc067c8
	 addu	$t2, $t0, $t1
	mfc0	$t0, $CP0_CONFIG
	srl	$t0, $t0, 6
	li	$t1, 0x1000
	andi	$t0, $t0, 7
	sllv	$t0, $t1, $t0
	addi	$t0, $t0, -0x20
	lui	$t1, HI(KSEG0)
	lui	$at, 0x1000
L_0xbfc067f8: /* 0xbfc067f8 */
	addu	$at, $at, $t0
	srl	$at, $at, 0xc
	sll	$at, $at, 8
	mtc0	$at, $CP0_TAGLO
	addu	$t2, $t0, $t1
	addi	$t0, $t0, -CACHE_LINE_SIZE
	cache	(CACHE_TYPE_L1D|INDEX_STORE_TAG), 0($t2)
	bgez	$t0, L_0xbfc067f8
tlb_rm7k_inv_l1i_loop: /* 0xbfc06818 */
	 lui	$at, 0x1000
	jr	$ra
	 nop
L_0xbfc06824: /* 0xbfc06824 */
	li	$t0, 0x3f
	lui	$t1, HI(0x0fffe000)
L_0xbfc0682c: /* 0xbfc0682c */
	li	$at, 0x1fff
	not	$t2, $at
	and	$t2, $t2, $t1
	lui	$at, HI(KSEG1)
	mtc0	$t0, $CP0_INDEX
	or	$t2, $t2, $at
	mtc0	$t2, $CP0_ENTRYHI
tlb_rm7k_inv_l1d_loop: /* 0xbfc06848 */
	srl	$at, $t1, 0xc
	sll	$at, $at, 6
	ori	$at, $at, 0x11
	mtc0	$at, $CP0_ENTRYLO0
	addiu	$t2, $at, 0x40
	mtc0	$at, $CP0_ENTRYLO1
	addi	$t0, $t0, -1
	addiu	$t1, $t1, LO(0x0fffe000)
	bgtz	$t0, L_0xbfc0682c
	 tlbwi
	mtc0	$zero, $CP0_TAGHI
tlb_r5k_init: /* 0xbfc06874 */
	mtc0	$zero, $CP0_TAGLO
	mtc0	$zero, $CP0_ECC
tlb_r5k_write_tlb_loop: /* 0xbfc0687c */
	lui	$t0, HI(KSEG0)
	li	$t1, 0x3fc0
L_0xbfc06884: /* 0xbfc06884 */
	or	$t2, $t0, $t1
	cache	(CACHE_TYPE_L1I|INDEX_STORE_TAG), 0($t2)
	addi	$t1, $t1, -0x40
	cache	(CACHE_TYPE_L1I|INDEX_STORE_TAG), 1($t2)
	bgez	$t1, L_0xbfc06884
	 nop
	mtc0	$zero, $CP0_TAGHI
	mtc0	$zero, $CP0_TAGLO
	mtc0	$zero, $CP0_ECC
	lui	$t0, HI(KSEG0)
	li	$t1, 0x3fe0
L_0xbfc068b0: /* 0xbfc068b0 */
	or	$t2, $t0, $t1
	cache	(CACHE_TYPE_L1D|INDEX_STORE_TAG), 0($t2)
	addi	$t1, $t1, -0x20
	cache	(CACHE_TYPE_L1D|INDEX_STORE_TAG), 1($t2)
	bgez	$t1, L_0xbfc068b0
	 nop
	mfc0	$t0, $CP0_CONFIG
	lui	$t1, HI(ROM_SIZE)
	srl	$t2, $t0, 0x10
	andi	$t2, $t2, 7
L_0xbfc068d8: /* 0xbfc068d8 */
	slti	$at, $t2, 6
	beqz	$at, L_0xbfc068d8
	 nop
	sllv	$t0, $t1, $t2
	andi	$at, $t0, 0x2000
	li	$t2, 0x80
tlb_r5k_inv_l1i_loop: /* 0xbfc068f0 */
	bgtz	$at, L_0xbfc068fc
	 nop
	li	$t2, 0x40
L_0xbfc068fc: /* 0xbfc068fc */
	mtc0	$zero, $CP0_TAGHI
	mtc0	$zero, $CP0_TAGLO
	srl	$t1, $t0, 1
	mtc0	$zero, $CP0_ECC
	sub	$t1, $t1, $t2
L_0xbfc06910: /* 0xbfc06910 */
	lui	$t3, HI(KSEG0)
	or	$t3, $t3, $t1
	cache	(CACHE_TYPE_L2|INDEX_STORE_TAG), 0($t3)
	sub	$t1, $t1, $t2
tlb_r5k_inv_l1d_loop: /* 0xbfc06920 */
	cache	(CACHE_TYPE_L2|INDEX_STORE_TAG), 1($t3)
	bgez	$t1, L_0xbfc06910
	 nop
	mtc0	$zero, $CP0_ECC
	mtc0	$zero, $CP0_TAGHI
	mtc0	$zero, $CP0_TAGLO
	nop
	move	$t0, $zero
L_0xbfc06940: /* 0xbfc06940 */
	lui	$t1, HI(KSEG0)
	or	$t1, $t1, $t0
	cache	(CACHE_TYPE_L1D|HIT_SET_VIRTUAL), 0($t1)
tlb_r10k_init: /* 0xbfc0694c */
	addi	$t0, $t0, 4
	li	$t2, 0x4000
tlb_r10k_write_tlb_loop: /* 0xbfc06954 */
	cache	(CACHE_TYPE_L1D|HIT_SET_VIRTUAL), 1($t1)
	slt	$at, $t0, $t2
	bnez	$at, L_0xbfc06940
	 nop
	mtc0	$zero, $CP0_ECC
	mtc0	$zero, $CP0_TAGHI
	mtc0	$zero, $CP0_TAGLO
	nop
	move	$t0, $zero
L_0xbfc06978: /* 0xbfc06978 */
	lui	$t1, HI(KSEG0)
	or	$t1, $t1, $t0
	cache	(CACHE_TYPE_L1I|HIT_SET_VIRTUAL), 0($t1)
	addi	$t0, $t0, 4
	li	$t2, 0x4000
	cache	(CACHE_TYPE_L1I|HIT_SET_VIRTUAL), 1($t1)
	slt	$at, $t0, $t2
	bnez	$at, L_0xbfc06978
	 nop
	mfc0	$t0, $CP0_CONFIG
	lui	$t1, HI(ROM_SIZE)
	srl	$t2, $t0, 0x10
	andi	$t2, $t2, 7
tlb_r10k_inv_l1i_loop: /* 0xbfc069ac */
	slti	$at, $t2, 6
	beqz	$at, tlb_r10k_inv_l1i_loop
	 nop
	sllv	$t0, $t1, $t2
	mtc0	$zero, $CP0_ECC
	srl	$t2, $t0, 1
	mtc0	$zero, $CP0_TAGHI
	mtc0	$zero, $CP0_TAGLO
	move	$t0, $zero
L_0xbfc069d0: /* 0xbfc069d0 */
	lui	$at, HI(KSEG0)
	or	$t1, $t0, $at
tlb_r10k_inv_l1d_loop: /* 0xbfc069d8 */
	cache	(CACHE_TYPE_L2|HIT_SET_VIRTUAL), 0($t1)
	addi	$t0, $t0, 0x10
	cache	(CACHE_TYPE_L2|HIT_SET_VIRTUAL), 1($t1)
	slt	$at, $t0, $t2
	bnez	$at, L_0xbfc069d0
	 nop
	b	L_0xbfc069f8
	 nop
L_0xbfc069f8: /* 0xbfc069f8 */
	jr	$ra
	 nop

/* Function F_0xbfc06a00 [0xbfc06a00 - 0xbfc06a48)
 */
F_0xbfc06a00: /* 0xbfc06a00 */
	addiu	$sp, $sp, -0x30
	sw	$ra, 0x2c($sp)
	sw	$at, 0x28($sp)
	sw	$t0, 0x24($sp)
	sw	$t1, 0x20($sp)
	sw	$t2, 0x1c($sp)
	sw	$t3, 0x18($sp)
	bal	F_0xbfc06710
	 sw	$t4, 0x14($sp)
	lw	$ra, 0x2c($sp)
	lw	$at, 0x28($sp)
	lw	$t0, 0x24($sp)
	lw	$t1, 0x20($sp)
	lw	$t2, 0x1c($sp)
tlb_r10k_inv_l2_loop: /* 0xbfc06a38 */
	lw	$t3, 0x18($sp)
	lw	$t4, 0x14($sp)
	jr	$ra
	 addiu	$sp, $sp, 0x30
	.fill	2, 4, 0x00000000
data_0xbfc06a50: /* 0xbfc06a50 */
	.ascii	"<post1> <SizeMEM>   \n   "
tlb_r10k_set_virt_l1d_loop: /* 0xbfc06a68 */
	.ascii	"     Bank0 = %dM \n        Bank1 = %dM \n        Bank2 = %"
tlb_r10k_set_virt_l1i_loop: /* 0xbfc06aa0 */
	.ascii	"dM \n        Bank3 = %dM \n        Bank4 = %dM \n        Bank5 = %dM \n        Bank6 = %dM \n"
tlb_r10k_set_virt_l2_loop: /* 0xbfc06af8 */
	.ascii	"        Bank7 = %dM \n"
	.fill	3, 1, 0
data_0xbfc06b10: /* 0xbfc06b10 */
	.ascii	"<post1> <SizeMEM"
tlb_r10k_exit: /* 0xbfc06b20 */
	.ascii	"> Enteri"
tlb_init_preserve: /* 0xbfc06b28 */
	.ascii	"ng routine\n"
	.byte	0
data_0xbfc06b34: /* 0xbfc06b34 */
	string 4, "<post1> <SizeMEM> ECC off \n"
data_0xbfc06b50: /* 0xbfc06b50 */
	string 4, "<post1> <SizeMEM> Initilaize BANK%d\n"
data_0xbfc06b78: /* 0xbfc06b78 */
	string 4, "<post1> <SizeMEM> mem_addr = 0x%0x\n"
data_0xbfc06b9c: /* 0xbfc06b9c */
	string 4, "<post1> <SizeMEM> bank%d =  0x%lx (32M)\n"
data_0xbfc06bc8: /* 0xbfc06bc8 */
	string 4, "<post1> <SizeMEM> bank%d = 0x%0lx (128M)\n"
data_0xbfc06bf4: /* 0xbfc06bf4 */
	string 4, "Error, no SIMM in bank0\n"
data_0xbfc06c10: /* 0xbfc06c10 */
	string 4, "<post1> <SizeMEM> bank%d = 0x%0lx, no simm installed\n"
data_0xbfc06c48: /* 0xbfc06c48 */
	string 4, "<post1> <SizeMEM> bank%d = 128M mem_addr[next] = 0x%0x\n"
data_0xbfc06c80: /* 0xbfc06c80 */
	string 4, "<post1> <SizeMEM> bank%d = 32M mem_addr[next] = 0x%0x\n"
data_0xbfc06cb8: /* 0xbfc06cb8 */
	string 4, "<post1> <SizeMEM> bank%d = 0M BankControl = 0x%0lx\n"
data_0xbfc06cec: /* 0xbfc06cec */
	string 4, "<post1> <SizeMEM> MEM Size = 0x%0x bytes\n"
data_0xbfc06d18: /* 0xbfc06d18 */
	string 4, "<post1> <CopyPOST1> ECC off \n"
data_0xbfc06d38: /* 0xbfc06d38 */
	string 4, "<post1> <CopyPOST1>\n  pass 1: Copy bootstrap 0x%0x to 0x%0x\n"
data_0xbfc06d78: /* 0xbfc06d78 */
	string 4, "<post1> <CopyPOST1>\n        pass 1: Bootstrap POST1DIAGS\n                Starting addr: 0x%0x\n                Ending   addr: 0x%0x\n                Target   addr: 0x%0x\n"
	.int	0x00000000
data_0xbfc06e28: /* 0xbfc06e28 */
	.int	0xffffffff
	.int	0x00000000
data_0xbfc06e30: /* 0xbfc06e30 */
	.int	0xfe000007
	.int	0x01fffff8
data_0xbfc06e38: /* 0xbfc06e38 */
	.int	0xfdffffff
	.int	0x02000000
data_0xbfc06e40: /* 0xbfc06e40 */
	.int	0xf8000007
	.int	0x07fffff8
data_0xbfc06e48: /* 0xbfc06e48 */
	.int	0xfdffffff
	.int	0x02000000
data_0xbfc06e50: /* 0xbfc06e50 */
	.int	0xf8000007
	.int	0x07fffff8
data_0xbfc06e58: /* 0xbfc06e58 */
	.int	0xfdffffff
	.int	0x02000000
data_0xbfc06e60: /* 0xbfc06e60 */
	.int	0xf8000007
	.int	0x07fffff8
data_0xbfc06e68: /* 0xbfc06e68 */
	.int	0xffffffff
	.int	0x00000000
data_0xbfc06e70: /* 0xbfc06e70 */
	.int	0xfe000007
	.int	0x01fffff8
data_0xbfc06e78: /* 0xbfc06e78 */
	.int	0xfdffffff
	.int	0x02000000
data_0xbfc06e80: /* 0xbfc06e80 */
	.int	0xf8000007
	.int	0x07fffff8
	.fill	2, 4, 0x00000000
data_0xbfc06e90: /* 0xbfc06e90 */
	string 4, "0123456789abcdef"
	.fill	3, 4, 0x00000000
data_0xbfc06eb0: /* 0xbfc06eb0 */
	.int	0x27bdff70
	.int	0xffa10000
	.int	0xffbf0008
	.int	0xffb00010
	.int	0xffb10018
	.int	0xffb20020
	.int	0xffb30028
	.int	0xffb40030
	.int	0xffb50038
	.int	0xffb60040
	.int	0xffb70048
	.int	0xffb70050
	.int	0x3c10a000
	.int	0x2610542c
	.int	0x0200f809
	.int	0x00000000
	.int	0xdfa10000
	.int	0xdfbf0008
	.int	0xdfb00010
	.int	0xdfb10018
	.int	0xdfb20020
	.int	0xdfb30028
	.int	0xdfb40030
	.int	0xdfb50038
	.int	0xdfb60040
	.int	0xdfb70048
	.int	0xdfb70050
	.int	0x03e00008
	.int	0x27bd0090
	.fill	3, 4, 0x00000000
	.int	0x40047800
	.int	0x00000000
	.int	0x24020023
	.int	0x00042202
	.int	0x308400ff
	.int	0x14440003
	.int	0x00000000
	.int	0x10000002
	.int	0x24020030
	.int	0x00000000
	.int	0x03e00008
	.int	0x00002025
	.int	0x40046000
	.int	0x00000000
	.int	0x24050001
	.int	0x00a02827
	.int	0x00852024
	.int	0x40846000
	.int	0x00002825
	.int	0x03e00008
	.int	0x00002025
	.int	0x40066000
	.int	0x3405ff00
	.int	0x00a02827
	.int	0x00a62824
	.int	0x00042200
	.int	0x00852825
	.int	0x34a50001
	.int	0x40856000
	.int	0x00002025
	.int	0x00002825
	.int	0x03e00008
	.int	0x00003025
	.int	0x3c02a000
	.int	0x24420000
	.int	0x1000fffd
	.int	0xfc5f0000
	.int	0x03e00008
	.int	0x00000000
	.int	0x40028000
	.int	0x24041000
	.int	0x00000000
	.int	0x00021242
	.int	0x30420007
	.int	0x00441004
	.int	0x03e00008
	.int	0x00002025
	.int	0x40028000
	.int	0x24041000
	.int	0x00000000
	.int	0x00021182
	.int	0x30420007
	.int	0x00441004
	.int	0x03e00008
	.int	0x00002025
	.int	0x40848000
	.int	0x03e00008
	.int	0x00000000
	.int	0x40028000
	.int	0x03e00008
	.int	0x00000000
	.int	0x40847000
	.int	0x03e00008
	.int	0x00000000
	.int	0x40027000
	.int	0x03e00008
	.int	0x00000000
	.int	0x4002d800
	.int	0x03e00008
	.int	0x00000000
	.int	0x40026000
	.int	0x03e00008
	.int	0x00000000
	.int	0x40026800
	.int	0x03e00008
	.int	0x00000000
	.int	0x40840000
	.int	0x00000000
	.int	0x03e00008
	.int	0x00000000
	.int	0x40820000
	.int	0x00000000
	.int	0x03e00008
	.int	0x00000000
	.int	0x40841000
	.int	0x00000000
	.int	0x03e00008
	.int	0x00000000
	.int	0x40841800
	.int	0x00000000
	.int	0x03e00008
	.int	0x00000000
	.int	0x40845000
	.int	0x00000000
	.int	0x03e00008
	.int	0x00000000
	.int	0x40842800
	.int	0x00000000
	.int	0x03e00008
	.int	0x00000000
	.int	0x03e00008
	.int	0x42000008
	.int	0x03e00008
	.int	0x42000001
	.int	0x03e00008
	.int	0x42000002
	.int	0x4084e000
	.int	0x00000000
	.int	0x03e00008
	.int	0x00000000
	.int	0x4002e000
	.int	0x00000000
	.int	0x03e00008
	.int	0x00000000
	.int	0x03e00008
	.int	0xbc910000
	.int	0x03e00008
	.int	0xbc950000
	.int	0x03e00008
	.int	0xbc990000
	.int	0x03e00008
	.int	0xbc9d0000
	.int	0x03e00008
	.int	0xbc810000
	.int	0x03e00008
	.int	0xbc850000
	.int	0x03e00008
	.int	0xbc890000
	.int	0x03e00008
	.int	0xbc8d0000
	.int	0x03e00008
	.int	0xbc940000
	.int	0x03e00008
	.int	0xbc800000
	.int	0x3c048000
	.int	0x03e00008
	.int	0xbc830000
	.int	0x03e00008
	.int	0xbc170000
	.int	0x03e00008
	.int	0xbc870000
	.int	0x03e00008
	.int	0xbc8b0000
	.int	0x00000000
	.int	0x27bdffd0
	.int	0xafbf001c
	.int	0xafb00018
	.int	0x3c0ebf31
	.int	0x0c0010e1
	.int	0xfdc00000
	.int	0x10400009
	.int	0x3c10bf31
	.int	0x36100008
	.int	0x240f0020
	.int	0x3c04000f
	.int	0xfe0f0000
	.int	0x0c0011f4
	.int	0x34844240
	.int	0x24180030
	.int	0xfe180000
	.int	0x3c10bf31
	.int	0x36100008
	.int	0x0c001125
	.int	0x24040001
	.int	0x10400007
	.int	0x24190020
	.int	0x3c04000f
	.int	0xfe190000
	.int	0x0c0011f4
	.int	0x34844240
	.int	0x24080030
	.int	0xfe080000
	.int	0x0c001125
	.int	0x24040002
	.int	0x1040000d
	.int	0xafa20020
	.int	0x24090020
	.int	0x3c04000f
	.int	0xfe090000
	.int	0x0c0011f4
	.int	0x34844240
	.int	0x240a0020
	.int	0x3c04000f
	.int	0xfe0a0000
	.int	0x0c0011f4
	.int	0x34844240
	.int	0x240b0030
	.int	0xfe0b0000
	.int	0x8fbf001c
	.int	0x8fa20020
	.int	0x8fb00018
	.int	0x03e00008
	.int	0x27bd0030
	.int	0x3c04bf39
	.int	0x24050055
	.int	0x3c06bf39
	.int	0x34c68000
	.int	0xa0850707
	.int	0x90c00707
	.int	0x90820707
	.int	0x24070055
	.int	0x00001825
	.int	0x0002103c
	.int	0x0002103f
	.int	0x10e20002
	.int	0x240800aa
	.int	0x24030001
	.int	0xa0880707
	.int	0x90c00707
	.int	0x90820707
	.int	0x240900aa
	.int	0x240a00f0
	.int	0x0002103c
	.int	0x0002103f
	.int	0x11220002
	.int	0x2418000f
	.int	0x34630002
	.int	0xa08a0707
	.int	0x90c00707
	.int	0x90800707
	.int	0xa0980707
	.int	0x90c00707
	.int	0x90800707
	.int	0x90800007
	.int	0x90800107
	.int	0x90800207
	.int	0x90800307
	.int	0x90800407
	.int	0x90800507
	.int	0x90800607
	.int	0xa0c50707
	.int	0x90800707
	.int	0x90c20707
	.int	0x0002103c
	.int	0x0002103f
	.int	0x50e20003
	.int	0xa0c80707
	.int	0x34630004
	.int	0xa0c80707
	.int	0x90800707
	.int	0x90c20707
	.int	0x0002103c
	.int	0x0002103f
	.int	0x51220003
	.int	0xa0ca0707
	.int	0x34630008
	.int	0xa0ca0707
	.int	0x90800707
	.int	0x90c00707
	.int	0xa0ca0707
	.int	0x90800707
	.int	0x90c00707
	.int	0x90c00007
	.int	0x90c00107
	.int	0x90c00207
	.int	0x90c00307
	.int	0x90c00407
	.int	0x90c00507
	.int	0x90c00607
	.int	0x03e00008
	.int	0x00601025
	.int	0x27bdfeb8
	.int	0x24010001
	.int	0xafbf0024
	.int	0xafb30020
	.int	0xafb2001c
	.int	0xafb10018
	.int	0x14810003
	.int	0xafb00014
	.int	0x10000008
	.int	0x3c12bf39
	.int	0x24010002
	.int	0x14810003
	.int	0x3c12bf39
	.int	0x10000003
	.int	0x36528000
	.int	0x100000b8
	.int	0x24020200
	.int	0x240e0080
	.int	0xa24e0307
	.int	0x240f0018
	.int	0xa24f0707
	.int	0xa2400107
	.int	0x2418000c
	.int	0xa2580007
	.int	0x24190003
	.int	0xa2590307
	.int	0x24080001
	.int	0xa2480207
	.int	0xa2400107
	.int	0x24090013
	.int	0xa2490407
	.int	0x924a0507
	.int	0x00008825
	.int	0x27ad0044
	.int	0x000a503c
	.int	0x000a503f
	.int	0x314c0001
	.int	0x1180000e
	.int	0x00009825
	.int	0x022d8021
	.int	0x27a20064
	.int	0x924e0007
	.int	0x26100001
	.int	0xa20effff
	.int	0x924f0507
	.int	0x000f783c
	.int	0x000f783f
	.int	0x31f90001
	.int	0x53200004
	.int	0x00008825
	.int	0x5602fff7
	.int	0x924e0007
	.int	0x00008825
	.int	0x92480507
	.int	0x24102710
	.int	0x0008403c
	.int	0x0008403f
	.int	0x310a0020
	.int	0x5540000e
	.int	0xa2530007
	.int	0x0c0011f4
	.int	0x24040064
	.int	0x924c0507
	.int	0x240bffff
	.int	0x020b8021
	.int	0x000c603c
	.int	0x000c603f
	.int	0x318e0020
	.int	0x55c00004
	.int	0xa2530007
	.int	0x1600fff5
	.int	0x00000000
	.int	0xa2530007
	.int	0x924f0507
	.int	0x26730001
	.int	0x327300ff
	.int	0x000f783c
	.int	0x000f783f
	.int	0x31f90001
	.int	0x13200012
	.int	0x0013983c
	.int	0x2a210100
	.int	0x1020000f
	.int	0x27a80044
	.int	0x02288021
	.int	0x92490007
	.int	0x26310001
	.int	0x2a210100
	.int	0xa2090000
	.int	0x924a0507
	.int	0x26100001
	.int	0x000a503c
	.int	0x000a503f
	.int	0x314c0001
	.int	0x51800004
	.int	0x0013983f
	.int	0x5420fff5
	.int	0x92490007
	.int	0x0013983f
	.int	0x2a6100ff
	.int	0x5420ffd1
	.int	0x92480507
	.int	0x0c0011f4
	.int	0x24042710
	.int	0x924e0507
	.int	0x2a210100
	.int	0x000e703c
	.int	0x000e703f
	.int	0x31d80001
	.int	0x53000014
	.int	0x83ac0044
	.int	0x10200011
	.int	0x27b90044
	.int	0x02398021
	.int	0x27b10144
	.int	0x92480007
	.int	0x26100001
	.int	0x24042710
	.int	0x0c0011f4
	.int	0xa208ffff
	.int	0x92490507
	.int	0x0211082b
	.int	0x0009483c
	.int	0x0009483f
	.int	0x312b0001
	.int	0x51600004
	.int	0x83ac0044
	.int	0x5420fff4
	.int	0x92480007
	.int	0x83ac0044
	.int	0x83ae0045
	.int	0x24010001
	.int	0x11800003
	.int	0x00000000
	.int	0x10000041
	.int	0x24020100
	.int	0x11c10003
	.int	0x83b80046
	.int	0x1000003d
	.int	0x24020101
	.int	0x24010002
	.int	0x13010003
	.int	0x24110003
	.int	0x10000038
	.int	0x24020102
	.int	0x27b00047
	.int	0x240200ff
	.int	0x920b0000
	.int	0x0011883c
	.int	0x0011883e
	.int	0x322900ff
	.int	0x0009483c
	.int	0x000b583c
	.int	0x000b583f
	.int	0x0009483f
	.int	0x112b0003
	.int	0x26230001
	.int	0x1000002a
	.int	0x36220100
	.int	0x92180001
	.int	0x0003183c
	.int	0x0003183e
	.int	0x306e00ff
	.int	0x000e703c
	.int	0x0018c03c
	.int	0x0018c03f
	.int	0x000e703f
	.int	0x51d80004
	.int	0x26230002
	.int	0x1000001e
	.int	0x34620100
	.int	0x26230002
	.int	0x920a0002
	.int	0x0003183c
	.int	0x0003183e
	.int	0x306900ff
	.int	0x0009483c
	.int	0x000a503c
	.int	0x000a503f
	.int	0x0009483f
	.int	0x512a0004
	.int	0x26230003
	.int	0x10000011
	.int	0x34620100
	.int	0x26230003
	.int	0x920f0003
	.int	0x0003183c
	.int	0x0003183e
	.int	0x306e00ff
	.int	0x000e703c
	.int	0x000f783c
	.int	0x000f783f
	.int	0x000e703f
	.int	0x11cf0003
	.int	0x26310004
	.int	0x10000004
	.int	0x34620100
	.int	0x1622ffcd
	.int	0x26100004
	.int	0x00001025
	.int	0x8fbf0024
	.int	0x8fb00014
	.int	0x8fb10018
	.int	0x8fb2001c
	.int	0x8fb30020
	.int	0x03e00008
	.int	0x27bd0148
	.int	0x3c02bf34
	.int	0xdc430000
	.int	0xdc580000
	.int	0x2401ffff
	.int	0x00617024
	.int	0x01c4182d
	.int	0x0301c824
	.int	0x0323082b
	.int	0x10200007
	.int	0x00000000
	.int	0xdc480000
	.int	0x2401ffff
	.int	0x01014824
	.int	0x0123082b
	.int	0x5420fffc
	.int	0xdc480000
	.int	0x03e00008
	.fill	3, 4, 0x00000000
	.int	0x1080000e
	.int	0x00801025
	.int	0x30850003
	.int	0x00052823
	.int	0x10a00006
	.int	0x00a41821
	.int	0x240effff
	.int	0x004e1021
	.int	0x5462fffe
	.int	0x240effff
	.int	0x10400004
	.int	0x240ffffc
	.int	0x004f1021
	.int	0x5440fffe
	.int	0x240ffffc
	.int	0x03e00008
	.int	0x00000000
	.int	0x3c0ebfd0
	.int	0x91cf0703
	.int	0x000f783c
	.int	0x000f783f
	.int	0x39f900de
	.int	0x2f390001
	.int	0x0019c83c
	.int	0x03e00008
	.int	0x0019103e
	.int	0x27bdffe8
	.int	0xafbf0014
	.int	0x240e0080
	.int	0x3c0fbfd0
	.int	0xa1ee0380
	.int	0x0c001208
	.int	0x24040020
	.int	0x3c18bfd0
	.int	0xa3000100
	.int	0x0c001208
	.int	0x24040020
	.int	0x24190018
	.int	0x3c08bfd0
	.int	0xa1190018
	.int	0x0c001208
	.int	0x24040020
	.int	0x24090003
	.int	0x3c0abfd0
	.int	0xa1490303
	.int	0x0c001208
	.int	0x24040020
	.int	0x3c0bbfd0
	.int	0xa1600100
	.int	0x0c001208
	.int	0x24040020
	.int	0x3c0cbfd0
	.int	0xa1800400
	.int	0x0c001208
	.int	0x24040020
	.int	0x240d00de
	.int	0x3c0ebfd0
	.int	0xa1cd07de
	.int	0x0c001208
	.int	0x24040020
	.int	0x8fbf0014
	.int	0x27bd0018
	.int	0x03e00008
	.int	0x00000000
	.int	0x3c02bfd0
	.int	0x34420503
	.int	0x904e0000
	.int	0x3c04bfd0
	.int	0x2403ffff
	.int	0x000e703c
	.int	0x000e703f
	.int	0x31d80001
	.int	0x1300000a
	.int	0x34840003
	.int	0x90990000
	.int	0x90480000
	.int	0x0019c83c
	.int	0x0008403c
	.int	0x0008403f
	.int	0x310a0001
	.int	0x0019c83f
	.int	0x1540fff8
	.int	0x332300ff
	.int	0x03e00008
	.int	0x00601025
	.int	0x3c02bfd0
	.int	0x34420503
	.int	0x904e0000
	.int	0x3c0abfd0
	.int	0x000e703c
	.int	0x000e703f
	.int	0x31d80001
	.int	0x57000008
	.int	0x914b0003
	.int	0x90590000
	.int	0x0019c83c
	.int	0x0019c83f
	.int	0x33290001
	.int	0x5120fffc
	.int	0x90590000
	.int	0x914b0003
	.int	0x000b583c
	.int	0x000b583f
	.int	0x316d007f
	.int	0x000d683c
	.int	0x000d683e
	.int	0x03e00008
	.int	0x31a200ff
	.int	0x27bdffe8
	.int	0x3c02bfd0
	.int	0x34420503
	.int	0xafbf0014
	.int	0xafa40018
	.int	0x904e0000
	.int	0x308500ff
	.int	0x0005283c
	.int	0x000e703c
	.int	0x000e703f
	.int	0x31d80020
	.int	0x17000007
	.int	0x0005283f
	.int	0x90590000
	.int	0x0019c83c
	.int	0x0019c83f
	.int	0x33290020
	.int	0x5120fffc
	.int	0x90590000
	.int	0x0005503c
	.int	0x000a503e
	.int	0x3c0dbfd0
	.int	0x014d6025
	.int	0x000c603c
	.int	0x000c603f
	.int	0xa1850000
	.int	0x0c001208
	.int	0x24040020
	.int	0x8fbf0014
	.int	0x27bd0018
	.int	0x03e00008
	.int	0x00000000
	.int	0x27bdff98
	.int	0xafb00014
	.int	0xafa40068
	.int	0xafa5006c
	.int	0xdfb00068
	.int	0xafb10018
	.int	0x00c08825
	.int	0xafbf001c
	.int	0x16000007
	.int	0xafa70074
	.int	0x54c00006
	.int	0x240e0030
	.int	0x0c001274
	.int	0x24040030
	.int	0x10000047
	.int	0x8fbf001c
	.int	0x240e0030
	.int	0x240f0030
	.int	0xa3af0051
	.int	0xa3ae0050
	.int	0x27a30052
	.int	0x27a40066
	.int	0x24020030
	.int	0x24630004
	.int	0xa062fffc
	.int	0xa062fffd
	.int	0xa062fffe
	.int	0x1464fffb
	.int	0xa062ffff
	.int	0x2e01000b
	.int	0x00003025
	.int	0x14200019
	.int	0x02002825
	.int	0x3c08a000
	.int	0x25085b40
	.int	0x27b00050
	.int	0x2407000a
	.int	0x2401000a
	.int	0x00a1001f
	.int	0x26100001
	.int	0x0000c012
	.int	0x0018103c
	.int	0x0002103f
	.int	0x00470019
	.int	0x0000c812
	.int	0x00b9502f
	.int	0x000a183c
	.int	0x0003183f
	.int	0x01035821
	.int	0x916c0000
	.int	0x00402825
	.int	0x10600002
	.int	0xa20cffff
	.int	0xafa6003c
	.int	0x2ca1000b
	.int	0x1020ffed
	.int	0x24c60001
	.int	0x3c08a000
	.int	0x0005683c
	.int	0x25085b40
	.int	0x000d683e
	.int	0x010d7021
	.int	0x91cf0000
	.int	0x03a6c021
	.int	0x10a00002
	.int	0xa30f0050
	.int	0xafa6003c
	.int	0x12200005
	.int	0x8fb90074
	.int	0x53200004
	.int	0x8fa6003c
	.int	0x10000002
	.int	0x03203025
	.int	0x8fa6003c
	.int	0x04c0000a
	.int	0x27a90050
	.int	0x00c98021
	.int	0x27b10050
	.int	0x0c001274
	.int	0x92040000
	.int	0x240affff
	.int	0x020a8021
	.int	0x0211082b
	.int	0x1020fffa
	.int	0x00000000
	.int	0x8fbf001c
	.int	0x8fb00014
	.int	0x8fb10018
	.int	0x03e00008
	.int	0x27bd0068
	.int	0x27bdff98
	.int	0xafb00014
	.int	0xafa40068
	.int	0xafa5006c
	.int	0xdfb00068
	.int	0xafbf001c
	.int	0xafb10018
	.int	0xafa60070
	.int	0x16000007
	.int	0xafa70074
	.int	0x54c00006
	.int	0x240f0030
	.int	0x0c001274
	.int	0x24040030
	.int	0x10000056
	.int	0x8fbf001c
	.int	0x240f0030
	.int	0x24180030
	.int	0xa3b80051
	.int	0xa3af0050
	.int	0x27a30052
	.int	0x27a40066
	.int	0x24020030
	.int	0x24630004
	.int	0xa062fffc
	.int	0xa062fffd
	.int	0xa062fffe
	.int	0x1464fffb
	.int	0xa062ffff
	.int	0x06010004
	.int	0x00003025
	.int	0x2411002d
	.int	0x10000003
	.int	0x0010282f
	.int	0x24110020
	.int	0x02002825
	.int	0x28a1000b
	.int	0x1420001a
	.int	0x0011883c
	.int	0x27b90050
	.int	0x3c08a000
	.int	0x25085b40
	.int	0x00d98021
	.int	0x2407000a
	.int	0x2401000a
	.int	0x00a1001e
	.int	0x26100001
	.int	0x00004812
	.int	0x0009103c
	.int	0x0002103f
	.int	0x00470019
	.int	0x00005012
	.int	0x00aa602f
	.int	0x000c183c
	.int	0x0003183f
	.int	0x01036821
	.int	0x91ae0000
	.int	0x00402825
	.int	0x10600002
	.int	0xa20effff
	.int	0xafa6003c
	.int	0x28a1000b
	.int	0x1020ffed
	.int	0x24c60001
	.int	0x3c08a000
	.int	0x0005783c
	.int	0x25085b40
	.int	0x000f783e
	.int	0x010fc021
	.int	0x93190000
	.int	0x03a64821
	.int	0x10a00002
	.int	0xa1390050
	.int	0xafa6003c
	.int	0x8faa0070
	.int	0x8fab0074
	.int	0x0011883f
	.int	0x11400005
	.int	0x2401002d
	.int	0x51600004
	.int	0x8fa6003c
	.int	0x10000002
	.int	0x01603025
	.int	0x8fa6003c
	.int	0x16210004
	.int	0x322400ff
	.int	0x0c001274
	.int	0xafa60048
	.int	0x8fa60048
	.int	0x04c0000a
	.int	0x27ad0050
	.int	0x00cd8021
	.int	0x27b10050
	.int	0x0c001274
	.int	0x92040000
	.int	0x240effff
	.int	0x020e8021
	.int	0x0211082b
	.int	0x1020fffa
	.int	0x00000000
	.int	0x8fbf001c
	.int	0x8fb00014
	.int	0x8fb10018
	.int	0x03e00008
	.int	0x27bd0068
	.int	0x27bdffc8
	.int	0xafb30024
	.int	0xafa40038
	.int	0xafa5003c
	.int	0xdfb30038
	.int	0xafb60030
	.int	0xafb1001c
	.int	0xafb00018
	.int	0x00c0b025
	.int	0xafbf0034
	.int	0xafb5002c
	.int	0xafb40028
	.int	0xafb20020
	.int	0x00e08025
	.int	0x16600007
	.int	0x00008825
	.int	0x14c00005
	.int	0x00000000
	.int	0x0c001274
	.int	0x24040030
	.int	0x10000032
	.int	0x8fbf0034
	.int	0x04e0002f
	.int	0x24150030
	.int	0x3c12a000
	.int	0x26525b40
	.int	0x24140058
	.int	0x16000007
	.int	0x02134016
	.int	0x326e000f
	.int	0x000e783c
	.int	0x000f783e
	.int	0x024fc021
	.int	0x10000006
	.int	0x93040000
	.int	0x3109000f
	.int	0x0009503c
	.int	0x000a503e
	.int	0x024a5821
	.int	0x91640000
	.int	0x0004203c
	.int	0x0004203f
	.int	0x1684000b
	.int	0x00801025
	.int	0x2841003a
	.int	0x14200008
	.int	0x240dffe0
	.int	0x004d6021
	.int	0x000c603c
	.int	0x000c603e
	.int	0x318400ff
	.int	0x0004203c
	.int	0x0004203f
	.int	0x00801025
	.int	0x12a20005
	.int	0x00000000
	.int	0x0c001274
	.int	0x26310001
	.int	0x10000008
	.int	0x240efffc
	.int	0x16c00003
	.int	0x00000000
	.int	0x52200004
	.int	0x240efffc
	.int	0x0c001274
	.int	0x00000000
	.int	0x240efffc
	.int	0x020e8021
	.int	0x0601ffd6
	.int	0x00000000
	.int	0x8fbf0034
	.int	0x8fb00018
	.int	0x8fb1001c
	.int	0x8fb20020
	.int	0x8fb30024
	.int	0x8fb40028
	.int	0x8fb5002c
	.int	0x8fb60030
	.int	0x03e00008
	.int	0x27bd0038
	.int	0x27bdffd0
	.int	0xafb30020
	.int	0xafa40030
	.int	0xafa50034
	.int	0xdfb30030
	.int	0xafb50028
	.int	0xafb10018
	.int	0xafb00014
	.int	0x00c0a825
	.int	0xafbf002c
	.int	0xafb40024
	.int	0xafb2001c
	.int	0x00e08025
	.int	0x16600007
	.int	0x00008825
	.int	0x14c00005
	.int	0x00000000
	.int	0x0c001274
	.int	0x24040030
	.int	0x10000025
	.int	0x8fbf002c
	.int	0x04e00022
	.int	0x24140030
	.int	0x3c12a000
	.int	0x26525b40
	.int	0x16000007
	.int	0x02134016
	.int	0x326e0007
	.int	0x000e783c
	.int	0x000f783e
	.int	0x024fc021
	.int	0x10000006
	.int	0x93040000
	.int	0x31090007
	.int	0x0009503c
	.int	0x000a503e
	.int	0x024a5821
	.int	0x91640000
	.int	0x0004203c
	.int	0x0004203f
	.int	0x12840005
	.int	0x00000000
	.int	0x0c001274
	.int	0x26310001
	.int	0x10000008
	.int	0x240dfffd
	.int	0x16a00003
	.int	0x00000000
	.int	0x52200004
	.int	0x240dfffd
	.int	0x0c001274
	.int	0x00000000
	.int	0x240dfffd
	.int	0x020d8021
	.int	0x0601ffe2
	.int	0x00000000
	.int	0x8fbf002c
	.int	0x8fb00014
	.int	0x8fb10018
	.int	0x8fb2001c
	.int	0x8fb30020
	.int	0x8fb40024
	.int	0x8fb50028
	.int	0x03e00008
	.int	0x27bd0030
	.int	0x27bdffd8
	.int	0xafb00018
	.int	0xafbf0024
	.int	0xafb20020
	.int	0xafb1001c
	.int	0xafa40028
	.int	0xafa5002c
	.int	0x04c00017
	.int	0x00c08025
	.int	0x3c12a000
	.int	0x26525b40
	.int	0xdfb10028
	.int	0x16000007
	.int	0x02114016
	.int	0x322e0001
	.int	0x000e783c
	.int	0x000f783e
	.int	0x024fc021
	.int	0x10000006
	.int	0x93040000
	.int	0x31090001
	.int	0x0009503c
	.int	0x000a503e
	.int	0x024a5821
	.int	0x91640000
	.int	0x0c001274
	.int	0x00000000
	.int	0x240cffff
	.int	0x020c8021
	.int	0x0601ffee
	.int	0x00000000
	.int	0x8fbf0024
	.int	0x8fb00018
	.int	0x8fb1001c
	.int	0x8fb20020
	.int	0x03e00008
	.int	0x27bd0028
	.int	0x27bdfff8
	.int	0xafa40008
	.int	0xafa5000c
	.int	0xafa60010
	.int	0xafa70014
	.int	0x03e00008
	.int	0x27bd0008
	.int	0x27bdffe8
	.int	0xafbf0014
	.int	0x3c04a000
	.int	0x0c00140d
	.int	0x24845b70
	.int	0x3c04a000
	.int	0x0c00140d
	.int	0x24845b98
	.int	0x8fbf0014
	.int	0x27bd0018
	.int	0x24020001
	.int	0x03e00008
	.int	0x00000000
	.int	0x27bdffe8
	.int	0xafbf0014
	.int	0x3c04a000
	.int	0x0c00140d
	.int	0x24845bbc
	.int	0x3c04a000
	.int	0x0c00140d
	.int	0x24845be4
	.int	0x8fbf0014
	.int	0x27bd0018
	.int	0x24020001
	.int	0x03e00008
	.int	0x00000000
	.int	0x27bdffe8
	.int	0xafbf0014
	.int	0x3c04a000
	.int	0x0c00140d
	.int	0x24845c08
	.int	0x3c04a000
	.int	0x0c00140d
	.int	0x24845c30
	.int	0x8fbf0014
	.int	0x27bd0018
	.int	0x24020001
	.int	0x03e00008
	.int	0x00000000
	.int	0x27bdffe8
	.int	0xafbf0014
	.int	0x3c04a000
	.int	0x0c00140d
	.int	0x24845c54
	.int	0x3c04a000
	.int	0x0c00140d
	.int	0x24845c7c
	.int	0x8fbf0014
	.int	0x27bd0018
	.int	0x24020001
	.int	0x03e00008
	.int	0x00000000
	.int	0x27bdffe8
	.int	0xafbf0014
	.int	0x3c04a000
	.int	0x0c00140d
	.int	0x24845ca0
	.int	0x3c04a000
	.int	0x0c00140d
	.int	0x24845cc8
	.int	0x8fbf0014
	.int	0x27bd0018
	.int	0x24020001
	.int	0x03e00008
	.int	0x00000000
	.int	0x27bdffe8
	.int	0xafbf0014
	.int	0x3c04a000
	.int	0x0c00140d
	.int	0x24845cec
	.int	0x3c04a000
	.int	0x0c00140d
	.int	0x24845d14
	.int	0x8fbf0014
	.int	0x27bd0018
	.int	0x24020001
	.int	0x03e00008
	.int	0x00000000
	.int	0x27bdffe8
	.int	0xafbf0014
	.int	0x3c04a000
	.int	0x0c00140d
	.int	0x24845d38
	.int	0x3c04a000
	.int	0x0c00140d
	.int	0x24845d60
	.int	0x8fbf0014
	.int	0x27bd0018
	.int	0x24020001
	.int	0x03e00008
	.int	0x00000000
	.int	0x27bdffe8
	.int	0xafbf0014
	.int	0x3c04a000
	.int	0x0c00140d
	.int	0x24845d84
	.int	0x3c04a000
	.int	0x0c00140d
	.int	0x24845dac
	.int	0x8fbf0014
	.int	0x27bd0018
	.int	0x24020001
	.int	0x03e00008
	.int	0x00000000
	.int	0x27bdffe8
	.int	0xafbf0014
	.int	0x3c04a000
	.int	0x0c00140d
	.int	0x24845dd0
	.int	0x3c04a000
	.int	0x0c00140d
	.int	0x24845df8
	.int	0x8fbf0014
	.int	0x27bd0018
	.int	0x24020001
	.int	0x03e00008
	.int	0x00000000
	.int	0x27bdffe8
	.int	0xafbf0014
	.int	0x3c04a000
	.int	0x0c00140d
	.int	0x24845e1c
	.int	0x3c04a000
	.int	0x0c00140d
	.int	0x24845e44
	.int	0x8fbf0014
	.int	0x27bd0018
	.int	0x24020001
	.int	0x03e00008
	.int	0x00000000
	.int	0x27bdffe8
	.int	0xafbf0014
	.int	0x3c04a000
	.int	0x0c00140d
	.int	0x24845e6c
	.int	0x3c04a000
	.int	0x0c00140d
	.int	0x24845e94
	.int	0x8fbf0014
	.int	0x27bd0018
	.int	0x24020001
	.int	0x03e00008
	.int	0x00000000
	.int	0x27bdffd8
	.int	0xafbf0024
	.int	0xafb30020
	.int	0xafb2001c
	.int	0xafb10018
	.int	0xafb00014
	.int	0x00002025
	.int	0x00002825
	.int	0x0c00102c
	.int	0x00003025
	.int	0x3c0eb400
	.int	0x35ce0200
	.int	0x3c0fb400
	.int	0xfdc00000
	.int	0x35ef0018
	.int	0x3c18b400
	.int	0xfde00000
	.int	0x37180020
	.int	0x3c19b400
	.int	0xff000000
	.int	0x37390028
	.int	0x3c08b400
	.int	0xff200000
	.int	0x35080050
	.int	0xfd000000
	.int	0x00009025
	.int	0x24100015
	.int	0x0c001020
	.int	0x00002025
	.int	0x00409825
	.int	0x0c001080
	.int	0x00002025
	.int	0x1a600013
	.int	0x00008825
	.int	0x0011883c
	.int	0x0011883e
	.int	0x0c00106c
	.int	0x02202025
	.int	0x0c00107c
	.int	0x02402025
	.int	0x0c001074
	.int	0x02002025
	.int	0x26100040
	.int	0x0c001078
	.int	0x02002025
	.int	0x0c001088
	.int	0x00000000
	.int	0x26310001
data_0xbfc081fc: /* 0xbfc081fc */
	string 4, "&R "
	.int	0x1633fff0
	.int	0x26100040
	.int	0x00008825
	.int	0x0c00104f
	.int	0x00002025
	.int	0x0002103c
	.int	0x0002983f
	.int	0x1a60000f
	.int	0x3c128000
	.int	0x0011883c
	.int	0x0011883e
	.int	0x02328025
	.int	0x0c0010a4
	.int	0x02002025
	.int	0x00102302
	.int	0x0c00108a
	.int	0x00042200
	.int	0x0c00109e
	.int	0x02002025
	.int	0x26310020
	.int	0x0233082a
	.int	0x5420fff4
	.int	0x0011883c
	.int	0x0c00105a
	.int	0x00000000
	.int	0x00025380
	.int	0x05410008
	.int	0x00408025
	.int	0x0c001057
	.int	0x34441000
	.int	0x0c0010a6
	.int	0x00002025
	.int	0x240befff
	.int	0x0c001057
	.int	0x020b2024
	.int	0x3c04a000
	.int	0x0c00140d
	.int	0x24845ebc
	.int	0x8fbf0024
	.int	0x8fb00014
	.int	0x8fb10018
	.int	0x8fb2001c
	.int	0x8fb30020
	.int	0x03e00008
	.int	0x27bd0028
	.int	0x27bdffe8
	.int	0xafbf0014
	.int	0x3c04a000
	.int	0x0c00140d
	.int	0x24845ef4
	.int	0x8fbf0014
	.int	0x27bd0018
	.int	0x2402ffff
	.int	0x03e00008
	.int	0x00000000
	.int	0x27bdffd8
	.int	0xafbf0024
	.int	0xafb10018
	.int	0x3c04a000
	.int	0xafb30020
	.int	0xafb2001c
	.int	0xafb00014
	.int	0x00008825
	.int	0x0c00140d
	.int	0x24845f24
	.int	0x3c10a000
	.int	0x3c12a000
	.int	0x26525f44
	.int	0x26105b60
	.int	0x2413ffff
	.int	0x0c0014a3
	.int	0x00000000
	.int	0x02402025
	.int	0x0c00140d
	.int	0x02202825
	.int	0x8e190000
	.int	0x0320f809
	.int	0x00000000
	.int	0x26310001
	.int	0x1453fff6
	.int	0x26100004
	.int	0x0c0014a3
	.int	0x00000000
	.int	0x8fbf0024
	.int	0x8fb00014
	.int	0x8fb10018
	.int	0x8fb2001c
	.int	0x8fb30020
	.int	0x27bd0028
	.int	0x03e00008
	.int	0x24020001
	.int	0x27bdffd0
	.int	0xafb10018
	.int	0x00808825
	.int	0xafbf002c
	.int	0xafb50028
	.int	0xafb2001c
	.int	0x3c04a000
	.int	0x00c09025
	.int	0x00a0a825
	.int	0xafb40024
	.int	0xafb30020
	.int	0xafb00014
	.int	0x0c00140d
	.int	0x24845f64
	.int	0x02208025
	.int	0x1aa00030
	.int	0x0000a025
	.int	0x3c118000
	.int	0x00102302
	.int	0x0c00108a
	.int	0x00042200
	.int	0x02309825
	.int	0x0c00109e
	.int	0x02602025
	.int	0x0c0010a0
	.int	0x02602025
	.int	0x02121826
	.int	0x00607027
	.int	0x02305825
	.int	0x000b583c
	.int	0x000ec03c
	.int	0x36040008
	.int	0x03035025
	.int	0x000b583f
	.int	0x00922826
	.int	0xfd6a0000
	.int	0x02245825
	.int	0x00a06827
	.int	0x000d783c
	.int	0x000b583c
	.int	0x000b583f
	.int	0x01e5c825
	.int	0x36060010
	.int	0x00d23826
	.int	0xfd790000
	.int	0x02265825
	.int	0x00e06027
	.int	0x000c703c
	.int	0x000b583c
	.int	0x000b583f
	.int	0x01c7c025
	.int	0x36080018
	.int	0x01124826
	.int	0xfd780000
	.int	0x02285825
	.int	0x01205027
	.int	0x000a683c
	.int	0x000b583c
	.int	0x26940001
	.int	0x000b583f
	.int	0x01a97825
	.int	0x26100020
	.int	0x1695ffd3
	.int	0xfd6f0000
	.int	0x3c04a000
	.int	0x0c00140d
	.int	0x24845f8c
	.int	0x8fbf002c
	.int	0x8fb00014
	.int	0x8fb10018
	.int	0x8fb2001c
	.int	0x8fb30020
	.int	0x8fb40024
	.int	0x8fb50028
	.int	0x03e00008
	.int	0x27bd0030
	.int	0x27bdffd8
	.int	0xafb2001c
	.int	0x00809025
	.int	0xafbf0024
	.int	0xafb30020
	.int	0xafb00014
	.int	0x3c04a000
	.int	0x00c08025
	.int	0x00a09825
	.int	0xafb10018
	.int	0x0c00140d
	.int	0x24845fb4
	.int	0x02402025
	.int	0x02602825
	.int	0x0c00152f
	.int	0x02003025
	.int	0x02408025
	.int	0x1a600007
	.int	0x00008825
	.int	0x3c128000
	.int	0x0c001094
	.int	0x02502025
	.int	0x26310001
	.int	0x1633fffc
	.int	0x26100020
	.int	0x3c04a000
	.int	0x0c00140d
	.int	0x24845fdc
	.int	0x8fbf0024
	.int	0x8fb00014
	.int	0x8fb10018
	.int	0x8fb2001c
	.int	0x8fb30020
	.int	0x03e00008
	.int	0x27bd0028
	.int	0x27bdff78
	.int	0x00803825
	.int	0xafbf001c
	.int	0xafb00018
	.int	0x3c04a000
	.int	0x00a08025
	.int	0xafa60090
	.int	0x24846004
	.int	0x0c00140d
	.int	0xafa70088
	.int	0x9fa70088
	.int	0x1a000033
	.int	0x00005825
	.int	0x9fb80090
	.int	0x3c0f8000
	.int	0x00ef7025
	.int	0x000e703c
	.int	0x00f86826
	.int	0x01a0c827
	.int	0x000e103f
	.int	0x0019703c
	.int	0x01cd1825
	.int	0x34ff0008
	.int	0x3c0e8000
	.int	0x03f8c826
	.int	0xafb9004c
	.int	0x03ee7825
	.int	0x9fae004c
	.int	0x000f783c
	.int	0x000f203f
	.int	0x01c07827
	.int	0x000f783c
	.int	0x01ee2825
	.int	0x34ee0010
	.int	0xafae003c
	.int	0x9faf003c
	.int	0x3c0e8000
	.int	0x01eec825
	.int	0x0019c83c
	.int	0x0019303f
	.int	0x01f87026
	.int	0x01c0c827
	.int	0x0019c83c
	.int	0x032e4025
	.int	0xafae0034
	.int	0x34ee0018
	.int	0xafae0024
	.int	0x9fb90024
	.int	0x03387826
	.int	0x01e07027
	.int	0x000e703c
	.int	0x01cf4825
	.int	0x3c0e8000
	.int	0xafaf0020
	.int	0x032e7825
	.int	0x000f783c
	.int	0x000f503f
	.int	0xfc430000
	.int	0xfc850000
	.int	0x256b0001
	.int	0xfcc80000
	.int	0x1570fffb
	.int	0xfd490000
	.int	0x3c04a000
	.int	0x0c00140d
	.int	0x24846028
	.int	0x8fbf001c
	.int	0x8fb00018
	.int	0x27bd0088
	.int	0x03e00008
	.int	0x00000000
	.int	0x27bdffc0
	.int	0xafb30024
	.int	0x00809825
	.int	0xafbf003c
	.int	0xafbe0038
	.int	0xafb40028
	.int	0x3c04a000
	.int	0x00c0a025
	.int	0x00a0f025
	.int	0xafb70034
	.int	0xafb60030
	.int	0xafb5002c
	.int	0xafb20020
	.int	0xafb1001c
	.int	0xafb00018
	.int	0x0c00140d
	.int	0x24846050
	.int	0x1bc00028
	.int	0x0000a825
	.int	0x0013b302
	.int	0x0016b200
	.int	0x3c0e8000
	.int	0x026eb825
	.int	0x36d600c0
	.int	0x24120020
	.int	0x3c118000
	.int	0x0c00109c
	.int	0x02e02025
	.int	0x0c00108e
	.int	0x00000000
	.int	0x00027942
	.int	0x000fc140
	.int	0x52d80004
	.int	0x00008025
	.int	0x0c001041
	.int	0x00000000
	.int	0x00008025
	.int	0x0010803c
	.int	0x0010803e
	.int	0x02131025
	.int	0x02224025
	.int	0x0008403c
	.int	0x0008403f
	.int	0x00541826
	.int	0xdd040000
	.int	0x00605027
	.int	0x000a603c
	.int	0x01837025
	.int	0x508e0004
	.int	0x26100008
	.int	0x0c001041
	.int	0x00000000
	.int	0x26100008
	.int	0x5612fff0
	.int	0x0010803c
	.int	0x26b50001
	.int	0x16beffe1
	.int	0x00000000
	.int	0x3c04a000
	.int	0x0c00140d
	.int	0x2484607c
	.int	0x8fbf003c
	.int	0x8fb00018
	.int	0x8fb1001c
	.int	0x8fb20020
	.int	0x8fb30024
	.int	0x8fb40028
	.int	0x8fb5002c
	.int	0x8fb60030
	.int	0x8fb70034
	.int	0x8fbe0038
	.int	0x03e00008
	.int	0x27bd0040
	.int	0x27bdff88
	.int	0xafb1001c
	.int	0x00808825
	.int	0xafbf003c
	.int	0xafb70034
	.int	0xafb20020
	.int	0x3c04a000
	.int	0x00c09025
	.int	0x00a0b825
	.int	0xafbe0038
	.int	0xafb60030
	.int	0xafb5002c
	.int	0xafb40028
	.int	0xafb30024
	.int	0xafb00018
	.int	0x0c00140d
	.int	0x248460a8
	.int	0x1ae0002e
	.int	0x0000a825
	.int	0x00117cc2
	.int	0x000fc3c0
	.int	0x37191000
	.int	0x3c0e8000
	.int	0x0011f302
	.int	0x001ef200
	.int	0x022eb025
	.int	0xafb90044
	.int	0x24140020
	.int	0x3c138000
	.int	0x0c0010ab
	.int	0x02c02025
	.int	0x0c00108e
	.int	0x00000000
	.int	0x9fa80044
	.int	0x10480003
	.int	0x00000000
	.int	0x0c001041
	.int	0x00000000
	.int	0x0c00108a
	.int	0x03c02025
	.int	0x0c00109e
	.int	0x02c02025
	.int	0x00008025
	.int	0x0010803c
	.int	0x0010803e
	.int	0x02111025
	.int	0x02625025
	.int	0x000a503c
	.int	0x000a503f
	.int	0x00521826
	.int	0xdd440000
	.int	0x00606027
	.int	0x000c703c
	.int	0x01c3c025
	.int	0x50980004
	.int	0x26100008
	.int	0x0c001041
	.int	0x00000000
	.int	0x26100008
	.int	0x5614fff0
	.int	0x0010803c
	.int	0x26b50001
	.int	0x16b7ffde
	.int	0x00000000
	.int	0x3c04a000
	.int	0x0c00140d
	.int	0x248460d4
	.int	0x8fbf003c
	.int	0x8fb00018
	.int	0x8fb1001c
	.int	0x8fb20020
	.int	0x8fb30024
	.int	0x8fb40028
	.int	0x8fb5002c
	.int	0x8fb60030
	.int	0x8fb70034
	.int	0x8fbe0038
	.int	0x03e00008
	.int	0x27bd0078
	.int	0x27bdffc8
	.int	0xafb30024
	.int	0x00809825
	.int	0xafbf0034
	.int	0xafb60030
	.int	0xafb40028
	.int	0x3c04a000
	.int	0x00c0a025
	.int	0x00a0b025
	.int	0xafb5002c
	.int	0xafb20020
	.int	0xafb1001c
	.int	0xafb00018
	.int	0x0c00140d
	.int	0x24846100
	.int	0x1ac00019
	.int	0x0000a825
	.int	0x24120020
	.int	0x3c118000
	.int	0x00008025
	.int	0x0010803c
	.int	0x0010803e
	.int	0x02131025
	.int	0x02227825
	.int	0x000f783c
	.int	0x000f783f
	.int	0x00541826
	.int	0xdde40000
	.int	0x0060c827
	.int	0x0019483c
	.int	0x01235825
	.int	0x508b0004
	.int	0x26100008
	.int	0x0c001041
	.int	0x00000000
	.int	0x26100008
	.int	0x5612fff0
	.int	0x0010803c
	.int	0x26b50001
	.int	0x56b6ffec
	.int	0x00008025
	.int	0x3c04a000
	.int	0x0c00140d
	.int	0x24846128
	.int	0x8fbf0034
	.int	0x8fb00018
	.int	0x8fb1001c
	.int	0x8fb20020
	.int	0x8fb30024
	.int	0x8fb40028
	.int	0x8fb5002c
	.int	0x8fb60030
	.int	0x03e00008
	.int	0x27bd0038
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
data_0xbfc089f0: /* 0xbfc089f0 */
	string 4, "0123456789abcdef"
	.fill	3, 4, 0x00000000
	.int	0xa00042c0
	.int	0xa0005404
	.fill	2, 4, 0x00000000
data_0xbfc08a20: /* 0xbfc08a20 */
	string 4, "<post1> <mem_scache1> Entering test\n"
data_0xbfc08a48: /* 0xbfc08a48 */
	string 4, "<post1> <mem_scache1> Leaving test\n"
data_0xbfc08a6c: /* 0xbfc08a6c */
	string 4, "<post1> <mem_scache2> Entering test\n"
data_0xbfc08a94: /* 0xbfc08a94 */
	string 4, "<post1> <mem_scache2> Leaving test\n"
data_0xbfc08ab8: /* 0xbfc08ab8 */
	string 4, "<post1> <mem_scache3> Entering test\n"
data_0xbfc08ae0: /* 0xbfc08ae0 */
	string 4, "<post1> <mem_scache3> Leaving test\n"
data_0xbfc08b04: /* 0xbfc08b04 */
	string 4, "<post1> <mem_scache4> Entering test\n"
data_0xbfc08b2c: /* 0xbfc08b2c */
	string 4, "<post1> <mem_scache4> Leaving test\n"
data_0xbfc08b50: /* 0xbfc08b50 */
	string 4, "<post1> <mem_scache5> Entering test\n"
data_0xbfc08b78: /* 0xbfc08b78 */
	string 4, "<post1> <mem_scache5> Leaving test\n"
data_0xbfc08b9c: /* 0xbfc08b9c */
	string 4, "<post1> <mem_scache6> Entering test\n"
data_0xbfc08bc4: /* 0xbfc08bc4 */
	string 4, "<post1> <mem_scache6> Leaving test\n"
data_0xbfc08be8: /* 0xbfc08be8 */
	string 4, "<post1> <mem_scache7> Entering test\n"
data_0xbfc08c10: /* 0xbfc08c10 */
	string 4, "<post1> <mem_scache7> Leaving test\n"
data_0xbfc08c34: /* 0xbfc08c34 */
	string 4, "<post1> <mem_scache8> Entering test\n"
data_0xbfc08c5c: /* 0xbfc08c5c */
	string 4, "<post1> <mem_scache8> Leaving test\n"
data_0xbfc08c80: /* 0xbfc08c80 */
	string 4, "<post1> <mem_scache9> Entering test\n"
data_0xbfc08ca8: /* 0xbfc08ca8 */
	string 4, "<post1> <mem_scache9> Leaving test\n"
data_0xbfc08ccc: /* 0xbfc08ccc */
	string 4, "<post1> <mem_scache91> Entering test\n"
data_0xbfc08cf4: /* 0xbfc08cf4 */
	string 4, "<post1> <mem_scache91> Leaving test\n"
data_0xbfc08d1c: /* 0xbfc08d1c */
	string 4, "<post1> <mem_scache10> Entering test\n"
data_0xbfc08d44: /* 0xbfc08d44 */
	string 4, "<post1> <mem_scache10> Leaving test\n"
data_0xbfc08d6c: /* 0xbfc08d6c */
	string 4, "<post1> <setUPmh> setting up diags exec environment\n"
data_0xbfc08da4: /* 0xbfc08da4 */
	string 4, "<post1> <endOFdiags> This is the end of post1\n"
data_0xbfc08dd4: /* 0xbfc08dd4 */
	string 4, "<post1> <post1diags> .......\n"
data_0xbfc08df4: /* 0xbfc08df4 */
	string 4, "<post1> <post1diags> diags%d\n"
data_0xbfc08e14: /* 0xbfc08e14 */
	string 4, "<post1> <pcache_init> Enter routine.\n"
data_0xbfc08e3c: /* 0xbfc08e3c */
	string 4, "<post1> <pcache_init> Leaving routine.\n"
data_0xbfc08e64: /* 0xbfc08e64 */
	string 4, "<post1> <scache_init> Enter routine.\n"
data_0xbfc08e8c: /* 0xbfc08e8c */
	string 4, "<post1> <scache_init> Leaving routine.\n"
data_0xbfc08eb4: /* 0xbfc08eb4 */
	string 4, "<post1> <mem_init> Enter routine.\n"
data_0xbfc08ed8: /* 0xbfc08ed8 */
	string 4, "<post1> <mem_init> Leaving routine.\n"
data_0xbfc08f00: /* 0xbfc08f00 */
	string 4, "<post1> <pcache_verify> Entering routine.\n"
data_0xbfc08f2c: /* 0xbfc08f2c */
	string 4, "<post1> <pcache_verify> Leaving routine.\n"
data_0xbfc08f58: /* 0xbfc08f58 */
	string 4, "<post1> <scache_verify> Entering routine.\n"
data_0xbfc08f84: /* 0xbfc08f84 */
	string 4, "<post1> <scache_verify> Leaving routine.\n"
data_0xbfc08fb0: /* 0xbfc08fb0 */
	string 4, "<post1> <mem_verify> Entering routine.\n"
data_0xbfc08fd8: /* 0xbfc08fd8 */
	string 4, "<post1> <mem_verify> Leaving routine.\n"
data_0xbfc09000: /* 0xbfc09000 */
	.int	0x00000019
	.int	0xdeadbeef
	.fill	2, 4, 0x00000000
data_0xbfc09010: /* 0xbfc09010 */
	.int	BASE_RENDER
	.fill	3, 4, 0x00000000

	section_checksum
	section_end post1
