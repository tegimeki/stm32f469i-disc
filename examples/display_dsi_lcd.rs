//! This example initializes the STM32F469I-DISCO LCD and draws to it.
//!
//! Run command: cargo embed --release --features="stm32f469,dsihost,log,ltdc,fmc,example-smps,log-rtt,rt,rtc" --example display_dsi_lcd

//#![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;

use stm32f469i_disc as board;

use core::{mem, slice};

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::{entry, exception};

use panic_probe as _;

use rtt_target::{rprintln, rtt_init_print};

use stm32_fmc::devices::is42s32400f_6;

use crate::board::hal::gpio::alt::fmc as alt;
use crate::board::hal::{fmc::FmcExt, pac, prelude::*};

use stm32f4xx_hal::ltdc::{DisplayConfig, DisplayController, Layer, PixelFormat};

use otm8009a::{Otm8009A, Otm8009AConfig};

use stm32f4xx_hal::dsi::{
    ColorCoding, DsiChannel, DsiCmdModeTransmissionKind, DsiConfig, DsiHost, DsiInterrupts,
    DsiMode, DsiPhyTimers, DsiPllConfig, DsiVideoMode, LaneCount,
};

pub const WIDTH: usize = 480;
pub const HEIGHT: usize = 800;

pub const DISPLAY_CONFIGURATION: DisplayConfig = DisplayConfig {
    active_width: WIDTH as _,
    active_height: HEIGHT as _,
    h_back_porch: 34,
    h_front_porch: 34,
    v_back_porch: 15,
    v_front_porch: 16,
    h_sync: 2,
    v_sync: 1,
    frame_rate: 60,
    h_sync_pol: true,
    v_sync_pol: true,
    no_data_enable_pol: false,
    pixel_clock_pol: true,
};

fn hue2rgb(hue: u32, level: u32) -> u32 {
    let hue = hue % 360;
    let sector: u32 = hue / 60;
    let fraction = hue % 60;
    let none = 0;
    let full = level;
    let rise = (level * fraction) / 60;
    let fall = (level * (60 - fraction)) / 60;
    let rgb = match sector {
        0 => (full, rise, none),
        1 => (fall, full, none),
        2 => (none, full, rise),
        3 => (none, fall, full),
        4 => (rise, none, full),
        5 => (full, none, fall),
        _ => (none, none, none),
    };
    rgb.2 | (rgb.1 << 8) | (rgb.0 << 16)
}

/// Configure pins for the FMC controller
macro_rules! fmc_pins {
    ($($alt:ident: $pin:expr,)*) => {
        (
            $(
                alt::$alt::from($pin.internal_pull_up(true))
            ),*
        )
    };
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut cp = Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();

    let hse_freq = 8.MHz();
    let clocks = rcc
        .cfgr
        .use_hse(hse_freq)
        // NOTE: possible PLL setup issue, requiring we limit AB2 clock
        .pclk2(32.MHz())
        .sysclk(180.MHz())
        .freeze();
    let mut delay = cp.SYST.delay(&clocks);

    cp.SCB.invalidate_icache();
    cp.SCB.enable_icache();

    let gpioa = dp.GPIOA.split();
    let gpioc = dp.GPIOC.split();
    let gpiod = dp.GPIOD.split();
    let gpioe = dp.GPIOE.split();
    let gpiof = dp.GPIOF.split();
    let gpiog = dp.GPIOG.split();
    let gpioh = dp.GPIOH.split();
    let gpioi = dp.GPIOI.split();

    #[rustfmt::skip]
    let pins = fmc_pins! {
        A0: gpiof.pf0, A1: gpiof.pf1, A2: gpiof.pf2, A3: gpiof.pf3,
        A4: gpiof.pf4, A5: gpiof.pf5, A6: gpiof.pf12, A7: gpiof.pf13,
        A8: gpiof.pf14, A9: gpiof.pf15, A10: gpiog.pg0, A11: gpiog.pg1,
        Ba0: gpiog.pg4, Ba1: gpiog.pg5,
        D0: gpiod.pd14, D1: gpiod.pd15, D2: gpiod.pd0, D3: gpiod.pd1,
        D4: gpioe.pe7, D5: gpioe.pe8, D6: gpioe.pe9, D7: gpioe.pe10,
        D8: gpioe.pe11, D9: gpioe.pe12, D10: gpioe.pe13, D11: gpioe.pe14,
        D12: gpioe.pe15, D13: gpiod.pd8, D14: gpiod.pd9, D15: gpiod.pd10,
        D16: gpioh.ph8, D17: gpioh.ph9, D18: gpioh.ph10, D19: gpioh.ph11,
        D20: gpioh.ph12, D21: gpioh.ph13, D22: gpioh.ph14, D23: gpioh.ph15,
        D24: gpioi.pi0, D25: gpioi.pi1, D26: gpioi.pi2, D27: gpioi.pi3,
        D28: gpioi.pi6, D29: gpioi.pi7, D30: gpioi.pi9, D31: gpioi.pi10,
        Nbl0: gpioe.pe0, Nbl1: gpioe.pe1, Nbl2: gpioi.pi4, Nbl3: gpioi.pi5,
        Sdcke0: gpioh.ph2, Sdclk: gpiog.pg8,
        Sdncas: gpiog.pg15, Sdne0: gpioh.ph3,
        Sdnras: gpiof.pf11, Sdnwe: gpioc.pc0,
    };

    rtt_init_print!();
    rprintln!("Initializing SDRAM...\r");

    let mut sdram = dp.FMC.sdram(pins, is42s32400f_6::Is42s32400f6 {}, &clocks);
    let sdram_size = 16 * 1024 * 1024;
    let ram_ptr: *mut u32 = sdram.init(&mut delay);

    let framebuffer = unsafe { slice::from_raw_parts_mut(ram_ptr, WIDTH * HEIGHT) };

    // Reset display
    rprintln!("Resetting LCD...\r");
    let mut lcd_reset = gpioh.ph7.into_push_pull_output();
    lcd_reset.set_low();
    delay.delay_ms(20u32);
    lcd_reset.set_high();
    delay.delay_ms(10u32);

    // Initialize LTDC
    rprintln!("Intializing display controller...\r");
    let ltdc_freq = 27_429.kHz();
    let mut display = DisplayController::<u32>::new(
        dp.LTDC,
        dp.DMA2D,
        None,
        PixelFormat::ARGB8888,
        DISPLAY_CONFIGURATION,
        Some(hse_freq),
    );

    display.config_layer(Layer::L1, framebuffer, PixelFormat::ARGB8888);
    display.enable_layer(Layer::L1);
    display.reload();

    // Initialize DSI Host
    // VCO = (8MHz HSE / 2 IDF) * 2 * 125 = 1000MHz
    // 1000MHz VCO / (2 * 1 ODF * 8) = 62.5MHz
    let dsi_pll_config = unsafe {
        DsiPllConfig::manual(125, 2, 0 /*div1*/, 4)
    };

    rprintln!("Initializing DSI... ");
    let dsi_config = DsiConfig {
        mode: DsiMode::Video {
            mode: DsiVideoMode::Burst,
        },
        lane_count: LaneCount::DoubleLane,
        channel: DsiChannel::Ch0,
        hse_freq,
        ltdc_freq,
        interrupts: DsiInterrupts::None,
        color_coding_host: ColorCoding::TwentyFourBits,
        color_coding_wrapper: ColorCoding::TwentyFourBits,
        lp_size: 4,
        vlp_size: 4,
    };

    let mut dsi_host = DsiHost::init(
        dsi_pll_config,
        DISPLAY_CONFIGURATION,
        dsi_config,
        dp.DSI,
        &clocks,
    )
    .unwrap();

    dsi_host.configure_phy_timers(DsiPhyTimers {
        dataline_hs2lp: 35,
        dataline_lp2hs: 35,
        clock_hs2lp: 35,
        clock_lp2hs: 35,
        dataline_max_read_time: 0,
        stop_wait_time: 10,
    });

    dsi_host.set_command_mode_transmission_kind(DsiCmdModeTransmissionKind::AllInLowPower);
    dsi_host.start();
    dsi_host.enable_bus_turn_around(); // Must be before read attempts

    let otm8009a_config = Otm8009AConfig {
        frame_rate: otm8009a::FrameRate::_60Hz,
        mode: otm8009a::Mode::Portrait, // to avoid tearing present in landscape mode
        color_map: otm8009a::ColorMap::Rgb,
        cols: WIDTH as u16,
        rows: HEIGHT as u16,
    };
    let mut otm8009a = Otm8009A::new();
    let result = otm8009a.init(&mut dsi_host, otm8009a_config, &mut delay);
    match result {
        Ok(_) => rprintln!("OTM8009A Init: OK\r"),
        Err(e) => rprintln!("OTM8009A Error: {:?}", e),
    }
    otm8009a.enable_te_output(533, &mut dsi_host).unwrap();

    // Not sure if this is needed
    dsi_host.set_command_mode_transmission_kind(DsiCmdModeTransmissionKind::AllInHighSpeed);
    dsi_host.force_rx_low_power(true);
    dsi_host.refresh();

    let words = sdram_size / mem::size_of::<u32>();
    let fb = unsafe { slice::from_raw_parts_mut(ram_ptr, words) };

    // rolling gradient display
    let mut hue = 0;
    let ratio = 3;
    let speed = 3;
    loop {
        let mut addr = 0;
        for row in 0..HEIGHT {
            let rgb = hue2rgb((hue + row as u32) / ratio, 255);
            for _col in 0..WIDTH {
                fb[addr] = rgb;
                addr += 1;
            }
        }
        hue += speed * if gpioa.pa0.is_high() { 5 } else { 1 };
        delay.delay_ms(15u32);
    }
}

#[exception]
unsafe fn HardFault(ef: &cortex_m_rt::ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

#[exception]
unsafe fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
