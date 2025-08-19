use gz::{self as gazebosim};

use rusty_robot_drivers::gps_traits::{Gps, GpsState};
use rusty_robot_drivers::imu_traits::{self, ImuData, ImuError, ImuReader};

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use rusty_robot_drivers::systems::QuadCopterMotors;
use std::f64::consts::PI;

pub struct GazeboDrone {
    pub imu_topic: String,
    imu_signal: Signal<CriticalSectionRawMutex, ImuData>,

    pub gps_topic: String,
    gps_signal: Signal<CriticalSectionRawMutex, GpsState>,

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
                "/world/openworld/model/{}/link/base_link/sensor/imu_sensor/imu",
                robot_name
            ),
            imu_signal: Signal::new(),

            gps_topic: format!(
                "/world/openworld/model/{}/link/base_link/sensor/navsat_sensor/navsat",
                robot_name
            ),
            gps_signal: Signal::new(),

            motors_topic: format!("/{}/command/motor_speed", robot_name),
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

                let mut imu_data = ImuData {
                    ..Default::default()
                };

                // accelerometer
                if let Some(data) = msg.linear_acceleration.as_ref() {
                    imu_data.accelerometer = Some(imu_traits::Vector3 {
                        x: data.x as f32,
                        y: data.y as f32,
                        z: data.z as f32,
                    })
                }

                // gyroscope
                if let Some(data) = msg.angular_velocity.as_ref() {
                    imu_data.gyroscope = Some(imu_traits::Vector3 {
                        x: data.x as f32,
                        y: data.y as f32,
                        z: data.z as f32,
                    })
                }

                // orientation
                if let Some(data) = msg.orientation.as_ref() {
                    imu_data.quaternion = Some(imu_traits::Quaternion {
                        w: data.w as f32,
                        x: data.x as f32,
                        y: data.y as f32,
                        z: data.z as f32,
                    })
                }

                // publish the update
                self.imu_signal.signal(imu_data.clone());
            })
        );

        // process navsat data via inline callback
        log::debug!("subscribing to GPS '{}'", self.gps_topic);
        assert!(
            node.subscribe(self.gps_topic.as_str(), |msg: gz::msgs::navsat::NavSat| {
                log::trace!("gps msg {}", msg.frame_id);

                self.gps_signal.signal(GpsState {
                    latitude: Some(msg.latitude_deg),
                    longitude: Some(msg.longitude_deg),
                    altitude: Some(msg.altitude),
                    satellite_count: None,
                    // TODO parse the msg.header.stamp (timestamp) to set the timestamp
                    timestamp: None,
                });
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
        rpm * (2.0 * PI / 60.0)
    }
}

impl ImuReader for GazeboDrone {
    /// Returns the most recent sensor data.
    fn get_data(&self) -> Result<ImuData, ImuError> {
        match self.imu_signal.try_take() {
            Some(data) => return Ok(data),
            None => return Err(ImuError::ReadError("no data".to_string())),
        }
    }

    /// Stops the reading thread.
    fn stop(&self) -> Result<(), ImuError> {
        // not implementable for sim
        Ok(())
    }
}

impl Gps for GazeboDrone {
    fn get_data(&self) -> Result<GpsState, &str> {
        match self.gps_signal.try_take() {
            Some(data) => return Ok(data),
            None => return Err("no new gps data"),
        }
    }
}

impl QuadCopterMotors for GazeboDrone {
    fn set_data(&mut self, velocities_pct: [u8; 4]) {
        self.motors_signal.signal(velocities_pct);
    }
}
