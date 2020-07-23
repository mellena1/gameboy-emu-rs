const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct FlagsRegister {
    // This bit becomes set (1) if the result of an operation has been zero (0). Used for conditional jumps.
    pub zero: bool,
    // Indicates whether the previous instruction has been an addition or subtraction ((rarely) used for the DAA instruction only)
    pub subtract: bool,
    // Indicates carry for lower 4bits of the result (also for DAA)
    pub half_carry: bool,
    // Becomes set when the result of an addition became bigger than FFh (8bit) or FFFFh (16bit).
    // Or when the result of a subtraction or comparision became less than zero.
    // Also the flag becomes set when a rotate/shift operation has shifted-out a "1"-bit.
    // Used for conditional jumps, and for instructions such like ADC, SBC, RL, RLA, etc.
    pub carry: bool,
}

impl FlagsRegister {
    pub fn new() -> FlagsRegister {
        FlagsRegister {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: false,
        }
    }
}

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION
            | (if flag.subtract { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION
            | (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION
            | (if flag.carry { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        FlagsRegister {
            zero: (byte >> ZERO_FLAG_BYTE_POSITION) & 0b1 == 1,
            subtract: (byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1 == 1,
            half_carry: (byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1 == 1,
            carry: (byte >> CARRY_FLAG_BYTE_POSITION) & 0b1 == 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_u8() {
        let f = FlagsRegister::from(0b1010_0000);
        assert_eq!(
            f,
            FlagsRegister {
                zero: true,
                subtract: false,
                half_carry: true,
                carry: false
            }
        );
    }

    #[test]
    fn into_u8() {
        let mut f = FlagsRegister::new();
        f.zero = true;
        f.half_carry = true;

        let u: u8 = f.into();
        assert_eq!(u, 0b1010_0000);
    }
}
