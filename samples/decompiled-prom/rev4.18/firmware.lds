PHDRS { firmware PT_LOAD; }
SECTIONS
{
  . = 0x81000000;
  .text : { *(.text) } :firmware

  __lma_pos = 0x81000000 + SIZEOF(.text);

  .rodata_header : AT(__lma_pos) { *(.rodata_header) } :firmware
  __lma_pos = __lma_pos + SIZEOF(.rodata_header);

  . = 0x81048e70;
  .rodata : AT(__lma_pos) { *(.rodata) } :firmware
  __lma_pos = __lma_pos + SIZEOF(.rodata);

  .rwdata_header : AT(__lma_pos) { *(.rwdata_header) } :firmware
  __lma_pos = __lma_pos + SIZEOF(.rwdata_header);

  . = 0x81054100;
  .rwdata : AT(__lma_pos) { *(.rwdata) } :firmware
  __lma_pos = __lma_pos + SIZEOF(.rwdata);

  .sentinel : AT(__lma_pos) { *(.sentinel) } :firmware

  /DISCARD/ : { *(.MIPS.options) *(.MIPS.abiflags) *(.gnu.attributes) *(.reginfo) }
}
