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
    // ~1s period; STM32F4 by default uses the 16MHz HSI on boot
    // (See section 6.2.2 in the reference manual)
    syst.set_reload(16_000_000);
    syst.enable_counter();

    // Set up GPIO pin A5 as push-pull output
    let p = stm32f446::Peripherals::take().unwrap();
    let rcc = p.RCC;
    // rcc.iopenr is the GPIO clock enable register
    // |x| is closure notation in Rust
    rcc.ahb1enr.write(|w| w.gpioaen().set_bit());

    // Set moder on fifth pin of GPIOB to 0b01, output
    let gpioa = p.GPIOA;
    unsafe { gpioa.moder.write(|w| w.moder5().bits(0b01)); }
    // Set typer on fifth pin as push-pull output
    gpioa.otyper.write(|w| w.ot5().clear_bit());

    // Restart the SysTick counter
    syst.clear_current();

    loop {
      // Toggle the LED every second SysTick tick
      // (Can't double the period because maximum reload value is 0x0ffffff)
      while !syst.has_wrapped() {};
      while !syst.has_wrapped() {};
      gpioa.odr.write(|w| w.odr5().set_bit());
      while !syst.has_wrapped() {};
      while !syst.has_wrapped() {};
      gpioa.odr.write(|w| w.odr5().clear_bit());      
    }
}
