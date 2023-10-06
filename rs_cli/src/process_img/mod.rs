// https://en.wikipedia.org/wiki/Grayscale#Converting_color_to_grayscale
// grayscale = 0.2126 R ^gama + 0.7152 G ^gama + 0.0722 B ^gama
// grayscale = 0.299 R ^gama + 0.587 G ^gama + 0.11 B ^gama => For images in color spaces such as Y'UV and its relatives, which are used in standard color TV and video systems
use image::{GenericImageView, ImageBuffer, Rgba};

mod utils;
mod color_scale;
mod pixelate;

type ImageRgba = ImageBuffer<Rgba<u8>, Vec<u8>>;

#[derive(Debug)]
pub struct Image<'a> {
    pub path: String,
    pub gama: f64,
    pub dimensions: (u32, u32),
    pub chars: Vec<&'a str>,
}

impl From<&str> for Image<'_> {
    fn from(path: &str) -> Self {       
        Image {
            path: path.to_string(),
            gama: 1.0,
            dimensions: (5,5),
            chars: vec![" ",".",",","-","~","+"]
        }
     }
}


pub enum ColorScale {
    Gray(),
    Blue(),
    Green(),
    Red(),
}
impl ColorScale {
    pub(crate) fn defaut() -> ColorScale {
        ColorScale::Gray()
    }
}


impl Image<'_> {
    pub fn color_scale(&self, color: ColorScale) -> ImageRgba {
        match color {
            ColorScale::Gray() => {
                color_scale::gray(self)
            },
            ColorScale::Blue() => {
                color_scale::blue(self)
            },
            ColorScale::Green() => {
                color_scale::green(self)
            },
            ColorScale::Red() => {
                color_scale::red(self)
            },
                
        }
    }

    pub fn pixelate(&self) -> ImageRgba {
        pixelate::pixelate(self)
    }

    pub fn _ascii_art(&self) -> ImageRgba {
        utils::ascii_art(self)
    }
}
