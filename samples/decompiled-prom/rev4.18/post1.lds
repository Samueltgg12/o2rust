PHDRS { post1 PT_LOAD; text_ram PT_LOAD AT(0xbfc06fd0); }
SECTIONS
{
  . = 0xbfc04400;
  .text : { *(.text) } :post1

  /* Relocated code: VMA=0xa0004000, LMA=0xbfc06fd0, len=0x2150 */
  . = 0xa0004000;
  __text_ram_lma = 0xbfc06fd0;
  data_0xbfc06fd0 = 0xbfc06fd0;
  .text_ram 0xa0004000 : AT(0xbfc06fd0) { *(.text_ram) } :text_ram

  /* Code after relocated section 1 */
  . = 0xbfc09120;
  .text_after_1 : AT(0xbfc09120) { *(.text_after_1) } :post1

  /DISCARD/ : { *(.MIPS.options) *(.MIPS.abiflags) *(.gnu.attributes) *(.reginfo) }
}
