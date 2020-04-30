use super::super::MOS6502;

impl MOS6502 {
    pub(crate) fn addressing_mode_none_cycle_0(&mut self) {
        self.set_address(self.pc);
    }
}