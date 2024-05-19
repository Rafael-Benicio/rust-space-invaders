use crate::enums::entity_enum::EntityType;
use crate::structs::collisionbody::CollisionBody;

pub trait BoxCollision {
    fn aabb_collision(&mut self, rect: &CollisionBody);
    fn collision_box(&self) -> (CollisionBody, EntityType);
}
