//! https://invensense.tdk.com/products/motion-tracking/6-axis/icm-42688-p/
//! [Datasheet](https://invensense.tdk.com/wp-content/uploads/2020/04/ds-000347_icm-42688-p-datasheet.pdf)

use log::*;

use crate::imu::Vector3;

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

/// https://invensense.tdk.com/wp-content/uploads/2020/04/ds-000347_icm-42688-p-datasheet.pdf?page=77
pub enum PowerMode {
    Sleep = 0,          // disables GYROSCOPE and ACCELEROMETER
    Enabled = 0b1111,   // LN GYROSCOPE, LN ACCELEROMETER
}

impl<'a, SPIDEVICE: embedded_hal_async::spi::SpiDevice> ICM42688<'a, SPIDEVICE> {
    /// software reset the chip to defaults
    pub async fn new(spi_dev: &'a mut SPIDEVICE) -> Result<Self, &'a str> {
        // verify the chip
        match read_register(spi_dev, REG_WHO_AM_I).await {
            Ok(v) => {
                if v != VAL_WHO_AM_I {
                    return Err("invalid chip id");
                }
            }
            Err(e) => {
                error!("spi failed [{:?}]", e);
                return Err("spi bus failed");
            }
        }

        // initialize chip
        // perform a software reset
        match write_register(spi_dev, REG_DEVICE_CONFIG, 0x01).await {
            Ok(_) => { /* TODO wait 1ms */ }
            Err(e) => {
                error!("spi failed [{:?}]", e);
                return Err("spi bus failed");
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

        Ok(ICM42688 {
            spi_dev: spi_dev,
            gyro_scale: gyro_scale,
            accel_scale: accel_scale,
        })
    }

    pub async fn set_power_mode(
        &mut self,
        mode: PowerMode,
    ) -> Result<(), <SPIDEVICE as embedded_hal_async::spi::ErrorType>::Error> {
        return match write_register(self.spi_dev, REG_PWR_MGMT0, mode as u8).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn read_imu(
        &mut self,
    ) -> Result<crate::imu::ImuData, <SPIDEVICE as embedded_hal_async::spi::ErrorType>::Error>
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

        Ok(crate::imu::ImuData {
            accelerometer: Self::rawaccel_to_mps2(&self, &buf[1..7]),
            gyroscope: Self::rawgyro_to_dps(&self, &buf[7..13]),
            ..Default::default()
        })
    }
    fn bytes_to_i16(msb: u8, lsb: u8) -> i16 {
        (((msb as u16) << 8) | (lsb as u16)) as i16
    }
    fn rawaccel_to_mps2(&self, buf: &[u8]) -> Option<Vector3> {
        let x16 = Self::bytes_to_i16(buf[0], buf[1]);
        let y16 = Self::bytes_to_i16(buf[2], buf[3]);
        let z16 = Self::bytes_to_i16(buf[4], buf[5]);

        const MPS2_PER_G: f32 = 9.80665;
        Some(Vector3 {
            x: (x16 as f32) * (self.accel_scale as f32) * MPS2_PER_G / (i16::MAX as f32),
            y: (y16 as f32) * (self.accel_scale as f32) * MPS2_PER_G / (i16::MAX as f32),
            z: (z16 as f32) * (self.accel_scale as f32) * MPS2_PER_G / (i16::MAX as f32),
        })
    }
    fn rawgyro_to_dps(&self, buf: &[u8]) -> Option<Vector3> {
        let x16 = Self::bytes_to_i16(buf[0], buf[1]);
        let y16 = Self::bytes_to_i16(buf[2], buf[3]);
        let z16 = Self::bytes_to_i16(buf[4], buf[5]);

        Some(Vector3 {
            x: (x16 as f32) * self.gyro_scale.as_f32() / (i16::MAX as f32),
            y: (y16 as f32) * self.gyro_scale.as_f32() / (i16::MAX as f32),
            z: (z16 as f32) * self.gyro_scale.as_f32() / (i16::MAX as f32),
        })
    }
}

