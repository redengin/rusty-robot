//! https://invensense.tdk.com/products/motion-tracking/6-axis/icm-42688-p/

// based upon https://github.com/bartslinger/zeroflight/blob/main/src/drivers/icm42688p.rs

pub const REG_WHO_AM_I: u8 = 0x75;
pub const REG_PWR_MGMT0: u8 = 0x4E;
pub const REG_FIFO_CONFIG: u8 = 0x16;
pub const REG_FIFO_CONFIG1: u8 = 0x5F;
pub const REG_INTF_CONFIG0: u8 = 0x4C;
pub const REG_INTF_CONFIG1: u8 = 0x4D;
pub const REG_SIGNAL_PATH_RESET: u8 = 0x4B;
pub const REG_GYRO_CONFIG0: u8 = 0x4F;
pub const REG_ACCEL_CONFIG0: u8 = 0x50;
pub const REG_REG_BANK_SEL: u8 = 0x76;
pub const REG_GYRO_CONFIG_STATIC2: u8 = 0x0B;
pub const REG_GYRO_CONFIG_STATIC3: u8 = 0x0C;
pub const REG_GYRO_CONFIG_STATIC4: u8 = 0x0D;
pub const REG_GYRO_CONFIG_STATIC5: u8 = 0x0E;
pub const REG_ACCEL_CONFIG_STATIC2: u8 = 0x03;
pub const REG_ACCEL_CONFIG_STATIC3: u8 = 0x04;
pub const REG_ACCEL_CONFIG_STATIC4: u8 = 0x05;

pub async fn read_register<SPIBUS: embedded_hal_async::spi::SpiBus>(
    spi_dev: &mut SPIBUS,
    reg: u8,
) -> u8 {
    let mut buf = [reg | 0x80, 0x00];
    spi_dev.transfer_in_place(&mut buf).await.ok();
    buf[1]
}

pub async fn write_register<SPIBUS: embedded_hal_async::spi::SpiBus>(
    spi_dev: &mut SPIBUS,
    reg: u8,
    val: u8,
) {
    let mut buf = [reg, val];
    spi_dev.transfer_in_place(&mut buf).await.ok();
}

pub async fn init<SPIBUS: embedded_hal_async::spi::SpiBus>(
    _spi_dev: &mut SPIBUS,
) {
    // let whoami = read_register(spi_dev, REG_WHO_AM_I).await;
    // if whoami != 0x47 {
    //     defmt::error!("IMU not found");
    //     panic!("IMU not found");
    // }

    // // disable power on accel and gyro for configuration (see datasheet 12.9)
    // write_register(spi_dev, REG_PWR_MGMT0, 0x00).await;
    // Timer::after_micros(10).await;
    // write_register(spi_dev, REG_FIFO_CONFIG, 0x80).await; // FIFO_CONFIG STOP-on-full
    // Timer::after_micros(10).await;
    // write_register(spi_dev, REG_FIFO_CONFIG1, 0x07).await; // FIFO_CONFIG1 enable temp, accel, gyro
    // Timer::after_micros(10).await;
    // write_register(spi_dev, REG_INTF_CONFIG0, 0xF0).await; // big Endian, count records, hold last sample
    // Timer::after_micros(10).await;
    // write_register(spi_dev, REG_SIGNAL_PATH_RESET, 0x02).await; // flush the FIFO
    // Timer::after_micros(10).await;
    // write_register(spi_dev, REG_GYRO_CONFIG0, 0x06).await; // gyro 1kHz ODR
    // Timer::after_micros(10).await;
    // write_register(spi_dev, REG_ACCEL_CONFIG0, 0x06).await; // accel 1kHz ODR
    // Timer::after_micros(10).await;

    // write_register(spi_dev, REG_REG_BANK_SEL, 1).await;
    // Timer::after_micros(10).await;
    // let aaf_enable = read_register(spi_dev, REG_GYRO_CONFIG_STATIC2).await;
    // Timer::after_micros(10).await;
    // write_register(spi_dev, REG_GYRO_CONFIG_STATIC2, aaf_enable & !0x03).await; // enable not and AAF
    // Timer::after_micros(10).await;
    // write_register(spi_dev, REG_GYRO_CONFIG_STATIC3, 6).await; // 258Hz gyro bandwith
    // Timer::after_micros(10).await;
    // write_register(spi_dev, REG_GYRO_CONFIG_STATIC4, 36).await; // 258Hz gyro bandwith
    // Timer::after_micros(10).await;
    // write_register(
    //     spi_dev,
    //     REG_GYRO_CONFIG_STATIC5,
    //     (10 << 4) & 0xF0, /* | ((36 >> 8) & 0x0F) */
    // )
    // .await; // 258Hz gyro bandwith
    // Timer::after_micros(10).await;

    // write_register(spi_dev, REG_REG_BANK_SEL, 2).await;
    // Timer::after_micros(10).await;
    // write_register(spi_dev, REG_ACCEL_CONFIG_STATIC2, 5 << 1).await; // 213Hz accel bandwith
    // Timer::after_micros(10).await;
    // write_register(spi_dev, REG_ACCEL_CONFIG_STATIC3, 25).await; // 213Hz accel bandwith
    // Timer::after_micros(10).await;
    // write_register(
    //     spi_dev,
    //     REG_ACCEL_CONFIG_STATIC4,
    //     (10 << 4) & 0xF0, /* | ((25 >> 8) & 0x0F) */
    // )
    // .await; // 213Hz accel bandwith

    // write_register(spi_dev, REG_REG_BANK_SEL, 0).await;
    // Timer::after_micros(10).await;

    // /*
    //   From Ardupilot:
    //   fix for the "stuck gyro" issue, which affects all IxM42xxx
    //   sensors. This disables the AFSR feature which changes the
    //   noise sensitivity with angular rate. When the switch happens
    //   (at around 100 deg/sec) the gyro gets stuck for around 2ms,
    //   producing constant output which causes a DC gyro bias
    // */
    // let v = read_register(spi_dev, REG_INTF_CONFIG1).await;
    // Timer::after_micros(10).await;
    // write_register(spi_dev, REG_INTF_CONFIG1, (v & 0x3F) | 0x40).await;
    // Timer::after_micros(10).await;

    // write_register(spi_dev, REG_PWR_MGMT0, 0x0F).await; // enable power on accel and gyro
    //                                                     // min 200us sleep recommended
    // Timer::after_micros(300).await;
}

// pub async fn get_fifo_count(
//     spi_dev: &mut embedded_hal_bus::spi::ExclusiveDevice<
//         Spi<'static, Async>,
//         Output<'static>,
//         NoDelay,
//     >,
// ) -> u16 {
//     let mut buf = [0x2E | 0x80, 0x00, 0x00];
//     spi_dev.transfer_in_place(&mut buf).await.ok();
//     u16::from_be_bytes([buf[1], buf[2]])
// }

// pub async fn get_fifo_sample(
//     spi_dev: &mut embedded_hal_bus::spi::ExclusiveDevice<
//         Spi<'static, Async>,
//         Output<'static>,
//         NoDelay,
//     >,
// ) -> [u8; 17] {
//     let mut buf: [u8; 17] = [0x00; 17];
//     buf[0] = 0x30 | 0x80;
//     spi_dev.transfer_in_place(&mut buf).await.ok();
//     buf
// }
