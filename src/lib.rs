//! This is a platform agnostic Rust driver for the AT24CXXX serial EEPROM,
//! based on the [`embedded-hal`](https://github.com/japaric/embedded-hal) traits.

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

extern crate embedded_hal as hal;

use hal::blocking::i2c::{Write, WriteRead};

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2c(E),
    /// Too much data passed for a write
    TooMuchData
}

/// AT24CXXX driver
#[derive(Debug, Default)]
pub struct At24cxxx<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    /// The I²C device address.
    address: u8
}

impl<I2C, E> At24cxxx<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    /// Create a new instance
    pub fn new(i2c: I2C, address: u8) -> Self {
        At24cxxx {
            i2c,
            address
        }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Write a single byte into an address.
    pub fn write_byte(&mut self, address: &[u8; 2], data: u8) -> Result<(), Error<E>> {
        let payload = [address[0], address[1], data];
        self.i2c
            .write(self.address, &payload)
            .map_err(Error::I2c)
    }

    /// Write up to a page (64 bytes)
    pub fn write_page(&mut self, address: &[u8; 2], data: &[u8]) -> Result<(), Error<E>> {
        const PAGE_SIZE : usize = 64;
        if data.len() > PAGE_SIZE {
            return Err(Error::TooMuchData);
        }
        let mut payload : [u8; 2 + PAGE_SIZE] = [0; 2 + PAGE_SIZE];
        payload[0] = address[0];
        payload[1] = address[1];
        payload[2..=(1+data.len())].copy_from_slice(&data);
        self.i2c
            .write(self.address, &payload[..=(1 + data.len())])
            .map_err(Error::I2c)
    }

    /// Read a single byte from an address.
    pub fn read_byte(&mut self, address: &[u8; 2]) -> Result<u8, Error<E>> {
        let mut data = [0; 1];
        self.i2c
            .write_read(self.address, &[address[0], address[1]], &mut data)
            .map_err(Error::I2c).and(Ok(data[0]))
    }
}


#[cfg(test)]
mod tests {
    extern crate embedded_hal_mock as hal;

    use super::*;

    const DEVICE_ADDRESS : u8 = 0x50;

    fn setup<'a>() -> At24cxxx<hal::I2cMock<'a>> {
        let mut dev = hal::I2cMock::new();
        dev.set_read_data(&[0xAB]);
        At24cxxx::new(dev, DEVICE_ADDRESS)
    }

    #[test]
    fn sends_correct_parameters_for_byte_read() {
        let mut eeprom = setup();
        let address = [0x12, 0x34];
        eeprom.read_byte(&address).unwrap();
        let dev = eeprom.destroy();
        assert_eq!(dev.get_last_address(), Some(DEVICE_ADDRESS));
        assert_eq!(dev.get_write_data(), &address);
    }

    #[test]
    fn can_read_byte() {
        let mut eeprom = setup();
        let data = eeprom.read_byte(&[0, 0]).unwrap();
        assert_eq!(data, 0xAB);
    }

    #[test]
    fn sends_correct_parameters_for_byte_write() {
        let mut eeprom = setup();
        let address = [0x12, 0x34];
        let data = 0xCD;
        eeprom.write_byte(&address, data).unwrap();
        let dev = eeprom.destroy();
        assert_eq!(dev.get_last_address(), Some(DEVICE_ADDRESS));
        assert_eq!(dev.get_write_data(), &[address[0], address[1], data]);
    }

    #[test]
    fn cannot_write_too_big_page() {
        let mut eeprom = setup();
        match eeprom.write_page(&[0, 0], &[0; 65]) {
            Err(Error::TooMuchData) => {},
            _ => panic!("Error::TooMuchData not returned.")
        }
    }

    #[test]
    fn sends_correct_parameters_for_page_write() {
        let mut eeprom = setup();
        let address = [0x12, 0x34];
        let data = [0xCD];
        eeprom.write_page(&address, &data).unwrap();
        let dev = eeprom.destroy();
        assert_eq!(dev.get_last_address(), Some(DEVICE_ADDRESS));
        assert_eq!(dev.get_write_data(), &[address[0], address[1], data[0]]);
    }

    #[test]
    fn can_write_page() {
        // cannot write a full page with 64 bytes because the write buffer
        // in embedded-hal-mock is limited to 64 bytes and the address
        // is prepended to the data.
        let mut eeprom = setup();
        let address = [0x12, 0x34];
        let data = [0xCD; 62];
        eeprom.write_page(&address, &data).unwrap();
        let dev = eeprom.destroy();

        assert_eq!(dev.get_last_address(), Some(DEVICE_ADDRESS));
        let mut payload = [0xCD; 64];
        payload[0] = address[0];
        payload[1] = address[1];
        assert_eq!(dev.get_write_data(), &payload[..]);
    }
}
