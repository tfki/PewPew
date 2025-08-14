use crate::cancel_token::CancelToken;
use crate::comm::message::SerialToGuiKind;
use crate::comm::GuiComm;
use log::info;
use sdl2::event::Event;
use sdl2::image::InitFlag;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::thread;
use std::time::{Duration, SystemTime};

pub fn run(comm: GuiComm, cancel_token: CancelToken) {
    let sdl_context = sdl2::init().unwrap();

    let video = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
    let refresh_rate = video.desktop_display_mode(0).unwrap().refresh_rate;
    let screen_width = video.desktop_display_mode(0).unwrap().w;
    let screen_height = video.desktop_display_mode(0).unwrap().h;

    let window = video
        .window(
            "PewPew sdl-sandbox",
            screen_width as u32,
            screen_height as u32,
        )
        .fullscreen_desktop()
        .opengl()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .present_vsync() //< this means the screen cannot
        // render faster than your display rate (usually 60Hz or 144Hz)
        // .software()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();
    let last_present = SystemTime::now();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'gui_running: loop {
        if cancel_token.was_canceled() {
            info!(target: "Gui Thread", "exiting because of cancel token");
            return;
        }
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    info!(target: "Gui Thread", "exiting because window was closed");
                    break 'gui_running;
                }
                _ => {}
            }
        }

        if SystemTime::now().duration_since(last_present).unwrap()
            > Duration::from_secs_f64(1.0 / refresh_rate as f64)
        {
            match comm.recv_from_serial() {
                Ok(message) => {
                    match message.kind {
                        SerialToGuiKind::Reload => {}
                        SerialToGuiKind::Shot => {}
                    }
                }
                Err(_) => {
                    // serial is dead
                    // just do nothing, because if serial is dead
                    // cancel token was cancelled too
                }
            }
            /*
            if let Ok(_recv) = receiver.try_recv() {
                canvas.set_draw_color(Color::WHITE);
                // do something with recv
            } else {
                canvas.set_draw_color(Color::BLACK);
            }
            canvas.clear();
            canvas.present();
            last_present = SystemTime::now();
            */
        }
        thread::sleep(Duration::from_millis(1));
    }
}
