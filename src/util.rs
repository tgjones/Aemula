pub(crate) trait Bit {
    fn bit(&self, n: u8) -> bool;
}

impl Bit for u8 {
    #[inline]
    fn bit(&self, n: u8) -> bool {
        self & (1 << n) != 0
    }
}

impl Bit for u16 {
    #[inline]
    fn bit(&self, n: u8) -> bool {
        self & (1 << n) != 0
    }
}