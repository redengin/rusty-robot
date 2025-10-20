//! Autonomous Drone (maneuvers to points in space)
use embassy_time::{Duration, Ticker};
use log::*;
use rusty_robot_flight_controllers::FlightController;
use std::env;

use embassy_executor::Spawner;
use rusty_robot_gazebo_quadcopter::GazeboDrone;

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
    // support logging
    env_logger::builder().format_timestamp_millis().init();

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

    // create the drone as a static instance
    let drone = &mut *mk_static!(GazeboDrone, GazeboDrone::new(robot_name));

    // spawn the drone thread
    spawner.spawn(drone_task(drone)).unwrap();

    // create the flight controller as a static instance
    let fc = &mut *mk_static!(FlightController<GazeboDrone>, FlightController::new(drone));
    // run the flight controller in main context
    const CYCLE_RATE_HZ: u64 = 8000;
    let mut ticker = Ticker::every(Duration::from_hz(CYCLE_RATE_HZ));
    loop {
        // TODO autonomous::step(drone);
        // let _ = <T as ImuReader>::get_data(drone);
        // let _ = <T as Gps>::get_data(drone);
        // <T as systems::QuadCopterMotors>::set_data(drone, velocities_pct);
        // let velocities_pct: [u8; 4] = [51, 51, 51, 51];
        // <GazeboDrone as rusty_robot_systems::QuadCopterMotors>::set_data(drone, velocities_pct);

        fc.step();

        ticker.next().await
    }
}

#[embassy_executor::task]
async fn drone_task(drone: &'static GazeboDrone) {
    drone.run().await;
}
