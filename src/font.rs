use std::collections::HashMap;
use std::fs::read_to_string;
use std::error::Error;
use serde::Deserialize;
use crate::ppm::PPM;

pub struct Font {
    char_map: HashMap::<char, FontChar>,
}

pub struct FontChar {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

#[derive(Deserialize, Debug)]
struct FontConfig {
    image: String,
    char_width: usize,
    char_height: usize,
    chars: HashMap<char, CharConfig>,
}

#[derive(Deserialize, Debug)]
struct CharConfig {
    pos: Vec<usize>
}

impl Font {
    pub fn load(filename: &str) -> Result<Self, Box<dyn Error>> {
        let config_text = read_to_string(filename)?;
        let config: FontConfig = toml::from_str(&config_text)?;
        
        let image = PPM::load(&config.image)?;

        let mut char_map = HashMap::<char, FontChar>::new();
        
        for (c, char_config) in &config.chars {
            let x0 = char_config.pos[0];
            let y0 = char_config.pos[1];
            let width = config.char_width;
            let height = config.char_height;
            let num_pixels = width * height;

            let mut data = vec![];

            for i in 0..num_pixels {
                let x = (i % width) + x0;
                let y = i / width + y0;

                if let Some(pixel) = image.pixel(x, y) {
                    if pixel.is_black() {
                        data.push(0u8);
                    } else {
                        data.push(255u8);
                    }
                }
            }

            let font_char = FontChar::new(config.char_width, config.char_height, data);
            char_map.insert(*c, font_char);
        }
        
        Ok(Font {
            char_map
        })
    }
    
    pub fn len(&self) -> usize {
        self.char_map.len()
    }
    
    pub fn char(&self, c: &char) -> Option<&FontChar> {
        self.char_map.get(c)
    }
}

impl FontChar {
    pub fn new(width: usize, height: usize, data: Vec<u8>) -> Self {
        Self {
            width, height, data,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }
    
    pub fn height(&self) -> usize {
        self.height
    }
    
    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_load() {
        let font = Font::load("fonts/57.toml").unwrap();
        let font_char = font.char(&'A').unwrap();

        assert_eq!(font.len(), 65);
        assert_eq!(font_char.width(), 5);
        assert_eq!(font_char.height(), 7);
        assert_eq!(font_char.data(), &vec![
            0,     0, 255,   0,   0,
            0,   255,   0, 255,   0,
            255,   0,   0,   0, 255,
            255,   0,   0,   0, 255,
            255, 255, 255, 255, 255,
            255,   0,   0,   0, 255,
            255,   0,   0,   0, 255,
        ]);

        let font_char = font.char(&'D').unwrap();
        assert_eq!(font_char.data(), &vec![
            255, 255, 255, 255,   0,
              0, 255,   0,   0, 255,
              0, 255,   0,   0, 255,
              0, 255,   0,   0, 255,
              0, 255,   0,   0, 255,
              0, 255,   0,   0, 255,
            255, 255, 255, 255,   0,
        ]);        
    }
}