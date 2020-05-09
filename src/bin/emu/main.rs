use aemula::systems::atari_2600::{Atari2600, cartridge::Cartridge, WIDTH, HEIGHT};
use std::{fs, path::Path};

use minifb::{Key, Window, WindowOptions};

fn main() {
    let mut atari_2600 = Atari2600::new();

    let timer_test_rom_path = Path::new("test_assets/systems/atari_2600/timer_test_v2_NTSC.bin");
    let timer_test_rom = fs::read(timer_test_rom_path).unwrap();
    let timer_test_cartridge = Cartridge::from_data(timer_test_rom);
    atari_2600.insert_cartridge(timer_test_cartridge);

    atari_2600.reset();

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // TODO: Make this less hard-coded.
        for _ in 0..228*262 {
            atari_2600.tick();
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&atari_2600.video_data, WIDTH, HEIGHT)
            .unwrap();
    }
}