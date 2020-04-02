use super::super::registers::Registers;

pub(crate) fn clc(r: &mut Registers) {
    r.p.c = false;
}

pub(crate) fn cld(r: &mut Registers) {
    r.p.d = false;
}

pub(crate) fn cli(r: &mut Registers) {
    r.p.i = false;
}

pub(crate) fn clv(r: &mut Registers) {
    r.p.v = false;
}

pub(crate) fn sed(r: &mut Registers) {
    r.p.d = true;
}

pub(crate) fn sei(r: &mut Registers) {
    r.p.i = true;
}

pub(crate) fn slc(r: &mut Registers) {
    r.p.c = true;
}