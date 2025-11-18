use rusty_robot::arrayvec::ArrayVec;

pub type Ssid = [u8; 32];
pub type Password = [u8; 63];
pub type Bssid = [u8; 6];

pub struct MeshConfig {
    pub channel: u8,
    pub ssid: Ssid,
    pub password: Password,

    /// bssid of the root
    pub root: Option<Bssid>,
}
impl MeshConfig {
    pub fn new(channel: u8, ssid: &str, password: &str) -> Self {
        Self {
            channel: channel,
            ssid: ssid.as_bytes().try_into().expect("ssid too long"),
            password: password.as_bytes().try_into().expect("password too long"),
            root: None,
        }
    }
}

pub struct Mesh {
    pub config: MeshConfig,
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

    fn scan(self, config: MeshConfig) -> ScanResults;
}
pub struct ScanEntry {
    pub bssid: Bssid,
    /// signal strength in dBm (decibel-milliwatts)
    pub rssi: i8,
}
// pub type ScanResults = ArrayVec<ScanEntry, 4>;
const MAX_SCAN_RESULTS: usize = 4;
pub struct ScanResults {
    pub results: ArrayVec<ScanEntry, MAX_SCAN_RESULTS>,
}
impl ScanResults {
    pub fn new() -> Self {
        ScanResults {
            results: ArrayVec::new(),
        }
    }

    /// adds the entry to the list if
    ///    * vector not full
    ///    * or rssi is larger than all other entries
    pub fn push(&mut self, entry: ScanEntry) {
        if self.results.is_full() {
            // scan the vector and replace the lowest rssi entry if this rssi is greater
            let mut lowest_index = 0;
            let mut lowest_rssi = self.results[0].rssi;
            for i in 1..self.results.capacity() {
                if self.results[i].rssi < lowest_rssi {
                    lowest_index = i;
                    lowest_rssi = self.results[i].rssi;
                }
            }
            if entry.rssi > lowest_rssi {
                // replace the entry
                self.results[lowest_index] = entry;
            }
        } else {
            // not full, so append this entry
            self.results.push(entry);
        }
    }
}

#[cfg(test)]
// extern crate std;
mod scan_results_tests {
    use core::char::MAX;

    use super::*;

    #[test]
    fn replaces_lowest_rssi_entry() {
        // setup
        let mut scan_results = ScanResults::new();
        // act
        for i in 0..(MAX_SCAN_RESULTS + 1)
        {
            scan_results.push(ScanEntry {
                bssid: Default::default(),
                rssi: i as i8
            });
        }
        // assert that lowest rssi entry was replaced
        assert_eq!(scan_results.results[0].rssi, (MAX_SCAN_RESULTS as i8), "didn't replace lower rssi");
    }
}