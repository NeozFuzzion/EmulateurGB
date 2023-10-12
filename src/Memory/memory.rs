
pub struct MemoryBus {
    pub(crate) memory: [u8; 0xFFFF]
}

impl MemoryBus {
    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write_byte(&mut self, addr: u16, byte: u8) {
        self.memory[addr as usize]=byte;
    }

    pub fn read_word(&self, address: u16) -> u16 {
        u16::from(self.read_byte(address)) | (u16::from(self.read_byte(address + 1)) << 8)
    }

    pub fn write_word(&mut self, addr: u16, word: u16) {
        self.memory[addr as usize] = (word & 0xFF) as u8;
        self.memory[(addr + 1) as usize] = ((word >> 8) & 0xFF) as u8;
    }
}
