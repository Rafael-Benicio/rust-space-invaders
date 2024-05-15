use crate::{state::GameState, UpdateComands};

pub trait Update {
    fn update(&mut self, _game_state: &GameState) -> Option<UpdateComands> {
        None
    }
}
