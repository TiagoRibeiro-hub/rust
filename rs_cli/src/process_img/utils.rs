use std::fs::File;

use super::*;
use colored::ColoredString;
use image::DynamicImage;
use image::ImageBuffer;
use colored::Colorize;

pub fn open(path: &String) -> DynamicImage {
    image::open(path).expect("File not found!")
}

fn gray_scale_operation(pixel: &mut Rgba<u8>, image: &Image) -> u8 {
    let r = pixel[0] as f64;
    let g = pixel[1] as f64;
    let b = pixel[2] as f64;
    if r == g && g == b {
        // already on grayscale
        return r.powf(image.gama) as u8;
    }
    let r = (0.2126 * r).powf(image.gama) as u8;
    let g = (0.7152 * g).powf(image.gama) as u8;
    let b = (0.0722 * b).powf(image.gama) as u8;
    r + g + b // grayscale
    
}

fn set_buffer_to_rgba8(image: &Image) -> (DynamicImage, ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let img = open(&image.path);
    let img_dims = img.to_rgba8().dimensions();

    let img_buffer: ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        ImageBuffer::new(img_dims.0, img_dims.1);
    (img, img_buffer)
}

pub fn gray_scale(image: &Image) -> ImageRgba {
    let (img, mut img_buffer) = set_buffer_to_rgba8(image);

    for (w, h, pixel) in img_buffer.enumerate_pixels_mut() {
        *pixel = img.get_pixel(w, h);
        let grayscale = gray_scale_operation(pixel, image);
        *pixel = image::Rgba([grayscale, grayscale, grayscale, pixel[3]]);
    }
    img_buffer
}

pub fn blue_scale(image: &Image) -> ImageRgba {
    let (img, mut img_buffer) = set_buffer_to_rgba8(image);

    for (w, h, pixel) in img_buffer.enumerate_pixels_mut() {
        *pixel = img.get_pixel(w, h);
        let grayscale = gray_scale_operation(pixel, image);
        *pixel = image::Rgba([0, 0, grayscale, pixel[3]]);
    }
    img_buffer
}

pub fn green_scale(image: &Image) -> ImageRgba {
    let (img, mut img_buffer) = set_buffer_to_rgba8(image);

    for (w, h, pixel) in img_buffer.enumerate_pixels_mut() {
        *pixel = img.get_pixel(w, h);
        let grayscale = gray_scale_operation(pixel, image);
        *pixel = image::Rgba([0, grayscale, 0, pixel[3]]);
    }
    img_buffer
}

pub fn red_scale(image: &Image) -> ImageRgba {
    let (img, mut img_buffer) = set_buffer_to_rgba8(image);

    for (w, h, pixel) in img_buffer.enumerate_pixels_mut() {
        *pixel = img.get_pixel(w, h);
        let grayscale = gray_scale_operation(pixel, image);
        *pixel = image::Rgba([grayscale, 0, 0, pixel[3]]);
    }
    img_buffer
}

pub fn resize(img: &ImageRgba, dims: (u32, u32)) -> ImageRgba {
    let (old_width, old_height) = img.dimensions();
    let (new_width, new_height) = dims;

    let mut img_buffer: ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        ImageBuffer::new(new_width, new_height);

    for (current_w, current_h, pixel) in img_buffer.enumerate_pixels_mut() {
        let w = (current_w as f32 * (old_width as f32 / new_width as f32)) as u32;
        let h = (current_h as f32 * (old_height as f32 / new_height as f32)) as u32;

        *pixel = *img.get_pixel(w, h);
    }
    img_buffer
}


pub fn ascii_art(image: &Image) -> Result<String, CustomError> {
    let (img, mut img_buffer) = set_buffer_to_rgba8(image);

    let x = (255.0 / image.chars.len() as f64).ceil();
    let mut ascii_art: String = String::default();
    
    for (w, h, pixel) in img_buffer.enumerate_pixels_mut() {
        *pixel = img.get_pixel(w, h);
        let grayscale = gray_scale_operation(pixel, &image);

        let idx = (grayscale as f64 / x).ceil();
        let char = image.chars[idx as usize];

        ascii_art.push_str(format!("{}", char.to_string().truecolor(grayscale, grayscale, grayscale)).as_ref());
    }

    Ok(ascii_art)
}