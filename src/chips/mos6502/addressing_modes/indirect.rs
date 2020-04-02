use super::super::pins::Pins;
use super::super::registers::Registers;

/// Read low byte of target address.
pub(crate) fn addressing_mode_indirect_cycle_0(r: &mut Registers, pins: &mut Pins) {
    pins.set_address(&r.pc);
    r.pc = r.pc.wrapping_add(1);
}

/// Read high byte of target address.
pub(crate) fn addressing_mode_indirect_cycle_1(r: &mut Registers, pins: &mut Pins) {
    pins.set_address(&r.pc);
    r.pc = r.pc.wrapping_add(1);
    r.ad.lo = pins.data;
}

/// Read low byte of pointer stored at target address.
pub(crate) fn addressing_mode_indirect_cycle_2(r: &mut Registers, pins: &mut Pins) {
    r.ad.hi = pins.data;
    pins.set_address(&r.ad);
}

/// Read high byte of pointer stored at (target address + 1).
pub(crate) fn addressing_mode_indirect_cycle_3(r: &mut Registers, pins: &mut Pins) {
    pins.address_lo = r.ad.lo.wrapping_add(1);
    r.ad.lo = pins.data;
}

/// Read high byte of pointer stored at (target address + 1).
pub(crate) fn addressing_mode_indirect_cycle_4(r: &mut Registers, pins: &mut Pins) {
    pins.address_hi = pins.data;
    pins.address_lo = r.ad.lo;
}