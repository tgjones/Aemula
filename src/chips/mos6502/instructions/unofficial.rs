use super::super::pins::Pins;
use super::super::registers::Registers;

use super::*;

pub(crate) fn anc(r: &mut Registers, pins: &mut Pins) {
    r.a &= pins.data;
    r.p.set_zero_negative_flags(r.a as i32);
    r.p.c = r.a & 0x80 != 0;
}

pub(crate) fn ane(r: &mut Registers, pins: &mut Pins) {
    r.a = (r.a | 0xEE) & r.x & pins.data;
    r.p.set_zero_negative_flags(r.a as i32);
}

pub(crate) fn arr(r: &mut Registers, pins: &mut Pins) {
    and(r, pins);
    
    // http://www.6502.org/users/andre/petindex/local/64doc.txt
    if r.bcd_enabled && r.p.d {
        // Do ROR.
        let mut a = r.a >> 1;

        // Add carry flag to MSB.
        if r.p.c {
            a |= 0x80;
        }

        // Set zero and negative flags as normal.
        r.p.set_zero_negative_flags(a as i32);

        // The V flag will be set if the bit 6 of the accumulator changed its state
        // between the AND and the ROR, cleared otherwise.
        r.p.v = ((r.a ^ a) & 0x40) != 0;

        // Now it gets weird: if low nibble is greater than 5, increment it by 6,
        // but if it carries, don't add it to high nibble.
        if r.a & 0xF >= 5 {
            a = (a.wrapping_add(6) & 0xF) | (a & 0xF0);
        }

        // If high nibble is greater than 5, increment it by 6,
        // and set the carry flag.
        if (r.a & 0xF0) >= 0x50 {
            a = a.wrapping_add(0x60);
            r.p.c = true;
        } else {
            r.p.c = false;
        }

        r.a = a;        
    } else {
        rora(r);

        // The C flag is copied from bit 6 of the result.
        r.p.c = r.a & 0x40 != 0;

        // The V flag is the result of an XOR operation between bit 6 and bit 5 of the result.
        r.p.v = (((r.a & 0x40) >> 6) ^ ((r.a & 0x20) >> 5)) == 1;
    }
}

pub(crate) fn asr(r: &mut Registers, pins: &mut Pins) {
    and(r, pins);
    lsra(r);
}

pub(crate) fn dcp(r: &mut Registers, pins: &mut Pins) {
    dec(r, pins);
    cmp(r, pins);
}

pub(crate) fn isb(r: &mut Registers, pins: &mut Pins) {
    inc(r, pins);
    sbc(r, pins);
}

pub(crate) fn las(r: &mut Registers, pins: &mut Pins) {
    r.a = pins.data & r.sp;
    r.x = r.a;
    r.sp = r.a;
    r.p.set_zero_negative_flags(r.a as i32);
}

pub(crate) fn lxa(r: &mut Registers, pins: &mut Pins) {
    r.a = (r.a | 0xEE) & pins.data;
    r.x = r.a;
    r.p.set_zero_negative_flags(r.a as i32);
}

pub(crate) fn rla(r: &mut Registers, pins: &mut Pins) {
    rol(r, pins);
    and(r, pins);
}

pub(crate) fn rra(r: &mut Registers, pins: &mut Pins) {
    ror(r, pins);
    adc(r, pins);
}

pub(crate) fn sbx(r: &mut Registers, pins: &mut Pins) {
    let (new_x, overflowed) = (r.a & r.x).overflowing_sub(pins.data);
    r.x = new_x;
    r.p.c = !overflowed;
    r.p.set_zero_negative_flags(r.x as i32);
}

pub(crate) fn sha(r: &Registers, pins: &mut Pins) {
    pins.data = r.a & r.x & pins.address_hi.wrapping_add(1);
    pins.rw = false;
}

pub(crate) fn shs(r: &mut Registers, pins: &mut Pins) {
    r.sp = r.a & r.x;
    sha(r, pins);
}

pub(crate) fn shx(r: &Registers, pins: &mut Pins) {
    pins.data = r.x & pins.address_hi.wrapping_add(1);
    pins.rw = false;
}

pub(crate) fn shy(r: &Registers, pins: &mut Pins) {
    pins.data = r.y & pins.address_hi.wrapping_add(1);
    pins.rw = false;
}

pub(crate) fn slo(r: &mut Registers, pins: &mut Pins) {
    asl(r, pins);
    ora(r, pins);
}

pub(crate) fn sre(r: &mut Registers, pins: &mut Pins) {
    lsr(r, pins);
    eor(r, pins);
}