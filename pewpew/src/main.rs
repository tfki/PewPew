use log::{error, info, warn};
use pewpew::cancel_token::CancelToken;
use pewpew::serial::config::SerialConfig;
use pewpew::serial::packet::Packet;
use pewpew::serial::reader::SerialReader;
use std::alloc::System;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::thread;
use std::time::{Duration, SystemTime};

fn run_serial(sender: Sender<Packet>, cancel_token: CancelToken) -> impl FnOnce() {
    // this function does not run the code below
    // instead it returns a closure (or a lambda) that someone else can run

    // move means that the closure takes ownership of all variables from the outside
    // that are used within the closure (sender, cancel_token)
    // || means that it has no parameters
    // and everything after that is the content of the closure
    // we do not need a "{ ... }" for the closure contents because the closure contains only one
    // statement, which is "loop"
    move || loop {
        let reader = match SerialConfig::default_from_user_settings() {
            Ok(cfg) => SerialReader::new(cfg),
            Err(e) => {
                error!(target: "Serial Thread", "could not create Serial Config: {e:?}, exiting");
                return;
            }
        };

        if let Err(e) = reader {
            error!(target: "Serial Thread", "could not open serial port: {e:?}, exiting");
            return;
        }

        for packet in reader.unwrap() {
            if cancel_token.was_canceled() {
                info!(target: "Serial Tread", "exiting because of cancel token");
                return;
            }

            match packet {
                Ok(packet) => {
                    if sender.send(packet).is_err() {
                        // send only ever fails if the receiver does not exist anymore
                        // so there is no point in continuing
                        error!(target: "Serial Thread", "failed to send packet to gui thread, exiting");
                        return;
                    }
                }
                Err(e) => {
                    warn!(target: "Serial Thread", "serial reader produced an error: {e:?}")
                }
            }
        }
    }
}

// TODO replace Packet in Receiver<Packet> with actual message type of Reload- and Shotmessages.
fn run_gui(receiver: Receiver<Packet>, cancel_token: CancelToken) {
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
    let mut last_present = SystemTime::now();
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
                } => break 'gui_running,
                _ => {}
            }
        }

        if SystemTime::now().duration_since(last_present).unwrap()
            > Duration::from_secs_f64(1.0 / refresh_rate as f64)
        {
            if let Ok(_recv) = receiver.try_recv() {
                canvas.set_draw_color(Color::WHITE);
                // do something with recv
            } else {
                canvas.set_draw_color(Color::BLACK);
            }
            canvas.clear();
            canvas.present();
            last_present = SystemTime::now();
        }
        thread::sleep(Duration::from_millis(1));
    }
}

fn main() {
    // set the environment variable "RUST_LOG" to "error", "warn", "info", "debug", "trace" or "off"
    // to set the log level. it seems to be "error" by default
    // env_logger prints timestamp in UTC by default, if it bothers you, set a custom format
    // with a local time
    env_logger::init();

    let (sender, receiver) = channel::<Packet>();
    let cancelled = CancelToken::default();

    thread::spawn(run_serial(sender, cancelled.clone()));

    // run gui on main thread
    // because macOS is shit, it allows gui operations to run only on the main thread
    // and because sdl2 wants to be cross-platform, it also only allows gui operations
    // on the main thread
    run_gui(receiver, cancelled);
}
