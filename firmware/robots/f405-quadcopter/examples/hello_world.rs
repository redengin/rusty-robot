//! Demonstration of F405 hardware
#![no_std]
#![no_main]


// upon panic, reset the chip
use panic_reset as _;
// #[panic_handler]
// fn panic(info: &core::panic::PanicInfo) -> ! {
//     error!("PANIC: {}", info);
//     cortex_m::asm::delay(1_000_000);
//     cortex_m::peripheral::SCB::sys_reset();
// }

use log::*;

// bind used interrupts to embassy runtime
embassy_stm32::bind_interrupts!(pub struct Irqs {
    OTG_FS => embassy_stm32::usb::InterruptHandler<embassy_stm32::peripherals::USB_OTG_FS>;
    USART1 => embassy_stm32::usart::InterruptHandler<embassy_stm32::peripherals::USART1>;
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

    // initialize spi IMU
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

    // initialize serial GPS
    let serial1_config = embassy_stm32::usart::Config::default();
    // serial1_config.baudrate = 38_400;
    // serial1_config.baudrate = 115_200;
    // serial1_config.data_bits = embassy_stm32::usart::DataBits::DataBits8,
    // serial1_config.stop_bits = embassy_stm32::usart::StopBits::STOP1,
    // serial1_config.parity = embassy_stm32::usart::Parity::ParityNone,
    let mut serial1 = embassy_stm32::usart::Uart::new(
        peripherals.USART1,
        peripherals.PA10,
        peripherals.PA9,
        Irqs,
        peripherals.DMA2_CH7,
        peripherals.DMA2_CH2,
        serial1_config
    ).unwrap();

    // demonstrate hardware (IMU and GPS)
    embassy_time::Timer::after_millis(1000).await;
    imu.set_power_mode(rusty_robot_drivers::imu::icm42688::PowerMode::Enabled).await.unwrap();
    loop {
        debug!("reading imu....");
        match imu.read_imu().await {
            Ok(v) => info!("accel: {:?}, gyro: {:?}", v.accelerometer.unwrap(), v.gyroscope.unwrap()),
            Err(e)  => error!("imu spi error [{:?}]", e),
        }

        // read GPS data
        let mut gps_buf: [u8; _] = [0; 255];
        match serial1.read_until_idle(&mut gps_buf).await
        {
            Ok(sz) => info!("gps read {sz} bytes <{:?}>", gps_buf),
            Err(e) => error!("gps read error [{:?}]", e)
        }

        embassy_time::Timer::after_millis(1000).await;
    }
}
