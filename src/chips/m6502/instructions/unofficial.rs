use super::super::M6502;

impl M6502 {
    pub(crate) fn anc(&mut self) {
        self.a &= self.data;
        self.p.set_zero_negative_flags(self.a);
        self.p.c = self.a & 0x80 != 0;
    }

    pub(crate) fn ane(&mut self) {
        self.a = (self.a | 0xEE) & self.x & self.data;
        self.p.set_zero_negative_flags(self.a);
    }

    pub(crate) fn arr(&mut self) {
        self.and();
        
        // http://www.6502.org/users/andre/petindex/local/64doc.txt
        if self.bcd_enabled && self.p.d {
            // Do ROR.
            let mut a = self.a >> 1;

            // Add carry flag to MSB.
            if self.p.c {
                a |= 0x80;
            }

            // Set zero and negative flags as normal.
            self.p.set_zero_negative_flags(a);

            // The V flag will be set if the bit 6 of the accumulator changed its state
            // between the AND and the ROR, cleared otherwise.
            self.p.v = ((self.a ^ a) & 0x40) != 0;

            // Now it gets weird: if low nibble is greater than 5, increment it by 6,
            // but if it carries, don't add it to high nibble.
            if self.a & 0xF >= 5 {
                a = (a.wrapping_add(6) & 0xF) | (a & 0xF0);
            }

            // If high nibble is greater than 5, increment it by 6,
            // and set the carry flag.
            if (self.a & 0xF0) >= 0x50 {
                a = a.wrapping_add(0x60);
                self.p.c = true;
            } else {
                self.p.c = false;
            }

            self.a = a;
        } else {
            self.rora();

            // The C flag is copied from bit 6 of the result.
            self.p.c = self.a & 0x40 != 0;

            // The V flag is the result of an XOR operation between bit 6 and bit 5 of the result.
            self.p.v = (((self.a & 0x40) >> 6) ^ ((self.a & 0x20) >> 5)) == 1;
        }
    }

    pub(crate) fn asr(&mut self) {
        self.and();
        self.lsra();
    }

    pub(crate) fn dcp(&mut self) {
        self.dec();
        self.cmp();
    }

    pub(crate) fn isb(&mut self) {
        self.inc();
        self.sbc();
    }

    pub(crate) fn las(&mut self) {
        self.a = self.data & self.sp;
        self.x = self.a;
        self.sp = self.a;
        self.p.set_zero_negative_flags(self.a);
    }

    pub(crate) fn lxa(&mut self) {
        self.a = (self.a | 0xEE) & self.data;
        self.x = self.a;
        self.p.set_zero_negative_flags(self.a);
    }

    pub(crate) fn rla(&mut self) {
        self.rol();
        self.and();
    }

    pub(crate) fn rra(&mut self) {
        self.ror();
        self.adc();
    }

    pub(crate) fn sbx(&mut self) {
        let (new_x, overflowed) = (self.a & self.x).overflowing_sub(self.data);
        self.x = new_x;
        self.p.c = !overflowed;
        self.p.set_zero_negative_flags(self.x);
    }

    pub(crate) fn sha(&mut self) {
        self.data = self.a & self.x & self.address_hi.wrapping_add(1);
        self.rw = false;
    }

    pub(crate) fn shs(&mut self) {
        self.sp = self.a & self.x;
        self.sha();
    }

    pub(crate) fn shx(&mut self) {
        self.data = self.x & self.address_hi.wrapping_add(1);
        self.rw = false;
    }

    pub(crate) fn shy(&mut self) {
        self.data = self.y & self.address_hi.wrapping_add(1);
        self.rw = false;
    }

    pub(crate) fn slo(&mut self) {
        self.asl();
        self.ora();
    }

    pub(crate) fn sre(&mut self) {
        self.lsr();
        self.eor();
    }
}