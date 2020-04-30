use super::super::MOS6502;

impl MOS6502 {
    pub(crate) fn addressing_mode_indirect_indexed_y_cycle_0(&mut self) {
        self.set_address(self.pc);
        self.pc = self.pc.wrapping_add(1);
    }

    pub(crate) fn addressing_mode_indirect_indexed_y_cycle_1(&mut self) {
        self.ad.hi = 0;
        self.ad.lo = self.data;
        self.set_address(self.ad);
    }

    pub(crate) fn addressing_mode_indirect_indexed_y_cycle_2(&mut self) {
        self.address_lo = self.ad.lo.wrapping_add(1);
        self.ad.lo = self.data;
    }

    pub(crate) fn addressing_mode_indirect_indexed_y_cycle_3(&mut self) {
        self.ad.hi = self.data;
        self.address_hi = self.ad.hi;
        self.address_lo = self.ad.lo.wrapping_add(self.y);
    }

    pub(crate) fn addressing_mode_indirect_indexed_y_cycle_3_read(&mut self) {
        let without_carry = self.ad.hi;
        let with_carry = self.ad.wrapping_add(self.y).hi;
        if without_carry == with_carry {
            self.tr += 1;
        }
    }

    /// This cycle can be skipped for read access if page boundary is not crossed.
    pub(crate) fn addressing_mode_indirect_indexed_y_cycle_4(&mut self) {
        self.set_address(self.ad.wrapping_add(self.y));
    }
}