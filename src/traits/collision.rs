use crate::structs::collisionbody::CollisionBody;
use crate::EntityType;

pub trait BoxCollision {
    fn aabb_collision(&mut self, rect: &CollisionBody);
    fn collision_box(&self) -> (CollisionBody, EntityType);
}
