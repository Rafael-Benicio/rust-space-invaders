use crate::base::collisionbody::CollisionBody;

pub trait BoxCollision {
    fn aabb_collision(&mut self, rect: CollisionBody) -> bool;
    fn collision_box(&self) -> CollisionBody;
}
