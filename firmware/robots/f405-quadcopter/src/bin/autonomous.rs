#![no_std]
#![no_main]

// upon panic, reset the chip
use panic_reset as _;

use log::*;

// bind used interrupts to embassy runtime
embassy_stm32::bind_interrupts!(struct Irqs {
    OTG_FS => embassy_stm32::usb::InterruptHandler<embassy_stm32::peripherals::USB_OTG_FS>;
});

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    let config = rusty_robot_f405_quadcopter::clock_config();
    let peripherals = embassy_stm32::init(config);

    // create the USB driver
    let mut usb_driver=
        rusty_robot_f405_quadcopter::usb_serial::driver(
            peripherals.USB_OTG_FS,
            peripherals.PA12,
            peripherals.PA11,
        );


    // spawner.spawn(task_usb_serial(usb_serial));



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
