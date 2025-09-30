//! https://invensense.tdk.com/products/motion-tracking/6-axis/icm-42688-p/
//! [Datasheet](https://invensense.tdk.com/wp-content/uploads/2020/04/ds-000347_icm-42688-p-datasheet.pdf)

use log::*;

use crate::imu_traits::Vector3;

const FLAG_READ_REG: u8 = 0x80;
pub const REG_WHO_AM_I: u8 = 0x75;
pub const REG_DEVICE_CONFIG: u8 = 0x11;

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

pub const VAL_WHO_AM_I: u8 = 0x47;

pub async fn read_register<SPIDEVICE: embedded_hal_async::spi::SpiDevice>(
    spi_dev: &mut SPIDEVICE,
    reg: u8,
) -> Result<u8, <SPIDEVICE as embedded_hal_async::spi::ErrorType>::Error> {
    let mut buf = [(FLAG_READ_REG | reg), 0xff];

    return match spi_dev.transfer_in_place(&mut buf).await {
        Ok(_) => Ok(buf[1]),
        Err(e) => Err(e),
    };
}

pub async fn write_register<SPIDEVICE: embedded_hal_async::spi::SpiDevice>(
    spi_dev: &mut SPIDEVICE,
    reg: u8,
    val: u8,
) -> Result<(), <SPIDEVICE as embedded_hal_async::spi::ErrorType>::Error> {
    let mut buf = [reg, val];
    return match spi_dev.transfer_in_place(&mut buf).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
}

pub struct ICM42688<'a, SPIDEVICE: embedded_hal_async::spi::SpiDevice> {
    spi_dev: &'a mut SPIDEVICE,
    gyro_scale: GyroScale,
    accel_scale: u8,
}

/// https://invensense.tdk.com/wp-content/uploads/2020/04/ds-000347_icm-42688-p-datasheet.pdf?page=10
#[derive(Clone)]
pub enum GyroScale {
    _15_625,
    _31_25,
    _62_5,
    _125,
    _250,
    _500,
    _1000,
    _2000,
}
impl GyroScale {
    fn as_f32(&self) -> f32 {
        match self {
            GyroScale::_15_625 => 15.625,
            GyroScale::_31_25 => 31.25,
            GyroScale::_62_5 => 62.5,
            GyroScale::_125 => 125.0,
            GyroScale::_250 => 250.0,
            GyroScale::_500 => 500.0,
            GyroScale::_1000 => 1000.0,
            GyroScale::_2000 => 2000.0,
        }
    }
}

/// https://invensense.tdk.com/wp-content/uploads/2020/04/ds-000347_icm-42688-p-datasheet.pdf?page=10
pub enum AccelScale {
    _2 = 2,
    _4 = 4,
    _8 = 8,
    _16 = 16,
}

impl<SPIDEVICE: embedded_hal_async::spi::SpiDevice> ICM42688<'static, SPIDEVICE> {
    pub async fn new(spi_dev: &'static mut SPIDEVICE) -> Option<Self> {
        // verify the chip
        match read_register(spi_dev, REG_WHO_AM_I).await {
            Ok(v) => {
                if v != VAL_WHO_AM_I {
                    return None;
                }
            }
            Err(e) => {
                error!("spi failed [{:?}]", e);
                return None;
            }
        }

        // initialize chip
        // perform a software reset
        match write_register(spi_dev, REG_DEVICE_CONFIG, 0x01).await {
            Ok(_) => { /* TODO wait 1ms */ }
            Err(e) => {
                error!("spi failed [{:?}]", e);
                return None;
            }
        }
        // initialize per expected reset values
        let gyro_scale = GyroScale::_2000;
        let accel_scale = AccelScale::_16 as u8;
        /*  From Ardupilot:
            fix for the "stuck gyro" issue, which affects all IxM42xxx
            sensors. This disables the AFSR feature which changes the
            noise sensitivity with angular rate. When the switch happens
            (at around 100 deg/sec) the gyro gets stuck for around 2ms,
            producing constant output which causes a DC gyro bias
        */
        match read_register(spi_dev, REG_INTF_CONFIG1).await {
            Ok(v) => {
                let _ = write_register(spi_dev, REG_INTF_CONFIG1, 0x3f & v).await;
            }
            Err(_e) => {
                // TODO how to handle this
            }
        }

        Some(ICM42688 {
            spi_dev: spi_dev,
            gyro_scale: gyro_scale,
            accel_scale: accel_scale,
        })
    }

    pub async fn read_imu(
        self,
    ) -> Result<crate::imu_traits::ImuData, <SPIDEVICE as embedded_hal_async::spi::ErrorType>::Error>
    {
        // burst read all the data
        let mut buf: [u8; 13] = [0xff; 13];
        const REG_START: u8 = 0x1f;
        buf[0] = FLAG_READ_REG | REG_START;

        let result = self.spi_dev.transfer_in_place(&mut buf).await;
        if result.is_err() {
            return Err(result.unwrap_err());
        }

        debug!("read_imu [{:?}]", buf);

        Ok(crate::imu_traits::ImuData {
            accelerometer: Self::rawaccel_to_mps2(&self, &buf[1..6]),
            gyroscope: Self::rawgyro_to_dps(&self, &buf[7..12]),
            ..Default::default()
        })
    }
    fn rawaccel_to_mps2(&self, buf: &[u8]) -> Option<Vector3> {
        let x16 = (((buf[0] as u16) << 8) | (buf[1] as u16)) as i16;
        let y16 = (((buf[2] as u16) << 8) | (buf[3] as u16)) as i16;
        let z16 = (((buf[4] as u16) << 8) | (buf[5] as u16)) as i16;

        const MPS2_PER_G: f32 = 9.80665;
        Some(Vector3 {
            x: (x16 as f32) * (self.accel_scale as f32) * MPS2_PER_G / (i16::MAX as f32),
            y: (y16 as f32) * (self.accel_scale as f32) * MPS2_PER_G / (i16::MAX as f32),
            z: (z16 as f32) * (self.accel_scale as f32) * MPS2_PER_G / (i16::MAX as f32),
        })
    }
    fn rawgyro_to_dps(&self, buf: &[u8]) -> Option<Vector3> {
        let x16 = (((buf[0] as u16) << 8) | (buf[1] as u16)) as i16;
        let y16 = (((buf[2] as u16) << 8) | (buf[3] as u16)) as i16;
        let z16 = (((buf[4] as u16) << 8) | (buf[5] as u16)) as i16;

        Some(Vector3 {
            x: (x16 as f32) * self.gyro_scale.as_f32() / (i16::MAX as f32),
            y: (y16 as f32) * self.gyro_scale.as_f32() / (i16::MAX as f32),
            z: (z16 as f32) * self.gyro_scale.as_f32() / (i16::MAX as f32),
        })
    }
}

// pub async fn init<SPIDEVICE: embedded_hal_async::spi::SpiDevice>(
//     spi_dev: &mut SPIDEVICE,
// ) -> Result<(), <SPIDEVICE as embedded_hal_async::spi::ErrorType>::Error> {
//     // https://invensense.tdk.com/wp-content/uploads/2020/04/ds-000347_icm-42688-p-datasheet.pdf#page=77
//     match write_register(spi_dev, REG_PWR_MGMT0, 0x0F).await {
//         Ok(()) => { /* FIXME wait 200us */ }
//         Err(e) => return Err(e),
//     }

//     Ok(())
// }

// pub struct ImuData {
//     pub accelerometer: Option<crate::imu_traits::Vector3>,
//     pub gyroscope: Option<crate::imu_traits::Vector3>,
// }
// enum POWER_MODE {
//     OFF,
//     LOW_POWER,
//     LOW_NOISE,
// }

// pub async fn read_imu<SPIDEVICE: embedded_hal_async::spi::SpiDevice>(
//     spi_dev: &mut SPIDEVICE,
// ) -> Result<ImuData, <SPIDEVICE as embedded_hal_async::spi::ErrorType>::Error> {
//     // burst read all the data
//     let mut buf: [u8; 13] = [0xff; 13];
//     const REG_START: u8 = 0x1f;
//     buf[0] = FLAG_READ_REG | REG_START;

//     let result = spi_dev.transfer_in_place(&mut buf).await;
//     if result.is_err() {
//         return Err(result.unwrap_err());
//     }

//     debug!("read_imu [{:?}]", buf);

//     Ok(ImuData {
//         accelerometer: None, // FIXME
//         gyroscope: None,     // FIXME
//     })
// }

// https://invensense.tdk.com/wp-content/uploads/2020/04/ds-000347_icm-42688-p-datasheet.pdf#page=67
// fn rawtemp_to_c(msb: u8, lsb: u8) -> f32
// {
//     let val = (msb as u16) << 8 | (lsb as u16);
//     (val as f32 / 132.48) + 25.0
// }

// pub struct ICM42688<'a, SPIDEVICE: embedded_hal_async::spi::SpiDevice> {
//     spi_dev: &'a mut SPIDEVICE,
//     gyro_scale: f32,
//     accel_scale: u8,
// }

// /// https://invensense.tdk.com/wp-content/uploads/2020/04/ds-000347_icm-42688-p-datasheet.pdf?page=10
// pub enum GyroScale {
//     _15_625,
//     _31_25,
//     _62_5,
//     _125,
//     _250,
//     _500,
//     _1000,
//     _2000,
// }
// fn gyroscale_to_float(v: GyroScale) -> f32 {
//     return match v {
//         GyroScale::_15_625 => 15.625,
//         GyroScale::_31_25 => 31.25,
//         GyroScale::_62_5 => 62.5,
//         GyroScale::_125 => 125.0,
//         GyroScale::_250 => 250.0,
//         GyroScale::_500 => 500.0,
//         GyroScale::_1000 => 1000.0,
//         GyroScale::_2000 => 2000.0,
//     };
// }

// /// https://invensense.tdk.com/wp-content/uploads/2020/04/ds-000347_icm-42688-p-datasheet.pdf?page=10
// pub enum AccelScale {
//     _2 = 2,
//     _4 = 4,
//     _8 = 8,
//     _16 = 16,
// }

// impl<SPIDEVICE: embedded_hal_async::spi::SpiDevice> ICM42688<'static, SPIDEVICE> {
//     pub async fn new(spi_dev: &'static mut SPIDEVICE) -> Option<Self> {
//         // verify the chip
//         match read_register(spi_dev, REG_WHO_AM_I).await {
//             Ok(v) => {
//                 if v != VAL_WHO_AM_I {
//                     return None;
//                 }
//             }
//             Err(e) => {
//                 error!("failed connection [{:?}]", e);
//                 return None;
//             }
//         }

//         // initialize chip
//         // perform a software reset
//         match write_register(spi_dev, REG_DEVICE_CONFIG, 0x01).await {
//             Ok(_) => { /* TODO wait 1ms */ }
//             Err(e) => {
//                 error!("failed connection [{:?}]", e);
//                 return None;
//             }
//         }
//         // initialize per expected reset values
//         let gyro_scale = gyroscale_to_float(GyroScale::_2000);
//         let accel_scale = AccelScale::_16 as u8;
//         /*  From Ardupilot:
//             fix for the "stuck gyro" issue, which affects all IxM42xxx
//             sensors. This disables the AFSR feature which changes the
//             noise sensitivity with angular rate. When the switch happens
//             (at around 100 deg/sec) the gyro gets stuck for around 2ms,
//             producing constant output which causes a DC gyro bias
//         */
//         match read_register(spi_dev, REG_INTF_CONFIG1).await {
//             Ok(v) => {
//                 let _ = write_register(spi_dev, REG_INTF_CONFIG1, 0x3f & v).await;
//             },
//             Err(_e) => {
//                 // TODO how to handle this
//             }
//         }

//         Some(ICM42688 {
//             spi_dev: spi_dev,
//             gyro_scale: gyro_scale,
//             accel_scale: accel_scale,
//         })
//     }

//     pub async fn power_mode(gyro: POWER_MODE, accel: POWER_MODE)
//     {

//     }
// }

// pub async fn config_gyro<SPIBUS: embedded_hal_async::spi::SpiBus>(spi_dev: &mut SPIBUS) -> bool {
//     const BANK1: u8 = 1;
//     if write_register(spi_dev, REG_REG_BANK_SEL, BANK1)
//         .await
//         .is_err()
//     {
//         debug!("Failed to select USER BANK 1");
//         return false;
//     }
//     match read_register(spi_dev, REG_GYRO_CONFIG_STATIC2).await {
//         Ok(_v) => (),
//         Err(e) => {
//             debug!("IMU spi error: {}", e.kind());
//         }
//     }

//     true
//     // write_register(spi_dev, REG_REG_BANK_SEL, 1).await;
//     // Timer::after_micros(10).await;
//     // let aaf_enable = read_register(spi_dev, REG_GYRO_CONFIG_STATIC2).await;
//     // Timer::after_micros(10).await;
//     // write_register(spi_dev, REG_GYRO_CONFIG_STATIC2, aaf_enable & !0x03).await; // enable not and AAF
//     // Timer::after_micros(10).await;
//     // write_register(spi_dev, REG_GYRO_CONFIG_STATIC3, 6).await; // 258Hz gyro bandwith
//     // Timer::after_micros(10).await;
//     // write_register(spi_dev, REG_GYRO_CONFIG_STATIC4, 36).await; // 258Hz gyro bandwith
//     // Timer::after_micros(10).await;
//     // write_register(
//     //     spi_dev,
//     //     REG_GYRO_CONFIG_STATIC5,
//     //     (10 << 4) & 0xF0, /* | ((36 >> 8) & 0x0F) */
//     // )
//     // .await; // 258Hz gyro bandwith
//     // Timer::after_micros(10).await;
// }

// pub async fn init<SPIBUS: embedded_hal_async::spi::SpiBus>(spi_dev: &mut SPIBUS) -> bool {
//     match read_register(spi_dev, REG_WHO_AM_I).await {
//         Ok(v) => {
//             if v != VAL_WHO_AM_I {
//                 debug!("Failed to recognize IMU [id:{}]", v);
//                 return false;
//             }
//         }
//         Err(e) => {
//             debug!("IMU spi error: {}", e.kind());
//             return false;
//         }
//     }

//     // disable power on accel and gyro for configuration (see datasheet 12.9)
//     if write_register(spi_dev, REG_PWR_MGMT0, 0x00).await.is_err() {
//         debug!("Failed to disable IMU accel/gyro");
//         return false;
//     }
//     // Timer::after_micros(10).await;

//     // configure FIFO_MODE
//     {
//         const STOP_ON_FILL: u8 = 0x80;
//         if write_register(spi_dev, REG_FIFO_CONFIG, STOP_ON_FILL)
//             .await
//             .is_err()
//         {
//             debug!("Failed write to REG_FIFO_CONFIG");
//             return false;
//         }
//     }
//     // Timer::after_micros(10).await;

//     // configure FIFO writers
//     {
//         // const FIFO_RESUME_PARTIAL_RD: u8 = 1 << 6;
//         // const FIFO_WM_GT_TH: u8 = 1 << 5;
//         // const FIFO_HIRES_EN: u8 = 1 << 4;
//         // const FIFO_TMST_FSYNC_EN: u8 = 1 << 3;
//         const FIFO_TEMP_EN: u8 = 1 << 2;
//         const FIFO_GYRO_EN: u8 = 1 << 1;
//         const FIFO_ACCEL_EN: u8 = 1 << 0;
//         let fifo_config1 = FIFO_TEMP_EN | FIFO_GYRO_EN | FIFO_ACCEL_EN;
//         if write_register(spi_dev, REG_FIFO_CONFIG1, fifo_config1)
//             .await
//             .is_err()
//         {
//             debug!("Failed write to REG_FIFO_CONFIG1");
//             return false;
//         }
//     }
//     // Timer::after_micros(10).await;
//     // write_register(spi_dev, REG_INTF_CONFIG0, 0xF0).await; // big Endian, count records, hold last sample
//     // Timer::after_micros(10).await;
//     // write_register(spi_dev, REG_SIGNAL_PATH_RESET, 0x02).await; // flush the FIFO
//     // Timer::after_micros(10).await;
//     // write_register(spi_dev, REG_GYRO_CONFIG0, 0x06).await; // gyro 1kHz ODR
//     // Timer::after_micros(10).await;
//     // write_register(spi_dev, REG_ACCEL_CONFIG0, 0x06).await; // accel 1kHz ODR
//     // Timer::after_micros(10).await;

//     // write_register(spi_dev, REG_REG_BANK_SEL, 1).await;
//     // Timer::after_micros(10).await;
//     // let aaf_enable = read_register(spi_dev, REG_GYRO_CONFIG_STATIC2).await;
//     // Timer::after_micros(10).await;
//     // write_register(spi_dev, REG_GYRO_CONFIG_STATIC2, aaf_enable & !0x03).await; // enable not and AAF
//     // Timer::after_micros(10).await;
//     // write_register(spi_dev, REG_GYRO_CONFIG_STATIC3, 6).await; // 258Hz gyro bandwith
//     // Timer::after_micros(10).await;
//     // write_register(spi_dev, REG_GYRO_CONFIG_STATIC4, 36).await; // 258Hz gyro bandwith
//     // Timer::after_micros(10).await;
//     // write_register(
//     //     spi_dev,
//     //     REG_GYRO_CONFIG_STATIC5,
//     //     (10 << 4) & 0xF0, /* | ((36 >> 8) & 0x0F) */
//     // )
//     // .await; // 258Hz gyro bandwith
//     // Timer::after_micros(10).await;

//     // write_register(spi_dev, REG_REG_BANK_SEL, 2).await;
//     // Timer::after_micros(10).await;
//     // write_register(spi_dev, REG_ACCEL_CONFIG_STATIC2, 5 << 1).await; // 213Hz accel bandwith
//     // Timer::after_micros(10).await;
//     // write_register(spi_dev, REG_ACCEL_CONFIG_STATIC3, 25).await; // 213Hz accel bandwith
//     // Timer::after_micros(10).await;
//     // write_register(
//     //     spi_dev,
//     //     REG_ACCEL_CONFIG_STATIC4,
//     //     (10 << 4) & 0xF0, /* | ((25 >> 8) & 0x0F) */
//     // )
//     // .await; // 213Hz accel bandwith

//     // write_register(spi_dev, REG_REG_BANK_SEL, 0).await;
//     // Timer::after_micros(10).await;

//     // /*
//     //   From Ardupilot:
//     //   fix for the "stuck gyro" issue, which affects all IxM42xxx
//     //   sensors. This disables the AFSR feature which changes the
//     //   noise sensitivity with angular rate. When the switch happens
//     //   (at around 100 deg/sec) the gyro gets stuck for around 2ms,
//     //   producing constant output which causes a DC gyro bias
//     // */
//     // let v = read_register(spi_dev, REG_INTF_CONFIG1).await;
//     // Timer::after_micros(10).await;
//     // write_register(spi_dev, REG_INTF_CONFIG1, (v & 0x3F) | 0x40).await;
//     // Timer::after_micros(10).await;

//     // enable power on accel and gyro
//     match write_register(spi_dev, REG_PWR_MGMT0, 0x0F).await {
//         Ok(_) => {
//             // min 200us sleep recommended
//             // Timer::after_micros(300).await;
//             return true;
//         }
//         Err(e) => {
//             debug!("Failed to start IMU [spi error:{}]", e.kind());
//             return false;
//         }
//     }
// }

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
