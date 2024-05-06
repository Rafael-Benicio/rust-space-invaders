use crate::structs::collisionbody::CollisionBody;
use crate::structs::vector2d::Vector2D;
use crate::traits::collision::BoxCollision;
use crate::traits::draw::Draw;
use crate::traits::update::Update;
use crate::BaseGameFlow;
use crate::Control;
use crate::EntityType;
use crate::UpdateComands;

use uuid::Uuid;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Shoot {
    id: Uuid,
    entity_type: EntityType,
    position: Vector2D<i32>,
    color: Color,
    rect: Rect,
    fisic_body: CollisionBody,
    shoot_vel: i32,
}

impl Shoot {
    pub fn new(shoot_point: Vector2D<i32>, entity_type: EntityType) -> Self {
        Shoot {
            id: Uuid::new_v4(),
            position: Vector2D::new(shoot_point.x - 5, shoot_point.y - 20),
            color: Color::RGB(255, 255, 255),
            rect: Rect::new(shoot_point.x - 5, shoot_point.y - 20, 10, 10),
            fisic_body: CollisionBody::new(shoot_point.x - 5, shoot_point.y - 20, 10, 10),
            shoot_vel: 3,
            entity_type,
        }
    }
}

impl BaseGameFlow for Shoot {
    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_type(&self) -> EntityType {
        self.entity_type
    }
}

impl Draw for Shoot {
    fn set_color(&mut self, r: u8, g: u8, b: u8) {
        self.color = Color::RGB(r, g, b)
    }

    fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.color);
        let _ = canvas.draw_rect(self.rect);
        let _ = canvas.fill_rect(self.rect);
    }
}

impl Update for Shoot {
    fn update(&mut self) -> Option<UpdateComands> {
        self.position.y += if self.entity_type == EntityType::Friendily {
            -self.shoot_vel
        } else {
            self.shoot_vel
        };

        self.rect.y = self.position.y;
        self.fisic_body.position.y = self.position.y;

        if self.fisic_body.position.y < 0 || self.fisic_body.is_colliding {
            return Some(UpdateComands::Remove(self.get_id()));
        }

        None
    }
}

impl Control for Shoot {}

impl BoxCollision for Shoot {
    fn aabb_collision(&mut self, rect: &CollisionBody) {
        if (rect.right_side()) > self.fisic_body.left_side()
            && (self.fisic_body.right_side()) > rect.left_side()
            && (rect.botton_side()) > self.fisic_body.top_side()
            && (self.fisic_body.botton_side()) > rect.top_side()
        {
            self.fisic_body.is_colliding = true;
        }
    }

    fn collision_box(&self) -> (CollisionBody, EntityType) {
        (self.fisic_body.clone(), self.entity_type)
    }
}
