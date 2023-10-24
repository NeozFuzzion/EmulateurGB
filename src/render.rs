
use glium::{self, glutin, texture, Surface};
use glutin::dpi::LogicalSize;
#[cfg(feature = "frame-capture")]
use image;
use std::borrow::Cow;
#[cfg(feature = "frame-capture")]
use std::fs::File;
#[cfg(feature = "frame-capture")]
use std::path::Path;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};
use crate::input::{Key, KeyType};


pub struct Renderer {
    display: glium::Display,
    texture: texture::texture2d::Texture2d,
    events_loop: glutin::EventsLoop,
    screen_data_receiver: Receiver<[u32;23040]>,
    key_sender: Sender<Key>,
}

impl Renderer {
    pub const WIDTH: u32 = 160;
    pub const HEIGHT: u32 = 144;

    pub fn new(
        title: &str,
        scale: u32,
        screen_data_receiver: Receiver<[u32;23040]>,
        key_sender: Sender<Key>,
    ) -> Self {
        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(LogicalSize::new(
                f64::from(Self::WIDTH * scale),
                f64::from(Self::HEIGHT * scale),
            ));

        let context = glutin::ContextBuilder::new();
        let display = match glium::Display::new(window, context, &events_loop) {
            Ok(d) => d,
            Err(e) => panic!("Failed to create display: {}", e),
        };
        let texture = match texture::texture2d::Texture2d::empty_with_format(
            &display,
            //no format u32 so translate u32 to u8u8u8 later
            texture::UncompressedFloatFormat::U8U8U8,
            texture::MipmapsOption::NoMipmap,
            Self::WIDTH,
            Self::HEIGHT,
        ) {
            Ok(t) => t,
            Err(e) => panic!("Failed to create texture: {}", e),
        };

        Self {
            display,
            texture,
            events_loop,
            screen_data_receiver,
            key_sender,
        }
    }

    pub fn start_loop(&mut self) {
        self.render_loop();
        loop {
            if let Err(mpsc::TryRecvError::Disconnected) = self.screen_data_receiver.try_recv() {
                break;
            }
        }
    }

    fn render_loop(&mut self) {
        let mut closed = false;

        while !closed {
            closed = self.window_events();
            match self.screen_data_receiver.try_recv() {
                Ok(data) => self.draw_data(&data),
                Err(mpsc::TryRecvError::Empty) => (),
                Err(mpsc::TryRecvError::Disconnected) => closed = true,
            }

        }
    }


    fn draw_data(&mut self, data: &[u32;23040]) {
        //glium doesn't like u32 texture so translate to u8u8u8
        let mut dataa= vec![0u8; (Self::WIDTH * Self::HEIGHT * 3) as usize];
        for i in 0..data.len() {
            dataa[i * 3] = (data[i] & 0xFF) as u8;
            dataa[i * 3 + 1] = ((data[i] >> 8) & 0xFF) as u8;
            dataa[i * 3 + 2] = ((data[i] >> 16) & 0xFF) as u8;
        }

        let raw_image_2d = glium::texture::RawImage2d {
            data: Cow::Borrowed(&dataa),
            width: Self::WIDTH,
            height: Self::HEIGHT,
            format: glium::texture::ClientFormat::U8U8U8,
        };

        self.texture.write(
            glium::Rect {
                left: 0,
                bottom: 0,
                width: Self::WIDTH,
                height: Self::HEIGHT,
            },
            raw_image_2d,
        );

        let target = self.display.draw();
        let (unsigned_width, unsigned_height) = target.get_dimensions();

        let width = i32::from(unsigned_width as u16);
        let height = i32::from(unsigned_height as u16);
        let blit_target = glium::BlitTarget {
            left: 0,
            bottom: height as u32,
            width,
            height: -height,
        };
        self.texture.as_surface().blit_whole_color_to(
            &target,
            &blit_target,
            glium::uniforms::MagnifySamplerFilter::Nearest,
        );
        if let Err(e) = target.finish() {
            println!("ERROR: Failed to write to display: {}", e)
        }
    }


    fn window_events(&mut self) -> bool {
        let mut closed = false;
        let key_sender = self.key_sender.clone();

        self.events_loop.poll_events(|ev| {
            if let glutin::Event::WindowEvent { event, .. } = ev {
                match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    glutin::WindowEvent::KeyboardInput { input, .. } => {
                        handle_keyboard_input(&key_sender, &input);
                    }
                    _ => (),
                }
            }
        });

        closed
    }

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

fn handle_keyboard_input(
    key_sender: &Sender<Key>,
    input: &glutin::KeyboardInput,
) {
    let is_down = input.state == glutin::ElementState::Pressed;

    match input.virtual_keycode {
        Some(glutin::VirtualKeyCode::Up) => send_key_event(key_sender, KeyType::Up, is_down),
        Some(glutin::VirtualKeyCode::Down) => send_key_event(key_sender, KeyType::Down, is_down),
        Some(glutin::VirtualKeyCode::Left) => send_key_event(key_sender, KeyType::Left, is_down),
        Some(glutin::VirtualKeyCode::Right) => send_key_event(key_sender, KeyType::Right, is_down),
        Some(glutin::VirtualKeyCode::Z) => send_key_event(key_sender, KeyType::A, is_down),
        Some(glutin::VirtualKeyCode::X) => send_key_event(key_sender, KeyType::B, is_down),
        Some(glutin::VirtualKeyCode::C) => send_key_event(key_sender, KeyType::Select, is_down),
        Some(glutin::VirtualKeyCode::V) => send_key_event(key_sender, KeyType::Start, is_down),
        Some(glutin::VirtualKeyCode::Space) => {
            // Handle Space key if needed
        }
        Some(glutin::VirtualKeyCode::Q) => {
            // Handle Q key if needed
        }
        _ => (),
    }
}