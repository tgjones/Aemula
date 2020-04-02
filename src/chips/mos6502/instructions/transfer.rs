use super::super::registers::Registers;

pub(crate) fn tax(r: &mut Registers) {
    r.x = r.p.set_zero_negative_flags(r.a as i32);
}

pub(crate) fn tay(r: &mut Registers) {
    r.y = r.p.set_zero_negative_flags(r.a as i32);
}

pub(crate) fn tsx(r: &mut Registers) {
    r.x = r.p.set_zero_negative_flags(r.sp as i32);
}

pub(crate) fn txa(r: &mut Registers) {
    r.a = r.p.set_zero_negative_flags(r.x as i32);
}

pub(crate) fn txs(r: &mut Registers) {
    r.sp = r.x;
}

pub(crate) fn tya(r: &mut Registers) {
    r.a = r.p.set_zero_negative_flags(r.y as i32);
}