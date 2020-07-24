use super::instructions::{
    ArithmeticTarget, Instruction, LoadByteSource, LoadByteTarget, LoadType, PushPopTarget,
};
use super::memorybus::MemoryBus;
use super::registers::Registers;

struct CPU {
    registers: Registers,
    pc: u16,
    sp: u16,
    mem: MemoryBus,
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
            pc: 0,
            sp: 0,
            mem: MemoryBus::new(),
        }
    }

    pub fn step(&mut self) {
        let mut instruction_byte = self.mem.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.read_next_byte();
        }

        self.pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
            self.execute(instruction)
        } else {
            let description = format!(
                "0x{}{:x}",
                if prefixed { "cb" } else { "" },
                instruction_byte
            );
            panic!("Unknown instruction found for: {}", description)
        };
    }

    pub fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::LD(load_type) => {
                let LoadType::Byte(target, source) = load_type;
                self.load(target, source);

                match source {
                    LoadByteSource::D8 => self.pc.wrapping_add(2),
                    _ => self.pc.wrapping_add(1),
                }
            }
            Instruction::PUSH(target) => {
                match target {
                    PushPopTarget::AF => self.push(self.registers.get_af()),
                    PushPopTarget::BC => self.push(self.registers.get_bc()),
                    PushPopTarget::DE => self.push(self.registers.get_de()),
                    PushPopTarget::HL => self.push(self.registers.get_hl()),
                }
                self.pc.wrapping_add(1)
            }
            Instruction::POP(target) => {
                let popped_val = self.pop();
                match target {
                    PushPopTarget::AF => self.registers.set_af(popped_val),
                    PushPopTarget::BC => self.registers.set_bc(popped_val),
                    PushPopTarget::DE => self.registers.set_de(popped_val),
                    PushPopTarget::HL => self.registers.set_hl(popped_val),
                }
                self.pc.wrapping_add(1)
            }
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::A => Instruction_ADD!(self, self.registers.a),
                    ArithmeticTarget::B => Instruction_ADD!(self, self.registers.b),
                    ArithmeticTarget::C => Instruction_ADD!(self, self.registers.c),
                    ArithmeticTarget::D => Instruction_ADD!(self, self.registers.d),
                    ArithmeticTarget::E => Instruction_ADD!(self, self.registers.e),
                    ArithmeticTarget::H => Instruction_ADD!(self, self.registers.h),
                    ArithmeticTarget::L => Instruction_ADD!(self, self.registers.l),
                }
                self.pc.wrapping_add(1)
            }
            Instruction::NOP() => self.pc.wrapping_add(1),
            Instruction::JP(test) => {
                let condition = test.condition_depending_on_flags_reg(self.registers.f);
                self.jump(condition)
            }
            Instruction::CALL(test) => {
                let condition = test.condition_depending_on_flags_reg(self.registers.f);
                self.call(condition)
            }
            Instruction::RET(test) => {
                let condition = test.condition_depending_on_flags_reg(self.registers.f);
                self.return_(condition)
            }
            _ => {
                panic!("Instruction: {:?} not implemented", instruction);
            }
        }
    }

    fn read_next_byte(&self) -> u8 {
        self.mem.read_byte(self.pc + 1)
    }

    fn read_next_word(&self) -> u16 {
        let lsb = self.mem.read_byte(self.pc + 1) as u16;
        let msb = self.mem.read_byte(self.pc + 2) as u16;

        (msb << 8) | lsb
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

    fn jump(&self, should_jump: bool) -> u16 {
        if should_jump {
            // Gameboy is little endian so read pc + 2 as most significant bit
            // and pc + 1 as least significant bit
            let least_significant_byte = self.mem.read_byte(self.pc + 1) as u16;
            let most_significant_byte = self.mem.read_byte(self.pc + 2) as u16;
            (most_significant_byte << 8) | least_significant_byte
        } else {
            // If we don't jump we need to still move the program
            // counter forward by 3 since the jump instruction is
            // 3 bytes wide (1 byte for tag and 2 bytes for jump address)
            self.pc.wrapping_add(3)
        }
    }

    fn load(&mut self, target: LoadByteTarget, source: LoadByteSource) {
        let source_value = match source {
            LoadByteSource::A => self.registers.a,
            LoadByteSource::B => self.registers.b,
            LoadByteSource::C => self.registers.c,
            LoadByteSource::D => self.registers.d,
            LoadByteSource::E => self.registers.e,
            LoadByteSource::H => self.registers.h,
            LoadByteSource::L => self.registers.l,
            LoadByteSource::D8 => self.read_next_byte(),
            LoadByteSource::HLI => self.mem.read_byte(self.registers.get_hl()),
        };
        match target {
            LoadByteTarget::A => self.registers.a = source_value,
            LoadByteTarget::B => self.registers.b = source_value,
            LoadByteTarget::C => self.registers.c = source_value,
            LoadByteTarget::D => self.registers.d = source_value,
            LoadByteTarget::E => self.registers.e = source_value,
            LoadByteTarget::H => self.registers.h = source_value,
            LoadByteTarget::L => self.registers.l = source_value,
            LoadByteTarget::HLI => self.mem.write_byte(self.registers.get_hl(), source_value),
        };
    }

    fn push(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.mem.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);

        self.sp = self.sp.wrapping_sub(1);
        self.mem.write_byte(self.sp, (value & 0xFF) as u8);
    }

    fn pop(&mut self) -> u16 {
        let lsb = self.mem.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        let msb = self.mem.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        (msb << 8) | lsb
    }

    fn call(&mut self, should_jump: bool) -> u16 {
        let next_pc = self.pc.wrapping_add(3);
        if should_jump {
            self.push(next_pc);
            self.read_next_word()
        } else {
            next_pc
        }
    }

    fn return_(&mut self, should_jump: bool) -> u16 {
        if should_jump {
            self.pop()
        } else {
            self.pc.wrapping_add(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::flags_register::FlagsRegister;
    use super::*;

    // TODO:
    // #[test]
    // fn step() {
    //     let mut cpu = CPU::new();
    // }

    // TODO:
    // #[test]
    // fn execute() {
    //     let mut cpu = CPU::new();
    // }

    #[test]
    fn read_next_byte() {
        let mut cpu = CPU::new();

        cpu.pc = 0x0000;
        cpu.mem.write_byte(0x0001, 0x8F);
        assert_eq!(cpu.read_next_byte(), 0x8F);
    }

    #[test]
    fn read_next_word() {
        let mut cpu = CPU::new();

        cpu.pc = 0x0000;
        cpu.mem.write_byte(0x0001, 0xFF);
        cpu.mem.write_byte(0x0002, 0xAA);
        assert_eq!(cpu.read_next_word(), 0xAAFF);
    }

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

    #[test]
    fn jump() {
        let mut cpu = CPU::new();

        cpu.pc = 0x0000;
        cpu.mem.write_byte(0x0001, 0xFF);
        cpu.mem.write_byte(0x0002, 0xAA);

        // false branch
        assert_eq!(cpu.jump(false), 0x0003);
        // true branch
        assert_eq!(cpu.jump(true), 0xAAFF);
    }

    // TODO:
    // #[test]
    // fn load() {
    //     let mut cpu = CPU::new();
    // }

    #[test]
    fn push() {
        let mut cpu = CPU::new();

        cpu.sp = 0x0002;
        cpu.push(0xAAFF);

        assert_eq!(cpu.sp, 0x0000);
        assert_eq!(cpu.mem.read_byte(0x0001), 0xAA);
        assert_eq!(cpu.mem.read_byte(0x0000), 0xFF);
    }

    #[test]
    fn pop() {
        let mut cpu = CPU::new();

        cpu.mem.write_byte(0x0001, 0xAA);
        cpu.mem.write_byte(0x0000, 0xFF);
        cpu.sp = 0x0000;
        let actual = cpu.pop();

        assert_eq!(actual, 0xAAFF);
        assert_eq!(cpu.sp, 0x0002);
    }

    #[test]
    fn call_shouldnt_jump() {
        let mut cpu = CPU::new();
        cpu.pc = 0x0000;
        assert_eq!(cpu.call(false), 0x0003);
    }

    #[test]
    fn call_should_jump() {
        let mut cpu = CPU::new();

        cpu.pc = 0x0100;
        cpu.mem.write_byte(0x0101, 0xFF);
        cpu.mem.write_byte(0x0102, 0xAA);

        cpu.sp = 0x0010;

        let actual = cpu.call(true);

        assert_eq!(actual, 0xAAFF);
        assert_eq!(cpu.mem.read_byte(0x000F), 0x01);
        assert_eq!(cpu.mem.read_byte(0x000E), 0x03);
    }

    #[test]
    fn return_shouldnt_jump() {
        let mut cpu = CPU::new();
        cpu.pc = 0x0000;
        assert_eq!(cpu.return_(false), 0x0001);
    }

    #[test]
    fn return_should_jump() {
        let mut cpu = CPU::new();

        cpu.sp = 0x000E;
        cpu.mem.write_byte(0x000F, 0x01);
        cpu.mem.write_byte(0x000E, 0x03);

        assert_eq!(cpu.return_(true), 0x0103);
    }
}
