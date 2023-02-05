use std::error::Error;
use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;

use rppal::system::DeviceInfo;
use spin_sleep::LoopHelper;

mod hub75;
mod img_buffer;
mod font;
mod ppm;

use hub75::{Hub75PinNums, Hub75Panel};
use img_buffer::{ImgBuffer, Color};

fn main() -> Result<(), Box<dyn Error>> {
    let running = Arc::new(AtomicBool::new(true));

    ctrlc::set_handler({
        let running = running.clone();
        move || {
            println!("Shutting down.");
            running.store(false, Ordering::SeqCst);
        }
    }).expect("Error setting ctrl-c handler!");

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
    // image.set_pixel(0, 0, Color::Red);
    // image.set_pixel(10, 10, Color::Green);
    // image.set_pixel(20, 20, Color::Blue);
    // image.set_pixel(30, 30, Color::White);
    // image.set_pixel(40, 0, Color::Yellow);
    // image.set_pixel(50, 10, Color::Purple);
    // image.set_pixel(60, 20, Color::Teal);

    let font = font::Font::load("fonts/57.toml")?;
    image.draw_str("ABCDEFGHIJ", &font, 0, 0, Color::Red);
    image.draw_str("KLMNOPQRST", &font, 0, 8, Color::Green);
    image.draw_str("UVWXYZabcd", &font, 0, 16, Color::Blue);
    image.draw_str("efghijklmn", &font, 0, 24, Color::Yellow);

    let mut loop_helper = LoopHelper::builder()
        .report_interval_s(1.0)
        .build_with_target_rate(1000.0);

    let (sender, receiver) = channel();
    
    let strobe_handle = thread::spawn({
        let running = running.clone();
        move || {
            panel.blank();

            while running.load(Ordering::SeqCst) {
                loop_helper.loop_start();

                panel.strobe_row(&image);

                if let Some(rate) = loop_helper.report_rate() {
                    sender.send(rate).unwrap();
                }
            
                loop_helper.loop_sleep();
            }

            panel.blank();
        }
    });

    let info_handle = thread::spawn({
        let running = running.clone();
        move || {
            while running.load(Ordering::SeqCst) {
                let _rate = receiver.recv().unwrap();
                // println!("Rate: {}", rate);
            }
        }
    });
    
    strobe_handle.join().expect("Strobe thread panicked!");
    info_handle.join().expect("Info thread panicked!");

    Ok(())
}
