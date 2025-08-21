use crate::comm::hitreg::HitregComm;
use crate::comm::message::{GuiToHitreg, HitregToGui};
use crate::comm::message::ToHitreg;
use crate::common::cancel_token::CancelToken;
use log::{debug, error, info};

#[derive(Debug)]
enum State {
    Idle,
    WaitingForFrames(u32),
}

pub fn run(mut comm: HitregComm, cancel_token: CancelToken) -> impl FnOnce() {
    move || {
        // i suggest implementing the hitreg with some kind of state machine

        let mut state = State::Idle;

        let mut chicken_data = Vec::new();
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
                    ToHitreg::FromSerial(_) => {
                        // store value in the brightness buffer
                        // and delete brightness values in the buffer that are older than x seconds
                    }
                    x => {
                        error!(target: "Hitreg Thread", "hitreg received unexpected message in state {state:?}, exiting: {x:?}");
                        return;
                    }
                },
                State::WaitingForFrames(0) => {
                    // all frames of the flashing sequence have arrived
                    // tell the gui the results

                    // for, just tell the gui that the first chicken was hit
                    let hit = chicken_data.first().map(|(entity, _)| *entity);
                    comm.send(HitregToGui::Result(hit)).unwrap(); // no hit

                    state = State::Idle;
                    debug!(target: "Hitreg Thread", "changing state to {state:?}");
                }
                State::WaitingForFrames(num_frames_to_go) => {
                    match comm.recv().unwrap() {
                        ToHitreg::FromGui(GuiToHitreg::Frame(_time)) => {
                            // look at the values in the brightness buffer here
                            // to determine if sensortag is seeing black or white?

                            state = State::WaitingForFrames(num_frames_to_go - 1);
                            debug!(target: "Hitreg Thread", "changing state to {state:?}");
                        }
                        ToHitreg::FromSerial(_) => {
                            // store value in the brightness buffer
                            // and delete brightness values in the buffer that are older than x seconds
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
