use super::*;
use image::{DynamicImage, ImageBuffer, Rgba};

pub fn open(path: &String) -> DynamicImage {
    image::open(path).expect("File not found!")
}

pub fn gray_scale_operation(pixel: &Rgba<u8>, gama: f64) -> u8 {
    let r = pixel[0] as f64;
    let g = pixel[1] as f64;
    let b = pixel[2] as f64;
    if r == g && g == b {
        // already on grayscale
        return r.powf(gama) as u8;
    }
    let r = (0.2126 * r).powf(gama) as u8;
    let g = (0.7152 * g).powf(gama) as u8;
    let b = (0.0722 * b).powf(gama) as u8;
    r + g + b // grayscale
}


pub fn set_buffer_to_rgba8(image: &ProcessImageObj) -> (DynamicImage, ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let img = open(&image.path);
    let img_dims = img.to_rgba8().dimensions();

    let img_buffer: ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        ImageBuffer::new(img_dims.0, img_dims.1);
    (img, img_buffer)
}

