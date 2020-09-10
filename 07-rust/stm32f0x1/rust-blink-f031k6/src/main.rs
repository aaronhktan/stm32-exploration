#![no_std]
#![no_main]

extern crate panic_halt;

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! { // ! means no return type
    // Check out the 'Cortex-M Peripherals' singleton
    let cm_p = cortex_m::Peripherals::take().unwrap();
    // Set up the SysTick peripheral
    // Rust variables are immutable by default; use mut to make mutable
    let mut syst = cm_p.SYST;
    syst.set_clock_source(SystClkSource::Core);
    // ~2s period; STM32F0 by default uses the 8MHz HSI on boot
    // (See section 6.2 of the reference manual)
    syst.set_reload(16_000_000);
    syst.enable_counter();

    // Set up GPIO pin B3 as push-pull output
    let p = stm32f0x1::Peripherals::take().unwrap();
    let rcc = p.RCC;
    // rcc.iopenr is the GPIO clock enable register
    // |x| is closure notation in Rust
    rcc.ahbenr.write(|w| w.iopben().set_bit());

    // Set moder on third pin of GPIOB to 0b01, output
    let gpiob = p.GPIOB;
    unsafe { gpiob.moder.write(|w| w.moder3().bits(0b01)); }
    // Set typer on third pin as push-pull output
    gpiob.otyper.write(|w| w.ot3().clear_bit());

    // Restart the SysTick counter
    syst.clear_current();

    loop {
      // Toggle the LED every SysTick tick
      while !syst.has_wrapped() {};
      gpiob.odr.write(|w| w.odr3().set_bit());
      while !syst.has_wrapped() {};
      gpiob.odr.write(|w| w.odr3().clear_bit());      
    }
}
