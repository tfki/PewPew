
// this is not only a serial reader
// but also an example for error handling
// reading from the serial port can cause two different kinds of errors
// one error comes from opening the serial port, it is of type serial::Error
// the other error can come from reading the port, this is a ReadPacketError

// by creating an enum that can contain both, and that can convert both into an instance of itself
// we can use the question mark operator in the main function to convert both errors
// into a SerialPrinterError

use pewpew::serial::config::SerialConfig;
use pewpew::serial::reader::{SerialReader, SerialReaderReadError};

#[derive(Debug)]
pub enum SerialPrintError {
    PortOpenFailed(serial::Error),
    ReadPacketFailed(SerialReaderReadError),
}

impl From<serial::Error> for SerialPrintError {
    fn from(value: serial::Error) -> Self {
        SerialPrintError::PortOpenFailed(value)
    }
}

impl From<SerialReaderReadError> for SerialPrintError {
    fn from(value: SerialReaderReadError) -> Self {
        SerialPrintError::ReadPacketFailed(value)
    }
}

pub fn main() -> Result<(), SerialPrintError> {
    let reader = SerialReader::new(SerialConfig::default())?;

    for packet in reader {
        println!("{:?}", packet?);
    }

    Ok(())
}
