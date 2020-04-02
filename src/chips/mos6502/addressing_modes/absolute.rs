use super::super::pins::Pins;
use super::super::registers::Registers;

pub(crate) fn addressing_mode_absolute_cycle_0(r: &mut Registers, pins: &mut Pins) {
    pins.set_address(&r.pc);
    r.pc = r.pc.wrapping_add(1);
}

pub(crate) fn addressing_mode_absolute_cycle_1(r: &mut Registers, pins: &mut Pins) {
    pins.set_address(&r.pc);
    r.pc = r.pc.wrapping_add(1);
    r.ad.lo = pins.data;
}

pub(crate) fn addressing_mode_absolute_cycle_2(r: &mut Registers, pins: &mut Pins) {
    pins.address_hi = pins.data;
    pins.address_lo = r.ad.lo;
}
