use aemula_macros::PinAccessors;
use crate::util::Bit;

// Horizontal Timing:
// - 

#[derive(PinAccessors)]
pub struct TIA {
    #[pin(in)]
    #[handle(transition_lo_to_hi, transition_hi_to_lo)]
    pin_osc: bool,

    /// Φ0 clock output.
    #[pin(out)]
    pin_phi0: bool,

    /// Φ2 clock input from CPU.
    #[pin(in)]
    #[handle(transition_lo_to_hi)]
    pin_phi2: bool,

    /// Chip select 0. Must be low to enable chip.
    #[pin(in)]
    pin_cs0: bool,

    /// Chip select 1. Must be high to enable chip.
    #[pin(in)]
    pin_cs1: bool,

    /// Chip select 2. Must be low to enable chip.
    #[pin(in)]
    pin_cs2: bool,

    /// Chip select 3. Must be low to enable chip.
    #[pin(in)]
    pin_cs3: bool,

    /// Ready pin.
    #[pin(out)]
    pin_rdy: bool,

    #[pin(out)]
    pin_sync: bool,

    /// Address pins A0..A5.
    #[pin(in)]
    pin_a: u8,

    /// Processor data pins 0 to 5. These pins are inputs only.
    #[pin(in)]
    pin_d_05: u8,

    /// Processor data pins 6 and 7. These pins are bidirectional.
    #[pin(bidirectional)]
    pin_d_67: u8,

    /// Read/write pin. Read = true, write = false.
    #[pin(in)]
    pin_rw: bool,

    /// Video luminance output (3 pins, LUM0..LUM2).
    #[pin(out)]
    pin_lum: u8,

    /// Video color output.
    // TODO: This should be a single pin. From the spec:
    // "A digital phase shifter is included on this chip to provide a
    // single color output with fifteen (15) phase angles."
    // But for now we just output a 4-bit colour.
    #[pin(out)]
    pin_col: u8,

    /// Vertical blank output.
    #[pin(out)]
    pin_blk: bool,

    /// Color delay input.
    #[pin(in)]
    pin_del: bool,

    /// Audio output 0.
    #[pin(out)]
    pin_aud0: bool,

    /// Audio output 1.
    #[pin(out)]
    pin_aud1: bool,

    /// Dumped and latched inputs.
    /// Dumped inputs (I0..I3) are used for paddles.
    /// Latched inputs (I4..I5) are used for joystick / paddle triggers.
    // TODO: May need to split these into separate pins.
    #[pin(in)]
    pin_i: u8,

    /// Helps with divide-by-3 from `osc` input to `phi0` output.
    phi0_clock_counter: u8,

    /// Clock divider. Divides the input OSC clock by 4.
    clock_divide_by_4: u8,

    horizontal_counter: u8,

    vsync: bool,
    vblank: bool,
}

impl TIA {
    pub fn new() -> Self {
        Self {
            pin_osc: false,
            pin_phi0: false,
            pin_phi2: false,
            pin_cs0: false,
            pin_cs1: false,
            pin_cs2: false,
            pin_cs3: false,
            pin_rdy: true,
            pin_sync: false,
            pin_a: 0,
            pin_d_05: 0,
            pin_d_67: 0,
            pin_rw: false,
            pin_lum: 0,
            pin_col: 0,
            pin_blk: false,
            pin_del: false,
            pin_aud0: false,
            pin_aud1: false,
            pin_i: 0,

            phi0_clock_counter: 0,
            clock_divide_by_4: 0,
            horizontal_counter: 0,

            vsync: false,
            vblank: false,
        }
    }

    pub fn is_selected(&self) -> bool {
        !self.pin_cs0 && self.pin_cs1 && !self.pin_cs2 && !self.pin_cs3
    }

    fn on_pin_osc_transition_lo_to_hi(&mut self) {
        // TODO

        self.update_phi0();
    }

    fn on_pin_osc_transition_hi_to_lo(&mut self) {
        // TODO

        self.clock_divide_by_4 += 1;
        if self.clock_divide_by_4 == 4 {
            self.clock_divide_by_4 = 0;

            TIA::update_polynomial_counter(&mut self.horizontal_counter);
            
            if self.horizontal_counter == 0b111111 {
                self.pin_rdy = true;
            }
        }

        self.update_phi0();
    }

    /// Updates 6-bit counter value.
    fn update_polynomial_counter(counter: &mut u8) {
        // Put high 5 bits from old value into low 5 bits in new value.
        let new_lo_bits = counter.wrapping_shr(1) & 0b11111;

        // New high bit is 1 if the low 2 bits are the same, and 0 if they are different.
        let new_hi_bit = if counter.bit(0) == counter.bit(1) { 1 } else { 0 };

        *counter = (new_hi_bit << 5) | new_lo_bits;
    }

    fn update_phi0(&mut self) {
        self.phi0_clock_counter += 1;

        if self.phi0_clock_counter == 3 {
            self.phi0_clock_counter = 0;
            self.pin_phi0 = !self.pin_phi0;
        }

        self.pin_sync = self.vsync; // || self.hsync
    }

    fn on_pin_phi2_transition_lo_to_hi(&mut self) {
        if self.is_selected() {
            if self.pin_rw {
                // Read registers.
                println!(
                    "TIA read register. Address = {:02X}", self.pin_a);

                match self.pin_a {
                    // CXM0P - Read collision
                    0x00 => {},

                    // TODO

                    // Ignore invalid addresses
                    _ => {},
                }
            } else {
                // Write registers.
                println!(
                    "TIA write register. Address = {:02X}. Data67 = {:02X}, Data05 = {:02X}", 
                    self.pin_a,
                    self.pin_d_67, 
                    self.pin_d_05);

                match self.pin_a {
                    // VSYNC - Vertical sync set/clear
                    0x00 => self.vsync = self.pin_d_05.bit(1),

                    // VBLANK - Vertical blank set/clear
                    0x01 => self.vblank = self.pin_d_05.bit(1),

                    // WSYNC - Wait for leading edge of horizontal blank
                    0x02 => self.pin_rdy = false,

                    // RSYNC - Reset horizontal sync counter
                    0x03 => {},// self.pin_rdy = false,

                    // NUSIZ0 - Number-size player-missile 0
                    0x04 => {},

                    // NUSIZ1 - Number-size player-missile 1
                    0x05 => {},

                    // COLUP0 - Color-luminance player 0
                    0x06 => {},

                    // COLUP1 - Color-luminance player 1
                    0x07 => {},

                    // COLUPF - Color-luminance playfield
                    0x08 => {},

                    // COLUBK - Color-luminance background
                    0x09 => {},

                    // CTRLPF - Control playfield ball size and collisions
                    0x0A => {},

                    // REFP0 - Reflect player 0
                    0x0B => {},

                    // REFP1 - Reflect player 1
                    0x0C => {},

                    // PF0 - Playfield register byte 0
                    0x0D => {},

                    // PF1 - Playfield register byte 1
                    0x0E => {},

                    // PF2 - Playfield register byte 2
                    0x0F => {},

                    // RESP0 - Reset player 0
                    0x10 => {},

                    // RESP1 - Reset player 1
                    0x11 => {},

                    // RESM0 - Reset missile 0
                    0x12 => {},

                    // RESM1 - Reset missile 1
                    0x13 => {},

                    // RESBL - Reset ball
                    0x14 => {},

                    // AUDC0 - Audio control 0
                    0x15 => {},

                    // AUDC1 - Audio control 1
                    0x16 => {},

                    // AUDF0 - Audio frequency 0
                    0x17 => {},

                    // AUDF1 - Audio frequency 1
                    0x18 => {},

                    // AUDV0 - Audio volume 0
                    0x19 => {},

                    // AUDv1 - Audio volume 1
                    0x1A => {},

                    // GRP0 - Graphics player 0
                    0x1B => {},

                    // GRP1 - Graphics player 1
                    0x1C => {},

                    // ENAM0 - Graphics (enable) missile 0
                    0x1D => {},

                    // ENAM1 - Graphics (enable) missile 1
                    0x1E => {},

                    // ENABL - Graphics (enable) ball
                    0x1F => {},

                    // HMP0 - Horizontal motion player 0
                    0x20 => {},

                    // HMP1 - Horizontal motion player 1
                    0x21 => {},

                    // HMM0 - Horizontal motion missile 0
                    0x22 => {},

                    // HMM1 - Horizontal motion missile 1
                    0x23 => {},

                    // HMBL - Horizontal motion ball
                    0x24 => {},

                    // VDELP0 - Vertical delay player 0
                    0x25 => {},

                    // VDELP1 - Vertical delay player 1
                    0x26 => {},

                    // VDELBL - Vertical delay ball
                    0x27 => {},

                    // RESMP0 - Reset missile 0 to player 0
                    0x28 => {},

                    // RESMP1 - Reset missile 1 to player 1
                    0x29 => {},

                    // HMOVE - Apply horizontal motion
                    0x2A => {},

                    // HMCLR - Clear horizontal motion registers
                    0x2B => {},

                    // CXCLR - Clear collision latches
                    0x2C => {},

                    // Ignore invalid addresses
                    _ => {}
                }
            }
        }
    }
}