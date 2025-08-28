#![no_std]
#![no_main]

// upon panic, reset the chip
use panic_reset as _;

use log::*;

use embassy_executor::Spawner;
// bind used interrupts to embassy runtime
embassy_stm32::bind_interrupts!(struct Irqs {
    OTG_FS => embassy_stm32::usb::InterruptHandler<embassy_stm32::peripherals::USB_OTG_FS>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let config = rusty_robot_f405_quadcopter::clock_config();
    let peripherals = embassy_stm32::init(config);

    // USB driver (FS: full-speed)
    //--------------------------------------------------------------------------------
    let mut ep_out_buffer = [0u8; 256];
    let mut usb_config = embassy_stm32::usb::Config::default();
    // disable vbus_detection - this is a safe default that works in all boards.
    // However, if your USB device is self-powered (can stay powered on if USB is unplugged),
    // you can enable vbus_detection to comply with the USB spec - but the board
    // has to support it or USB won't work at all. See docs on `vbus_detection` for details.
    usb_config.vbus_detection = false;
    let usb_driver = embassy_stm32::usb::Driver::new_fs(
        peripherals.USB_OTG_FS,
        Irqs,
        peripherals.PA12,
        peripherals.PA11,
        &mut ep_out_buffer,
        usb_config,
    );
    // create serial device description
    let mut usb_descriptor = embassy_usb::Config::new(0xc0de, 0xcafe);
    usb_descriptor.manufacturer = Some("rusty-robot");
    usb_descriptor.product = Some("f405-usb-serial");
    // build the USB serial interface
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut control_buf = [0; 64];
    let mut state = embassy_usb::class::cdc_acm::State::new();
    let mut builder = embassy_usb::Builder::new(
        usb_driver,
        usb_descriptor,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut [], // no msos descriptors
        &mut control_buf,
    );
    // Create classes on the builder.
    let usb_serial_class =
        embassy_usb::class::cdc_acm::CdcAcmClass::new(&mut builder, &mut state, 64);
    // Build the builder.
    let mut usb_serial = builder.build();

    // Run the USB device.
    let usb_serial_fut = usb_serial.run();

    // USB Logger
    //--------------------------------------------------------------------------------
    // let usb_logger: embassy_usb_logger::UsbLogger<1024, embassy_usb_logger::DummyHandler> =
    //     embassy_usb_logger::UsbLogger::new();
    static usb_logger: embassy_usb_logger::UsbLogger<1024, embassy_usb_logger::DummyHandler> =
        embassy_usb_logger::UsbLogger::new();
    // let mut usb_logger_state = embassy_usb_logger::LoggerState::new();
    // let usb_logger_fut = usb_logger.run(&mut usb_logger_state, usb_driver);
    let usb_logger_fut = usb_logger.create_future_from_class(usb_serial_class);

    let log_fut = async {
        use embassy_stm32::gpio::{Level, Output, Speed};
        let mut led1 = Output::new(peripherals.PC14, Level::High, Speed::Low);

        loop {
            use embassy_time::Timer;

            info!("turning light off");
            led1.set_high();
            Timer::after_millis(500).await;
            info!("turning light on");
            led1.set_low();
            Timer::after_millis(500).await;
        }
    };

    // Run everything concurrently.
    // If we had made everything `'static` above instead, we could do this using separate tasks instead.
    // embassy_futures::join::join(usb_serial_fut, log_fut).await;
    embassy_futures::join::join3(usb_serial_fut, usb_logger_fut, log_fut).await;
}
