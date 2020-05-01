use super::super::M6502;

impl M6502 {
    pub(crate) fn addressing_mode_indexed_indirect_x_cycle_0(&mut self) {
        self.set_address(self.pc);
        self.pc = self.pc.wrapping_add(1);
    }

    pub(crate) fn addressing_mode_indexed_indirect_x_cycle_1(&mut self) {
        self.ad.hi = 0;
        self.ad.lo = self.data;
        self.set_address(self.ad);
    }

    pub(crate) fn addressing_mode_indexed_indirect_x_cycle_2(&mut self) {
        self.ad.lo = self.ad.lo.wrapping_add(self.x);
        self.set_address(self.ad);
    }

    pub(crate) fn addressing_mode_indexed_indirect_x_cycle_3(&mut self) {
        self.address_lo = self.ad.lo.wrapping_add(1);
        self.ad.lo = self.data;
    }

    pub(crate) fn addressing_mode_indexed_indirect_x_cycle_4(&mut self) {
        self.address_hi = self.data;
        self.address_lo = self.ad.lo;
    }
}