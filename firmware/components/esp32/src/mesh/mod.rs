use log::*;
use rusty_robot::mk_static;

pub struct Esp32MeshController<'d> {
    wifi_controller: esp_radio::wifi::WifiController<'d>,
    wifi_interfaces: esp_radio::wifi::Interfaces<'d>,
}

impl Esp32MeshController<'_> {
    pub fn new(
        device: esp_hal::peripherals::WIFI<'static>,
        protocols: esp_radio::wifi::Protocol,
    ) -> Self {
        // static radio controller
        let radio = mk_static!(esp_radio::Controller, esp_radio::init().unwrap());

        // configure wifi radio
        let radio_config = esp_radio::wifi::Config::default();
        // TODO does ESP32 use the env ESP_WIFI_CONFIG_COUNTRY_CODE?
        // if not we should do it
        // .with_country_code(country_code);

        let (mut wifi_controller, wifi_interfaces) =
            esp_radio::wifi::new(radio, device, radio_config).unwrap();

        // set the wifi protocols
        //  must set_mode() before setting protocol
        wifi_controller
            .set_mode(esp_radio::wifi::WifiMode::ApSta)
            .unwrap();
        wifi_controller.set_protocol(protocols.into()).unwrap();

        trace!("created the radio controller");
        Esp32MeshController {
            wifi_controller,
            wifi_interfaces,
        }
    }
}

use rusty_robot_drivers::radio::mesh;
trait MeshConfigExt {
    fn to_mode_config(&self) -> esp_radio::wifi::ModeConfig;
    fn to_scan_config(&self) -> esp_radio::wifi::ScanConfig<'_>;
}
impl MeshConfigExt for mesh::MeshConfig {
    fn to_mode_config(&self) -> esp_radio::wifi::ModeConfig {
        use esp_radio::wifi::{self, AccessPointConfig, ClientConfig};
        esp_radio::wifi::ModeConfig::ApSta(
            ClientConfig::default(),
            AccessPointConfig::default()
                .with_channel(self.channel)
                .with_ssid(str::from_utf8(&self.ssid).unwrap().into())
                .with_auth_method(wifi::AuthMethod::Wpa3Personal)
                .with_password(str::from_utf8(&self.password).unwrap().into()),
        )
    }

    fn to_scan_config(&self) -> esp_radio::wifi::ScanConfig<'_> {
        esp_radio::wifi::ScanConfig::default()
            .with_channel(self.channel)
            .with_ssid(str::from_utf8(&self.ssid).unwrap().into())
    }
}

use esp_radio::wifi::AccessPointInfo;
extern crate alloc;
fn esp32_scan_to_scan_results(results: alloc::vec::Vec<AccessPointInfo>) -> mesh::ScanResults {
    let mut ret = mesh::ScanResults::new();

    for result in &results {
        if ret.is_full() {
            // scan the vector and replace the lowest rssi entry if this rssi is greater
            let mut lowest_index = 0;
            let mut lowest_rssi = ret[0].rssi;
            for i in 1..ret.capacity() {
                if ret[i].rssi < lowest_rssi {
                    lowest_index = i;
                    lowest_rssi = ret[i].rssi;
                }
            }
            if result.signal_strength > lowest_rssi {
                ret[lowest_index].bssid = result.bssid;
                ret[lowest_index].rssi = result.signal_strength;
            }
        } else {
            // not full, so append this entry
            ret.push(mesh::ScanEntry {
                bssid: result.bssid,
                rssi: result.signal_strength,
            })
        }
    }

    ret
}
#[cfg(test)]
mod scan_results_tests {
    use super::*;
    use esp_radio::wifi::AccessPointInfo;
    use rusty_robot_drivers::radio::mesh::ScanResults;

    #[test]
    fn replaces_lowest_rssi_entry() {
        // setup - create an oversized (+1) vector of incremental rssi
        let mut results = alloc::vec::Vec::<AccessPointInfo>::new();
        let dummy = ScanResults::new();
        let max_i = dummy.capacity() + 1;
        for i in 0..max_i {
            results.push(AccessPointInfo {
                bssid: i,
                signal_strength: i,
                ..Default::default()
            });
        }
        // act
        let scan_results = esp32_scan_to_scan_results(results);
        assert!(scan_results.is_full(), "setup failed to overflow ScanResults");
        // assert that lowest rssi entry was replaced
        assert!(scan_results[0].bssid == max_i, "didn't replace lower rssi");
        assert!(scan_results[0].rssi == max_i, "didn't replace lower rssi");
    }
}

impl rusty_robot_drivers::radio::mesh::MeshNode for Esp32MeshController<'_> {
    fn start(mut self, config: mesh::MeshConfig) {
        self.wifi_controller
            .set_config(&config.to_mode_config())
            .unwrap();

        self.wifi_controller.start().unwrap();
    }

    fn scan(mut self, config: mesh::MeshConfig) -> mesh::ScanResults {
        let results = self
            .wifi_controller
            .scan_with_config(config.to_scan_config())
            .unwrap();

        esp32_scan_to_scan_results(results)
    }
}
