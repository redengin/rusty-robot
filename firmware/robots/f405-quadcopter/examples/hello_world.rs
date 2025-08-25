#![no_std]
#![no_main]

// upon panic, reset the chip
use panic_reset as _;

use embassy_executor::Spawner;
// bind used interrupts to embassy runtime
embassy_stm32::bind_interrupts!(struct Irqs {
    OTG_FS => embassy_stm32::usb::InterruptHandler<embassy_stm32::peripherals::USB_OTG_FS>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let config = rusty_robot_f405_quadcopter::clock_config();
    let peripherals = embassy_stm32::init(config);

    // create HAL USB driver (FS: full-speed)
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
    let usb_config = rusty_robot_f405_quadcopter::usb_serial::config();

    // build the USB serial interface
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut control_buf = [0; 64];
    let mut builder = embassy_usb::Builder::new(
        driver,
        usb_config,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut [], // no msos descriptors
        &mut control_buf,
    );
    // Create classes on the builder.
    let mut state = embassy_usb::class::cdc_acm::State::new();
    let mut class = embassy_usb::class::cdc_acm::CdcAcmClass::new(&mut builder, &mut state, 64);
    // Build the builder.
    let mut usb_serial = builder.build();

    // Run the USB device.
    let usb_fut = usb_serial.run();

    // Do stuff with the class!
    let echo_fut = async {
        loop {
            class.wait_connection().await;
            // info!("Connected");
            let _ = echo(&mut class).await;
            // info!("Disconnected");
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
        // info!("data: {:x}", data);
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
