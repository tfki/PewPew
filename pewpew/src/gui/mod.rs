mod engine;
mod scenes;

use std::{thread, time::Duration};

use crate::common::cancel_token::CancelToken;
use crate::comm::gui::GuiComm;
use crate::gui::engine::gui_context;
use crate::gui::engine::gui_context::GuiContext;
use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS, Chunk}; // sudo apt install libsdl2-mixer-dev

pub fn run(comm: GuiComm, cancel_token: CancelToken) {
    let mut gui_context = GuiContext::new(gui_context::Settings::default(), cancel_token, comm);

    let sdl = sdl2::init().unwrap();
    let _audio = sdl.audio().unwrap();
    // Initialize mixer with MP3 support
    sdl2::mixer::init(InitFlag::MP3).unwrap();
    sdl2::mixer::open_audio(44_100, AUDIO_S16LSB, DEFAULT_CHANNELS, 1024).unwrap();

    // Load and play mp3
    let chunk = Chunk::from_file("res/glock19-18535.mp3").unwrap();
    sdl2::mixer::Channel::all().play(&chunk, 0).unwrap();
    thread::sleep(Duration::from_secs(2));
    chunk = Chunk::from_file("res/gun-shot-359196.mp3").unwrap();
    sdl2::mixer::Channel::all().play(&chunk, 0).unwrap();
    thread::sleep(Duration::from_secs(2));
    chunk = Chunk::from_file("res/ak47_boltpull.mp3").unwrap();
    sdl2::mixer::Channel::all().play(&chunk, 0).unwrap();
    thread::sleep(Duration::from_secs(2));
    chunk = Chunk::from_file("res/glock-reload.mp3").unwrap();
    sdl2::mixer::Channel::all().play(&chunk, 0).unwrap();
    thread::sleep(Duration::from_secs(2));
    chunk = Chunk::from_file("res/ahhhh.mp3").unwrap();
    sdl2::mixer::Channel::all().play(&chunk, 0).unwrap();
    thread::sleep(Duration::from_secs(2));
    chunk = Chunk::from_file("res/wilhelm_scream.mp3").unwrap();
    sdl2::mixer::Channel::all().play(&chunk, 0).unwrap();
    thread::sleep(Duration::from_secs(2));



    scenes::intro::run(&mut gui_context);
    loop {
        let player_datas = scenes::pregame::run(&mut gui_context);
        let player_datas = scenes::game::run(&mut gui_context, player_datas);
        scenes::scoreboard::run(&mut gui_context, player_datas);
    }
    //scenes::sandbox::run(&mut gui_context);
}
