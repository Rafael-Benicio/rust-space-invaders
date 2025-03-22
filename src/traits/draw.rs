use std::collections::HashMap;
use std::u8;

use crate::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::Texture;

pub trait Draw {
    fn set_color(&mut self, _r: u8, _g: u8, _b: u8) {}

    fn get_color(&self) -> Option<Color> {
        None
    }

    fn get_texture_name(&self) -> Option<&String> {
        None
    }

    fn get_draw_rect(&self) -> &Rect;

    fn render(&self, _canvas: &mut Canvas<Window>, _textures: &mut HashMap<String, Texture>) {}
}
