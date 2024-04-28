use sdl2::event::Event;

pub trait Control {
    fn control(&mut self, event: Event);

    fn set_position(&mut self, x: i32, y: i32);
}
