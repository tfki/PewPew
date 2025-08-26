use crate::comm::gui::GuiComm;
use crate::comm::hitreg::HitregComm;
use crate::comm::message::{GuiToHitreg, HitregToGui, SerialToGui, SerialToHitReg};
use crate::comm::serial::SerialComm;
use std::sync::mpsc::channel;

pub mod gui;
pub mod hitreg;
pub mod message;
pub mod serial;

pub fn new() -> (SerialComm, HitregComm, GuiComm) {
    let (serial_to_hitreg_tx, serial_to_hitreg_rx) = channel::<SerialToHitReg>();
    let (gui_to_hitreg_tx, gui_to_hitreg_rx) = channel::<GuiToHitreg>();
    let (hitreg_to_gui_tx, hitreg_to_gui_rx) = channel::<HitregToGui>();
    let (serial_to_gui_tx, serial_to_gui_rx) = channel::<SerialToGui>();

    (
        SerialComm::new(serial_to_hitreg_tx, serial_to_gui_tx),
        HitregComm::new(hitreg_to_gui_tx, serial_to_hitreg_rx, gui_to_hitreg_rx),
        GuiComm::new(serial_to_gui_rx, gui_to_hitreg_tx, hitreg_to_gui_rx),
    )
}
