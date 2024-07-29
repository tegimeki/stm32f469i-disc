#![no_std]
#![allow(non_camel_case_types)]

pub use stm32f4xx_hal as hal;

pub use crate::hal::pac::interrupt::*;
pub use crate::hal::pac::Interrupt;
pub use crate::hal::pac::Peripherals;

pub mod fmc_sdram; // TODO: move to stmf4xx-hal
pub mod led;
pub mod sdram;
