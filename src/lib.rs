use crate::traits::base_game_flow::BaseGameFlow;
use crate::traits::controler::Control;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::{Window, WindowBuildError};
use sdl2::EventPump;
use sdl2::VideoSubsystem;

pub mod base;
pub mod state;
pub mod traits;

pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 600;
pub const FRAME_HATE: i16 = 60;

pub fn event_listener(event_pump: &mut EventPump, player: &mut Box<dyn BaseGameFlow>) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return false,
            _ => {
                player.control(event);
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
