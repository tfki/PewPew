use pewpew::common::cancel_token::CancelToken;
use std::thread;
use pewpew::{comm, gui, hitreg};

fn main() {
    // set the environment variable "RUST_LOG" to "error", "warn", "info", "debug", "trace" or "off"
    // to set the log level. it seems to be "error" by default
    // env_logger prints timestamp in UTC by default, if it bothers you, set a custom format
    // with a local time
    env_logger::init();

    let (serial_comm, hitreg_comm, gui_comm) = comm::new();
    let cancelled = CancelToken::default();

    //thread::spawn(pewpew::serial::run(serial_comm, cancelled.clone()));
    thread::spawn(hitreg::run(hitreg_comm, cancelled.clone()));

    // run gui on main thread
    // because macOS is shit, it allows gui operations to run only on the main thread
    // and because sdl2 wants to be cross-platform, it also only allows gui operations
    // on the main thread
    gui::run(gui_comm, cancelled);
}
