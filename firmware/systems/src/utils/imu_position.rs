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
    pub fn update(&mut self, imu_data: rusty_robot_drivers::imu::ImuData, elapsed: Duration) -> RelativePosition
    {
        let elapsed_s = elapsed.as_secs_f32();

        // use the last velocity vector to update position
        self.current.x = self.last_velocity.x * elapsed_s;
        self.current.y = self.last_velocity.y * elapsed_s;
        self.current.z = self.last_velocity.z * elapsed_s;


        // update velocity from current acceleration data
        if let Some(acceleration) = imu_data.accelerometer {
            log::trace!("XXXXXXXXXXXXX   {:?}", acceleration);
            self.last_velocity.x = self.last_velocity.x + (acceleration.x * elapsed_s);
            self.last_velocity.y = self.last_velocity.y + (acceleration.y * elapsed_s);
            self.last_velocity.z = self.last_velocity.z + (acceleration.z * elapsed_s);
        }
        else {
            log::warn!("no accelerometer data");
        }

        // return the current estimate
        log::trace!("{:?}", self);
        self.current.clone()
    }

    pub fn position(self) -> RelativePosition
    {
        self.current.clone()
    }

}