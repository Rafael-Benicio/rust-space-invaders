use crate::base::retangulo::RetanguloChar;
use crate::traits::controler::Control;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::{Window, WindowBuildError};
use sdl2::EventPump;
use sdl2::VideoSubsystem;

pub mod base;
pub mod traits;

pub fn event_listener(event_pump: &mut EventPump, my_rect: &mut RetanguloChar) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return false,
            _ => {
                my_rect.control(event);
            }
        }
    }
    return true;
}

pub fn create_window(
    title: &str,
    width: u32,
    height: u32,
    video_subsystem: &VideoSubsystem,
) -> Result<Window, WindowBuildError> {
    video_subsystem
        .window(title, width, height)
        .position_centered()
        .build()
}
