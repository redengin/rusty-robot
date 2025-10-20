use rusty_robot::{Quaternion, Vector3};
use rusty_robot_drivers::imu_traits::ImuReader;

pub struct Trajectory {
    pub attitude: Quaternion,
    /// velocity in meters per second
    pub rate: f32,
}

pub struct FlightController<'a, Drone>
where
    Drone: rusty_robot_drivers::imu_traits::ImuReader
    + rusty_robot_drivers::gps_traits::Gps
    + rusty_robot_systems::QuadCopterMotors,
{
    drone: &'a Drone,
}

impl<Drone> FlightController<'static, Drone>
where
    Drone: rusty_robot_drivers::imu_traits::ImuReader
    + rusty_robot_drivers::gps_traits::Gps
    + rusty_robot_systems::QuadCopterMotors,
{
    pub fn new(drone: &'static Drone) -> Self {
        FlightController { drone }
    }

    pub fn step(&self) {
        // let imu_data = <Drone as rusty_robot_drivers::imu_traits::ImuReader>::get_data(&self.drone);
        // let gps_data = <Drone as rusty_robot_drivers::gps_traits::Gps>::get_data(&self.drone);
        let velocities_pct: [u8; 4] = [51, 51, 51, 51];
        <Drone as rusty_robot_systems::QuadCopterMotors>::set_data(self.drone, velocities_pct);
    }
}
