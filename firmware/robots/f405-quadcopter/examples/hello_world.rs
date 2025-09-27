//! Demonstration of F405 hardware
#![no_std]
#![no_main]

use embassy_stm32::time::Hertz;
// upon panic, reset the chip
use panic_reset as _;

use log::*;

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
    spawner.spawn(rusty_robot_f405_quadcopter::usb::logger_task(usb_driver)).unwrap();
    info!("Initializing...");

    // initialize the IMU
    // pin mapping per https://raw.githubusercontent.com/betaflight/unified-targets/master/configs/default/DAKE-DAKEFPVF405.config
    // TODO create a pre-processor to digest the betaflight maps into rust code
    let mut imu_spi_config = embassy_stm32::spi::Config::default();
    imu_spi_config.frequency = Hertz(300_000);
    let mut imu_spi = embassy_stm32::spi::Spi::new(
        peripherals.SPI1,
        peripherals.PA5,
        peripherals.PA7,
        peripherals.PA6,
        peripherals.DMA2_CH3,
        peripherals.DMA2_CH0,
        imu_spi_config,
    );
    use embassy_stm32::gpio;
    let mut cs = gpio::Output::new(peripherals.PA4, gpio::Level::High, gpio::Speed::VeryHigh);
    // let cs = gpio::Output::new(peripherals.PA4, gpio::Level::Low, gpio::Speed::VeryHigh);
    let _ = rusty_robot_drivers::imu::icm42688::read_register(
        &mut imu_spi,
        rusty_robot_drivers::imu::icm42688::REG_WHO_AM_I,
    );

    // demonstrate logging
    embassy_time::Timer::after_millis(1000).await;
    let mut r = 0x1d;
    loop {
        cs.set_low();
        match rusty_robot_drivers::imu::icm42688::read_register(
            &mut imu_spi,
            r,
        )
        .await
        {
            Ok(v) => debug!("read 0x{r:x} = 0x{v:x}"),
            Err(_) => error!("failed to read register"),
        };
        // r = if r < 0x76 {r+1} else {0x11};
        cs.set_high();

        embassy_time::Timer::after_millis(1000).await;
    }
}
