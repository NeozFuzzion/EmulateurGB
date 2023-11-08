use std::fs::File;
use std::io::{Read, Write};
use std::path;
use std::path::PathBuf;
use crate::cartridge::MemoryBankController;

pub(crate) struct Mbc5{
    data:Vec<u8>,
    has_ram:bool,
    has_battery:bool,
    //    has_rumble:bool,
    number_rombank:u16,
    number_rambank:u16,
    ram:Vec<u8>,
    ram_enable:bool,
    rombank: u16,
    rambank: u16,
    path: PathBuf,
}

impl Mbc5 {
    pub fn new(bytes: Vec<u8>, cart_path: &str) -> Self {
        let mut has_ram =false;
        let mut has_battery=false;
        //let mut has_rumble = false;
        let path = PathBuf::from(cart_path);
        let savepath= path.with_extension("gbsave");

        match bytes[0x147] {
            0x1A=>has_ram=true,
            0x1B=>{has_ram=true;has_battery=true},
           /* 0x1C=>has_rumble=true,
            0x1D=>{has_rumble=true;has_ram=true},
            0x1E=>{has_rumble=true;has_ram=true;has_battery=true},*/
            _ => {}
        }

        let number_rambank:u16 = match bytes[0x149]{
            0x02 => 1,
            0x03 => 4,
            0x04 => 16,
            0x05 => 8,
            _ => 0
        };

        let ram = if has_ram{
            let mut data = vec![0; (0x2000 * number_rambank) as usize];
            if has_battery{
                if let Some(save) = loadsave(savepath) {
                    data=save;
                } else {
                    println!("Le fichier n'a pas été trouvé ou une erreur est survenue lors de sa lecture.");
                }
            }
            data
        }else {
            vec![]
        };

        let number_rombank = match bytes[0x148]{
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
            //has_rumble,
            number_rombank,
            number_rambank,
            ram,
            ram_enable:false,
            rombank: 1,
            rambank: 0,
            path,
        }
    }

}

impl MemoryBankController for Mbc5 {
    //https://gbdev.io/pandocs/MBC5.html
    fn read_byte(&self, address: u16) -> u8{
        let address_correct= if address < 0x4000 {
            address as usize
        } else {
            (self.rombank as usize * 0x4000) | ((address as usize) & 0x3FFF)
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
                    self.ram[((self.rambank * 0x2000) | (address & 0x1FFF)) as usize]
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
            0x2000 ..= 0x2FFF => self.rombank = ((self.rombank & 0x100) | (byte as u16)) % self.number_rombank,
            0x3000 ..= 0x3FFF => self.rombank = ((self.rombank & 0xFF) | ((byte as u16 & 1) << 8)) % self.number_rombank,
            0x4000 ..= 0x5FFF => self.rambank = (byte as u16 & 0x0F) % self.number_rambank,
            0x6000 ..= 0x7FFF => { /* Do nothing but why don't know */ },
            0xA000..=0xBFFF => {
                if !self.ram_enable {
                    return
                }
                /*if self.has_ram && self.has_battery{
                    let path= self.path.with_extension("gbsave");
                    File::create(path).and_then(|mut f| f.write_all(&*self.ram)).expect("error saving");
                }*/

                self.ram[((self.rambank * 0x2000) | (address & 0x1FFF))as usize] = byte;
            }
            _ => panic!("GG i didn't thought someone can go there if you want to know you are lost in MBC5 write_byte")
        }
    }

}


//make at the moment it destroys object
impl Drop for Mbc5 {
    fn drop(&mut self) {
        if self.has_ram && self.has_battery{
            let path= self.path.with_extension("gbsave");
            File::create(path).and_then(|mut f| f.write_all(&self.ram)).expect("error saving");

        }

    }
}

fn loadsave(path: path::PathBuf) -> Option<Vec<u8>> {
    let mut data = vec![];

    if let Ok(mut file) = File::open(path) {
        if file.read_to_end(&mut data).is_ok() {
            return Some(data);
        }
    }

    None
}