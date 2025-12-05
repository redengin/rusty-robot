#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
// provide panic handler
use rusty_robot_esp32::{self as _, *};

// provide logging support
use log::*;

// provide scheduling primitives
// use embassy_time::{Duration, Timer};

const CHANNEL: u8 = 9;
const SSID: &str = "mesh-benchmark";
const PASSWORD: &str = "mesh-benchmark-password";

fn create_wifi_modeconfig(peer_bssid: Option<[u8; 6]>) -> esp_radio::wifi::ModeConfig {
    return match peer_bssid {
        Some(bssid) => esp_radio::wifi::ModeConfig::AccessPointStation(
            esp_radio::wifi::sta::StationConfig::default()
                .with_channel(CHANNEL)
                .with_ssid(SSID.into())
                .with_bssid(bssid)
                .with_auth_method(esp_radio::wifi::AuthMethod::Wpa3Personal)
                .with_password(PASSWORD.into()),
            esp_radio::wifi::ap::AccessPointConfig::default()
                .with_channel(CHANNEL)
                .with_ssid(SSID.into())
                .with_auth_method(esp_radio::wifi::AuthMethod::Wpa3Personal)
                .with_password(PASSWORD.into()),
        ),
        None => esp_radio::wifi::ModeConfig::AccessPointStation(
            esp_radio::wifi::sta::StationConfig::default(),
            esp_radio::wifi::ap::AccessPointConfig::default()
                .with_channel(CHANNEL)
                .with_ssid(SSID.into())
                .with_auth_method(esp_radio::wifi::AuthMethod::Wpa3Personal)
                .with_password(PASSWORD.into()),
        ),
    };
}

#[esp_rtos::main]
async fn main(_spawner: embassy_executor::Spawner) -> ! {
    // initialize the logger
    esp_println::logger::init_logger(LevelFilter::Trace);
    trace!("initializing...");

    // create a heap (alloc required for esp_radio)
    create_heap!();

    // initialize the SoC
    use esp_hal::clock::CpuClock;
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // initialize embassy scheduler
    use esp_hal::timer::timg::TimerGroup;
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    use esp_hal::interrupt::software::SoftwareInterruptControl;
    let sw_int = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_int.software_interrupt0);

    // initialize the radio for WiFi
    let wifi_config = esp_radio::wifi::Config::default().with_country_code(country_code_from_env());
    let (mut wifi_controller, _wifi_interfaces) =
        esp_radio::wifi::new(peripherals.WIFI, wifi_config).unwrap();

    // configure radio protocols (must set_config() before set_protocol())
    wifi_controller
        .set_config(&create_wifi_modeconfig(None))
        .unwrap();
    wifi_controller
        .set_protocol(esp_radio::wifi::Protocol::P802D11LR.into())
        .unwrap();

    // benchmarking....
    info!("Starting benchmarking...");
    wifi_controller.start().unwrap();

    // configure scanning for peers
    let scan_config = esp_radio::wifi::scan::ScanConfig::default()
        .with_ssid(SSID.into())
        .with_channel(CHANNEL)
        .with_scan_type(esp_radio::wifi::scan::ScanTypeConfig::Passive(
            // 103ms is the common beacon period
            esp_hal::time::Duration::from_millis(103),
        ));

    let mut last_peer_time = esp_hal::time::Instant::now();

    loop {
        trace!("loop {}", esp_alloc::HEAP.stats());
        // NOTE: non-async scan will leak memory
        let scan_result = wifi_controller
            .scan_with_config_async(scan_config)
            .await
            .unwrap();

        if scan_result.len() > 0 {
            // memo the time between finding a peer
            let now = esp_hal::time::Instant::now();
            info!(
                "{} ms since last peer connection",
                (now - last_peer_time).as_millis()
            );
            last_peer_time = now;

            // connect to peers
            for peer in &scan_result {
                // must reconfigure wifi controller in order to connect
                if wifi_controller
                    .set_config(&create_wifi_modeconfig(Some(peer.bssid)))
                    .is_err()
                {
                    warn!("unable to configure STA bssid");
                    continue
                }

                // connect
                // FIXME connect_async delays too long
                // match wifi_controller.connect_async().await {
                match wifi_controller.connect() {
                    Ok(_) => {
                        debug!("connection successful [{:?}]", peer.bssid);
                        wifi_controller.disconnect().unwrap();
                    }
                    Err(_) => {
                        debug!("connection failed");
                    }
                }
            }
        }
    }
}
