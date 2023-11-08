use std::{thread, borrow::Cow};
use std::sync::mpsc::{self, Sender};
use std::sync::mpsc::TryRecvError;
use std::time::SystemTime;

use processor::registres::Registers;
use glium::{Texture2d, texture, Surface};
use input::Key;

mod processor;
mod mmu;
mod ppu;
mod input;
mod cartridge;


extern crate glium;
use crate::processor::clock::Clock;
use crate::input::KeyType;

fn main() {
    let reg=Registers ::new();


    let (tx     , rx) = mpsc::channel();
    let (key_sender, key_receiver) = mpsc::channel();
    let (stop_sender, stop_receiver) = mpsc::channel();

    let mut cpu = processor::cpu::Cpu {
        registers: reg,
        pc: 0x0100,
        bus: mmu::memory::MemoryBus{ rom: cartridge::new("/home/cytech/pkmn_red.gb"), interrupt_flags: 0, interrupt_enabled: 0, wram: [0_u8; 0x2000],  hram: [0_u8; 0x80], gpu: ppu::gpu::Gpu::new(),screen_sender: tx, input: input::Input::new(key_receiver), clock: Clock::default() },
        sp: 0xFFFE,
        halt: false,
        interrupt_master_enable: true,
        ei:0,
        di:0,
        cycle:0,
        stop: stop_receiver,
    };

    thread::spawn(move || {
        let mut now = SystemTime::now();
        loop {
            // each cycle take around 238 ns because in 1s 4 194 304 cycle are made not most accurate but my pov on it
            let timed_cycle=cpu.run()as u128*238*4;
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


    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build();

    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_inner_size(160*3,144*3)
        .with_title("Wow une image")
        .build(&event_loop);

    let texture = Texture2d::empty_with_format(
        &display,
        //no format u32 so translate u32 to u8u8u8 later
        texture::UncompressedFloatFormat::U8U8U8U8,
        texture::MipmapsOption::NoMipmap,
        160,
        144,
    ).unwrap();


    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Poll;
        match rx.try_recv() {
            Ok(data) => {
                //glium doesn't like u32 texture so translate to u8u8u8
                let mut dataa= vec![0u8; (160 * 144 * 3) as usize];
                for i in 0..data.len() {
                    dataa[i * 3] = (data[i] & 0xFF) as u8;
                    dataa[i * 3 + 1] = ((data[i] >> 8) & 0xFF) as u8;
                    dataa[i * 3 + 2] = ((data[i] >> 16) & 0xFF) as u8;
                }

                let image = glium::texture::RawImage2d {
                    data: Cow::Borrowed(&dataa),
                    width: 160,
                    height: 144,
                    format: glium::texture::ClientFormat::U8U8U8,
                };

                // Mettre à jour la texture avec les nouvelles données
                texture.write(
                    glium::Rect {
                        left: 0,
                        bottom: 0,
                        width: 160,
                        height: 144,
                    },
                    image,
                );

                let target = display.draw();
                let (unsigned_width, unsigned_height) = target.get_dimensions();

                let width = i32::from(unsigned_width as u16);
                let height = i32::from(unsigned_height as u16);
                let blit_target = glium::BlitTarget {
                    left: 0,
                    bottom: height as u32,
                    width,
                    height: -height,
                };
                texture.as_surface().blit_whole_color_to(
                    &target,
                    &blit_target,
                    glium::uniforms::MagnifySamplerFilter::Nearest,
                );
                if let Err(e) = target.finish() {
                    println!("ERROR: Failed to write to display: {}", e)
                }
            }
            ,
            Err(mpsc::TryRecvError::Empty) => (),
            Err(mpsc::TryRecvError::Disconnected) =>{control_flow.set_exit();},

        }

        if let winit::event::Event::WindowEvent { event, .. } = event {
           match event {
        
               winit::event::WindowEvent::CloseRequested => {stop_sender.send(true).unwrap();
                   control_flow.set_exit();},
               winit::event::WindowEvent::KeyboardInput { input, .. } => {
                   let is_down = input.state == winit::event::ElementState::Pressed;
        
                   match input.virtual_keycode {
                       Some(winit::event::VirtualKeyCode::Up) => send_key_event(&key_sender, KeyType::Up, is_down),
                       Some(winit::event::VirtualKeyCode::Down) => send_key_event(&key_sender, KeyType::Down, is_down),
                       Some(winit::event::VirtualKeyCode::Left) => send_key_event(&key_sender, KeyType::Left, is_down),
                       Some(winit::event::VirtualKeyCode::Right) => send_key_event(&key_sender, KeyType::Right, is_down),
                       Some(winit::event::VirtualKeyCode::Q) => send_key_event(&key_sender, KeyType::A, is_down),
                       Some(winit::event::VirtualKeyCode::S) => send_key_event(&key_sender, KeyType::B, is_down),
                       Some(winit::event::VirtualKeyCode::W) => send_key_event(&key_sender, KeyType::Select, is_down),
                       Some(winit::event::VirtualKeyCode::X) => send_key_event(&key_sender, KeyType::Start, is_down),
                       _ => (),
                   }
               }
               _ => (),
           }
        }
    });
}

fn send_key_event(
    key_sender: &Sender<Key>,
    key_type: KeyType,
    is_down: bool,
) {
    let _ = key_sender.send(Key {
        key_type,
        is_down,
    });
}