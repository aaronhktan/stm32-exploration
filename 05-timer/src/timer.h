#ifdef F031K6
#include "stm32f031x6.h"
#else
#include "stm32f446xx.h"
#endif

void start_timer(TIM_TypeDef *TIMx,
                 uint32_t core_clock_hz,
                 uint16_t ms);
void stop_timer(TIM_TypeDef *TIMx);
