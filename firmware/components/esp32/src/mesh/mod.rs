pub struct MeshController<'d> {
    wifi_controller: esp_radio::wifi::WifiController<'d>,
    wifi_interfaces: esp_radio::wifi::Interfaces<'d>,
}

pub fn new<'d>(
    inited: &'d esp_radio::Controller<'d>,
    device: esp_hal::peripherals::WIFI<'d>,
) -> MeshController<'d> {
    // parse environment variable into country_code
    let country_code_bytes = env!("ESP_WIFI_CONFIG_COUNTRY_CODE").as_bytes();
    let country_code: [u8; 2] = [country_code_bytes[0], country_code_bytes[1]];

    // configure radio
    let radio_config = esp_radio::wifi::Config::default().with_country_code(country_code);

    let (wifi_controller, wifi_interfaces) =
        esp_radio::wifi::new(inited, device, radio_config).unwrap();

    MeshController {
        wifi_controller,
        wifi_interfaces,
    }
}
