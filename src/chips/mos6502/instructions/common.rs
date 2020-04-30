use super::super::MOS6502;

impl MOS6502 {
    pub(crate) fn rmw_cycle(&mut self) {
        self.ad.lo = self.data;
        self.rw = false;
    }
}