use sdl2::{render::Canvas, video::Window};

pub trait BaseDrawFunction {
    fn new(x: i32, y: i32, width: u32, height: u32) -> Self;

    fn _set_color(&mut self, r: u8, g: u8, b: u8);

    fn render(&self, canvas: &mut Canvas<Window>);

    fn set_position(&mut self, x: i32, y: i32);

    fn update_position(&mut self, x: i32, y: i32);
}
