//! Support for IMU hardware

// --- Standard IMU Data ---
// #[derive(Debug, Clone, Copy, Default)]
#[derive(Debug, Clone, Default)]
pub struct ImuData {
    /// Acceleration including gravity (m/s²)
    pub accelerometer: Option<Vector3>,
    /// Angular velocity (deg/s)
    pub gyroscope: Option<Vector3>,
    /// Magnetic field vector (micro Tesla, µT)
    pub magnetometer: Option<Vector3>,
    /// Orientation as a unit quaternion (WXYZ order)
    pub quaternion: Option<Quaternion>,
    /// Orientation as Euler angles (deg)
    pub euler: Option<Vector3>,
    /// Linear acceleration (acceleration without gravity) (m/s²)
    pub linear_acceleration: Option<Vector3>,
    /// Estimated gravity vector (m/s²)
    pub gravity: Option<Vector3>,
    /// Temperature (°C)
    pub temperature: Option<f32>,
    /// Calibration status
    pub calibration_status: Option<u8>,
}

// -- Standard IMU fucntions ---
pub trait ImuReader {
    /// Retrieves the latest available IMU data.
    ///    if necessary, restore the IMU to readable state
    fn get_data(&self) -> Result<ImuData, &str>;

    /// if possible puts the IMU into low-power mode
    fn stop(&self) -> Result<(), &str>;
}

#[derive(Debug, Clone, Default)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Default)]
pub struct Quaternion {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// drivers
pub mod icm42688;