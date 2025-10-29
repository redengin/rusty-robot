#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

// provide panic handler
use rusty_robot_esp32::{self as _, mesh};

use log::*;

// provide scheduler api
use embassy_time::{Duration, Timer};

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(spawner: embassy_executor::Spawner) -> ! {
    // initialize the logger
    esp_println::logger::init_logger_from_env();

    // create a heap allocator (required by esp_radio)
    const HEAP_SIZE: usize = 98767;
    esp_alloc::heap_allocator!(#[unsafe(link_section = ".dram2_uninit")] size: HEAP_SIZE);

    // initialize the SoC
    use esp_hal::clock::CpuClock;
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // initialize embassy scheduler
    use esp_hal::timer::timg::TimerGroup;
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    // create the radio mesh
    let radioController = esp_radio::init().unwrap();
    let mesh = mesh::new(&radioController, peripherals.WIFI);
    // let mesh_node = mk_static!(MeshNode, MeshNode::new(peripherals.WIFI));

    // // initialize WiFi Long Range (LR) for mesh (both AP and STA)
    // let radio = esp_radio::init().expect("Failed to initialize Wi-Fi/BLE controller");
    // let (mut radio_controller, _radio_interfaces) =
    //     esp_radio::wifi::new(&radio, peripherals.WIFI, esp_radio::wifi::Config::default()).unwrap();
    // //      configure radio for mesh (AP and STA)
    // radio_controller
    //     .set_mode(esp_radio::wifi::WifiMode::ApSta)
    //     .unwrap();
    // //      configure the AP and STA
    // radio_controller
    //     .set_config(&esp_radio::wifi::ModeConfig::ApSta(
    //         // STA configuration
    //         esp_radio::wifi::ClientConfig::default(),
    //         // AP configuration
    //         esp_radio::wifi::AccessPointConfig::default()
    //             .with_ssid(env!("AP_SSID").into())
    //             .with_channel(
    //                 env!("AP_CHANNEL")
    //                     .parse()
    //                     .expect("failed to parse AP_CHANNEL"),
    //             )
    //             .with_auth_method(esp_radio::wifi::AuthMethod::Wpa2Personal)
    //             .with_password(env!("AP_PASSWORD").into()),
    //     ))
    //     .expect("Failed to configure AP and STA");
    // //      configure radio for WiFi LR (must be after set_config)
    // radio_controller
    //     .set_protocol(esp_radio::wifi::Protocol::P802D11LR.into())
    //     .expect("Failed to enable WiFi LR");

    // //      start the radio controller
    // radio_controller.start().unwrap();
    // info!(
    //     "SSID: {} \t channel: {}",
    //     env!("AP_SSID"),
    //     env!("AP_CHANNEL")
    // );

    // TODO: Spawn some tasks
    let _ = spawner;

    loop {
        Timer::after(Duration::from_secs(1)).await;
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.1/examples/src/bin
}
