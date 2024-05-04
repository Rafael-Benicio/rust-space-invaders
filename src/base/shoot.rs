use crate::base::collisionbody::CollisionBody;
use crate::base::vector2d::Vector2D;
use crate::traits::draw::Draw;
use crate::traits::update::Update;
use crate::BaseGameFlow;
use crate::Control;
use crate::UpdateComands;

use uuid::Uuid;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Shoot {
    id: Uuid,
    position: Vector2D<i32>,
    color: Color,
    rect: Rect,
    fisic_body: CollisionBody,
    shoot_vel: i32,
    by_player: bool,
}

impl Shoot {
    pub fn new(shoot_point: Vector2D<i32>, actor: bool) -> Self {
        Shoot {
            id: Uuid::new_v4(),
            position: Vector2D::new(shoot_point.x - 5, shoot_point.y),
            color: Color::RGB(255, 255, 255),
            rect: Rect::new(shoot_point.x - 5, shoot_point.y - 5, 10, 10),
            fisic_body: CollisionBody::new(shoot_point.x - 5, shoot_point.y - 5, 10, 10),
            shoot_vel: 10,
            by_player: actor,
        }
    }
}

impl Drop for Shoot {
    fn drop(&mut self) {
        println!("Shoot está sendo dropado \n → {}", self.id);
    }
}

impl BaseGameFlow for Shoot {
    fn get_id(&self) -> Uuid {
        self.id
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
        self.position.y += if self.by_player {
            -self.shoot_vel
        } else {
            self.shoot_vel
        };

        self.rect.y = self.position.y;
        self.fisic_body.position.y = self.position.y;

        if self.fisic_body.position.y < 0 {
            return Some(UpdateComands::Remove(self.get_id()));
        }

        None
    }
}

impl Control for Shoot {}
