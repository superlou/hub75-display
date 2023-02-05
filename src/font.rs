use std::collections::HashMap;
use std::fs::read_to_string;
use std::error::Error;
use serde::Deserialize;
use crate::ppm::PPM;

pub struct Font {
    char_map: HashMap::<char, FontChar>,
}

struct FontChar {
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

                let pixel = image.pixel(x, y);
                if pixel.is_black() {
                    data.push(0u8);
                } else {
                    data.push(255u8);
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
    
    pub fn width(&self, c: &char) -> usize {
        self.char_map[c].width
    }
    
    pub fn height(&self, c: &char) -> usize {
        self.char_map[c].height
    }
    
    pub fn char_data(&self, c: &char) -> &Vec<u8> {
        &self.char_map[c].data
    }
}

impl FontChar {
    pub fn new(width: usize, height: usize, data: Vec<u8>) -> Self {
        Self {
            width, height, data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_load() {
        let font = Font::load("fonts/57.toml").unwrap();
        assert_eq!(font.len(), 5);
        assert_eq!(font.width(&'A'), 5);
        assert_eq!(font.height(&'A'), 7);
        assert_eq!(font.char_data(&'A'), &vec![
            0,     0, 255,   0,   0,
            0,   255,   0, 255,   0,
            255,   0,   0,   0, 255,
            255,   0,   0,   0, 255,
            255, 255, 255, 255, 255,
            255,   0,   0,   0, 255,
            255,   0,   0,   0, 255,
        ]);
        
        assert_eq!(font.char_data(&'D'), &vec![
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