use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::cartridge::rom::Rom;
use crate::cartridge::mbc5::Mbc5;
pub(crate) mod rom;
pub(crate) mod mbc5;

pub fn new(cart_path: &str) -> Box<dyn MemoryBankController> {

    // Check if the file has the ".gb" extension
    let file_extension = Path::new(cart_path)
        .extension()
        .and_then(|ext| ext.to_str());

    if file_extension != Some("gb") {
        panic!("Invalid file extension. Expected '.gb'");
    }

    let mut input_file = match File::open(cart_path) {
        Ok(file) => file,
        Err(err) => {
            panic!("Error opening the file: {}", err);
        }
    };
    let mut bytes : Vec<u8> = vec![];
    input_file.read_to_end(&mut bytes).expect("read bytes from file");
    
    //check nintendo logo
    assert_eq!(&bytes[0x104..0x134], &[0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E]);
    
    if calculate_checksum(&bytes) != bytes[0x014D] {
        // The checksum doesn't match, handle the error accordingly
        println!("Checksum mismatch! ROM may be corrupt.");
    } else {
        // Checksum is valid, continue with your program
        println!("Checksum is valid.");
    }
    
    match bytes[0x147] {
        0x00 => Box::new(Rom::new(bytes)),
        //0x01..=0x03 => Box::new(Mbc1::with_ram_and_battery(0,bytes, 0x2000)),
        //0x05..0x06 => Box::new(Mbc2::new())
        //0x08..0x09 => Box::new(Rom::new())  //rom with ram and battery docs says not known
        //0x0B..0x0D => Box::new(Mmm01::new())
        //0x0F..0x13 => Box::new(Mbc3::new())
        0x19..=0x1B => Box::new(Mbc5::new(bytes,cart_path)),  // 1B for pokemon red
        //0x1C..=0x1E => Box::new(Mbc5::new(bytes)),  //rumble
        //0x20 => Box::new(Mbc6::new())
        //0x22 => Box::new(Mbc7::new())
        //0xFC => Box::new(PocketCam::new())
        //0xFD => Box::new(BandaiTama::new())
        //0xFE => Box::new(HuC3::new())
        //0xFF => Box::new(HuC1::new())
        _ => panic!("Type of game not recognize")
    }
}


fn calculate_checksum(rom: &[u8]) -> u8 {
    let mut checksum: u8 = 0;

    for address in 0x0134..=0x014C {
        checksum = checksum.wrapping_sub(rom[address as usize]).wrapping_sub(1);
    }

    checksum
}


pub trait MemoryBankController : Send {
    fn read_byte(&self, address: u16) -> u8;

    fn write_byte(&mut self, address: u16,byte: u8);
}
