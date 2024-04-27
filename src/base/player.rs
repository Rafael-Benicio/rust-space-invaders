use crate::base::collisionbody::CollisionBody;
use crate::traits::collision::BoxCollision;
use crate::traits::controler::Control;
use crate::traits::draw_base::BaseDrawFunction;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
};

pub struct Player {
    rect: Rect,
    color: Color,
    acceleration: u8,
    fisic_body: CollisionBody,
}

impl Player {
    pub fn new(size: (u32, u32)) -> Self {
        Player {
            rect: Rect::new((size.0 * 6) as i32, (size.1 * 14) as i32, size.0, size.1),
            fisic_body: CollisionBody::new(
                (size.0 * 6) as i32,
                (size.1 * 14) as i32,
                size.0,
                size.1,
            ),
            acceleration: 10,
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
            } => self.update_position(-1, 0),
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => self.update_position(1, 0),

            _ => {}
        };
    }

    fn set_position(&mut self, x: i32, _y: i32) {
        self.rect.x = if x < 0 {
            0
        } else if x + self.fisic_body.proportions.x as i32 > 800 {
            800 - self.fisic_body.proportions.x as i32
        } else {
            x
        };
        self.fisic_body.set_position(self.rect.x, 0);
    }

    fn update_position(&mut self, x: i32, _y: i32) {
        self.set_position(self.rect.x + x * self.acceleration as i32, 0);
    }
}

impl BoxCollision for Player {
    fn aabb_collision(&self, rect: CollisionBody) -> bool {
        if (rect.right_side()) > self.fisic_body.left_side()
            && (self.fisic_body.right_side()) > rect.left_side()
            && (rect.botton_side()) > self.rect.y
            && (self.fisic_body.botton_side()) > rect.top_side()
        {
            return true;
        }
        false
    }
    fn collision_box(&self) -> CollisionBody {
        self.fisic_body.clone()
    }
}
