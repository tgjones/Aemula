use super::super::M6502;

impl M6502 {
    /// Read low byte of target address.
    pub(crate) fn addressing_mode_indirect_cycle_0(&mut self) {
        self.set_address(self.pc);
        self.pc = self.pc.wrapping_add(1);
    }

    /// Read high byte of target address.
    pub(crate) fn addressing_mode_indirect_cycle_1(&mut self) {
        self.set_address(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.ad.lo = self.data;
    }

    /// Read low byte of pointer stored at target address.
    pub(crate) fn addressing_mode_indirect_cycle_2(&mut self) {
        self.ad.hi = self.data;
        self.set_address(self.ad);
    }

    /// Read high byte of pointer stored at (target address + 1).
    pub(crate) fn addressing_mode_indirect_cycle_3(&mut self) {
        self.address_lo = self.ad.lo.wrapping_add(1);
        self.ad.lo = self.data;
    }

    /// Read high byte of pointer stored at (target address + 1).
    pub(crate) fn addressing_mode_indirect_cycle_4(&mut self) {
        self.address_hi = self.data;
        self.address_lo = self.ad.lo;
    }
}