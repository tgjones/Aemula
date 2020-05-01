use super::super::M6502;

impl M6502 {
    pub(crate) fn and(&mut self) {
        self.a = self.p.set_zero_negative_flags((self.a & self.data) as i32);
    }

    pub(crate) fn eor(&mut self) {
        self.a = self.p.set_zero_negative_flags((self.a ^ self.data) as i32);
    }

    pub(crate) fn ora(&mut self) {
        self.a = self.p.set_zero_negative_flags((self.a | self.data) as i32);
    }
}