#![no_std]
#![no_main]

// Environment Variables (config.toml)
const WIFI_CHANNEL: &str = env!("WIFI_CHANNEL");
const WIFI_PASSWORD: &str = env!("WIFI_PASSWORD");

// provide panic handler
use rusty_robot_esp32::{self as _};

use log::*;
use rusty_robot::mk_static;

use embassy_time::{Duration, Timer};

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
    use embassy_net::Ipv4Cidr;
    let address = Ipv4Cidr::new("192.168.9.1".parse().unwrap(), 24);
    let rng = esp_hal::rng::Rng::new();
    use embassy_net::{StackResources, StaticConfigV4};
    let (network_stack, runner) = embassy_net::new(
        wifi_interfaces.access_point,
        embassy_net::Config::ipv4_static(StaticConfigV4 {
            address: address,
            gateway: None,
            dns_servers: Default::default(),
        }),
        mk_static!(StackResources<3>, StackResources::<3>::new()),
        rng.random() as u64, // provide a random seed (TODO security implications)
    );
    network_stack.config_v4().unwrap();

    // start the network stack
    spawner.spawn(net_task(runner)).unwrap();

    // configure the AP
    let ap_config = esp_radio::wifi::ap::AccessPointConfig::default()
        .with_ssid("robot <MAC>".into())
        .with_channel(WIFI_CHANNEL.parse().unwrap())
        // .with_auth_method(esp_radio::wifi::AuthMethod::Wpa3Personal)
        .with_auth_method(esp_radio::wifi::AuthMethod::Wpa)
        .with_password(WIFI_PASSWORD.into());
    wifi_controller
        .set_config(&esp_radio::wifi::ModeConfig::AccessPoint(ap_config))
        .unwrap();

    // start the AP
    wifi_controller.start().unwrap();

    // FIXME
    // provide a hello-world web service
    spawner.spawn(hello_task(network_stack)).unwrap();

    loop {
        // TODO perform health checks
        // in the meantime sleep so the scheduler will run
        Timer::after(Duration::from_secs(1000)).await;
    }
}

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, esp_radio::wifi::WifiDevice<'static>>) {
    runner.run().await
}

#[embassy_executor::task]
async fn hello_task(network_stack: embassy_net::Stack<'static>) -> ! {
    // create socket buffers
    let mut rx_buffer = [0; 1536];
    let mut tx_buffer = [0; 1536];

    // create a tcp socket
    use embassy_net::tcp::TcpSocket;
    let mut socket = TcpSocket::new(network_stack, &mut rx_buffer, &mut tx_buffer);
    socket.set_timeout(Some(embassy_time::Duration::from_secs(10)));

    // respond to socket requests
    loop {
        use embassy_net::IpListenEndpoint;
        let r = socket
            .accept(IpListenEndpoint {
                addr: None,
                port: 80,
            })
            .await;

        if let Err(_e) = r {
            continue;
        }

        use embedded_io_async::Write;
        let r = socket
            .write_all(
                b"HTTP/1.0 200 OK\r\n\r\n\
            <html>\
                <body>\
                    <h1>Hello Rust! Hello esp-radio!</h1>\
                </body>\
            </html>\r\n\
            ",
            )
            .await;
        if let Err(_e) = r {
            // println!("write error: {:?}", e);
        }

        let r = socket.flush().await;
        if let Err(_e) = r {
            // println!("flush error: {:?}", e);
        }
        Timer::after(Duration::from_millis(1000)).await;

        socket.close();
        Timer::after(Duration::from_millis(1000)).await;

        socket.abort();
    }
}
