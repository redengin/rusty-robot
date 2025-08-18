use gz::{self as gazebosim};

use rusty_robot_drivers::imu_traits::{self, ImuData, ImuError, ImuReader};
use rusty_robot_drivers::gps_traits::{Gps, GpsState};

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};

pub struct GazeboDrone {
    pub imu_topic: String,
    pub imu_signal: Signal<CriticalSectionRawMutex, ImuData>,

    pub gps_topic: String,
    pub gps_signal: Signal<CriticalSectionRawMutex, GpsState>,
}

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
        }
    }

    pub fn run(&'static self) {
        let mut node = gazebosim::transport::Node::new().unwrap();

        // handle IMU updates
        log::debug!("subscribing to IMU '{}'", self.imu_topic);
        assert!(
            node.subscribe(self.imu_topic.as_str(), |msg: gz::msgs::imu::IMU| {
                log::trace!("imu msg {}", msg.entity_name);

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

                let gps_data = GpsState {
                    latitude: Some(msg.latitude_deg),
                    longitude: Some(msg.longitude_deg),
                    altitude: Some(msg.altitude),
                    satellite_count: None,
                    // TODO parse the msg.header.stamp (timestamp) to set the timestamp
                    timestamp: None,
                };

                self.gps_signal.signal(gps_data);
            })
        );
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