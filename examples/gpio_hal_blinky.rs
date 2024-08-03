//! Cycle through the LEDs on the board in order
#![no_main]
#![no_std]

use panic_probe as _;

use defmt_rtt as _;

use stm32f469i_disc as board;

use crate::board::{hal::pac, hal::prelude::*, led::Leds};

use cortex_m::peripheral::Peripherals;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (pac::Peripherals::take(), Peripherals::take()) {
        let gpiod = p.GPIOD.split();
        let gpiog = p.GPIOG.split();
        let gpiok = p.GPIOK.split();

        let rcc = p.RCC.constrain();

        let clocks = rcc.cfgr.sysclk(180.MHz()).freeze();

        let mut delay = cp.SYST.delay(&clocks);
        let pause = 200_u32;

        let mut leds = Leds::new(gpiod, gpiog, gpiok);

        loop {
            for led in leds.iter_mut() {
                led.on();
                delay.delay_ms(pause);
            }

            for led in leds.iter_mut() {
                led.off();
                delay.delay_ms(pause);
            }
        }
    }

    loop {
        continue;
    }
}
