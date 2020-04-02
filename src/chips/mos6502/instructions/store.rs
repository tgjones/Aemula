use super::super::pins::Pins;
use super::super::registers::Registers;

pub(crate) fn sax(r: &mut Registers, pins: &mut Pins) {
    pins.data = r.a & r.x;
    pins.rw = false;
}

pub(crate) fn sta(r: &mut Registers, pins: &mut Pins) {
    pins.data = r.a;
    pins.rw = false;
}

pub(crate) fn stx(r: &mut Registers, pins: &mut Pins) {
    pins.data = r.x;
    pins.rw = false;
}

pub(crate) fn sty(r: &mut Registers, pins: &mut Pins) {
    pins.data = r.y;
    pins.rw = false;
}