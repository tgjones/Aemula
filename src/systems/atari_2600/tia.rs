use aemula_macros::PinAccessors;

#[derive(PinAccessors)]
pub struct TIA {
    #[pin(in)]
    #[handle(transition_lo_to_hi)]
    #[handle(transition_hi_to_lo)]
    osc: bool,

    #[pin(out)]
    phi0: bool,

    /// Helps with divide-by-3 from `osc` input to `phi0` output.
    phi0_clock_counter: u8,
}

impl TIA {
    fn on_osc_transition_lo_to_hi(&mut self) {

    }

    fn on_osc_transition_hi_to_lo(&mut self) {
        // TODO
    }
}