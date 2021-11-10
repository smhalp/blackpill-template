#![no_main]
#![no_std]

use panic_semihosting as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::stm32;

#[entry]
fn main() -> ! {
    let _peripherals = cortex_m::Peripherals::take().unwrap();
    let _device = stm32::Peripherals::take().unwrap();

    hprintln!("Hello semihosting world");
    let mut x = 0;
    loop {
        x += 1;
        hprintln!("{} loop iteration{}", x, if x > 1 { "s" } else { "" })
    }
}
