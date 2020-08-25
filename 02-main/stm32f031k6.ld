/* ENTRY command tells where program is expected to start */
/* Could go into SECTIONS section */
ENTRY(reset_handler)

/* SRAM is mmap'ed at 0x2000 0000 (See chapter 5, fig. 9) */
/* F031K6 has 4KB of SRAM; 4KB = 0x1000 */
/* so end of stack is 0x2000 0000 + 0x0000 1000*/
_estack = 0x20001000;

/* Minimum size of stack, 1KB = 1024 = 0x400 */
_Min_Leftover_RAM = 0x400;

/* Flash is mmap'ed at 0x0800 0000 */
MEMORY
{
  /* name (attr) : ORIGIN = origin, LENGTH = len */
  FLASH ( rx )     : ORIGIN = 0x08000000, LENGTH = 32K
  RAM ( rxw )     : ORIGIN = 0x20000000, LENGTH = 4K
}

SECTIONS
{
  /* secname start BLOCK(align) (NOLOAD) : AT (ldaddr) */
  /* Vector table first */
  /* .vector_table ALIGN(4): */
  .vector_table :
  {
    /* Advance location counter "." to next boundary at 4 */
    . = ALIGN(4);
    /* Keep symbol even if not referenced */
    KEEP (*(.vector_table))
    . = ALIGN(4);
  } >FLASH

  /* Text section contains program code */
  .text :
  {
    . = ALIGN(4);
    *(.text) /* All .text input sections in all input files */
    *(.text*)
    . = ALIGN(4);
  } >FLASH

  /* rodata contains read-only data */
  /* data is copied from text section to here when program runs */
  .rodata :
  {
    . = ALIGN(4);
    *(.rodata)
    *(.rodata*)
    . = ALIGN(4);
  } >FLASH

  /* sidata for variables, which can change */
  _sidata = .;
  /* specifies the "load address" of the .data section */
  /* _sidata is the load memory address, ALIGN(4) is the */
  /* virtual memory address, or address when file is run. */
  .data : AT(_sidata)
  {
    . = ALIGN(4);
    /* Mark start/end locations for data section */
    _sdata = .;
    *(.data)
    *(.data*)
    _edata = .;
    . = ALIGN(4);
  } >RAM

  /* bss for variables initialized to 0 */
  .bss :
  {
    . = ALIGN(4);
    _sbss = .;
    *(.bss)
    *(.bss*)
    *(COMMON) /* Place common symbols in this location, standard practice */
    /* Also called "Input Section Common" */
    . = ALIGN(4);
    _ebss = .;
  } >RAM 

  /* heap */
  .dynamic_allocations :
  {
    . = ALIGN(4);
    _ssystem_ram = .;
    . = . + _Min_Leftover_RAM;
    . = ALIGN(4);
    _esystem_ram = .;
  } >RAM 
}
