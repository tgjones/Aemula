use super::super::MOS6502;

impl MOS6502 {
    fn do_adc_binary(&mut self, value: u8) {
        let temp_1 = (self.a as u16).wrapping_add(value as u16);
        let temp = if self.p.c { temp_1.wrapping_add(1) } else { temp_1 };
        self.p.v = ((self.a as u16 ^ temp) & (value as u16 ^ temp) & 0x80) == 0x80;
        self.p.c = temp > 0xFF;
        self.a = temp as u8;
        self.p.set_zero_negative_flags(self.a as i32);
    }

    fn do_adc_decimal(&mut self, value: u8) {
        let temp_1 = self.a.wrapping_add(value);
        let temp = if self.p.c { temp_1.wrapping_add(1) } else { temp_1 };

        self.p.z = temp == 0;

        let mut ah = 0;
        let mut al = (self.a & 0xF) + (value & 0xF) + (if self.p.c { 1 } else { 0 });
        if al > 9
        {
            al -= 10;
            al &= 0xF;
            ah = 1;
        }

        ah += (self.a >> 4) + (value >> 4);
        self.p.n = (ah & 8) == 8;
        self.p.v = ((self.a ^ value) & 0x80) == 0 && ((self.a ^ (ah << 4)) & 0x80) == 0x80;
        self.p.c = false;

        if ah > 9
        {
            self.p.c = true;
            ah -= 10;
            ah &= 0xF;
        }

        self.a = (al & 0xF) | (ah << 4);
    }

    pub(crate) fn adc(&mut self) {
        if !self.p.d || !self.bcd_enabled {
            self.do_adc_binary(self.data);
        } else {
            self.do_adc_decimal(self.data);
        }
    }

    fn do_sbc_decimal(&mut self, value: u8) {
        let value_u32 = value as u32;
        let carry = if self.p.c { 0 } else { 1 };
        let mut al = ((self.a & 0xF) as u32).wrapping_sub(value_u32 & 0xF).wrapping_sub(carry);
        let mut ah = ((self.a >> 4) as u32).wrapping_sub(value_u32 >> 4);

        if (al & 0x10) == 0x10 {
            al = (al - 6) & 0xF;
            ah = ah.wrapping_sub(1);
        }

        if (ah & 0x10) == 0x10 {
            ah = (ah - 6) & 0xF;
        }

        let result = (self.a as u32).wrapping_sub(value_u32).wrapping_sub(carry);
        self.p.n = (result & 0x80) == 0x80;
        self.p.z = (result & 0xFF) == 0;
        self.p.v = ((self.a as u32 ^ result) & (value_u32 ^ self.a as u32) & 0x80) == 0x80;
        self.p.c = (result & 0x100) == 0;
        self.a = (al | (ah << 4)) as u8;
    }

    pub(crate) fn sbc(&mut self) {
        if !self.p.d || !self.bcd_enabled {
            let value = !self.data;
            self.do_adc_binary(value);
        } else {
            self.do_sbc_decimal(self.data);
        }
    }

    pub(crate) fn dec(&mut self) {
        self.data = self.p.set_zero_negative_flags(self.ad.lo.wrapping_sub(1) as i32);
        self.rw = false;
    }

    pub(crate) fn dex(&mut self) {
        self.x = self.p.set_zero_negative_flags(self.x as i32 - 1);
    }

    pub(crate) fn dey(&mut self) {
        self.y = self.p.set_zero_negative_flags(self.y as i32 - 1);
    }

    pub(crate) fn inc(&mut self) {
        self.data = self.p.set_zero_negative_flags(self.ad.lo.wrapping_add(1) as i32);
        self.rw = false;
    }

    pub(crate) fn inx(&mut self) {
        self.x = self.p.set_zero_negative_flags(self.x as i32 + 1);
    }

    pub(crate) fn iny(&mut self) {
        self.y = self.p.set_zero_negative_flags(self.y as i32 + 1);
    }
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
        let mut cpu = MOS6502::new();

        cpu.p.d = bcd;
        cpu.p.c = c;
        cpu.a = a;

        cpu.data = addend;

        cpu.adc();

        assert_eq!(expected_a, cpu.a);

        assert_eq!(expected_c, cpu.p.c);
        assert_eq!(expected_z, cpu.p.z);
        assert_eq!(expected_v, cpu.p.v);
        assert_eq!(expected_n, cpu.p.n);
    }
}