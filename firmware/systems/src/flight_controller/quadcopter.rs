use embassy_time::Timer;

use crate::utils::TimeBound;

/// Quad Copter Motors
pub trait Motors {
    /// set the velocity percent (0-255%) for all motors
    fn set_data(&self, velocities_pct: [u8; 4]);
}

pub struct FlightController<'a, Robot>
where
    // all flight controllers need an imu
    Robot: rusty_robot_drivers::imu::ImuReader,
{
    drone: &'a Robot,

    imu_position: crate::utils::ImuPosition,
}

impl<Robot> FlightController<'static, Robot>
where
    Robot:
        // rusty_robot_drivers::imu_traits::ImuReader + rusty_robot_drivers::gps_traits::Gps + Motors,
        rusty_robot_drivers::imu::ImuReader + Motors,
{
    pub fn new(drone: &'static Robot) -> Self {
        FlightController {
            drone: drone,
            imu_position: Default::default(),
        }
    }

    pub async fn run(&mut self) {

        let mut timebound = TimeBound::new();
        loop {
            log::info!("run loop");
            // determine the elapsed time
            let elapsed = timebound.step();

            // use the IMU data
            if let Ok(imu_data) = <Robot as rusty_robot_drivers::imu::ImuReader>::get_data(&self.drone)
            {
                // update the imu estimated position
                self.imu_position.update(imu_data, elapsed);
            }

            let velocities_pct: [u8; 4] = [51, 51, 51, 51];
            <Robot as Motors>::set_data(self.drone, velocities_pct);

            Timer::after_millis(1).await
        }

        // TODO update estimated position via kalman filter

        // FIXME only use gps data if robot provides it
        // let _gps_data =
        //     <Robot as rusty_robot_drivers::gps_traits::Gps>::get_data(&self.drone).unwrap();

    }
}
