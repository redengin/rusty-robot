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

// provide profiling macros
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

const CHANNEL: u8 = 9;
const SSID: &str = "mesh-benchmark";
const PASSWORD: &str = "mesh-benchmark-password";

fn create_wifi_config(peer_bssid: Option<[u8; 6]>) -> esp_radio::wifi::ModeConfig {
    return match peer_bssid {
        Some(bssid) => esp_radio::wifi::ModeConfig::ApSta(
            esp_radio::wifi::ClientConfig::default()
                .with_channel(CHANNEL)
                .with_ssid(SSID.into())
                .with_bssid(bssid)
                .with_auth_method(esp_radio::wifi::AuthMethod::Wpa3Personal)
                .with_password(PASSWORD.into()),
            esp_radio::wifi::AccessPointConfig::default()
                .with_channel(CHANNEL)
                .with_ssid(SSID.into())
                .with_auth_method(esp_radio::wifi::AuthMethod::Wpa3Personal)
                .with_password(PASSWORD.into()),
        ),
        None => esp_radio::wifi::ModeConfig::ApSta(
            esp_radio::wifi::ClientConfig::default(),
            esp_radio::wifi::AccessPointConfig::default()
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

    // create a heap allocator (required by esp_radio)
    // const HEAP_SIZE: usize = 98767;
    const HEAP_SIZE: usize = 48_000;
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
    let radio = esp_radio::init().unwrap();
    let radio_config = esp_radio::wifi::Config::default();
    let (mut wifi_controller, _wifi_interfaces) =
        esp_radio::wifi::new(&radio, peripherals.WIFI, radio_config).unwrap();
    wifi_controller
        .set_config(&create_wifi_config(None))
        .unwrap();
    wifi_controller
        .set_protocol(esp_radio::wifi::Protocol::P802D11LR.into())
        .unwrap();

    // benchmarking....
    info!("Starting benchmarking...");
    profile!("starting radio", wifi_controller.start().unwrap());

    // configure scanning for peers
    let scan_config = esp_radio::wifi::ScanConfig::default()
        .with_channel(CHANNEL)
        .with_ssid(SSID.into())
        .with_scan_type(esp_radio::wifi::ScanTypeConfig::Passive(
            core::time::Duration::from_millis(103),
        ));
    let mut last_peer_time = esp_hal::time::Instant::now();

    loop {
        trace!("scanning {}", esp_alloc::HEAP.stats());
        let scan_result = wifi_controller.scan_with_config(scan_config).unwrap();

        if scan_result.len() > 0 {
            // memo the time between finding a peer
            let now = esp_hal::time::Instant::now();
            trace!(
                "{} ms since last peer connection",
                (now - last_peer_time).as_millis()
            );
            last_peer_time = now;

            // connect to peers
            for peer in &scan_result {

                // display heap stats
                trace!("connecting set_config() {}", esp_alloc::HEAP.stats());

                // must reconfigure wifi controller in order to connect
                wifi_controller
                    .set_config(&create_wifi_config(Some(peer.bssid)))
                    .unwrap();

                // connect
                trace!("connecting connect() {}", esp_alloc::HEAP.stats());
                wifi_controller.connect().unwrap();

                // FIXME test connection
                // let start = esp_hal::time::Instant::now();
                // while (esp_hal::time::Instant::now() - start).as_millis() < 100
                // {
                //     if wifi_controller.is_connected().unwrap()
                //     {
                //         info!("connected [{:?}]", peer.bssid);
                //     }
                // }

                // disconnect (else next scan will panic)
                trace!("connecting disconnect() {}", esp_alloc::HEAP.stats());
                wifi_controller.disconnect().unwrap();
            }
        }
    }
}
