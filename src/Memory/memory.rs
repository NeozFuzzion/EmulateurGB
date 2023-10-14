use core::panic;

use crate::GPU::gpu::GPU;

pub struct MemoryBus {
    pub(crate) memory: [u8; 0xFFFF],
    pub(crate) wram: [u8; 0x2000], 
    pub(crate) hram: [u8; 0x80],
    pub(crate) gpu: GPU,

    pub(crate) interrupt_flags: u8,
    pub(crate) interrupt_enabled: u8,
}

impl MemoryBus {
    pub fn read_byte(&self, address: u16) -> u8 {
        println!("{:x}",address);
        match address {
            0x0000..=0x7FFF | 0xA000..=0xBFFF => self.memory[address as usize], // ROM and cart RAM
            0x8000..=0x9FFF => panic!("RGPU"),             // Load from GPU
            0xC000..=0xFDFF => self.wram[(address & 0x1FFF) as usize],        // Working RAM
            0xFE00..=0xFE9F => panic!("RGPU"),                    // Graphics - sprite information
            0xFF00 => panic!("RInput"),                                   // Input read
            0xFF01..=0xFF02 => panic!("RSerial"),                     // Serial read
            0xFF04..=0xFF07 => panic!("RClock"),                 // read Clock values
            0xFF0F => self.interrupt_flags,                                // Interrupt flags
            0xFF10..=0xFF26 => panic!("RSound"),                 // Sound control
            0xFF30..=0xFF3F => panic!("RSound"),                 // Sound wave pattern RAM
            0xFF40..=0xFF4B => self.gpu.read_lcd_reg(address),
            0xFF4C..=0xFF7F => panic!("MMU ERROR: Memory mapped I/O (read) (CGB only) not implemented"),
            0xFF80..=0xFFFE => self.hram[(address & 0x7F) as usize], // High RAM
            0xFFFF => self.interrupt_enabled,                     // Interrupt enable
            _ => 0,
        }
    }

    pub fn write_byte(&mut self, address: u16, byte: u8) {
        println!("{:x}",address);
        match address {
            0x0000..=0x7FFF | 0xA000..=0xBFFF => self.memory[address as usize]=byte, // ROM and cart RAM
            0x8000..=0x9FFF => panic!("WGPU"),              // Write to GPU
            0xC000..=0xFDFF => self.wram[(address & 0x1FFF) as usize] = byte,        // Working RAM
            0xFE00..=0xFE9F => panic!("WGPU"),                    // Graphics - sprite information
            0xFF00 => panic!("WInput"),                                     // Input write
            0xFF01..=0xFF02 => panic!("WSerial"),                     // Serial write
            0xFF04..=0xFF07 => panic!("WClock"),                 // write Clock values
            0xFF0F => self.interrupt_flags = byte,                                // Interrupt flags
            //0xFF10..=0xFF26 => panic!("WSound"),                 // Sound control
            //0xFF30..=0xFF3F => panic!("WSound"),                 // Sound wave pattern RAM
            0xFF46 => panic!("Rsprite"),
            0xFF40..=0xFF45 | 0xFF47..=0xFF4B => self.gpu.write_lcd_reg(address,byte),
            /*0xFF4C..=0xFF7F => panic!(
                "MMU ERROR: Memory mapped I/O (write) (CGB only) not implemented. Addr: 0x{:X}",
                addr
            ),*/
            0xFF80..=0xFFFE => self.hram[(address & 0x7F) as usize] = byte, // High RAM
            0xFFFF => self.interrupt_enabled = byte,                     // Interrupt enable
            _ => (),
        }
    }

    pub fn read_word(&self, address: u16) -> u16 {
        u16::from(self.read_byte(address)) | (u16::from(self.read_byte(address + 1)) << 8)
    }

    pub fn write_word(&mut self, addr: u16, word: u16) {
        self.memory[addr as usize] = (word & 0xFF) as u8;
        self.memory[(addr + 1) as usize] = ((word >> 8) & 0xFF) as u8;
    }

    pub fn get_triggered_interrupts(&self) -> u8 {
        self.interrupt_flags & self.interrupt_enabled
    }

    pub fn reset_interrupt(&mut self, flag: u8) {
        self.interrupt_flags &= !flag;
    }

}
