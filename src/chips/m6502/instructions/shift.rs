use super::super::M6502;

impl M6502 {
    fn asl_helper(&mut self, value: u8) -> u8 {
        self.p.c = (value & 0x80) == 0x80;
        self.p.set_zero_negative_flags((value as i32) << 1)
    }

    pub(crate) fn asl(&mut self) {
        self.data = self.asl_helper(self.ad.lo);
        self.rw = false;
    }

    pub(crate) fn asla(&mut self) {
        self.a = self.asl_helper(self.a);
    }

    fn lsr_helper(&mut self, value: u8) -> u8 {
        self.p.c = (value & 0x1) == 0x1;
        self.p.set_zero_negative_flags((value as i32) >> 1)
    }

    pub(crate) fn lsr(&mut self) {
        self.data = self.lsr_helper(self.ad.lo);
        self.rw = false;
    }

    pub(crate) fn lsra(&mut self) {
        self.a = self.lsr_helper(self.a);
    }

    fn rol_helper(&mut self, value: u8) -> u8 {
        let temp = if self.p.c { 1 } else { 0 };
        self.p.c = (value & 0x80) == 0x80;
        self.p.set_zero_negative_flags(((value << 1) | temp) as i32)
    }

    pub(crate) fn rol(&mut self) {
        self.data = self.rol_helper(self.ad.lo);
        self.rw = false;
    }

    pub(crate) fn rola(&mut self) {
        self.a = self.rol_helper(self.a);
    }

    fn ror_helper(&mut self, value: u8) -> u8 {
        let temp = (if self.p.c { 1 } else { 0 }) << 7;
        self.p.c = (value & 0x1) == 0x1;
        self.p.set_zero_negative_flags(((value >> 1) | temp) as i32)
    }

    pub(crate) fn ror(&mut self) {
        self.data = self.ror_helper(self.ad.lo);
        self.rw = false;
    }

    pub(crate) fn rora(&mut self) {
        self.a = self.ror_helper(self.a);
    }
}