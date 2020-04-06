use super::super::pins::Pins;
use super::super::registers::Registers;

pub(crate) fn and(r: &mut Registers, pins: &Pins) {
    r.a = r.p.set_zero_negative_flags((r.a & pins.data) as i32);
}

pub(crate) fn eor(r: &mut Registers, pins: &Pins) {
    r.a = r.p.set_zero_negative_flags((r.a ^ pins.data) as i32);
}

pub(crate) fn ora(r: &mut Registers, pins: &Pins) {
    r.a = r.p.set_zero_negative_flags((r.a | pins.data) as i32);
}
