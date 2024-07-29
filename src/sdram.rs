//! SDRAM peripheral on F469 discovery board
//! 16MByte (128Mbit) @ 0xC0000000

use crate::fmc_sdram::FmcSdram;
use crate::hal::pac::FMC;

pub struct Sdram {
    //    pins: PINS,
    sdram: FmcSdram,
    size: u32,
}

impl Sdram {
    pub fn new(fmc: FMC) -> Self {
        Sdram {
            sdram: FmcSdram::new(fmc),
            size: 16,
        }
    }
}
