
#[derive(Default)]
pub struct GPU {
    //https://gbdev.io/pandocs/STAT.html  &&   https://gbdev.io/pandocs/LCDC.html
    pub(crate) lcdc: u8,                        //FF40
    pub(crate) stat: u8,                        //FF41
    pub(crate) scy: u8,                         //FF42
    pub(crate) scx: u8,                         //FF43
    pub(crate) ly: u8,                          //FF44
    pub(crate) lyc: u8,                         //FF45   
    pub(crate) wy: u8,                          //FF4A
    pub(crate) wx: u8,                          //FF4B
    pub(crate) bgp: u8,                         //FF47
    pub(crate) obp0: u8,                        //FF48
    pub(crate) obp1: u8,                        //FF49
}



impl GPU {
    pub fn read_lcd_reg(&self, address:u16) -> u8{
        match address {
            0xFF40 => self.lcdc,
            0xFF41 => self.stat,
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => 148,
            0xFF45 => self.lyc,

            0xFF47 => self.bgp,
            0xFF48 => self.obp0,
            0xFF49 => self.obp1,
            0xFF4A => self.wy,
            0xFF4B => self.wx,
            _ => panic!("Unknown GPU control read operation: 0x{:X}", address),
        }
    }

    pub fn write_lcd_reg(&mut self, address:u16, value: u8){
        match address {
            0xFF40 => self.lcdc=value,
            0xFF41 => self.stat=value,
            0xFF42 => self.scy=value,
            0xFF43 => self.scx=value,
            0xFF44 =>  self.ly=value,
            0xFF45 => self.lyc=value,

            0xFF47 => self.bgp=value,
            0xFF48 => self.obp0=value,
            0xFF49 => self.obp1=value,
            0xFF4A => self.wy=value,
            0xFF4B => self.wx=value,
            _ => panic!("Unknown GPU control read operation: 0x{:X}", address),
        }
    }
}