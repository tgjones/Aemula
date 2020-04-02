use super::super::pins::Pins;
use super::super::registers::Registers;

/// Read low byte of target address.
pub(crate) fn jsr_0(r: &mut Registers, pins: &mut Pins) {
    pins.set_address(&r.pc);
    r.pc = r.pc.wrapping_add(1);
}

/// Put SP on address bus.
pub(crate) fn jsr_1(r: &mut Registers, pins: &mut Pins) {
    pins.address_hi = 0x01;
    pins.address_lo = r.sp;
    r.ad.lo = pins.data;
}

/// Write PC high byte to stack.
pub(crate) fn jsr_2(r: &mut Registers, pins: &mut Pins) {
    pins.address_hi = 0x01;
    pins.address_lo = r.sp;
    r.sp = r.sp.wrapping_sub(1);
    pins.data = r.pc.hi;
    pins.rw = false;
}

/// Write PC low byte to stack.
pub(crate) fn jsr_3(r: &mut Registers, pins: &mut Pins) {
    pins.address_hi = 0x01;
    pins.address_lo = r.sp;
    r.sp = r.sp.wrapping_sub(1);
    pins.data = r.pc.lo;
    pins.rw = false;
}

/// Read high byte of target address.
pub(crate) fn jsr_4(r: &mut Registers, pins: &mut Pins) {
    pins.set_address(&r.pc);
}

pub(crate) fn jsr_5(r: &mut Registers, pins: &mut Pins) {
    r.pc.hi = pins.data;
    r.pc.lo = r.ad.lo;
}

pub(crate) fn rti_0(r: &mut Registers, pins: &mut Pins) {
    pins.address_hi = 0x01;
    pins.address_lo = r.sp;
    r.sp = r.sp.wrapping_add(1);
}

pub(crate) fn rti_1(r: &mut Registers, pins: &mut Pins) {
    pins.address_lo = r.sp;
    r.sp = r.sp.wrapping_add(1);
}

pub(crate) fn rti_2(r: &mut Registers, pins: &mut Pins) {
    pins.address_lo = r.sp;
    r.sp = r.sp.wrapping_add(1);
    r.p.set_from_u8(pins.data);
}

pub(crate) fn rti_3(r: &mut Registers, pins: &mut Pins) {
    pins.address_lo = r.sp;
    r.ad.lo = pins.data;
}

pub(crate) fn rti_4(r: &mut Registers, pins: &mut Pins) {
    r.pc.hi = pins.data;
    r.pc.lo = r.ad.lo;
}

pub(crate) fn rts_0(r: &mut Registers, pins: &mut Pins) {
    pins.address_hi = 0x01;
    pins.address_lo = r.sp;
    r.sp = r.sp.wrapping_add(1);
}

pub(crate) fn rts_1(r: &mut Registers, pins: &mut Pins) {
    pins.address_lo = r.sp;
    r.sp = r.sp.wrapping_add(1);
}

pub(crate) fn rts_2(r: &mut Registers, pins: &mut Pins) {
    pins.address_lo = r.sp;
    r.ad.lo = pins.data;
}

pub(crate) fn rts_3(r: &mut Registers, pins: &mut Pins) {
    r.pc.hi = pins.data;
    r.pc.lo = r.ad.lo;
    pins.set_address(&r.pc);
    r.pc = r.pc.wrapping_add(1);
}