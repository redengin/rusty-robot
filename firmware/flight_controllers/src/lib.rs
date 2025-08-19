#![no_std]

// use rusty_robot_drivers::imu_traits;
// use rusty_robot_drivers::gps_traits;

/// autonomous drone controlled by waypoints
pub mod autonomous {
    use rusty_robot_drivers::{
        gps_traits::{self, Gps},
        imu_traits::{self, ImuReader},
        systems,
    };

    pub async fn run<T>(drone: &T, cycle_rate_hz: u64)
    where
        T: imu_traits::ImuReader + gps_traits::Gps + systems::QuadCopterMotors,
    {
        let cycle_duration = embassy_time::Duration::from_hz(cycle_rate_hz);
        let mut cycle_count: u64 = 0;
        loop {
            log::trace!(target:"autonomous_flight_controller", "starting cycle {}", cycle_count);
            cycle_count += 1;

            // figure out the next cycle instant
            let start_instant = embassy_time::Instant::now();
            let next_start_instant = start_instant.saturating_add(cycle_duration);

            // TODO step flight controller
            let _ = <T as ImuReader>::get_data(drone);
            let _ = <T as Gps>::get_data(drone);
            let velocities_pct: [u8; 4] = [51, 51, 51, 51];
            <T as systems::QuadCopterMotors>::set_data(drone, velocities_pct);

            // await the next cycle
            if embassy_time::Instant::now() > next_start_instant {
                log::warn!(target:"autonomous_flight_controller",
                    "cycle {} exceeded duration {} Hz",
                    cycle_count, cycle_rate_hz,
                );
            } else {
                embassy_time::Timer::at(next_start_instant).await
            }
        }
    }
}
