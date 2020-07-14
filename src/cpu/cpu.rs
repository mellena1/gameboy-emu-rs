use super::instructions::{ArithmeticTarget, Instruction};
use super::registers::Registers;

struct CPU {
    registers: Registers,
}

macro_rules! Instruction_ADD {
    ( $self: ident, $val_to_add:expr ) => {{
        $self.add($val_to_add);
    }};
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: Registers::new(),
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => match target {
                ArithmeticTarget::A => Instruction_ADD!(self, self.registers.a),
                ArithmeticTarget::B => Instruction_ADD!(self, self.registers.b),
                ArithmeticTarget::C => Instruction_ADD!(self, self.registers.c),
                ArithmeticTarget::D => Instruction_ADD!(self, self.registers.d),
                ArithmeticTarget::E => Instruction_ADD!(self, self.registers.e),
                ArithmeticTarget::H => Instruction_ADD!(self, self.registers.h),
                ArithmeticTarget::L => Instruction_ADD!(self, self.registers.l),
            },
        }
    }

    fn add(&mut self, value: u8) {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);

        // set new flag values
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        // Half Carry is set if adding the lower nibbles of the value and register A
        // than the addition caused a carry from the lower nibble to the upper nibble.
        // together result in a value bigger than 0xF. If the result is larger than 0xF
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;

        self.registers.a = new_value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::flags_registers::FlagsRegister;

    #[test]
    fn add() {
        let mut cpu = CPU::new();

        cpu.add(0xF);
        let mut expected_f = FlagsRegister::new();
        assert_eq!(cpu.registers.f, expected_f);
        assert_eq!(cpu.registers.a, 0xF);

        cpu.add(0xF0);
        assert_eq!(cpu.registers.f, expected_f);
        assert_eq!(cpu.registers.a, 0xFF);

        cpu.add(1);
        expected_f.zero = true;
        expected_f.carry = true;
        expected_f.half_carry = true;
        assert_eq!(cpu.registers.f, expected_f);
        assert_eq!(cpu.registers.a, 0x00);
    }
}
