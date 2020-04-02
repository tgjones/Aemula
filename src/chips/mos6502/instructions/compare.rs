use super::super::pins::Pins;
use super::super::registers::Registers;

pub(crate) fn bit(r: &mut Registers, pins: &Pins) {
    let value = pins.data;
    r.p.z = (r.a & value) == 0;
    r.p.v = (value & 0x40) == 0x40;
    r.p.n = (value & 0x80) == 0x80;
}

fn compare(r: &mut Registers, register: u8, pins: &Pins) {
    r.p.set_zero_negative_flags(register as i32 - pins.data as i32);
    r.p.c = register >= pins.data;
}

pub(crate) fn cmp(r: &mut Registers, pins: &Pins) {
    compare(r, r.a, pins);
}

pub(crate) fn cpx(r: &mut Registers, pins: &Pins) {
    compare(r, r.x, pins);
}

pub(crate) fn cpy(r: &mut Registers, pins: &Pins) {
    compare(r, r.y, pins);
}