/* SRAM is mmap'ed at 0x2000 0000 (See chapter 5, fig. 15) */
/* F446RET6 has 128KB of SRAM; 128KB = 0x2 0000 */
/* so end of stack is 0x2000 0000 + 0x0002 0000*/
_estack = 0x20020000;

/* Flash is mmap'ed at 0x0800 0000 */
MEMORY
{
  FLASH ( rx )     : ORIGIN = 0x08000000, LENGTH = 512K
  RAM ( rxw )     : ORIGIN = 0x20000000, LENGTH = 128K
}

