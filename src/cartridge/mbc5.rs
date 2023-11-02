use crate::cartridge::MemoryBankController;

pub(crate) struct Mbc5{
    data:Vec<u8>,
    has_ram:bool,
    has_battery:bool,
    has_rumble:bool,
    number_rombank:u16,
    number_rambank:u16,
    ram:Vec<u8>,
    ram_enable:bool,
    rombank: u16,
    rambank: u16,
}

impl Mbc5 {
    pub fn new(bytes: Vec<u8>) -> Self {
        let mut has_ram =false;
        let mut has_battery=false;
        let mut has_rumble = false;


        match bytes[0x147] {
            0x1A=>has_ram=true,
            0x1B=>{has_ram=true;has_battery=true},
            0x1C=>has_rumble=true,
            0x1D=>{has_rumble=true;has_ram=true},
            0x1E=>{has_rumble=true;has_ram=true;has_battery=true},
            _ => {}
        }

        let mut number_rambank:u16 = match bytes[0x149]{
            0x02 => 1,
            0x03 => 4,
            0x04 => 16,
            0x05 => 8,
            _ => 0
        };

        let ram = if has_ram{
            vec![0; (0x2000 * number_rambank) as usize]
        }else {
            vec![]
        };

        let mut number_rombank = match bytes[0x148]{
            0x00=>2,
            0x01=>4,
            0x02=>8,
            0x03=>16,
            0x04=>32,
            0x05=>64,
            0x06=>128,
            0x07=>256,
            0x08=>512,
            0x52=>72,
            0x53=>80,
            0x54=>96,
            _ => 0
        };
        Self {
            data: bytes,
            has_ram,
            has_battery,
            has_rumble,
            number_rombank,
            number_rambank,
            ram,
            ram_enable:false,
            rombank: 1,
            rambank: 0,
        }
    }

}

impl MemoryBankController for Mbc5 {
    //https://gbdev.io/pandocs/MBC5.html
    fn read_byte(&self, address: u16) -> u8{
        let address_correct= if address < 0x4000 {
            address as usize
        } else {
            self.rombank as usize * 0x4000 | ((address as usize) & 0x3FFF)
        };
        match address{
            0x0000..=0x3FFF => {
                self.data[address as usize]
            }
            0x4000..=0x7FFF => {
                self.data[address_correct]
            }
            0xA000..=0xBFFF => {
                if self.has_ram{
                    self.ram[(self.rambank * 0x2000 | (address & 0x1FFF)) as usize]
                } else {
                    0
                }
            }
            _ => panic!("GG i didn't thought someone can go there if you want to know you are lost in MBC5 read_byte")
        }
    }

    fn write_byte(&mut self, address: u16,byte: u8){
        //println!("addresse : {:x} value : {}", address, byte);

        match address {
            0x0000 ..= 0x1FFF => self.ram_enable = byte & 0x0F == 0x0A,
            0x2000 ..= 0x2FFF => self.rombank = (self.rombank & 0x100) | (byte as u16),
            0x3000 ..= 0x3FFF => self.rombank = (self.rombank & 0xFF) | ((byte as u16 & 1) << 8),
            0x4000 ..= 0x5FFF => self.rambank = byte as u16 & 0x0F,
            0xA000..=0xBFFF => {
                if self.ram_enable == false {
                    return
                }
                self.ram[(self.rambank * 0x2000 | (address & 0x1FFF))as usize] = byte;
            }
            _ => panic!("GG i didn't thought someone can go there if you want to know you are lost in MBC5 write_byte")
        }
    }
}