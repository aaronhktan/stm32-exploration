#include "main.h"

// Interrupt toggles this flag; when button is pressed,
// the flag should be toggled.
static volatile uint8_t led_on = 0;

#ifdef F031K6
// Note: the name of this interrupt handler
// must match the definition in the vector table!
void EXTI0_1_handler(void) {
  if (EXTI->PR & (1 << BUTTON_PIN)) {
    // Clear the EXTI status flag
    EXTI->PR |= (1 << BUTTON_PIN);
    led_on = !led_on;
  }
}
#else
void EXTI1_handler(void) {
  if (EXTI->PR & (1 << BUTTON_PIN)) {
    // Clear the EXTI status flag
    EXTI->PR |= (1 << BUTTON_PIN);
    led_on = !led_on;
  }
}
#endif

int main(void) {
#ifdef F031K6
  // Enable GPIO B port for button and LED
  RCC->AHBENR |= RCC_AHBENR_GPIOBEN;
  // Enable the System Configuration peripheral for interrupts
  RCC->APB2ENR |= RCC_APB2ENR_SYSCFGCOMPEN;
#else
  // Enable GPIO A port for LED
  RCC->AHB1ENR |= RCC_AHB1ENR_GPIOAEN;
  // Enable GPIO B port for button
  RCC->AHB1ENR |= RCC_AHB1ENR_GPIOBEN;
  // Enable the System Configuration peripheral for interrupts
  RCC->APB2ENR |= RCC_APB2ENR_SYSCFGEN;
#endif

  // Button is pull-up input on Pin B1
  GPIOB->MODER &= ~(0x3 << GPIO_MODER_MODER1_Pos);
  GPIOB->PUPDR &= ~(0x3 << BUTTON_PUPD_POS);
  GPIOB->PUPDR |= (0x1 << BUTTON_PUPD_POS);

  // LED is push-pull output on Pin B3 on F031K6
  // or Pin A5 on F446RE
  LED_PORT->MODER &= ~(0x3 << LED_MODE_POS);
  LED_PORT->MODER |= (0x1 << LED_MODE_POS);
  LED_PORT->OTYPER &= ~(0x1 << LED_PIN);

  // Enable the interrupt using the EXTI peripheral
  // There are four EXTICR registers in SYSCFG
  // (see section 9.1.2-9.1.6 for F031, 8.2.3-8.2.6 for F446)
  // Set the first EXTI peripheral to check for interrupts on Port B
  SYSCFG->EXTICR[0] &= ~(SYSCFG_EXTICR1_EXTI1_Msk);
  SYSCFG->EXTICR[0] |= (SYSCFG_EXTICR1_EXTI1_PB);

  // EXTI->IMR is the interrupt mask register; enable for button pin
  EXTI->IMR |= (1 << BUTTON_PIN);
  // Disable Rising Trigger Selection Register for button pin
  EXTI->RTSR &= ~(1 << BUTTON_PIN);
  // Enable Falling Trigger Selection Register for button pin
  EXTI->FTSR |= (1 << BUTTON_PIN);

  // Enable the NVIC interrupt; priority 3 is lowest
#ifdef F031K6
  NVIC_SetPriority(EXTI0_1_IRQn, 0x03);
  NVIC_EnableIRQ(EXTI0_1_IRQn);
#else
  NVIC_SetPriority(EXTI1_IRQn, 0x03);
  NVIC_EnableIRQ(EXTI1_IRQn);
#endif

  // Poll for input on button and then set output
  while (1) {
    if (led_on) {
      LED_PORT->ODR |= (0x1 << LED_PIN);
    } else {
      LED_PORT->ODR &= ~(0x1 << LED_PIN);
    }
  }
}

