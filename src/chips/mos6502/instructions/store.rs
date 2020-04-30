use super::super::MOS6502;

impl MOS6502 {
    pub(crate) fn sax(&mut self) {
        self.data = self.a & self.x;
        self.rw = false;
    }

    pub(crate) fn sta(&mut self) {
        self.data = self.a;
        self.rw = false;
    }

    pub(crate) fn stx(&mut self) {
        self.data = self.x;
        self.rw = false;
    }

    pub(crate) fn sty(&mut self) {
        self.data = self.y;
        self.rw = false;
    }
}