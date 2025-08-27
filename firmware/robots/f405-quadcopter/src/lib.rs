#![no_std]

use cortex_m::peripheral;

pub fn clock_config() -> embassy_stm32::Config {
    // clock configuration for embassy runtime (w/ support USB serial)
    let mut config = embassy_stm32::Config::default();
    {
        use embassy_stm32::rcc::*;
        use embassy_stm32::time::Hertz;

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
    return config;
}

pub mod usb_serial {
    pub fn config<'a>() -> embassy_usb::Config<'a> {
        // create serial device description
        let mut config = embassy_usb::Config::new(0xc0de, 0xcafe);
        config.manufacturer = Some("rusty-robot");
        config.product = Some("f405-usb-serial");

        return config;
    }

    pub struct UsbSerial<'a> {
        config_descriptor: &'a mut [u8; 256],
        bos_descriptor: &'a mut [u8; 256],
        control_buffer: &'a mut [u8; 64],
    }

    impl<'a> UsbSerial<'a> {
        pub fn new<D: embassy_usb::driver::Driver<'a>>(driver: D) -> Self {

            let s = Self {
                // FIXME How do I create these arrays?
                config_descriptor: todo!(),
                bos_descriptor: todo!(),
                control_buffer: todo!(),
            };

            let mut builder = embassy_usb::Builder::new(
                driver,
                config(),
                s.config_descriptor,
                s.bos_descriptor,
                &mut [],
                s.control_buffer,
            );

            // Create classes on the builder.
            // let mut state = embassy_usb::class::cdc_acm::State::new();
            // let mut class =
            //     embassy_usb::class::cdc_acm::CdcAcmClass::new(&mut builder, &mut state, 64);
            // // Build the builder.
            // let mut usb_serial = builder.build();

            return s;
        }
    }
}
