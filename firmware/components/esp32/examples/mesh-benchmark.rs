#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
// provide panic handler
use rusty_robot_esp32::{self as _};

// provide logging support
use log::*;

use rusty_robot_esp32::mesh;

// provide scheduler api
use embassy_time::{Duration, Timer};

// provide profiling macros
// macro_rules! profile {
//     ($label::&str ) => {
//         {
//             let start = esp_hal::time::Instant::now();
//             let r = $expression;
//             let end = esp_hal::time::Instant::now();
//             debug!("{} took {} ms", $label, (end - start).as_millis());
//             r
//         }
//     };
// }
macro_rules! profile {
    ($label:tt, $expression:expr) => {{
        let start = esp_hal::time::Instant::now();
        let r = $expression;
        let end = esp_hal::time::Instant::now();
        trace!("{} took {} ms", $label, (end - start).as_millis());
        r
    }};
    ($label:tt, $block:block) => {{
        let start = esp_hal::time::Instant::now();
        let r = $block;
        let end = esp_hal::time::Instant::now();
        trace!("{} took {} ms", $label, (end - start).as_millis());
        r
    }};
}

#[esp_rtos::main]
async fn main(_spawner: embassy_executor::Spawner) -> ! {
    // initialize the logger
    esp_println::logger::init_logger(LevelFilter::Trace);

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
    let mut mesh = mesh::new(peripherals.WIFI);

    // benchmarking....
    info!("Starting benchmarking...");
    profile!("starting radio", mesh.wifi_controller.start().unwrap());

    // configure scanning for peers
    let mut last_peer_time = esp_hal::time::Instant::now();
    let scan_config = esp_radio::wifi::ScanConfig::default()
        .with_channel(
            env!("AP_CHANNEL")
                .parse()
                .expect("failed to parse AP_CHANNEL"),
        )
        .with_ssid(env!("AP_SSID").into());

    loop {
        // let scan_result = profile!(
        //     "wifi scan",
        //     mesh.wifi_controller.scan_with_config(scan_config).unwrap()
        // );
        let scan_result = mesh.wifi_controller.scan_with_config(scan_config).unwrap();

        if scan_result.len() > 0 {
            // memo the time between finding a peer
            let now = esp_hal::time::Instant::now();
            info!(
                "{} ms since last peer connection",
                (now - last_peer_time).as_millis()
            );
            last_peer_time = now;

            info!("found {} peers", scan_result.len());
            for peer in scan_result {
                // must reconfigure wifi controller in order to connect
                mesh.wifi_controller
                    .set_config(&esp_radio::wifi::ModeConfig::ApSta(
                        // STA configuration
                        esp_radio::wifi::ClientConfig::default()
                            .with_channel(
                                env!("AP_CHANNEL")
                                    .parse()
                                    .expect("failed to parse AP_CHANNEL"),
                            )
                            .with_ssid(env!("AP_SSID").into())
                            .with_bssid(peer.bssid)
                            .with_auth_method(esp_radio::wifi::AuthMethod::Wpa2Personal)
                            .with_password(env!("AP_PASSWORD").into()),
                        // AP configuration
                        esp_radio::wifi::AccessPointConfig::default()
                            .with_channel(
                                env!("AP_CHANNEL")
                                    .parse()
                                    .expect("failed to parse AP_CHANNEL"),
                            )
                            .with_ssid(env!("AP_SSID").into())
                            .with_auth_method(esp_radio::wifi::AuthMethod::Wpa2Personal)
                            .with_password(env!("AP_PASSWORD").into()),
                    ))
                    .expect("Failed to reconfigure wifi");

                // connect
                mesh.wifi_controller.connect().unwrap();

                // disconnect
                mesh.wifi_controller.disconnect().unwrap();
            }
        }

        // Timer::after(Duration::from_secs(1)).await;
    }
}
