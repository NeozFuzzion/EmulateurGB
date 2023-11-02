
use std::thread;
use std::sync::mpsc;

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

    let mut cpu = cpu::cpu::Cpu {
        registers: reg,
        pc: 0x0100,
        bus: memory::memory::MemoryBus{ rom: cartridge::new("D:/Prog/Zelda.gb"), interrupt_flags: 0, interrupt_enabled: 0, wram: [0_u8; 0x2000],  hram: [0_u8; 0x80], gpu: gpu::gpu::GPU::new(),screen_sender: tx, input: input::Input::new(key_receiver), clock: Clock::default() },
        sp: 0xFFFE,
        halt: false,
        interrupt_master_enable: true,
        ei:0,
        di:0,
        cycle:0,
    };

    let mut window_game = render::Renderer::new(
        "Wow une image",
        3,
        rx,
        key_sender,
    );
    //cpu.bus.init();
    let cpu_thread = thread::spawn(move || {
        let mut now = SystemTime::now();
        loop {
            // each cycle take around 238 ns because in 1s 4 194 304 cycle are made not most accurate but my pov on it
            let timed_cycle=cpu.run()as u128*238*4;
            let mut difference=SystemTime::now().duration_since(now).expect("Le temps actuel est antérieur au temps de départ.").as_nanos();

            //wait until the cpu catch our
            while difference<timed_cycle{
                difference= SystemTime::now().duration_since(now).expect("Le temps actuel est antérieur au temps de départ.").as_nanos();
            }
            now = SystemTime::now();
        }
    });


    window_game.start_loop();



    if let Err(e) = cpu_thread.join() {
        panic!("Error: Failed to join cpu thread: {:?}", e);
    }

}
