use crate::UpdateComands;

pub trait Update {
    fn update(&mut self) -> Option<UpdateComands> {
        None
    }
}
