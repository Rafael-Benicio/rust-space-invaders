use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct GameState {
    pub run: bool,
    pub window: Canvas<Window>,
}

impl GameState {
    pub fn new(window: Canvas<Window>) -> Self {
        GameState {
            run: false,
            window: window,
        }
    }
}
