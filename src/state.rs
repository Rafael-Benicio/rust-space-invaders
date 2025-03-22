use crate::traits::draw::Draw;
use crate::BaseGameFlow;
use crate::Enemy;
use crate::ENTITY_COLUNMS_N;
use crate::ENTITY_SIZE;
use crate::TEXTURE_FILES;
use rand::Rng;

pub struct GameState {
    pub run: bool,
    pub level: i32,
    pub enemy_movement_direction: i32,
    pub enemy_counter: i32,
    pub enemy_killed: i32,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            run: true,
            level: 1,
            enemy_movement_direction: -1,
            enemy_counter: 0,
            enemy_killed: 0,
        }
    }
}

impl GameState {
    pub fn enemys_instance(entity_game: &mut Vec<Box<dyn BaseGameFlow>>, n_rows: i32) -> i32 {
        for pos_x in 1..=ENTITY_COLUNMS_N {
            for pos_y in 1..=n_rows {
                let r: u8 = (pos_x * 255 / ENTITY_COLUNMS_N) as u8;
                let g: u8 = (pos_y * 255 / n_rows) as u8;
                let b: u8 = ((ENTITY_COLUNMS_N - pos_x) * 255 / ENTITY_COLUNMS_N) as u8;

                let txr_index: usize = match pos_y {
                    1 => rand::thread_rng().gen_range(3..=4),
                    2 | 3 => rand::thread_rng().gen_range(1..=2),
                    _ => 5,
                };

                let mut my_rect: Enemy =
                    Enemy::new(pos_x, pos_y, ENTITY_SIZE, TEXTURE_FILES[txr_index]);
                my_rect.set_color(r, g, b);
                entity_game.push(Box::new(my_rect));
            }
        }

        n_rows * ENTITY_COLUNMS_N
    }
}
