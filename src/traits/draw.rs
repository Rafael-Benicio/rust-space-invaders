use crate::Window;
use sdl2::render::Canvas;

pub trait Draw {
    fn set_color(&mut self, _r: u8, _g: u8, _b: u8) {}

    fn render(&self, _canvas: &mut Canvas<Window>) {}
}
