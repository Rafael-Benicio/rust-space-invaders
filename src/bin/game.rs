extern crate sdl2;

use game::state::GameState;
use game::structs::collisionbody::CollisionBody;
use game::structs::player::Player;
use game::structs::shoot::Shoot;
use game::traits::base_game_flow::BaseGameFlow;
use game::traits::draw::Draw;
use game::EntityType;
use game::UpdateComands;
use game::{create_window, enemys_instance, event_listener};
use game::{ENTITY_SIZE, WINDOW_HEIGHT, WINDOW_WIDTH};

use uuid::Uuid;

use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::{Sdl, VideoSubsystem};

use std::time::Duration;

pub fn main() {
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
    let mut shoot_pool: Vec<Shoot> = Vec::new();
    let mut entity_game: Vec<Box<dyn BaseGameFlow>> = Vec::new();
    let mut direction_flag: i32 = -1;

    let mut player: Player = Player::new(ENTITY_SIZE);
    player.set_color(255, 255, 255);
    entity_game.push(Box::new(player));

    enemys_instance(&mut entity_game);

    'running: loop {
        game_state.window.set_draw_color(Color::RGB(0, 0, 0));
        game_state.window.clear();

        if !event_listener(&mut event_pump, &mut entity_game) {
            break 'running;
        };

        for entity in entity_game.iter_mut() {
            match entity.update(&game_state) {
                Some(UpdateComands::Remove(id)) => drop_pool.push(id),
                Some(UpdateComands::Shoot(shoot)) => shoot_pool.push(shoot),
                Some(UpdateComands::MoveDirection(direction)) => direction_flag = direction,
                None => collision_pool.push(entity.collision_box()),
            }
        }

        game_state.enemy_movement_direction = direction_flag;

        while let Some(shoot) = shoot_pool.pop() {
            entity_game.push(Box::new(shoot));
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
