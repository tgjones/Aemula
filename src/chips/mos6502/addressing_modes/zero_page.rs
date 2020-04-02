use super::super::pins::Pins;
use super::super::registers::Registers;

pub(crate) fn addressing_mode_zero_page_cycle_0(r: &mut Registers, pins: &mut Pins) {
    pins.set_address(&r.pc);
    r.pc = r.pc.wrapping_add(1);
}

pub(crate) fn addressing_mode_zero_page_cycle_1(pins: &mut Pins) {
    pins.address_hi = 0;
    pins.address_lo = pins.data;
}