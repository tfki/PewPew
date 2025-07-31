use std::io::{BufRead, BufReader};
use serial::{SerialPort, SystemPort};
use crate::serial::packet;
use crate::serial::packet::{MessageParseError, Packet};
use crate::serial::config::SerialConfig;

pub struct SerialReader {
    reader: BufReader<SystemPort>,
    buffer: Vec<u8>,
    discard_broken_packets: bool,
}

impl SerialReader {
    pub fn new(config: SerialConfig) -> Result<SerialReader, serial::Error> {
        let settings: serial::PortSettings = serial::PortSettings {
            baud_rate: config.baudrate,
            char_size: config.char_size,
            parity: config.parity,
            stop_bits: config.stop_bits,
            flow_control: config.flow_control,
        };

        let mut port = serial::open(config.port_path)?;

        port.configure(&settings)?;
        port.set_timeout(config.timeout)?;

        let reader = BufReader::new(port);

        Ok(SerialReader {
            reader,
            buffer: Vec::new(),
            discard_broken_packets: true,
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
                Err(_) if self.discard_broken_packets => self.next().unwrap(),
                Err(e) => Err(SerialReaderReadError::MessageParseError(e)),
            }
        })
    }
}
