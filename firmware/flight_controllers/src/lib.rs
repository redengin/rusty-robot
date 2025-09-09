#![no_std]

pub trait FlightController {
    /// evolve the current state toward the desired state
    fn step(&self);
}

/// autonomous drone controlled by waypoints
pub mod autonomous {

    // pub struct Autonomous<T> {
    //     drone: T,
    // }

    // impl<T> crate::FlightController for Autonomous<T> {
    //     fn step(&self) {
            
    //     }
    // }
    // use rusty_robot_drivers::{
    //     gps_traits::{self, Gps},
    //     imu_traits::{self, ImuReader},
    // };
    // use rusty_robot_robots::systems;

    // use crate::FlightController;

    // pub fn step<T>(drone: &T)
    // where
    //     T: imu_traits::ImuReader + gps_traits::Gps + systems::QuadCopterMotors,
    // {
    //     // TODO step an actual flight controller
    //     let _ = <T as ImuReader>::get_data(drone);
    //     let _ = <T as Gps>::get_data(drone);
    //     let velocities_pct: [u8; 4] = [51, 51, 51, 51];
    //     <T as systems::QuadCopterMotors>::set_data(drone, velocities_pct);
    // }
}

// demonstrate use of neural network on hardware
pub mod learn_xor;
