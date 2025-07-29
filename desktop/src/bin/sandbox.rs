use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use serial::SerialPort;
use std::io::{BufRead, BufReader, Read};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn main() -> Result<(), String> {
    // thread::spawn(|| {
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

            if data.len() < 7 {
                println!("Skipping incomplete package..");
                continue;
            }

            let clock_start = data.len() - 7;
            let clock_end = data.len() - 3;
            let brightness_start = data.len() - 3;
            let brightness_end = data.len() - 1;

            let clock = u32::from_le_bytes(data[clock_start..clock_end].try_into().unwrap());
            let brightness = u16::from_le_bytes(data[brightness_start..brightness_end].try_into().unwrap());

            let time = SystemTime::now();
            let diff = time.duration_since(last_clock_time);

            print!("\rbrightness {brightness:5.}");

            if clock == last_clock { panic!(); }

            last_clock = clock;
            last_clock_time = time;

            //println!("{} {clock} {brightness}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis());
        }
    // });

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 2560, 1440)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    let mut color = Color::WHITE;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.fill_rect(Rect::new(0, 0, 2560, 1440))?;
        canvas.present();

        thread::sleep(Duration::from_secs(1));

        canvas.set_draw_color(Color::WHITE);
        canvas.fill_rect(Rect::new(0, 0, 2560, 1440))?;
        canvas.present();

        thread::sleep(Duration::from_secs(1));

        canvas.set_draw_color(Color::BLACK);
        canvas.fill_rect(Rect::new(0, 0, 2560, 1440))?;
        canvas.present();

        thread::sleep(Duration::from_secs(1));

        return Ok(());
    }

    Ok(())
}
