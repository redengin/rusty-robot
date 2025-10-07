use gz::{self as gazebosim};

use rusty_robot_drivers::imu_traits::{self, ImuData, ImuReader};
use rusty_robot_drivers::{gps_traits, nmea};

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use rusty_robot_robots::systems::QuadCopterMotors;

pub struct GazeboDrone {
    pub imu_topic: String,
    imu_signal: Signal<CriticalSectionRawMutex, ImuData>,

    pub gps_topic: String,
    gps_signal: Signal<CriticalSectionRawMutex, nmea::Nmea>,

    pub motors_topic: String,
    motors_signal: Signal<CriticalSectionRawMutex, [u8; 4]>,
}

/// max RPM of motors
// note the skybot caps motor velocity at 1000 radians/s (aka 9550 RPM)
const MAX_MOTOR_RPM: f64 = 10_000.0;

impl GazeboDrone {
    pub fn new(robot_name: &String) -> Self {
        GazeboDrone {
            imu_topic: format!(
                "/world/openworld/model/{robot_name}/link/base_link/sensor/imu_sensor/imu"
            ),
            imu_signal: Signal::new(),

            gps_topic: format!(
                "/world/openworld/model/{robot_name}/link/base_link/sensor/navsat_sensor/navsat",
            ),
            gps_signal: Signal::new(),

            motors_topic: format!("/{robot_name}/command/motor_speed"),
            motors_signal: Signal::new(),
        }
    }

    pub async fn run(&'static self) {
        let mut node = gazebosim::transport::Node::new().unwrap();

        // handle IMU updates
        log::debug!("subscribing to IMU '{}'", self.imu_topic);
        assert!(
            node.subscribe(self.imu_topic.as_str(), |msg: gz::msgs::imu::IMU| {
                log::trace!("imu msg {:?}", msg.entity_name);

                // convert the message into an IMU state
                let imu_data = ImuData {
                    accelerometer: Some(imu_traits::Vector3 {
                        x: msg.linear_acceleration.x as f32,
                        y: msg.linear_acceleration.y as f32,
                        z: msg.linear_acceleration.z as f32,
                    }),
                    gyroscope: Some(imu_traits::Vector3{
                        x: msg.angular_velocity.x as f32,
                        y: msg.angular_velocity.y as f32,
                        z: msg.angular_velocity.z as f32,
                    }),
                    ..Default::default()
                };

                // publish the update
                self.imu_signal.signal(imu_data);
            })
        );

        // process navsat data via inline callback
        log::debug!("subscribing to GPS '{}'", self.gps_topic);
        assert!(
            node.subscribe(self.gps_topic.as_str(), |msg: gz::msgs::navsat::NavSat| {
                log::trace!("gps msg {}", msg.frame_id);

                // convert the message into a GPS update
                let mut nmea = nmea::Nmea::default();
                nmea.latitude = Some(msg.latitude_deg);
                nmea.longitude = Some(msg.longitude_deg);
                nmea.altitude = Some(msg.altitude as f32);

                // publish the update
                self.gps_signal.signal(nmea);
            })
        );

        let mut motors_publisher = node
            .advertise::<gazebosim::msgs::actuators::Actuators>(&self.motors_topic.as_str())
            .unwrap();

        // handle publishing signals back to gazebosim
        loop {
            // awaits motor update
            let velocities_pct = self.motors_signal.wait().await;
            let mut msg = gazebosim::msgs::actuators::Actuators::new();
            msg.velocity = vec![
                Self::rpm_to_radians_per_second(MAX_MOTOR_RPM / 100.0 * (velocities_pct[0] as f64)),
                Self::rpm_to_radians_per_second(MAX_MOTOR_RPM / 100.0 * (velocities_pct[1] as f64)),
                Self::rpm_to_radians_per_second(MAX_MOTOR_RPM / 100.0 * (velocities_pct[2] as f64)),
                Self::rpm_to_radians_per_second(MAX_MOTOR_RPM / 100.0 * (velocities_pct[3] as f64)),
            ];
            if motors_publisher.publish(&msg) {
                log::trace!("sent motor update {:?}", msg.velocity);
            } else {
                log::warn!("failed to send motor update");
            }
        }
    }

    fn rpm_to_radians_per_second(rpm: f64) -> f64 {
        use std::f64::consts::PI;
        rpm * (2.0 * PI / 60.0)
    }
}

impl ImuReader for GazeboDrone {
    fn get_data(&self) -> Result<ImuData, &str> {
        match self.imu_signal.try_take() {
            Some(data) => return Ok(data),
            None => return Err("no data"),
        }
    }

    fn stop(&self) -> Result<(), &str> {
        // not implementable for sim
        Ok(())
    }
}

impl gps_traits::Gps for GazeboDrone {
    fn get_data(&self) -> Result<nmea::Nmea, &str> {
        match self.gps_signal.try_take() {
            Some(data) => return Ok(data),
            None => return Err("no new gps data"),
        }
    }
}

impl QuadCopterMotors for GazeboDrone {
    fn set_data(&self, velocities_pct: [u8; 4]) {
        self.motors_signal.signal(velocities_pct);
    }
}
