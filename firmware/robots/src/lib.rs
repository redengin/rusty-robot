#![no_std]

pub mod systems {
    pub trait QuadCopterMotors {
        /// set the velocity percent (0-255%) for all motors
        fn set_data(&self, velocities_pct: [u8;4]);
    }
}