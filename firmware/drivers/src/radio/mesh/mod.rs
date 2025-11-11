pub type Ssid = [u8; 32];
pub type Password = [u8; 63];
pub type Bssid = [u8; 6];

pub struct MeshConfig {
    pub channel: u8,
    pub ssid: Ssid,
    pub password: Password,

    /// bssid of the root
    pub root: Option<Bssid>
}
impl MeshConfig {
    pub fn new(channel: u8, ssid: &str, password: &str) -> Self
    {
        Self {
            channel: channel,
            ssid: ssid.as_bytes().try_into().expect("ssid too long"),
            password: password.as_bytes().try_into().expect("password too long"),
            root: None
        }
    }
}

pub struct Mesh {
    config: MeshConfig,
}

impl Mesh {
    pub fn new(config: MeshConfig) -> Self {
        Self { config }
    }

    pub fn run(self) -> ! {
        loop {
            // handle all current connections to AP

            // scan for nodes 
        }
    }
}

pub trait MeshNode {
    /// initialize the radios and begin broadcasting SSID
    fn start(self, config: MeshConfig);

    fn scan(self, config: MeshConfig);
}

// pub struct ScanEntry {
//     // hardware id of the node
//     pub bssid: Bssid,
//     // signal strength
//     pub rssi: i8,
// }
