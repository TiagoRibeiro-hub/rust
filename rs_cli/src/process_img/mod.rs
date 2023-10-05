// https://en.wikipedia.org/wiki/Grayscale#Converting_color_to_grayscale
// grayscale = 0.2126 R ^gama + 0.7152 G ^gama + 0.0722 B ^gama
// grayscale = 0.299 R ^gama + 0.587 G ^gama + 0.11 B ^gama => For images in color spaces such as Y'UV and its relatives, which are used in standard color TV and video systems

mod utils;

use image::{GenericImageView, ImageBuffer, Rgba};

type ImageRgba = ImageBuffer<Rgba<u8>, Vec<u8>>;

#[derive(Debug)]
pub struct Image {
    pub path: String,
    pub gama: f64,
    pub dimensions: (u32, u32),
    pub chars: Vec<char>,
}

impl From<&str> for Image {
    fn from(path: &str) -> Self {       
        Image {
            path: path.to_string(),
            gama: 1.0,
            dimensions: (5,5),
            chars: vec!['@', '#', '$', '%', '?', '*', ':', '+', '-',',', '.',' ']
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


impl Image {
    pub fn color_scale(&self, color: ColorScale) -> ImageRgba {
        match color {
            ColorScale::Gray() => {
                utils::gray_scale(self)
            },
            ColorScale::Blue() => {
                utils::blue_scale(self)
            },
            ColorScale::Green() => {
                utils::green_scale(self)
            },
            ColorScale::Red() => {
                utils::red_scale(self)
            },
                
        }
    }

    pub fn pixelate(&self) -> ImageRgba {
        let img = utils::open(&self.path);
        let img_dims = img.dimensions();
        let small_img = utils::resize(
            &img.to_rgba8(),
            (
                (img_dims.0 / self.dimensions.0),
                (img_dims.1 / self.dimensions.1),
            ),
        );
        utils::resize(&small_img, img_dims) // pixelate
    }

    pub fn ascii_art(self) -> ImageRgba {
        utils::ascii_art(self)
    }
}
