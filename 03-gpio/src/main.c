#include "main.h"

int main(void) {
  // RCC is a pointer to an RCC_TypeDef
  // It is the memory location of the mmap'ed reset and clock controller
  // AHBENR controls the "AHB peripheral clock register"
  // and setting GPIOBEN enables GPIO B 
#ifdef F031K6
  RCC->AHBENR |= RCC_AHBENR_GPIOBEN;
#else
  // On the F446RE, since the user LED is on GPIO A port, need to
  // enable that one as well.
  RCC->AHB1ENR |= RCC_AHB1ENR_GPIOAEN;
  RCC->AHB1ENR |= RCC_AHB1ENR_GPIOBEN;
#endif

  // Button is pull-up input on Pin B1
  GPIOB->MODER &= ~(0x3 << GPIO_MODER_MODER1_Pos);
  // Zero out the bits for PUPDR before setting
  GPIOB->PUPDR &= ~(0x3 << BUTTON_PUPD_POS);
  GPIOB->PUPDR |= (0x1 << BUTTON_PUPD_POS);

  // LED is push-pull output on Pin B3 on F031K6
  // or Pin A5 on F446RE
  LED_PORT->MODER &= ~(0x3 << LED_MODE_POS);
  LED_PORT->MODER |= (0x1 << LED_MODE_POS);
  LED_PORT->OTYPER &= ~(0x1 << LED_PIN);

  // Poll for input on button and then set output
  while (1) {
    // Need to invert input because LOW is pressed
    uint16_t idr_val = ~GPIOB->IDR;
    // Turn on LED when button is pressed, leave off otherwise
    if (idr_val & (1 << BUTTON_PIN)) {
      LED_PORT->ODR |= (0x1 << LED_PIN);
    } else {
      LED_PORT->ODR &= ~(0x1 << LED_PIN);
    }
  }
}

