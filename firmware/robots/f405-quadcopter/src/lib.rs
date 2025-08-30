#![no_std]

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

pub mod usb {
    use embassy_stm32::peripherals;
    use embassy_stm32::usb;

    use static_cell::StaticCell;
    static EP_OUT_BUFFER: StaticCell<[u8; 256]> = StaticCell::new();

    pub fn driver(
        usb: embassy_stm32::Peri<'static, peripherals::USB_OTG_FS>,
        dp: embassy_stm32::Peri<'static, peripherals::PA12>,
        dm: embassy_stm32::Peri<'static, peripherals::PA11>,
    ) -> usb::Driver<'static, peripherals::USB_OTG_FS> {
        // bind the Irq
        embassy_stm32::bind_interrupts!(struct Irqs {
            OTG_FS => embassy_stm32::usb::InterruptHandler<embassy_stm32::peripherals::USB_OTG_FS>;
        });

        // USB driver (FS: full-speed)
        //--------------------------------------------------------------------------------
        let mut usb_config = embassy_stm32::usb::Config::default();
        // disable vbus_detection - this is a safe default that works in all boards.
        // However, if your USB device is self-powered (can stay powered on if USB is unplugged),
        // you can enable vbus_detection to comply with the USB spec - but the board
        // has to support it or USB won't work at all. See docs on `vbus_detection` for details.
        usb_config.vbus_detection = false;

        embassy_stm32::usb::Driver::new_fs(
            usb,
            Irqs,
            dp,
            dm,
            EP_OUT_BUFFER.init([0; _]),
            usb_config,
        )
    }

    pub mod usb_serial {
        use embassy_stm32::peripherals;
        use embassy_stm32::usb;
        use embassy_usb::UsbDevice;

        use static_cell::StaticCell;
        static CONFIG_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
        static BOS_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
        static CONTROL_BUFFER: StaticCell<[u8; 64]> = StaticCell::new();
        static STATE: StaticCell<embassy_usb::class::cdc_acm::State> = StaticCell::new();

        pub fn device(
            usb_driver: usb::Driver<'static, peripherals::USB_OTG_FS>,
        ) -> (
            UsbDevice<'static, usb::Driver<'static, peripherals::USB_OTG_FS>>,
            embassy_usb::class::cdc_acm::CdcAcmClass<
                'static,
                usb::Driver<'static, peripherals::USB_OTG_FS>,
            >,
        ) {
            // create serial device description
            let mut usb_descriptor = embassy_usb::Config::new(0xc0de, 0xcafe);
            usb_descriptor.manufacturer = Some("rusty-robot");
            usb_descriptor.product = Some("f405-usb-serial");

            // build the USB serial interface w/ class
            let mut builder = embassy_usb::Builder::new(
                usb_driver,
                usb_descriptor,
                CONFIG_DESCRIPTOR.init([0; _]),
                BOS_DESCRIPTOR.init([0; _]),
                &mut [], // no msos descriptors
                CONTROL_BUFFER.init([0; _]),
            );
            // Create classes on the builder.
            let usb_serial_class = embassy_usb::class::cdc_acm::CdcAcmClass::new(
                &mut builder,
                STATE.init(embassy_usb::class::cdc_acm::State::new()),
                64,
            );

            (builder.build(), usb_serial_class)
        }
    }
}
