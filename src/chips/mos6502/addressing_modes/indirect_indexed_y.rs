use super::super::pins::Pins;
use super::super::registers::Registers;

pub(crate) fn addressing_mode_indirect_indexed_y_cycle_0(r: &mut Registers, pins: &mut Pins) {
    pins.set_address(&r.pc);
    r.pc = r.pc.wrapping_add(1);
}

pub(crate) fn addressing_mode_indirect_indexed_y_cycle_1(r: &mut Registers, pins: &mut Pins) {
    r.ad.hi = 0;
    r.ad.lo = pins.data;
    pins.set_address(&r.ad);
}

pub(crate) fn addressing_mode_indirect_indexed_y_cycle_2(r: &mut Registers, pins: &mut Pins) {
    pins.address_lo = r.ad.lo.wrapping_add(1);
    r.ad.lo = pins.data;
}

pub(crate) fn addressing_mode_indirect_indexed_y_cycle_3(r: &mut Registers, pins: &mut Pins) {
    r.ad.hi = pins.data;
    pins.address_hi = r.ad.hi;
    pins.address_lo = r.ad.lo.wrapping_add(r.y);
}

pub(crate) fn addressing_mode_indirect_indexed_y_cycle_3_read(r: &mut Registers) {
    let without_carry = r.ad.hi;
    let with_carry = r.ad.wrapping_add(r.y).hi;
    if without_carry == with_carry {
        r.tr += 1;
    }
}

/// This cycle can be skipped for read access if page boundary is not crossed.
pub(crate) fn addressing_mode_indirect_indexed_y_cycle_4(r: &mut Registers, pins: &mut Pins) {
    pins.set_address(&r.ad.wrapping_add(r.y));
}