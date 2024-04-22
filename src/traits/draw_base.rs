use sdl2::{render::Canvas, video::Window};

pub trait BaseDrawFunction {
    fn set_color(&mut self, r: u8, g: u8, b: u8);

    fn render(&self, canvas: &mut Canvas<Window>);
}
