//! Autonomous Drone (maneuvers to points in space)
use embassy_executor::{main, task};
use embassy_time::{Duration, Ticker};
use log::*;
use std::env;

use rusty_robot_common::mk_static;
use rusty_robot_gazebo_quadcopter::GazeboDrone;
use rusty_robot_systems::flight_controller::quadcopter::FlightController;

use embassy_executor::Spawner;
#[main]
async fn main(spawner: Spawner) {
    // support logging
    env_logger::builder().format_timestamp_millis().init();
    info!("hello world");

    // collect command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!(
            "drone name must be the first argument ({} <drone_name>)",
            args[0]
        )
    };
    let robot_name = &args[1];
    info!("{} under autonomous control", robot_name);

    // spawn the drone thread
    let drone = &mut *mk_static!(GazeboDrone, GazeboDrone::new(robot_name));
    spawner.spawn(drone_task(drone)).unwrap();

    // run the flight controller on the main
    let mut fc = FlightController::new(drone);
    fc.run().await;

}


#[task]
async fn drone_task(drone: &'static GazeboDrone) {
    // operate the drone (simulated) hardware
    drone.run().await;
}

