extern crate aemula_macros;

use aemula_macros::PinAccessors;

mod timer;

const TIMER_FLAG: u8 = 0x80;
const PA7_FLAG:   u8 = 0x40;

/// 6532 chip, originally manufactured by MOS Technologies.
/// 
/// Known as RIOT (RAM, I/O, Timer), it contains:
/// - 128 bytes of RAM
/// - Two 8-bit bidirectional ports for communicating with peripherals
/// - Programmable interval timer
/// - Programmable edge detect circuit
#[derive(PinAccessors)]
pub struct M6532 {
    /// Reset Pin
    #[pin(in)]
    #[handle(always)]
    res: bool,

    /// Read/Write Pin (read = true, write = false)
    #[pin(in)]
    rw: bool,

    /// Interrupt Request Pin. Maybe be activated by either a transition on PA7,
    /// or timeout of the interval timer.
    #[pin(out)]
    irq: bool,

    /// Data Bus Pins (D0-D7)
    #[pin(bidirectional)]
    db: u8,

    /// Address Pins (A0-A6)
    #[pin(in)]
    a: u8,

    /// Peripheral A Port Pins (PA0-PA7)
    #[pin(bidirectional)]
    pa: u8,

    /// Peripheral B Port Pins (PB0-PB7)
    #[pin(bidirectional)]
    pb: u8,

    /// Input Clock Pin
    #[pin(in)]
    #[handle(transition_lo_to_hi, transition_hi_to_lo)]
    phi2: bool,

    /// Chip Select 1 Pin. Must be high to enable chip.
    #[pin(in)]
    cs1: bool,

    /// Chip Select 2 Pin. Must be low to enable chip.
    #[pin(in)]
    cs2: bool,

    /// RAM Select Pin
    #[pin(in)]
    rs: bool,

    /// 128 bytes of RAM
    ram: [u8; 128],

    /// Data Direction Register A
    ddra: u8,

    /// Data Direction Register B
    ddrb: u8,

    /// Output Register A
    ora: u8,

    /// Output Register B
    orb: u8,

    /// Handles the timer part of the RIOT chip.
    timer: timer::Timer,

    /// Stores whether timer and PA7 interrupts are enabled.
    /// Bit 7 is 1 if timer interrupts are enabled.
    /// Bit 6 is 1 if PA7 interrupts are enabled.
    irq_enabled: u8,

    /// Current state of the two interrupt flags: timer and PA7.
    /// Bit 7 is 1 if a timer interrupt should occur.
    /// Bit 6 is 1 if a PA7 interrupt should occur.
    /// If either of these is set to 1, the IRQ pin will be set low.
    irq_state: u8,

    /// True for positive edge-detect, false for negative edge-detect.
    pa7_active_edge_direction: bool,
}

impl M6532 {
    pub fn new() -> Self {
        Self {
            res: false,
            rw: false,
            irq: true,
            db: 0,
            a: 0,
            pa: 0,
            pb: 0,
            phi2: false,
            cs1: false,
            cs2: false,
            rs: false,
            ram: [0; 128],
            ddra: 0,
            ddrb: 0,
            ora: 0,
            orb: 0,
            timer: timer::Timer::new(),
            irq_enabled: 0,
            irq_state: 0,
            pa7_active_edge_direction: false,
        }
    }

    pub fn is_selected(&self) -> bool {
        // To access chip, CS1 must be high and CS2 must be low.
        self.cs1 && !self.cs2
    }

    fn on_res_set(&mut self) {
        self.ddra = 0;
        self.ddrb = 0;
        self.ora = 0;
        self.orb = 0;

        // TODO: Reset timer.
    }

    fn on_phi2_transition_lo_to_hi(&mut self) {
        // Set IRQ pin based on interrupt flags.
        // The following condition tests whether one of the following are true:
        // - Timer interrupts are enabled, and the timer interrupt flag is set, or
        // - PA7 interrupts are enabled, and the PA7 interrupt flag is set.
        // Note that IRQ pin is active low.
        self.irq = (self.irq_state & self.irq_enabled) == 0;

        // To access chip, CS1 must be high and CS2 must be low.
        if self.is_selected() {
            if self.rs {
                // Access I/O registers or interval timer.
                if (self.a & 0x4) != 0 { // Check A2 pin
                    // Access interval timer.
                    if self.rw {
                        if self.a & 0x1 != 0 { // Check A0 pin
                            // Read interrupt flags.
                            self.db = self.irq_enabled;
                            self.irq_state &= !PA7_FLAG; // Clear PA7 flag
                        } else {
                            // Read timer.
                            self.db = self.timer.value();
                            if self.db != 0xFF {
                                self.irq_state &= !TIMER_FLAG; // Clear timer flag
                            }
                            if (self.a & 0x8) != 0 { // Check A3 pin
                                self.irq_enabled |= TIMER_FLAG;
                            } else {
                                self.irq_enabled &= !TIMER_FLAG;
                            }
                        }
                    } else {
                        if self.a & 0x10 != 0 { // Check A4 pin
                            // Write timer.
                            let interval_duration = M6532::get_interval_duration(self.a & 0x3); // A0 and A1 determine interval duration.
                            self.timer.reset(self.db, interval_duration);
                            if self.db != 0xFF {
                                self.irq_state &= !TIMER_FLAG; // Clear timer flag
                            }
                            if (self.a & 0x8) != 0 { // Check A3 pin
                                self.irq_enabled |= TIMER_FLAG;
                            } else {
                                self.irq_enabled &= !TIMER_FLAG;
                            }
                        } else {
                            // Write edge detect control.
                            if (self.a & 0x2) != 0 { // Check A1 pin
                                self.irq_enabled |= PA7_FLAG;
                            } else {
                                self.irq_enabled &= !PA7_FLAG;
                            }
                            self.pa7_active_edge_direction = (self.a & 0x1) != 0; // Check A0 pin
                        }
                    }
                } else {
                    // Access I/O registers.
                    let register = self.a & 0x3; // A0 and A1 determine register.
                    if self.rw {
                        // Read I/O registers.
                        self.read_io_register(register);
                    } else {
                        // Write I/O registers.
                        self.write_io_register(register);
                    }
                }
            } else {
                // Access RAM.
                if self.rw {
                    // Read RAM.
                    self.db = self.ram[self.a as usize];
                } else {
                    // Write RAM.
                    self.ram[self.a as usize] = self.db;
                }
            }
        }
    }

    /// According to the diagram on page 2-57 of the R6532 data sheet,
    /// the timer counts on the falling edge of phi2.
    fn on_phi2_transition_hi_to_lo(&mut self) {
        self.timer.tick();

        // Either the timer has just expired, or the timer had already expired.
        if self.timer.expired() {
            self.irq_state |= TIMER_FLAG;
        }
    }

    fn read_io_register(&mut self, register: u8) {
        self.db = match register {
            0b00 => self.ora,
            0b01 => self.ddra,
            0b10 => self.orb,
            0b11 => self.ddrb,
            _ => unreachable!()
        };
    }

    fn write_io_register(&mut self, register: u8) {
        match register {
            0b00 => self.ora = self.db,
            0b01 => self.ddra = self.db,
            0b10 => self.orb = self.db,
            0b11 => self.ddrb = self.db,
            _ => unreachable!()
        }
    }

    fn get_interval_duration(a1_a0: u8) -> u16 {
        match a1_a0 {
            0b00 => 1,
            0b01 => 8,
            0b10 => 64,
            0b11 => 1024,
            _ => unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_ram() {
        let mut chip = M6532::new();

        // Peek into chip to set test RAM data.
        chip.ram[10] = 42;

        chip.cs1 = true;
        chip.cs2 = false;
        chip.rs = false;
        chip.rw = true;

        chip.a = 10;

        chip.set_phi2(true);

        assert_eq!(42, chip.db);
    }

    #[test]
    fn write_ram() {
        let mut chip = M6532::new();

        chip.cs1 = true;
        chip.cs2 = false;
        chip.rs = false;
        chip.rw = false;

        chip.a = 10;
        chip.db = 42;

        chip.set_phi2(true);

        assert_eq!(42, chip.ram[10]);
    }

    #[test]
    fn interrupt_timing() {
        // This test is based on the example on page 9 of the MOS6532 data sheet:
        // http://archive.6502.org/datasheets/mos_6532_riot.pdf

        let mut chip = M6532::new();

        // Enable timer interrupts.
        chip.irq_enabled |= TIMER_FLAG;

        for i in 0..=500 {
            if i == 0 {
                // Write Timer 1T
                chip.cs1 = true;
                chip.cs2 = false;
                chip.rs = true;
                chip.rw = false;
                chip.a = 0b00011101; // Write Timer 1T, enable timer interrupts
                chip.db = 0x34; // 52
            } else {
                // Deselect chip.
                chip.cs1 = false;
                chip.cs2 = true;
            }

            chip.set_phi2(true);

            let timer_value = chip.timer.value();
            match i {
                0   => assert_eq!(0x33, timer_value),
                213 => assert_eq!(0x19, timer_value),
                415 => assert_eq!(0x00, timer_value),
                416 => assert_eq!(0xFF, timer_value),
                500 => assert_eq!(0xAB, timer_value), // Data sheet says it should be 0xAC?
                _ => {}
            }

            match i {
                0..=415 => assert_eq!(true, chip.irq),
                _       => assert_eq!(false, chip.irq)
            }

            chip.set_phi2(false);
        }
    }
}