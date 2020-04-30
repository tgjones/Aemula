use super::super::{BrkFlags, MOS6502};

impl MOS6502 {
    pub(crate) fn brk_0(&mut self) {
        if !self.brk_flags.contains(BrkFlags::NMI | BrkFlags::IRQ) { 
            self.pc = self.pc.wrapping_add(1);
        }
        self.address_hi = 0x01;
        self.address_lo = self.sp;
        self.sp = self.sp.wrapping_sub(1);
        self.data = self.pc.hi;
        if !self.brk_flags.contains(BrkFlags::RESET) {
            self.rw = false;
        }
    }

    pub(crate) fn brk_1(&mut self) {
        self.address_hi = 0x01;
        self.address_lo = self.sp;
        self.sp = self.sp.wrapping_sub(1);
        self.data = self.pc.lo;
        if !self.brk_flags.contains(BrkFlags::RESET) {
            self.rw = false;
        }
    }

    pub(crate) fn brk_2(&mut self) {
        self.address_hi = 0x01;
        self.address_lo = self.sp;
        self.sp = self.sp.wrapping_sub(1);
        self.data = self.p.as_u8(self.brk_flags == BrkFlags::NONE); 
        self.ad.hi = 0xFF;
        if self.brk_flags.contains(BrkFlags::RESET) {
            self.ad.lo = 0xFC;
        } else {
            self.rw = false; 
            if self.brk_flags.contains(BrkFlags::NMI) {
                self.ad.lo = 0xFA;
            } else {
                self.ad.lo = 0xFE;
            }
        }
    }

    pub(crate) fn brk_3(&mut self) {
        self.address_hi = self.ad.hi;
        self.address_lo = self.ad.lo;
        self.ad.lo += 1;
        self.p.i = true; 
        self.brk_flags = BrkFlags::NONE;
    }

    pub(crate) fn brk_4(&mut self) {
        self.address_lo = self.ad.lo;
        self.ad.lo = self.data;
    }

    pub(crate) fn brk_5(&mut self) {
        self.pc.hi = self.data;
        self.pc.lo = self.ad.lo;
    }
}