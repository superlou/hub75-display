use std::collections::HashMap;

pub struct ImgBuffer {
    plane: [u8; 32*64],
    rows: usize,
    cols: usize,
}

pub enum Color {
    Red = 0x01,
    Green = 0x02,
    Yellow = 0x03,
    Blue = 0x04,
    Purple = 0x05,
    Teal = 0x06,
    White = 0x07,
}

impl ImgBuffer {
    pub fn new() -> ImgBuffer {
        ImgBuffer {
            plane: [0; 64*32],
            rows: 32,
            cols: 64,
        }
    }

    pub fn get_display_row(&self, row: usize) -> &[u8] {
        let len = self.cols;
        let start = row * len;
        let finish = row * len + len;
        &self.plane[start..finish]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        let i = (y % (self.rows / 2)) * self.cols + x;

        let mut data = self.plane[i];

        if y < self.rows / 2 {
            data &= 0b11111000;
            data |= color as u8;
        } else {
            data &= 0b1100011;
            data |= (color as u8) << 3;
        }
        
        self.plane[i] = data;
    }

    pub fn draw_character(&mut self, c: char, font: &HashMap<char, FontChar>, x: usize, y: usize, color: Color) {
        
    }
}

pub struct FontChar {
    width: usize,
    height: usize,
    data: [u8; 5*7],
}

impl FontChar {
    pub fn new(data: [u8; 5*7]) -> Self {
        Self {
            width: 5, height: 7, data,
        }
    }
}