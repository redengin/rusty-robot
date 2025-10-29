pub fn new<'d>(
    inited: &'d esp_radio::Controller<'d>,
    device: esp_hal::peripherals::WIFI<'d>,
) -> Result<
    (esp_radio::wifi::WifiController<'d>, esp_radio::wifi::Interfaces<'d>),
    esp_radio::wifi::WifiError,
> {
    // parse environment variable into country_code
    let country_code_bytes = env!("ESP_WIFI_CONFIG_COUNTRY_CODE").as_bytes();
    let country_code: [u8;2] = [country_code_bytes[0], country_code_bytes[1]];

    // configure radio
    let radioConfig = esp_radio::wifi::Config::default()
        .with_country_code(country_code);

    esp_radio::wifi::new(inited, device, radioConfig)
}
