//! Support for IMU hardware

/// Standard IMU Data
#[derive(Default, Debug)]
pub struct ImuData {
    /// Acceleration including gravity (m/s²)
    pub accelerometer: Option<Vector3>,
    /// Angular velocity (deg/s)
    pub gyroscope: Option<Vector3>,
    /// Magnetic field vector (micro Tesla, µT)
    pub magnetometer: Option<Vector3>,
}

// -- primitives for the IMU --
#[derive(Default, Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}


/// IMU hardware trait
pub trait ImuReader {
    /// Retrieves the latest available IMU data.
    ///    if necessary, restore the IMU to readable state
    fn get_data(&self) -> Result<ImuData, &str>;

    /// if possible puts the IMU into low-power mode
    fn stop(&self) -> Result<(), &str>;
}


// drivers
pub mod icm42688;