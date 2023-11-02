use crate::cartridge::MemoryBankController;

pub struct Rom {
    data: Vec<u8>,
}
impl Rom {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            data: bytes,
        }
    }

}

impl MemoryBankController for Rom {
    fn read_byte(&self, address: u16) -> u8{
        self.data[address as usize]
    }

    fn write_byte(&mut self, _address: u16,_byte: u8){
        //no writing right in rom only
    }
}
