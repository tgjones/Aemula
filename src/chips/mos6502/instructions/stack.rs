use super::super::pins::Pins;
use super::super::registers::Registers;

pub(crate) fn pha(r: &mut Registers, pins: &mut Pins) {
    pins.address_hi = 0x01;
    pins.address_lo = r.sp;
    r.sp = r.sp.wrapping_sub(1);
    pins.data = r.a;
    pins.rw = false;
}

pub(crate) fn php(r: &mut Registers, pins: &mut Pins) {
    pins.address_hi = 0x01;
    pins.address_lo = r.sp;
    r.sp = r.sp.wrapping_sub(1);
    pins.data = r.p.as_u8(true);
    pins.rw = false;
}

pub(crate) fn pla_0(r: &mut Registers, pins: &mut Pins) {
    pins.address_hi = 0x01;
    pins.address_lo = r.sp;
    r.sp = r.sp.wrapping_add(1);
}

pub(crate) fn pla_1(r: &mut Registers, pins: &mut Pins) {
    pins.address_hi = 0x01;
    pins.address_lo = r.sp;
}

pub(crate) fn pla_2(r: &mut Registers, pins: &mut Pins) {
    r.a = r.p.set_zero_negative_flags(pins.data as i32);
}

pub(crate) fn plp_0(r: &mut Registers, pins: &mut Pins) {
    pins.address_hi = 0x01;
    pins.address_lo = r.sp;
    r.sp = r.sp.wrapping_add(1);
}

pub(crate) fn plp_1(r: &mut Registers, pins: &mut Pins) {
    pins.address_hi = 0x01;
    pins.address_lo = r.sp;
}

pub(crate) fn plp_2(r: &mut Registers, pins: &mut Pins) {
    let temp = r.p.set_zero_negative_flags(pins.data as i32);
    r.p.set_from_u8(temp);
}