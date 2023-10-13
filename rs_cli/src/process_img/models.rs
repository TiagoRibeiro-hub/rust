use image::{ImageBuffer, Rgba};


pub type ImageRgba = ImageBuffer<Rgba<u8>, Vec<u8>>;

pub enum ColorScale {
    Gray(),
    Blue(),
    Green(),
    Red(),
}
impl ColorScale {
    pub fn defaut() -> ColorScale {
        ColorScale::Gray()
    }
}

pub enum ResizeForm {
    Bilinear(),
    Bicubic(),
}
