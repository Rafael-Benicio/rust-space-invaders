use crate::enums::update_commands::UpdateCommands;
use crate::state::GameState;

pub trait Update {
    fn update(&mut self, _game_state: &GameState) -> Option<UpdateCommands> {
        None
    }
}
