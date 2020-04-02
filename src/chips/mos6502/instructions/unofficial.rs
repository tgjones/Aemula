use super::super::pins::Pins;
use super::super::registers::Registers;

use super::*;

pub(crate) fn dcp(r: &mut Registers, pins: &mut Pins) {
    dec(r, pins);
    cmp(r, pins);
}

pub(crate) fn isb(r: &mut Registers, pins: &mut Pins) {
    inc(r, pins);
    sbc(r, pins);
}

pub(crate) fn rla(r: &mut Registers, pins: &mut Pins) {
    rol(r, pins);
    and(r, pins);
}

pub(crate) fn rra(r: &mut Registers, pins: &mut Pins) {
    ror(r, pins);
    adc(r, pins);
}

pub(crate) fn slo(r: &mut Registers, pins: &mut Pins) {
    asl(r, pins);
    ora(r, pins);
}

pub(crate) fn sre(r: &mut Registers, pins: &mut Pins) {
    lsr(r, pins);
    eor(r, pins);
}