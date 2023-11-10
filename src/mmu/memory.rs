use core::panic;
use std::sync::mpsc::{Receiver, Sender};
use crate::cartridge;
use crate::processor::clock::Clock;

use crate::ppu::gpu::Gpu;
use crate::input::{Input, Key};
use crate::cartridge::MemoryBankController;


pub struct MemoryBus {
    pub(crate) rom: Box<dyn MemoryBankController>,
    pub(crate) wram: [u8; 0x2000], 
    pub(crate) hram: [u8; 0x80],
    pub(crate) gpu: Gpu,
    pub(crate) screen_sender:Sender<[u32;23040]>,
    pub(crate) interrupt_flags: u8,
    pub(crate) interrupt_enabled: u8,
    pub(crate) input: Input,
    pub(crate) clock: Clock,
}

impl MemoryBus {

    pub fn new(rom_path: &str, screen_sender: Sender<[u32; 23040]>, key_receiver: Receiver<Key>) -> Self {
        let cartridge = cartridge::new(rom_path);

        MemoryBus {
            rom: cartridge,
            interrupt_flags: 0,
            interrupt_enabled: 0,
            wram: [0_u8; 0x2000],
            hram: [0_u8; 0x80],
            gpu: Gpu::new(),
            screen_sender,
            input: Input::new(key_receiver),
            clock: Clock::default(),
        }
    }


    pub fn read_byte(&self, address: u16) -> u8 {

        match address {
            0x0000..=0x7FFF | 0xA000..=0xBFFF => self.rom.read_byte(address),                                   // ROM and RAM from cart
            0x8000..=0x9FFF => self.gpu.read_vram(address),                                                // GPU Vram
            0xC000..=0xFDFF => self.wram[(address & 0x1FFF) as usize],                                          // Working RAM with the echo one
            0xFE00..=0xFE9F =>  self.gpu.read_oam(address),                                                // GPU Sprite
            0xFF00 => self.input.read(),                                                                        // Input read
            //0xFF01..=0xFF02 => panic!("RSerial"),                                                             // Serial read needed to play multi not require here
            0xFF04..=0xFF07 => self.clock.read(address),                                                        // Read Clock values
            0xFF0F => self.interrupt_flags,                                                                     // Interrupt flags
            //0xFF10..=0xFF26 => panic!("RSound"),                                                              // Sound control not require here
            //0xFF30..=0xFF3F => panic!("RSound"),                                                              // Sound wave pattern RAM not require here
            0xFF40..=0xFF4B => self.gpu.read_lcd_reg(address),
            0xFF4C..=0xFF7F => panic!("MMU ERROR: memory mapped I/O (read) (CGB only) not implemented"),
            0xFF80..=0xFFFE =>self.hram[(address & 0x7F) as usize],                                             // High RAM
            0xFFFF => self.interrupt_enabled,                                                                   // Interrupt enable
            _ => 0,
        }
    }

    pub fn write_byte(&mut self, address: u16, byte: u8) {

        match address {
            0x0000..=0x7FFF | 0xA000..=0xBFFF => self.rom.write_byte(address,byte),                             // ROM and RAM from cart
            0x8000..=0x9FFF => self.gpu.write_vram(address,byte),                                         // GPU Vram
            0xC000..=0xFDFF => self.wram[(address & 0x1FFF) as usize] = byte,                                   // Working RAM
            0xFE00..=0xFE9F => self.gpu.write_oam(address,byte),                                          // GPU Sprite
            0xFF00 => self.input.write(byte),                                                             // Input write
            //0xFF01..=0xFF02 => panic!("WSerial"),                                                             // Serial write needed to play multi not require here
            0xFF04..=0xFF07 => self.clock.write(address,byte),                                            // write Clock values
            0xFF0F => self.interrupt_flags = byte,                                                              // Interrupt flags
            //0xFF10..=0xFF26 => panic!("WSound"),                                                              // Sound control
            //0xFF30..=0xFF3F => panic!("WSound"),                                                              // Sound wave pattern RAM
            0xFF46 => self.dma_into_oam(byte),
            0xFF40..=0xFF45 | 0xFF47..=0xFF4B => self.gpu.write_lcd_reg(address,byte),
            //0xFF4C..=0xFF7F => panic!((CGB only)),
            0xFF80..=0xFFFE =>self.hram[(address & 0x7F) as usize] = byte,                                      // High RAM
            0xFFFF => self.interrupt_enabled = byte,                                                            // Interrupt enable
            _ => (),
        }
    }

    pub fn read_word(&self, address: u16) -> u16 {
        u16::from(self.read_byte(address)) | (u16::from(self.read_byte(address + 1)) << 8)
    }

    pub fn write_word(&mut self, addr: u16, word: u16) {
        self.write_byte(addr, (word & 0xFF) as u8);
        self.write_byte(addr + 1 ,((word >> 8) & 0xFF) as u8);
    }

    pub fn run(&mut self,cycle:u8){
        self.gpu.run(self.screen_sender.clone(),cycle);
        self.interrupt_flags |= self.gpu.interrupt;
        self.gpu.interrupt = 0;

        self.input.run();
        self.interrupt_flags |= self.input.interrupt;
        self.input.interrupt = 0;

        self.clock.run((cycle * 4) as u32);
        self.interrupt_flags |= self.clock.interrupt;
        self.clock.interrupt=0;
    }

    fn dma_into_oam(&mut self, dma_start: u8) {
        // DMA start 0x0000, 0x0100, 0x0200, etc
        let actual_dma_start = u16::from(dma_start) * 0x100;
        for i in 0..0xA0_u16 {
            let value = self.read_byte(actual_dma_start + i);
            self.gpu.write_oam(i, value);
        }
    }
}
