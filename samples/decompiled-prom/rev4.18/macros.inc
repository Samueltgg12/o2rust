/* Auto-generated assembly macros */

.altmacro

/*
 * Macros for constructing 32-bit addresses from lui + addiu/load/store patterns.
 * Use HI and LO when the low bits come from ADDIU or load/store displacement
 * (sign-extended). These match the GNU assembler's %hi/%lo operators.
 *
 * When the low 16 bits have bit 15 set (>= 0x8000), they will be sign-extended
 * to a negative value. To compensate, HI adds 1 to the high bits.
 */
#define LO(x) (((x) & 0xffff) - (((x) & 0x8000) << 1))
#define HI(x) (((x) >> 16) + (((x) & 0x8000) >> 15))

/*
 * Macros for constructing 32-bit values from lui + ori patterns.
 * Use HI_UNSIGNED and LO_UNSIGNED when the low bits come from ORI (zero-extended).
 * WARNING: Do not use these with %hi/%lo assembler operators.
 */
#define LO_UNSIGNED(x) ((x) & 0xffff)
#define HI_UNSIGNED(x) ((x) >> 16)

/*
 * Macros for accessing BSS (uninitialized data).
 * BSS_BASE is automatically calculated as the end of rwdata.
 * Use BSS_HI and BSS_LO with offsets relative to BSS_BASE.
 * These use %%hi/%%lo relocations to allow forward reference to BSS_BASE.
 */
#define BSS_HI(offset) %hi(BSS_BASE + (offset))
#define BSS_LO(offset) %lo(BSS_BASE + (offset))

/*
 * Mark the start of a section for automatic length calculation.
 * Call this immediately after the first two instructions.
 * The section start is 8 bytes before the current position
 * (accounting for the two instructions already emitted).
 */
.macro section_start name
\name\()_start = . - 8
.endm

/*
 * Mark the end of a section for automatic length calculation.
 * Call this as the last directive in the section.
 */
.macro section_end name
\name\()_end = .
.endm

/*
 * Track the size of an ELF section.
 * Use elfsect_start at the beginning and elfsect_end at the end.
 * The size is: __elfsect_<name>_end - __elfsect_<name>_start
 */
.macro elfsect_start name
__elfsect_\name\()_start = .
.endm

.macro elfsect_end name
__elfsect_\name\()_end = .
.endm

/* Shorthand for section size */
#define ELFSECT_SIZE(name) (__elfsect_ ## name ## _end - __elfsect_ ## name ## _start)

/*
 * Emit an SHDR header (56 bytes).
 * This macro emits the header starting at the "SHDR" magic.
 * The first two instructions should be emitted as code before this macro.
 * String lengths and section length are calculated automatically
 * using label arithmetic (requires section_start before and section_end after).
 * The SHDR checksum is set to zero; use the checksum tool to set it.
 *
 * Usage: shdr <name>, <version>, <type>
 *
 * Parameters:
 *   name          - Section name string (max 31 chars)
 *   version       - Version string (max 7 chars)
 *   type          - Section type (SECTION_TYPE_*)
 */
.macro shdr name, version, type
	/* Magic "SHDR" */
	.ascii "SHDR"
	/* Section length (calculated from section_start/section_end labels) */
	.int \name\()_end - \name\()_start
	/* name_len, version_len, section_type, padding */
	.byte \name\()_name_end - \name\()_name_start
	.byte \name\()_version_end - \name\()_version_start
	.byte \type
	.byte 0
	/* Name field (32 bytes) */
\name\()_name_start:
	.ascii "\name"
\name\()_name_end:
	.fill 32 - (\name\()_name_end - \name\()_name_start), 1, 0
	/* Version field (8 bytes) */
\name\()_version_start:
	.ascii "\version"
\name\()_version_end:
	.fill 8 - (\name\()_version_end - \name\()_version_start), 1, 0
	/* SHDR Checksum (placeholder, set by checksum tool) */
	shdr_checksum
.endm

/*
 * Emit a placeholder SHDR checksum (4 bytes of zeros).
 * The checksum tool will calculate and set the correct value.
 */
.macro shdr_checksum
	.int 0x00000000
.endm

/*
 * Emit an SHDR header with explicit length (for multi-section loadable firmware).
 * Use this instead of 'shdr' when section content spans multiple ELF sections.
 * The length should be the sum of all ELF section sizes.
 */
.macro shdr_with_len name, version, type, length
	.ascii "SHDR"
	.int \length
	.byte \name\()_name_end - \name\()_name_start
	.byte \name\()_version_end - \name\()_version_start
	.byte \type
	.byte 0
\name\()_name_start:
	.ascii "\name"
\name\()_name_end:
	.fill 32 - (\name\()_name_end - \name\()_name_start), 1, 0
\name\()_version_start:
	.ascii "\version"
\name\()_version_end:
	.fill 8 - (\name\()_version_end - \name\()_version_start), 1, 0
	shdr_checksum
.endm

/*
 * Emit a placeholder section checksum (4 bytes of zeros).
 * This should be the last 4 bytes of the section.
 * The checksum tool will calculate and set the correct value.
 */
.macro section_checksum
	.int 0x00000000
.endm

/*
 * Mark the start of a subsection for automatic length calculation.
 * The subsection length is calculated from subsect_start to subsect_end.
 */
.macro subsect_start addr
subsection_\addr\()_start = .
.endm

/*
 * Emit a subsection header (8 bytes).
 * Contains the load address and subsection length.
 * The length is calculated automatically using label arithmetic.
 */
.macro subsect_header addr
	.int \addr
	.int subsection_\addr\()_end - subsection_\addr\()_start
.endm

/*
 * Mark the end of a subsection.
 */
.macro subsect_end addr
subsection_\addr\()_end = .
.endm

/*
 * Emit a sentinel marker (8 bytes) at the end of all subsections.
 * The sentinel has a load address and zero length.
 */
.macro sentinel addr
	.int \addr
	.int 0x00000000
.endm

/*
 * Emit a string with automatic zero-padding to pad_to boundary.
 * The padding fills from the end of the string to the next multiple of pad_to.
 * Use pad_to=4 for code sections (4-byte alignment).
 * Use pad_to=1 for data sections (no padding).
 */
.macro string pad_to, str
LOCAL start
LOCAL end
start = .
.ascii \str
end = .
/* Always emit at least 1 byte (NUL terminator), plus padding to pad_to */
.fill (\pad_to - ((end - start) & (\pad_to - 1))), 1, 0
.endm
