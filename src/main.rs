use std::fs::File;
use std::io::{Write, Seek, SeekFrom, Read};
use std::fs::OpenOptions;

mod instructions;

struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagsRegister,
    h: u8,
    l: u8,
}


struct MemoryBus {
    memory: [u8; 0xFFFF]
}

impl MemoryBus {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}

enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always
}


#[derive(Clone)]
struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool
}

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;


impl std::convert::From<FlagsRegister> for u8  {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero       { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
        (if flag.subtract   { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION |
        (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if flag.carry      { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry
        }
    }
}

impl Registers {
    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }

    fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | u8::from(self.f.clone()) as u16
    }

    fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = FlagsRegister::from(value as u8).clone();
    }

    fn get_hlm(&mut self) -> u16 {
        let res = self.get_hl();
        self.set_hl(res.wrapping_sub(1));
        res
    }

    fn get_hlp(&mut self) -> u16 {
        let res = self.get_hl();
        self.set_hl(res.wrapping_add(1));
        res
    }
}

fn main() {
    let reg=Registers{
        a: 50,
        b: 7,
        c: 0,
        d: 30,
        e: 2,
        f: FlagsRegister {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: false,
        },
        h: 0,
        l: 0,
    };
    
    let mut input_file = File::open("F:/Prog/Pokemon.gb").expect("gameboy rom file");
    let mut bytes = [0;0xFFFF];
    input_file.read(&mut bytes).expect("read bytes from file");
    let mut cpu = instructions::CPU{
        registers: reg,
        pc: 0x0150,
        bus: instructions::MemoryBus{ memory: bytes },
        sp: 0xFFFE,
        interrupt_enable_flag: false,
        halt: false,
        ei: 0,
        di: 0
    };

    // Créez ou ouvrez le fichier de sortie pour écriture
    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("output3.txt")
        .expect("output file");

    for _i in 1..100 {
        cpu.step();
    }
    


}
