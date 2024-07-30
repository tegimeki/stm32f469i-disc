stm32f469i-disc
===============
_stm32f469i-disc_ provides a board support package for the STM32F469I-DISCOVERY
kit.  Programming is provided by `probe-rs`; see `.cargo/config.toml`.

Peripheral Support
------------------
- [x] Green, Orange, Red, Blue user LEDs
- [ ] OTM8009A LCD (requires FMC SDRAM, DSI host, LTDC driver in stm32f4xx-hal - only the latter are ported so I am working on SDRAM+DSI in separate repos/branches)
- [ ] FT6206 touch controller (i2c)
- [ ] Other standard peripherals (my initial goal is to get the display up)

Credits
-------
Thanks to the authors of [stm32f429i-disc](https://github.com/stm32-rs/stm32f429i-disc.git) and [stm32f407g-disc](https://github.com/stm32-rs/stm32f407g-disc.git) crates for solid starting points.

License
-------

[0-clause BSD license](LICENSE-0BSD.txt).
