#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt_macros;
extern crate panic_itm as _;
extern crate stm32f4xx_hal as hal;

use cortex_m::peripheral::Peripherals;
use cortex_m_rt_macros::entry;
use hal::delay::Delay;
use hal::gpio;
use hal::gpio::gpioa::{PA6, PA7};
use hal::prelude::*;
use hal::stm32;

struct Leds {
    a: PA6<gpio::Output<gpio::PushPull>>,
    b: PA7<gpio::Output<gpio::PushPull>>,
}

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) =
        (stm32::Peripherals::take(), Peripherals::take())
    {
        let gpioa = p.GPIOA.split();

        // Configure LED outputs
        let mut leds = Leds {
            a: gpioa.pa6.into_push_pull_output(),
            b: gpioa.pa7.into_push_pull_output(),
        };

        // Constrain clock registers
        let rcc = p.RCC.constrain();

        // Configure clock to 168 MHz (i.e. the maximum) and freeze it
        let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();

        // Get delay provider
        let mut delay = Delay::new(cp.SYST, &clocks);

        loop {
            // only A on
            leds.a.set_low();
            leds.b.set_high();
            delay.delay_ms(250_u16);
            // both on
            leds.b.set_low();
            delay.delay_ms(250_u16);
            // only B on
            leds.a.set_high();
            delay.delay_ms(250_u16);
            // both off
            leds.b.set_high();
            delay.delay_ms(250_u16);
        }
    }

    loop {}
}
