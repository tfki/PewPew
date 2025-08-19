use crate::user_settings;
use std::fmt::{Debug, Formatter};
use std::time::Duration;

#[derive(Debug)]
pub struct SerialConfig {
    pub baudrate: u32,
    pub timeout: Duration,
    pub port_path: &'static str,
}

pub enum SerialConfigError {
    SerialPortIsNotSet,
}

impl Debug for SerialConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SerialConfigError::SerialPortIsNotSet => write!(
                f,
                "SerialPortIsNotSet: go to user_settings.rs and set a default port"
            ),
        }
    }
}

impl SerialConfig {
    pub fn default_from_user_settings() -> Result<Self, SerialConfigError> {
        match user_settings::SERIAL_PORT {
            None => Err(SerialConfigError::SerialPortIsNotSet),
            Some(port) => Ok(SerialConfig {
                baudrate: 115200,
                // i wanted to use something like infinity for the timeout, but u64::MAX causes crashes
                timeout: Duration::from_secs(100000),
                port_path: port,
            }),
        }
    }
}
