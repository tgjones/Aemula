mod rom;

pub(crate) struct Pins {
    pub character_data: u8,

    /// Data entry window
    pub dew: bool,

    /// Character rounding select
    pub crs: bool,

    /// Red output
    pub r: bool,

    /// Green output
    pub g: bool,

    /// Blue output
    pub b: bool,

    /// Load output shift register enable
    pub lose: bool,

    /// 1MHz input
    pub f1: bool,

    /// 6MHz input,
    pub tr6: bool,
}

impl Pins {
    fn new() -> Self {
        Self {
            character_data: 0,
            dew: false,
            crs: false,
            r: false,
            g: false,
            b: false,
            lose: false,
            f1: false,
            tr6: false,
        }
    }
}

pub(crate) struct SAA5050 {
    pub pins: Pins,

    character_data_stored: u8,
}

impl SAA5050 {
    pub(crate) fn new() -> Self {
        Self {
            pins: Pins::new(),
            character_data_stored: 0,
        }
    }

    /// Should be called at 6MHz.
    pub(crate) fn tick(&mut self) {
        if self.pins.f1 {
            self.character_data_stored = self.pins.character_data;
        }

        // Lookup character.
        let _ = rom::CHARACTERS[0]; // TODO
    }
}