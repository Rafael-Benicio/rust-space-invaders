use crate::traits::collision::BoxColision;
use crate::traits::draw_base::BaseDrawFunction;
use crate::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;

pub struct Retangulo {
    rect: Rect,
    color: Color,
}

impl BaseDrawFunction for Retangulo {
    fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Retangulo {
            rect: Rect::new(x, y, width, height),
            color: Color::RGB(255, 255, 255),
        }
    }

    fn _set_color(&mut self, r: u8, g: u8, b: u8) {
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
    }

    fn update_position(&mut self, x: i32, y: i32) {
        self.set_position(self.rect.x + x * 10, self.rect.y + y * 10);
    }
}

impl BoxColision for Retangulo {
    fn aabb_collision(&self, rect: (i32, i32, i32, i32)) -> bool {
        if (rect.0 + rect.2) > self.rect.x
            && (self.rect.x + (self.rect.width() as i32)) > rect.0
            && (rect.1 + rect.3) > self.rect.y
            && (self.rect.y + (self.rect.height() as i32)) > rect.1
        {
            return true;
        }
        false
    }
    fn collision_box(&self) -> (i32, i32, i32, i32) {
        (
            self.rect.x,
            self.rect.y,
            self.rect.width() as i32,
            self.rect.height() as i32,
        )
    }
}
