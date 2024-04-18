use sdl2::event::Event;

pub trait Control {
    fn control(&mut self, event: Event);
}
