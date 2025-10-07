//! Demonstration of F405 hardware
#![no_std]
#![no_main]

// upon panic, reset the chip
// use panic_reset as _;
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    error!("PANIC: {}", info);
    cortex_m::asm::delay(1_000_000);
    cortex_m::peripheral::SCB::sys_reset();
}

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
    spawner
        .spawn(rusty_robot_f405_quadcopter::usb::logger_task(usb_driver))
        .unwrap();
    info!("Initializing...");

    // initialize the IMU Bus/Device
    // pin mapping per https://raw.githubusercontent.com/betaflight/unified-targets/master/configs/default/DAKE-DAKEFPVF405.config
    // TODO create a pre-processor to digest the betaflight maps into rust code
    let spi1_config = embassy_stm32::spi::Config::default();
    let spi1 = embassy_stm32::spi::Spi::new(
        peripherals.SPI1,
        peripherals.PA5,
        peripherals.PA7,
        peripherals.PA6,
        peripherals.DMA2_CH3,
        peripherals.DMA2_CH0,
        spi1_config,
    );
    use embassy_stm32::gpio;
    // NOTE: my chip requires that CS toggle - holding CS LOW results devolves into reads of 0xFF
    // let imu_cs = gpio::Output::new(peripherals.PA4, gpio::Level::High, gpio::Speed::VeryHigh);
    let imu_cs = gpio::Output::new(peripherals.PA4, gpio::Level::Low, gpio::Speed::VeryHigh);
    let mut imu_dev = embedded_hal_bus::spi::ExclusiveDevice::new_no_delay(spi1, imu_cs).unwrap();
    // create the IMU driver
    let mut imu = rusty_robot_drivers::imu::icm42688::ICM42688::new(&mut imu_dev).await.unwrap();

    // demonstrate hardware (IMU and GPS)
    embassy_time::Timer::after_millis(1000).await;
    // imu.set_power_mode(rusty_robot_drivers::imu::icm42688::PowerMode::LowPower).await.unwrap();
    imu.set_power_mode(rusty_robot_drivers::imu::icm42688::PowerMode::LowNoise).await.unwrap();
    loop {
        debug!("reading imu....");
        match imu.read_imu().await {
            Ok(v) => info!("accel: {:?}, gyro: [{:?}", v.accelerometer.unwrap(), v.gyroscope.unwrap()),
            Err(e)  => error!("imu spi error [{:?}", e),
        }
        //     Some(icm42688) => {
        //         // imu.read_imu().await;
        //         // let imu = imu.take(); 
        //         // match imu.read_imu().await {
        //         // Ok(data) => {}
        //         // Err(e) => error!("failed to read imu [{e}]"),
        //     },
        //     None => error!("imu not found"),
        // }
        // let r = 0x75;
        // match rusty_robot_drivers::imu::icm42688::read_register(
        //     &mut imu_dev,
        //     r,
        // )
        // .await
        // {
        //     Ok(v) => debug!("read 0x{r:x} = 0x{v:x}"),
        //     Err(e) => error!("failed to read register [{e}]"),
        // };

        // match rusty_robot_drivers::imu::icm42688::read_imu(
        //     &mut imu_dev,
        // )
        // .await
        // {
        //     Ok(_v) => {}, //debug!("temp: {} C", v.temperature_c),
        //     Err(e) => error!("failed to read register [{e}]"),
        // };

        embassy_time::Timer::after_millis(1000).await;
    }
}
