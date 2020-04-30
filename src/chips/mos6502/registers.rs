#[derive(Copy, Clone)]
pub struct SplitRegister16 {
    pub lo: u8,
    pub hi: u8,
}

impl SplitRegister16 {
    pub fn new() -> SplitRegister16 {
        SplitRegister16 {
            lo: 0,
            hi: 0,
        }
    }

    pub fn from_u16(value: u16) -> SplitRegister16 {
        let bytes = value.to_le_bytes();
        SplitRegister16 {
            lo: bytes[0],
            hi: bytes[1],
        }
    }

    pub fn to_u16(&self) -> u16 {
        u16::from_le_bytes([self.lo, self.hi])
    }

    pub fn wrapping_add(&self, value: u8) -> SplitRegister16 {
        let new_value = self.to_u16().wrapping_add(value as u16);
        SplitRegister16::from_u16(new_value)
    }

    pub fn wrapping_add_i8(&self, value: i8) -> SplitRegister16 {
        let new_value = (self.to_u16() as i32).wrapping_add(value as i32) as u16;
        SplitRegister16::from_u16(new_value)
    }
}