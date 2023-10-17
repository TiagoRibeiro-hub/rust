// grayscale = 0.2126 R ^gama + 0.7152 G ^gama + 0.0722 B ^gama
// grayscale = 0.299 R ^gama + 0.587 G ^gama + 0.11 B ^gama => For images in color spaces such as Y'UV and its relatives, which are used in standard color TV and video systems
use super::{
    utils::set_buffer_to_rgba8, ProcessImageObj, models::ImageRgba,
};
use self::utils::*;
use image::{GenericImageView, Rgba};

mod utils;
mod traits;

pub fn gray(image: &ProcessImageObj) -> ImageRgba {
    let (img, mut img_buffer) = set_buffer_to_rgba8(image);

    for (w, h, pixel) in img_buffer.enumerate_pixels_mut() {
        *pixel = img.get_pixel(w, h);
        let grayscale = gray_scale_operation(pixel);
        *pixel = Rgba([grayscale, grayscale, grayscale, pixel[3]]);
    }
    img_buffer
}

pub fn blue(image: &ProcessImageObj) -> ImageRgba {
    let (img, mut img_buffer) = set_buffer_to_rgba8(image);

    for (w, h, pixel) in img_buffer.enumerate_pixels_mut() {
        *pixel = img.get_pixel(w, h);
        let grayscale = gray_scale_operation(pixel);
        *pixel = Rgba([0, 0, grayscale, pixel[3]]);
    }
    img_buffer
}

pub fn green(image: &ProcessImageObj) -> ImageRgba {
    let (img, mut img_buffer) = set_buffer_to_rgba8(image);

    for (w, h, pixel) in img_buffer.enumerate_pixels_mut() {
        *pixel = img.get_pixel(w, h);
        let grayscale = gray_scale_operation(pixel);
        *pixel = Rgba([0, grayscale, 0, pixel[3]]);
    }
    img_buffer
}

pub fn red(image: &ProcessImageObj) -> ImageRgba {
    let (img, mut img_buffer) = set_buffer_to_rgba8(image);

    for (w, h, pixel) in img_buffer.enumerate_pixels_mut() {
        *pixel = img.get_pixel(w, h);
        let grayscale = gray_scale_operation(pixel);
        *pixel = Rgba([grayscale, 0, 0, pixel[3]]);
    }
    img_buffer
}

pub fn darken(image: &ProcessImageObj) -> ImageRgba {
    let (img, mut img_buffer) = set_buffer_to_rgba8(image);

    for (w, h, pixel) in img_buffer.enumerate_pixels_mut() {
        *pixel = img.get_pixel(w, h);
        let (r, g, b) = utils::darken(pixel, image.gama);
        *pixel = Rgba([r, g, b, pixel[3]]);
    }
    img_buffer
}

pub fn lighten(image: &ProcessImageObj) -> ImageRgba {
    let (img, mut img_buffer) = set_buffer_to_rgba8(image);

    for (w, h, pixel) in img_buffer.enumerate_pixels_mut() {
        *pixel = img.get_pixel(w, h);
        let (r, g, b) = utils::lighten(pixel, image.gama);
        *pixel = Rgba([r, g, b, pixel[3]]);
    }
    img_buffer
}

pub fn invert(image: &ProcessImageObj) -> ImageRgba {
    let (img, mut img_buffer) = set_buffer_to_rgba8(image);

    for (w, h, pixel) in img_buffer.enumerate_pixels_mut() {
        *pixel = img.get_pixel(w, h);
        let (r, g, b) = utils::invert(pixel);
        *pixel = Rgba([r, g, b, pixel[3]]);
    }
    img_buffer
}

pub fn low_contrast(image: &ProcessImageObj) -> ImageRgba {
    let (img, mut img_buffer) = set_buffer_to_rgba8(image);

    for (w, h, pixel) in img_buffer.enumerate_pixels_mut() {
        *pixel = img.get_pixel(w, h);
        let (r, g, b) = utils::low_contrast(pixel);
        *pixel = Rgba([r, g, b, pixel[3]]);
    }
    img_buffer
}

pub fn high_contrast(image: &ProcessImageObj) -> ImageRgba {
    let (img, mut img_buffer) = set_buffer_to_rgba8(image);

    for (w, h, pixel) in img_buffer.enumerate_pixels_mut() {
        *pixel = img.get_pixel(w, h);
        let (r, g, b) = utils::high_contrast(pixel);
        *pixel = Rgba([r, g, b, pixel[3]]);
    }
    img_buffer
}

#[test]
fn colors() {
    // original 800 x 596
    let image =
        ProcessImageObj::from("/home/tiago/rust/projects/cli/imgs/chestnut_tailed_starling.jpg");
    // ! gray
    // * single 33.49516ms
    let start = std::time::Instant::now();
    let result = gray(&image);
    let _ = result.save("/home/tiago/rust/projects/cli/imgs/teste_par.png");
    println!("{:?}", start.elapsed());
}
