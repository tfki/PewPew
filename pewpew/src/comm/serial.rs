use crate::comm::message::{FromSerial, SerialToGui, SerialToHitReg};
use std::sync::mpsc::SendError;
use std::sync::mpsc::Sender;

pub struct SerialComm {
    serial_to_hitreg_tx: Sender<SerialToHitReg>,
    serial_to_gui_tx: Sender<SerialToGui>,
}

impl SerialComm {
    pub fn new(
        serial_to_hitreg_tx: Sender<SerialToHitReg>,
        serial_to_gui_tx: Sender<SerialToGui>,
    ) -> Self {
        Self {
            serial_to_hitreg_tx,
            serial_to_gui_tx,
        }
    }

    pub fn send_to_gui(&self, message: SerialToGui) -> Result<(), SendError<SerialToGui>> {
        self.serial_to_gui_tx.send(message)
    }

    pub fn send_to_hitreg(&self, message: SerialToHitReg) -> Result<(), SendError<SerialToHitReg>> {
        self.serial_to_hitreg_tx.send(message)
    }

    pub fn send(&self, message: FromSerial) -> Result<(), SendError<FromSerial>> {
        match message {
            FromSerial::ToGui(message) => self
                .send_to_gui(message)
                .map_err(|e| SendError(FromSerial::ToGui(e.0))),
            FromSerial::ToHitReg(message) => self
                .send_to_hitreg(message)
                .map_err(|e| SendError(FromSerial::ToHitReg(e.0))),
        }
    }
}
