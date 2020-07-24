pub const MEM_SIZE: usize = 0xFFFF;

pub struct MemoryBus {
    memory: [u8; MEM_SIZE],
}

impl MemoryBus {
    pub fn new() -> Self {
        MemoryBus {
            memory: [0; MEM_SIZE],
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, byte: u8) {
        self.memory[address as usize] = byte;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_byte() {
        let mut mem = MemoryBus::new();
        mem.memory[0x0000] = 0x49;
        assert_eq!(mem.read_byte(0x0000), 0x49);
    }

    #[test]
    fn write_byte() {
        let mut mem = MemoryBus::new();
        mem.write_byte(0x0000, 0x49);
        assert_eq!(mem.memory[0x0000], 0x49);
    }
}
