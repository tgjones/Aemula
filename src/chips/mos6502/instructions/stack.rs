use super::super::MOS6502;

impl MOS6502 {
    pub(crate) fn pha(&mut self) {
        self.address_hi = 0x01;
        self.address_lo = self.sp;
        self.sp = self.sp.wrapping_sub(1);
        self.data = self.a;
        self.rw = false;
    }

    pub(crate) fn php(&mut self) {
        self.address_hi = 0x01;
        self.address_lo = self.sp;
        self.sp = self.sp.wrapping_sub(1);
        self.data = self.p.as_u8(true);
        self.rw = false;
    }

    pub(crate) fn pla_0(&mut self) {
        self.address_hi = 0x01;
        self.address_lo = self.sp;
        self.sp = self.sp.wrapping_add(1);
    }

    pub(crate) fn pla_1(&mut self) {
        self.address_hi = 0x01;
        self.address_lo = self.sp;
    }

    pub(crate) fn pla_2(&mut self) {
        self.a = self.p.set_zero_negative_flags(self.data as i32);
    }

    pub(crate) fn plp_0(&mut self) {
        self.address_hi = 0x01;
        self.address_lo = self.sp;
        self.sp = self.sp.wrapping_add(1);
    }

    pub(crate) fn plp_1(&mut self) {
        self.address_hi = 0x01;
        self.address_lo = self.sp;
    }

    pub(crate) fn plp_2(&mut self) {
        let temp = self.p.set_zero_negative_flags(self.data as i32);
        self.p.set_from_u8(temp);
    }
}