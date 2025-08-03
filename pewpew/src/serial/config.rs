use crate::user_settings;
use serial::{BaudRate, CharSize, FlowControl, Parity, StopBits};
use std::time::Duration;

#[derive(Debug)]
pub struct SerialConfig {
    pub baudrate: BaudRate,
    pub char_size: CharSize,
    pub parity: Parity,
    pub stop_bits: StopBits,
    pub flow_control: FlowControl,
    pub timeout: Duration,
    pub port_path: &'static str,
}

impl SerialConfig {
    pub fn with_path(mut self, port_path: &'static str) -> Self {
        self.port_path = port_path;
        self
    }

    pub fn with_baudrate(mut self, baudrate: serial::BaudRate) -> Self {
        self.baudrate = baudrate;
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn with_parity(mut self, parity: Parity) -> Self {
        self.parity = parity;
        self
    }

    pub fn with_stop_bits(mut self, stop_bits: StopBits) -> Self {
        self.stop_bits = stop_bits;
        self
    }

    pub fn with_flow_control(mut self, flow_control: FlowControl) -> Self {
        self.flow_control = flow_control;
        self
    }

    pub fn with_char_size(mut self, char_size: CharSize) -> Self {
        self.char_size = char_size;
        self
    }
}

impl Default for SerialConfig {
    fn default() -> Self {
        SerialConfig {
            baudrate: BaudRate::Baud115200,
            char_size: CharSize::Bits8,
            parity: Parity::ParityNone,
            stop_bits: StopBits::Stop1,
            flow_control: FlowControl::FlowNone,
            // i wanted to use something like infinity for the timeout, but u64::MAX causes crashes
            timeout: Duration::from_secs(100000),
            port_path: user_settings::SERIAL_PORT
                .expect("go to user_settings.rs and set SERIAL_PORT"),
        }
    }
}
