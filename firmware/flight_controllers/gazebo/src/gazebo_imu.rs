use rusty_robot_drivers::imu_traits::{ImuReader, ImuData, ImuError};

use gz as gazebosim;

pub struct GazeboImu {
    node : gazebosim::transport::Node,
}

impl GazeboImu {
    pub fn new(robotName: String) -> Self
    {
        let mut node= gazebosim::transport::Node::new().unwrap();
        // FIXME find topic for robot's IMU
        let imuTopic = "hello world";
        node.subscribe(imuTopic, GazeboImu::update);

        GazeboImu {
            node: node,
        }
    }

    fn update(msg: gz::msgs::imu::IMU) {
        // TODO parse message for get_data()
    }
}

impl ImuReader for GazeboImu {
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
