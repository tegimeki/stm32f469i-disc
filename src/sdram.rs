//! 16MByte SDRAM peripheral on F469 discovery board

use crate::hal;
use crate::hal::{fmc::FmcExt, pac::FMC};
use core::mem;
use stm32_fmc::devices::is42s32400f_6;
use stm32_fmc::{AddressPinSet, PinsSdram, SdramPinSet};
use stm32f4xx_hal::rcc::Clocks;

#[macro_export]
macro_rules! sdram_pins {
    ($c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr) => {
        (
            // address
            alt::A0::from($f.pf0.internal_pull_up(true)),
            alt::A1::from($f.pf1.internal_pull_up(true)),
            alt::A2::from($f.pf2.internal_pull_up(true)),
            alt::A3::from($f.pf3.internal_pull_up(true)),
            alt::A4::from($f.pf4.internal_pull_up(true)),
            alt::A5::from($f.pf5.internal_pull_up(true)),
            alt::A6::from($f.pf12.internal_pull_up(true)),
            alt::A7::from($f.pf13.internal_pull_up(true)),
            alt::A8::from($f.pf14.internal_pull_up(true)),
            alt::A9::from($f.pf15.internal_pull_up(true)),
            alt::A10::from($g.pg0.internal_pull_up(true)),
            alt::A11::from($g.pg1.internal_pull_up(true)),
            // bank
            alt::Ba0::from($g.pg4.internal_pull_up(true)),
            alt::Ba1::from($g.pg5.internal_pull_up(true)),
            // data
            alt::D0::from($d.pd14.internal_pull_up(true)),
            alt::D1::from($d.pd15.internal_pull_up(true)),
            alt::D2::from($d.pd0.internal_pull_up(true)),
            alt::D3::from($d.pd1.internal_pull_up(true)),
            alt::D4::from($e.pe7.internal_pull_up(true)),
            alt::D5::from($e.pe8.internal_pull_up(true)),
            alt::D6::from($e.pe9.internal_pull_up(true)),
            alt::D7::from($e.pe10.internal_pull_up(true)),
            alt::D8::from($e.pe11.internal_pull_up(true)),
            alt::D9::from($e.pe12.internal_pull_up(true)),
            alt::D10::from($e.pe13.internal_pull_up(true)),
            alt::D11::from($e.pe14.internal_pull_up(true)),
            alt::D12::from($e.pe15.internal_pull_up(true)),
            alt::D13::from($d.pd8.internal_pull_up(true)),
            alt::D14::from($d.pd9.internal_pull_up(true)),
            alt::D15::from($d.pd10.internal_pull_up(true)),
            alt::D16::from($h.ph8.internal_pull_up(true)),
            alt::D17::from($h.ph9.internal_pull_up(true)),
            alt::D18::from($h.ph10.internal_pull_up(true)),
            alt::D19::from($h.ph11.internal_pull_up(true)),
            alt::D20::from($h.ph12.internal_pull_up(true)),
            alt::D21::from($h.ph13.internal_pull_up(true)),
            alt::D22::from($h.ph14.internal_pull_up(true)),
            alt::D23::from($h.ph15.internal_pull_up(true)),
            alt::D24::from($i.pi0.internal_pull_up(true)),
            alt::D25::from($i.pi1.internal_pull_up(true)),
            alt::D26::from($i.pi2.internal_pull_up(true)),
            alt::D27::from($i.pi3.internal_pull_up(true)),
            alt::D28::from($i.pi6.internal_pull_up(true)),
            alt::D29::from($i.pi7.internal_pull_up(true)),
            alt::D30::from($i.pi9.internal_pull_up(true)),
            alt::D31::from($i.pi10.internal_pull_up(true)),
            // NBL
            alt::Nbl0::from($e.pe0.internal_pull_up(true)),
            alt::Nbl1::from($e.pe1.internal_pull_up(true)),
            alt::Nbl2::from($i.pi4.internal_pull_up(true)),
            alt::Nbl3::from($i.pi5.internal_pull_up(true)),
            // Control
            alt::Sdcke0::from($h.ph2.internal_pull_up(true)),
            alt::Sdclk::from($g.pg8.internal_pull_up(true)),
            alt::Sdncas::from($g.pg15.internal_pull_up(true)),
            alt::Sdne0::from($h.ph3.internal_pull_up(true)),
            alt::Sdnras::from($f.pf11.internal_pull_up(true)),
            alt::Sdnwe::from($c.pc0.internal_pull_up(true)),
        )
    };
}
pub use sdram_pins;

pub struct Sdram {
    pub mem: *mut u32,
    pub words: usize,
}

impl Sdram {
    pub fn new<BANK: SdramPinSet, ADDR: AddressPinSet, PINS: PinsSdram<BANK, ADDR>>(
        fmc: FMC,
        pins: PINS,
        clocks: &Clocks,
        delay: &mut hal::timer::SysDelay,
    ) -> Self {
        Self {
            mem: fmc
                .sdram(pins, is42s32400f_6::Is42s32400f6 {}, clocks)
                .init(delay),
            words: 16 * 1024 * 1024 / mem::size_of::<u32>(),
        }
    }
}
