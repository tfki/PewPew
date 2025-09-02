pub mod config;
pub mod packet;
pub mod reader;

use crate::comm::message::{SerialToGui, SerialToGuiKind, SerialToHitReg};
use crate::comm::serial::SerialComm;
use crate::common::cancel_token::CancelToken;
use crate::serial::config::SerialConfig;
use crate::serial::packet::{MagazineStatus, PacketContent};
use crate::serial::reader::SerialReader;
use log::{error, info, warn};

pub fn run(comm: SerialComm, cancel_token: CancelToken) -> impl FnOnce() {
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
                    match packet.content {
                        PacketContent::ButtonPressed(MagazineStatus{ammo, ammo_max}) => {
                            if comm
                                .send_to_gui(SerialToGui {
                                    sensortag_id: packet.sensortag_id,
                                    timestamp: packet.timestamp,
                                    ammo: ammo,
                                    ammo_max: ammo_max,
                                    kind: SerialToGuiKind::Shot,
                                })
                                .is_err()
                            {
                                // send only ever fails if the receiver does not exist anymore
                                // so there is no point in continuing
                                error!(target: "Serial Thread", "failed to send packet to gui thread, exiting");
                                return;
                            }
                        }
                        PacketContent::Brightness(value_raw) => {
                            if comm
                                .send_to_hitreg(SerialToHitReg {
                                    sensortag_id: packet.sensortag_id,
                                    timestamp: packet.timestamp,
                                    value_raw,
                                })
                                .is_err()
                            {
                                // send only ever fails if the receiver does not exist anymore
                                // so there is no point in continuing
                                error!(target: "Serial Thread", "failed to send packet to hitreg thread, exiting");
                                return;
                            }
                        }
                        PacketContent::Reloaded(MagazineStatus{ammo, ammo_max}) => {
                            if comm
                                .send_to_gui(SerialToGui {
                                    sensortag_id: packet.sensortag_id,
                                    timestamp: packet.timestamp,
                                    ammo: ammo,
                                    ammo_max: ammo_max,
                                    kind: SerialToGuiKind::Reload,
                                })
                                .is_err()
                            {
                                // send only ever fails if the receiver does not exist anymore
                                // so there is no point in continuing
                                error!(target: "Serial Thread", "failed to send packet to gui thread, exiting");
                                return;
                            }
                        }
                    }
                }
                Err(e) => {
                    warn!(target: "Serial Thread", "serial reader produced an error: {e:?}")
                }
            }
        }
    }
}
