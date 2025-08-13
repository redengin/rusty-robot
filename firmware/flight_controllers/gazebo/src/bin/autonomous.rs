//! Autonomous Drone (maneuvers to points in space)
use std::env;

use log::*;
use embassy_executor::Spawner;
use embassy_time::Timer;

use rusty_robot_gazebo::{GazeboDrone};
// use rusty_robot_drivers::imu_traits;

// #[embassy_executor::task]
// async fn run() {
//     loop {
//         info!("tick");


//         Timer::after_secs(1).await;
//     }
// }

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_timestamp_millis()
        .init();

    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();
    let robot_name = &args[2];
    let mut drone = GazeboDrone::new(robot_name);

    // TODO spawn the control threads

    // operate the drone
    drone.run();
}