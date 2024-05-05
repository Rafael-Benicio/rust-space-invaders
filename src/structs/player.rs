use crate::base::collisionbody::CollisionBody;
use crate::base::shoot::Shoot;
use crate::base::vector2d::Vector2D;
use crate::traits::base_game_flow::BaseGameFlow;
use crate::traits::collision::BoxCollision;
use crate::traits::controler::Control;
use crate::traits::draw::Draw;
use crate::traits::update::Update;
use crate::EntityType;
use crate::UpdateComands;
use crate::Uuid;
use crate::{FRAME_HATE, WINDOW_WIDTH};

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
    max_vel: i32,
    direction: Vector2D<i32>,
    momentum: Vector2D<i32>,
    momentum_frame_counter: i16,
}

impl Player {
    pub fn new(size: (u32, u32)) -> Self {
        Player {
            entity_type: EntityType::Friendily,
            id: Uuid::new_v4(),
            position: Vector2D::new((size.0 * 6) as i32, (size.1 * 14) as i32),
            proportions: Vector2D::new(size.0, size.1),
            rect: Rect::new((size.0 * 6) as i32, (size.1 * 14) as i32, size.0, size.1),
            fisic_body: CollisionBody::new(
                (size.0 * 6) as i32,
                (size.1 * 14) as i32,
                size.0,
                size.1,
            ),
            direction: Vector2D::new(0, 0),
            max_vel: 10,
            momentum: Vector2D::new(0, 0),
            acceleration: 2,
            momentum_frame_counter: 0,
            color: Color::RGB(255, 255, 255),
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
            } => self.direction.x = -1,
            KeyDown {
                keycode: Some(Keycode::Right),
                ..
            }
            | KeyDown {
                keycode: Some(Keycode::D),
                ..
            } => self.direction.x = 1,
            KeyUp {
                keycode: Some(Keycode::Left),
                ..
            }
            | KeyUp {
                keycode: Some(Keycode::A),
                ..
            } => self.direction.x = 0,
            KeyUp {
                keycode: Some(Keycode::Right),
                ..
            }
            | KeyUp {
                keycode: Some(Keycode::D),
                ..
            } => self.direction.x = 0,
            KeyDown {
                keycode: Some(Keycode::Return),
                ..
            }
            | KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => return Some(Shoot::new(self.get_center_point(), self.entity_type)),
            _ => {}
        };

        None
    }

    fn set_position(&mut self, x: i32, _y: i32) {
        self.position.x = if x < 0 {
            self.momentum.x -= self.momentum.x;
            0
        } else if x + self.fisic_body.proportions.x as i32 > WINDOW_WIDTH as i32 {
            self.momentum.x -= self.momentum.x;
            WINDOW_WIDTH as i32 - self.fisic_body.proportions.x as i32
        } else {
            x
        };
        self.rect.x = self.position.x;
        self.fisic_body.set_position(self.position.x, 0);
    }
}

impl Update for Player {
    fn update(&mut self) -> Option<UpdateComands> {
        let accel = self.direction.x * self.acceleration as i32;
        self.momentum.x += accel;

        self.momentum.x = if self.momentum.x > self.max_vel {
            self.max_vel
        } else if self.momentum.x < -self.max_vel {
            -self.max_vel
        } else {
            self.momentum.x
        };

        if self.momentum_frame_counter > FRAME_HATE / 6 {
            let variation = self.momentum.x / 2;

            self.momentum_frame_counter = 0;
            self.momentum.x = match variation {
                0 => 0,
                _ => self.momentum.x - variation,
            };
        } else if accel == 0 {
            self.momentum_frame_counter += 1;
        }

        self.set_position(self.position.x + self.momentum.x, 0);

        if self.fisic_body.is_colliding {
            return Some(UpdateComands::Remove(self.get_id()));
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
            self.fisic_body.is_colliding = false;
        }
    }

    fn collision_box(&self) -> (CollisionBody, EntityType) {
        (self.fisic_body.clone(), self.entity_type)
    }
}
