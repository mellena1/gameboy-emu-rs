use super::flags_register::FlagsRegister;

#[derive(Debug)]
pub enum Instruction {
    LD(LoadType),
    PUSH(PushPopTarget),
    POP(PushPopTarget),
    ADD(ArithmeticTarget),
    NOP(),
    HALT(),
    JP(JumpTest),
    CALL(JumpTest),
    RET(JumpTest),
}

#[derive(Debug)]
pub enum PushPopTarget {
    AF,
    BC,
    DE,
    HL,
}

#[derive(Debug)]
pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug)]
pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

impl JumpTest {
    pub fn condition_depending_on_flags_reg(&self, f: FlagsRegister) -> bool {
        match self {
            JumpTest::NotZero => !f.zero,
            JumpTest::Zero => f.zero,
            JumpTest::NotCarry => !f.carry,
            JumpTest::Carry => f.carry,
            JumpTest::Always => true,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum LoadByteTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
}

#[derive(Copy, Clone, Debug)]
pub enum LoadByteSource {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    D8,
    HLI,
}

macro_rules! load_type {
    ($target:ident, $src:ident) => {
        LoadType::Byte(LoadByteTarget::$target, LoadByteSource::$src)
    };
}

#[derive(Copy, Clone, Debug)]
pub enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
}

impl Instruction {
    pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Self::from_byte_prefixed(byte)
        } else {
            Self::from_byte_not_prefixed(byte)
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            // TODO: SWAP n

            // TODO: RLC n

            // TODO: RL n

            // TODO: RRC n

            // TODO: RR n

            // TODO: SLA n

            // TODO: SRA n

            // TODO: SRL n

            // TODO: BIT b,r

            // TODO: SET b,r

            // TODO: RES b,r
            _ => None,
        }
    }

    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            // TODO: LD nn,n

            // LD r1,r2
            0x7F => Some(Instruction::LD(load_type!(A, A))),
            0x78 => Some(Instruction::LD(load_type!(A, B))),
            0x79 => Some(Instruction::LD(load_type!(A, C))),
            0x7A => Some(Instruction::LD(load_type!(A, D))),
            0x7B => Some(Instruction::LD(load_type!(A, E))),
            0x7C => Some(Instruction::LD(load_type!(A, H))),
            0x7D => Some(Instruction::LD(load_type!(A, L))),
            0x7E => Some(Instruction::LD(load_type!(A, HLI))),

            0x40 => Some(Instruction::LD(load_type!(B, B))),
            0x41 => Some(Instruction::LD(load_type!(B, C))),
            0x42 => Some(Instruction::LD(load_type!(B, D))),
            0x43 => Some(Instruction::LD(load_type!(B, E))),
            0x44 => Some(Instruction::LD(load_type!(B, H))),
            0x45 => Some(Instruction::LD(load_type!(B, L))),
            0x46 => Some(Instruction::LD(load_type!(B, HLI))),

            0x48 => Some(Instruction::LD(load_type!(C, B))),
            0x49 => Some(Instruction::LD(load_type!(C, C))),
            0x4A => Some(Instruction::LD(load_type!(C, D))),
            0x4B => Some(Instruction::LD(load_type!(C, E))),
            0x4C => Some(Instruction::LD(load_type!(C, H))),
            0x4D => Some(Instruction::LD(load_type!(C, L))),
            0x4E => Some(Instruction::LD(load_type!(C, HLI))),

            0x50 => Some(Instruction::LD(load_type!(D, B))),
            0x51 => Some(Instruction::LD(load_type!(D, C))),
            0x52 => Some(Instruction::LD(load_type!(D, D))),
            0x53 => Some(Instruction::LD(load_type!(D, E))),
            0x54 => Some(Instruction::LD(load_type!(D, H))),
            0x55 => Some(Instruction::LD(load_type!(D, L))),
            0x56 => Some(Instruction::LD(load_type!(D, HLI))),

            0x58 => Some(Instruction::LD(load_type!(E, B))),
            0x59 => Some(Instruction::LD(load_type!(E, C))),
            0x5A => Some(Instruction::LD(load_type!(E, D))),
            0x5B => Some(Instruction::LD(load_type!(E, E))),
            0x5C => Some(Instruction::LD(load_type!(E, H))),
            0x5D => Some(Instruction::LD(load_type!(E, L))),
            0x5E => Some(Instruction::LD(load_type!(E, HLI))),

            0x60 => Some(Instruction::LD(load_type!(H, B))),
            0x61 => Some(Instruction::LD(load_type!(H, C))),
            0x62 => Some(Instruction::LD(load_type!(H, D))),
            0x63 => Some(Instruction::LD(load_type!(H, E))),
            0x64 => Some(Instruction::LD(load_type!(H, H))),
            0x65 => Some(Instruction::LD(load_type!(H, L))),
            0x66 => Some(Instruction::LD(load_type!(H, HLI))),

            0x68 => Some(Instruction::LD(load_type!(L, B))),
            0x69 => Some(Instruction::LD(load_type!(L, C))),
            0x6A => Some(Instruction::LD(load_type!(L, D))),
            0x6B => Some(Instruction::LD(load_type!(L, E))),
            0x6C => Some(Instruction::LD(load_type!(L, H))),
            0x6D => Some(Instruction::LD(load_type!(L, L))),
            0x6E => Some(Instruction::LD(load_type!(L, HLI))),

            0x70 => Some(Instruction::LD(load_type!(HLI, B))),
            0x71 => Some(Instruction::LD(load_type!(HLI, C))),
            0x72 => Some(Instruction::LD(load_type!(HLI, D))),
            0x73 => Some(Instruction::LD(load_type!(HLI, E))),
            0x74 => Some(Instruction::LD(load_type!(HLI, H))),
            0x75 => Some(Instruction::LD(load_type!(HLI, L))),
            // TODO: LD (HL),n 36 12
            // 0x36 => Some(Instruction::LD(load_type!(HLI, n))),

            // TODO: LD A,n

            // TODO: LD n,A

            // TODO: LD A,(C)

            // TODO: LD (C),A

            // TODO: LDD A,(HL)

            // TODO: LDD (HL),A

            // TODO: LDI A,(HL)

            // TODO: LDI (HL),A

            // TODO: LDH (n),A

            // TODO: LDH A,(n)

            // TODO: LD n,nn

            // TODO: LDHL SP,HL

            // TODO: LDHL SP,n

            // TODO: LD (nn),SP

            // PUSH nn
            0xF5 => Some(Instruction::PUSH(PushPopTarget::AF)),
            0xC5 => Some(Instruction::PUSH(PushPopTarget::BC)),
            0xD5 => Some(Instruction::PUSH(PushPopTarget::DE)),
            0xE5 => Some(Instruction::PUSH(PushPopTarget::HL)),

            // POP nn
            0xF1 => Some(Instruction::POP(PushPopTarget::AF)),
            0xC1 => Some(Instruction::POP(PushPopTarget::BC)),
            0xD1 => Some(Instruction::POP(PushPopTarget::DE)),
            0xE1 => Some(Instruction::POP(PushPopTarget::HL)),

            // ADD A,n
            0x87 => Some(Instruction::ADD(ArithmeticTarget::A)),
            0x80 => Some(Instruction::ADD(ArithmeticTarget::B)),
            0x81 => Some(Instruction::ADD(ArithmeticTarget::C)),
            0x82 => Some(Instruction::ADD(ArithmeticTarget::D)),
            0x83 => Some(Instruction::ADD(ArithmeticTarget::E)),
            0x84 => Some(Instruction::ADD(ArithmeticTarget::H)),
            0x85 => Some(Instruction::ADD(ArithmeticTarget::L)),
            // TODO: ADD A,(HL) 86 8
            // 0x86 => Some(Instruction::ADD(ArithmeticTarget::HL)),
            // TODO: ADD A,# C6 8
            // 0xC6 => Some(Instruction::ADD(#)),

            // TODO: ADC A,n

            // TODO: SUB n

            // TODO: SBC A,n

            // TODO: AND n

            // TODO: OR n

            // TODO: XOR n

            // TODO: CP n

            // TODO: INC n

            // TODO: DEC n

            // TODO: ADD HL,n

            // TODO: ADD SP,n

            // TODO: INC nn

            // TODO: DEC nn

            // TODO: DAA

            // TODO: CPL

            // TODO: CCF

            // TODO: SCF

            // NOP
            0x00 => Some(Instruction::NOP()),

            // HALT
            0x76 => Some(Instruction::HALT()),

            // TODO: STOP

            // TODO: DI

            // TODO: EI

            // TODO: RLCA

            // TODO: RLA

            // TODO: RRCA

            // TODO: RRA

            // JP nn
            0xC3 => Some(Instruction::JP(JumpTest::Always)),

            // JP cc,nn
            0xC2 => Some(Instruction::JP(JumpTest::NotZero)),
            0xCA => Some(Instruction::JP(JumpTest::Zero)),
            0xD2 => Some(Instruction::JP(JumpTest::NotCarry)),
            0xDA => Some(Instruction::JP(JumpTest::Carry)),

            // TODO: JP (HL)

            // TODO: JR n

            // TODO: JR cc,n

            // CALL nn
            0xCD => Some(Instruction::CALL(JumpTest::Always)),

            // CALL cc,nn
            0xC4 => Some(Instruction::CALL(JumpTest::NotZero)),
            0xCC => Some(Instruction::CALL(JumpTest::Zero)),
            0xD4 => Some(Instruction::CALL(JumpTest::NotCarry)),
            0xDC => Some(Instruction::CALL(JumpTest::Carry)),

            // TODO: RST n

            // RET
            0xC9 => Some(Instruction::RET(JumpTest::Always)),

            // RET cc
            0xC0 => Some(Instruction::RET(JumpTest::NotZero)),
            0xC8 => Some(Instruction::RET(JumpTest::Zero)),
            0xD0 => Some(Instruction::RET(JumpTest::NotCarry)),
            0xD8 => Some(Instruction::RET(JumpTest::Carry)),

            // TODO: RETI
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::flags_register::FlagsRegister;
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn JumpTest_condition_depending_on_flags_reg() {
        let mut f = FlagsRegister::new();
        f.carry = true;
        f.zero = true;

        assert_eq!(JumpTest::NotZero.condition_depending_on_flags_reg(f), false);
        assert_eq!(JumpTest::Zero.condition_depending_on_flags_reg(f), true);
        assert_eq!(
            JumpTest::NotCarry.condition_depending_on_flags_reg(f),
            false
        );
        assert_eq!(JumpTest::Carry.condition_depending_on_flags_reg(f), true);
        assert_eq!(JumpTest::Always.condition_depending_on_flags_reg(f), true);
    }
}
