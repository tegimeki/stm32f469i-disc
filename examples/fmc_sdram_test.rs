//! Test the on-board SDRAM
#![no_main]
#![no_std]

use panic_probe as _;

use stm32f469i_disc as board;

use crate::board::{hal::pac, hal::prelude::*, sdram::Sdram};

use cortex_m::peripheral::Peripherals;

use cortex_m_rt::entry;

use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (pac::Peripherals::take(), Peripherals::take()) {
        let rcc = p.RCC.constrain();

        let _clocks = rcc.cfgr.sysclk(180.MHz()).freeze();

        let _sdram = crate::board::sdram::Sdram::new(p.FMC);

        rtt_init_print!();
        let something = 12;
        rprintln!("Testing SDRAM...{}\r", something);
    }
    loop {}
}
