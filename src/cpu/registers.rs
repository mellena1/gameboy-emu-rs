use super::flags_register::FlagsRegister;

macro_rules! register_getter {
    ( $x:expr, $y:expr ) => {
        ($x as u16) << 8 | $y as u16
    };
}

macro_rules! register_setter {
    ( $x:expr, $y:expr, $value:expr ) => {
        $x = ($value >> 8) as u8;
        $y = (($value & 0xFF) as u8).into();
    };
}

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagsRegister,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagsRegister::new(),
            h: 0,
            l: 0,
        }
    }

    pub fn get_af(&self) -> u16 {
        register_getter!(self.a, u8::from(self.f))
    }

    pub fn set_af(&mut self, value: u16) {
        register_setter!(self.a, self.f, value);
    }

    pub fn get_bc(&self) -> u16 {
        register_getter!(self.b, self.c)
    }

    pub fn set_bc(&mut self, value: u16) {
        register_setter!(self.b, self.c, value);
    }

    pub fn get_de(&self) -> u16 {
        register_getter!(self.d, self.e)
    }

    pub fn set_de(&mut self, value: u16) {
        register_setter!(self.d, self.e, value);
    }

    pub fn get_hl(&self) -> u16 {
        register_getter!(self.h, self.l)
    }

    pub fn set_hl(&mut self, value: u16) {
        register_setter!(self.h, self.l, value);
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_set_af() {
        let mut r = Registers::new();
        assert_eq!(r.get_af(), 0u16);

        r.set_af(0x0FF0);
        assert_eq!(r.a, 0x0F);
        assert_eq!(
            r.f, FlagsRegister {
                zero: true,
                subtract: true,
                half_carry: true,
                carry: true
            }
        );
        assert_eq!(r.get_af(), 0x0FF0);
    }

    #[test]
    fn get_set_bc() {
        let mut r = Registers::new();
        assert_eq!(r.get_bc(), 0u16);

        r.set_bc(0xF00F);
        assert_eq!(r.b, 0xF0);
        assert_eq!(r.c, 0x0F);
        assert_eq!(r.get_bc(), 0xF00F);
    }

    #[test]
    fn get_set_de() {
        let mut r = Registers::new();
        assert_eq!(r.get_de(), 0u16);

        r.set_de(0xF00F);
        assert_eq!(r.d, 0xF0);
        assert_eq!(r.e, 0x0F);
        assert_eq!(r.get_de(), 0xF00F);
    }

    #[test]
    fn get_set_hl() {
        let mut r = Registers::new();
        assert_eq!(r.get_hl(), 0u16);

        r.set_hl(0xF00F);
        assert_eq!(r.h, 0xF0);
        assert_eq!(r.l, 0x0F);
        assert_eq!(r.get_hl(), 0xF00F);
    }
}
