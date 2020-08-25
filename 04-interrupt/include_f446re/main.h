// STM32F446 header included from Cube package
// Drivers/CMSIS/Include
#include "stm32f446xx.h"

#include <stdint.h>

// On the F446RE, B1 is the eighth pin from the bottom
// on the Morpho headers (doesn't seem to be on the
// Arduino headers for some reason...)
#define BUTTON_PIN          1
#define BUTTON_PUPD_POS     GPIO_PUPDR_PUPD1_Pos

#define LED_PORT            GPIOA
#define LED_PIN             5
#define LED_MODE_POS        GPIO_MODER_MODER5_Pos

