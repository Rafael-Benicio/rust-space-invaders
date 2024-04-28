use crate::base::collisionbody::CollisionBody;
use crate::base::vector2d::Vector2D;

use crate::traits::draw_base::BaseDrawFunction;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

struct Shoot {
    color: Color,
    rect: Rect,
    fisic_body: CollisionBody,
}

impl Shoot {
    fn new(shoot_point: Vector2D<i32>) -> Self {
        Shoot {
            color: Color::RGB(255, 255, 255),
            rect: Rect::new(shoot_point.x - 5, shoot_point.x, 10, 10),
            fisic_body: CollisionBody::new(shoot_point.x - 5, shoot_point.x, 10, 10),
        }
    }
}

impl BaseDrawFunction for Shoot {
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
