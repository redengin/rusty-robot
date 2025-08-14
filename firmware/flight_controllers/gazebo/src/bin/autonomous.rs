//! Autonomous Drone (maneuvers to points in space)
use std::env;

use log::*;
use embassy_executor::Spawner;
use embassy_time::Timer;

use rusty_robot_gazebo::{GazeboDrone};

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

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_timestamp_millis()
        .init();

    // collect command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { panic!("drone name must be the first argument ({} <drone_name>)", args[0])};
    let robot_name = &args[1];
    info!("{} under autonomous control", robot_name);

    // create the drone as a static instance
    let drone= &mut *mk_static!(
        GazeboDrone,
        GazeboDrone::new(robot_name)
    );

    // TODO spawn the control threads

    // operate the drone
    drone.run().await
}

// use rusty_robot_drivers::imu_traits;

// #[embassy_executor::task]
// async fn run() {
//     loop {
//         info!("tick");


//         Timer::after_secs(1).await;
//     }
// }
