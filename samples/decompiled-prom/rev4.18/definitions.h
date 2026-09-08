/* Auto-generated definitions header */
#ifndef _DEFINITIONS_H_
#define _DEFINITIONS_H_

/* ELF Header Constants */
#define ELFCLASS32			1
#define ELFDATA2MSB			2
#define EV_CURRENT			1
#define ELFOSABI_NONE			0
#define EM_MIPS				8

/* MIPS ELF e_flags */
#define EF_MIPS_NOREORDER		0x00000001
#define EF_MIPS_PIC			0x00000002
#define EF_MIPS_ARCH_2			0x10000000

/* Memory Segments */
#define KUSEG				0x00000000
#define KSEG0				0x80000000
#define KSEG1				0xa0000000
#define KSEG2				0xc0000000

/* Register Access */
#define LO32_OFFSET			0x04	/* Offset to low 32 bits of 64-bit register (big-endian) */

/* Low Memory Areas */
#define ARCS_SPB			0xa0001000	/* ARCS System Parameter Block */

/* ARCS SPB Field Offsets */
#define ARCS_SPB_OFFSET_SIGNATURE	0x00000000	/* SPB Signature */
#define ARCS_SPB_OFFSET_LENGTH		0x00000004	/* SPB Length */
#define ARCS_SPB_OFFSET_VERSION		0x00000008	/* Version */
#define ARCS_SPB_OFFSET_REVISION	0x0000000a	/* Revision */
#define ARCS_SPB_OFFSET_RESTART_BLOCK	0x0000000c	/* Pointer to Restart Block */
#define ARCS_SPB_OFFSET_DEBUG_BLOCK	0x00000010	/* Pointer to Debug Block */
#define ARCS_SPB_OFFSET_GE_VECTOR	0x00000014	/* GEVector */
#define ARCS_SPB_OFFSET_UTLB_MISS_VECTOR	0x00000018	/* UTLBMiss Vector */
#define ARCS_SPB_OFFSET_FIRMWARE_VECTOR_LENGTH	0x0000001c	/* Firmware Vector Length */
#define ARCS_SPB_OFFSET_FIRMWARE_VECTOR	0x00000020	/* Pointer to Firmware Vector */
#define ARCS_SPB_OFFSET_PRIVATE_VECTOR_LENGTH	0x00000024	/* Private Vector Length */
#define ARCS_SPB_OFFSET_PRIVATE_VECTOR	0x00000028	/* Pointer to Private Vector */
#define ARCS_SPB_OFFSET_ADAPTER_COUNT	0x0000002c	/* Adapter Count */

/* Restart Block Field Offsets */
#define RTSB_OFFSET_SIGNATURE		0x00000000	/* RSTB Signature */
#define RTSB_OFFSET_LENGTH		0x00000004	/* RSTB Length */
#define RTSB_OFFSET_VERSION		0x00000008	/* Version */
#define RTSB_OFFSET_REVISION		0x0000000a	/* Revision */
#define RTSB_OFFSET_NEXT_RSTB		0x0000000c	/* Pointer to Next Restart Block */
#define RTSB_OFFSET_RESTART_ADDRESS	0x00000010	/* Restart Address */
#define RTSB_OFFSET_BOOT_MASTER_ID	0x00000014	/* Boot Master ID */
#define RTSB_OFFSET_PROCESSOR_ID	0x00000018	/* Processor ID */
#define RTSB_OFFSET_BOOT_STATUS		0x0000001c	/* Boot Status */
#define RTSB_OFFSET_CHECKSUM		0x00000020	/* Checksum */
#define RTSB_OFFSET_SAVE_AREA_LENGTH	0x00000024	/* Save Area Length */
#define RTSB_OFFSET_SAVED_STATE_AREA	0x00000028	/* Saved State Area */

#define FIRMWARE_VECTOR			0xa0001800	/* Firmware vector table */
#define PRIVATE_VECTOR			0xa0001c00	/* Private vector table */

#define FV_OFFSET_LOAD			0x00000000
#define FV_OFFSET_INVOKE		0x00000004
#define FV_OFFSET_EXECUTE		0x00000008
#define FV_OFFSET_HALT			0x0000000c
#define FV_OFFSET_POWER_DOWN		0x00000010
#define FV_OFFSET_RESTART		0x00000014
#define FV_OFFSET_REBOOT		0x00000018
#define FV_OFFSET_ENTER_INTERACTIVE_MODE	0x0000001c
#define FV_OFFSET_GET_PEER		0x00000024
#define FV_OFFSET_GET_CHILD		0x00000028
#define FV_OFFSET_GET_PARENT		0x0000002c
#define FV_OFFSET_GET_CONFIGURATION_DATA	0x00000030
#define FV_OFFSET_ADD_CHILD		0x00000034
#define FV_OFFSET_DELETE_COMPONENT	0x00000038
#define FV_OFFSET_GET_COMPONENT		0x0000003c
#define FV_OFFSET_SAVE_CONFIGURATION	0x00000040
#define FV_OFFSET_GET_SYSTEM_ID		0x00000044
#define FV_OFFSET_GET_MEMORY_DESCRIPTOR	0x00000048
#define FV_OFFSET_GET_TIME		0x00000050
#define FV_OFFSET_GET_RELATIVE_TIME	0x00000054
#define FV_OFFSET_GET_DIRECTORY_ENTRY	0x00000058
#define FV_OFFSET_OPEN			0x0000005c
#define FV_OFFSET_CLOSE			0x00000060
#define FV_OFFSET_READ			0x00000064
#define FV_OFFSET_GET_READ_STATUS	0x00000064
#define FV_OFFSET_WRITE			0x0000006c
#define FV_OFFSET_SEEK			0x00000070
#define FV_OFFSET_MOUNT			0x00000074
#define FV_OFFSET_GET_ENVIRONMENT_VARIABLE	0x00000078
#define FV_OFFSET_SET_ENVIRONMENT_VARIABLE	0x0000007c
#define FV_OFFSET_GET_FILE_INFORMATION	0x00000080
#define FV_OFFSET_SET_FILE_INFORMATION	0x00000084
#define FV_OFFSET_FLUSH_ALL_CACHES	0x00000088
#define FV_OFFSET_TEST_UNICODE_CHARACTER	0x0000008c
#define FV_OFFSET_GET_DISPLAY_STATUS	0x00000090

/* SHDR Constants */
#define SHDR_SIZE			64
#define SHDR_OFFSET_MAGIC		8
#define SHDR_OFFSET_SECTION_LEN		12
#define SHDR_OFFSET_NAME_LEN		16
#define SHDR_OFFSET_VERSION_LEN		17
#define SHDR_OFFSET_SECTION_TYPE	18
#define SHDR_OFFSET_NAME		20
#define SHDR_OFFSET_VERSION		52
#define SHDR_OFFSET_CHECKSUM		60
#define SHDR_OFFSET_SUBSECTION_HEADER	64
#define SUBSECTION_HEADER_SIZE		8
#define SUBSECTION_HEADER_OFFSET_ADDR	0
#define SUBSECTION_HEADER_OFFSET_LEN	4

/* Section Types */
#define SECTION_TYPE_DATA		0
#define SECTION_TYPE_CODE		1
#define SECTION_TYPE_LOADABLE		2

/* Magic Numbers */
#define ARCS_MAGIC			0x53435241	/* "ARCS" */
#define ELF_MAGIC			0x7f454c46	/* "\x7fELF" */
#define GDA_MAGIC			0x58464552	/* "XFER" */
#define RTSB_MAGIC			0x42545352	/* "RTSB" (Restart Block) */
#define SGI_LABEL_MAGIC			0x0be5a941	/* SGI disk partition label */
#define SHDR_MAGIC			0x53484452	/* "SHDR" */

#define WARM_START_COOKIE		0x00007d83

/* Sentinel Values */
#define HEXDIGIT_INVALID		999999	/* 0x000f423f */

/* Hardware Constants */
#define UART_BASE_CLOCK			1843200	/* 1.8432 MHz */

/* Global Data Area address */
#define GDA_ADDR			(KSEG0 | 0x400)

/* Stack addresses */
#define SLOADER_STACK			(KSEG0 | 0x1000)
#define POST1_STACK			(KSEG1 | 0x1000)

/* ctype Table Offsets */
#define CTYPE_TOLOWER			0x102

/* Time Constants */
#define SECONDS_IN_1_DAY		86400
#define SECONDS_IN_365_DAYS		31536000
#define SECONDS_IN_366_DAYS		31622400
#define EPOC_1970			1970

/* TLB Constants */
#define PAGE_SIZE			4096
#define PAGE_SHIFT			12
#define PAGE_OFFSET_MASK		0x1fff	/* TLB entry offset mask (covers two 4KB pages) */
#define R5000_NUM_TLB_ENTRIES		48
#define RM7000_NUM_TLB_ENTRIES		48
#define R10000_NUM_TLB_ENTRIES		64

/* General Purpose Registers */
#define zero				0
#define at				1
#define v0				2
#define v1				3
#define a0				4
#define a1				5
#define a2				6
#define a3				7
#define t0				8
#define t1				9
#define t2				10
#define t3				11
#define t4				12
#define t5				13
#define t6				14
#define t7				15
#define s0				16
#define s1				17
#define s2				18
#define s3				19
#define s4				20
#define s5				21
#define s6				22
#define s7				23
#define t8				24
#define t9				25
#define k0				26
#define k1				27
#define gp				28
#define sp				29
#define fp				30
#define ra				31

/* CP0 (System Control Coprocessor) Registers */
#define CP0_INDEX			0
#define CP0_RANDOM			1
#define CP0_ENTRYLO0			2
#define CP0_ENTRYLO1			3
#define CP0_CONTEXT			4
#define CP0_PAGEMASK			5
#define CP0_WIRED			6
#define CP0_INFO			7
#define CP0_BADVADDR			8
#define CP0_COUNT			9
#define CP0_ENTRYHI			10
#define CP0_COMPARE			11
#define CP0_STATUS			12
#define CP0_CAUSE			13
#define CP0_EPC				14
#define CP0_PRID			15
#define CP0_CONFIG			16
#define CP0_LLADDR			17
#define CP0_WATCHLO			18
#define CP0_WATCHHI			19
#define CP0_XCONTEXT			20
#define CP0_FRAMEMASK			21
#define CP0_DIAGNOSTIC			22
#define CP0_DEBUG			23
#define CP0_DEPC			24
#define CP0_PERFORMANCE			25
#define CP0_ECC				26
#define CP0_CACHEERR			27
#define CP0_TAGLO			28
#define CP0_TAGHI			29
#define CP0_ERROREPC			30
#define CP0_DESAVE			31

/* CP0_STATUS bits */
#define ST0_IE				(1 <<  0)
#define ST0_EXL				(1 <<  1)
#define ST0_KX				(1 <<  7)
#define ST0_IM				0x0000ff00
#define ST0_DE				(1 << 16)
#define ST0_CH				(1 << 18)
#define ST0_NMI				(1 << 19)
#define ST0_SR				(1 << 20)
#define ST0_BEV				(1 << 22)
#define ST0_FR				(1 << 26)
#define ST0_CU0				(1 << 28)
#define ST0_CU1				(1 << 29)

/* CP0_CAUSE bits */
#define CAUSE_EXCCODE			0x7c

/* Exception codes (ExcCode field values, already shifted) */
#define EXC_BP				0x24	/* Breakpoint */

/* CP0_ENTRYLO0/1 bits */
#define ENTRYLO_G			(1 << 0)	/* Global */
#define ENTRYLO_C_UNCACHED		(2 << 3)
#define ENTRYLO_PFN_SHIFT		6	/* Page frame number shift */

/* CP0_CONFIG bits */
#define CONF_CM_CACHABLE_NONCOHERENT	3
#define CONF_CM_CMASK			7
#define CONF_CU				( 1u <<  3)
#define CONF_DB				( 1u <<  4)
#define CONF_IB				( 1u <<  5)
#define CONF_DC				( 7u <<  6)
#define CONF_DC_SHIFT			6
#define CONF_IC				( 7u <<  9)
#define CONF_IC_SHIFT			9
#define CONF_CACHE_SIZE_MASK		7
#define CONF_SC				( 1u << 17)
#define CONF_SB				( 3u << 22)

/* R5000-specific CONFIG bits */
#define R5K_CONF_SE			( 1u << 12)
#define R5K_CONF_SS			( 3u << 20)

/* R5000-specific CP0_TAGLO bits */
#define R5K_TAGLO_PTAG_SHIFT		8
#define R5K_TAGLO_DIRTY			0xc0

/* RM7000-specific CONFIG bits */
#define RM7K_CONF_TE			( 1u << 12)

/* RM7000-specific CP0_TAGLO bits */
#define RM7K_TAGLO_DIRTY		0xc0

/* RM7000-specific CP0_TAGHI bits */
#define RM7K_TAGHI_PTAG_SHIFT		8

/* R10000-specific CONFIG bits */
#define R10K_CONF_SS_SHIFT		16

/* R10000 cache block sizes */
#define R10K_L1I_BLOCK_SIZE		0x40
#define R10K_L1D_BLOCK_SIZE		0x20
#define R10K_L2_BLOCK_SIZE		0x10

/* R10000-specific CP0_TAGLO bits */
#define R10K_TAGLO_DIRTY		0xc8
#define R10K_L2_TAGLO_DIRTY		0xc00

/* CP0_PRID constants */
#define PRID_REV_MASK			0x00ff
#define PRID_IMP_MASK			0xff00
#define PRID_IMP_SHIFT			8
#define PRID_IMP_R4000			0x04
#define PRID_IMP_R4600			0x20
#define PRID_IMP_R4700			0x21
#define PRID_IMP_R5000			0x23
#define PRID_IMP_RM7000			0x27
#define PRID_IMP_NEVADA			0x28

/* CP1 (FPU) Control Registers */
#define CP1_FEIR			30
#define CP1_FCSR			31

/* Cache Operation Constants */
#define CACHE_TYPE_L1I			0x00
#define CACHE_TYPE_L1D			0x01
#define CACHE_TYPE_L3			0x02
#define CACHE_TYPE_L2			0x03

#define INDEX_WRITEBACK_INV		0x00
#define INDEX_LOAD_TAG			0x04
#define INDEX_STORE_TAG			0x08
#define CREATE_DIRTY_EXCLUSIVE		0x0c
#define HIT_INVALIDATE			0x10
#define HIT_WRITEBACK_INV		0x14
#define HIT_WRITEBACK			0x18
#define HIT_SET_VIRTUAL			0x1c

#define CACHE_LINE_SIZE			0x20

/* IP32 Physical Addresses */
#define PHYS_BASE_CRIME			0x14000000
#define PHYS_BASE_RENDER		0x15000000
#define PHYS_BASE_MACE			0x1f000000
#define  MACE_PCI			  0x080000
#define  MACE_ETHERNET			  0x280000
#define  MACE_PERIPHERAL		  0x300000
#define   MACE_PERIPHERAL_AUDIO		   0x00000
#define   MACE_PERIPHERAL_ISA		   0x10000
#define   MACE_PERIPHERAL_KBD_MS	   0x20000
#define   MACE_PERIPHERAL_I2C		   0x30000
#define   MACE_PERIPHERAL_UST		   0x40000
#define  MACE_ISA_EXTERNAL		  0x380000
#define   MACE_ISA_UART_1		   0x10000
#define   MACE_ISA_UART_2		   0x18000
#define   MACE_ISA_RTC			   0x20000
#define PHYS_SYSTEM_ROM			0x1fc00000

/* ROM Addresses */
#define ROM_SIZE			0x80000
#define ROM_START			(KSEG1 | PHYS_SYSTEM_ROM)
#define ROM_END				(KSEG1 | (PHYS_SYSTEM_ROM + ROM_SIZE))
#define ROM_ALIGN			256

/* IP32 Device Base Addresses */
#define BASE_CRIME			(KSEG1 | PHYS_BASE_CRIME)
#define BASE_RENDER			(KSEG1 | PHYS_BASE_RENDER)
#define BASE_MACE_PCI			(KSEG1 | PHYS_BASE_MACE | MACE_PCI)
#define BASE_MEC			(KSEG1 | PHYS_BASE_MACE | MACE_ETHERNET)
#define BASE_AUDIO			(KSEG1 | PHYS_BASE_MACE | MACE_PERIPHERAL | MACE_PERIPHERAL_AUDIO)
#define BASE_ISA			(KSEG1 | PHYS_BASE_MACE | MACE_PERIPHERAL | MACE_PERIPHERAL_ISA)
#define BASE_KBD_MS			(KSEG1 | PHYS_BASE_MACE | MACE_PERIPHERAL | MACE_PERIPHERAL_KBD_MS)
#define BASE_I2C			(KSEG1 | PHYS_BASE_MACE | MACE_PERIPHERAL | MACE_PERIPHERAL_I2C)
#define BASE_UST			(KSEG1 | PHYS_BASE_MACE | MACE_PERIPHERAL | MACE_PERIPHERAL_UST)
#define BASE_UART_1			(KSEG1 | PHYS_BASE_MACE | MACE_ISA_EXTERNAL | MACE_ISA_UART_1)
#define BASE_UART_2			(KSEG1 | PHYS_BASE_MACE | MACE_ISA_EXTERNAL | MACE_ISA_UART_2)
#define BASE_RTC			(KSEG1 | PHYS_BASE_MACE | MACE_ISA_EXTERNAL | MACE_ISA_RTC)

/* ISA Interface Registers */
#define ISA_RING_BASE_AND_RESET		0x00
#define ISA_MISC_CONTROL		0x08
#define ISA_RESET			0x01
#define ISA_FLASH_ROM_WRITE_ENABLE	(1 << 0)
#define ISA_RED_LED			(1 << 4)
#define ISA_GREEN_LED			(1 << 5)

/* MACE PCI Host Bridge Registers */
#define MACE_PCI_ERROR_ADDR		0x00
#define MACE_PCI_ERROR_FLAGS		0x04
#define MACE_PCI_CONTROL		0x08
#define MACE_PCI_CONFIG_ADDR		0xcf8
#define MACE_PCI_CONFIG_DATA		0xcfc

/* MACE Audio Interface Registers */
#define MACE_AUDIO_STATUS		0x00
#define MACE_AUDIO_CODEC_STATUS		0x08
#define MACE_AUDIO_CODEC_INPUT_MASK	0x10
#define MACE_AUDIO_CODEC_INPUT		0x18
#define MACE_AUDIO_RING_CTRL_CHAN(x)	(0x20 * (x) + 0x8 * 0)
#define MACE_AUDIO_RD_PTR_CHAN(x)	(0x20 * (x) + 0x8 * 1)
#define MACE_AUDIO_WR_PTR_CHAN(x)	(0x20 * (x) + 0x8 * 2)
#define MACE_AUDIO_RING_DEPTH_CHAN(x)	(0x20 * (x) + 0x8 * 3)

/* MACE I2C Interface Registers */
#define MACE_I2C_CONFIG			0x00
#define MACE_I2C_STATUS			0x10
#define MACE_I2C_DATA			0x18

/* UART Registers */
#define BYTE_OFFSET			7
#define UART_REG(x)			(((x) << 8) + BYTE_OFFSET)
#define UART_DATA			UART_REG(0x00)
#define UART_IER			UART_REG(0x01)
#define UART_IIR			UART_REG(0x02)
#define UART_LCR			UART_REG(0x03)
#define UART_MCR			UART_REG(0x04)
#define UART_LSR			UART_REG(0x05)
#define UART_MSR			UART_REG(0x06)
#define UART_SCR			UART_REG(0x07)

/* MACE Ethernet Interface Registers */
#define MACE_ETH_MAC_CONTROL		0x00
#define MACE_ETH_INTR_STATUS		0x08
#define MACE_ETH_RX_MCL_WR_PTR		0x45
#define MACE_ETH_RX_MCL_RD_PTR		0x46
#define MACE_ETH_RX_MCL_DEPTH		0x47
#define MACE_ETH_MCL_RECEIVE_FIFO(x)	((x) + 0x100)

/* MACE PS/2 Interface Registers */
#define MACE_KEYBOARD_TX_BUF		0x00
#define MACE_KEYBOARD_RX_BUF		0x08
#define MACE_KEYBOARD_CONTROL		0x10
#define MACE_KEYBOARD_STATUS		0x18
#define MACE_MOUSE_TX_BUF		0x20
#define MACE_MOUSE_RX_BUF		0x28
#define MACE_MOUSE_CONTROL		0x30
#define MACE_MOUSE_STATUS		0x38

/* CRIME Registers */
#define CRIME_ID_OFFSET			0x0000
#define CRIME_CONTROL_OFFSET		0x0008
#define CRIME_INTSTAT_OFFSET		0x0010
#define CRIME_INTMASK_OFFSET		0x0018
#define CRIME_SOFT_INT_OFFSET		0x0020
#define CRIME_HARD_INT_OFFSET		0x0028
#define CRIME_WATCHDOG_OFFSET		0x0030
#define CRIME_TIMER_OFFSET		0x0038
#define CRIME_CPU_ERROR_ADDR		0x0040
#define CRIME_CPU_ERROR_STAT		0x0048
#define CRIME_CPU_ERROR_ENA		0x0050
#define CRIME_MC_STATUS_CTRL		0x0200
#define CRIME_BANK_0_CTRL		0x0208
#define CRIME_BANK_1_CTRL		0x0210
#define CRIME_BANK_2_CTRL		0x0218
#define CRIME_BANK_3_CTRL		0x0220
#define CRIME_BANK_4_CTRL		0x0228
#define CRIME_BANK_5_CTRL		0x0230
#define CRIME_BANK_6_CTRL		0x0238
#define CRIME_BANK_7_CTRL		0x0240
#define CRIME_REFRESH_COUNTER		0x0248
#define CRIME_ERROR_STATUS		0x0250
#define CRIME_ERROR_ADDR		0x0258
#define CRIME_SYNDROME_BITS		0x0260
#define CRIME_GENERATED_CHECK_BITS	0x0268
#define CRIME_REPLACEMENT_CHECK_BITS	0x0270

/* CRIME ID Register Bits */
#define CRIME_ID_REV			0x0f

/* CRIME Control Register Bits */
#define CRIME_CONTROL_TRITON_SYSADC	0x2000
#define CRIME_CONTROL_CRIME_SYSADC	0x1000
#define CRIME_CONTROL_HARD_RESET	0x0800
#define CRIME_CONTROL_SOFT_RESET	0x0400

/* CRIME Rendering Engine Registers */
#define RENDER_INTERFACE_CTRL		0x0400
#define CRIME_RE_TLB_A			0x1000
#define CRIME_RE_TLB_B			0x1200
#define CRIME_RE_TLB_C			0x1400
#define CRIME_DE_MODE_SRC		0x2000
#define CRIME_DE_MODE_DST		0x2008
#define CRIME_DE_DRAWMODE		0x2018
#define CRIME_DE_SCRMASK0		0x2020
#define CRIME_DE_SCRMASK1		0x2028
#define CRIME_DE_SCRMASK2		0x2030
#define CRIME_DE_SCRMASK3		0x2038
#define CRIME_DE_SCRMASK4		0x2040
#define CRIME_DE_SCISSOR		0x2048
#define CRIME_DE_WINOFFSET_SRC		0x2050
#define CRIME_DE_WINOFFSET_DST		0x2058
#define CRIME_DE_PRIMITIVE		0x2060
#define CRIME_DE_X_VERTEX_0		0x2070
#define CRIME_DE_X_VERTEX_1		0x2074
#define CRIME_DE_XFER_STEP_X		0x20a8
#define CRIME_DE_XFER_STEP_Y		0x20ac
#define CRIME_DE_STIPPLE_MODE		0x20c0
#define CRIME_DE_STIPPLE_PAT		0x20c4
#define CRIME_DE_FG			0x20d0
#define CRIME_DE_ROP			0x21b0
#define CRIME_DE_PLANEMASK		0x21b8
#define CRIME_DE_NULL			0x21f0
#define CRIME_DE_FLUSH			0x21f8
#define MTE_MODE			0x3000
#define MTE_BYTE_MASK			0x3008
#define MTE_STIPPLE_MASK		0x3010
#define MTE_FG_VALUE			0x3018
#define MTE_SRC0			0x3020
#define MTE_SRC1			0x3028
#define MTE_DST0			0x3030
#define MTE_DST1			0x3038
#define MTE_SRC_Y_STEP			0x3040
#define MTE_DST_Y_STEP			0x3048
#define MTE_NULL			0x3070
#define MTE_FLUSH			0x3078
#define CRIME_DE_STATUS			0x4000
#define CRIME_DE_START			0x0800

/* RTC Register Definitions */
#define RTC_REG(x)			((x) << 8)

#define RTC_SECONDS			RTC_REG(0x00)
#define RTC_SECONDS_ALARM		RTC_REG(0x01)
#define RTC_MINUTES			RTC_REG(0x02)
#define RTC_MINUTES_ALARM		RTC_REG(0x03)
#define RTC_HOURS			RTC_REG(0x04)
#define RTC_HOURS_ALARM			RTC_REG(0x05)
#define RTC_DAY_OF_WEEK			RTC_REG(0x06)
#define RTC_DAY_OF_MONTH		RTC_REG(0x07)
#define RTC_MONTH			RTC_REG(0x08)
#define RTC_YEAR			RTC_REG(0x09)
#define RTC_CTRL_A			RTC_REG(0x0a)
#define RTC_CTRL_B			RTC_REG(0x0b)
#define RTC_CTRL_C			RTC_REG(0x0c)
#define RTC_CTRL_D			RTC_REG(0x0d)
#define RTC_CRC				RTC_REG(0x47)
#define RTC_CENTURY			RTC_REG(0x48)
#define RTC_DATE_ALARM			RTC_REG(0x49)
#define RTC_EXT_CTRL_4A			RTC_REG(0x4a)
#define RTC_EXT_CTRL_4B			RTC_REG(0x4b)

#define RTC_NVRAM_BASE			0x0E
#define RTC_NVRAM(x)			RTC_REG(RTC_NVRAM_BASE + (x))

/* BSS Variable Names */
#define bss_start			0x0
#define render_base			0xaa0
#define crime_base			0xaa4
#define gbe_base			0xaa8

#endif /* _DEFINITIONS_H_ */
