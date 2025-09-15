//! Demonstration of F405 hardware
#![no_std]
#![no_main]

// upon panic, reset the chip
use panic_reset as _;

use log::*;

use embassy_stm32::{spi, time::Hertz};

// bind used interrupts to embassy runtime
embassy_stm32::bind_interrupts!(pub struct Irqs {
    OTG_FS => embassy_stm32::usb::InterruptHandler<embassy_stm32::peripherals::USB_OTG_FS>;
});

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    let peripherals = rusty_robot_f405_quadcopter::init();

    // create the USB driver
    let usb_driver = embassy_stm32::usb::Driver::new_fs(
        peripherals.USB_OTG_FS,
        Irqs,
        peripherals.PA12,
        peripherals.PA11,
        rusty_robot_f405_quadcopter::usb::EP_OUT_BUFFER.init([0; _]),
        embassy_stm32::usb::Config::default(),
    );
    // start the logger
    use rusty_robot_f405_quadcopter::usb_logger_task;
    spawner.spawn(usb_logger_task(usb_driver)).unwrap();
    info!("Initializing...");

    // initialize the IMU
    let mut imu_spi_config = spi::Config::default();
    imu_spi_config.frequency = Hertz(300_000);
    imu_spi_config.mode = spi::MODE_3;
    let spi = embassy_stm32::spi::Spi::new(
        peripherals.SPI1,
        peripherals.PA5,
        peripherals.PA7,
        peripherals.PA6,
        peripherals.DMA2_CH3,
        peripherals.DMA2_CH0,
        imu_spi_config,
    );
    let imu = rusty_robot_drivers::imu::icm42688::ICM42688::new();



    // demonstrate logging
    loop {
        info!("Hello World!");
        debug!("Goodbye World!");
        embassy_time::Timer::after_millis(500).await;
    }
}
