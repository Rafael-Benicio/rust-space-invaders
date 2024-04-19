use crate::base::collisionbody::CollisionBody;
use crate::traits::collision::BoxCollision;
use crate::traits::controler::Control;
use crate::traits::draw_base::BaseDrawFunction;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
};

pub struct RetanguloChar {
    rect: Rect,
    color: Color,
    fisic_body: CollisionBody,
}

impl BaseDrawFunction for RetanguloChar {
    fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        RetanguloChar {
            rect: Rect::new(x, y, width, height),
            fisic_body: CollisionBody::new(x, y, width, height),
            color: Color::RGB(255, 255, 255),
        }
    }

    fn set_color(&mut self, r: u8, g: u8, b: u8) {
        self.color = Color::RGB(r, g, b)
    }

    fn render(&self, canvas: &mut Canvas<Window>) {
        // canvas.clear();
        canvas.set_draw_color(self.color);
        let _ = canvas.draw_rect(self.rect);
        let _ = canvas.fill_rect(self.rect);
    }

    fn set_position(&mut self, x: i32, y: i32) {
        self.rect.x = x;
        self.rect.y = y;
        self.fisic_body.set_position(x, y);
    }

    fn update_position(&mut self, x: i32, y: i32) {
        self.set_position(self.rect.x + x * 10, self.rect.y + y * 10);
    }
}

impl Control for RetanguloChar {
    fn control(&mut self, event: Event) {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => self.update_position(0, -1),
            Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => self.update_position(0, 1),
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
}

impl BoxCollision for RetanguloChar {
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
