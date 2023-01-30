use std::error::Error;
use std::thread;
use std::time::Instant;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

mod hub75;
use hub75::{Hub75PinNums, Hub75Panel};

fn main() -> Result<(), Box<dyn Error>> {
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

//    let mut pin = Gpio::new()?.get(pins.lines[0])?.into_output();

    let mut panel = Hub75Panel::new(64, 32, pins);

    let start = Instant::now();

    while start.elapsed().as_secs() < 5 {
        panel.test();
    }
    
    panel.blank();

    Ok(())
}
