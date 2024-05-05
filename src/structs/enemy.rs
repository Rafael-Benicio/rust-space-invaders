use rand::Rng;
use crate::structs::shoot::Shoot;
use crate::FRAME_HATE;
use crate::structs::vector2d::Vector2D;
use crate::structs::collisionbody::CollisionBody;
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
    position:Vector2D<i32>,
    proportions:Vector2D<u32>,
    entity_type: EntityType,
    fisic_body: CollisionBody,
    shoot_frame_counter:i16,
    shoot_period:i16, 
}

impl Enemy {
    pub fn new(x :i32,y :i32,size: (u32, u32)) -> Self {
        let position_x:i32=size.0 as i32 * x;
        let position_y:i32=size.1 as i32 * y;

        let proportions_w:u32=size.0 - 10;
        let proportions_h:u32=size.1 - 10;

        Enemy {
            id: Uuid::new_v4(),
            entity_type: EntityType::Hostile,
            position:Vector2D::new(position_x, position_y),
            proportions:Vector2D::new(proportions_w,proportions_h),
            rect: Rect::new(
                position_x, position_y, 
                proportions_w, proportions_h
                ),
            fisic_body: CollisionBody::new(
                position_x, position_y, 
                proportions_w, proportions_h
                ),
            color: Color::RGB(255, 255, 255),
            shoot_frame_counter:0,
            shoot_period:rand::thread_rng().gen_range(5..15),
        }
    }

    fn get_center_point(&self) -> Vector2D<i32> {
        Vector2D::new(
            self.position.x + (self.proportions.x as i32 / 2),
            self.position.y,
        )
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

        println!("{:?}",(FRAME_HATE * self.shoot_period) );

        if FRAME_HATE * self.shoot_period == self.shoot_frame_counter {
            self.shoot_frame_counter=0;
            return Some(
                UpdateComands::Shoot(
                    Shoot::new(self.get_center_point(), self.entity_type)
                ));
        }

        self.shoot_frame_counter+=1;

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
