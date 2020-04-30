use super::super::MOS6502;

impl MOS6502 {
    pub(crate) fn bit(&mut self) {
        let value = self.data;
        self.p.z = (self.a & value) == 0;
        self.p.v = (value & 0x40) == 0x40;
        self.p.n = (value & 0x80) == 0x80;
    }

    fn compare(&mut self, register: u8) {
        self.p.set_zero_negative_flags(register as i32 - self.data as i32);
        self.p.c = register >= self.data;
    }

    pub(crate) fn cmp(&mut self) {
        self.compare(self.a);
    }

    pub(crate) fn cpx(&mut self) {
        self.compare(self.x);
    }

    pub(crate) fn cpy(&mut self) {
        self.compare(self.y);
    }
}