use super::super::M6502;

impl M6502 {
    pub(crate) fn clc(&mut self) {
        self.p.c = false;
    }

    pub(crate) fn cld(&mut self) {
        self.p.d = false;
    }

    pub(crate) fn cli(&mut self) {
        self.p.i = false;
    }

    pub(crate) fn clv(&mut self) {
        self.p.v = false;
    }

    pub(crate) fn sed(&mut self) {
        self.p.d = true;
    }

    pub(crate) fn sei(&mut self) {
        self.p.i = true;
    }

    pub(crate) fn slc(&mut self) {
        self.p.c = true;
    }
}