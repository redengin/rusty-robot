#![no_std]

/// initializes the hardware via embassy
pub fn init() -> embassy_stm32::Peripherals {
    //! uses the internal oscillator
    let mut clock_config = embassy_stm32::Config::default();
    {
        use embassy_stm32::rcc::*;
        use embassy_stm32::time::Hertz;

        clock_config.rcc.hse = Some(Hse {
            freq: Hertz(8_000_000),
            mode: HseMode::Oscillator,
        });
        clock_config.rcc.pll_src = PllSource::HSE;
        clock_config.rcc.pll = Some(Pll {
            prediv: PllPreDiv::DIV4,
            mul: PllMul::MUL168,
            divp: Some(PllPDiv::DIV2), // 8mhz (hse) / 4 * 168 / 2 = 168Mhz.
            divq: Some(PllQDiv::DIV7), // 8mhz (hse) / 4 * 168 / 7 = 48Mhz.
            divr: None,
        });
        clock_config.rcc.ahb_pre = AHBPrescaler::DIV1;
        clock_config.rcc.apb1_pre = APBPrescaler::DIV4;
        clock_config.rcc.apb2_pre = APBPrescaler::DIV2;
        clock_config.rcc.sys = Sysclk::PLL1_P;
        clock_config.rcc.mux.clk48sel = mux::Clk48sel::PLL1_Q;
    };

    // initialize the hardware using embassy
    embassy_stm32::init(clock_config)
}

pub mod usb {
    // provide a static so that usb_driver can be used in threads
    pub static EP_OUT_BUFFER: static_cell::StaticCell<[u8; 256]> = static_cell::StaticCell::new();
}

/// log support for USB serial
#[embassy_executor::task]
pub async fn usb_logger_task(
    driver: embassy_stm32::usb::Driver<'static, embassy_stm32::peripherals::USB_OTG_FS>,
) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);

    // TODO provide formatted logs
    // use embassy_usb_logger::{UsbLogger, DummyHandler};
    // let logger: UsbLogger<1024, DummyHandler> = UsbLogger::with_custom_style(custom_style);
}
