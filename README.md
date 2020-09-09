# STM32 Experiments

A set of code examples and comments to teach myself a bit more about microcontrollers.

## Info

I've adapted most of the code by following along [Vivonomicon's](https://vivonomicon.com/category/stm32_baremetal_examples/) STM32 examples.

The additions to the code presented in the examples are:
- Porting the examples to other boards (specifically, Nucleo STM32F446RET6)
- Adding comments for parts that were new to me or to explain certain aspects in more detail
- Modifications and additions to make the Makefile support multiple targets (F031K6 and F446RE)

## Contents

This repository contains code samples and examples to understand aspects of embedded programming from first principles.
- `01-basic` contains **linker scripts** and **ARM Thumb assembly**.
- `02-main` contains **vector tables** and **C**. It also explores the **ELF binary format**.
- `03-gpio` takes input via a button and toggles an LED with **memory-mapped IO**.
- `04-interrupt` does the same thing as `03-gpio` but uses **interrupts** using the **NVIC** and **EXTI** peripherals.
- `05-timer` explores the **clock peripherals**, specifically using the **16MHz HSI oscillator** to drive a **PLL** and core clock (HCLK).
- `06-freertos` explores configuration of and using **FreeRTOS** to blink LEDs at intervals.

## Build and run
Development was done on macOS, so all the makefiles work on macOS. I have not verified that they work on Linux.

To build, `make <target-name>` should work, where target-name is either `f031k6` or `f446re`.

To run,
- Run `st-util`.
- Type `arm-none-eabi-gdb <output-name.elf>`.
- In GDB, `target extended-remote :4242`.
- `load`, and then `continue`.

## Future
- I intend to continue learning more about embedded programming with this series! A few fundamentals that I haven't had the time to include the basic communication peripherals (UART, SPI, I2C).
