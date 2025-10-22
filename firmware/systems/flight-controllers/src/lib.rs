#![no_std]

// use rusty_robot::{Quaternion, Vector3};

// pub struct Trajectory {
//     pub attitude: Quaternion,
//     /// velocity in meters per second
//     pub rate: f32,
// }

pub struct FlightController<'a, Drone>
where
    Drone: rusty_robot_drivers::imu_traits::ImuReader
        + rusty_robot_drivers::gps_traits::Gps
        + rusty_robot_systems::QuadCopterMotors,
{
    drone: &'a Drone,
    mode: Mode,
}

impl<Drone> FlightController<'static, Drone>
where
    Drone: rusty_robot_drivers::imu_traits::ImuReader
        + rusty_robot_drivers::gps_traits::Gps
        + rusty_robot_systems::QuadCopterMotors,
{
    pub fn new(drone: &'static Drone) -> Self {
        FlightController {
            drone,
            mode: Mode::Ascent,
        }
    }

    pub fn step(&mut self) {
        // let imu_data = <Drone as rusty_robot_drivers::imu_traits::ImuReader>::get_data(&self.drone);
        let gps_data = <Drone as rusty_robot_drivers::gps_traits::Gps>::get_data(&self.drone)
            .unwrap();

        match self.mode {
            Mode::Ascent => {
                const ASCENT_ALTITUDE: f32 = 100.0;
                if gps_data.altitude.unwrap() > ASCENT_ALTITUDE 
                {
                    self.mode = Mode::Hold;
                }
                else {
                    let velocities_pct: [u8; 4] = [51, 51, 51, 51];
                    <Drone as rusty_robot_systems::QuadCopterMotors>::set_data(self.drone, velocities_pct);
                }
            }
            Mode::Hold => {

            }
        }
    }
}

enum Mode {
    Ascent,
    Hold,
}