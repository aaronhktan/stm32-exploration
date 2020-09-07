#include "main.h"

#include "timer.h"
#ifdef F031K6
#include "stm32f031x6.h"
#else
#include "stm32f446xx.h"
#endif

#include <stdint.h>

// Interrupt toggles this flag; when button is pressed,
// the flag should be toggled.
static volatile uint8_t led_on = 0;

// Speed of the PLL, differs depending on device
static uint32_t core_clock_hz = 0;

// Note: the name of this interrupt handler
// must match the definition in the vector table!
void TIM2_handler(void) {
  // Timer 2 update interrupt event
  if (TIM2->SR & TIM_SR_UIF) {
    TIM2->SR &= ~(TIM_SR_UIF);
    LED_PORT->ODR ^= (1 << LED_PIN);
  }
}

int main(void) {
#ifdef F031K6
  // Enable GPIO B port for LED
  RCC->AHBENR |= RCC_AHBENR_GPIOBEN;
  // Enable the TIM2 clock
  RCC->APB1ENR |= RCC_APB1ENR_TIM2EN;

  // Enable one wait state for the Flash access time
  FLASH->ACR &= ~(0x00000017);
  FLASH->ACR |= (FLASH_ACR_LATENCY | FLASH_ACR_PRFTBE);

  // Configure PLL to (HSI / 2) * 12 = 48MHz
  RCC->CFGR &= ~(RCC_CFGR_PLLMUL | RCC_CFGR_PLLSRC);
  RCC->CFGR |= (RCC_CFGR_PLLSRC_HSI_DIV2 | RCC_CFGR_PLLMUL12);
#else
  // Enable GPIO A port for LED
  RCC->AHB1ENR |= RCC_AHB1ENR_GPIOAEN;
  // Enable the TIM2 clock (TIM5 could also be used,
  // since both are 32-bit clocks)
  RCC->APB1ENR |= RCC_APB1ENR_TIM2EN;

  // Enable wait states for Flash access time
  // If HCLK is 48MHz, assuming voltage 2V7-3V6, use one wait state
  // (Table 3.4.1 of reference manual)
  FLASH->ACR &= ~(0x0000010F);
  FLASH->ACR |= (FLASH_ACR_LATENCY_1WS | FLASH_ACR_PRFTEN);

  // Configure PLL to use HSI (16MHz RC oscillator)
  // formula is PLL frequency = HSI / M * N / P
  // HSI / M = 2 for stability; 100 < HSI / M * N < 432
  // (16 / 8) * 96 / 4 = 48Mhz
  RCC->PLLCFGR &= ~(RCC_PLLCFGR_PLLM | RCC_PLLCFGR_PLLN |
                    RCC_PLLCFGR_PLLP | RCC_PLLCFGR_PLLSRC);
  RCC->PLLCFGR |= (RCC_PLLCFGR_PLLSRC_HSI | RCC_PLLCFGR_PLLM_3 |
                   RCC_PLLCFGR_PLLN_5 | RCC_PLLCFGR_PLLN_6 |
                   RCC_PLLCFGR_PLLP_0);
#endif

  // Turn on PLL and wait for it to be ready
  RCC->CR |= RCC_CR_PLLON;
  while (!(RCC->CR & RCC_CR_PLLRDY)) {}

  // Set the PLL as the system clock source
  // (This also sets the core clock frequency, HCLK, on F446)
  RCC->CFGR &= ~(RCC_CFGR_SW);
  RCC->CFGR |= RCC_CFGR_SW_PLL;
  core_clock_hz = 48000000;

  // LED is push-pull output on Pin B3 on F031K6
  // or Pin A5 on F446RE
  LED_PORT->MODER &= ~(0x3 << LED_MODE_POS);
  LED_PORT->MODER |= (0x1 << LED_MODE_POS);
  LED_PORT->OTYPER &= ~(0x1 << LED_PIN);

  // Enable the NVIC interrupt; priority 3 is lowest
  NVIC_SetPriority(TIM2_IRQn, 0x03);
  NVIC_EnableIRQ(TIM2_IRQn);

  start_timer(TIM2, core_clock_hz, 1000);

  while (1)  {
  }

  return 0;
}

