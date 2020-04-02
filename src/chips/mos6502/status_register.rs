pub struct StatusRegister {
    /// Carry
    pub(crate) c: bool,

    /// Zero
    pub(crate) z: bool,

    /// Interrupt disable
    pub(crate) i: bool,

    /// Binary coded decimal
    pub(crate) d: bool,

    /// Overflow
    pub(crate) v: bool,

    /// Negative
    pub(crate) n: bool,
}

impl StatusRegister {
    pub(crate) fn new() -> StatusRegister {
        StatusRegister {
            c: false,
            z: false,
            i: false,
            d: false,
            v: false,
            n: false,
        }
    }

    pub(crate) fn set_from_u8(&mut self, value: u8) {
        self.c = (value & 0x01) == 0x01;
        self.z = (value & 0x02) == 0x02;
        self.i = (value & 0x04) == 0x04;
        self.d = (value & 0x08) == 0x08;
        self.v = (value & 0x40) == 0x40;
        self.n = (value & 0x80) == 0x80;
    }

    pub(crate) fn set_zero_negative_flags(&mut self, value: i32) -> u8 {
        let clamped_value = value & 0xFF;
    
        self.z = clamped_value == 0;
        self.n = (clamped_value & 0x80) == 0x80;

        clamped_value as u8
    }

    pub(crate) fn as_u8(&self, bit_4_set: bool) -> u8 {
        let mut result = 0;
        if self.c {
            result |= 0x01;
        }
        if self.z {
            result |= 0x02;
        }
        if self.i {
            result |= 0x04;
        }
        if self.d {
            result |= 0x08;
        }
        if bit_4_set { // Bit 4 is 1 if being pushed to stack by an instruction (PHP or BRK).
            result |= 0x10;
        }
        result |= 0x20; // Bit 5 is always 1.
        if self.v {
            result |= 0x40;
        }
        if self.n {
            result |= 0x80;
        }
        result
    }
}