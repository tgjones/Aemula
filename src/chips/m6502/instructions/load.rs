use super::super::M6502;

impl M6502 {
    pub(crate) fn lax(&mut self) {
        self.lda();
        self.ldx();
    }

    pub(crate) fn lda(&mut self) {
        self.a = self.p.set_zero_negative_flags(self.data as i32);
    }

    pub(crate) fn ldx(&mut self) {
        self.x = self.p.set_zero_negative_flags(self.data as i32);
    }

    pub(crate) fn ldy(&mut self) {
        self.y = self.p.set_zero_negative_flags(self.data as i32);
    }
}