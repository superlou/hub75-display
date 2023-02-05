use std::fs::read;
use std::error::Error;

pub struct PPM {
    format: String,
    width: usize,
    height: usize,
    max: usize,
    pixels: Vec<Pixel>
}

#[derive(PartialEq, Debug)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

pub enum ParseState {
    Start,
    FoundFormat,
    FoundWidth,
    FoundHeight,
    FoundMax,
}

impl Pixel {
    pub fn new(data: &[u8]) -> Self {
        assert_eq!(data.len(), 3);

        Pixel {
            r: data[0], g: data[1], b: data[2]
        }
    }
    
    pub fn is_black(&self) -> bool {
        self.r == 0 && self.g == 0 && self.b == 0
    }
}

impl PPM {
    pub fn from_vec(v: &Vec<u8>) -> Self {
        let mut state = ParseState::Start;
        let mut in_comment = false;
        
        let mut buffer = String::new();
        
        let newline = 10u8;
        let space = 32u8;
        let pound = 35u8;
        
        let mut format = String::new();
        let mut width = 0usize;
        let mut height = 0usize;
        let mut max = 0usize;
        
        let mut header_len = 0;
        
        for b in v {
            header_len += 1;
            
            if in_comment {
                // Don't do anything except check if we are done with the comment
                if b == &newline {
                    in_comment = false;
                }
            } else if [newline, space, pound].contains(b) {
                if b == &pound {
                    in_comment = true;
                }
                
                if buffer.len() == 0 {
                    continue;
                }
                
                dbg!(&buffer);
                match state {
                    ParseState::Start => {
                        format = buffer;
                        state = ParseState::FoundFormat;
                    },
                    ParseState::FoundFormat => {
                        width = str::parse(&buffer).unwrap();
                        state = ParseState::FoundWidth;
                    },
                    ParseState::FoundWidth => {
                        height = str::parse(&buffer).unwrap();
                        state = ParseState::FoundHeight;
                    },
                    ParseState::FoundHeight => {
                        max = str::parse(&buffer).unwrap();
                        state = ParseState::FoundMax;
                        break;
                    },
                    _ => {},
                };
                    
                buffer = String::new();
            } else {
                buffer.push(*b as char);
            }
        }
        
        dbg!(&header_len);

        let pixel_data = &v[header_len..];
        let pixels: Vec<Pixel> = pixel_data
                                .chunks(3)
                                .map(|x| Pixel::new(x))
                                .collect();
        
        PPM {
            format, width, height, max,
            pixels: pixels,
        }
    }

    pub fn load(filename: &str) -> Result<Self, Box<dyn Error>> {
        let data = read(filename)?;
        Ok(Self::from_vec(&data))
    }

    pub fn format(&self) -> &str {
        &self.format
    }

    pub fn pixel(&self, x: usize, y: usize) -> &Pixel {
        let index = y * self.width + x;
        &self.pixels[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn header1() {
        let h = "P6 1024 788 255\n\x01\x02\x03\x04\x05\x06"
                .as_bytes().to_vec();
        let ppm = PPM::from_vec(&h);
        
        assert_eq!(ppm.format, "P6");
        assert_eq!(ppm.width, 1024);
        assert_eq!(ppm.height, 788);
        assert_eq!(ppm.max, 255);
        assert_eq!(ppm.pixels.len(), 2);
        assert_eq!(ppm.pixel(0, 0), &Pixel::new(&[1, 2, 3]));
        assert_eq!(ppm.pixel(1, 0), &Pixel::new(&[4, 5, 6]));
    }
    
    #[test]
    fn header2() {
        let h = "P6\n\
                1024 788\n\
                # A comment\n\
                255\n\
                \x01\x02\x03\x04\x05\x06"
                .as_bytes().to_vec();
        let ppm = PPM::from_vec(&h);
        
        assert_eq!(ppm.format, "P6");
        assert_eq!(ppm.width, 1024);
        assert_eq!(ppm.height, 788);
        assert_eq!(ppm.max, 255);
        assert_eq!(ppm.pixels.len(), 2);
        assert_eq!(ppm.pixel(0, 0), &Pixel::new(&[1, 2, 3]));
        assert_eq!(ppm.pixel(1, 0), &Pixel::new(&[4, 5, 6]));
    }
    
    #[test]
    fn header3() {
        let h = "P6\n\
                 1024 # the image width\n\
                 788 # the image height\n\
                 # A comment\n\
                 255\n\
                 \x01\x02\x03\x04\x05\x06"
                 .as_bytes().to_vec();
        let ppm = PPM::from_vec(&h);
        
        assert_eq!(ppm.format, "P6");
        assert_eq!(ppm.width, 1024);
        assert_eq!(ppm.height, 788);
        assert_eq!(ppm.max, 255);
        assert_eq!(ppm.pixels.len(), 2);
        assert_eq!(ppm.pixel(0, 0), &Pixel::new(&[1, 2, 3]));
        assert_eq!(ppm.pixel(1, 0), &Pixel::new(&[4, 5, 6]));
    }

    #[test]
    fn square_image() {
        let h = "P6 2 2 256\n\
                \x01\x02\x03\x04\x05\x06\
                \x7a\x7b\x7c\x7d\x7e\x7f"
                .as_bytes().to_vec();
        let ppm = PPM::from_vec(&h);

        assert_eq!(ppm.width, 2);
        assert_eq!(ppm.height, 2);
        assert_eq!(ppm.pixel(0, 0), &Pixel::new(&[1, 2, 3]));
        assert_eq!(ppm.pixel(1, 0), &Pixel::new(&[4, 5, 6]));
        assert_eq!(ppm.pixel(0, 1), &Pixel::new(&[122, 123, 124]));
        assert_eq!(ppm.pixel(1, 1), &Pixel::new(&[125, 126, 127]));
    }

    #[test]
    fn check_black_pixels() {
        let pixel = Pixel::new(&[0, 0, 0]);
        assert_eq!(pixel.is_black(), true);

        let pixel = Pixel::new(&[0, 0, 1]);
        assert_eq!(pixel.is_black(), false);

        let pixel = Pixel::new(&[0, 1, 0]);
        assert_eq!(pixel.is_black(), false);
        
        let pixel = Pixel::new(&[1, 0, 0]);
        assert_eq!(pixel.is_black(), false);        
    }

    #[test]
    fn can_load_from_file() {
        let ppm = PPM::load("fonts/57.ppm").unwrap();
        assert_eq!(ppm.format(), "P6");
    }
}