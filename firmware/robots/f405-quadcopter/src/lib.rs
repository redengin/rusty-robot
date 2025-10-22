#![no_std]


pub struct F405Quadcopter {
    // imu_dev: embedded_hal_bus::spi::ExclusiveDevice,
}
impl F405Quadcopter {
    pub fn new(
        // imu_dev: embedded_hal_bus::spi::ExclusiveDevice,
    ) -> Self
    {
        F405Quadcopter {
            // imu_dev,
        }
    }
}

use rusty_robot_drivers::imu_traits::{ImuReader, ImuData};
impl ImuReader for F405Quadcopter {
    fn get_data(&self) -> Result<ImuData, &str>
    {
        Err("not implemented")
    }
    
    fn stop(&self) -> Result<(), &str> {
        Err("not implemented")
    }
}

use rusty_robot_drivers::{gps_traits::Gps, nmea::Nmea};
impl Gps for F405Quadcopter {
    fn get_data(&self) -> Result<Nmea, &str> {
        Err("not implemented")
    }
}

use rusty_robot_systems::QuadCopterMotors;
impl QuadCopterMotors for F405Quadcopter {
    fn set_data(&self, velocities_pct: [u8; 4]) {
        todo!()
    }
}



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
    const EP_OUT_BUFFER_SIZE: usize = 10 * 1024;
    // provide a static so that usb_driver can be used in threads
    pub static EP_OUT_BUFFER: static_cell::StaticCell<[u8; EP_OUT_BUFFER_SIZE]> =
        static_cell::StaticCell::new();

    /// logging support for USB serial using embassy task
    /// use `socat - /dev/ttyACM*` to read the stream
    #[embassy_executor::task]
    pub async fn logger_task(
        driver: embassy_stm32::usb::Driver<'static, embassy_stm32::peripherals::USB_OTG_FS>,
    ) {
        // create the LOGGER
        use embassy_usb_logger::DummyHandler;
        static LOGGER: embassy_usb_logger::UsbLogger<EP_OUT_BUFFER_SIZE, DummyHandler> =
            embassy_usb_logger::UsbLogger::with_custom_style(log_style);
        // provide the global logger interface
        unsafe {
            // FIXME choose log level(s) from environment
            let _ = ::log::set_logger_racy(&LOGGER)
                .map(|()| log::set_max_level_racy(log::LevelFilter::Debug));
        }
        // run the logger service
        LOGGER
            .run(&mut ::embassy_usb_logger::LoggerState::new(), driver)
            .await;

        // provide styling for log messages
        // TODO use a standardized style
        fn log_style(
            record: &log::Record,
            writer: &mut embassy_usb_logger::Writer<EP_OUT_BUFFER_SIZE>,
        ) {
            use core::fmt::Write;
            let level = record.level().as_str();
            let target = record.target();
            // log level priority is descending
            if (record.level() < log::LevelFilter::Debug) || record.file().is_none() {
                write!(writer, "{level:>5}:{target}:{}\n", record.args()).unwrap();
            } else {
                // provide extra info for debug and below
                let file = match record.file() {
                    Some(v) => v,
                    None => "",
                };
                let line = match record.line() {
                    Some(v) => v,
                    None => 0,
                };
                write!(
                    writer,
                    "{level:>5}:{target}:{} [{file}:{line}]\n",
                    record.args()
                )
                .unwrap();
            }
        }
    }
}
