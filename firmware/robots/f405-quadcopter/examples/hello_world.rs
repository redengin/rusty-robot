#![no_std]
#![no_main]

use defmt::*;

// FIXME (undesired implementations)
use panic_probe as _;
// use {defmt_rtt as _, panic_probe as _};

// bind used interrupts to embassy runtime
embassy_stm32::bind_interrupts!(struct Irqs {
    OTG_FS => embassy_stm32::usb::InterruptHandler<embassy_stm32::peripherals::USB_OTG_FS>;
});


use embassy_executor::Spawner;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // clock configuration for embassy runtime (for desired peripherals)
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
    let peripherals = embassy_stm32::init(config);

    // create the full-speed (fs) USB serial driver, from the HAL.
    let mut ep_out_buffer = [0u8; 256];
    let mut config = embassy_stm32::usb::Config::default();
    config.vbus_detection = false;
    let driver = embassy_stm32::usb::Driver::new_fs(
        peripherals.USB_OTG_FS,
        Irqs,
        peripherals.PA12,
        peripherals.PA11,
        &mut ep_out_buffer,
        config,
    );
    // create serial device description
    let mut config = embassy_usb::Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some("rusty-robot");
    config.product = Some("USB-serial example");
    config.serial_number = Some("12345678");
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut control_buf = [0; 64];
    let mut state = embassy_usb::class::cdc_acm::State::new();
    let mut builder = embassy_usb::Builder::new(
        driver,
        config,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut [], // no msos descriptors
        &mut control_buf,
    );
    // Create classes on the builder.
    let mut class = embassy_usb::class::cdc_acm::CdcAcmClass::new(&mut builder, &mut state, 64);
    // Build the builder.
    let mut usb_serial = builder.build();

    // Run the USB device.
    let usb_fut = usb_serial.run();

    // Do stuff with the class!
    let echo_fut = async {
        loop {
            class.wait_connection().await;
            info!("Connected");
            let _ = echo(&mut class).await;
            info!("Disconnected");
        }
    };

    // Run everything concurrently.
    // If we had made everything `'static` above instead, we could do this using separate tasks instead.
    embassy_futures::join::join(usb_fut, echo_fut).await;
}


async fn echo<'d, T: embassy_stm32::usb::Instance + 'd>(
    class: &mut embassy_usb::class::cdc_acm::CdcAcmClass<'d, embassy_stm32::usb::Driver<'d, T>>,
) -> Result<(), Disconnected> {
    let mut buf = [0; 64];
    loop {
        let n = class.read_packet(&mut buf).await?;
        let data = &buf[..n];
        info!("data: {:x}", data);
        class.write_packet(data).await?;
    }
}

struct Disconnected {}
use embassy_usb::driver::EndpointError;
impl From<embassy_usb::driver::EndpointError> for Disconnected {
    fn from(val: EndpointError) -> Self {
        match val {
            // EndpointError::BufferOverflow => panic!("Buffer overflow"),
            EndpointError::BufferOverflow => Disconnected {},
            EndpointError::Disabled => Disconnected {},
        }
    }
}
