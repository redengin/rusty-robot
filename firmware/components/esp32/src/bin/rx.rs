#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use log::*;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    error!("PANIC - {:?}", info);
    esp_hal::system::software_reset()
}

// provide scheduler api
use embassy_time::{Duration, Timer};

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(spawner: embassy_executor::Spawner) -> ! {
    // initialize the logger
    esp_println::logger::init_logger_from_env();

    // initialize the SoC
    use esp_hal::clock::CpuClock;
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // create a heap allocator (required by esp_radio)
    esp_alloc::heap_allocator!(#[unsafe(link_section = ".dram2_uninit")] size: 98767);

    // initialize embassy scheduler
    use esp_hal::timer::timg::TimerGroup;
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    // initialize ESP-NOW
    let radio = esp_radio::init().expect("Failed to initialize Wi-Fi/BLE controller");
    const ESP_WIFI__CONFIG_COUNTRY_CODE: &str = env!("ESP_WIFI_CONFIG_COUNTRY_CODE");
    let country_code: [u8; 2] = ESP_WIFI__CONFIG_COUNTRY_CODE
        .as_bytes()
        .try_into()
        .expect("set [env] ESP_WIFI_CONFIG_COUNTRY_CODE");
    let country_info = esp_radio::wifi::CountryInfo::from(country_code);
    let (mut radio_controller, radio_interfaces) = esp_radio::wifi::new(
        &radio,
        peripherals.WIFI,
        esp_radio::wifi::Config::default().with_country_code(country_info),
    ).expect("Failed to initialize WiFi");
    radio_controller.set_mode(esp_radio::wifi::WifiMode::Sta).unwrap();
    radio_controller.start().unwrap();
    let mut esp_now = radio_interfaces.esp_now;
    // TODO is this necessary?
    let radio_channel = 11;
    esp_now.set_channel(radio_channel).unwrap();
    info!("Initialized ESP-NOW [version: {}] on channel {}", esp_now.version().unwrap(), radio_channel);

    // TODO: Spawn some tasks
    let _ = spawner;

    loop {
        Timer::after(Duration::from_secs(1)).await;
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.1/examples/src/bin
}
