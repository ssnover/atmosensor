use core::fmt::Debug;
use embedded_hal::blocking::i2c::{Write, WriteRead};
use nb::Error;

pub const ADDRESS: u8 = 0x61;

pub struct Scd30 {
    addr: u8,
}

impl Scd30 {
    pub fn new() -> Self {
        Self::new_with_address(ADDRESS)
    }

    pub fn new_with_address(addr: u8) -> Self {
        Scd30 { addr }
    }

    pub fn soft_reset<I2C, E>(&self, i2c: &mut I2C) -> Result<(), E> 
    where
        I2C: WriteRead<Error = E> + Write<Error = E>,
        E: Debug,
    {
        i2c.write(self.addr, &(0xd304 as u16).to_be_bytes())
    }
}