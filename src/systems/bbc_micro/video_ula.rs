pub(crate) struct Pins {
    /// LSB of address. Used to distinguish between writing to SHEILA &20 and &21.
    pub a0: bool,

    pub data: u8,
    pub crtc_clk: bool,
    pub r_in: bool,
    pub g_in: bool,
    pub b_in: bool,
    pub r: bool,
    pub g: bool,
    pub b: bool,

    /// Chip select
    pub cs: bool,

    /// 16MHz input
    pub clk_16mhz: bool,

    /// 8MHZ output
    pub clk_8mhz: bool,

    /// 4MHZ output
    pub clk_4mhz: bool,

    /// 2MHZ output
    pub clk_2mhz: bool,

    /// 1MHZ output
    pub clk_1mhz: bool,
}

impl Pins {
    fn new() -> Self {
        Self {
            a0: false,
            data: 0,
            crtc_clk: false,
            r_in: false,
            g_in: false,
            b_in: false,
            r: false,
            g: false,
            b: false,
            cs: false,
            clk_16mhz: false,
            clk_8mhz: false,
            clk_4mhz: false,
            clk_2mhz: false,
            clk_1mhz: false,
        }
    }
}

enum SelectedFlashColour {
    FirstColour,
    SecondColour,
}

enum RGBInputSource {
    OnChipSerialiser,
    TeletextInput,
}

enum CRTCClockRate {
    /// Modes 4-7
    _1MHz,

    /// Modes 0-3
    _2MHz,
}

pub(crate) struct VideoULA {
    pub(crate) pins: Pins,

    /// Selects which colour of the two flashing colours is actually displayed at a given time.
    selected_flash_colour: SelectedFlashColour,

    /// Selects whether RGB input comes from the video serialiser in the ULA
    /// or from the teletext chip.
    rgb_input_source: RGBInputSource, 

    /// 2-bit number that determines the actual number of displayed characters per line.
    num_characters_per_line: u8,

    crtc_clock_rate: CRTCClockRate,

    cursor_width_in_bytes: u8,

    large_cursor: bool,

    /// Palette RAM. Stores the lookup from logical to actual colour.
    /// There are 8 actual colours available, with a 4th bit as a "flash" bit.
    /// 
    /// The 8 colours are:
    /// 
    /// CODE   BGR   Colour
    /// 0      000   Black
    /// 1      001   Red
    /// 2      010   Green
    /// 3      011   Yellow
    /// 4      100   Blue
    /// 5      101   Magenta
    /// 6      110   Cyan
    /// 7      111   White
    palette: [u8; 16],

    clock_counter: u8,
}

impl VideoULA {
    pub(crate) fn new() -> Self {
        Self {
            pins: Pins::new(),

            selected_flash_colour: SelectedFlashColour::FirstColour,
            rgb_input_source: RGBInputSource::OnChipSerialiser,
            num_characters_per_line: 0,
            crtc_clock_rate: CRTCClockRate::_1MHz,
            cursor_width_in_bytes: 0,
            large_cursor: false,

            palette: [0; 16],

            clock_counter: 0,
        }
    }

    pub(crate) fn tick(&mut self) {
        if self.pins.cs {
            self.tick_processor_interface();
        }

        if self.pins.clk_16mhz {
            self.tick_16mhz();
        }
    }

    fn tick_processor_interface(&mut self) {
        if self.pins.a0 {
            // Palette
            
            let logical_colour = self.pins.data >> 4;
            let actual_colour = self.pins.data & 0xF;
            self.palette[logical_colour as usize] = actual_colour;
        } else {
            // Video control register.

            self.selected_flash_colour = match self.pins.data & 1 {
                0 => SelectedFlashColour::FirstColour,
                1 => SelectedFlashColour::SecondColour,
                _ => unreachable!()
            };

            self.rgb_input_source = match (self.pins.data >> 1) & 1 {
                0 => RGBInputSource::OnChipSerialiser,
                1 => RGBInputSource::TeletextInput,
                _ => unreachable!()
            };

            self.num_characters_per_line = (self.pins.data >> 2) & 0x3;

            self.crtc_clock_rate = match (self.pins.data >> 4) & 1 {
                0 => CRTCClockRate::_1MHz,
                1 => CRTCClockRate::_2MHz,
                _ => unreachable!()
            };

            self.cursor_width_in_bytes = (self.pins.data >> 5) & 0x3;

            self.large_cursor = self.pins.data & 0x80 != 0;
        }
    }

    fn tick_16mhz(&mut self) {
        match self.rgb_input_source {
            RGBInputSource::OnChipSerialiser => {
                // TODO
            }

            RGBInputSource::TeletextInput => {
                self.pins.r = self.pins.r_in;
                self.pins.g = self.pins.g_in;
                self.pins.b = self.pins.b_in;
            }
        }

        self.clock_counter = (self.clock_counter + 1) & 0xF;

        self.pins.clk_8mhz = self.clock_counter & 0b001 == 0b001;
        self.pins.clk_4mhz = self.clock_counter & 0b011 == 0b011;
        self.pins.clk_2mhz = self.clock_counter & 0b111 == 0b111;
        self.pins.clk_1mhz = self.clock_counter == 0;

        match self.crtc_clock_rate {
            CRTCClockRate::_1MHz => {
                self.pins.crtc_clk = self.pins.clk_1mhz;
            }

            CRTCClockRate::_2MHz => {
                self.pins.crtc_clk = self.pins.clk_2mhz;
            }
        }
    }
}