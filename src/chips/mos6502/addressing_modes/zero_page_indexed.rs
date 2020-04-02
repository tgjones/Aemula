use super::super::pins::Pins;
use super::super::registers::Registers;

pub(crate) fn addressing_mode_zero_page_indexed_cycle_0(r: &mut Registers, pins: &mut Pins) {
    pins.set_address(&r.pc);
    r.pc = r.pc.wrapping_add(1);
}

pub(crate) fn addressing_mode_zero_page_indexed_cycle_1(r: &mut Registers, pins: &mut Pins) {
    r.ad.hi = 0;
    r.ad.lo = pins.data;
    pins.set_address(&r.ad);
}

fn addressing_mode_zero_page_indexed_cycle_2(r: &mut Registers, pins: &mut Pins, index_register_value: u8) {
    pins.address_hi = 0;
    pins.address_lo = r.ad.lo.wrapping_add(index_register_value);
}

pub(crate) fn addressing_mode_zero_page_x_cycle_2(r: &mut Registers, pins: &mut Pins) {
    addressing_mode_zero_page_indexed_cycle_2(r, pins, r.x);
}

pub(crate) fn addressing_mode_zero_page_y_cycle_2(r: &mut Registers, pins: &mut Pins) {
    addressing_mode_zero_page_indexed_cycle_2(r, pins, r.y);
}