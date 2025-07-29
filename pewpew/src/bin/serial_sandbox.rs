use serial::SerialPort;
use std::io::{BufRead, BufReader};
use std::time::{Duration, SystemTime};

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

    let mut last_clock = 0;
    let mut last_clock_time = SystemTime::now();
    let mut reader = BufReader::new(port);

    println!();
    loop {
        let mut data = Vec::new();
        reader.read_until(255_u8, &mut data).unwrap();

        if data.len() < PAYLOAD_LENGTH {
            println!("Skipping incomplete package..");
            continue;
        }

        let clock_start = data.len() - 7;
        let clock_end = data.len() - 3;
        let brightness_start = data.len() - 3;
        let brightness_end = data.len() - 1;

        let clock = u32::from_le_bytes(data[clock_start..clock_end].try_into().unwrap());
        let brightness =
            u16::from_le_bytes(data[brightness_start..brightness_end].try_into().unwrap());

        print!("\rclock: {clock:8.} - brightness: {brightness:5.}");

        let time = SystemTime::now();
        let diff = time.duration_since(last_clock_time);

        if clock == last_clock {
            panic!();
        }

        last_clock = clock;
        last_clock_time = time;
    }
}
