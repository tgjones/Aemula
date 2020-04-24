use aemula_macros::PinAccessors;

#[derive(PinAccessors)]
pub struct Cartridge {
    #[pin(in)]
    #[handle(change)]
    a: u16,

    #[pin(out)]
    d: u8,

    rom_data: Vec<u8>
}

impl Cartridge {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            a: 0,
            d: 0,

            rom_data: data
        }
    }

    fn on_a_change(&mut self) {
        self.d = self.rom_data[self.a as usize];
    }
}