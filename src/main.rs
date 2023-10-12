extern crate glutin;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use glutin::PossiblyCurrent;
use glutin::NotCurrent;
use glutin::Context;

fn main() {
    // Créez un event loop
    let event_loop = EventLoop::new();

    // Créez une fenêtre Glutin
    let window_builder = WindowBuilder::new().with_title("Game Boy Emulator");
    let context_builder = ContextBuilder::new()
        .with_gl(glutin::GlRequest::Latest)
        .with_vsync(true);

    // Créez le contexte OpenGL
    let context = unsafe {
        let context = ContextBuilder::new()
            .build_windowed(window_builder, &event_loop)
            .unwrap();
        let context = context.make_current().unwrap();
        context
    };

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
            Event::RedrawRequested(_) => {
                // Dessinez le contenu de votre émulateur Game Boy ici

                // Swap buffers pour afficher le rendu
                context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}



/*use std::fs::File;
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
        pc: 0x0100,
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
*/