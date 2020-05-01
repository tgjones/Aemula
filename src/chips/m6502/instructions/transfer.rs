use super::super::M6502;

impl M6502 {
    pub(crate) fn tax(&mut self) {
        self.x = self.p.set_zero_negative_flags(self.a);
    }

    pub(crate) fn tay(&mut self) {
        self.y = self.p.set_zero_negative_flags(self.a);
    }

    pub(crate) fn tsx(&mut self) {
        self.x = self.p.set_zero_negative_flags(self.sp);
    }

    pub(crate) fn txa(&mut self) {
        self.a = self.p.set_zero_negative_flags(self.x);
    }

    pub(crate) fn txs(&mut self) {
        self.sp = self.x;
    }

    pub(crate) fn tya(&mut self) {
        self.a = self.p.set_zero_negative_flags(self.y);
    }
}