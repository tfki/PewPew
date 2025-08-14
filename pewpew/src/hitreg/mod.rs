use crate::cancel_token::CancelToken;
use log::info;
use crate::comm::hitreg::HitregComm;

pub fn run(
    mut comm: HitregComm,
    cancel_token: CancelToken,
) -> impl FnOnce() {
    move || {
        loop {
            if let Ok(message) = comm.try_recv() {
                info!(target: "Hitreg Thread", "received a message: {message:?}");
            }
            if cancel_token.was_canceled() {
                info!(target: "Hitreg Thread", "exiting because of cancel token");
                return;
            }
        }
    }
}
