#![no_std]

use log::*;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // display location
    if let Some(location) = info.location() {
        error!(
            "PANIC at {} {}:{}",
            location.file(),
            location.line(),
            location.column()
        );
    }
    // display message
    error!("{}", info.message());

    loop {
        // wait for logging message to publish
        let delay = esp_hal::delay::Delay::new();
        delay.delay_millis(1000);

        // if release build, reset to resume mission
        #[cfg(not(debug_assertions))]
        esp_hal::system::software_reset()
    }
}

#[macro_export]
macro_rules! create_heap {
    () => {
        const BOOTLOADER_RAM_SZ: usize = 64 * 1024;
        esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: BOOTLOADER_RAM_SZ);
    }
    // ($size:literal) => {
    //     const BOOTLOADER_RAM_SZ: usize = 64 * 1024;
    //     esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: BOOTLOADER_RAM_SZ);

    //     // FIXME rust doesn't allow comprehension
    //     // if $size > BOOTLOADER_RAM_SZ {
    //     //     const more: usize = $size - BOOTLOADER_RAM_SZ;
    //     //     // esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: $size - BOOTLOADER_RAM_SZ);
    //     // }
    // }
}

/// convert config.toml env string to country code (two chars)
pub fn country_code_from_env() -> [u8; 2]
{
    let country_bytes = env!("ESP_WIFI_CONFIG_COUNTRY_CODE").as_bytes();
    return [country_bytes[0], country_bytes[1]];
}

// provide profiling macros
#[macro_export]
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

// FIXME
// pub mod mesh;
