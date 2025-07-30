use pewpew::Message;
use serial::SerialPort;
use std::io::{BufRead, BufReader};
use std::time::Duration;

static PAYLOAD_LENGTH: usize = 7;

pub fn main() -> ! {
    const SETTINGS: serial::PortSettings = serial::PortSettings {
        baud_rate: serial::Baud115200,
        char_size: serial::Bits8,
        parity: serial::ParityNone,
        stop_bits: serial::Stop1,
        flow_control: serial::FlowNone,
    };

    let mut port = serial::open("/dev/serial/by-path/platform-vhci_hcd.0-usb-0:1:1.0").unwrap();

    port.configure(&SETTINGS).unwrap();
    port.set_timeout(Duration::from_secs(1000)).unwrap();

    let mut reader = BufReader::new(port);

    println!();
    loop {
        let mut data = Vec::new();
        reader.read_until(255_u8, &mut data).unwrap();

        if let Ok(message) = Message::try_from(data.as_slice()) {
            println!("{message:?}");
        }
    }
}
