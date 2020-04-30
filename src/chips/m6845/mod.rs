struct RegisterDefinition(u8, bool, bool); // mask, can_read, can_write

// Although there are only 18 internal registers, we index using a 5-bit value,
// so to avoid out-of-range indexing we set indices 18 to 31 to zeros.
const REGISTER_DEFINITONS: [RegisterDefinition; 32] = [
    RegisterDefinition(0xFF, false, true), // R0
    RegisterDefinition(0xFF, false, true), // R1
    RegisterDefinition(0xFF, false, true), // R2
    RegisterDefinition(0xFF, false, true), // R3
    RegisterDefinition(0x7F, false, true), // R4
    RegisterDefinition(0x1F, false, true), // R5
    RegisterDefinition(0x7F, false, true), // R6
    RegisterDefinition(0x7F, false, true), // R7
    RegisterDefinition(0x03, false, true), // R8
    RegisterDefinition(0x1F, false, true), // R9
    RegisterDefinition(0x7F, false, true), // R10
    RegisterDefinition(0x1F, false, true), // R11
    RegisterDefinition(0x3F, false, true), // R12
    RegisterDefinition(0xFF, false, true), // R13
    RegisterDefinition(0x3F, true, true),  // R14
    RegisterDefinition(0xFF, true, true),  // R15
    RegisterDefinition(0x00, true, false), // R16
    RegisterDefinition(0x00, true, false), // R17
    RegisterDefinition(0x00, false, false),
    RegisterDefinition(0x00, false, false),
    RegisterDefinition(0x00, false, false),
    RegisterDefinition(0x00, false, false),
    RegisterDefinition(0x00, false, false),
    RegisterDefinition(0x00, false, false),
    RegisterDefinition(0x00, false, false),
    RegisterDefinition(0x00, false, false),
    RegisterDefinition(0x00, false, false),
    RegisterDefinition(0x00, false, false),
    RegisterDefinition(0x00, false, false),
    RegisterDefinition(0x00, false, false),
    RegisterDefinition(0x00, false, false),
    RegisterDefinition(0x00, false, false),
];

#[derive(Copy, Clone)]
pub struct Pins {
    // -----------------------------------------------
    // Processor Interface
    // -----------------------------------------------

    /// Data bus
    pub d: u8,

    /// Chip select
    pub cs: bool,

    /// Register select (address register = false, data register = true)
    pub rs: bool,

    /// Read/write (read = true, write = false)
    pub rw: bool,

    // -----------------------------------------------
    // CRT Control
    // -----------------------------------------------

    /// Vertical sync active
    pub vs: bool,

    /// Horizontal sync active
    pub hs: bool,

    /// Display timing
    pub disptmg: bool,

    // -----------------------------------------------
    // Refresh Memory / Character Generator Addressing
    // -----------------------------------------------

    /// Refresh memory address
    pub ma: u16,

    /// Row address
    pub ra: u8,

    // -----------------------------------------------
    // Other
    // -----------------------------------------------

    /// Cursor
    pub cursor: bool,

    /// Character clock
    pub clk: bool,

    // /// Light pen strobe
    // lpstb: bool,
}

impl Pins {
    fn new() -> Self {
        Self {
            d: 0,
            cs: false,
            rs: false,
            rw: false,
            
            vs: false,
            hs: false,
            disptmg: false,

            ma: 0,
            ra: 0,

            cursor: false,
            clk: false,
            //lpstb: false,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
struct RegisterNames {
    horizontal_total:         u8,
    horizontal_displayed:     u8,
    horizontal_sync_position: u8,
    sync_widths:              u8,
    vertical_total:           u8,
    vertical_total_adjust:    u8,
    vertical_displayed:       u8,
    vertical_sync_position:   u8,
    interlace_mode_and_skew:  u8,
    max_raster_address:       u8,
    cursor_start:             u8,
    cursor_end:               u8,
    start_address_hi:         u8,
    start_address_lo:         u8,
    cursor_address_hi:        u8,
    cursor_address_lo:        u8,
    lightpen_hi:              u8,
    lightpen_lo:              u8,
}

impl RegisterNames {
    fn get_horizontal_sync_width(&self) -> u8 {
        self.sync_widths & 0xF
    }

    fn get_vertical_sync_width(&self) -> u8 {
        self.sync_widths >> 4
    }
}

#[repr(C)]
union Registers {
    array: [u8; 18],
    names: RegisterNames,
}

impl Registers {
    fn new() -> Self {
        Self {
            array: [0; 18]
        }
    }
}

pub(crate) struct M6845 {
    pub pins: Pins,

    address: u8,
    registers: Registers,

    memory_address: u16,
    memory_address_stored: u16,

    horizontal_counter: u8,
    vertical_counter: u8,
    raster_address: u8,

    // Other state
    horizontal_sync: bool,
    horizontal_sync_counter: u8,
    horizontal_display_enable: bool,
    vertical_sync: bool,
    vertical_sync_counter: u8,
    vertical_display_enable: bool,
}

impl M6845 {
    pub(crate) fn new() -> Self {
        Self {
            pins: Pins::new(),

            address: 0,
            registers: Registers::new(),

            memory_address: 0,
            memory_address_stored: 0,

            horizontal_counter: 0,
            vertical_counter: 0,
            raster_address: 0,

            horizontal_sync: false,
            horizontal_sync_counter: 0,
            horizontal_display_enable: false,
            vertical_sync: false,
            vertical_sync_counter: 0,
            vertical_display_enable: false,
        }
    }

    pub(crate) fn tick(&mut self) {
        if self.pins.cs {
            self.tick_processor_interface();
        }

        if self.pins.clk {
            self.tick_crt();
        }
    }

    /// This function emulates what happens when the E (enable) and CS (chip select) pins are active.
    /// When E is set, the "processor interface" is active, which includes
    /// - D0-D7 (data bus)
    /// - CS (chip select)
    /// - RS (register select)
    /// - RW (read/write)
    fn tick_processor_interface(&mut self) {
        if self.pins.rs {
            // Data registers
            let register_definition = &REGISTER_DEFINITONS[self.address as usize];
            unsafe {
                if self.pins.rw { // Read
                    if register_definition.1 { // can_read
                        self.pins.d = self.registers.array[self.address as usize];
                    } else {
                        self.pins.d = 0;
                    }
                } else { // Write
                    if register_definition.2 { // can_write
                        self.registers.array[self.address as usize] = self.pins.d & register_definition.0;
                    }
                }
            }
        } else {
            // Address register
            if self.pins.rw {
                self.pins.d = 0; // Can't read address register.
            } else {
                self.address = self.pins.d & 0x1F;
            }
        }

        //println!("Tick 6845 rs {} data {:02X} rw {}", self.pins.rs, self.pins.d, self.pins.rw);
    }

    fn at_end_of_scanline(&self) -> bool {
        unsafe {
            self.horizontal_counter >= self.registers.names.horizontal_total
        }
    }

    fn at_end_of_character_row(&self) -> bool {
        unsafe {
            let mut max_raster_address = self.registers.names.max_raster_address;

            // Are we in the vertical adjust area?
            if self.vertical_counter >= self.registers.names.vertical_total {
                max_raster_address += self.registers.names.vertical_total_adjust;
            }

            self.raster_address >= (max_raster_address + 1)
        }
    }

    fn at_end_of_vertical_displayed(&self) -> bool {
        unsafe {
            self.vertical_counter == self.registers.names.vertical_displayed
        }
    }

    fn at_start_of_vertical_sync(&self) -> bool {
        unsafe {
            self.vertical_counter >= self.registers.names.vertical_sync_position
        }
    }

    fn at_end_of_vertical_sync(&self) -> bool {
        unsafe {
            self.vertical_sync_counter == self.registers.names.get_vertical_sync_width()
        }
    }

    fn at_end_of_horizontal_displayed(&self) -> bool {
        unsafe {
            self.horizontal_counter >= self.registers.names.horizontal_displayed
        }
    }

    fn at_start_of_horizontal_sync(&self) -> bool {
        unsafe {
            self.horizontal_counter >= self.registers.names.horizontal_sync_position
        }
    }

    fn at_end_of_horizontal_sync(&self) -> bool {
        unsafe {
            self.horizontal_sync_counter == self.registers.names.get_horizontal_sync_width()
        }
    }

    fn at_end_of_frame(&self) -> bool {
        unsafe {
            self.vertical_counter >= self.registers.names.vertical_total
        }
    }

    fn get_start_address(&self) -> u16 {
        unsafe {
            let register_names = self.registers.names;
            ((register_names.start_address_hi as u16) << 8) | (register_names.start_address_lo as u16)
        }
    }

    /// This function emulates what happens when the CLK pin is active.
    /// This drives the CRT functions of the CRTC, which includes everything
    /// except the processor interface (handled by `cycle_processor_interface` above).
    fn tick_crt(&mut self) {
        if self.at_end_of_scanline() {
            //println!("End of scanline");

            if self.at_end_of_character_row() {
                //println!("End of character row");
                self.raster_address = 0;

                if self.at_end_of_vertical_displayed() {
                   // println!("End of vertical displayed");
                    self.vertical_display_enable = false;
                }

                if self.at_end_of_frame() {
                    //println!("End of frame");
                    self.vertical_counter = 0;
                    self.vertical_display_enable = true;
                    self.memory_address_stored = self.get_start_address();
                }

                if self.at_start_of_vertical_sync() {
                    //println!("Start of vertical sync");
                    self.vertical_sync = true;
                    self.vertical_sync_counter = 0;
                }
            } else {
                self.raster_address = (self.raster_address + 1) & 0x1F; // Wrap at 5 bits.
            }

            self.horizontal_counter = 0;
            self.horizontal_display_enable = true;

            // Start ma from the value we stored in the previous scanline.
            self.memory_address = self.memory_address_stored;

            if self.vertical_sync {
                self.vertical_counter += 1;
                if self.at_end_of_vertical_sync() {
                    //println!("End of vertical sync");
                    self.vertical_sync = false;
                }
            }
        } else {
            self.horizontal_counter += 1;
            self.memory_address = (self.memory_address + 1) & 0x3FFF; // Wrap at 14 bits.
        }

        if self.at_end_of_horizontal_displayed() {
            //println!("End of horizontal displayed");
            self.horizontal_display_enable = false;

            // Keep track of which memory address we've reached,
            // so we can restart from here in the next scanline.
            self.memory_address_stored = self.memory_address;
        }

        if self.at_start_of_horizontal_sync() {
           // println!("Start of horizontal sync");
            self.horizontal_sync = true;
            self.horizontal_sync_counter = 0;
        }

        if self.horizontal_sync {
            if self.at_end_of_horizontal_sync() {
                //println!("End of horizontal sync");
                self.horizontal_sync = false;
            } else {
                self.horizontal_sync_counter += 1;
            }
        }

        // Set pins.
        self.pins.hs = self.horizontal_sync;
        self.pins.vs = self.vertical_sync;
        self.pins.disptmg = self.horizontal_display_enable && self.vertical_display_enable;
        self.pins.ma = self.memory_address;
        self.pins.ra = self.raster_address;
    }
}