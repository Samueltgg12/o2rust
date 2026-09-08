PHDRS { post1 PT_LOAD; }
SECTIONS
{
  . = 0xbfc04400;
  .text : { *(.text) } :post1
}
