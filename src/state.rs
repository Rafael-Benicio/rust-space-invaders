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
