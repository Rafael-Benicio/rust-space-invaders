use crate::enums::entity_enum::FriendilyType;
use crate::enums::entity_enum::EntityType;
use crate::enums::update_commands::UpdateCommands;
use crate::state::GameState;
use crate::structs::collisionbody::CollisionBody;
use crate::structs::shoot::Shoot;
use crate::structs::vector2d::Vector2D;
use crate::traits::base_game_flow::BaseGameFlow;
use crate::traits::collision::BoxCollision;
use crate::traits::controler::Control;
use crate::traits::draw::Draw;
use crate::traits::update::Update;
use crate::Uuid;
use crate::FRAME_HATE;
use crate::WINDOW_WIDTH;

use sdl2::event::{Event, Event::KeyDown, Event::KeyUp};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Player {
    id: Uuid,
    entity_type: EntityType,
    position: Vector2D<i32>,
    proportions: Vector2D<u32>,
    rect: Rect,
    color: Color,
    acceleration: u8,
    fisic_body: CollisionBody,
    direction: Vector2D<i32>,
    frame_counter_shoot: i16,
    can_shoot: bool,
}

impl Player {
    pub fn new(size: (u32, u32)) -> Self {
        let position_x: i32 = (size.0 * 6) as i32;
        let position_y: i32 = (size.1 * 14) as i32;

        let proportions_w: u32 = size.0;
        let proportions_h: u32 = size.1;

        Player {
            entity_type: EntityType::Friendily(FriendilyType::Player),
            id: Uuid::new_v4(),
            position: Vector2D::new(position_x, position_y),
            proportions: Vector2D::new(size.0, size.1),
            rect: Rect::new(position_x, position_y, proportions_w, proportions_h),
            fisic_body: CollisionBody::new(position_x, position_y, proportions_w, proportions_h),
            direction: Vector2D::new(0, 0),
            acceleration: 2,
            color: Color::RGB(255, 255, 255),
            frame_counter_shoot: 0,
            can_shoot: true,
        }
    }

    fn get_center_point(&self) -> Vector2D<i32> {
        Vector2D::new(
            self.position.x + (self.proportions.x as i32 / 2),
            self.position.y,
        )
    }
}

impl BaseGameFlow for Player {
    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_type(&self) -> EntityType {
        self.entity_type
    }
}

impl Draw for Player {
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

impl Control for Player {
    fn control(&mut self, event: Event) -> Option<Shoot> {
        match event {
            KeyDown {
                keycode: Some(Keycode::Left),
                ..
            }
            | KeyDown {
                keycode: Some(Keycode::A),
                ..
            } => {
                self.direction.x = -1;
            }
            KeyDown {
                keycode: Some(Keycode::Right),
                ..
            }
            | KeyDown {
                keycode: Some(Keycode::D),
                ..
            } => {
                self.direction.x = 1;
            }
            KeyDown {
                keycode: Some(Keycode::RCtrl),
                ..
            }
            | KeyDown {
                keycode: Some(Keycode::LCtrl),
                ..
            } => {
                self.acceleration = 10;
            }
            KeyUp {
                keycode: Some(Keycode::RCtrl),
                ..
            }
            | KeyUp {
                keycode: Some(Keycode::LCtrl),
                ..
            } => {
                self.acceleration = 2;
            }
            KeyDown {
                keycode: Some(Keycode::Return),
                ..
            }
            | KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => {
                if self.can_shoot {
                    self.can_shoot = false;
                    return Some(Shoot::new(self.get_center_point(), EntityType::Friendily(FriendilyType::Shoot)));
                }
            }
            _ => {}
        };

        None
    }

    fn set_position(&mut self, x: i32, _y: i32) {
        self.position.x = x;
        self.rect.x = x;
        self.fisic_body.set_position(x, 0);
    }
}

impl Update for Player {
    fn update(&mut self, _game_state: &GameState) -> Option<UpdateCommands> {
        let mut accel = self.acceleration as i32 * self.direction.x;

        accel = if (self.position.x + accel + self.proportions.x as i32) < WINDOW_WIDTH as i32
            && self.position.x + accel > 0
        {
            accel
        } else {
            0
        };

        if FRAME_HATE / 20 == self.frame_counter_shoot {
            self.can_shoot = true;
            self.frame_counter_shoot = 0;
        } else {
            self.frame_counter_shoot += 1
        }

        self.set_position(self.position.x + accel, 0);

        if self.fisic_body.is_colliding {
            return Some(UpdateCommands::Remove(self.get_id()));
        }

        None
    }
}

impl BoxCollision for Player {
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
