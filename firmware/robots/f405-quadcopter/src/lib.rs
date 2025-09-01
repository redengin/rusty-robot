#![no_std]


use embassy_usb_logger::{DummyHandler, UsbLogger};

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
    const USB_LOG_BUFFER_SZ: usize = 1024;

    // unformatted logs
    // embassy_usb_logger::run!(USB_LOG_BUFFER_SZ, log::LevelFilter::Info, driver);

    /// does not support receiving data over USB serial
    use embassy_usb_logger::DummyHandler;
    static LOGGER: embassy_usb_logger::UsbLogger<USB_LOG_BUFFER_SZ, DummyHandler> =
        embassy_usb_logger::UsbLogger::new();
    // provide the global logger interface
    unsafe {
        // FIXME choose log level(s) from environment
        let _ = ::log::set_logger_racy(&LOGGER).map(|()| log::set_max_level_racy(log::LevelFilter::Debug));
    }
    LOGGER.run(&mut ::embassy_usb_logger::LoggerState::new(), driver).await;


    // provide formatted logs
    // let logger: UsbLogger<USB_LOG_BUFFER_SZ, DummyHandler> =
    //     // embassy_usb_logger::UsbLogger::with_custom_style(log_style);
    //     embassy_usb_logger::UsbLogger::with_custom_style(
    //         |_record, writer| {
    //             writer.write_str("hello world").unwrap();
    //         }
    //     );

    // logger.run(&mut ::embassy_usb_logger::LoggerState::new(), driver).await;

    // fn log_style(record: &log::Record, writer: &mut embassy_usb_logger::Writer<USB_LOG_BUFFER_SZ>) {
    // }
}
