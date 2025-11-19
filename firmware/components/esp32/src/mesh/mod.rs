use log::*;
use rusty_robot::mk_static;

pub struct Esp32MeshController<'d> {
    pub wifi_controller: esp_radio::wifi::WifiController<'d>,
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

use rusty_robot_drivers::radio::mesh::Bssid;
use rusty_robot_drivers::radio::mesh::MeshConfig;
trait MeshConfigExt {
    fn to_mode_config(&self, peer: Option<Bssid>) -> esp_radio::wifi::ModeConfig;
    fn to_scan_config(&self) -> esp_radio::wifi::ScanConfig<'_>;
}
impl MeshConfigExt for MeshConfig {
    fn to_mode_config(&self, peer: Option<Bssid>) -> esp_radio::wifi::ModeConfig {
        use esp_radio::wifi::{AccessPointConfig, AuthMethod, ClientConfig};

        return match peer {
            Some(bssid) => esp_radio::wifi::ModeConfig::ApSta(
                ClientConfig::default()
                    .with_channel(self.channel)
                    .with_ssid(self.ssid.clone())
                    .with_bssid(bssid)
                    .with_auth_method(AuthMethod::Wpa3Personal)
                    .with_password(self.password.clone()),
                AccessPointConfig::default()
                    .with_channel(self.channel)
                    .with_ssid(self.ssid.clone())
                    .with_auth_method(AuthMethod::Wpa3Personal)
                    .with_password(self.password.clone()),
            ),
            None => esp_radio::wifi::ModeConfig::ApSta(
                ClientConfig::default(),
                AccessPointConfig::default()
                    .with_channel(self.channel)
                    .with_ssid(self.ssid.clone())
                    .with_auth_method(AuthMethod::Wpa3Personal)
                    .with_password(self.password.clone()),
            ),
        };
    }

    fn to_scan_config(&self) -> esp_radio::wifi::ScanConfig<'_> {
        esp_radio::wifi::ScanConfig::default()
            .with_channel(self.channel)
            .with_ssid(&self.ssid)
    }
}

use rusty_robot_drivers::radio::mesh::{ScanEntry, ScanResults};
impl rusty_robot_drivers::radio::mesh::MeshNode for Esp32MeshController<'_> {
    fn start(&mut self, config: &MeshConfig) {
        self.wifi_controller
            .set_config(&config.to_mode_config(None))
            .unwrap();

        self.wifi_controller.start().unwrap();
    }

    fn is_started(&self) -> bool {
        self.wifi_controller.is_started().unwrap()
    }

    fn scan(&mut self, config: &MeshConfig) -> ScanResults {
        // perform the scan
        let results = self
            .wifi_controller
            .scan_with_config(config.to_scan_config())
            .unwrap();

        // create the response
        let mut ret = ScanResults::new();
        for entry in results {
            ret.add(ScanEntry {
                bssid: entry.bssid,
                rssi: entry.signal_strength,
            })
        }
        ret
    }

    fn connect(&mut self, config: &MeshConfig, bssid: rusty_robot_drivers::radio::mesh::Bssid) {
        // configure STA
        self.wifi_controller
            .set_config(&config.to_mode_config(Some(bssid)))
            .unwrap();

        match self.wifi_controller.connect() {
            Ok(_) => return,
            Err(_) => return,
        }
    }

    fn is_connected(&self) -> bool {
        self.wifi_controller.is_connected().unwrap()
    }
}
