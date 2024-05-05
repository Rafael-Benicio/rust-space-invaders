extern crate sdl2;

use game::base::bloco::Retangulo;
use game::base::collisionbody::CollisionBody;
use game::base::player::Player;
use game::state::GameState;
use game::traits::base_game_flow::BaseGameFlow;
use game::traits::draw::Draw;
use game::EntityType;
use game::UpdateComands;
use game::{create_window, event_listener};
use game::{WINDOW_HEIGHT, WINDOW_WIDTH};

use uuid::Uuid;

use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::{Sdl, VideoSubsystem};

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
    let mut drop_pool: Vec<Uuid> = Vec::new();
    let mut collision_pool: Vec<(CollisionBody, EntityType)> = Vec::new();

    let mut player: Player = Player::new(entity_size);
    let mut my_rect_2: Retangulo = Retangulo::new(entity_size);
    player.set_color(255, 255, 255);
    my_rect_2.set_color(255, 255, 0);

    let mut entity_game: Vec<Box<dyn BaseGameFlow>> = Vec::new();
    entity_game.push(Box::new(player));
    entity_game.push(Box::new(my_rect_2));

    'running: loop {
        game_state.window.set_draw_color(Color::RGB(0, 0, 0));
        game_state.window.clear();

        if !event_listener(&mut event_pump, &mut entity_game) {
            break 'running;
        };

        for entity in entity_game.iter_mut() {
            if let Some(UpdateComands::Remove(val)) = entity.update() {
                drop_pool.push(val)
            } else {
                collision_pool.push(entity.collision_box())
            }
        }

        for entity in entity_game.iter_mut() {
            for (colliders, entity_type) in collision_pool.iter() {
                if entity.get_type() != *entity_type {
                    entity.aabb_collision(colliders);
                }
            }
        }

        entity_game.retain(|entity| !drop_pool.contains(&entity.get_id()));

        drop_pool.clear();
        collision_pool.clear();

        for entity in entity_game.iter_mut() {
            entity.render(&mut game_state.window);
        }

        game_state.window.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
