use super::super::pins::Pins;
use super::super::registers::Registers;

pub(crate) fn addressing_mode_none_cycle_0(r: &mut Registers, pins: &mut Pins) {
    pins.set_address(&r.pc);
}