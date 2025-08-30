#![no_std]
#![no_main]

// upon panic, reset the chip
use panic_reset as _;

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    let peripherals = rusty_robot_f405_quadcopter::init();

    // TODO grok the betaflight configs to choose pins
    // example: https://raw.githubusercontent.com/betaflight/unified-targets/master/configs/default/DAKE-DAKEFPVF405.config
    let led1_pin = peripherals.PC14;

    // blink the light
    use embassy_stm32::gpio::{Level, Output, Speed};
    let led1 = Output::new(led1_pin, Level::Low, Speed::Low);
    spawner.spawn(blinky_task(led1)).unwrap();
}

#[embassy_executor::task]
async fn blinky_task(mut led1: embassy_stm32::gpio::Output<'static>) {
    use embassy_time::Duration;
    let mut ticker = embassy_time::Ticker::every(Duration::from_hz(2));
    loop {
        ticker.next().await;
        led1.set_high();
        ticker.next().await;
        led1.set_low();
    }
}
