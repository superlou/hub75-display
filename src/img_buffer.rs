use crate::font::{Font, FontChar};

pub struct ImgBuffer {
    plane: [u8; 32*64],
    rows: usize,
    cols: usize,
}

#[derive(Clone, Copy)]
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

    fn draw_font_char(&mut self, font_char: &FontChar, x0: usize, y0: usize, color: Color) {
        let data = font_char.data();
        let width = font_char.width();

        for (i, d) in data.iter().enumerate() {
            let x = (i % width) + x0;
            let y = (i / width) + y0;

            if *d > 0 {
                self.set_pixel(x, y, color);
            }
        }
    }

    pub fn draw_str(&mut self, text: &str, font: &Font, x0: usize, y0: usize, color: Color) {
        let mut x = x0;
        let char_spacing = 1;

        for c in text.chars() {
            match font.char(&c) {
                Some(font_char) => {
                    self.draw_font_char(font_char, x, y0, color);
                    x += font_char.width() + char_spacing;
                },
                None => {}
            }
        }
    }
}
