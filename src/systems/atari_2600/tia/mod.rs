use aemula_macros::PinAccessors;

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

    /// Video luminance output 0.
    #[pin(out)]
    pin_lum0: bool,

    /// Video luminance output 1.
    #[pin(out)]
    pin_lum1: bool,

    /// Video luminance output 2.
    #[pin(out)]
    pin_lum2: bool,

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
            pin_lum0: false,
            pin_lum1: false,
            pin_lum2: false,
            pin_blk: false,
            pin_del: false,
            pin_aud0: false,
            pin_aud1: false,
            pin_i: 0,

            phi0_clock_counter: 0,
        }
    }

    pub fn is_selected(&self) -> bool {
        !self.pin_cs0 && self.pin_cs1 && !self.pin_cs2 && !self.pin_cs3
    }

    fn on_pin_osc_transition_lo_to_hi(&mut self) {

    }

    fn on_pin_osc_transition_hi_to_lo(&mut self) {
        // TODO
    }
}