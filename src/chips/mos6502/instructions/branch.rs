use super::super::pins::Pins;
use super::super::registers::Registers;

fn branch_0(r: &mut Registers, pins: &mut Pins, flag: bool, value: bool) {
    pins.set_address(&r.pc);
    r.ad = r.pc.wrapping_add_i8(pins.data as i8);
    // If branch was not taken, fetch next instruction.
    if flag != value {
        pins.fetch_next_instruction(r);
    }
}

pub(crate) fn branch_0_bcc(r: &mut Registers, pins: &mut Pins) {
    branch_0(r, pins, r.p.c, false);
}

pub(crate) fn branch_0_bcs(r: &mut Registers, pins: &mut Pins) {
    branch_0(r, pins, r.p.c, true);
}

pub(crate) fn branch_0_beq(r: &mut Registers, pins: &mut Pins) {
    branch_0(r, pins, r.p.z, true);
}

pub(crate) fn branch_0_bmi(r: &mut Registers, pins: &mut Pins) {
    branch_0(r, pins, r.p.n, true);
}

pub(crate) fn branch_0_bne(r: &mut Registers, pins: &mut Pins) {
    branch_0(r, pins, r.p.z, false);
}

pub(crate) fn branch_0_bpl(r: &mut Registers, pins: &mut Pins) {
    branch_0(r, pins, r.p.n, false);
}

pub(crate) fn branch_0_bvc(r: &mut Registers, pins: &mut Pins) {
    branch_0(r, pins, r.p.v, false);
}

pub(crate) fn branch_0_bvs(r: &mut Registers, pins: &mut Pins) {
    branch_0(r, pins, r.p.v, true);
}

/// Executed if branch was taken.
pub(crate) fn branch_1(r: &mut Registers, pins: &mut Pins) {
    pins.address_lo = r.ad.lo;
    if r.ad.hi == r.pc.hi { // Are we branching to the same page?
        r.pc = r.ad;
        pins.fetch_next_instruction(r);
    }
}

/// Only executed if page was crossed.
pub(crate) fn branch_2(r: &mut Registers) {
    r.pc = r.ad;
}

pub(crate) fn jmp(r: &mut Registers, pins: &Pins) {
    r.pc.hi = pins.address_hi;
    r.pc.lo = pins.address_lo;
}