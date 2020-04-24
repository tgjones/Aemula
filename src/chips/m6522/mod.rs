extern crate aemula_macros;

use aemula_macros::PinAccessors;

#[derive(PinAccessors)]
pub(crate) struct M6522 {
    // -----------------------------------------------
    // Processor Interface
    // -----------------------------------------------

    /// Phase two clock. Data transfers between the 6522 and CPU only take place while
    /// phi2 is high.
    #[pin(in)]
    #[handle(transition_lo_to_hi, transition_hi_to_lo)]
    phi2: bool,

    // Chip select. On a real 6522 there are two chip select pins, CS1 and CS2.
    // Here we simply to a single pin.
    cs: bool,

    /// Register select (RS0-RS3)
    rs: u8,

    /// Read/write (read = true, write = false)
    rw: bool,

    /// Data bus (D0-D7)
    #[pin(bidirectional)]
    d: u8,

    /// Reset
    #[pin(in)]
    #[handle(always)]
    res: bool,

    /// Interrupt request
    irq: bool,

    // -----------------------------------------------
    // Peripheral Interface
    // -----------------------------------------------
    
    /// Peripheral A Port (PA0-PA7)
    pa: u8,

    /// Peripheral A control line (CA1)
    ca1: bool,

    /// Peripheral A control line (CA2)
    ca2: bool,

    /// Peripheral B port (PB0-PB7)
    pb: u8,

    /// Peripheral B control line (CB1)
    cb1: bool,

    /// Peripheral B control line (CB2)
    cb2: bool,
}

impl M6522 {
    pub(crate) fn new() -> Self {
        Self {
            phi2: false,
            cs: false,
            rs: 0,
            rw: false,
            d: 0,
            res: false,
            irq: false,

            pa: 0,
            ca1: false,
            ca2: false,

            pb: 0,
            cb1: false,
            cb2: false,
        }
    }

    // Pin handlers

    fn on_phi2_transition_lo_to_hi(&mut self) {
        // TODO
    }

    fn on_phi2_transition_hi_to_lo(&mut self) {
        // TODO
    }

    // pub fn set_res(&mut self, value: bool) {
    //     self.res = value;
    //     self.on_res_set(); 
    // }

    fn on_res_set(&mut self) {
        if !self.res {
            // TODO: Reset all values.
        }
    }

    // pub(crate) fn tick(&mut self, pins: &mut Pins) {
    //     match (self.last_pins.phi2, pins.phi2) {
    //         // phi2 low -> high
    //         (false, true) => {

    //         }

    //         // phi2 high -> low
    //         (true, false) => {

    //         }

    //         // phi2 didn't change
    //         _ => {}
    //     }

    //     self.last_pins = pins;
    // }
}