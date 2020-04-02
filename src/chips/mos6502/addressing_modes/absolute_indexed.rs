use super::super::pins::Pins;
use super::super::registers::Registers;

/// Set address bus to PC (to fetch BAL, low byte of base address), increment PC.
pub(crate) fn addressing_mode_absolute_indexed_cycle_0(r: &mut Registers, pins: &mut Pins) {
    pins.set_address(&r.pc);
    r.pc = r.pc.wrapping_add(1);
}

/// Set address bus to PC (to fetch BAH, high byte of base address), increment PC.
pub(crate) fn addressing_mode_absolute_indexed_cycle_1(r: &mut Registers, pins: &mut Pins) {
    pins.set_address(&r.pc);
    r.pc = r.pc.wrapping_add(1);
    r.ad.lo = pins.data;
}

/// Set address bus to BAH,BAL+index
pub(crate) fn addressing_mode_absolute_indexed_cycle_2(index_register_value: u8, r: &mut Registers, pins: &mut Pins) {
    r.ad.hi = pins.data;
    pins.address_hi = r.ad.hi;
    pins.address_lo = r.ad.lo.wrapping_add(index_register_value);
}

/// If, when the index register is added to BAL (the low byte of the base address),
/// the resulting address is on the same page, then we skip the next cycle.
/// 
/// Otherwise if it's on the next page, then we execute an extra cycle
/// to add the carry value to BAH (the high byte of the base address).
/// 
/// This conditional check only happens for instructions that read memory.
/// For instructions that write to memory, we always execute the extra cycle.
pub(crate) fn addressing_mode_absolute_indexed_cycle_2_read(index_register_value: u8, r: &mut Registers) {
    let without_carry = r.ad.hi;
    let with_carry = r.ad.wrapping_add(index_register_value).hi;
    if without_carry == with_carry {
        r.tr += 1;
    }
}

pub(crate) fn addressing_mode_absolute_indexed_cycle_3(index_register_value: u8, r: &mut Registers, pins: &mut Pins) {
    pins.set_address(&r.ad.wrapping_add(index_register_value));
}