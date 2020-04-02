use super::status_register::StatusRegister;

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

bitflags! {
    pub struct BrkFlags: u8 {
        const NONE = 0;
        const IRQ = 1;
        const NMI = 2;
        const RESET = 4;
    }
}

pub struct Registers {
    /// Accumulator
    pub a: u8,

    /// X index register
    pub x: u8,

    /// Y index register
    pub y: u8,

    // Program counter
    pub pc: SplitRegister16,

    /// Stack pointer
    pub sp: u8,

    /// Processor status
    pub p: StatusRegister,

    /// Instruction register - stores opcode of instruction being executed.
    pub ir: u8,

    /// Timing register - stores the progress through the current instruction, from 0 to 7
    pub tr: u8,

    pub brk_flags: BrkFlags,
    pub ad: SplitRegister16,

    pub bcd_enabled: bool,
}

impl Registers {
    pub fn new(bcd_enabled: bool) -> Registers {
        Registers {
            a: 0,
            x: 0,
            y: 0,

            pc: SplitRegister16::new(),

            sp: 0,

            p: StatusRegister::new(),

            ir: 0, // Set to 0 to start with a BRK instruction
            tr: 0,
            brk_flags: BrkFlags::NONE,
            ad: SplitRegister16::new(),

            bcd_enabled,
        }
    }
}