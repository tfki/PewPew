use std::io::{BufRead, BufReader};
use std::time::Duration;
use serialport::SerialPort;
use crate::serial::packet;
use crate::serial::packet::{MessageParseError, Packet};
use crate::serial::config::SerialConfig;

pub struct SerialReader {
    reader: BufReader<Box<dyn SerialPort>>,
    buffer: Vec<u8>,
}

impl SerialReader {
    pub fn new(config: SerialConfig) -> Result<SerialReader, serialport::Error> {
        let port = serialport::new(config.port_path, config.baudrate)
            .timeout(Duration::from_millis(10000))
            .open();

        let reader = BufReader::new(port?);

        Ok(SerialReader {
            reader,
            buffer: Vec::new(),
        })
    }
}

#[derive(Debug)]
pub enum SerialReaderReadError {
    MessageParseError(MessageParseError),
    IoError(std::io::Error),
}

impl Iterator for SerialReader {
    type Item = Result<Packet, SerialReaderReadError>;

    fn next(&mut self) -> Option<Self::Item> {
        // never returns none
        // if the serial port is broken (like when the usb cable is pulled)
        // the iterator will keep returning io errors

        self.buffer.clear();

        Some(if let Err(e) = self.reader.read_until(packet::DELIMITER, &mut self.buffer) {
            Err(SerialReaderReadError::IoError(e))
        } else {
            match Packet::try_from(self.buffer.as_slice()) {
                Ok(packet) => Ok(packet),
                Err(e) => Err(SerialReaderReadError::MessageParseError(e)),
            }
        })
    }
}
