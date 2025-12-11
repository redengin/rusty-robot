use core::time::Duration;

/// Estimated position tracked via IMU accelerometer
#[derive(Default, Debug)]
pub struct ImuPosition {
    current: RelativePosition,
    last_velocity: Velocity,
}

#[derive(Default, Debug, Clone)]
pub struct RelativePosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Default, Debug, Clone)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl ImuPosition {

    pub fn new(current: RelativePosition, velocity: Velocity) -> Self
    {
        Self {
            current: current,
            last_velocity: velocity,
        }
    }

    /// calculate new position relative to last update
    /// NOTE: calling more frequently increases accuracy
    pub fn update(mut self, imu_data: rusty_robot_drivers::imu::ImuData, elapsed: Duration) -> RelativePosition
    {
        // use the last velocity vector to update position
        self.current.x = self.last_velocity.x * elapsed.as_secs_f32();
        self.current.y = self.last_velocity.y * elapsed.as_secs_f32();
        self.current.z = self.last_velocity.z * elapsed.as_secs_f32();


        // update velocity from current acceleration data
        if let Some(acceleration) = imu_data.accelerometer {
            self.last_velocity.x = self.last_velocity.x + (acceleration.x * elapsed.as_secs_f32());
            self.last_velocity.y = self.last_velocity.y + (acceleration.y * elapsed.as_secs_f32());
            self.last_velocity.z = self.last_velocity.z + (acceleration.z * elapsed.as_secs_f32());
        }

        // return the current estimate
        self.current.clone()
    }

    pub fn position(self) -> RelativePosition
    {
        self.current.clone()
    }

}