// Tells compiler what assembly syntax we're using
.syntax unified
.cpu cortex-m0
.fpu softvfp
.thumb

// Global memory locations, exports following symbol
.global vtable
.global default_irq_handler

// Vector table
// .type tells ELF's symbol table whether the symbol
//   is an object or function
// .size tells ELF's symbol table how large the symbol
//   is; can be constant or expression
.type vtable, %object
.section .vector_table,"a",%progbits
// Can find vector table in 2.3.4 of stm32f0 manual
vtable:
  // Exception number 0-15
  .word _estack
  .word reset_handler
  .word NMI_handler
  .word HardFault_handler
  .word 0
  .word 0
  .word 0
  .word 0
  .word 0
  .word 0
  .word 0
  .word SVCall_handler
  .word 0
  .word 0
  .word PendSV_handler
  .word SysTick_handler
  // Exception number 16-31
  // According to the manual, 16-47 are manually configurable
  .word WWDG_handler /* Window WatchDog */
  .word PVD_handler /* PVD through ETI Line detect */
  .word RTC_handler /* RTC through EXTI line */
  .word FLASH_handler /* FLASH */
  .word RCC_handler /* RCC */
  .word EXTI0_1_handler /* EXTI Line 0 and 1 */
  .word EXTI2_3_handler /* EXTI Line 2 and 3 */
  .word EXTI4_15_handler /* EXTI Line 4 to 15 */
  .word 0
  .word DMA1_Channel1_handler /* DMA1 Channel 1 */
  .word DMA1_Channel2_3_handler /* DMA1 Channel 2 and 3 */
  .word DMA1_Channel4_5_handler /* DMA1 Channel 4 and 5 */
  .word ADC1_handler /* ADC1 */
  .word TIM1_BRK_UP_TRG_COM_handler /* TIM1 Break, Update, Trigger, and Commutation */
  .word TIM1_CC_handler /* TIM1 Capture Compare */
  .word TIM2_handler /* TIM2 */
  .word TIM3_handler /* TIM3 */
  .word 0
  .word 0
  .word TIM14_handler /* TIM14 */
  .word 0
  .word TIM16_handler /* TIM16 */
  .word TIM17_handler /* TIM17 */
  .word I2C1_handler /* I2C1 */
  .word 0
  .word SPI1_handler /* SPI1 */
  .word 0
  .word USART1_handler /* USART1 */
  .word 0
  .word 0
  .word 0
  .word 0

  // (Location to boot from for RAM startup)
  #define boot_ram_base 0xF108F85F
  .word boot_ram_base

  // Provide "weak alias" for each exception handler to the default handler
  // Since they're weak aliases, any function with the same name will override
  // the weak alias.
  .weak NMI_handler
  .thumb_set NMI_handler,default_irq_handler
  .weak HardFault_handler
  .thumb_set HardFault_handler,default_irq_handler
  .weak SVCall_handler
  .thumb_set SVCall_handler,default_irq_handler
  .weak PendSV_handler
  .thumb_set PendSV_handler,default_irq_handler
  .weak SysTick_handler
  .thumb_set SysTick_handler,default_irq_handler
  .weak WWDG_handler
  .thumb_set WWDG_handler,default_irq_handler
  .weak PVD_handler
  .thumb_set PVD_handler,default_irq_handler
  .weak RTC_handler
  .thumb_set RTC_handler,default_irq_handler
  .weak FLASH_handler
  .thumb_set FLASH_handler,default_irq_handler
  .weak RCC_handler
  .thumb_set RCC_handler,default_irq_handler
  .weak EXTI0_1_handler
  .thumb_set EXTI0_1_handler,default_irq_handler
  .weak EXTI2_3_handler
  .thumb_set EXTI2_3_handler,default_irq_handler
  .weak EXTI4_15_handler
  .thumb_set EXTI4_15_handler,default_irq_handler
  .weak DMA1_Channel1_handler
  .thumb_set DMA1_Channel1_handler,default_irq_handler
  .weak DMA1_Channel2_3_handler
  .thumb_set DMA1_Channel2_3_handler,default_irq_handler
  .weak DMA1_Channel4_5_handler
  .thumb_set DMA1_Channel4_5_handler,default_irq_handler
  .weak ADC1_handler
  .thumb_set ADC1_handler,default_irq_handler
  .weak TIM1_BRK_UP_TRG_COM_handler
  .thumb_set TIM1_BRK_UP_TRG_COM_handler,default_irq_handler
  .weak TIM1_CC_handler
  .thumb_set TIM1_CC_handler,default_irq_handler
  .weak TIM2_handler
  .thumb_set TIM2_handler,default_irq_handler
  .weak TIM3_handler
  .thumb_set TIM3_handler,default_irq_handler
  .weak TIM14_handler
  .thumb_set TIM14_handler,default_irq_handler
  .weak TIM16_handler
  .thumb_set TIM16_handler,default_irq_handler
  .weak TIM17_handler
  .thumb_set TIM16_handler,default_irq_handler
  .weak I2C1_handler
  .thumb_set I2C1_handler,default_irq_handler
  .weak SPI1_handler
  .thumb_set SPI1_handler,default_irq_handler
  .weak USART1_handler
  .thumb_set USART1_handler,default_irq_handler
.size vtable, .-vtable

// Section is allocatable and executable ("ax")
// and contains data (@progbits) as opposed to
// "no data" (@nobits), taking up no space
.section .text.default_irq_handler,"ax",%progbits
default_irq_handler:
  default_interrupt_loop:
    B default_interrupt_loop
.size default_irq_handler, .-default_irq_handler

