/* Section: version */
/* Version: 4.18 */
/* Type: SECTION_TYPE_DATA */
/* Length: 0x388 (904 bytes) */

#include "definitions.h"
#include "macros.inc"

.set noreorder
.set noat

version_start:
/* ELF/SHDR Header (64 bytes) - ELF e_ident overlaid on SHDR */
	.int	ELF_MAGIC	/* e_ident[EI_MAG0-3] */
	.byte	ELFCLASS32	/* e_ident[EI_CLASS] */
	.byte	ELFDATA2MSB	/* e_ident[EI_DATA] */
	.byte	EV_CURRENT	/* e_ident[EI_VERSION] */
	.byte	ELFOSABI_NONE	/* e_ident[EI_OSABI] */
	.int	SHDR_MAGIC	/* SHDR magic / e_ident[EI_PAD] */
	.int	version_end - version_start	/* SHDR section_len */
	.byte	version_name_end - version_name_start	/* name_len */
	.byte	version_version_end - version_version_start	/* version_len */
	.byte	SECTION_TYPE_DATA	/* type / e_machine high byte */
	.byte	EM_MIPS	/* pad / e_machine low byte */
version_name_start:
	.ascii	"version"
version_name_end:
	.fill	4 - ((version_name_end - version_name_start) & 3), 1, 0
	.int	0x00000000	/* e_phoff */
	.int	0x00000244	/* e_shoff */
	.int	EF_MIPS_ARCH_2 | EF_MIPS_NOREORDER | EF_MIPS_PIC	/* e_flags */
	.short	52	/* e_ehsize */
	.short	0	/* e_phentsize */
	.short	0	/* e_phnum */
	.short	40	/* e_shentsize */
	.short	8	/* e_shnum */
	.short	7	/* e_shstrndx */
version_version_start:
	.ascii	"4.18"
version_version_end:
	.fill	4 - ((version_version_end - version_version_start) & 3), 1, 0
	shdr_checksum

	.fill	5, 4, 0x00000000
	.int	0x02080000
	.int	0x00001f00
	.fill	8, 4, 0x00000000
data_0xbfc6927c:
	string 4, "VSEGOK"
data_0xbfc69284:
	string 4, "Tue Oct 22 10:58:00 PDT 2002 \nVERSION 4.18\nO2 R5K/R7K/R10K/R12K\nIRIX 6.5.x IP32prom IP32PROM-v4\n"
	.int	0x00000000
	.int	0x002e726f
data_0xbfc692f0:
	string 4, "data"
	.fill	4, 4, 0x00000000
	.int	0x00000001
	.int	0x00000000
	.int	0x00000070
	.int	0x03000003
	.int	0x70090615
	.fill	2, 4, 0x00000000
	.int	0x00000060
	.fill	3, 4, 0x00000000
	.int	0x00000060
	.int	0x00000002
	.int	0x00000178
	.fill	3, 4, 0x00000000
	.int	0x00000078
	.int	0x0000000b
	.int	0x00000190
	.int	0x00000009
	.int	0x0000019b
	.int	0x00000001
	.int	0x000001a4
	.int	0x00000001
	.int	0x000001ec
	.int	0x00000001
	.int	0x000001f0
	.int	0x00000001
	.int	0x00000000
	.int	0x2c200002
	.int	0x00000001
	.int	0x00000000
	.int	0x20200000
	.int	0x00766572
data_0xbfc69394:
	string 4, "sion.s"
data_0xbfc6939c:
	string 4, ".rodata"
	.int	0x00000003
	.int	0x00000001
	.int	0x00000000
	.int	0x0000000b
	.int	0x00000000
	.int	0x00000002
	.fill	8, 4, 0x00000000
	.int	0x00000001
	.int	0x1d800000
	.fill	3, 4, 0x00000000
	.int	0x0000ffff
	.int	0x00000001
	.int	0x00000000
	.int	0x004fffff
	.int	0x002e4d49
	.int	0x50532e6f
	.int	0x7074696f
	.int	0x6e73002e
data_0xbfc69410:
	string 4, "reginfo"
data_0xbfc69418:
	string 4, ".rodata"
data_0xbfc69420:
	string 4, ".strtab"
data_0xbfc69428:
	string 4, ".symtab"
data_0xbfc69430:
	string 4, ".mdebug"
data_0xbfc69438:
	string 4, ".shstrtab"
	.fill	10, 4, 0x00000000
	.int	0x00000001
	.int	0x7000000d
	.int	0x08000002
	.int	0x00000000
	.int	0x00000034
	.int	0x00000028
	.fill	2, 4, 0x00000000
	.int	0x00000004
	.int	0x00000000
	.int	0x0000000f
	.int	0x70000006
	.int	0x00000002
	.int	0x00000000
	.int	0x0000005c
	.int	0x00000018
	.fill	2, 4, 0x00000000
	.int	0x00000004
	.int	0x00000018
	.int	0x00000018
	.int	0x00000001
	.int	0x00000003
	.int	0x00000000
	.int	0x0000007c
	.int	0x00000070
	.fill	2, 4, 0x00000000
	.int	0x00000010
	.int	0x00000001
	.int	0x00000020
	.int	0x00000003
	.fill	2, 4, 0x00000000
	.int	0x000000ec
	.int	0x00000009
	.fill	4, 4, 0x00000000
	.int	0x00000028
	.int	0x00000002
	.fill	2, 4, 0x00000000
	.int	0x000000f8
	.int	0x00000020
	.int	0x00000004
	.int	0x00000002
	.int	0x00000004
	.int	0x00000010
	.int	0x00000030
	.int	0x70000005
	.fill	2, 4, 0x00000000
	.int	0x00000118
	.int	0x000000e8
	.fill	2, 4, 0x00000000
	.int	0x00000004
	.int	0x00000001
	.int	0x00000038
	.int	0x00000003
	.fill	2, 4, 0x00000000
	.int	0x00000200
	.int	0x00000042
	.fill	4, 4, 0x00000000

	section_checksum
version_end:
