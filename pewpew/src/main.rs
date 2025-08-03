use log::{error, info, warn};
use pewpew::cancel_token::CancelToken;
use pewpew::serial::config::SerialConfig;
use pewpew::serial::packet::Packet;
use pewpew::serial::reader::SerialReader;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc};
use std::thread;

fn run_serial(sender: Sender<Packet>, cancel_token: Arc<CancelToken>) -> impl FnOnce() {
    // this function does not run the code below
    // instead it returns a closure (or a lambda) that someone else can run

    // move means that the closure takes ownership of all variables from the outside
    // that are used within the closure (sender, cancel_token)
    // || means that it has no parameters
    // and everything after that is the content of the closure
    // we do not need a "{ ... }" for the closure contents because the closure contains only one
    // statement, which is "loop"
    move || loop {
        let reader = match SerialConfig::default() {
            Ok(cfg) => SerialReader::new(cfg),
            Err(e) => {
                error!(target: "Serial Thread", "could not create Serial Config: {:?}, exiting", e);
                cancel_token.cancel();
                return;
            }
        };

        if let Err(e) = reader {
            error!(target: "Serial Thread", "could not open serial port: {:?}, exiting", e);
            cancel_token.cancel();
            return;
        }

        for packet in reader.unwrap() {
            if cancel_token.was_canceled() {
                info!(target: "Serial Tread", "exiting because of cancel token");
                cancel_token.cancel();
                return;
            }

            match packet {
                Ok(packet) => {
                    if sender.send(packet).is_err() {
                        // send only ever fails if the receiver does not exist anymore
                        // so there is no point in continuing
                        error!(target: "Serial Thread", "failed to send packet to gui thread, exiting");
                        cancel_token.cancel();
                        return;
                    }
                }
                Err(e) => {
                    warn!(target: "Serial Thread", "serial reader produced an error: {:?}", e)
                }
            }
        }
    }
}

fn run_gui(_receiver: Receiver<Packet>, cancel_token: Arc<CancelToken>) {
    // TODO
    loop {
        if cancel_token.was_canceled() {
            info!(target: "Gui Thread", "exiting because of cancel token");
            return;
        }
    }
}

fn main() {
    // set the environment variable "RUST_LOG" to "error", "warn", "info", "debug", "trace" or "off"
    // to set the log level. it seems to be "error" by default
    // env_logger prints timestamp in UTC by default, if it bothers you, set a custom format
    // with a local time
    env_logger::init();

    let (sender, receiver) = mpsc::channel::<Packet>();
    let cancelled = CancelToken::new();

    thread::spawn(run_serial(sender, cancelled.clone()));

    // run gui on main thread
    // because macOS is shit, it allows gui operations to run only on the main thread
    // and because sdl2 wants to be cross-platform, it also only allows gui operations
    // on the main thread
    run_gui(receiver, cancelled);
}
