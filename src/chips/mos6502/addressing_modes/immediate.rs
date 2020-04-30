use super::super::MOS6502;

impl MOS6502 {
    pub(crate) fn addressing_mode_immediate_cycle_0(&mut self) {
        self.set_address(self.pc);
        self.pc = self.pc.wrapping_add(1);
    }
}