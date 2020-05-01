use super::super::M6502;

impl M6502 {
    fn branch_0(&mut self, flag: bool, value: bool) {
        self.set_address(self.pc);
        self.ad = self.pc.wrapping_add_i8(self.data as i8);
        // If branch was not taken, fetch next instruction.
        if flag != value {
            self.fetch_next_instruction();
        }
    }

    pub(crate) fn branch_0_bcc(&mut self) {
        self.branch_0(self.p.c, false);
    }

    pub(crate) fn branch_0_bcs(&mut self) {
        self.branch_0(self.p.c, true);
    }

    pub(crate) fn branch_0_beq(&mut self) {
        self.branch_0(self.p.z, true);
    }

    pub(crate) fn branch_0_bmi(&mut self) {
        self.branch_0(self.p.n, true);
    }

    pub(crate) fn branch_0_bne(&mut self) {
        self.branch_0(self.p.z, false);
    }

    pub(crate) fn branch_0_bpl(&mut self) {
        self.branch_0(self.p.n, false);
    }

    pub(crate) fn branch_0_bvc(&mut self) {
        self.branch_0(self.p.v, false);
    }

    pub(crate) fn branch_0_bvs(&mut self) {
        self.branch_0(self.p.v, true);
    }

    /// Executed if branch was taken.
    pub(crate) fn branch_1(&mut self) {
        self.address_lo = self.ad.lo;
        if self.ad.hi == self.pc.hi { // Are we branching to the same page?
            self.pc = self.ad;
            self.fetch_next_instruction();
        }
    }

    /// Only executed if page was crossed.
    pub(crate) fn branch_2(&mut self) {
        self.pc = self.ad;
    }

    pub(crate) fn jmp(&mut self) {
        self.pc.hi = self.address_hi;
        self.pc.lo = self.address_lo;
    }
}