use super::m6502::{M6502, M6502Options};

pub struct M6507 {
    inner: M6502,
}

impl M6507 {
    pub fn new() -> Self {
        Self {
            inner: M6502::new_with_options(M6502Options {
                bcd_enabled: true
            })
        }
    }

    pub fn set_pin_res(&mut self, value: bool) {
        self.inner.set_res(value);
    }

    pub fn set_pin_rdy(&mut self, value: bool) {
        self.inner.set_rdy(value);
    }

    pub fn pin_address(&self) -> u16 {
        self.inner.get_address()
    }

    pub fn pin_data(&self) -> u8 {
        self.inner.data()
    }

    pub fn set_pin_data(&mut self, value: u8) {
        self.inner.set_data(value);
    }

    pub fn pin_phi2(&self) -> bool {
        self.inner.phi2()
    }

    pub fn set_pin_phi0(&mut self, value: bool) {
        self.inner.set_phi0(value);
    }

    pub fn pin_rw(&self) -> bool {
        self.inner.rw
    }
}