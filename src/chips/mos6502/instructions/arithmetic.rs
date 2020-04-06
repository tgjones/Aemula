use super::super::pins::Pins;
use super::super::registers::Registers;

fn do_adc_binary(r: &mut Registers, value: u8) {
    let temp_1 = (r.a as u16).wrapping_add(value as u16);
    let temp = if r.p.c { temp_1.wrapping_add(1) } else { temp_1 };
    r.p.v = ((r.a as u16 ^ temp) & (value as u16 ^ temp) & 0x80) == 0x80;
    r.p.c = temp > 0xFF;
    r.a = temp as u8;
    r.p.set_zero_negative_flags(r.a as i32);
}

fn do_adc_decimal(r: &mut Registers, value: u8) {
    let temp_1 = r.a.wrapping_add(value);
    let temp = if r.p.c { temp_1.wrapping_add(1) } else { temp_1 };

    r.p.z = temp == 0;

    let mut ah = 0;
    let mut al = (r.a & 0xF) + (value & 0xF) + (if r.p.c { 1 } else { 0 });
    if al > 9
    {
        al -= 10;
        al &= 0xF;
        ah = 1;
    }

    ah += (r.a >> 4) + (value >> 4);
    r.p.n = (ah & 8) == 8;
    r.p.v = ((r.a ^ value) & 0x80) == 0 && ((r.a ^ (ah << 4)) & 0x80) == 0x80;
    r.p.c = false;

    if ah > 9
    {
        r.p.c = true;
        ah -= 10;
        ah &= 0xF;
    }

    r.a = (al & 0xF) | (ah << 4);
}

pub(crate) fn adc(r: &mut Registers, pins: &Pins) {
    if !r.p.d || !r.bcd_enabled {
        do_adc_binary(r, pins.data);
    } else {
        do_adc_decimal(r, pins.data);
    }
}

fn do_sbc_decimal(r: &mut Registers, value: u8) {
    let value_u32 = value as u32;
    let carry = if r.p.c { 0 } else { 1 };
    let mut al = ((r.a & 0xF) as u32).wrapping_sub(value_u32 & 0xF).wrapping_sub(carry);
    let mut ah = ((r.a >> 4) as u32).wrapping_sub(value_u32 >> 4);

    if (al & 0x10) == 0x10 {
        al = (al - 6) & 0xF;
        ah = ah.wrapping_sub(1);
    }

    if (ah & 0x10) == 0x10 {
        ah = (ah - 6) & 0xF;
    }

    let result = (r.a as u32).wrapping_sub(value_u32).wrapping_sub(carry);
    r.p.n = (result & 0x80) == 0x80;
    r.p.z = (result & 0xFF) == 0;
    r.p.v = ((r.a as u32 ^ result) & (value_u32 ^ r.a as u32) & 0x80) == 0x80;
    r.p.c = (result & 0x100) == 0;
    r.a = (al | (ah << 4)) as u8;
}

pub(crate) fn sbc(r: &mut Registers, pins: &Pins) {
    if !r.p.d || !r.bcd_enabled {
        let value = !pins.data;
        do_adc_binary(r, value);
    } else {
        do_sbc_decimal(r, pins.data);
    }
}

pub(crate) fn dec(r: &mut Registers, pins: &mut Pins) {
    pins.data = r.p.set_zero_negative_flags(r.ad.lo.wrapping_sub(1) as i32);
    pins.rw = false;
}

pub(crate) fn dex(r: &mut Registers) {
    r.x = r.p.set_zero_negative_flags(r.x as i32 - 1);
}

pub(crate) fn dey(r: &mut Registers) {
    r.y = r.p.set_zero_negative_flags(r.y as i32 - 1);
}

pub(crate) fn inc(r: &mut Registers, pins: &mut Pins) {
    pins.data = r.p.set_zero_negative_flags(r.ad.lo.wrapping_add(1) as i32);
    pins.rw = false;
}

pub(crate) fn inx(r: &mut Registers) {
    r.x = r.p.set_zero_negative_flags(r.x as i32 + 1);
}

pub(crate) fn iny(r: &mut Registers) {
    r.y = r.p.set_zero_negative_flags(r.y as i32 + 1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(false, 0, 1, false, 1, false, false, false, false)]
    #[test_case(false, 0, 1, true, 2, false, false, false, false)]
    #[test_case(false, 255, 1, false, 0, true, true, false, false)]
    #[test_case(false, 127, 1, false, 128, false, false, true, true)]
    #[test_case(true, 0, 0, false, 0, false, true, false, false)]
    fn test_adc(bcd: bool, a: u8, addend: u8, c: bool, 
                expected_a: u8, expected_c: bool, expected_z: bool,
                expected_v: bool, expected_n: bool) {
        let mut registers = Registers::new(true);
        registers.p.d = bcd;
        registers.p.c = c;
        registers.a = a;

        let mut pins = Pins::new();
        pins.data = addend;

        adc(&mut registers, &mut pins);

        assert_eq!(expected_a, registers.a);

        assert_eq!(expected_c, registers.p.c);
        assert_eq!(expected_z, registers.p.z);
        assert_eq!(expected_v, registers.p.v);
        assert_eq!(expected_n, registers.p.n);
    }
}