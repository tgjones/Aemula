use super::super::M6502;

impl M6502 {
    /// Set address bus to PC (to fetch BAL, low byte of base address), increment PC.
    pub(crate) fn addressing_mode_absolute_indexed_cycle_0(&mut self) {
        self.set_address(self.pc);
        self.pc = self.pc.wrapping_add(1);
    }

    /// Set address bus to PC (to fetch BAH, high byte of base address), increment PC.
    pub(crate) fn addressing_mode_absolute_indexed_cycle_1(&mut self) {
        self.set_address(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.ad.lo = self.data;
    }

    /// Set address bus to BAH,BAL+index
    pub(crate) fn addressing_mode_absolute_indexed_cycle_2(&mut self, index_register_value: u8) {
        self.ad.hi = self.data;
        self.address_hi = self.ad.hi;
        self.address_lo = self.ad.lo.wrapping_add(index_register_value);
    }

    /// If, when the index register is added to BAL (the low byte of the base address),
    /// the resulting address is on the same page, then we skip the next cycle.
    /// 
    /// Otherwise if it's on the next page, then we execute an extra cycle
    /// to add the carry value to BAH (the high byte of the base address).
    /// 
    /// This conditional check only happens for instructions that read memory.
    /// For instructions that write to memory, we always execute the extra cycle.
    pub(crate) fn addressing_mode_absolute_indexed_cycle_2_read(&mut self, index_register_value: u8) {
        let without_carry = self.ad.hi;
        let with_carry = self.ad.wrapping_add(index_register_value).hi;
        if without_carry == with_carry {
            self.tr += 1;
        }
    }

    pub(crate) fn addressing_mode_absolute_indexed_cycle_3(&mut self, index_register_value: u8) {
        self.set_address(self.ad.wrapping_add(index_register_value));
    }
}