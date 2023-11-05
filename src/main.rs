/*use std::fs::File;
use std::io::Write;*/
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;

use cpu::registres::Registers;

mod cpu;
mod memory;
mod gpu;
mod render;
mod input;
mod cartridge;


extern crate glium;
extern crate glutin;
use std::time::{SystemTime};
use crate::cpu::clock::Clock;

fn main() {
    let reg=Registers ::new();


    let (tx     , rx) = mpsc::channel();
    let (key_sender, key_receiver) = mpsc::channel();
    let (stop_sender, stop_receiver) = mpsc::channel();


    let mut cpu = cpu::cpu::Cpu {
        registers: reg,
        pc: 0x0100,
        bus: memory::memory::MemoryBus{ rom: cartridge::new("D:/Prog/Pokemon.gb"), interrupt_flags: 0, interrupt_enabled: 0, wram: [0_u8; 0x2000],  hram: [0_u8; 0x80], gpu: gpu::gpu::GPU::new(),screen_sender: tx, input: input::Input::new(key_receiver), clock: Clock::default() },
        sp: 0xFFFE,
        halt: false,
        interrupt_master_enable: true,
        ei:0,
        di:0,
        cycle:0,
        stop: stop_receiver,
    };

    let mut window_game = render::Renderer::new(
        "Wow une image",
        rx,
        key_sender,
        stop_sender,
    );
    /*
   let mut file = File::create("output2.txt").expect("Impossible d'ouvrir le fichier de sortie.");
    let mut file1 = File::create("output1.txt").expect("Impossible d'ouvrir le fichier de sortie.");
    let mut x=0;*/
    let thread_cpu = thread::spawn(move || {
        let mut now = SystemTime::now();
        loop {
            /*let preinst = cpu.bus.read_byte(cpu.pc);
            let preinst2 = cpu.bus.read_byte(cpu.pc+1);*/
            // each cycle take around 238 ns because in 1s 4 194 304 cycle are made not most accurate but my pov on it
            let timed_cycle=cpu.run()as u128*238*4;
    /* x+=1;
     if x> 5698548{
         let line1 = format!("Tour de boucle {} : {:x} op : {:x} a : {}  b : {}  c : {}  d : {}  e : {}  h : {} l : {}  flag : {:b}  lcdc : {:b} ly : {} interruption : {:b} inst:{:x} {:x} hram:{:?}\n", x,cpu.pc,cpu.bus.read_byte(cpu.pc),cpu.registers.a,cpu.registers.b,cpu.registers.c,cpu.registers.d,cpu.registers.e,cpu.registers.h,cpu.registers.l , (cpu.registers.f.zero as u8)<<7 | (cpu.registers.f.subtract as u8)<<6 | (cpu.registers.f.half_carry as u8)<<5 | (cpu.registers.f.carry as u8)<<4,cpu.bus.gpu.lcdc,cpu.bus.gpu.ly,cpu.bus.interrupt_flags,preinst,preinst2,cpu.bus.hram);
         file.write_all(line1.as_bytes()).expect("Impossible d'écrire dans le fichier.");

     }let line2 = format!("Tour de boucle {} : {:x} lcdc : {:b} clockgpu : {}\n", x,cpu.pc,cpu.bus.gpu.lcdc,cpu.cycle);
            file1.write_all(line2.as_bytes()).expect("Impossible d'écrire dans le fichier.");
            //if (x==5698550){panic!("tot_cycle :")}
*/

            let mut difference=SystemTime::now().duration_since(now).expect("Le temps actuel est antérieur au temps de départ.").as_nanos();



            match cpu.stop.try_recv() {
                Ok(_data) => break,
                Err(TryRecvError::Empty) => (),
                _ => {}
            }
            //wait until the cpu catch our
            while difference<timed_cycle{
                difference= SystemTime::now().duration_since(now).expect("Le temps actuel est antérieur au temps de départ.").as_nanos();
            }
            now = SystemTime::now();
        }
    });


    window_game.start_loop();
    if let Err(e) = thread_cpu.join() {
        panic!("Error: Failed to join CPU thread: {:?}", e);
    }
}
