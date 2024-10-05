use crate::sdl2::image::LoadTexture;
extern crate sdl2;

use game::enums::entity_enum::EntityType;
use game::enums::entity_enum::FriendilyType;
use game::enums::entity_enum::HostileType;
use game::enums::update_commands::UpdateCommands;
use game::state::GameState;
use game::structs::collisionbody::CollisionBody;
use game::structs::player::Player;
use game::structs::shoot::Shoot;
use game::traits::base_game_flow::BaseGameFlow;
use game::traits::draw::Draw;
use game::ENTITY_COLUNMS_N;
use game::{create_window, enemys_instance, event_listener};
use game::{ASSETS_PATH, ENTITY_SIZE, TEXTURE_FILES, WINDOW_HEIGHT, WINDOW_WIDTH};

use sdl2::render::Texture;
use uuid::Uuid;

use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::{Sdl, VideoSubsystem};

use std::collections::HashMap;
use std::path::Path;
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

    let texture_creator = game_state.window.texture_creator();
    // -------------------------------------------------------------------------------------
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut drop_pool: Vec<Uuid> = Vec::new();
    let mut collision_pool: Vec<(CollisionBody, EntityType)> = Vec::new();
    let mut shoot_pool: Vec<Shoot> = Vec::new();
    let mut entity_game: Vec<Box<dyn BaseGameFlow>> = Vec::new();
    let mut direction_flag: i32 = -1;
    let mut advance_flag_counter = 0;
    let mut texture: HashMap<String, Texture> = Default::default();

    let mut player: Player = Player::new(ENTITY_SIZE, TEXTURE_FILES[0]);
    player.set_color(255, 0, 255);
    entity_game.push(Box::new(player));

    game_state.enemy_counter = enemys_instance(&mut entity_game, 5);

    for txr_file in TEXTURE_FILES {
        let path = format!("{}{}", ASSETS_PATH, txr_file);
        match texture_creator.load_texture(Path::new(&path)) {
            Ok(txr) => texture.insert(txr_file.to_string(), txr),
            Err(_) => {
                panic!("Não conseguiu carregar")
            }
        };
    }

    'running: loop {
        game_state.window.set_draw_color(Color::RGB(0, 0, 0));
        game_state.window.clear();

        // Input
        if !event_listener(&mut event_pump, &mut entity_game) || !game_state.run {
            break 'running;
        };

        // Update
        for entity in entity_game.iter_mut() {
            match entity.update(&game_state) {
                Some(UpdateCommands::Remove(id)) => drop_pool.push(id),
                Some(UpdateCommands::Shoot(shoot)) => shoot_pool.push(shoot),
                Some(UpdateCommands::MoveDirection(direction)) => direction_flag = direction,
                None => collision_pool.push(entity.collision_box()),
            }
        }

        game_state.enemy_movement_direction = direction_flag;

        while let Some(shoot) = shoot_pool.pop() {
            entity_game.push(Box::new(shoot));
        }

        for entity in entity_game.iter_mut() {
            for (colliders, entity_type) in collision_pool.iter() {
                if entity.get_type().diff(entity_type) {
                    entity.aabb_collision(colliders);
                }
            }
        }

        entity_game.retain(|entity| {
            let drop_item: bool = !drop_pool.contains(&entity.get_id());
            if !drop_item {
                match entity.get_type() {
                    EntityType::Hostile(HostileType::Enemy) => game_state.enemy_killed += 1,
                    EntityType::Friendily(FriendilyType::Player) => {
                        game_state.run = false;
                    }
                    _ => {}
                }
            }

            drop_item
        });

        if game_state.enemy_killed % ENTITY_COLUNMS_N == 0
            && game_state.enemy_killed != advance_flag_counter
        {
            for entity in entity_game.iter_mut() {
                if entity.get_type() == EntityType::Hostile(HostileType::Enemy) {
                    advance_flag_counter = game_state.enemy_killed;
                    entity.go_forward(1);
                }
            }
        }

        drop_pool.clear();
        collision_pool.clear();

        for entity in entity_game.iter_mut() {
            entity.render(&mut game_state.window, &mut texture);
        }

        if game_state.enemy_counter == game_state.enemy_killed {
            game_state.level += 1;
            game_state.enemy_killed = 0;
            game_state.enemy_counter = enemys_instance(&mut entity_game, 4 + game_state.level);
        }

        game_state.window.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
