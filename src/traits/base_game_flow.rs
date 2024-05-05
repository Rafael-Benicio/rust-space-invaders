use crate::traits::collision::BoxCollision;
use crate::traits::controler::Control;
use crate::traits::draw::Draw;
use crate::traits::update::Update;
use crate::EntityType;

use crate::Uuid;

pub trait BaseGameFlow: Update + Draw + Control + BoxCollision {
    fn get_id(&self) -> Uuid;

    fn get_type(&self) -> EntityType;
}
