use crate::structs::enemy::Enemy;
use crate::structs::shoot::Shoot;
use crate::traits::base_game_flow::BaseGameFlow;
use crate::traits::controler::Control;
use crate::traits::draw::Draw;

use uuid::Uuid;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::{Window, WindowBuildError};
use sdl2::EventPump;
use sdl2::VideoSubsystem;

pub mod state;
pub mod structs;
pub mod traits;

pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 600;
pub const FRAME_HATE: i16 = 60;
pub const ENTITY_SIZE: (u32, u32) = (WINDOW_WIDTH / 13, WINDOW_HEIGHT / 16);

pub enum UpdateComands {
    Remove(Uuid),
    Shoot(Shoot),
}

#[derive(PartialEq, Copy, Clone)]
pub enum EntityType {
    Hostile,
    Friendily,
}

pub fn event_listener(
    event_pump: &mut EventPump,
    entity_game: &mut Vec<Box<dyn BaseGameFlow>>,
) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return false,
            _ => {
                if let Some(shoot) = entity_game[0].control(event) {
                    entity_game.push(Box::new(shoot));
                }
            }
        }
    }
    true
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

pub fn enemys_instance(entity_game: &mut Vec<Box<dyn BaseGameFlow>>) {
    for pos_x in 1..=11 {
        for pos_y in 1..=5 {
            let mut my_rect: Enemy = Enemy::new(pos_x, pos_y, ENTITY_SIZE);
            my_rect.set_color(pos_x as u8 * 10, pos_y as u8 * 10, 0);
            entity_game.push(Box::new(my_rect));
        }
    }
}
