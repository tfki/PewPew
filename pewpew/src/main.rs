use std::sync::mpsc;
use std::thread;
use pewpew::serial::packet::Packet;

fn main() {
    let (sender, receiver) = mpsc::channel::<Packet>();

    thread::spawn(move || {
        loop {
            // todo run serial reader here and send packets to gui via sender
        }
    });

    // todo run gui here
}
