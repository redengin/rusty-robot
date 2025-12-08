use core::default;

use crate::{Instant, TimeBound};

/// Quad Copter Motors
pub trait Motors {
    /// set the velocity percent (0-255%) for all motors
    fn set_data(&self, velocities_pct: [u8; 4]);
}

pub struct FlightController<'a, Robot>
where
    // all flight controllers need an imu
    Robot: rusty_robot_drivers::imu_traits::ImuReader,
{
    timebound: TimeBound,
    drone: &'a Robot,
}

impl<Robot> FlightController<'static, Robot>
where
    Robot:
        rusty_robot_drivers::imu_traits::ImuReader + rusty_robot_drivers::gps_traits::Gps + Motors,
{
    pub fn new(drone: &'static Robot) -> Self {
        FlightController {
            timebound: TimeBound::new(),
            drone: drone,
        }
    }

    pub fn step(&mut self) {
        // determine the elapsed time
        let _elapsed = self.timebound.step();

        let _imu_data =
            <Robot as rusty_robot_drivers::imu_traits::ImuReader>::get_data(&self.drone);
        // TODO update estimated position via kalman filter

        // FIXME only use gps data if robot provides it
        let _gps_data =
            <Robot as rusty_robot_drivers::gps_traits::Gps>::get_data(&self.drone).unwrap();

        let velocities_pct: [u8; 4] = [51, 51, 51, 51];
        <Robot as Motors>::set_data(self.drone, velocities_pct);

    }
}
