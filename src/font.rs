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
                
        let mut char_map = HashMap::<char, FontChar>::new();  
        
        for (c, char_config) in &config.chars {
            let data = vec![0];
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
    
    pub fn data(&self, c: &char) -> &Vec<u8> {
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
        assert_eq!(font.data(&'A'), &vec![
            0,     0, 255,   0,   0,
            0,   255,   0, 255,   0,
            255,   0,   0,   0, 255,
            255,   0,   0,   0, 255,
            255, 255, 255, 255, 255,
            255,   0,   0,   0, 255,
            255,   0,   0,   0, 255,
        ]);
        
        assert_eq!(font.data(&'D'), &vec![
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