pub trait BoxColision {
    fn aabb_collision(&self, rect: (i32, i32, i32, i32)) -> bool;
    fn collision_box(&self) -> (i32, i32, i32, i32);
}
