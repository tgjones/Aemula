use super::super::MOS6502;

impl MOS6502 {
    pub(crate) fn addressing_mode_zero_page_cycle_0(&mut self) {
        self.set_address(self.pc);
        self.pc = self.pc.wrapping_add(1);
    }

    pub(crate) fn addressing_mode_zero_page_cycle_1(&mut self) {
        self.address_hi = 0;
        self.address_lo = self.data;
    }
}