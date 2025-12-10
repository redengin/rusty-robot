#![no_std]

// re-export shared dependencies
pub use nmea;   // GPS sentence parsing support

// provide basic GPS API
pub mod gps_traits {
    pub trait Gps {
        // provide the latest GPS State
        fn get_data(&self) -> Result<nmea::Nmea, &str>;
    }
}

// provide IMU drivers and traits
pub mod imu;


pub mod radio;