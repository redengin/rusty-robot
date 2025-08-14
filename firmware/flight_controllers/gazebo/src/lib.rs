use embassy_time::Timer;
use gz::{self as gazebosim};
use rusty_robot_drivers::imu_traits::{self, ImuData, ImuError, ImuReader};
use rusty_robot_drivers::nmea_parser;
use rusty_robot_drivers::nmea_parser::gnss::{self, GnsData};

pub struct GazeboDrone {
    node: gazebosim::transport::Node,
    imu_topic: String,
    imu_data: Option<imu_traits::ImuData>,
    navsat_topic: String,
    navsat_data: Option<GnsData>,
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

            navsat_topic: format!(
                "/world/openworld/model/{}/link/base_link/sensor/navsat_sensor/navsat",
                robot_name
            ),
            navsat_data: None,
        }
    }

    pub async fn run(&'static mut self) {
        // process IMU data via inline callback
        assert!(
            self.node
                .subscribe(self.imu_topic.as_str(), |msg: gz::msgs::imu::IMU| {
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
        // assert!(self.node.subscribe(
        //     self.navsat_topic.as_str(),
        //     |msg: gz::msgs::navsat::NavSat| {
        //         let mut navsat_data = GnsData {
        //             source: gnss::NavigationSystem::Other,
        //             timestamp: None,
        //             latitude: None,
        //             longitude: None,
        //             gps_mode: nmea_parser::gnss::gns::GnsModeIndicator::SimulationMode,

        //         };
        //     }
        // ));

        // sit and spin
        loop {
            Timer::after_secs(1).await;
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
