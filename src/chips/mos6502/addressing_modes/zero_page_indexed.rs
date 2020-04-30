use super::super::MOS6502;

impl MOS6502 {
    pub(crate) fn addressing_mode_zero_page_indexed_cycle_0(&mut self) {
        self.set_address(self.pc);
        self.pc = self.pc.wrapping_add(1);
    }

    pub(crate) fn addressing_mode_zero_page_indexed_cycle_1(&mut self) {
        self.ad.hi = 0;
        self.ad.lo = self.data;
        self.set_address(self.ad);
    }

    fn addressing_mode_zero_page_indexed_cycle_2(&mut self, index_register_value: u8) {
        self.address_hi = 0;
        self.address_lo = self.ad.lo.wrapping_add(index_register_value);
    }

    pub(crate) fn addressing_mode_zero_page_x_cycle_2(&mut self) {
        self.addressing_mode_zero_page_indexed_cycle_2(self.x);
    }

    pub(crate) fn addressing_mode_zero_page_y_cycle_2(&mut self) {
        self.addressing_mode_zero_page_indexed_cycle_2(self.y);
    }
}