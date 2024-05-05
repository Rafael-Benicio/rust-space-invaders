use crate::base::collisionbody::CollisionBody;
use crate::traits::collision::BoxCollision;
use crate::traits::draw::Draw;
use crate::traits::update::Update;
use crate::BaseGameFlow;
use crate::Control;
use crate::EntityType;
use crate::UpdateComands;
use crate::Uuid;
use crate::Window;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;

pub struct Enemy {
    id: Uuid,
    rect: Rect,
    color: Color,
    entity_type: EntityType,
    fisic_body: CollisionBody,
}

impl Enemy {
    pub fn new(size: (u32, u32)) -> Self {
        Enemy {
            id: Uuid::new_v4(),
            entity_type: EntityType::Hostile,
            rect: Rect::new(size.0 as i32, size.1 as i32, size.0, size.1),
            fisic_body: CollisionBody::new(size.0 as i32, size.1 as i32, size.0, size.1),
            color: Color::RGB(255, 255, 255),
        }
    }
}

impl Drop for Enemy {
    fn drop(&mut self) {
        println!("Inimigo dropado");
    }
}

impl BaseGameFlow for Enemy {
    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_type(&self) -> EntityType {
        self.entity_type
    }
}

impl Draw for Enemy {
    fn set_color(&mut self, r: u8, g: u8, b: u8) {
        self.color = Color::RGB(r, g, b)
    }

    fn render(&self, canvas: &mut Canvas<Window>) {
        // canvas.clear();
        canvas.set_draw_color(self.color);
        let _ = canvas.draw_rect(self.rect);
        let _ = canvas.fill_rect(self.rect);
    }
}

impl Update for Enemy {
    fn update(&mut self) -> Option<UpdateComands> {
        if self.fisic_body.is_colliding {
            return Some(UpdateComands::Remove(self.get_id()));
        }
        None
    }
}

impl BoxCollision for Enemy {
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

impl Control for Enemy {}
