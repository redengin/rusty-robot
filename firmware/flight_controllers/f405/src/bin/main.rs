#![no_std]
#![no_main]

use {panic_probe as _};
use defmt::*;

use embassy_executor::Spawner;
// use embassy_stm32::gpio::{Level, Output, Speed};
// use embassy_time::Timer;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // TODO
    info!("Booting....");
}
