use std::thread;
use std::time::Duration;
use rppal::gpio::{Gpio, OutputPin, Level};
use crate::img_buffer::ImgBuffer;

pub struct Hub75PinNums<const LC: usize> {
    pub lines: [u8; LC],
	pub r: [u8; 2],
	pub g: [u8; 2],
	pub b: [u8; 2],
    pub clk: u8,
    pub lat: u8,
    pub oe: u8,
}

pub struct Hub75Pins<const LC: usize> {
    pub lines: [OutputPin; LC],
	pub r: [OutputPin; 2],
	pub g: [OutputPin; 2],
	pub b: [OutputPin; 2],
    pub clk: OutputPin,
    pub lat: OutputPin,
    pub oe: OutputPin,
}

impl<const LC: usize> Hub75Pins<LC> {
	pub fn from_pin_nums(pin_nums: &Hub75PinNums<LC>) -> Hub75Pins<LC> {
		let gpio = Gpio::new().unwrap();

		let lines: [OutputPin; LC] = pin_nums.lines.into_iter().map(|x|
			 gpio.get(x).expect("Couldn't get pin").into_output()
		).collect::<Vec<OutputPin>>().try_into().unwrap();
		
		let r: [OutputPin; 2] = pin_nums.r.into_iter().map(|x|
			 gpio.get(x).unwrap().into_output()
		).collect::<Vec<OutputPin>>().try_into().unwrap();

		let g: [OutputPin; 2] = pin_nums.g.into_iter().map(|x|
			 gpio.get(x).unwrap().into_output()
		).collect::<Vec<OutputPin>>().try_into().unwrap();

		let b: [OutputPin; 2] = pin_nums.b.into_iter().map(|x|
			 gpio.get(x).unwrap().into_output()
		).collect::<Vec<OutputPin>>().try_into().unwrap();

		Hub75Pins {
			lines, r, g, b,
			clk: gpio.get(pin_nums.clk).unwrap().into_output(),
			lat: gpio.get(pin_nums.lat).unwrap().into_output(),
			oe: gpio.get(pin_nums.oe).unwrap().into_output(),
		}
	}
}

pub struct Hub75Panel<const LC: usize> {
	cols: usize,
	rows: usize,
	pins: Hub75Pins<LC>,
	active_row: usize,
}

impl<const LC: usize> Hub75Panel<LC> {
	pub fn new(cols: usize, rows: usize, pins: Hub75PinNums<LC>) -> Hub75Panel<LC> {
		let pins = Hub75Pins::from_pin_nums(&pins);
		Hub75Panel {cols, rows, pins, active_row: 0}
	}

	pub fn strobe_row(&mut self, img_buffer: &ImgBuffer) {
		let row_data = img_buffer.get_display_row(self.active_row);

		for i in 0..self.cols {
			self.set_pins_for_byte(row_data[i]);
			self.clock();
		}

		self.pins.oe.set_high();
		self.latch();
		self.select_row(self.active_row);
		self.pins.oe.set_low();

		self.active_row = (self.active_row + 1) % 16;
	}

	fn set_pins_for_byte(&mut self, byte: u8) {
		self.pins.r[0].write((byte & 1 != 0).into());
		self.pins.g[0].write((byte & 2 != 0).into());
		self.pins.b[0].write((byte & 4 != 0).into());
		self.pins.r[1].write((byte & 8 != 0).into());
		self.pins.g[1].write((byte & 16 != 0).into());
		self.pins.b[1].write((byte & 32 != 0).into());
	}

	pub fn test(&mut self) {
		for i in 0..(self.rows / 2) {
			for i in 0..self.cols {
				if i < 32 {
					self.pins.r[0].set_high();
					self.pins.b[1].set_high();
					self.pins.g[0].set_low();
					self.pins.g[1].set_low();
				} else {
					self.pins.g[0].set_high();
					self.pins.g[1].set_high();
				}
				
				self.clock();
			}

			self.pins.oe.set_high();
			self.latch();
			self.select_row(i);
			self.pins.oe.set_low();
		}
	}

	pub fn select_row(&mut self, row: usize) {
		for line in self.pins.lines.iter_mut() {
			line.set_low();
		}

		match row {
			0 => {},
			1 => {
				self.pins.lines[0].set_high();
			},
			2 => {
				self.pins.lines[1].set_high();
			},
			3 => {
				self.pins.lines[0].set_high();
				self.pins.lines[1].set_high();
			},
			4 => {
				self.pins.lines[2].set_high();
			},
			5 => {
				self.pins.lines[0].set_high();
				self.pins.lines[2].set_high();
			},
			6 => {
				self.pins.lines[1].set_high();
				self.pins.lines[2].set_high();
			},
			7 => {
				self.pins.lines[0].set_high();
				self.pins.lines[1].set_high();
				self.pins.lines[2].set_high();
			},
			8 => {
				self.pins.lines[3].set_high();
			},
			9 => {
				self.pins.lines[0].set_high();
				self.pins.lines[3].set_high();
			},
			10 => {
				self.pins.lines[1].set_high();
				self.pins.lines[3].set_high();
			},
			11 => {
				self.pins.lines[0].set_high();
				self.pins.lines[1].set_high();
				self.pins.lines[3].set_high();
			},
			12 => {
				self.pins.lines[2].set_high();
				self.pins.lines[3].set_high();
			},
			13 => {
				self.pins.lines[0].set_high();
				self.pins.lines[2].set_high();
				self.pins.lines[3].set_high();
			},
			14 => {
				self.pins.lines[1].set_high();
				self.pins.lines[2].set_high();
				self.pins.lines[3].set_high();
			},
			15 => {
				self.pins.lines[0].set_high();
				self.pins.lines[1].set_high();
				self.pins.lines[2].set_high();
				self.pins.lines[3].set_high();
			}
			_ => {}
		}
	}

	pub fn clock(&mut self) {
		self.pins.clk.set_high();
		//thread::sleep(Duration::from_micros(1));
		self.pins.clk.set_low();
		//thread::sleep(Duration::from_micros(1));
	}

	pub fn latch(&mut self) {
		self.pins.lat.set_high();
		//thread::sleep(Duration::from_micros(1));
		self.pins.lat.set_low();
	}

	pub fn blank(&mut self) {
		for line in self.pins.lines.iter_mut() {
			line.set_low();
		}

		self.pins.oe.set_low();
	}
}