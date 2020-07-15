/* SRAM is mmap'ed at 0x2000 0000 (See chapter 5, fig. 9) */
/* F031K6 has 4KB of SRAM; 4KB = 0x1000 */
/* so end of stack is 0x2000 0000 + 0x0000 1000*/
_estack = 0x20001000;

/* Flash is mmap'ed at 0x0800 0000 */
MEMORY
{
  FLASH ( rx )     : ORIGIN = 0x08000000, LENGTH = 32K
  RAM ( rxw )     : ORIGIN = 0x20000000, LENGTH = 4K
}

