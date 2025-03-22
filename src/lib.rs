use std::collections::HashMap;
use std::path::Path;

use crate::structs::enemy::Enemy;

use crate::traits::base_game_flow::BaseGameFlow;
use crate::traits::controler::Control;

use sdl2::image::LoadTexture;
use sdl2::render::{Canvas, Texture, TextureCreator};
use uuid::Uuid;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::{Window, WindowContext};
use sdl2::VideoSubsystem;
use sdl2::{EventPump, Sdl};

pub mod enums;
pub mod state;
pub mod structs;
pub mod traits;

pub const WINDOW_WIDTH: u32 = 800;
pub const ENTITY_COLUNMS_N: i32 = 11;
pub const WINDOW_HEIGHT: u32 = 600;
pub const FRAME_HATE: i16 = 60;
pub const ENTITY_SIZE: (u32, u32) = (WINDOW_WIDTH / 13, WINDOW_HEIGHT / 16);
pub const ASSETS_PATH: &str = "./src/assets/";
pub const TEXTURE_FILES: [&str; 6] = [
    "player.png",
    "ship_1.png",
    "ship_2.png",
    "ship_3.png",
    "ship_4.png",
    "ship_5.png",
];

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

pub fn init_game(
    title: &str,
    width: u32,
    height: u32,
) -> (EventPump, TextureCreator<WindowContext>, Canvas<Window>) {
    let sdl_context: Sdl = sdl2::init().expect("Erro in sdl2 init");
    let video_subsystem: VideoSubsystem = sdl_context
        .video()
        .expect("Erro in VideoSubsystem creation");

    let window = video_subsystem
        .window(title, width, height)
        .position_centered()
        .build()
        .expect("Erro in window creation")
        .into_canvas()
        .build()
        .expect("Erro in GameState creation");

    let event_pump = sdl_context.event_pump().unwrap();

    (event_pump, window.texture_creator(), window)
}

pub fn load_image<'a>(
    texture_creator: &'a mut TextureCreator<WindowContext>,
) -> HashMap<String, Texture<'a>> {
    let mut texture: HashMap<String, Texture> = Default::default();

    for txr_file in TEXTURE_FILES {
        let path = format!("{}{}", ASSETS_PATH, txr_file);
        match texture_creator.load_texture(Path::new(&path)) {
            Ok(txr) => texture.insert(txr_file.to_string(), txr),
            Err(_) => {
                panic!("Não conseguiu carregar")
            }
        };
    }

    texture
}

#[macro_export]
macro_rules! frame_hate {
    ($a:expr) => {
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / $a));
    };
}

#[macro_export]
macro_rules! draw_simple_rect_image {
    ($canvas:expr,$txt_map:expr,$self:expr) => {{
        let color = $self.get_color().unwrap_or(Color {
            r: 0,
            g: 0,
            b: 0,
            a: u8::MAX,
        });

        if let Some(txt_name) = $self.get_texture_name() {
            if let Some(txr) = $txt_map.get_mut(txt_name) {
                txr.set_color_mod(color.r, color.g, color.b);
                let _ = $canvas.copy(&txr, None, *$self.get_draw_rect());
            }
        } else {
            $canvas.set_draw_color(color);
            let _ = $canvas.draw_rect(*$self.get_draw_rect());
            let _ = $canvas.fill_rect(*$self.get_draw_rect());
        }
    }};
}

#[macro_export]
macro_rules! draw_simple_rect {
    ($canvas:expr,$self:expr) => {{
        $canvas.set_draw_color($self.color);
        let _ = $canvas.draw_rect(*$self.get_draw_rect());
        let _ = $canvas.fill_rect(*$self.get_draw_rect());
    }};
}

#[macro_export]
macro_rules! keydown {
    ($key:pat) => {
        KeyDown {
            keycode: Some($key),
            ..
        }
    };
}

#[macro_export]
macro_rules! keyup {
    ($key:pat) => {
        KeyUp {
            keycode: Some($key),
            ..
        }
    };
}
