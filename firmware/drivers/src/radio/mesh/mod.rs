use rusty_robot::arrayvec::{self, ArrayVec};

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

#[derive(Debug)]
pub struct ScanEntry {
    pub bssid: Bssid,
    /// signal strength in dBm (decibel-milliwatts)
    pub rssi: i8,
}

const MAX_SCAN_RESULTS: usize = 4;
/// fixed size vector of ScanEntry (ordered in descending signal strength)
pub struct ScanResults {
    results: ArrayVec<ScanEntry, MAX_SCAN_RESULTS>,
}
impl core::ops::Index<usize> for ScanResults {
    type Output = ScanEntry;
    fn index(&self, index: usize) -> &Self::Output {
        &self.results[index]
    }
}
impl ScanResults {
    pub fn new() -> Self {
        ScanResults {
            results: ArrayVec::new(),
        }
    }

    /// adds the entry to the list (descending rssi order)
    /// if entry's rssi is not greater than all other entries, it is dropped
    pub fn push(&mut self, entry: ScanEntry) {
        if self.results.is_empty() {
            self.results.push(entry);
        }
        else {
            // find an insertion point (rssi smaller than new entry)
            for i in 0..self.results.len()
            {
                if self.results[i].rssi < entry.rssi {
                    // if full, drop the last entry
                    if self.results.is_full() {
                        self.results.pop();
                    }
                    self.results.insert(i, entry);
                    return;
                }
            }
            // wasn't inserted, so can we append?
            if !self.results.is_full() {
                self.results.push(entry);
            }
        }
    }
}
impl IntoIterator for ScanResults {
    type Item = ScanEntry;

    type IntoIter = arrayvec::IntoIter<ScanEntry, MAX_SCAN_RESULTS>;

    fn into_iter(self) -> Self::IntoIter {
        self.results.into_iter()
    }
}

#[cfg(test)]
mod scan_results_tests {
    use super::*;

    #[test]
    fn replaces_lowest_rssi_entry() {
        // setup
        let mut scan_results = ScanResults::new();
        // act
        for i in 0..(MAX_SCAN_RESULTS + 1) {
            scan_results.push(ScanEntry {
                bssid: Default::default(),
                rssi: i as i8,
            });
        }
        // assert that lowest rssi entry was replaced
        assert_eq!(
            scan_results[0].rssi,
            (MAX_SCAN_RESULTS as i8),
            "didn't replace lower rssi"
        );

        // TEST USE - print the entries
        // for e in scan_results {
        //     extern crate std;
        //     use std::println;
        //     println!("entry: [{:?}]", e);
        // }
    }
}
