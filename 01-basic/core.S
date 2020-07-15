// Tells compiler what assembly syntax we're using
.syntax unified
.cpu cortex-m0
.fpu softvfp
.thumb

// Global memory locations, exports following symbol
.global vtable
.global reset_handler

// Vector table
// .type tells ELF's symbol table whether the symbol
//   is an object or function
// .size tells ELF's symbol table how large the symbol
//   is; can be constant or expression
.type vtable, %object
vtable:
  .word _estack
  .word reset_handler
.size vtable, .-vtable

// Define the reset handler
.type reset_handler, %function
reset_handler:
  // Put stack pointer to end of stack, since stm32
  // uses a pre-decrementing stack
  LDR r0, = _estack
  MOV sp, r0

  // Put some immediates into registers
  MOVS r4, #6 // Limitation: immediate only up to 8 bits
  LDR r6, = 0xDEADBEEF
  LDR r7, = 0x00ECEBAD // pseudo-instruction, up to 32-bits
 
  main_loop:
    ADDS r4, r4, #1 // Add immediate
    ADDS r6, r6, #1
    B main_loop // Unconditional jump
.size reset_handler, .-reset_handler

