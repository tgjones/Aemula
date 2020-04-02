use super::super::pins::Pins;
use super::super::registers::Registers;

pub(crate) fn addressing_mode_invalid_cycle_0(r: &mut Registers, pins: &mut Pins) {
    pins.set_address(&r.pc);
}

pub(crate) fn addressing_mode_invalid_cycle_1(r: &mut Registers, pins: &mut Pins) {
    pins.address_hi = 0xFF;
    pins.address_lo = 0xFF;
    pins.data = 0xFF;
    r.ir -= 1;
}