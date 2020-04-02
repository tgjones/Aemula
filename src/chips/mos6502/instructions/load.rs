use super::super::pins::Pins;
use super::super::registers::Registers;

pub(crate) fn lax(r: &mut Registers, pins: &Pins) {
    lda(r, pins);
    ldx(r, pins);
}

pub(crate) fn lda(r: &mut Registers, pins: &Pins) {
    r.a = r.p.set_zero_negative_flags(pins.data as i32);
}

pub(crate) fn ldx(r: &mut Registers, pins: &Pins) {
    r.x = r.p.set_zero_negative_flags(pins.data as i32);
}

pub(crate) fn ldy(r: &mut Registers, pins: &Pins) {
    r.y = r.p.set_zero_negative_flags(pins.data as i32);
}