mod registers;
mod status_register;
mod addressing_modes;
mod instructions;

use aemula_macros::PinAccessors;

use self::registers::SplitRegister16;
use self::status_register::StatusRegister;

bitflags! {
    pub struct BrkFlags: u8 {
        const NONE = 0;
        const IRQ = 1;
        const NMI = 2;
        const RESET = 4;
    }
}

#[derive(PinAccessors)]
pub struct M6502 {
    //////////////////////////////////////////
    // Pins
    //////////////////////////////////////////

    pub address_lo: u8,
    pub address_hi: u8,

    #[pin(bidirectional)]
    data: u8,

    pub rdy: bool,
    pub irq: bool,
    pub nmi: bool,

    #[pin(out)]
    sync: bool,

    pub res: bool,

    #[pin(in)]
    #[handle(transition_lo_to_hi, transition_hi_to_lo)]
    phi0: bool,

    /// When PHI1 is high, external devices can read from the address bus or data bus.
    #[pin(out)]
    phi1: bool,

    /// When PHI2 is high, external devices can write to the data bus.
    #[pin(out)]
    phi2: bool,

    /// Read/write (read = true, write = false)
    pub rw: bool,

    //////////////////////////////////////////
    // Registers
    //////////////////////////////////////////

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
    ir: u8,

    /// Timing register - stores the progress through the current instruction, from 0 to 7
    tr: u8,

    //////////////////////////////////////////
    // Other internal storage
    //////////////////////////////////////////

    brk_flags: BrkFlags,
    ad: SplitRegister16,

    bcd_enabled: bool,
}

pub struct M6502Options {
    pub bcd_enabled: bool,
}

impl M6502 {
    pub fn new() -> Self {
        let options = M6502Options {
            bcd_enabled: true,
        };
        M6502::new_with_options(options)
    }

    pub fn new_with_options(options: M6502Options) -> Self {
        Self {
            address_lo: 0,
            address_hi: 0,
            data: 0,
            rdy: false,
            irq: false,
            nmi: false,
            sync: true,
            res: true,
            rw: true,

            phi0: false,
            phi1: false,
            phi2: false,

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

            bcd_enabled: options.bcd_enabled,
        }
    }

    pub fn get_address(&self) -> u16 {
        u16::from_le_bytes([self.address_lo, self.address_hi])
    }

    pub fn set_address(&mut self, register: SplitRegister16) {
        self.address_lo = register.lo;
        self.address_hi = register.hi;
    }

    pub fn fetch_next_instruction(&mut self) {
        self.set_address(self.pc);
        self.sync = true;
    }

    // How this is actually supposed to work is:
    // - PHI1 is when address lines change.
    // - PHI2 is when the data is transferred.

    fn on_phi0_transition_lo_to_hi(&mut self) {
        // TODO: Write to the data bus.



        self.phi2 = self.phi0;
        self.phi1 = !self.phi0;
    }

    fn on_phi0_transition_hi_to_lo(&mut self) {
        // TODO: Read previously requested data from bus.
        // TODO: Put address onto the bus.
        // TODO: Set RW pin.
        // TODO: Set SYNC pin for an opcode fetch.

        // If SYNC pin is set, this is the start of a new instruction.
        // We will have the new opcode in the DATA pins.
        if self.sync {
            self.ir = self.data;
            self.tr = 0;
            self.sync = false;

            if self.res {
                self.brk_flags = BrkFlags::RESET;
            }

            if self.brk_flags != BrkFlags::NONE {
                self.ir = 0;
                self.res = false;
            } else {
                self.pc = self.pc.wrapping_add(1);
            }
        }

        // Assume we're going to read.
        self.rw = true;

        // Include generated file with actual instruction implementations.
        include!(concat!(env!("OUT_DIR"), "/mos6502_instructions.generated.rs"));

        // Increment timing register.
        self.tr += 1;

        self.phi2 = self.phi0;
        self.phi1 = !self.phi0;
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

        let mut cpu = M6502::new();

        while cpu.pc.to_u16() != 0x45C2 {
            cpu.set_phi0(true);
            cpu.set_phi0(false);

            let address = cpu.get_address();

            if cpu.rw {
                cpu.data = match address {
                    0x0000..=0x3FFF => ram[address as usize],
                    0x4000..=0xFFFF => rom[(address - 0x4000) as usize],
                }
            } else {
                match address {
                    0x0000..=0x3FFF => ram[address as usize] = cpu.data,
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

        let mut cpu = M6502::new();

        while cpu.pc.to_u16() != 0x3399 && cpu.pc.to_u16() != 0xD0FE {
            cpu.set_phi0(true);
            cpu.set_phi0(false);

            let address = cpu.get_address();

            if cpu.rw {
                cpu.data = ram[address as usize];
            } else {
                ram[address as usize] = cpu.data;
            }
        }

        assert_eq!(0x3399, cpu.pc.to_u16());
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

        let options = M6502Options {
            bcd_enabled: false,
        };
        let mut cpu = M6502::new_with_options(options);

        let test_log_path = env::temp_dir().join("nestest_aemula.log");
        let mut test_log_buffer = File::create(&test_log_path)?;

        let mut cycles = 0;
        let mut should_log = false;
        while cpu.pc.to_u16() != 0xC66E {
            cpu.set_phi0(true);
            cpu.set_phi0(false);

            cycles += 1;

            if cycles == 7 {
                should_log = true;
            }

            if should_log && cpu.sync {
                write!(test_log_buffer,
                    "{:04X}  A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} CPUC:{}\n", 
                    cpu.pc.to_u16(),
                    cpu.a,
                    cpu.x,
                    cpu.y,
                    cpu.p.as_u8(false),
                    cpu.sp,
                    cycles - 7)?;
            }

            let address = cpu.get_address();

            if should_log {
                write!(test_log_buffer, "      ")?;
            }
            if cpu.rw {
                cpu.data = match address {
                    0x0000..=0x1FFF => ram[(address & 0x07FF) as usize],
                    0x4000..=0x4017 => apu[(address - 0x4000) as usize],
                    0x8000..=0xFFFF => rom[((address - 0x8000) & 0x3FFF) as usize],
                    _ => 0,
                };
                if should_log {
                    write!(test_log_buffer, "READ      ${:04X} => ${:02X}\n", address, cpu.data)?;
                }
            } else {
                match address {
                    0x0000..=0x1FFF => ram[(address & 0x07FF) as usize] = cpu.data,
                    0x4000..=0x4017 => apu[(address - 0x4000) as usize] = cpu.data,
                    _ => {},
                }
                if should_log {
                    write!(test_log_buffer, "WRITE     ${:04X} <= ${:02X}\n", address, cpu.data)?;
                }
            }
        }

        assert_eq!(0x00, ram[0x0002]);
        assert_eq!(0x00, ram[0x0003]);

        diff("assets/nestest.log", test_log_path.to_str().unwrap());

        Ok(())
    }

    #[test]
    fn c64_suite() {
        fn petscii_to_ascii(character: u8) -> String {
            match character {
                147 => "\n------------\n".to_string(), // Clear
                14 => "".to_string(), // Toggle lowercase/uppercase character set
                0xC1..=0xDA => ((character - 0xC1 + 65) as char).to_string(),
                0x41..=0x5A => ((character - 0x41 + 97) as char).to_string(),
                _ => (character as char).to_string()
            }
        }

        fn setup_test(filename: &String, ram: &mut [u8; 0x10000], cpu: &mut M6502) {
            // Reset RAM.
            for i in ram.iter_mut() {
                *i = 0x00;
            }

            // Load test data.
            // First two bytes contain starting address.
            let path = Path::new(ASSET_PATH).join("c64_test_suite/bin").join(filename);
            let test_data = fs::read(path).unwrap();
            let start_address = (test_data[0] as usize) | ((test_data[1] as usize) << 8);
            for i in 2..test_data.len() {
                ram[start_address + i - 2] = test_data[i];
            }

            // Initialize some memory locations.
            ram[0x0002] = 0x00;
            ram[0xA002] = 0x00;
            ram[0xA003] = 0x80;
            ram[0xFFFE] = 0x48;
            ram[0xFFFF] = 0xFF;
            ram[0x01FE] = 0xFF;
            ram[0x01FF] = 0x7F;

            // Install KERNAL "IRQ handler".
            static IRQ_ROUTINE: [u8; 19] = [
                0x48,             // PHA
                0x8A,             // TXA
                0x48,             // PHA
                0x98,             // TYA
                0x48,             // PHA
                0xBA,             // TSX
                0xBD, 0x04, 0x01, // LDA $0104,X
                0x29, 0x10,       // AND #$10
                0xF0, 0x03,       // BEQ $FF58
                0x6C, 0x16, 0x03, // JMP ($0316)
                0x6C, 0x14, 0x03, // JMP ($0314)
            ];
            for i in 0..IRQ_ROUTINE.len() {
                ram[0xFF48 + i] = IRQ_ROUTINE[i];
            }

            // Stub CHROUT routine.
            ram[0xFFD2] = 0x60; // RTS

            // Stub load routine.
            ram[0xE16F] = 0xEA; // NOP

            // Stub GETIN routine.
            ram[0xFFE4] = 0xA9; // LDA #3
            ram[0xFFE5] = 0x03;
            ram[0xFFE6] = 0x60; // RTS

            // Initialize registers.
            cpu.sp = 0xFD;
            cpu.p.i = true;
            
            // Initialize RESET vector.
            ram[0xFFFC] = 0x01;
            ram[0xFFFD] = 0x08;
        }

        let mut ram: [u8; 0x10000] = [0; 0x10000];
        let mut log = String::new();
        let mut test_filename = " start".to_string();

        loop {
            let mut cpu = M6502::new();
            setup_test(&test_filename, &mut ram, &mut cpu);

            loop {
                cpu.set_phi0(true);
                cpu.set_phi0(false);

                let address = cpu.get_address();

                if cpu.rw {
                    match address {
                        0xFFD2 => { // Print character
                            if cpu.a == 13 {
                                println!("{}", log);
                                log.clear();
                            } else {
                                log += &petscii_to_ascii(cpu.a);
                            }
                            ram[0x030C] = 0x00;
                        },
                        0xE16F => { // Load
                            let filename_address = ram[0xBB] as usize | ((ram[0xBC] as usize) << 8);
                            let filename_len = ram[0xB7] as usize;
                            test_filename.clear();
                            for i in 0..filename_len {
                                test_filename += &petscii_to_ascii(ram[filename_address + i]);
                            }
                            if test_filename == "trap17" {
                                // All tests passed. Everything from trap17 onwards is C64-specific.
                                return;
                            }
                            break; // Break to outer loop, and load next test.
                        },
                        0x8000 | 0xA474 => { // Exit
                            println!("{}", log);
                            panic!("Test failed");
                        },
                        _ => {}
                    }
                    cpu.data = ram[address as usize];
                } else {
                    ram[address as usize] = cpu.data;
                }
            }
        }
    }
}