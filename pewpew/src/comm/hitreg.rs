use crate::comm::message::{GuiToHitreg, HitregToGui, SerialToHitReg, ToHitreg};
use std::sync::mpsc::{Receiver, RecvError, SendError, Sender, TryRecvError};

pub struct HitregComm {
    hitreg_to_gui_tx: Sender<HitregToGui>,
    serial_to_hitreg_rx: Receiver<SerialToHitReg>,
    gui_to_hitreg_rx: Receiver<GuiToHitreg>,

    // there is a recv method that uses try_recv on serial_to_hitreg and gui_to_hitreg
    // if there are a lot of messages from serial, this will 'starve' messages from gui
    // thus we use this boolean to alternate between the two and make it fair
    which: bool,
}

impl HitregComm {
    pub fn new(
        hitreg_to_gui_tx: Sender<HitregToGui>,
        serial_to_hitreg_rx: Receiver<SerialToHitReg>,
        gui_to_hitreg_rx: Receiver<GuiToHitreg>,
    ) -> Self {
        Self {
            hitreg_to_gui_tx,
            serial_to_hitreg_rx,
            gui_to_hitreg_rx,
            which: false,
        }
    }

    pub fn send(&self, message: HitregToGui) -> Result<(), SendError<HitregToGui>> {
        self.hitreg_to_gui_tx.send(message)
    }

    pub fn recv_from_serial(&self) -> Result<SerialToHitReg, RecvError> {
        self.serial_to_hitreg_rx.recv()
    }

    pub fn try_recv_from_serial(&self) -> Result<SerialToHitReg, TryRecvError> {
        self.serial_to_hitreg_rx.try_recv()
    }

    pub fn recv_from_gui(&self) -> Result<GuiToHitreg, RecvError> {
        self.gui_to_hitreg_rx.recv()
    }

    pub fn try_recv_from_gui(&self) -> Result<GuiToHitreg, TryRecvError> {
        self.gui_to_hitreg_rx.try_recv()
    }

    pub fn recv(&mut self) -> Result<ToHitreg, RecvError> {
        // TODO there is an edge case here that is not handled correctly (yet) where both receivers are disconnected
        // i cant be bothered right now

        self.which = !self.which;
        if self.which {
            loop {
                if let Ok(message) = self.serial_to_hitreg_rx.try_recv() {
                    return Ok(ToHitreg::FromSerial(message));
                } else if let Ok(message) = self.gui_to_hitreg_rx.try_recv() {
                    return Ok(ToHitreg::FromGui(message));
                }
            }
        } else {
            loop {
                if let Ok(message) = self.gui_to_hitreg_rx.try_recv() {
                    return Ok(ToHitreg::FromGui(message));
                } else if let Ok(message) = self.serial_to_hitreg_rx.try_recv() {
                    return Ok(ToHitreg::FromSerial(message));
                }
            }
        }
    }

    pub fn try_recv(&mut self) -> Result<ToHitreg, TryRecvError> {
        // TODO there is an edge case here that is not handled correctly (yet) where both receivers are disconnected
        // i cant be bothered right now

        self.which = !self.which;

        #[allow(clippy::collapsible_else_if)]
        if self.which {
            if let Ok(message) = self.serial_to_hitreg_rx.try_recv() {
                Ok(ToHitreg::FromSerial(message))
            } else if let Ok(message) = self.gui_to_hitreg_rx.try_recv() {
                Ok(ToHitreg::FromGui(message))
            } else {
                Err(TryRecvError::Empty)
            }
        } else {
            if let Ok(message) = self.gui_to_hitreg_rx.try_recv() {
                Ok(ToHitreg::FromGui(message))
            } else if let Ok(message) = self.serial_to_hitreg_rx.try_recv() {
                Ok(ToHitreg::FromSerial(message))
            } else {
                Err(TryRecvError::Empty)
            }
        }
    }
}
