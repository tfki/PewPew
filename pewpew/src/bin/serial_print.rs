// this is not only a serial reader
// but also an example for error handling
// reading from the serial port can cause two different kinds of errors
// one error comes from opening the serial port, it is of type serial::Error
// another error can come from reading the port, this is a ReadPacketError
// the last error can come from creating a default serial port config, when the serial port path is not set

// by creating an enum that can contain all of them, and that can convert all into an instance of itself
// we can use the question mark operator in the main function to convert both errors
// into a SerialPrinterError

use pewpew::serial::config::{SerialConfig, SerialConfigError};
use pewpew::serial::reader::{SerialReader, SerialReaderReadError};

#[derive(Debug)]
pub enum SerialPrintError {
    PortOpenFailed(serial::Error),
    ReadPacketFailed(SerialReaderReadError),
    CreateConfigFailed(SerialConfigError),
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

impl From<SerialConfigError> for SerialPrintError {
    fn from(value: SerialConfigError) -> Self {
        SerialPrintError::CreateConfigFailed(value)
    }
}

pub fn main() -> Result<(), SerialPrintError> {
    let reader = SerialReader::new(SerialConfig::default_from_user_settings()?)?;

    for packet in reader {
        println!("{:?}", packet?);
    }

    Ok(())
}
