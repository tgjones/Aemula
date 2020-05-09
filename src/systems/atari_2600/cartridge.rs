use aemula_macros::PinAccessors;
use crate::util::Bit;

pub trait Cartridge {
    fn set_pin_a(&mut self, value: u16);
    fn set_pin_d(&mut self, value: u8);
    fn pin_d(&self) -> u8;
}

impl dyn Cartridge {
    pub fn from_data(data: Vec<u8>) -> Box<dyn Cartridge> {
        match data.len() {
            2048 => Box::new(Cartridge2K::new(data)),
            4096 => Box::new(Cartridge4K::new(data)),
            _ => panic!("Unknown cartridge type")
        }
    }
}

#[derive(PinAccessors)]
pub struct Cartridge2K {
    /// Address pins A0..A10 are used to address the ROM memory.
    /// Address pin A12 is used as a chip select.
    #[pin(in)]
    #[handle(change)]
    pin_a: u16,

    #[pin(bidirectional)]
    pin_d: u8,

    rom_data: Vec<u8>
}

impl Cartridge2K {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            pin_a: 0,
            pin_d: 0,

            rom_data: data
        }
    }

    fn on_pin_a_change(&mut self) {
        if self.pin_a.bit(12) {
            self.pin_d = self.rom_data[(self.pin_a & 0x7FF) as usize];
        }
    }
}

impl Cartridge for Cartridge2K {
    fn set_pin_a(&mut self, value: u16) {
        Cartridge2K::set_pin_a(self, value);
    }

    fn set_pin_d(&mut self, value: u8) {
        Cartridge2K::set_pin_d(self, value);
    }

    fn pin_d(&self) -> u8 {
        Cartridge2K::pin_d(self)
    }
}

#[derive(PinAccessors)]
pub struct Cartridge4K {
    /// Address pins A0..A11 are used to address the ROM memory.
    /// Address pin A12 is used as a chip select.
    #[pin(in)]
    #[handle(change)]
    pin_a: u16,

    #[pin(bidirectional)]
    pin_d: u8,

    rom_data: Vec<u8>
}

impl Cartridge4K {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            pin_a: 0,
            pin_d: 0,

            rom_data: data
        }
    }

    fn on_pin_a_change(&mut self) {
        if self.pin_a.bit(12) {
            self.pin_d = self.rom_data[(self.pin_a & 0xFFF) as usize];
        }
    }
}

impl Cartridge for Cartridge4K {
    fn set_pin_a(&mut self, value: u16) {
        Cartridge4K::set_pin_a(self, value);
    }

    fn set_pin_d(&mut self, value: u8) {
        Cartridge4K::set_pin_d(self, value);
    }

    fn pin_d(&self) -> u8 {
        Cartridge4K::pin_d(self)
    }
}