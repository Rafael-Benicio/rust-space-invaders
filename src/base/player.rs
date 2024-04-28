use crate::base::collisionbody::CollisionBody;
use crate::base::vector2d::Vector2D;

use crate::traits::controler::Control;
use crate::traits::draw_base::BaseDrawFunction;
use crate::{FRAME_HATE, WINDOW_WIDTH};
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
};

pub struct Player {
    position:Vector2D<i32>,
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
            position:Vector2D::new((size.0 * 6) as i32, (size.1 * 14) as i32),
            rect: Rect::new((size.0 * 6) as i32, (size.1 * 14) as i32, size.0, size.1),
            fisic_body: CollisionBody::new(
                (size.0 * 6) as i32,
                (size.1 * 14) as i32,
                size.0,
                size.1,
            ),
            direction: Vector2D::new(0,0),
            max_vel: 10,
            momentum: Vector2D::new(0,0),
            acceleration: 2,
            momentum_frame_counter: 0,
            color: Color::RGB(255, 255, 255),
        }
    }
}

impl BaseDrawFunction for Player {
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
    fn control(&mut self, event: Event) {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => self.direction.x = -1,
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => self.direction.x = 1,

            _ => {}
        };
    }

    fn set_position(&mut self, x: i32, _y: i32) {

        self.position.x = if x < 0 {
            self.momentum.x-=self.momentum.x;
            0
        } else if x + self.fisic_body.proportions.x as i32 > WINDOW_WIDTH as i32 {
            self.momentum.x-=self.momentum.x;
            WINDOW_WIDTH as i32 - self.fisic_body.proportions.x as i32
        } else {
            x
        };
        self.rect.x=self.position.x;
        self.fisic_body.set_position(self.position.x, 0);
    }

    fn update(&mut self) {
        let accel = self.direction.x * self.acceleration as i32;
        self.direction.x = 0;
        self.momentum.x = accel + self.momentum.x;

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
    }
}
