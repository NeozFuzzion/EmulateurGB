use std::sync::mpsc::Sender;

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
    pub(crate) screen_buffer: [u32; 160*144],
    pub(crate) interrupt: u8,
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
            screen_buffer: [0_u32; 160*144],
            interrupt: 0,
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

    pub fn read_vram(&self, addr: u16) -> u8 {
        self.vram[(addr & 0x1FFF) as usize]
    }
    pub fn write_vram(&mut self, address: u16, value: u8) {
        self.vram[(address & 0x1FFF) as usize] = value;
    }

    pub fn read_oam(&self, addr: u16) -> u8 {
        self.oam[(addr & 0xFF) as usize]
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
        //the 2 different possibilities from the man
        if is_bg && self.lcdc & 0b00001000 > 0 || !is_bg && self.lcdc & 0b01000000 > 0 {
            return 0x9C00;
        }
        0x9800
    }

    pub fn run(&mut self, x: Sender<[u32;23040]>){
        self.ly = (self.ly + 1) % 154;
        if self.stat & 0x40 > 0 && self.ly == self.lyc {
            self.interrupt |= 0x02;
        }
        //println!("{}",self.interrupt);
        if self.ly == 144 {
            // V-Blank
            self.interrupt |= 0x01; // Mark V-Blank interrupt
            x.send((self.screen_buffer)).unwrap();
        }
        self.step_bgwin();
        //self.step_sprites();
    }


    pub fn step_bgwin(&mut self){
        //bg on ? or Vblank
        if !(self.lcdc & 0x01 > 0) || self.ly >= 144 {
            return;
        }

        //Add scrolling y (place of the bg in the tile map) and current line(ly)
        let bgy = self.scy.wrapping_add(self.ly);
        //Keep all bit above the 3rd move by 3 to get a value 0-31
        let bgy_tile_num = (u16::from(bgy) & 0xFF) >> 3;
        let bgy_pixel_in_tile = u16::from(bgy) & 0x07;

        for x in 0..160 {
            let (tile_number, x_pixel_in_tile, y_pixel_in_tile): (u8, u8, u16) =  {
                //As previously take same type as value because of 8*8 tile on a 256*256 map
                let bgx = u32::from(self.scx) + x;
                let bgx_tile_num  = ((bgx & 0xFF) >> 3) as u16;
                //Stock in reverse
                let bgx_pixel_in_tile = 7 - (bgx & 0x07) as u8;

                //Vram like a line so y*32 + x to get the right number
                let tile_number: u8 = self.read_vram(self.addresses_tile_map(true) + bgy_tile_num * 32 + bgx_tile_num);

                (tile_number, bgx_pixel_in_tile, bgy_pixel_in_tile)
            };

            let tile_addr = if self.lcdc & 0b00010000 > 0 {
                u16::from(tile_number) * 16 + 0x8000
            } else {
                // offset
                let adjusted_tile_number = (i16::from(tile_number as i8) + 128) as u16;
                adjusted_tile_number * 16 + 0x8800
            };

            let tile_line_addr = tile_addr + y_pixel_in_tile * 2;

            //Retrieve the 2 line to merge to get the pixel id color
            let (tile_line_data_1, tile_line_data_2) = (
                self.read_vram(tile_line_addr),
                self.read_vram(tile_line_addr + 1),
            );

            let pixel_in_line_mask = 1 << x_pixel_in_tile;
            let pixel_data_1: u8 = if tile_line_data_1 & pixel_in_line_mask > 0 {
                0b01
            } else {
                0b00
            };
            let pixel_data_2: u8 = if tile_line_data_2 & pixel_in_line_mask > 0 {
                0b10
            } else {
                0b00
            };
            //merge value of both line to get id color
            let palette_color_id = pixel_data_1 | pixel_data_2;

            let pixel_addr = (u32::from(self.ly) * 160 + x) as usize;
            self.screen_buffer[pixel_addr] = self.bgp[palette_color_id as usize];

        }
    }




}

fn value_to_palette(value: u8) -> [u32; 4] {
    // Define the color values as hexadecimal 0=>White 1=>lightGray 2=>DarkGray 3=>Black
    let colors = [0xffffff, 0xaaaaaa, 0x555555 ,0x000000];
    let mut result = [0; 4];

    for i in 0..4 {
        // Get the color index by masking the value with 0b11 (binary 11)
        let color_index = (value >> (2 * i)) & 0b11;

        // Assign the corresponding color value to the result array
        result[i] = colors[color_index as usize];
    }

    result
}