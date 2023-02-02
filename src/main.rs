use std::error::Error;
use std::thread;
use std::time::Instant;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

mod hub75;
use hub75::{Hub75PinNums, Hub75Panel};

mod img_buffer;
use img_buffer::{ImgBuffer, Color};

fn main() -> Result<(), Box<dyn Error>> {
    let running = Arc::new(AtomicBool::new(true));

    ctrlc::set_handler({
        let running = running.clone();
        move || {
            println!("Shutting down.");
            running.store(false, Ordering::SeqCst);
        }
    });

    println!("Device info: {}", DeviceInfo::new()?.model());

    let pins = Hub75PinNums {
        lines: [22, 26, 27, 20, 24],
        r: [5, 12],
        g: [13, 16],
        b: [6, 23],
        clk: 17,
        oe: 4,
        lat: 21,
    };

    let mut panel = Hub75Panel::new(64, 32, pins);
    let mut image = ImgBuffer::new();
    image.set_pixel(0, 0, Color::Red);
    image.set_pixel(10, 10, Color::Green);
    image.set_pixel(20, 20, Color::Blue);
    image.set_pixel(30, 30, Color::White);
    image.set_pixel(40, 0, Color::Yellow);
    image.set_pixel(50, 10, Color::Purple);
    image.set_pixel(60, 20, Color::Teal);

    let thread_handle = thread::spawn(move || {
        panel.blank();

        while running.load(Ordering::SeqCst) {
            panel.strobe_row(&image);
        }

        panel.blank();
    });
    
    thread_handle.join();

    Ok(())
}
