use crate::chips::{mos6502, m6532};

mod cartridge;
mod tia;

pub struct Atari2600 {
    cpu: mos6502::MOS6502,
    pia: m6532::M6532,
    cartridge: Option<cartridge::Cartridge>,
}

impl Atari2600 {
    pub fn new() -> Self {
        let cpu_options = mos6502::MOS6502Options {
            bcd_enabled: true
        };
        let (cpu, pins) = mos6502::MOS6502::new_with_options(cpu_options);

        Self {
            cpu,
            pia: m6532::M6532::new(),
            cartridge: None,
        }
    }

    pub fn insert_cartridge(&mut self, cartridge: cartridge::Cartridge) {
        self.cartridge = Some(cartridge);
    }

    pub fn remove_cartridge(&mut self) {
        self.cartridge = None;
    }

    pub fn reset(&mut self) {
        //self.cpu.reset();
    }

    pub fn tick(&mut self) {

    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};
    use super::Atari2600;
    use super::cartridge::Cartridge;

    #[test]
    fn timer_test() {
        let mut system = Atari2600::new();

        let timer_test_rom_path = Path::new("test_assets/systems/atari_2600/timer_test_v2_NTSC.rom");
        let timer_test_rom = fs::read(timer_test_rom_path).unwrap();
        let timer_test_cartridge = Cartridge::new(timer_test_rom);
        system.insert_cartridge(timer_test_cartridge);

        system.reset();

        // TODO: Don't fix loop count.
        for _ in 0..1000000 {
            system.tick();
        }
    }
}