extern crate sdl2;

use game::base::bloco::Retangulo;
use game::base::player::Player;
use game::state::GameState;
use game::traits::base_game_flow::BaseGameFlow;
use game::traits::draw::Draw;
use game::{create_window, event_listener};
use game::{WINDOW_HEIGHT, WINDOW_WIDTH};

use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::{Sdl, VideoSubsystem};

use std::collections::LinkedList;
use std::time::Duration;

pub fn main() {
    let entity_size: (u32, u32) = (WINDOW_WIDTH / 13, WINDOW_HEIGHT / 16);

    let sdl_context: Sdl = sdl2::init().expect("Erro in sdl2 init");
    let video_subsystem: VideoSubsystem = sdl_context
        .video()
        .expect("Erro in VideoSubsystem creation");
    let window: Window = create_window(
        "Space Invaders ",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        &video_subsystem,
    )
    .expect("Erro in window creation");

    let mut game_state: GameState = GameState::new(
        window
            .into_canvas()
            .build()
            .expect("Erro in GameState creation"),
    );
    // -------------------------------------------------------------------------------------
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut player: Player = Player::new(entity_size);
    let mut my_rect_2: Retangulo = Retangulo::new(250, 250, 50, 50);
    player.set_color(255, 255, 255);
    my_rect_2.set_color(255, 255, 0);

    let mut entity_game: LinkedList<Box<dyn BaseGameFlow>> = LinkedList::new();
    entity_game.push_back(Box::new(player));
    entity_game.push_back(Box::new(my_rect_2));

    'running: loop {
        if !event_listener(&mut event_pump, &mut entity_game) {
            break 'running;
        };

        for entity in entity_game.iter_mut() {
            entity.update();
        }

        game_state.window.set_draw_color(Color::RGB(0, 0, 0));
        game_state.window.clear();

        for entity in entity_game.iter_mut() {
            entity.render(&mut game_state.window);
        }

        game_state.window.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
