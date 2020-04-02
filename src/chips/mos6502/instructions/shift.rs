use super::super::pins::Pins;
use super::super::registers::Registers;

fn asl_helper(r: &mut Registers, value: u8) -> u8 {
    r.p.c = (value & 0x80) == 0x80;
    r.p.set_zero_negative_flags((value as i32) << 1)
}

pub(crate) fn asl(r: &mut Registers, pins: &mut Pins) {
    pins.data = asl_helper(r, r.ad.lo);
    pins.rw = false;
}

pub(crate) fn asla(r: &mut Registers) {
    r.a = asl_helper(r, r.a);
}

fn lsr_helper(r: &mut Registers, value: u8) -> u8 {
    r.p.c = (value & 0x1) == 0x1;
    r.p.set_zero_negative_flags((value as i32) >> 1)
}

pub(crate) fn lsr(r: &mut Registers, pins: &mut Pins) {
    pins.data = lsr_helper(r, r.ad.lo);
    pins.rw = false;
}

pub(crate) fn lsra(r: &mut Registers) {
    r.a = lsr_helper(r, r.a);
}

fn rol_helper(r: &mut Registers, value: u8) -> u8 {
    let temp = if r.p.c { 1 } else { 0 };
    r.p.c = (value & 0x80) == 0x80;
    r.p.set_zero_negative_flags(((value << 1) | temp) as i32)
}

pub(crate) fn rol(r: &mut Registers, pins: &mut Pins) {
    pins.data = rol_helper(r, r.ad.lo);
    pins.rw = false;
}

pub(crate) fn rola(r: &mut Registers) {
    r.a = rol_helper(r, r.a);
}

fn ror_helper(r: &mut Registers, value: u8) -> u8 {
    let temp = (if r.p.c { 1 } else { 0 }) << 7;
    r.p.c = (value & 0x1) == 0x1;
    r.p.set_zero_negative_flags(((value >> 1) | temp) as i32)
}

pub(crate) fn ror(r: &mut Registers, pins: &mut Pins) {
    pins.data = ror_helper(r, r.ad.lo);
    pins.rw = false;
}

pub(crate) fn rora(r: &mut Registers) {
    r.a = ror_helper(r, r.a);
}