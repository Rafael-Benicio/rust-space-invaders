use std::collections::HashMap;

use crate::Window;
use sdl2::render::Canvas;
use sdl2::render::Texture;

pub trait Draw {
    fn set_color(&mut self, _r: u8, _g: u8, _b: u8) {}

    fn render(&self, _canvas: &mut Canvas<Window>, _textures: &mut HashMap<String, Texture>) {}
}
