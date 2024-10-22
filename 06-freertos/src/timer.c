#include "timer.h"

void setup_timer(void) {
#ifdef F031K6
  // Enable the TIM2 clock
  RCC->APB1ENR |= RCC_APB1ENR_TIM2EN;

  // Enable one wait state for the Flash access time
  FLASH->ACR &= ~(0x00000017);
  FLASH->ACR |= (FLASH_ACR_LATENCY | FLASH_ACR_PRFTBE);

  // Configure PLL to (HSI / 2) * 12 = 48MHz
  RCC->CFGR &= ~(RCC_CFGR_PLLMUL | RCC_CFGR_PLLSRC);
  RCC->CFGR |= (RCC_CFGR_PLLSRC_HSI_DIV2 | RCC_CFGR_PLLMUL12);
#else
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
}

