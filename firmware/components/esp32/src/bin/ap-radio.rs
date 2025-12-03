#![no_std]
#![no_main]

use core::net::Ipv4Addr;

// provide panic handler
use rusty_robot_esp32::{self as _};

use log::*;
use rusty_robot::mk_static;

use embassy_time::{Timer, Duration};

// Environment Variables

#[esp_rtos::main]
async fn main(spawner: embassy_executor::Spawner) -> ! {
    // initialize logging
    esp_println::logger::init_logger_from_env();
    trace!("initializing...");

    // create a heap allocator (required by esp_radio)
    // const HEAP_SIZE: usize = 98767;
    // esp_alloc::heap_allocator!(#[unsafe(link_section = ".dram2_uninit")] size: HEAP_SIZE);
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 64 * 1024);
    esp_alloc::heap_allocator!(size: 36 * 1024);

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
    let (mut wifi_controller, wifi_interfaces) =
        esp_radio::wifi::new(peripherals.WIFI, Default::default()).unwrap();

    // initialize network stack
    use embassy_net::{Ipv4Cidr, StackResources, StaticConfigV4};
    let address = Ipv4Cidr::new("192.168.9.1".parse().unwrap(), 24);
    let rng = esp_hal::rng::Rng::new();
    let (stack, runner) = embassy_net::new(
        wifi_interfaces.access_point,
        embassy_net::Config::ipv4_static(StaticConfigV4 {
            address: address,
            gateway: None,
            dns_servers: Default::default(),
        }),
        // config,
        mk_static!(StackResources<3>, StackResources::<3>::new()),
        rng.random() as u64, // provide a random seed
                             //  TODO security implications
    );
    stack.config_v4().unwrap();

    // start the network
    spawner.spawn(net_task(runner)).unwrap();

    // configure the AP
    let ap_config = esp_radio::wifi::ModeConfig::default();
    wifi_controller.set_config(&ap_config);

    loop {}
}

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, esp_radio::wifi::WifiDevice<'static>>) {
    runner.run().await
}

