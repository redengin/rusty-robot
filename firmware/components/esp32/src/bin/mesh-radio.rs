#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

// provide panic handler
use rusty_robot_esp32::{self as _, mesh::{self, Esp32MeshController}};

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
    trace!("initializing...");

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
    let protocols = esp_radio::wifi::Protocol::P802D11LR;
    let mesh = mesh::Esp32MeshController::new(peripherals.WIFI, protocols);

    // spawn mesh controller
    spawner.spawn(mesh_controller_task(mesh)).unwrap();
    // <Esp32MeshController as rusty_robot_drivers::radio::mesh::MeshNode>::start_ap(mesh);

    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}


/// prototype of a generic mesh controller
/// FIXME
#[embassy_executor::task]
async fn mesh_controller_task(mut mesh_controller: Esp32MeshController<'static>)
{
    // let mesh_controller = mesh as rusty_robot_drivers::radio::mesh::MeshNode;

    use rusty_robot_drivers::radio::mesh::{MeshConfig};
    let mesh_config = MeshConfig::new(
        env!("MESH_CHANNEL").parse().expect("channel must be a number [1..14]"),
        env!("MESH_SSID"),
        env!("MESH_PASSWORD")
    );

    // start the radio
    <Esp32MeshController as rusty_robot_drivers::radio::mesh::MeshNode>::start(&mut mesh_controller, &mesh_config);

    loop {
        let scan_results = <Esp32MeshController as rusty_robot_drivers::radio::mesh::MeshNode>::scan(&mut mesh_controller, &mesh_config);

        for entry in scan_results
        {
            
        }
    }

}