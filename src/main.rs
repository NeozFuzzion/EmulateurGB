extern crate winit;

use winit::{
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use std::{fs::File, thread};
use std::io::{Write, Seek, SeekFrom, Read};
use std::fs::OpenOptions;
use std::sync::mpsc;

use CPU::registres::Registers;

mod CPU;
mod Memory;
mod GPU;


fn main() {
    let reg=Registers ::new();
    
    let mut input_file = File::open("F:/Prog/Tetris.gb").expect("gameboy rom file");
    let mut bytes = [0;0xFFFF];
    input_file.read(&mut bytes).expect("read bytes from file");
    /*let (screen_data_sender, screen_data_receiver) = mpsc::sync_channel(1);
    let (screen_exit_sender, screen_exit_receiver) = mpsc::channel();
    let (key_data_sender, key_data_receiver) = mpsc::channel();
    let (throttled_state_sender, throttled_state_receiver) = mpsc::channel();*/
    let mut cpu = CPU::cpu::CPU{
        registers: reg,
        pc: 0x0100,
        bus: Memory::memory::MemoryBus{ memory: bytes, interrupt_flags: 0, interrupt_enabled: 0, wram: [0_u8; 0x2000],  hram: [0_u8; 0x80], gpu: GPU::gpu::GPU::new() },
        sp: 0xFFFE,
        halt: false,
        interrupt_master_enable: false,
        ei:0,
        di:0,
    };
    // Créez ou ouvrez le fichier de sortie pour écriture
    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("output3.txt")
        .expect("output file");
    let mut x = 1;
    // Créez un event loop pour gérer les événements
    let event_loop = EventLoop::new();

    // Créez une fenêtre
    let window = WindowBuilder::new()
        .with_title("Affichage graphique en Rust")
        .build(&event_loop)
        .unwrap();

    let cpu_thread = thread::spawn(move || {
        loop {
            cpu.run();
            x+=1
        }
    });

    
    // Boucle principale
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                if let Some(key_code) = input.virtual_keycode {
                    match key_code {
                        VirtualKeyCode::Up => println!("Flèche vers le haut appuyée"),
                        VirtualKeyCode::Down => println!("Flèche vers le bas appuyée"),
                        VirtualKeyCode::Left => println!("Flèche vers la gauche appuyée"),
                        VirtualKeyCode::Right => println!("Flèche vers la droite appuyée"),
                        VirtualKeyCode::A => println!("Touche A appuyée"),
                        VirtualKeyCode::B => println!("Touche B appuyée"),
                        VirtualKeyCode::W => println!("Touche W appuyée"),
                        VirtualKeyCode::X => println!("Touche X appuyée"),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    });
   
    

    if let Err(e) = cpu_thread.join() {
        panic!("Error: Failed to join CPU thread: {:?}", e);
    }

}
