
pub type Bssid = [u8;6];

pub struct MeshConfig<'a> {
    ssid: &'a str,
    channel: u8,
    password: &'a str,

    /// bssid of the root
    root: Bssid,
}

pub struct ScanEntry {
    // hardware id of the node
    pub bssid: Bssid,
    // signal strength
    pub rssi: i8,
}