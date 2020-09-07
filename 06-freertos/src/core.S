.syntax unified
.cpu cortex-m0
.fpu softvfp
.thumb

// Global memory locations, exports reset_handler symbol
.global reset_handler

.type reset_handler, %function
reset_handler:
  // Set the stack pointer to the end of the stack
  LDR r0, =_estack
  MOV sp, r0

  // Copy data from flash to RAM
  // R0 stores progress along data section
  MOVS r0, #0
  // Load start/end addresses of the data section
  // and the start of the data init section
  LDR r1, =_sdata
  LDR r2, =_edata
  LDR r3, =_sidata
  B copy_sidata_loop

  copy_sidata:
    // r4 contains data at r3[r0]
    // r3 = begining of sidata, r0 contains offset into sidata 
    LDR r4, [r3, r0] // LDR Rd, [Rn, Rm]
    // Store contents of r4 into r1[r0]
    STR r4, [r1, r0]
    // Increase the progress offset
    ADDS r0, r0, #4

  copy_sidata_loop:
    // Copy the next word from sidata->data
    ADDS r4, r0, r1
    CMP r4, r2
    // Jump to copy_sidata if r4 < r2, aka offset into
    // data has not yet reached the end
    BCC copy_sidata

  // Fill bss section with 0s
  MOVS r0, #0
  LDR r1, =_sbss
  LDR r2, =_ebss

  reset_bss:
    STR r0, [r1]
    ADDS r1, r1, #4
  
  reset_bss_loop:
    CMP r1, r2
    BCC reset_bss
   
  B main
.size reset_handler, .-reset_handler
