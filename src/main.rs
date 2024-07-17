extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{self, FRect, Rect};
use std::time::Duration;


mod SDL_Graph;
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    

    let mut window = video_subsystem.window("SDL_Graph_rs", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    
    window.set_resizable(true);
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
     

     // main loop
    'running: loop {
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 10, 10));
        canvas.fill_rect(Rect::new(30, 30, 200, 200));
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}