extern crate sdl2;

mod class;
use class::retangulo::Retangulo;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::video::{Window, WindowBuildError};
use sdl2::{EventPump, Sdl, VideoSubsystem};
use std::time::Duration;

pub fn main() {
    let window_width: u32 = 800;
    let window_height: u32 = 600;
    let sdl_context: Sdl = sdl2::init().unwrap();
    let video_subsystem: VideoSubsystem = sdl_context.video().unwrap();
    let window: Window =
        create_window("Minha Game", window_width, window_height, &video_subsystem).unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    // -------------------------------------------------------------------------------------------------
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut my_rect_1: Retangulo = Retangulo::new(50, 50, 50, 50);
    let mut my_rect_2: Retangulo = Retangulo::new(250, 250, 50, 50);
    my_rect_1._set_color(255, 255, 255);
    my_rect_2._set_color(255, 255, 0);

    'running: loop {
        canvas.set_draw_color(Color::RGB(3, 3, 0));
        canvas.clear();

        my_rect_1.render(&mut canvas);

        my_rect_2.render(&mut canvas);

        if !event_listener(&mut event_pump, &mut my_rect_1) {
            break 'running;
        };
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn event_listener(event_pump: &mut EventPump, my_rect: &mut Retangulo) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return false,
            _ => {
                my_rect.control(event);
            }
        }
    }
    return true;
}

fn create_window(
    title: &str,
    width: u32,
    height: u32,
    video_subsystem: &VideoSubsystem,
) -> Result<Window, WindowBuildError> {
    video_subsystem
        .window(title, width, height)
        .position_centered()
        .build()
}
