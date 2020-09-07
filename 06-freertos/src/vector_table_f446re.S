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
// Can find vector table in 2.3.4 of stm32f4 manual
vtable:
  // Exception number 0-15
  .word _estack
  .word reset_handler
  .word NMI_handler
  .word HardFault_handler
  .word MemManage_handler
  .word BusFault_handler
  .word UsageFault_handler
  .word 0
  .word 0
  .word 0
  .word 0
  .word SVCall_handler
  .word DebugMon_handler
  .word 0
  .word PendSV_handler
  .word SysTick_handler
  // According to the manual, 16 and above are configurable
  .word WWDG_handler /* Window WatchDog */
  .word PVD_handler /* PVD through EXTI Line detect */
  .word TAMP_STAMP_handler /* Tamper and Timestamp through EXTI line */
  .word RTC_handler /* RTC through EXTI line */
  .word FLASH_handler /* FLASH */
  .word RCC_handler /* RCC */
  .word EXTI0_handler /* EXTI Line 0 */
  .word EXTI1_handler /* EXTI Line 1 */
  .word EXTI2_handler /* EXTI Line 2 */
  .word EXTI3_handler /* EXTI Line 3 */
  .word EXTI4_handler /* EXTI Line 4 */
  .word DMA1_Stream0_handler /* DMA1 Stream 0 */
  .word DMA1_Stream1_handler /* DMA1 Stream 1 */
  .word DMA1_Stream2_handler /* DMA1 Stream 2 */
  .word DMA1_Stream3_handler /* DMA1 Stream 3 */
  .word DMA1_Stream4_handler /* DMA1 Stream 4 */
  .word DMA1_Stream5_handler /* DMA1 Stream 5 */
  .word DMA1_Stream6_handler /* DMA1 Stream 6 */
  .word ADC1_handler /* ADC1, 2, 3 */
  .word CAN1_TX_handler /* CAN Transmit */
  .word CAN1_RX0_handler /* CAN Receive 0 */
  .word CAN1_RX1_handler /* CAN Receive 1 */
  .word CAN1_SCE_handler /* CAN SCE, not sure exactly what this is */
  .word EXTI9_5_handler /* EXTI Line 5-9 */
  .word TIM1_BRK_TIM9_handler /* TIM1 Break, and TIM9 */
  .word TIM1_UP_TIM10_handler /* TIM1 Update, and TIM10 */
  .word TIM1_TRG_COM_TIM11_handler /* TIM1 Trigger, Commutation, and TIM11 */
  .word TIM1_CC_handler /* TIM1 Capture Compare */
  .word TIM2_handler /* TIM2 */
  .word TIM3_handler /* TIM3 */
  .word TIM4_handler /* TIM4 */
  .word I2C1_EV_handler /* I2C1 Event */
  .word I2C1_ER_handler /* I2C1 Error */
  .word I2C2_EV_handler /* I2C2 Event */
  .word I2C2_ER_handler /* I2C2 Error */
  .word SPI1_handler /* SPI1 */
  .word SPI2_handler /* SPI2 */
  .word USART1_handler /* USART1 */
  .word USART2_handler /* USART2 */
  .word USART3_handler /* USART3 */
  .word EXTI15_10_handler /* EXTI 10-15 */
  .word RTC_Alarm_handler /* RTC Alarm A and B through EXTI Line */
  .word OTG_FS_WKUP_handler /* USB OTG Full-Speed wakeup through EXTI */
  .word TIM8_BRK_TIM12_handler /* TIM8 Break, and TIM12 */
  .word TIM8_UP_TIM13_handler /* TIM8 Update, and TIM13 */
  .word TIM8_TRG_COM_TIM14_handler /* TIM8 Trigger, Commutation, and TIM14 */
  .word TIM8_CC_handler /* TIM8 Capture Compare */
  .word DMA1_Stream7_handler /* DMA1 Stream7 */
  .word FMC_handler /* FMC (for LCD?) */
  .word SDIO_handler /* SDIO (SD card interface) */
  .word TIM5_handler /* TIM5 */
  .word SPI3_handler /* SPI3 */
  .word UART4_handler /* UART4 */
  .word UART5_handler /* UART5 */
  .word TIM6_DAC_handler /* TIM6 and DAC1/DAC2 underrun errors */
  .word TIM7_handler /* TIM7 */
  .word DMA2_Stream0_handler /* DMA2 Stream 0 */
  .word DMA2_Stream1_handler /* DMA2 Stream 1 */
  .word DMA2_Stream2_handler /* DMA2 Stream 2 */
  .word DMA2_Stream3_handler /* DMA2 Stream 3 */
  .word DMA2_Stream4_handler /* DMA2 Stream 4 */
  .word 0
  .word 0
  .word CAN2_TX_handler /* CAN Transmit */
  .word CAN2_RX0_handler /* CAN Receive 0 */
  .word CAN2_RX1_handler /* CAN Receive 1 */
  .word CAN2_SCE_handler /* CAN SCE, not sure exactly what this is */
  .word OTG_FS_handler /* USB OTG Full-Speed interrupt */
  .word DMA2_Stream5_handler /* DMA2 Stream 5 */
  .word DMA2_Stream6_handler /* DMA2 Stream 6 */
  .word DMA2_Stream7_handler /* DMA2 Stream 7 */
  .word USART6_handler /* USART6 */
  .word I2C3_EV_handler /* I2C3 Event */
  .word I2C3_ER_handler /* I2C3 Error */
  .word OTG_HS_EP1_OUT_handler /* USB OTG High-Speed End Point 1 Out */
  .word OTG_HS_EP1_IN_handler /* USB OTG High-Speed End Point 1 In */
  .word OTG_HS_WKUP_handler /* USB OTG High-Speed wakeup through EXTI */
  .word OTG_HS_handler /* USB OTG High-Speed */
  .word DCMI_handler /* DCMI */
  .word 0
  .word 0
  .word FPU_handler /* FPU */
  .word 0
  .word 0
  .word SPI4_handler /* SPI4 */
  .word 0
  .word 0
  .word SAI1_handler /* SAI1 (serial audio interface) */
  .word 0
  .word 0
  .word 0
  .word SAI2_handler /* SAI2 */
  .word QuadSPI_handler /* QuadSPI; faster SPI with four data lines instead of two */
  .word CEC_handler /* CEC (HDMI controller interface) */
  .word SPDIF_RX_handler /* SPIDIF (optical audio) */
  .word FMPI2C_EV_handler /* FMPI2C1 (Fast-Mode-Plus) event */
  .word FMPI2C_ER_handler /* FMPI2C1 error */

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
  .weak MemManage_handler
  .thumb_set MemManage_handler,default_irq_handler
  .weak BusFault_handler
  .thumb_set BusFault_handler,default_irq_handler
  .weak UsageFault_handler
  .thumb_set UsageFault_handler,default_irq_handler
  .weak SVCall_handler
  .thumb_set SVCall_handler,default_irq_handler
  .weak DebugMon_handler
  .thumb_set DebugMon_handler,default_irq_handler
  .weak PendSV_handler
  .thumb_set PendSV_handler,default_irq_handler
  .weak SysTick_handler
  .thumb_set SysTick_handler,default_irq_handler
  .weak WWDG_handler
  .thumb_set WWDG_handler,default_irq_handler
  .weak PVD_handler
  .thumb_set PVD_handler,default_irq_handler
  .weak TAMP_STAMP_handler
  .thumb_set TAMP_STAMP_handler,default_irq_handler
  .weak RTC_handler
  .thumb_set RTC_handler,default_irq_handler
  .weak FLASH_handler
  .thumb_set FLASH_handler,default_irq_handler
  .weak RCC_handler
  .thumb_set RCC_handler,default_irq_handler
  .weak EXTI0_handler
  .thumb_set EXTI0_handler,default_irq_handler
  .weak EXTI1_handler
  .thumb_set EXTI1_handler,default_irq_handler
  .weak EXTI2_handler
  .thumb_set EXTI2_handler,default_irq_handler
  .weak EXTI3_handler
  .thumb_set EXTI3_handler,default_irq_handler
  .weak EXTI4_handler
  .thumb_set EXTI4_handler,default_irq_handler
  .weak DMA1_Stream0_handler
  .thumb_set DMA1_Stream0_handler,default_irq_handler
  .weak DMA1_Stream1_handler
  .thumb_set DMA1_Stream1_handler,default_irq_handler
  .weak DMA1_Stream2_handler
  .thumb_set DMA1_Stream2_handler,default_irq_handler
  .weak DMA1_Stream3_handler
  .thumb_set DMA1_Stream3_handler,default_irq_handler
  .weak DMA1_Stream4_handler
  .thumb_set DMA1_Stream4_handler,default_irq_handler
  .weak DMA1_Stream5_handler
  .thumb_set DMA1_Stream5_handler,default_irq_handler
  .weak DMA1_Stream6_handler
  .thumb_set DMA1_Stream6_handler,default_irq_handler
  .weak ADC1_handler
  .thumb_set ADC1_handler,default_irq_handler
  .weak CAN1_TX_handler
  .thumb_set CAN1_TXhandler,default_irq_handler
  .weak CAN1_RX0_handler
  .thumb_set CAN1_RX0_handler,default_irq_handler
  .weak CAN1_RX1_handler
  .thumb_set CAN1_RX1_handler,default_irq_handler
  .weak CAN1_SCE_handler
  .thumb_set CAN1_SCE_handler,default_irq_handler
  .weak EXTI9_5_handler
  .thumb_set EXTI9_5_handler,default_irq_handler
  .weak TIM1_BRK_TIM9_handler
  .thumb_set TIM1_BRK_TIM9_handler,default_irq_handler
  .weak TIM1_UP_TIM10_handler
  .thumb_set TIM1_UP_TIM10_handler,default_irq_handler
  .weak TIM1_TRG_COM_TIM11_handler
  .thumb_set TIM1_TRG_COM_TIM11_handler,default_irq_handler
  .weak TIM1_CC_handler
  .thumb_set TIM1_CC_handler,default_irq_handler
  .weak TIM2_handler
  .thumb_set TIM2_handler,default_irq_handler
  .weak TIM3_handler
  .thumb_set TIM3_handler,default_irq_handler
  .weak TIM4_handler
  .thumb_set TIM4_handler,default_irq_handler
  .weak I2C1_EV_handler
  .thumb_set I2C1_EV_handler,default_irq_handler
  .weak I2C1_ER_handler
  .thumb_set I2C1_ER_handler,default_irq_handler
  .weak I2C2_EV_handler
  .thumb_set I2C2_EV_handler,default_irq_handler
  .weak I2C2_ER_handler
  .thumb_set I2C2_ER_handler,default_irq_handler
  .weak SPI1_handler
  .thumb_set SPI1_handler,default_irq_handler
  .weak SPI2_handler
  .thumb_set SPI2_handler,default_irq_handler
  .weak USART1_handler
  .thumb_set USART1_handler,default_irq_handler
  .weak USART2_handler
  .thumb_set USART2_handler,default_irq_handler
  .weak USART3_handler
  .thumb_set USART3_handler,default_irq_handler
  .weak EXTI15_10_handler
  .thumb_set EXTI15_10_handler,default_irq_handler
  .weak RTC_Alarm_handler
  .thumb_set RTC_Alarm_handler,default_irq_handler
  .weak OTG_FS_WKUP_handler
  .thumb_set OTG_FS_WKUP_handler,default_irq_handler
  .weak TIM8_BRK_TIM12_handler
  .thumb_set TIM8_BRK_TIM12_handler,default_irq_handler
  .weak TIM8_UP_TIM13_handler
  .thumb_set TIM8_UP_TIM13_handler,default_irq_handler
  .weak TIM8_TRG_COM_TIM14_handler
  .thumb_set TIM8_TRG_COM_TIM14_handler,default_irq_handler
  .weak TIM8_CC_handler
  .thumb_set TIM8_CC_handler,default_irq_handler
  .weak DMA1_Stream7_handler
  .thumb_set DMA1_Stream7_handler,default_irq_handler
  .weak FMC_handler
  .thumb_set FMC_handler,default_irq_handler
  .weak SDIO_handler
  .thumb_set SDIO_handler,default_irq_handler
  .weak TIM5_handler
  .thumb_set TIM5_handler,default_irq_handler
  .weak SPI3_handler
  .thumb_set SPI3_handler,default_irq_handler
  .weak UART4_handler
  .thumb_set UART4_handler,default_irq_handler
  .weak UART5_handler
  .thumb_set UART5_handler,default_irq_handler
  .weak TIM6_DAC_handler
  .thumb_set TIM6_DAC_handler,default_irq_handler
  .weak TIM7_handler
  .thumb_set TIM7_handler,default_irq_handler
  .weak DMA2_Stream0_handler
  .thumb_set DMA2_Stream0_handler,default_irq_handler
  .weak DMA2_Stream1_handler
  .thumb_set DMA2_Stream1_handler,default_irq_handler
  .weak DMA2_Stream2_handler
  .thumb_set DMA2_Stream2_handler,default_irq_handler
  .weak DMA2_Stream3_handler
  .thumb_set DMA2_Stream3_handler,default_irq_handler
  .weak DMA2_Stream4_handler
  .thumb_set DMA2_Stream4_handler,default_irq_handler
  .weak CAN2_TX_handler
  .thumb_set CAN2_TX_handler,default_irq_handler
  .weak CAN2_RX0_handler
  .thumb_set CAN2_RX0_handler,default_irq_handler
  .weak CAN2_RX1_handler
  .thumb_set CAN2_RX1_handler,default_irq_handler
  .weak CAN2_SCE_handler
  .thumb_set CAN2_SCE_handler,default_irq_handler
  .weak OTG_FS_handler
  .thumb_set OTG_FS_handler,default_irq_handler
  .weak DMA2_Stream5_handler
  .thumb_set DMA2_Stream5_handler,default_irq_handler
  .weak DMA2_Stream6_handler
  .thumb_set DMA2_Stream6_handler,default_irq_handler
  .weak DMA2_Stream7_handler
  .thumb_set DMA2_Stream7_handler,default_irq_handler
  .weak USART6_handler
  .thumb_set UART6_handler,default_irq_handler
  .weak I2C3_EV_handler
  .thumb_set I2C_EV_handler,default_irq_handler
  .weak I2C3_ER_handler
  .thumb_set I2C_ER_handler,default_irq_handler
  .weak OTG_HS_EP1_OUT_handler
  .thumb_set OTG_HS_EP1_OUT_handler,default_irq_handler
  .weak OTG_HS_EP1_IN_handler
  .thumb_set OTG_HS_EP1_IN_handler,default_irq_handler
  .weak OTG_HS_WKUP_handler
  .thumb_set OTG_HS_WKUP_handler,default_irq_handler
  .weak OTG_HS_handler
  .thumb_set OTG_HS_handler,default_irq_handler
  .weak DCMI_handler
  .thumb_set DCMI_handler,default_irq_handler
  .weak FPU_handler
  .thumb_set FPU_handler,default_irq_handler
  .weak SPI4_handler
  .thumb_set SPI4_handler,default_irq_handler
  .weak SAI1_handler
  .thumb_set SAI1_handler,default_irq_handler
  .weak SAI2_handler
  .thumb_set SAI2_handler,default_irq_handler
  .weak QuadSPI_handler
  .thumb_set QuadSPI_handler,default_irq_handler
  .weak CEC_handler
  .thumb_set CEC_handler,default_irq_handler
  .weak SPDIF_RX_handler
  .thumb_set SPDIF_RX_handler,default_irq_handler
  .weak FMPI2C_EV_handler
  .thumb_set FMPI2C_EV_handler,default_irq_handler
  .weak FMPI2C_ER_handler
  .thumb_set FMPI2C_ER_handler,default_irq_handler
.size vtable, .-vtable

// Section is allocatable and executable ("ax")
// and contains data (@progbits) as opposed to
// "no data" (@nobits), taking up no space
.section .text.default_irq_handler,"ax",%progbits
default_irq_handler:
  default_interrupt_loop:
    B default_interrupt_loop
.size default_irq_handler, .-default_irq_handler

