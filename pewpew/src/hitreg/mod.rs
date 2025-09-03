use crate::comm::hitreg::HitregComm;
use crate::comm::message::ToHitreg;
use crate::comm::message::{GuiToHitreg, HitregToGui};
use crate::common::cancel_token::CancelToken;
use log::{debug, error, info};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
enum State {
    Idle,
    WaitingForFlashFrameEnd(u32),
}

pub struct BrightnessBuffer {
    pub val: u16,
    pub sensortag_id: u16,
    pub time: u32,
    pub is_white: bool,
}

const BRIGHTNESS_GRADIENT_THRESHOLD: u16 = 100;

pub fn run(mut comm: HitregComm, cancel_token: CancelToken) -> impl FnOnce() {
    move || {
        let mut state = State::Idle;
        let mut chicken_data = Vec::new();
        let mut gui_sequence: Vec<(u16, u32, bool)> = Vec::new();
        let mut gui_timestamps: Vec<SystemTime> = Vec::new();
        // val, tag_id, timestamp, white/black
        let mut serial_brightness_buffer = BrightnessBuffer {
            val: 0,
            sensortag_id: 0,
            time: 0,
            is_white: false,
        };

        fn store_brightness_in_buffer(
            buf: &mut BrightnessBuffer,
            sensortag_id: u16,
            time: u32,
            val: u16,
        ) {
            debug!(target: "Hitreg Thread", "received brightness {val} at t={}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis());

            let new_is_white: bool;
            if buf.is_white {
                if buf.val > val && buf.val - val > BRIGHTNESS_GRADIENT_THRESHOLD {
                    // gradient indicates it switched to LOW
                    new_is_white = false;
                } else {
                    new_is_white = true;
                }
            } else if val > buf.val && val - buf.val > BRIGHTNESS_GRADIENT_THRESHOLD {
                // gradient indicates it switched to HIGH
                new_is_white = true;
            } else {
                new_is_white = false;
            }
            *buf = BrightnessBuffer {
                val,
                sensortag_id,
                time,
                is_white: new_is_white,
            };
        }

        loop {
            if cancel_token.was_canceled() {
                info!(target: "Hitreg Thread", "exiting because of cancel token");
                return;
            }

            match state {
                State::Idle => match comm.recv().unwrap() {
                    ToHitreg::FromGui(GuiToHitreg::FlashingSequenceStart {
                        num_frames,
                        sequences,
                    }) => {
                        chicken_data = sequences;
                        debug!(target: "Hitreg Thread", "{chicken_data:?}");

                        state = State::WaitingForFlashFrameEnd(num_frames);
                        debug!(target: "Hitreg Thread", "changing state to {state:?} at t={}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis());
                    }
                    ToHitreg::FromSerial(serial_to_hit_reg) => {
                        store_brightness_in_buffer(
                            &mut serial_brightness_buffer,
                            serial_to_hit_reg.sensortag_id,
                            serial_to_hit_reg.timestamp,
                            serial_to_hit_reg.value_raw,
                        );
                    }
                    x => {
                        error!(target: "Hitreg Thread", "hitreg received unexpected message in state {state:?}, exiting: {x:?}");
                        return;
                    }
                },
                State::WaitingForFlashFrameEnd(0) => {
                    if let Ok(serial_to_hit_reg) = comm.try_recv_from_serial() {
                        store_brightness_in_buffer(
                            &mut serial_brightness_buffer,
                            serial_to_hit_reg.sensortag_id,
                            serial_to_hit_reg.timestamp,
                            serial_to_hit_reg.value_raw,
                        );
                    }

                    // all frames of the flashing sequence have arrived
                    // tell the gui the results
                    // TODO work with timings in gui_timestamps? test delay on gui_sequence
                    let desired_length = chicken_data.first().unwrap().1.len();
                    if gui_timestamps.len() != desired_length
                        || gui_sequence.len() != desired_length
                    {
                        error!(target: "Hitreg Thread", "amount of frame-timestamps from gui does not match length of flashing sequences");
                    }
                    let gui_seq = gui_sequence.iter().map(|(_, _, x)| *x).collect::<Vec<_>>();
                    debug!(target: "Hitreg Thread", "{gui_seq:?}");
                    let hit = chicken_data
                        .iter()
                        .find_map(|(entity, sequence)| (sequence == &gui_seq).then_some(*entity));
                    comm.send(HitregToGui::Result(hit)).unwrap();
                    gui_timestamps.clear();
                    gui_sequence.clear();
                    state = State::Idle;
                    debug!(target: "Hitreg Thread", "changing state to {state:?} at t={}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis());
                }
                State::WaitingForFlashFrameEnd(num_frames_to_go) => {
                    match comm.recv().unwrap() {
                        ToHitreg::FromGui(GuiToHitreg::FlashBlackFrameEnd(_time)) => {
                            // set current Color to Black
                            serial_brightness_buffer.is_white = false;
                        }
                        ToHitreg::FromGui(GuiToHitreg::FlashFrameEnd(time)) => {
                            gui_timestamps.push(time);
                            if serial_brightness_buffer.time == 0 {
                                error!(target: "Hitreg Thread", "no brightness measurements available (or timestamp is 0)");
                            }
                            // read latest serial_brightness_buffer value into gui_sequence
                            gui_sequence.push((
                                serial_brightness_buffer.sensortag_id,
                                serial_brightness_buffer.time,
                                serial_brightness_buffer.is_white,
                            ));
                            state = State::WaitingForFlashFrameEnd(num_frames_to_go - 1);
                            debug!(target: "Hitreg Thread", "changing state to {state:?} at t={}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis());
                        }
                        ToHitreg::FromSerial(serial_to_hit_reg) => {
                            store_brightness_in_buffer(
                                &mut serial_brightness_buffer,
                                serial_to_hit_reg.sensortag_id,
                                serial_to_hit_reg.timestamp,
                                serial_to_hit_reg.value_raw,
                            );
                        }
                        x => {
                            error!(target: "Hitreg Thread", "hitreg received unexpected message in state {state:?}, exiting: {x:?}");
                            return;
                        }
                    }
                }
            }
        }
    }
}
