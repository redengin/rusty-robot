use embassy_time::Timer;
use gz::{self as gazebosim};
use rusty_robot_drivers::gps::{Gps, GpsState};
use rusty_robot_drivers::imu_traits::{self, ImuData, ImuError, ImuReader};
use rusty_robot_drivers::systems;
use std::f64::consts::PI;

/// max RPM of motors
// note the skybot caps motor velocity at 1000 radians/s
const MAX_MOTOR_RPM: f64 = 40_000.0;

pub struct GazeboDrone {
    node: gazebosim::transport::Node,

    imu_topic: String,
    imu_data: Option<imu_traits::ImuData>,

    gps_topic: String,
    gps_data: GpsState,

    motors_topic: String,
    motor_publisher: Option<gazebosim::transport::Publisher<gazebosim::msgs::actuators::Actuators>>,
}

impl GazeboDrone {
    pub fn new(robot_name: &String) -> Self {
        GazeboDrone {
            node: gazebosim::transport::Node::new().unwrap(),

            imu_topic: format!(
                "/world/openworld/model/{}/link/base_link/sensor/imu_sensor/imu",
                robot_name
            ),
            imu_data: None,

            gps_topic: format!(
                "/world/openworld/model/{}/link/base_link/sensor/navsat_sensor/navsat",
                robot_name
            ),
            gps_data: Default::default(),

            motors_topic: format!("/{}/command/motor_speed", robot_name),
            motor_publisher: None,
        }
    }

    pub async fn run(&'static mut self) {
        // process IMU data via inline callback
        log::debug!("subscribing to IMU '{}'", self.imu_topic);
        assert!(
            self.node
                .subscribe(self.imu_topic.as_str(), |msg: gz::msgs::imu::IMU| {
                    log::trace!("imu msg {}", msg.entity_name);

                    let mut imu_data = ImuData {
                        ..Default::default()
                    };

                    // accelerometer
                    if msg.linear_acceleration.is_some() {
                        let gz_linear_acceleration = msg.linear_acceleration.unwrap();
                        imu_data.accelerometer = Some(imu_traits::Vector3 {
                            x: gz_linear_acceleration.x as f32,
                            y: gz_linear_acceleration.y as f32,
                            z: gz_linear_acceleration.z as f32,
                        });
                    }

                    // gyroscope
                    if msg.angular_velocity.is_some() {
                        let gz_angular_velocity = msg.angular_velocity.unwrap();
                        imu_data.gyroscope = Some(imu_traits::Vector3 {
                            x: gz_angular_velocity.x as f32,
                            y: gz_angular_velocity.y as f32,
                            z: gz_angular_velocity.z as f32,
                        });
                    }

                    // orientation
                    if msg.orientation.is_some() {
                        let gz_quaternion = msg.orientation.unwrap();
                        imu_data.quaternion = Some(imu_traits::Quaternion {
                            w: gz_quaternion.w as f32,
                            x: gz_quaternion.x as f32,
                            y: gz_quaternion.y as f32,
                            z: gz_quaternion.z as f32,
                        });
                    }

                    self.imu_data = Some(imu_data);
                })
        );

        // process navsat data via inline callback
        log::debug!("subscribing to GPS '{}'", self.gps_topic);
        assert!(
            self.node
                .subscribe(self.gps_topic.as_str(), |msg: gz::msgs::navsat::NavSat| {
                    log::trace!("gps msg {}", msg.frame_id);

                    self.gps_data = GpsState {
                        latitude: Some(msg.latitude_deg),
                        longitude: Some(msg.longitude_deg),
                        altitude: Some(msg.altitude),
                        satellite_count: None,
                        // TODO parse the msg.header.stamp (timestamp) to set the timestamp
                        timestamp: None,
                    }
                })
        );

        // provide the motors topic
        self.motor_publisher = Some(
            self.node
                .advertise::<gazebosim::msgs::actuators::Actuators>(&self.motors_topic)
                .unwrap(),
        );

        // sit and spin
        loop {
            Timer::after_secs(1).await;
            log::trace!("running...");
        }
    }
}

impl ImuReader for GazeboDrone {
    /// Returns the most recent sensor data.
    fn get_data(&self) -> Result<ImuData, ImuError> {
        match self.imu_data {
            None => return Err(ImuError::ReadError("no data".to_string())),
            Some(data) => Ok(data),
        }
    }

    /// Stops the reading thread.
    fn stop(&self) -> Result<(), ImuError> {
        // not implementable for sim
        Ok(())
    }
}

impl Gps for GazeboDrone {
    fn get_data(&self) -> GpsState {
        self.gps_data.clone()
    }
}

impl systems::QuadCopterMotors for GazeboDrone {
    fn set_data(&mut self, velocities: [u8; 4]) {
        match &mut self.motor_publisher {
            Some(publisher) => {
                let mut msg = gazebosim::msgs::actuators::Actuators::new();
                msg.velocity = vec![
                    rpm_to_radians_per_second(MAX_MOTOR_RPM / 100.0 * (velocities[0] as f64)),
                    rpm_to_radians_per_second(MAX_MOTOR_RPM / 100.0 * (velocities[1] as f64)),
                    rpm_to_radians_per_second(MAX_MOTOR_RPM / 100.0 * (velocities[2] as f64)),
                    rpm_to_radians_per_second(MAX_MOTOR_RPM / 100.0 * (velocities[3] as f64)),
                ];

                if publisher.publish(&msg) {
                    log::trace!("sent motor update {:?}", msg.velocity);
                } else {
                    log::warn!("failed to send motor update");
                }
            }
            None => {
                log::warn!("motor publisher not active, ignoring motor update");
            }
        }
    }
}

fn rpm_to_radians_per_second(rpm: f64) -> f64 {
    2.0 * PI * rpm * 60.0
}
