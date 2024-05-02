use crate::base::collisionbody::CollisionBody;
use crate::traits::draw::Draw;
use crate::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;

pub struct Retangulo {
    rect: Rect,
    color: Color,
    fisic_body: CollisionBody,
}

impl Retangulo {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Retangulo {
            rect: Rect::new(x, y, width, height),
            fisic_body: CollisionBody::new(x, y, width, height),
            color: Color::RGB(255, 255, 255),
        }
    }
}

impl Draw for Retangulo {
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
