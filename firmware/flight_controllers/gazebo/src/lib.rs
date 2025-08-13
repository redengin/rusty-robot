use embassy_time::Timer;
use gz as gazebosim;
use log::*;
use rusty_robot_drivers::imu_traits::{ImuData, ImuError, ImuReader};

pub struct GazeboDrone {
    node: gazebosim::transport::Node,
    imu_topic: String,
    navsat_topic: String,
}

impl GazeboDrone {
    pub fn new(robot_name: &String) -> Self {
        GazeboDrone {
            node: gazebosim::transport::Node::new().unwrap(),
            imu_topic: format!(
                "/world/openworld/model/{}/link/base_link/sensor/imu_sensor/imu",
                robot_name
            ),
            navsat_topic: format!(
                "/world/openworld/model/{}/link/base_link/sensor/navsat_sensor/navsat",
                robot_name
            ),
        }
    }

    pub async fn run(&mut self) {
        // process IMU data via inline callback
        assert!(
            self.node
                .subscribe(self.imu_topic.as_str(), |msg: gz::msgs::imu::IMU| {
                    // TODO parse the data into get_data response
                    
                })
        );
        // process navsat data via inline callback
        assert!(self.node.subscribe(
            self.navsat_topic.as_str(),
            |msg: gz::msgs::navsat::NavSat| {
                // TODO parse the data into ??? response
            }
        ));

        // sit and spin
        loop {
            Timer::after_secs(1).await;
        }
    }
}

impl ImuReader for GazeboDrone {
    /// Returns the most recent sensor data.
    fn get_data(&self) -> Result<ImuData, ImuError> {
        // FIXME
        // Ok(self.data.read().map(|data| *data)?)
        Ok(ImuData {
            ..Default::default()
        })
    }

    /// Stops the reading thread.
    fn stop(&self) -> Result<(), ImuError> {
        // not necessary for sim
        Ok(())
    }
}
