use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;
use std::thread;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 1920, 1080)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.fill_rect(Rect::new(0, 0, 2560, 1440))?;
        canvas.present();

        thread::sleep(Duration::from_secs(1));

        canvas.set_draw_color(Color::WHITE);
        canvas.fill_rect(Rect::new(0, 0, 2560, 1440))?;
        canvas.present();

        thread::sleep(Duration::from_secs(1));

        canvas.set_draw_color(Color::BLACK);
        canvas.fill_rect(Rect::new(0, 0, 2560, 1440))?;
        canvas.present();

        thread::sleep(Duration::from_secs(1));

        return Ok(());
    }

    Ok(())
}
