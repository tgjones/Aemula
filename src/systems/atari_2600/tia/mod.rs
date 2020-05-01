use aemula_macros::PinAccessors;

#[derive(PinAccessors)]
pub struct TIA {
    #[pin(in)]
    #[handle(transition_lo_to_hi, transition_hi_to_lo)]
    pin_osc: bool,

    #[pin(out)]
    pin_phi0: bool,

    #[pin(out)]
    pin_rdy: bool,

    #[pin(out)]
    pin_sync: bool,

    /// Address pins A0..A5.
    #[pin(in)]
    pin_a: u8,

    /// Data pins. Only D6 and D7 are bidirectional. The other pins are inputs only.
    #[pin(bidirectional)]
    pin_d: u8,

    /// Helps with divide-by-3 from `osc` input to `phi0` output.
    phi0_clock_counter: u8,
}

impl TIA {
    pub fn new() -> Self {
        Self {
            pin_osc: false,
            pin_phi0: false,
            pin_rdy: true,
            pin_sync: false,
            pin_a: 0,
            pin_d: 0,

            phi0_clock_counter: 0,
        }
    }

    fn on_pin_osc_transition_lo_to_hi(&mut self) {

    }

    fn on_pin_osc_transition_hi_to_lo(&mut self) {
        // TODO
    }
}