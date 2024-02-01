use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
};
pub struct Retangulo {
    my_rect: Rect,
    cor: Color,
}

impl Retangulo {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Retangulo {
        Retangulo {
            my_rect: Rect::new(x, y, width, height),
            cor: Color::RGB(255, 255, 255),
        }
    }

    pub fn _set_color(&mut self, r: u8, g: u8, b: u8) {
        self.cor = Color::RGB(r, g, b)
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        // canvas.clear();
        canvas.set_draw_color(self.cor);
        let _ = canvas.draw_rect(self.my_rect);
        let _ = canvas.fill_rect(self.my_rect);
    }

    pub fn _get_center(&self) -> (i32, i32) {
        (
            (self.my_rect.x + (self.my_rect.width() / 2) as i32),
            (self.my_rect.y + (self.my_rect.width() / 2) as i32),
        )
    }

    pub fn _set_position(&mut self, x: i32, y: i32) {
        self.my_rect.x = x;
        self.my_rect.y = y;
    }

    pub fn update_position(&mut self, x: i32, y: i32) {
        self.my_rect.x += x * 10;
        self.my_rect.y += y * 10;
    }

    pub fn control(&mut self, event: Event) {
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

// trait Control {
//     fn control(&mut self, event: Event) {
//         match event {
//             Event::KeyDown {
//                 keycode: Some(Keycode::Up),
//                 ..
//             } => self.update_position(0, -1),
//             Event::KeyDown {
//                 keycode: Some(Keycode::Down),
//                 ..
//             } => self.update_position(0, 1),
//             Event::KeyDown {
//                 keycode: Some(Keycode::Left),
//                 ..
//             } => self.update_position(-1, 0),
//             Event::KeyDown {
//                 keycode: Some(Keycode::Right),
//                 ..
//             } => self.update_position(1, 0),

//             _ => {}
//         };
//     }
// }
