use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct GameState {
    pub run: bool,
    pub window: Canvas<Window>,
    pub enemy_movement_direction: i32,
    pub enemy_counter: i32,
    pub enemy_kiled: i32,
}

impl GameState {
    pub fn new(window: Canvas<Window>) -> Self {
        GameState {
            run: false,
            window,
            enemy_movement_direction: -1,
            enemy_counter: 0,
            enemy_kiled: 0,
        }
    }
}
