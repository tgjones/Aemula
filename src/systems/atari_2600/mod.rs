pub mod cartridge;
mod tia;
mod palette;

use crate::util::Bit;
use crate::chips::{m6507::M6507, m6532::M6532};
use cartridge::Cartridge;
use tia::TIA;

pub const WIDTH: usize = 160;
pub const HEIGHT: usize = 192;

pub struct Atari2600 {
    cpu: M6507,
    riot: M6532,
    tia: TIA, 
    cartridge: Option<Box<dyn cartridge::Cartridge>>,

    pub video_data: Vec<u32>,

    // TODO: Move these to somewhere better.
    sync_counter: usize,
    current_scanline: usize,
    current_pos: usize,
}

impl Atari2600 {
    pub fn new() -> Self {
        Self {
            cpu: M6507::new(),
            riot: M6532::new(),
            tia: TIA::new(),
            cartridge: None,

            video_data: vec![0; WIDTH * HEIGHT],

            sync_counter: 0,
            current_scanline: 0,
            current_pos: 0,
        }
    }

    pub fn insert_cartridge(&mut self, cartridge: Box<dyn Cartridge>) {
        self.cartridge = Some(cartridge);
    }

    pub fn remove_cartridge(&mut self) {
        self.cartridge = None;
    }

    pub fn reset(&mut self) {
        self.cpu.set_pin_res(false);
        self.cpu.set_pin_res(true);
    }

    pub fn tick(&mut self) {
        self.tia.set_pin_osc(true);

        self.do_cpu_cycle();

        self.tia.set_pin_osc(false);

        self.do_cpu_cycle();

        self.do_tv_output();
    }

    fn do_cpu_cycle(&mut self) {
        // TODO: Optimise this. If PHI0 didn't chance during last OSC edge,
        // then nothing else relevant to a CPU tick will have changed either.

        self.cpu.set_pin_rdy(self.tia.pin_rdy());
        self.cpu.set_pin_phi0(self.tia.pin_phi0());

        let address = self.cpu.pin_address();

        // Set RIOT pins.
        self.riot.set_rs(address.bit(9));               // RIOT RS is connected to A9.
        self.riot.set_cs1(address.bit(7));              // RIOT CS1 is connected to A7.
        self.riot.set_cs2(address.bit(12));             // RIOT CS2 is connected to A12.
        self.riot.set_rw(self.cpu.pin_rw());            // RIOT RW is connected to CPU RW.
        self.riot.set_a((address as u8) & 0b1111111);   // RIOT Address pins are connected to A0..A6.
        self.riot.set_phi2(self.cpu.pin_phi2());

        // Set TIA pins.
        self.tia.set_pin_cs0(address.bit(12));          // TIA CS0 is connected to A12.
        self.tia.set_pin_cs1(true);                     // TIA CS1 is held high.
        self.tia.set_pin_cs2(false);                    // TIA CS2 is held low.
        self.tia.set_pin_cs3(address.bit(7));           // TIA CS3 is connected to A7.
        self.tia.set_pin_rw(self.cpu.pin_rw());         // TIA RW is connected to CPU RW.
        self.tia.set_pin_a((address as u8) & 0b111111); // TIA Address pins are connected to A0..A5.
        self.tia.set_pin_phi2(self.cpu.pin_phi2());

        // TODO: Figure out a nice way to determine which chip is driving the bus.
        if self.cpu.pin_rw() {
            // Read data.
            if self.tia.is_selected() {
                // On the TIA data pins, only pins 6 and 7 are bidirectional,
                // so we combine those with the existing value on the CPU data bus.
                self.cpu.set_pin_data((self.cpu.pin_data() & 0x3F) | (self.tia.pin_d_67() << 6));
            }
            else if self.riot.is_selected() {
                self.cpu.set_pin_data(self.riot.db());
            }
            // If a cartridge is plugged in, always give it a chance to provide data.
            if let Some(c) = &mut self.cartridge {
                c.set_pin_d(self.cpu.pin_data());
                c.set_pin_a(self.cpu.pin_address() & 0x1FFF);
                self.cpu.set_pin_data(c.pin_d());
            }
        } else {
            // Write data.
            if self.tia.is_selected() {
                self.tia.set_pin_d_05(self.cpu.pin_data() & 0x3F);
                self.tia.set_pin_d_67(self.cpu.pin_data() >> 6);
            }
            else if self.riot.is_selected() {
                self.riot.set_db(self.cpu.pin_data());
            }
            // TODO: Write to cartridge?
        }
    }

    // Based on https://github.com/SavourySnaX/EDL/blob/a6a19f9db0a939230458d36bfe2715466cfad5d2/examples/2600/2600.c#L824
    // TODO: Figure this out.
    fn do_tv_output(&mut self) {
        if self.tia.pin_sync() {
            self.sync_counter += 1;
        } else {
            if self.sync_counter > 0 {
                if self.sync_counter < 300 {
                    // Assume HSYNC has just finished.
                    self.current_scanline += 1;
                    self.current_pos = 0;
                } else {
                    // Assume VSYNC has just finished.
                    self.current_scanline = 0;
                    self.current_pos = 0;
                    // TODO: Output framebuffer to screen.
                }
            }
            self.sync_counter = 0;

            if self.current_scanline > 37 
                && self.current_scanline < 37 + 192
                && self.current_pos < 228 {
                //let palette_index = self.tia.pin_lum() | self.tia.pin_col() << 3;
                let palette_index = 10;
                let color = palette::NTSC_PALETTE[palette_index as usize];
                self.video_data[self.current_scanline * WIDTH + self.current_pos] = color;
            }
        }
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

        let timer_test_rom_path = Path::new("test_assets/systems/atari_2600/timer_test_v2_NTSC.bin");
        let timer_test_rom = fs::read(timer_test_rom_path).unwrap();
        let timer_test_cartridge = Cartridge::from_data(timer_test_rom);
        system.insert_cartridge(timer_test_cartridge);

        system.reset();

        // TODO: Don't fix loop count.
        for _ in 0..1000000 {
            system.tick();
        }
    }
}