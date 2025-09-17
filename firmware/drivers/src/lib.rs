#![no_std]

// re-export shared dependencies
pub use nmea;   // GPS sentence parsing support

// provide IMU traits
pub mod imu_traits;

// provide basic GPS API
pub mod gps_traits {
    pub trait Gps {
        // provide the latest GPS State
        fn get_data(&self) -> Result<nmea::Nmea, &str>;
    }
}

// provide IMU drivers
pub mod imu;
    