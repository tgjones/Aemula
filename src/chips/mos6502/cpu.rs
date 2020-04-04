use super::registers::{BrkFlags, Registers};
use super::pins::Pins;

use super::addressing_modes::*;
use super::instructions::*;

pub struct MOS6502 {
    registers: Registers,
    last_pins: Pins,
}

pub struct MOS6502Options {
    bcd_enabled: bool,
}

impl MOS6502 {
    pub fn new() -> (MOS6502, Pins) {
        let options = MOS6502Options {
            bcd_enabled: true,
        };
        MOS6502::new_with_options(options)
    }

    pub fn new_with_options(options: MOS6502Options) -> (MOS6502, Pins) {
        let mut pins = Pins::new();
        pins.sync = true;
        pins.res = true;
        pins.rw = true;

        let mos6502 = MOS6502 {
            registers: Registers::new(options.bcd_enabled),
            last_pins: pins
        };

        (mos6502, pins)
    }

    pub fn cycle(&mut self, mut pins: Pins) -> Pins {
        // If SYNC pin is set, this is the start of a new instruction.
        // We will have the new opcode in the DATA pins.
        if pins.sync {
            self.registers.ir = pins.data;
            self.registers.tr = 0;
            pins.sync = false;

            if pins.res {
                self.registers.brk_flags = BrkFlags::RESET;
            }

            if self.registers.brk_flags != BrkFlags::NONE {
                self.registers.ir = 0;
                pins.res = false;
            } else {
                self.registers.pc = self.registers.pc.wrapping_add(1);
            }
        }

        // Assume we're going to read.
        pins.rw = true;

        // Include generated file with actual instruction implementations.
        include!(concat!(env!("OUT_DIR"), "/mos6502_instructions.generated.rs"));

        // Increment timing register.
        self.registers.tr += 1;

        // Store pins state.
        self.last_pins = pins;

        pins
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env, fs, fs::File, io::Write, path::Path};
    use file_diff::diff;

    const ASSET_PATH:&'static str = "test_assets/chips/mos6502";

    #[test]
    fn all_suite_a() {
        let path = Path::new(ASSET_PATH).join("AllSuiteA.bin");
        let rom = fs::read(path).unwrap();

        let mut ram = [0; 0x4000];

        let (mut cpu, mut pins) = MOS6502::new();

        while cpu.registers.pc.to_u16() != 0x45C2 {
            pins = cpu.cycle(pins);
            let address = pins.get_address();

            if pins.rw {
                pins.data = match address {
                    0x0000..=0x3FFF => ram[address as usize],
                    0x4000..=0xFFFF => rom[(address - 0x4000) as usize],
                }
            } else {
                match address {
                    0x0000..=0x3FFF => ram[address as usize] = pins.data,
                    _ => {},
                }
            }
        }

        assert_eq!(0xFF, ram[0x0210]);
    }

    #[test]
    fn dormann_functional_test() {
        let path = Path::new(ASSET_PATH).join("6502_functional_test.bin");
        let mut ram = fs::read(path).unwrap();
        assert_eq!(0x10000, ram.len());

        // Patch the test start address into the RESET vector.
        ram[0xFFFC] = 0x00;
        ram[0xFFFD] = 0x04;

        let (mut cpu, mut pins) = MOS6502::new();

        while cpu.registers.pc.to_u16() != 0x3399 && cpu.registers.pc.to_u16() != 0xD0FE {
            pins = cpu.cycle(pins);
            let address = pins.get_address();

            if pins.rw {
                pins.data = ram[address as usize];
            } else {
                ram[address as usize] = pins.data;
            }
        }

        assert_eq!(0x3399, cpu.registers.pc.to_u16());
    }

    #[test]
    fn nes_test() -> Result<(), std::io::Error> {
        let path = Path::new(ASSET_PATH).join("nestest.nes");
        let mut cartridge_bytes = fs::read(path).unwrap();
        let rom = &mut cartridge_bytes[16..(16+0x4000)];

        // Patch the test start address into the RESET vector.
        rom[0x3FFC] = 0x00;
        rom[0x3FFD] = 0xC0;

        let mut ram = [0; 0x0800];

        // APU and I/O registers - for the purposes of this test, treat them as RAM.
        let mut apu = [0; 0x18];

        let options = MOS6502Options {
            bcd_enabled: false,
        };
        let (mut cpu, mut pins) = MOS6502::new_with_options(options);

        let test_log_path = env::temp_dir().join("nestest_aemula.log");
        let mut test_log_buffer = File::create(&test_log_path)?;

        let mut cycles = 0;
        let mut should_log = false;
        while cpu.registers.pc.to_u16() != 0xC66E {
            pins = cpu.cycle(pins);

            cycles += 1;

            if cycles == 7 {
                should_log = true;
            }

            if should_log && pins.sync {
                write!(test_log_buffer,
                    "{:04X}  A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} CPUC:{}\n", 
                    cpu.registers.pc.to_u16(),
                    cpu.registers.a,
                    cpu.registers.x,
                    cpu.registers.y,
                    cpu.registers.p.as_u8(false),
                    cpu.registers.sp,
                    cycles - 7)?;
            }

            let address = pins.get_address();

            if should_log {
                write!(test_log_buffer, "      ")?;
            }
            if pins.rw {
                pins.data = match address {
                    0x0000..=0x1FFF => ram[(address & 0x07FF) as usize],
                    0x4000..=0x4017 => apu[(address - 0x4000) as usize],
                    0x8000..=0xFFFF => rom[((address - 0x8000) & 0x3FFF) as usize],
                    _ => 0,
                };
                if should_log {
                    write!(test_log_buffer, "READ      ${:04X} => ${:02X}\n", address, pins.data)?;
                }
            } else {
                match address {
                    0x0000..=0x1FFF => ram[(address & 0x07FF) as usize] = pins.data,
                    0x4000..=0x4017 => apu[(address - 0x4000) as usize] = pins.data,
                    _ => {},
                }
                if should_log {
                    write!(test_log_buffer, "WRITE     ${:04X} <= ${:02X}\n", address, pins.data)?;
                }
            }
        }

        assert_eq!(0x00, ram[0x0002]);
        assert_eq!(0x00, ram[0x0003]);

        diff("assets/nestest.log", test_log_path.to_str().unwrap());

        Ok(())
    }
}