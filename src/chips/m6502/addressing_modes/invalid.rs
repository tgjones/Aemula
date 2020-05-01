use super::super::M6502;

impl M6502 {
    pub(crate) fn addressing_mode_invalid_cycle_0(&mut self) {
        self.set_address(self.pc);
    }

    pub(crate) fn addressing_mode_invalid_cycle_1(&mut self) {
        self.address_hi = 0xFF;
        self.address_lo = 0xFF;
        self.data = 0xFF;
        self.ir -= 1;
    }
}