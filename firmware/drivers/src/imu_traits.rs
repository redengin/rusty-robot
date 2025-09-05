// adapted from https://github.com/kscalelabs/imu/blob/master/imu/imu-traits/src/lib.rs
//MIT License
//
// Copyright (c) 2024 K-Scale Labs
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// --- Standard IMU Data ---
#[derive(Debug, Clone, Copy, Default)]
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


// --- Basic Types ---
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Quaternion {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    // pub fn euler_to_quaternion(&self) -> Quaternion {
    //     // Convert Euler angles (in radians) to quaternion
    //     // Using the ZYX rotation order (yaw, pitch, roll)
    //     let (roll, pitch, yaw) = (self.x, self.y, self.z);

    //     let cr = (roll * 0.5).cos();
    //     let sr = (roll * 0.5).sin();
    //     let cp = (pitch * 0.5).cos();
    //     let sp = (pitch * 0.5).sin();
    //     let cy = (yaw * 0.5).cos();
    //     let sy = (yaw * 0.5).sin();

    //     let w = cr * cp * cy + sr * sp * sy;
    //     let x = sr * cp * cy - cr * sp * sy;
    //     let y = cr * sp * cy + sr * cp * sy;
    //     let z = cr * cp * sy - sr * sp * cy;

    //     Quaternion { w, x, y, z }
    // }
}

// impl fmt::Display for Vector3 {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Vector3(x={}, y={}, z={})", self.x, self.y, self.z)
//     }
// }

impl Quaternion {
    // pub fn rotate_vector(&self, v: Vector3, inverse: bool) -> Vector3 {
    //     // Rotate a vector by a quaternion using the formula:
    //     // v' = q * v * q^-1
    //     // Where q^-1 is the conjugate since we normalize the quaternion
    //     const EPS: f32 = 1e-6;
    //     let n2 = self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z;
    //     let scale = 1.0 / (n2.sqrt() + EPS);

    //     let w = self.w * scale;
    //     let mut x = self.x * scale;
    //     let mut y = self.y * scale;
    //     let mut z = self.z * scale;

    //     // 2. Conjugate if inverse
    //     if inverse {
    //         x = -x;
    //         y = -y;
    //         z = -z;
    //     }

    //     // 3. Apply the expanded formula
    //     let vx = v.x;
    //     let vy = v.y;
    //     let vz = v.z;
    //     let xx = w * w * vx + 2.0 * y * w * vz - 2.0 * z * w * vy
    //         + x * x * vx
    //         + 2.0 * y * x * vy
    //         + 2.0 * z * x * vz
    //         - z * z * vx
    //         - y * y * vx;
    //     let yy = 2.0 * x * y * vx + y * y * vy + 2.0 * z * y * vz + 2.0 * w * z * vx - z * z * vy
    //         + w * w * vy
    //         - 2.0 * w * x * vz
    //         - x * x * vy;
    //     let zz = 2.0 * x * z * vx + 2.0 * y * z * vy + z * z * vz - 2.0 * w * y * vx
    //         + w * w * vz
    //         + 2.0 * w * x * vy
    //         - y * y * vz
    //         - x * x * vz;

    //     Vector3 {
    //         x: xx,
    //         y: yy,
    //         z: zz,
    //     }
    // }
}

// impl fmt::Display for Quaternion {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "Quaternion(w={}, x={}, y={}, z={})",
//             self.w, self.x, self.y, self.z
//         )
//     }
// }