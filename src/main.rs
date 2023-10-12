use std::fs::File;
use std::io::{Write, Seek, SeekFrom, Read};
use std::fs::OpenOptions;

use CPU::registres::Registers;

mod CPU;
mod Memory;


fn main() {
    let reg=Registers ::new();
    
    let mut input_file = File::open("/home/cytech/Tetris.gb").expect("gameboy rom file");
    let mut bytes = [0;0xFFFF];
    input_file.read(&mut bytes).expect("read bytes from file");
    let mut cpu = CPU::cpu::CPU{
        registers: reg,
        pc: 0x0150,
        bus: Memory::memory::MemoryBus{ memory: bytes },
        sp: 0xFFFE,
        halt: false,
        ei: 0,
        di: 0,
        interrupt_master_enable: false,
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
