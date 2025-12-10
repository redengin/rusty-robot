// re-export shared dependencies
pub use nmea; // GNNS sentence parsing support

// provide basic GNNS API
pub trait GNNS {
    // provide the latest GPS State
    fn get_data(&self) -> Result<nmea::Nmea, &str>;
}
