use crate::structs::shoot::Shoot;
use crate::Uuid;

pub enum UpdateCommands {
    Remove(Uuid),
    MoveDirection(i32),
    Shoot(Shoot),
}
