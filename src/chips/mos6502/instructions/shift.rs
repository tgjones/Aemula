use super::super::pins::Pins;
use super::super::registers::Registers;

fn asl(r: &mut Registers, value: u8) -> u8 {
    r.p.c = (value & 0x80) == 0x80;
    r.p.set_zero_negative_flags((value as i32) << 1)
}

pub(crate) fn asl_0(r: &mut Registers, pins: &mut Pins) {
    r.ad.lo = pins.data;
    pins.rw = false;
}

pub(crate) fn asl_1(r: &mut Registers, pins: &mut Pins) {
    pins.data = asl(r, r.ad.lo);
    pins.rw = false;
}

pub(crate) fn asla(r: &mut Registers) {
    r.a = asl(r, r.a);
}

pub(crate) fn lsr_0(r: &mut Registers, pins: &mut Pins) {
    r.ad.lo = pins.data;
    pins.rw = false;
}

fn lsr(r: &mut Registers, value: u8) -> u8 {
    r.p.c = (value & 0x1) == 0x1;
    r.p.set_zero_negative_flags((value as i32) >> 1)
}

pub(crate) fn lsr_1(r: &mut Registers, pins: &mut Pins) {
    pins.data = lsr(r, r.ad.lo);
    pins.rw = false;
}

pub(crate) fn lsra(r: &mut Registers) {
    r.a = lsr(r, r.a);
}

pub(crate) fn rol_0(r: &mut Registers, pins: &mut Pins) {
    r.ad.lo = pins.data;
    pins.rw = false;
}

fn rol(r: &mut Registers, value: u8) -> u8 {
    let temp = if r.p.c { 1 } else { 0 };
    r.p.c = (value & 0x80) == 0x80;
    r.p.set_zero_negative_flags(((value << 1) | temp) as i32)
}

pub(crate) fn rol_1(r: &mut Registers, pins: &mut Pins) {
    pins.data = rol(r, r.ad.lo);
    pins.rw = false;
}

pub(crate) fn rola(r: &mut Registers) {
    r.a = rol(r, r.a);
}

pub(crate) fn ror_0(r: &mut Registers, pins: &mut Pins) {
    r.ad.lo = pins.data;
    pins.rw = false;
}

fn ror(r: &mut Registers, value: u8) -> u8 {
    let temp = (if r.p.c { 1 } else { 0 }) << 7;
    r.p.c = (value & 0x1) == 0x1;
    r.p.set_zero_negative_flags(((value >> 1) | temp) as i32)
}

pub(crate) fn ror_1(r: &mut Registers, pins: &mut Pins) {
    pins.data = ror(r, r.ad.lo);
    pins.rw = false;
}

pub(crate) fn rora(r: &mut Registers) {
    r.a = ror(r, r.a);
}