use crate::structs::shoot::Shoot;

use sdl2::event::Event;

pub trait Control {
    fn control(&mut self, _event: Event) -> Option<Shoot> {
        None
    }

    fn set_position(&mut self, _x: i32, _y: i32) {}

    fn go_forward(&mut self, _advance: i32) {}
}
