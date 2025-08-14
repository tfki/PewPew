use crate::comm::message::{GuiToHitReg, HitregToGui, SerialToGui, ToGui};
use std::sync::mpsc::{Receiver, RecvError, SendError, Sender, TryRecvError};
use std::time::Duration;

pub struct GuiComm {
    serial_to_gui_rx: Receiver<SerialToGui>,
    gui_to_hitreg_tx: Sender<GuiToHitReg>,
    hitreg_to_gui_rx: Receiver<HitregToGui>,

    // there is a recv method that uses try_recv on serial_to_hitreg and gui_to_hitreg
    // if there are a lot of messages from serial, this will 'starve' messages from gui
    // thus we use this boolean to alternate between the two and make it fair
    which: bool,
}

impl GuiComm {
    pub fn new(
        serial_to_gui_rx: Receiver<SerialToGui>,
        gui_to_hitreg_tx: Sender<GuiToHitReg>,
        hitreg_to_gui_rx: Receiver<HitregToGui>,
    ) -> Self {
        Self {
            serial_to_gui_rx,
            gui_to_hitreg_tx,
            hitreg_to_gui_rx,
            which: false,
        }
    }

    pub fn send(&self, message: GuiToHitReg) -> Result<(), SendError<GuiToHitReg>> {
        self.gui_to_hitreg_tx.send(message)
    }

    pub fn recv_from_serial(&self) -> Result<SerialToGui, RecvError> {
        self.serial_to_gui_rx.recv()
    }

    pub fn try_recv_from_serial(&self) -> Result<SerialToGui, TryRecvError> {
        self.serial_to_gui_rx.try_recv()
    }

    pub fn recv_from_hitreg(&self) -> Result<HitregToGui, RecvError> {
        self.hitreg_to_gui_rx.recv()
    }

    pub fn try_recv_from_hitreg(&self) -> Result<HitregToGui, TryRecvError> {
        self.hitreg_to_gui_rx.try_recv()
    }

    pub fn recv(&mut self) -> Result<ToGui, RecvError> {
        // TODO there is an edge case here that is not handled correctly (yet) where both receivers are disconnected
        // i cant be bothered right now

        self.which = !self.which;
        if self.which {
            loop {
                if let Ok(message) = self.hitreg_to_gui_rx.recv_timeout(Duration::from_millis(1)) {
                    return Ok(ToGui::FromHitreg(message));
                } else if let Ok(message) =
                    self.serial_to_gui_rx.recv_timeout(Duration::from_millis(1))
                {
                    return Ok(ToGui::FromSerial(message));
                }
            }
        } else {
            loop {
                if let Ok(message) = self.serial_to_gui_rx.recv_timeout(Duration::from_millis(1)) {
                    return Ok(ToGui::FromSerial(message));
                } else if let Ok(message) =
                    self.hitreg_to_gui_rx.recv_timeout(Duration::from_millis(1))
                {
                    return Ok(ToGui::FromHitreg(message));
                }
            }
        }
    }

    pub fn try_recv(&mut self) -> Result<ToGui, TryRecvError> {
        // TODO there is an edge case here that is not handled correctly (yet) where both receivers are disconnected
        // i cant be bothered right now

        self.which = !self.which;

        #[allow(clippy::collapsible_else_if)]
        if self.which {
            if let Ok(message) = self.hitreg_to_gui_rx.try_recv() {
                Ok(ToGui::FromHitreg(message))
            } else if let Ok(message) = self.serial_to_gui_rx.try_recv() {
                Ok(ToGui::FromSerial(message))
            } else {
                Err(TryRecvError::Empty)
            }
        } else {
            if let Ok(message) = self.serial_to_gui_rx.try_recv() {
                Ok(ToGui::FromSerial(message))
            } else if let Ok(message) = self.hitreg_to_gui_rx.try_recv() {
                Ok(ToGui::FromHitreg(message))
            } else {
                Err(TryRecvError::Empty)
            }
        }
    }
}
