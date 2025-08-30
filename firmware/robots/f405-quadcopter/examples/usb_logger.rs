#![no_std]
#![no_main]

// upon panic, reset the chip
use panic_reset as _;

use log::*;

use embassy_executor::Spawner;
// bind used interrupts to embassy runtime
embassy_stm32::bind_interrupts!(pub struct Irqs {
    OTG_FS => embassy_stm32::usb::InterruptHandler<embassy_stm32::peripherals::USB_OTG_FS>;
});

static EP_OUT_BUFFER: static_cell::StaticCell<[u8; 256]> = static_cell::StaticCell::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let config = rusty_robot_f405_quadcopter::clock_config();
    let peripherals = embassy_stm32::init(config);

    // USB driver (FS: full-speed)
    //--------------------------------------------------------------------------------
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
        EP_OUT_BUFFER.init([0;_]),
        usb_config,
    );

    spawner.spawn(usb_logger_task(usb_driver)).unwrap();

    spawner.spawn(hello_world_task()).unwrap();

    // // Run everything concurrently.
}

#[embassy_executor::task]
async fn usb_logger_task(driver: embassy_stm32::usb::Driver<'static, embassy_stm32::peripherals::USB_OTG_FS>) {
   embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::task]
async fn hello_world_task() {
    loop {
        info!("Hello World!");
        embassy_time::Timer::after_millis(500).await;
    }
}