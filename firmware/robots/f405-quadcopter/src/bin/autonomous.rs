#![no_std]
#![no_main]

// upon panic, reset the chip
use panic_reset as _;

use log::*;

// FIXME move to common
// support a dynamically constructed static object
// When you are okay with using a nightly compiler it's better to use https://docs.rs/static_cell/2.1.0/static_cell/macro.make_static.html
macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}


// bind used interrupts to embassy runtime
embassy_stm32::bind_interrupts!(pub struct Irqs {
    OTG_FS => embassy_stm32::usb::InterruptHandler<embassy_stm32::peripherals::USB_OTG_FS>;
});

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    let peripherals = rusty_robot_f405_quadcopter::init();

    // create the USB driver
    // On it's own stm32f only supports full-speed (new_fs) - requiring additional PHY hardware
    // for high-speed which supports higher rates - but we don't need high rates.
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

}

// #[embassy_executor::task]
// // async fn flight_controller_task(drone: &'static GazeboDrone) {
// async fn flight_controller_task() {
//     // TODO
// }
