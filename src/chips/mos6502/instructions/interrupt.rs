use super::super::pins::Pins;
use super::super::registers::{BrkFlags, Registers};

pub(crate) fn brk_0(r: &mut Registers, pins: &mut Pins) {
    if !r.brk_flags.contains(BrkFlags::NMI | BrkFlags::IRQ) { 
        r.pc = r.pc.wrapping_add(1);
    }
    pins.address_hi = 0x01;
    pins.address_lo = r.sp;
    r.sp = r.sp.wrapping_sub(1);
    pins.data = r.pc.hi;
    if !r.brk_flags.contains(BrkFlags::RESET) {
        pins.rw = false;
    }
}

pub(crate) fn brk_1(r: &mut Registers, pins: &mut Pins) {
    pins.address_hi = 0x01;
    pins.address_lo = r.sp;
    r.sp = r.sp.wrapping_sub(1);
    pins.data = r.pc.lo;
    if !r.brk_flags.contains(BrkFlags::RESET) {
        pins.rw = false;
    }
}

pub(crate) fn brk_2(r: &mut Registers, pins: &mut Pins) {
    pins.address_hi = 0x01;
    pins.address_lo = r.sp;
    r.sp = r.sp.wrapping_sub(1);
    pins.data = r.p.as_u8(r.brk_flags == BrkFlags::NONE); 
    r.ad.hi = 0xFF;
    if r.brk_flags.contains(BrkFlags::RESET) {
        r.ad.lo = 0xFC;
    } else {
        pins.rw = false; 
        if r.brk_flags.contains(BrkFlags::NMI) {
            r.ad.lo = 0xFA;
        } else {
            r.ad.lo = 0xFE;
        }
    }
}

pub(crate) fn brk_3(r: &mut Registers, pins: &mut Pins) {
    pins.address_hi = r.ad.hi;
    pins.address_lo = r.ad.lo;
    r.ad.lo += 1;
    r.p.i = true; 
    r.brk_flags = BrkFlags::NONE;
}

pub(crate) fn brk_4(r: &mut Registers, pins: &mut Pins) {
    pins.address_lo = r.ad.lo;
    r.ad.lo = pins.data;
}

pub(crate) fn brk_5(r: &mut Registers, pins: &mut Pins) {
    r.pc.hi = pins.data;
    r.pc.lo = r.ad.lo;
}