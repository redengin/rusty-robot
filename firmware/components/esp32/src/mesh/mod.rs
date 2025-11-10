use rusty_robot::mk_static;

extern crate alloc;
use alloc::string::String;

// pub struct MeshConfig {
//     pub channel: u8,
//     pub ssid: String,
//     pub password: String,
// }

use rusty_robot_drivers::radio::mesh::MeshConfig;

// impl MeshConfig {
//     pub fn from_env() -> Self {
//         MeshConfig {
//             channel: env!("MESH_CHANNEL")
//                 .parse()
//                 .expect("failed to parse channel"),
//             ssid: env!("MESH_SSID").into(),
//             password: env!("MESH_PASSWORD").into(),
//         }
//     }

//     pub fn to_wifi_mode_config(self, peer_bssid: Option<[u8; 6]>) -> esp_radio::wifi::ModeConfig {
//         return match peer_bssid {
//             Some(bssid) => esp_radio::wifi::ModeConfig::ApSta(
//                 esp_radio::wifi::ClientConfig::default()
//                     .with_channel(self.channel)
//                     .with_ssid(self.ssid.clone())
//                     .with_bssid(bssid)
//                     .with_auth_method(esp_radio::wifi::AuthMethod::Wpa3Personal)
//                     .with_password(self.password.clone()),
//                 esp_radio::wifi::AccessPointConfig::default()
//                     .with_channel(self.channel)
//                     .with_ssid(self.ssid)
//                     .with_auth_method(esp_radio::wifi::AuthMethod::Wpa3Personal)
//                     .with_password(self.password),
//             ),
//             None => esp_radio::wifi::ModeConfig::ApSta(
//                 esp_radio::wifi::ClientConfig::default(),
//                 esp_radio::wifi::AccessPointConfig::default()
//                     .with_channel(self.channel)
//                     .with_ssid(self.ssid)
//                     .with_auth_method(esp_radio::wifi::AuthMethod::Wpa3Personal)
//                     .with_password(self.password),
//             ),
//         };
//     }
// }

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
        wifi_controller.set_mode(esp_radio::wifi::WifiMode::ApSta).unwrap();
        wifi_controller.set_protocol(protocols.into()).unwrap();

        Esp32MeshController {
            wifi_controller,
            wifi_interfaces,
        }
    }
}


impl rusty_robot_drivers::radio::mesh::MeshNode for Esp32MeshController<'_>
{
    fn start(mut self, config: MeshConfig) {
        self.wifi_controller.start().unwrap();
    }
}