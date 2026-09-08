/* Section: env */
/* Version: 1.0 */
/* Type: SECTION_TYPE_DATA */
/* Length: 0x400 (1024 bytes) */

#include "definitions.h"
#include "macros.inc"

.set noreorder
.set noat

	.int	0x00000000
	.int	0x00000000

	section_start env
	shdr env, 1.0, SECTION_TYPE_DATA

data_0xbfc04040:
	string 1, "AutoLoad=Yes"
data_0xbfc0404d:
	string 1, "console=g"
data_0xbfc04057:
	string 1, "diskless=0"
data_0xbfc04062:
	string 1, "dbaud=9600"
data_0xbfc0406d:
	string 1, "volume=80"
data_0xbfc04077:
	string 1, "sgilogo=y"
data_0xbfc04081:
	string 1, "TimeZone=PST8PDT"
data_0xbfc04092:
	string 1, "OSLoadPartition=pci(0)scsi(0)disk(1)rdisk(0)partition(0)"
data_0xbfc040cb:
	string 1, "OSLoadFilename=/unix"
data_0xbfc040e0:
	string 1, "netaddr=192.168.2.1"
data_0xbfc040f4:
	string 1, "monitor=h"
	.byte	0x00
	.byte	0x00
	.fill	191, 4, 0x00000000

	section_checksum
	section_end env
