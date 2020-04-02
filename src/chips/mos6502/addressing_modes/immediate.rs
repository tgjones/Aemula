use super::super::pins::Pins;
use super::super::registers::Registers;

pub(crate) fn addressing_mode_immediate_cycle_0(r: &mut Registers, pins: &mut Pins) {
    pins.set_address(&r.pc);
    r.pc = r.pc.wrapping_add(1);
}