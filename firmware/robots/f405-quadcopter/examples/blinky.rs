#![no_std]
#![no_main]

use embassy_stm32::timer::low_level::Timer;
// upon panic, reset the chip
use panic_reset as _;

use embassy_executor::Spawner;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    use embassy_stm32::rcc::*;
    use embassy_stm32::time::Hertz;

    // set up the clocks
    let mut config = embassy_stm32::Config::default();
    {
        config.rcc.hse = Some(Hse {
            freq: Hertz(8_000_000),
            mode: HseMode::Oscillator,
        });
        config.rcc.pll_src = PllSource::HSE;
        config.rcc.pll = Some(Pll {
            prediv: PllPreDiv::DIV4,
            mul: PllMul::MUL168,
            divp: Some(PllPDiv::DIV2), // 8mhz / 4 * 168 / 2 = 168Mhz.
            divq: Some(PllQDiv::DIV7), // 8mhz / 4 * 168 / 7 = 48Mhz.
            divr: None,
        });
        config.rcc.ahb_pre = AHBPrescaler::DIV1;
        config.rcc.apb1_pre = APBPrescaler::DIV4;
        config.rcc.apb2_pre = APBPrescaler::DIV2;
        config.rcc.sys = Sysclk::PLL1_P;
        config.rcc.mux.clk48sel = mux::Clk48sel::PLL1_Q;
    }
    let peripherals = embassy_stm32::init(config);

    // 
    use embassy_stm32::gpio::{Level, Output, Speed};

    // blink the light (forever)
    let mut led1 = Output::new(peripherals.PC14, Level::High, Speed::Low);
    loop {
        use embassy_time::Timer;

        // for _ in 0..30_000_000 {
            led1.set_high();
        // }
        Timer::after_millis(500).await;
        // for _ in 0..30_000_000 {
            led1.set_low();
        // }
        Timer::after_millis(500).await;
    }
}
