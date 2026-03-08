#![no_std]

use crc16::{CCITT_FALSE, State};
use embedded_hal::{delay::DelayNs, i2c::I2c};
use esp_hal::delay::Delay;
use registers::*;

#[derive(Debug, Copy, Clone)]
pub struct CY8CMBR3108<I2C> {
    i2c: I2C,
    address: u8,
}

pub const CY8CMBR3108_I2C_ADDR: u8 = 0x37;
pub const CY8CMBR3108_FAMILY_ID: u8 = 0x9A;
pub const CY8CMBR3108_CONFIG_SIZE: usize = 128;
pub const CY8CMBR3XXX_CTRL_CMD_CALC_CRC: u8 = 0x02;
pub const CY8CMBR3XXX_CTRL_CMD_RESET: u8 = 0xFF;

pub mod registers {
    pub const CY8CMBR3XXX_SENSOR_EN: u8 = 0x00F;
    pub const CY8CMBR3XXX_FAMILY_ID: u8 = 0x8F;
    pub const CY8CMBR3XXX_DEVICE_ID: u8 = 0x90;
    pub const CY8CMBR3XXX_DEVICE_REV: u8 = 0x92;
    pub const CY8CMBR3XXX_BUTTON_STAT: u8 = 0xAA;
    pub const CY8CMBR3XXX_CTRL_CMD: u8 = 0x86;
    pub const CY8CMBR3XXX_CTRL_CMD_STATUS: u8 = 0x88;
    pub const CY8CMBR3XXX_CTRL_CMD_ERR: u8 = 0x89;
}

pub const DEFAULT_CONFIG: [u8; 128] = [
    0b1100_1100,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    128,
    128,
    128,
    128,
    128,
    128,
    128,
    128,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    3,
    12,
    0,
    50,
    51,
    51,
    0,
    0,
    0,
    0,
    0,
    128,
    5,
    0,
    0,
    2,
    0,
    2,
    0,
    0,
    5,
    0,
    50,
    20,
    20,
    30,
    30,
    0,
    0,
    30,
    30,
    0,
    0,
    0,
    1,
    1,
    0,
    15,
    15,
    15,
    15,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    84,
    3,
    1,
    8,
    0,
    55,
    6,
    0,
    0,
    10,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    44,
    177,
];

impl<I2C: I2c> CY8CMBR3108<I2C> {
    /// Create new builder with a default I2C address of 0x0F
    #[allow(clippy::new_ret_no_self)]
    pub fn new(i2c: I2C) -> Self {
        CY8CMBR3108 {
            i2c,
            address: CY8CMBR3108_I2C_ADDR,
        }
    }

    pub fn init(&mut self) -> Result<bool, I2C::Error> {
        Ok(true)
    }

    pub fn ready(&mut self) -> Result<bool, I2C::Error> {
        for i in 0..3 {
            let output = self.get_family_id();
            match output {
                Ok(family_id) => {
                    if family_id == CY8CMBR3108_FAMILY_ID {
                        return Ok(true);
                    }
                }
                Err(_) => continue,
            }
        }
        Ok(false)
    }

    pub fn write_configuration(&mut self) -> Result<bool, I2C::Error> {
        let mut state = State::<CCITT_FALSE>::new();
        state.update(&DEFAULT_CONFIG[0..126]);
        let new_crc = state.get();

        let mut config = DEFAULT_CONFIG;
       
        config[0x7e] = (new_crc & 0xFF) as u8; // CRC low byte
        config[0x7f] = (new_crc >> 8) as u8; // CRC high byte

        self.write_all(0x00, config)?;
        self.update_configuration(
            registers::CY8CMBR3XXX_CTRL_CMD,
            CY8CMBR3XXX_CTRL_CMD_CALC_CRC,
        )?;
       Delay::new().delay_millis(500);
     
       
        self.update_configuration(registers::CY8CMBR3XXX_CTRL_CMD, CY8CMBR3XXX_CTRL_CMD_RESET)
            .unwrap();

       Delay::new().delay_millis(100);

       for i in 0..5 {
        let is_device_ready = self.ready();
        match is_device_ready {
            Ok(true) => {
                break;
            }
            Ok(false) => {
            }
            Err(e) => {},
        }
    }

        Ok(true)
    }

    pub fn get_family_id(&mut self) -> Result<u8, I2C::Error> {
        let family_id = self.read_configuration_1_byte(CY8CMBR3XXX_FAMILY_ID)?;
        Ok(family_id[0])
    }

    pub fn get_device_id(&mut self) -> Result<u16, I2C::Error> {
        self.read_configuration_2_bytes(CY8CMBR3XXX_DEVICE_ID)
    }

    pub fn get_device_revision(&mut self) -> Result<u16, I2C::Error> {
        self.read_configuration_2_bytes(CY8CMBR3XXX_DEVICE_REV)
    }

    pub fn get_button_status(&mut self) -> Result<u16, I2C::Error> {
        self.read_configuration_2_bytes(CY8CMBR3XXX_BUTTON_STAT)
    }

    pub fn read_all(&mut self) -> Result<[u8; CY8CMBR3108_CONFIG_SIZE], I2C::Error> {
        let mut read_buffer: [u8; CY8CMBR3108_CONFIG_SIZE] = [0; CY8CMBR3108_CONFIG_SIZE];
        self.i2c
            .write_read(self.address, &[0x00], &mut read_buffer)?;
        Ok(read_buffer)
    }

    fn write_all(
        &mut self,
        register: u8,
        config: [u8; CY8CMBR3108_CONFIG_SIZE],
    ) -> Result<bool, I2C::Error> {
        let mut data = [0u8; CY8CMBR3108_CONFIG_SIZE + 1];
        data[0] = register; // First byte is the register address
        data[1..].copy_from_slice(&config); // Copy config data after the register

        self.i2c.write(self.address, &data)?;
        Ok(true)
    }

    fn read_configuration_2_bytes(&mut self, register: u8) -> Result<u16, I2C::Error> {
        let mut read_buffer: [u8; 2] = [0; 2];
        self.i2c
            .write_read(self.address, &[register], &mut read_buffer)?;
        Ok(u16::from_ne_bytes([read_buffer[0], read_buffer[1]]))
    }
    fn read_configuration_1_byte(&mut self, register: u8) -> Result<[u8; 1], I2C::Error> {
        let mut read_buffer: [u8; 1] = [0; 1];
        self.i2c
            .write_read(self.address, &[register], &mut read_buffer)?;
        Ok(read_buffer)
    }
    fn update_configuration(&mut self, register: u8, data: u8) -> Result<bool, I2C::Error> {
        self.i2c.write(self.address, &[register, data])?;
        Ok(true)
    }
}
