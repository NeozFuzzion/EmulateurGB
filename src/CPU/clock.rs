#[derive(Default)]
pub struct Clock {
    div:u8,
    counter_div:u32,
    tima:u8,
    tma:u8,
    tac:u8,
    pub interrupt:u8,
    timer:u8,
    counter:u32
}

impl Clock {
    pub fn read(&self, address: u16)->u8{
        match address {
            0xFF04 => self.div,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac,
            _ => panic!("Error reading clock")
        }
    }

    pub fn write(&mut self, address: u16, value: u8){
        match address {
            0xFF04 => self.div=0,
            0xFF05 => self.tima=value,
            0xFF06 => self.tma=value,
            0xFF07 => {
                self.tac=value;
                self.tima += 1;
            },
            _ => panic!("Error writing clock")
        }
    }

    pub fn run(&mut self,ticks:u32  ){
        self.counter_div += ticks;
        while self.counter_div >= 0xFF {
            self.div = self.div.wrapping_add(1);
            self.counter_div -= 256;
        }

        if self.tac  & 0x4 != 0 {
            self.counter += ticks;
            let limit = match self.tac & 0x3 { 0 => 1024 , 1 => 16, 2 => 64, 3 => 256,
                _ => panic!("error of limit timer")
            };
            while self.counter >= limit {
                self.timer = self.timer.wrapping_add(1);
                if self.timer == 0 {
                    self.timer = self.tma;
                    self.interrupt |= 0x04;
                }
                self.counter -= limit;
            }
        }
    }
}