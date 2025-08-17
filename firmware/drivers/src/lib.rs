#![no_std]

// re-export shared dependencies
pub use imu_traits;
pub use nmea_parser;

// provide basic GPS API
pub mod gps {
    #[derive(Default, Clone, Debug)]
    pub struct GpsState {
        pub latitude: Option<f64>,
        pub longitude: Option<f64>,
        pub altitude: Option<f64>,
        pub satellite_count: Option<u8>,
        pub timestamp: Option<nmea_parser::chrono::DateTime<nmea_parser::chrono::Utc>>,
    }

    pub trait Gps {
        // provide the latest GPS State
        fn get_data(&self) -> GpsState;
    }
}

pub mod systems {
    pub trait QuadCopterMotors {
        /// set the velocity percent (0-255%) for all motors
        fn set_data(&mut self, velocities_pct: [u8;4]);
    }
}
