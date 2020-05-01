use super::super::M6502;

impl M6502 {
    pub(crate) fn rmw_cycle(&mut self) {
        self.ad.lo = self.data;
        self.rw = false;
    }
}