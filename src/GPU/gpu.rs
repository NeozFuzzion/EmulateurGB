

pub struct GPU {
    pub(crate) vram: [u8; 0x2000],
    pub(crate) oam: [u8; 0xA0],
    //https://gbdev.io/pandocs/STAT.html  &&   https://gbdev.io/pandocs/LCDC.html
    pub(crate) lcdc: u8,                        //FF40
    pub(crate) stat: u8,                        //FF41
    pub(crate) scy: u8,                         //FF42
    pub(crate) scx: u8,                         //FF43
    pub(crate) ly: u8,                          //FF44
    pub(crate) lyc: u8,                         //FF45   
    pub(crate) wy: u8,                          //FF4A
    pub(crate) wx: u8,                          //FF4B
    pub(crate) bgp_value: u8,                         //FF47
    bgp: [u32; 4],
    pub(crate) obp0_value: u8,                        //FF48
    obp0: [u32; 4],
    pub(crate) obp1_value: u8,                        //FF49
    obp1: [u32; 4],
}



impl GPU {
    pub fn new() -> Self {
        Self {
            vram: [0_u8; 0x2000],
            oam: [0_u8; 0xA0],
            lcdc: 0,
            stat: 0,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            wy: 0,
            wx: 0,
            bgp_value: 0,
            bgp: [0;4],
            obp0_value: 0,
            obp0: [0;4],
            obp1_value: 0,
            obp1: [0;4],
        }
    }
    pub fn read_lcd_reg(&self, address:u16) -> u8{
        match address {
            0xFF40 => self.lcdc,
            0xFF41 => self.stat,
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,

            0xFF47 => self.bgp_value,
            0xFF48 => self.obp0_value,
            0xFF49 => self.obp1_value,
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

            0xFF47 =>{
                self.bgp_value=value;
                self.bgp=value_to_palette(self.bgp_value);
            }
            0xFF48 => {
                self.obp0_value = value;
                self.obp0=value_to_palette(self.obp0_value);
            }
            0xFF49 => {
                self.obp1_value = value;
                self.obp1=value_to_palette(self.obp1_value);
            }
            0xFF4A => self.wy=value,
            0xFF4B => self.wx=value,
            _ => panic!("Unknown GPU control read operation: 0x{:X}", address),
        }
    }
    pub fn write_vram(&mut self, address: u16, value: u8) {
        self.vram[(address & 0x1FFF) as usize] = value;
    }
    pub fn write_oam(&mut self, address: u16, value: u8) {
        self.oam[(address & & 0xFF) as usize] = value;
    }

    fn addresses_data_tiles(&self) -> u16 {
        if self.lcdc & 0b00010000 > 0 {
            0x8000
        } else {
            0x8800
        }
    }



    fn addresses_tile_map(&self, is_bg:bool) -> u16 {
        if is_bg && self.lcdc & 0b00001000 > 0 || !is_bg && self.lcdc & 0b01000000 > 0 {
            return 0x9C00;
        }
        0x9800
    }




}

fn value_to_palette(value: u8) -> [u32; 4] {
    // Define the color values as hexadecimal 0=>White 1=>lightGray 2=>DarkGray 3=>Black
    let colors = [0x000000, 0x555555, 0xaaaaaa, 0xffffff];
    let mut result = [0; 4];

    for i in 0..4 {
        // Get the color index by masking the value with 0b11 (binary 11)
        let color_index = (value >> (2 * i)) & 0b11;

        // Assign the corresponding color value to the result array
        result[i] = colors[color_index as usize];
    }

    result
}