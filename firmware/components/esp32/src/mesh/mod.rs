use rusty_robot::mk_static;

extern crate alloc;
use alloc::string::String;

pub struct MeshConfig {
    pub channel: u8,
    pub ssid: String,
    pub password: String,
}

impl MeshConfig {
    pub fn from_env() -> Self {
        MeshConfig {
            channel: env!("MESH_CHANNEL")
                .parse()
                .expect("failed to parse channel"),
            ssid: env!("MESH_SSID").into(),
            password: env!("MESH_PASSWORD").into(),
        }
    }

    pub fn to_wifi_mode_config(self, peer_bssid: Option<[u8; 6]>) -> esp_radio::wifi::ModeConfig {
        return match peer_bssid {
            Some(bssid) => esp_radio::wifi::ModeConfig::ApSta(
                esp_radio::wifi::ClientConfig::default()
                    .with_channel(self.channel)
                    .with_ssid(self.ssid.clone())
                    .with_bssid(bssid)
                    .with_auth_method(esp_radio::wifi::AuthMethod::Wpa3Personal)
                    .with_password(self.password.clone()),
                esp_radio::wifi::AccessPointConfig::default()
                    .with_channel(self.channel)
                    .with_ssid(self.ssid)
                    .with_auth_method(esp_radio::wifi::AuthMethod::Wpa3Personal)
                    .with_password(self.password),
            ),
            None => esp_radio::wifi::ModeConfig::ApSta(
                esp_radio::wifi::ClientConfig::default(),
                esp_radio::wifi::AccessPointConfig::default()
                    .with_channel(self.channel)
                    .with_ssid(self.ssid)
                    .with_auth_method(esp_radio::wifi::AuthMethod::Wpa3Personal)
                    .with_password(self.password),
            ),
        };
    }
}

pub struct Esp32MeshController<'d> {
    // mesh_config: MeshConfig,
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
        //  must set dummy ModeConfig - else set_protocol() fails
        wifi_controller.set_config(&esp_radio::wifi::ModeConfig::ApSta(
                esp_radio::wifi::ClientConfig::default(),
                esp_radio::wifi::AccessPointConfig::default()
        )).unwrap();
        wifi_controller.set_protocol(protocols.into()).unwrap();

        Esp32MeshController {
            wifi_controller,
            wifi_interfaces,
        }
    }
}

pub struct MeshController<'d> {
    pub wifi_controller: esp_radio::wifi::WifiController<'d>,
    pub wifi_interfaces: esp_radio::wifi::Interfaces<'d>,
}

/// Create a new mesh node
/// environment variables (see config.toml)
///     * ESP_WIFI_CONFIG_COUNTRY_CODE - constrains radio operation per regulation
///     * AP_CHANNEL - initial channel for the mesh
///     * AP_SSID - name of the mesh
///     * AP_PASSWORD - secret used to join mesh
pub fn new<'d>(
    // inited: &'d esp_radio::Controller<'d>,
    device: esp_hal::peripherals::WIFI<'d>,
) -> MeshController<'d> {
    // TODO is this already handled?
    // parse environment variable into country_code
    // let country_code_bytes = env!("ESP_WIFI_CONFIG_COUNTRY_CODE").as_bytes();
    // let country_code: [u8; 2] = [country_code_bytes[0], country_code_bytes[1]];

    let radio = mk_static!(esp_radio::Controller, esp_radio::init().unwrap());

    // configure radio
    let radio_config = esp_radio::wifi::Config::default();
    // .with_country_code(country_code);

    let (mut wifi_controller, wifi_interfaces) =
        esp_radio::wifi::new(radio, device, radio_config).unwrap();

    // configure wifi controller
    wifi_controller
        .set_mode(esp_radio::wifi::WifiMode::ApSta)
        .unwrap();
    // wifi_controller
    //     .set_config(&esp_radio::wifi::ModeConfig::ApSta(
    //         // STA configuration
    //         esp_radio::wifi::ClientConfig::default(),
    //         // AP configuration
    //         esp_radio::wifi::AccessPointConfig::default()
    //             .with_channel(
    //                 env!("AP_CHANNEL")
    //                     .parse()
    //                     .expect("failed to parse AP_CHANNEL"),
    //             )
    //             .with_ssid(env!("AP_SSID").into())
    //             .with_auth_method(esp_radio::wifi::AuthMethod::Wpa2Personal)
    //             .with_password(env!("AP_PASSWORD").into()),
    //     ))
    //     .expect("Failed to configure AP and STA");

    //      configure radio for WiFi LR (must be after set_config)
    wifi_controller
        .set_protocol(esp_radio::wifi::Protocol::P802D11LR.into())
        .expect("Failed to enable WiFi LR");

    MeshController {
        wifi_controller,
        wifi_interfaces,
    }
}

impl MeshController<'_> {
    pub fn start(mut self) -> Result<(), esp_radio::wifi::WifiError> {
        // configure WiFi events
        // esp_radio::wifi::event::StaStart::replace_handler(f)

        self.wifi_controller.start()
    }

    // pub fn connect(
    //     mut self,
    //     peer: esp_radio::wifi::AccessPointInfo,
    // ) -> Result<(), esp_radio::wifi::WifiError> {
    //     // must reconfigure the wifi_controller per the API
    //     // FIXME clean up this config creation
    //     self.wifi_controller
    //         .set_config(&esp_radio::wifi::ModeConfig::ApSta(
    //             // STA configuration
    //             esp_radio::wifi::ClientConfig::default()
    //                 .with_channel(
    //                     env!("AP_CHANNEL")
    //                         .parse()
    //                         .expect("failed to parse AP_CHANNEL"),
    //                 )
    //                 .with_ssid(env!("AP_SSID").into())
    //                 .with_bssid(peer.bssid)
    //                 .with_auth_method(esp_radio::wifi::AuthMethod::Wpa2Personal)
    //                 .with_password(env!("AP_PASSWORD").into()),
    //             // AP configuration
    //             esp_radio::wifi::AccessPointConfig::default()
    //                 .with_channel(
    //                     env!("AP_CHANNEL")
    //                         .parse()
    //                         .expect("failed to parse AP_CHANNEL"),
    //                 )
    //                 .with_ssid(env!("AP_SSID").into())
    //                 .with_auth_method(esp_radio::wifi::AuthMethod::Wpa2Personal)
    //                 .with_password(env!("AP_PASSWORD").into()),
    //         ))
    //         .expect("Failed to configure AP and STA");

    //     self.wifi_controller.connect()
    // }
}
