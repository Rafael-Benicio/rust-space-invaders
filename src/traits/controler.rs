use crate::base::shoot::Shoot;

use sdl2::event::Event;

pub trait Control {
    fn control(&mut self, _event: Event) -> Option<Shoot> {
        None
    }

    fn set_position(&mut self, _x: i32, _y: i32) {}
}
