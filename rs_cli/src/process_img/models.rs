use image::{ImageBuffer, Rgba};


pub type ImageRgba = ImageBuffer<Rgba<u8>, Vec<u8>>;

pub enum ColorsProcesses {
    Gray(),
    Blue(),
    Green(),
    Red(),
    Darken(),
    Lighten(),
    Invert(),
    LowContrast(),
    HighContrast(),
}

pub enum ResizeForm {
    Bilinear(),
    Bicubic(),
}

pub enum Filter {
    Box(),
}