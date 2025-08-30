#![no_std]
#![no_main]

use embassy_stm32::peripherals;
// upon panic, reset the chip
use panic_reset as _;

use log::*;

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    let config = rusty_robot_f405_quadcopter::clock_config();
    let peripherals = embassy_stm32::init(config);

    // create the USB driver
    let usb_driver = rusty_robot_f405_quadcopter::usb::driver(
        peripherals.USB_OTG_FS,
        peripherals.PA12,
        peripherals.PA11,
    );

    // start the USB UART interface
    let (usb_serial, usb_class) = rusty_robot_f405_quadcopter::usb::usb_serial::device(usb_driver);
    spawner.spawn(task_usb_serial(usb_serial)).unwrap();

    // start the logger
    spawner.spawn(task_logger(usb_class)).unwrap();

    // demonstrations
    spawner.spawn(task_demo()).unwrap();
}

#[embassy_executor::task]
async fn task_demo() {
    loop {
        info!("hello world!");
        embassy_time::Timer::after_millis(500).await;
    }
}

#[embassy_executor::task]
async fn task_usb_serial(
    mut usb_serial: embassy_usb::UsbDevice<
        'static,
        embassy_stm32::usb::Driver<'static, embassy_stm32::peripherals::USB_OTG_FS>,
    >,
) {
    usb_serial.run().await
}

// #[embassy_executor::task]
// async fn task_logger(driver: embassy_stm32::usb::Driver<'static, peripherals::USB_OTG_FS>) {
//     embassy_usb_logger::run!(1024, log::LevelFilter::Debug, driver);
// }
#[embassy_executor::task]
async fn task_logger(
    mut class: embassy_usb::class::cdc_acm::CdcAcmClass<
        'static,
        embassy_stm32::usb::Driver<'static, peripherals::USB_OTG_FS>,
    >,
) {
    // use embassy_usb_logger::DummyHandler;

    // static LOGGER: embassy_usb_logger::UsbLogger<1024, DummyHandler> =
    //     embassy_usb_logger::UsbLogger::new();
    // let logger_run = LOGGER.create_future_from_class(class);
    // logger_run.await
    loop {
        class.wait_connection().await;
        info!("Connected");
        // let _ = echo(&mut class).await;
        _ = class.write_packet("hello world!".as_bytes()).await;
        info!("Disconnected");
    }
}
