#include "main.h"

#include "timer.h"

#include "stm32f446xx.h"
#include "FreeRTOS.h"
#include "task.h"

#include <stdint.h>

// Have two different delays for the tasks
static const int led_delay_1 = 888;
static const int led_delay_2 = 500;

// Blinking LED task
static void led_task(void *args) {
  int delay_ms = *(int *)args;

  while (1) {
    LED_PORT->ODR ^= (1 << LED_PIN);
    // pdMS_TO_TICKS is a macro defined by FreeRTOS
    // Since it calculates time in ticks rather than milliseconds
    vTaskDelay(pdMS_TO_TICKS(delay_ms));
  }
}

int main(void) {
  // Set up clocks
  setup_timer();

  // Enable GPIO A port for LED
  RCC->AHB1ENR |= RCC_AHB1ENR_GPIOAEN;

  // LED is push-pull output on Pin B3 on F031K6
  // or Pin A5 on F446RE
  LED_PORT->MODER &= ~(0x3 << LED_MODE_POS);
  LED_PORT->MODER |= (0x1 << LED_MODE_POS);
  LED_PORT->OTYPER &= ~(0x1 << LED_PIN);
  LED_PORT->OSPEEDR &= ~(0x3 << (LED_MODE_POS));
  LED_PORT->PUPDR &= ~(1 << LED_PIN);
  LED_PORT->ODR |= (1 << LED_PIN);

  // Create and run LED tasks
  xTaskCreate(led_task, "LED_blink_1", 128, (void *)&led_delay_1,
              configMAX_PRIORITIES-1, NULL);
  xTaskCreate(led_task, "LED_blink_2", 128, (void *)&led_delay_2,
              configMAX_PRIORITIES-1, NULL);

  // Start the scheduler
  vTaskStartScheduler();

  while (1)  {}

  return 0;
}

