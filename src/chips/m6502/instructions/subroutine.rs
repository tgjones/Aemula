use super::super::M6502;

impl M6502 {
    /// Read low byte of target address.
    pub(crate) fn jsr_0(&mut self) {
        self.set_address(self.pc);
        self.pc = self.pc.wrapping_add(1);
    }

    /// Put SP on address bus.
    pub(crate) fn jsr_1(&mut self) {
        self.address_hi = 0x01;
        self.address_lo = self.sp;
        self.ad.lo = self.data;
    }

    /// Write PC high byte to stack.
    pub(crate) fn jsr_2(&mut self) {
        self.address_hi = 0x01;
        self.address_lo = self.sp;
        self.sp = self.sp.wrapping_sub(1);
        self.data = self.pc.hi;
        self.rw = false;
    }

    /// Write PC low byte to stack.
    pub(crate) fn jsr_3(&mut self) {
        self.address_hi = 0x01;
        self.address_lo = self.sp;
        self.sp = self.sp.wrapping_sub(1);
        self.data = self.pc.lo;
        self.rw = false;
    }

    /// Read high byte of target address.
    pub(crate) fn jsr_4(&mut self) {
        self.set_address(self.pc);
    }

    pub(crate) fn jsr_5(&mut self) {
        self.pc.hi = self.data;
        self.pc.lo = self.ad.lo;
    }

    pub(crate) fn rti_0(&mut self) {
        self.address_hi = 0x01;
        self.address_lo = self.sp;
        self.sp = self.sp.wrapping_add(1);
    }

    pub(crate) fn rti_1(&mut self) {
        self.address_lo = self.sp;
        self.sp = self.sp.wrapping_add(1);
    }

    pub(crate) fn rti_2(&mut self) {
        self.address_lo = self.sp;
        self.sp = self.sp.wrapping_add(1);
        self.p.set_from_u8(self.data);
    }

    pub(crate) fn rti_3(&mut self) {
        self.address_lo = self.sp;
        self.ad.lo = self.data;
    }

    pub(crate) fn rti_4(&mut self) {
        self.pc.hi = self.data;
        self.pc.lo = self.ad.lo;
    }

    pub(crate) fn rts_0(&mut self) {
        self.address_hi = 0x01;
        self.address_lo = self.sp;
        self.sp = self.sp.wrapping_add(1);
    }

    pub(crate) fn rts_1(&mut self) {
        self.address_lo = self.sp;
        self.sp = self.sp.wrapping_add(1);
    }

    pub(crate) fn rts_2(&mut self) {
        self.address_lo = self.sp;
        self.ad.lo = self.data;
    }

    pub(crate) fn rts_3(&mut self) {
        self.pc.hi = self.data;
        self.pc.lo = self.ad.lo;
        self.set_address(self.pc);
        self.pc = self.pc.wrapping_add(1);
    }
}