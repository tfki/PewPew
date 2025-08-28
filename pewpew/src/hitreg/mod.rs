use crate::comm::hitreg::HitregComm;
use crate::comm::message::{GuiToHitreg, HitregToGui};
use crate::comm::message::ToHitreg;
use crate::common::cancel_token::CancelToken;
use log::{debug, error, info};
use std::collections::VecDeque;
use std::time::SystemTime;

#[derive(Debug)]
enum State {
    Idle,
    WaitingForFrames(u32),
}

// TODO set to middle of black and white
const BRIGHTNESS_THRESHOLD: u16 = 10000;

pub fn run(mut comm: HitregComm, cancel_token: CancelToken) -> impl FnOnce() {
    move || {
        let mut state = State::Idle;
        let mut chicken_data = Vec::new();
        let mut gui_sequence: Vec<bool> = Vec::new();
        let mut gui_timestamps: Vec<SystemTime> = Vec::new();
        let mut serial_brightness_buffer: VecDeque<(u16, u32, bool)> = VecDeque::with_capacity(20);

        fn store_brightness_in_buffer(buf: &mut VecDeque<(u16, u32, bool)>, sensortag_id: u16, time: u32, val:u16) {
            if buf.len() == buf.capacity() {
            buf.pop_back();
            } else {
                if val >= BRIGHTNESS_THRESHOLD {
                    // WHITE
                    buf.push_front((sensortag_id, time, true));
                } else {
                    buf.push_front((sensortag_id, time, false));
                }
            }
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

                        state = State::WaitingForFrames(num_frames);
                        debug!(target: "Hitreg Thread", "changing state to {state:?}");
                    }
                    ToHitreg::FromSerial(serial_to_hit_reg) => {
                        store_brightness_in_buffer(&mut serial_brightness_buffer, serial_to_hit_reg.sensortag_id, serial_to_hit_reg.timestamp, serial_to_hit_reg.value_raw);
                    }
                    x => {
                        error!(target: "Hitreg Thread", "hitreg received unexpected message in state {state:?}, exiting: {x:?}");
                        return;
                    }
                },
                State::WaitingForFrames(0) => {
                    // all frames of the flashing sequence have arrived
                    // tell the gui the results
                    // TODO work with timings in gui_timestamps? test delay on gui_sequence
                    if gui_timestamps.len() != chicken_data.first().unwrap().1.len() {
                        error!(target: "Hitreg Thread", "amount of frame-timestamps from gui does not match length of flashing sequences");
                    }
                    let buffer_slice = &gui_sequence.
                        iter().
                        take(chicken_data[0].1.len()).
                        copied().
                        collect::<Vec<_>>();
                    let hit = chicken_data
                        .iter()
                        .find_map(
                            |(entity, sequence)|
                                {(sequence == buffer_slice).then_some(*entity)}
                        );
                    comm.send(HitregToGui::Result(hit)).unwrap(); // no hit
                    gui_timestamps.clear();
                    state = State::Idle;
                    debug!(target: "Hitreg Thread", "changing state to {state:?}");
                }
                State::WaitingForFrames(num_frames_to_go) => {
                    match comm.recv().unwrap() {
                        ToHitreg::FromGui(GuiToHitreg::Frame(time)) => {
                            gui_timestamps.push(time);
                            if serial_brightness_buffer.is_empty() {
                                error!(target: "Hitreg Thread", "no brightness measurements available");
                            }
                            // read latest serial_brightness_buffer value into gui_sequence
                            gui_sequence.push(serial_brightness_buffer.front().copied().unwrap().2);
                            state = State::WaitingForFrames(num_frames_to_go - 1);
                            debug!(target: "Hitreg Thread", "changing state to {state:?}");
                        }
                        ToHitreg::FromSerial(serial_to_hit_reg) => {
                            store_brightness_in_buffer(&mut serial_brightness_buffer, serial_to_hit_reg.sensortag_id, serial_to_hit_reg.timestamp, serial_to_hit_reg.value_raw);
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
