use rusty_robot_drivers::imu_traits::{ImuReader, ImuData, ImuError};

use gz as gazebosim;

pub struct GazeboDrone {
    node : gazebosim::transport::Node,
}

impl GazeboDrone {
    pub fn new(robot_name: &String) -> Self
    {
        // connect to gazebo
        let mut node= gazebosim::transport::Node::new().unwrap();

        // subscribe to drone sensors
        let imu_topic = format!("/world/openworld/model/{}/link/base_link/sensor/imu_sensor/imu", robot_name);
        assert!(node.subscribe(&imu_topic, GazeboDrone::imu_update));
        let navsat_topic = format!("/world/openworld/model/{}/link/base_link/sensor/navsat_sensor/navsat", robot_name);
        assert!(node.subscribe(&navsat_topic, GazeboDrone::imu_update));

        GazeboDrone {
            node: node,
        }
    }

    fn imu_update(msg: gz::msgs::imu::IMU) {
        // TODO parse message for get_data()
    }
    fn navsat_update(msg: gz::msgs::imu::IMU) {
        // TODO parse message for get_data()
    }
}

impl ImuReader for GazeboDrone {
    /// Returns the most recent sensor data.
    fn get_data(&self) -> Result<ImuData, ImuError> {
        // FIXME
        // Ok(self.data.read().map(|data| *data)?)
        Ok(ImuData{..Default::default()})
    }

    /// Stops the reading thread.
    fn stop(&self) -> Result<(), ImuError> {
        // not necessary for sim
        Ok(())
    }
}
