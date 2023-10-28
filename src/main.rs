
use std::{fs::File, thread};
use std::io::{Write, Seek, SeekFrom, Read};
use std::fs::OpenOptions;
use std::sync::mpsc;

use CPU::registres::Registers;

mod CPU;
mod Memory;
mod GPU;
mod render;
mod input;

extern crate glium;
extern crate glutin;
use glium::{ Surface, Display, Frame, Program, VertexBuffer, implement_vertex, uniform};
use glium::uniforms::{Uniforms, UniformValue, AsUniformValue};
use std::io::prelude::*;
use glutin::EventsLoop;

fn main() {
    let reg=Registers ::new();

    let mut input_file = File::open("D:/Prog/Tetris.gb").expect("gameboy rom file");
    let mut bytes = [0;0xFFFF];
    input_file.read(&mut bytes).expect("read bytes from file");

    let (tx     , rx) = mpsc::channel();
    let (key_sender, key_receiver) = mpsc::channel();

    let mut cpu = CPU::cpu::CPU{
        registers: reg,
        pc: 0x0100,
        bus: Memory::memory::MemoryBus{ memory: bytes, interrupt_flags: 0, interrupt_enabled: 0, wram: [0_u8; 0x2000],  hram: [0_u8; 0x80], gpu: GPU::gpu::GPU::new(),screen_sender: tx, input: input::Input::new(key_receiver) },
        sp: 0xFFFE,
        halt: false,
        interrupt_master_enable: true,
        ei:0,
        di:0,
        cycle:0,
    };
    // Créez ou ouvrez le fichier de sortie pour écriture
    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("output3.txt")
        .expect("output file");
    let mut x = 1;
    let mut window_game = render::Renderer::new(
        "Wow une image",
        3,
        rx,
        key_sender,
    );

    let mut file = File::create("output2.txt").expect("Impossible d'ouvrir le fichier de sortie.");
    let mut file1 = File::create("output1.txt").expect("Impossible d'ouvrir le fichier de sortie.");
    let mut tot_cycle=0_u32;
    let cpu_thread = thread::spawn(move || {
        loop {
            let cycle=cpu.run();
            /*if x>=3800000{
                let line1 = format!("Tour de boucle {} : {:x} vram : {:?} oam : {:?}\n", x,cpu.pc,cpu.bus.gpu.vram, cpu.bus.gpu.oam);
                file.write_all(line1.as_bytes()).expect("Impossible d'écrire dans le fichier.");
                let line2 = format!("Tour de boucle {} : {:x} lcdc : {:b}\n", x,cpu.pc,cpu.bus.gpu.lcdc);
                file1.write_all(line2.as_bytes()).expect("Impossible d'écrire dans le fichier.");
        }*/
            tot_cycle+=cpu.cycle as u32;
           /* if (x==4000000){
                panic!("tot_cycle : {}",tot_cycle)
            };*/

            x+=1;
        }
    });


    window_game.start_loop();



    if let Err(e) = cpu_thread.join() {
        panic!("Error: Failed to join CPU thread: {:?}", e);
    }

}
