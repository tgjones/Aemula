use super::registers::{Registers, SplitRegister16};

#[derive(Copy, Clone)]
pub struct Pins {
    pub address_lo: u8,
    pub address_hi: u8,
    pub data: u8,
    pub rdy: bool,
    pub irq: bool,
    pub nmi: bool,
    pub sync: bool,
    pub res: bool,
    pub rw: bool,
}

impl Pins {
    pub fn new() -> Pins {
        Pins {
            address_lo: 0,
            address_hi: 0,
            data: 0,
            rdy: false,
            irq: false,
            nmi: false,
            sync: false,
            res: false,
            rw: false,
        }
    }

    pub fn get_address(&self) -> u16 {
        u16::from_le_bytes([self.address_lo, self.address_hi])
    }

    pub fn set_address(&mut self, register: &SplitRegister16) {
        self.address_lo = register.lo;
        self.address_hi = register.hi;
    }

    pub fn fetch_next_instruction(&mut self, r: &Registers) {
        self.set_address(&r.pc);
        self.sync = true;
    }
}