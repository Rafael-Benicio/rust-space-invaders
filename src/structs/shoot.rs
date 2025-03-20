use crate::BaseGameFlow;
use crate::traits::collision::BoxCollision;
use crate::enums::entity_enum::EntityType;
use crate::enums::entity_enum::FriendilyType;
use crate::enums::update_commands::UpdateCommands;
use crate::state::GameState;
use crate::structs::collisionbody::CollisionBody;
use crate::structs::vector2d::Vector2D;
use crate::traits::draw::Draw;
use crate::traits::update::Update;
use crate::Control;
use sdl2::render::Texture;
use std::collections::HashMap;

use uuid::Uuid;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use space_macros::{BaseGameFlow,BoxCollision};

#[derive(BaseGameFlow,BoxCollision)]
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

impl Draw for Shoot {
    fn set_color(&mut self, r: u8, g: u8, b: u8) {
        self.color = Color::RGB(r, g, b)
    }

    fn render(&self, canvas: &mut Canvas<Window>, _textures: &mut HashMap<String, Texture<'_>>) {
        canvas.set_draw_color(self.color);
        let _ = canvas.draw_rect(self.rect);
        let _ = canvas.fill_rect(self.rect);
    }
}

impl Update for Shoot {
    fn update(&mut self, _game_state: &GameState) -> Option<UpdateCommands> {
        self.position.y += if self.entity_type == EntityType::Friendily(FriendilyType::Shoot) {
            -self.shoot_vel
        } else {
            self.shoot_vel
        };

        self.rect.y = self.position.y;
        self.fisic_body.position.y = self.position.y;

        if self.fisic_body.position.y < 0 || self.fisic_body.is_colliding {
            return Some(UpdateCommands::Remove(self.get_id()));
        }

        None
    }
}

impl Control for Shoot {}