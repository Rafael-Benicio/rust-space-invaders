use crate::base::vector2d::Vector2D;
use crate::traits::collision::BoxCollision;

#[derive(Clone)]
pub struct CollisionBody {
    pub position: Vector2D<i32>,
    pub proportions: Vector2D<u32>,
    pub is_colliding: bool,
}

impl CollisionBody {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        CollisionBody {
            position: Vector2D { x: x, y: y },
            proportions: Vector2D {
                x: width,
                y: height,
            },
            is_colliding: false,
        }
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.position.x = x;
        self.position.y = y;
    }

    pub fn top_side(&self) -> i32 {
        self.position.y
    }

    pub fn botton_side(&self) -> i32 {
        self.position.y + self.proportions.y as i32
    }

    pub fn left_side(&self) -> i32 {
        self.position.x
    }

    pub fn right_side(&self) -> i32 {
        self.position.x + self.proportions.x as i32
    }
}

impl BoxCollision for CollisionBody {
    fn aabb_collision(&mut self, rect: CollisionBody) -> bool {
        if (rect.right_side()) > self.left_side()
            && (self.right_side()) > rect.left_side()
            && (rect.botton_side()) > self.top_side()
            && (self.botton_side()) > rect.top_side()
        {
            self.is_colliding = true;
            return true;
        }
        self.is_colliding = false;
        false
    }

    fn collision_box(&self) -> CollisionBody {
        self.clone()
    }
}
