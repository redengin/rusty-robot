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

use rusty_robot_drivers::radio::mesh::{self, ScanEntry};
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

impl rusty_robot_drivers::radio::mesh::MeshNode for Esp32MeshController<'_> {
    fn start(mut self, config: mesh::MeshConfig) {
        self.wifi_controller
            .set_config(&config.to_mode_config())
            .unwrap();

        self.wifi_controller.start().unwrap();
    }

    fn scan(mut self, config: mesh::MeshConfig) -> mesh::ScanResults {
        // perform the scan
        let results = self
            .wifi_controller
            .scan_with_config(config.to_scan_config())
            .unwrap();

        // create the response
        let mut ret = mesh::ScanResults::new();
        for entry in results {
            ret.push(ScanEntry {
                bssid: entry.bssid,
                rssi: entry.signal_strength,
            })
        }
        ret
    }
}
