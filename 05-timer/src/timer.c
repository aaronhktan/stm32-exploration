#include "timer.h"

// Timer function!
void start_timer(TIM_TypeDef *TIMx,
                 uint32_t core_clock_hz,
                 uint16_t ms) {
  // Disable timer counter first
  TIMx->CR1 &= (TIM_CR1_CEN);

  // Reset the peripheral
  RCC->APB1RSTR |= RCC_APB1RSTR_TIM2RST;
  RCC->APB1RSTR &= ~(RCC_APB1RSTR_TIM2RST);

  TIMx->PSC = core_clock_hz / 1000;
  TIMx->ARR = ms;
  // "Update Generation" event, tells timer to reset and use new values
  TIMx->EGR |= TIM_EGR_UG;
  // Enable hardware interrupt
  TIMx->DIER |= TIM_DIER_UIE;
  // Enable timer
  TIMx->CR1 |= TIM_CR1_CEN;
}

void stop_timer(TIM_TypeDef *TIMx) {
  // Turn off timer counter
  TIMx->CR1 &= ~(TIM_CR1_CEN);
  // Clear the 'pending update interrupt' flag
  // (just in case!)
  TIMx->SR &= ~(TIM_SR_UIF);
}

