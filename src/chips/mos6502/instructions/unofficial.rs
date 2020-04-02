use super::super::pins::Pins;
use super::super::registers::Registers;

use super::*;

pub(crate) fn dcp_0(r: &mut Registers, pins: &mut Pins) {
    dec_0(r, pins);
}

pub(crate) fn dcp_1(r: &mut Registers, pins: &mut Pins) {
    dec_1(r, pins);
    cmp(r, pins);
}

pub(crate) fn isb_0(r: &mut Registers, pins: &mut Pins) {
    inc_0(r, pins);
}

pub(crate) fn isb_1(r: &mut Registers, pins: &mut Pins) {
    inc_1(r, pins);
    sbc(r, pins);
}

pub(crate) fn rla_0(r: &mut Registers, pins: &mut Pins) {
    rol_0(r, pins);
}

pub(crate) fn rla_1(r: &mut Registers, pins: &mut Pins) {
    rol_1(r, pins);
    and(r, pins);
}

pub(crate) fn rra_0(r: &mut Registers, pins: &mut Pins) {
    ror_0(r, pins);
}

pub(crate) fn rra_1(r: &mut Registers, pins: &mut Pins) {
    ror_1(r, pins);
    adc(r, pins);
}

pub(crate) fn slo_0(r: &mut Registers, pins: &mut Pins) {
    asl_0(r, pins);
}

pub(crate) fn slo_1(r: &mut Registers, pins: &mut Pins) {
    asl_1(r, pins);
    ora(r, pins);
}

pub(crate) fn sre_0(r: &mut Registers, pins: &mut Pins) {
    lsr_0(r, pins);
}

pub(crate) fn sre_1(r: &mut Registers, pins: &mut Pins) {
    lsr_1(r, pins);
    eor(r, pins);
}