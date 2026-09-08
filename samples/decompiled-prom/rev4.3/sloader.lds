PHDRS { sloader PT_LOAD; }
SECTIONS
{
  . = 0xbfc00000;
  .text : { *(.text) } :sloader
}
