#![no_std]

pub use static_cell;
pub use arrayvec;

// support a dynamically constructed static object
// When you are okay with using a nightly compiler it's better to use https://docs.rs/static_cell/2.1.0/static_cell/macro.make_static.html
#[macro_export]
macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: rusty_robot::static_cell::StaticCell<$t> = rusty_robot::static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

// #[derive(Debug, Clone, Copy, Default)]
#[derive(Clone, Copy, Default)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl core::fmt::Debug for Vector3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("")
            .field("x", &format_args!("{:.3}", &self.x))
            .field("y", &format_args!("{:.3}", &self.y))
            .field("z", &format_args!("{:.3}", &self.z))
            .finish()
    }
}
impl Vector3 {
    // FIXME find a no_std trig library (libm doesn't leverage hardware)
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

#[derive(Clone, Copy, Default)]
pub struct Quaternion {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl core::fmt::Debug for Quaternion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("")
            .field("w", &format_args!("{:.3}", &self.w))
            .field("x", &format_args!("{:.3}", &self.x))
            .field("y", &format_args!("{:.3}", &self.y))
            .field("z", &format_args!("{:.3}", &self.z))
            .finish()
    }
}
impl Quaternion {
    // FIXME find a no_std trig library (libm doesn't leverage hardware)
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
