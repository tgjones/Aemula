use crate::chips::{m6522, m6845, mos6502, saa5050};

mod video_ula;

pub struct BBCMicro {
    ram: [u8; 0x8000],

    cpu: mos6502::MOS6502,
    cpu_pins: mos6502::Pins,

    crtc: m6845::M6845,

    video_ula: video_ula::VideoULA,

    teletext: saa5050::SAA5050,

    // system_via: m6522::M6522,
    // user_via: m6522::M6522,
    
    os_rom: Vec<u8>,
    basic_rom: Vec<u8>,

    // For debugging.
    clock_counter: u64,
}

impl BBCMicro {
    pub fn new(os_rom: Vec<u8>, basic_rom: Vec<u8>) -> Self {
        assert_eq!(0x4000, os_rom.len());

        let ram = [0; 0x8000];

        let (cpu, cpu_pins) = mos6502::MOS6502::new_with_options(mos6502::MOS6502Options {
            bcd_enabled: true
        });

        let crtc = m6845::M6845::new();

        let video_ula = video_ula::VideoULA::new();

        let teletext = saa5050::SAA5050::new();

        // let system_via = m6522::M6522::new();
        // let user_via = m6522::M6522::new();

        Self {
            ram,
            cpu,
            cpu_pins,
            crtc,
            video_ula,
            teletext,
            // system_via,
            // user_via,
            os_rom,
            basic_rom,
            clock_counter: 0,
        }
    }

    /// Called at 16MHz.
    pub fn tick(&mut self) {
        // Tick Video ULA at 16MHz.
        self.video_ula.pins.clk_16mhz = true;
        self.video_ula.pins.r_in = self.teletext.pins.r;
        self.video_ula.pins.g_in = self.teletext.pins.g;
        self.video_ula.pins.b_in = self.teletext.pins.b;
        self.video_ula.tick();
        self.video_ula.pins.clk_16mhz = false;

        if self.video_ula.pins.clk_2mhz {
            self.tick_cpu();
        }

        // Tick CRTC.
        // TODO: Not sure of order here, should CRTC and SAA5050 be ticked before Video ULA?
        self.crtc.pins.clk = self.video_ula.pins.crtc_clk;
        self.crtc.tick();
        self.crtc.pins.clk = false;
        //println!("CRTC ma ${:04X} ra ${:02X}", self.crtc_pins.ma, self.crtc_pins.ra);

        // Only read screen memory if CRTC clock pin is active.
        if self.video_ula.pins.crtc_clk {
            let video_data = self.ram[self.crtc.pins.ma as usize];
            self.video_ula.pins.data = video_data;
            self.teletext.pins.character_data = video_data;
            //println!("Video data ${:02X} address {:04X}", video_data, self.crtc.pins.ma);
        }

        // Tick SAA5050 Teletext chip.
        self.teletext.pins.dew = self.crtc.pins.vs;
        self.teletext.pins.crs = (self.crtc.pins.ra & 1) == 1;
        self.teletext.pins.lose = self.crtc.pins.disptmg;
        self.teletext.pins.f1 = self.video_ula.pins.clk_1mhz;
        self.teletext.pins.tr6 = self.clock_counter % 6 == 0; // Hacky way to generate 6MHz clock.
        self.teletext.tick();

        // TODO: Do something with Video ULA's RGB output.
        // self.video_ula.pins.r
        // self.video_ula.pins.g
        // self.video_ula.pins.b
    }

    fn tick_cpu(&mut self) {
        self.clock_counter += 1;
        
        // Tick CPU and perform requested memory reads / writes.
        
        // TODO: 1MHz cycle stretching.
        let mut cpu_pins = self.cpu.cycle(self.cpu_pins);

        // if cpu_pins.sync {
        //     println!("{:04X}  A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} CPUC:{}", 
        //              self.cpu.registers.pc.to_u16(),
        //              self.cpu.registers.a,
        //              self.cpu.registers.x,
        //              self.cpu.registers.y,
        //              self.cpu.registers.p.as_u8(false),
        //              self.cpu.registers.sp,
        //              self.cycles);
        // }

        let address = cpu_pins.get_address();

        if self.clock_counter % 10000 == 0 {
            println!("Mode {}", self.ram[0x355]);
        }

        match address {
            // RAM
            0x0000..=0x7FFF => {
                if cpu_pins.rw {
                    cpu_pins.data = self.ram[address as usize];
                } else {
                    self.ram[address as usize] = cpu_pins.data;
                }
            }

            // 0xFC00..=0xFCFF => 0, // FRED I/O
            // 0xFD00..=0xFDFF => 0, // JIM I/O

            // SHEILA
            0xFE00..=0xFEFF => {
                match address as u8 {
                    // 6845 CRTC
                    0x00..=0x07 => {
                        self.crtc.pins.cs = true;
                        self.crtc.pins.rs = (address & 1) == 1;
                        self.crtc.pins.rw = cpu_pins.rw;
                        self.crtc.pins.d = cpu_pins.data;
                        self.crtc.tick();
                        self.crtc.pins.cs = false;
                        println!("CRTC rs {:05} d ${:02X} rw {}", self.crtc.pins.rs, cpu_pins.data, self.crtc.pins.rw);
                    }

                    // Video ULA
                    0x20..=0x2F => {
                        if !cpu_pins.rw {
                            self.video_ula.pins.cs = true;
                            self.video_ula.pins.a0 = (address & 1) == 1;
                            self.video_ula.pins.data = cpu_pins.data;
                            self.video_ula.tick();
                            self.video_ula.pins.cs = false;
                            println!("Video ULA a0 {:05} d ${:02X}", self.video_ula.pins.a0, self.video_ula.pins.data);
                        }
                    }

                    _ => {
                       // println!("Unknown address {:04X} data {:02X}", address, cpu_pins.data);
                    }
                }
            }

            // Paged ROM
            0x8000..=0xBFFF => {
                if cpu_pins.rw {
                    cpu_pins.data = self.basic_rom[(address - 0x8000) as usize];
                }
            }

            // Operating System ROM
            0xC000..=0xFFFF => {
                if cpu_pins.rw {
                    cpu_pins.data = self.os_rom[(address - 0xC000) as usize];
                }
            }
        }

        // if cpu_pins.rw {
        //     println!("READ      ${:04X} => ${:02X}", address, cpu_pins.data);
        // } else {
        //     println!("WRITE     ${:04X} <= ${:02X}", address, cpu_pins.data);
        // }

        self.cpu_pins = cpu_pins;
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};
    use super::BBCMicro;

    //#[test]
    fn boot_rom() {
        let os_rom_path = Path::new("assets/systems/bbc_micro/roms/os.rom");
        let os_rom = fs::read(os_rom_path).unwrap();

        let basic_rom_path = Path::new("assets/systems/bbc_micro/roms/BASIC.rom");
        let basic_rom = fs::read(basic_rom_path).unwrap();

        let mut bbc_micro = BBCMicro::new(os_rom, basic_rom);

        // TODO: Don't fix loop count.
        for _ in 0..1000000 {
            bbc_micro.tick();
        }
    }
}