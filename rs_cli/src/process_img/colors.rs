// https://en.wikipedia.org/wiki/Grayscale#Converting_color_to_grayscale
// grayscale = 0.2126 R ^gama + 0.7152 G ^gama + 0.0722 B ^gama
// grayscale = 0.299 R ^gama + 0.587 G ^gama + 0.11 B ^gama => For images in color spaces such as Y'UV and its relatives, which are used in standard color TV and video systems
use image::GenericImageView;
use super::{*, utils::{set_buffer_to_rgba8, gray_scale_operation}};

pub fn gray(image: &ProcessImageObj) -> ImageRgba {
    let (img, mut img_buffer) = set_buffer_to_rgba8(image);

    for (w, h, pixel) in img_buffer.enumerate_pixels_mut() {
        *pixel = img.get_pixel(w, h);
        let grayscale = gray_scale_operation(pixel);
        *pixel = image::Rgba([grayscale, grayscale, grayscale, pixel[3]]);
    }
    img_buffer
}

pub fn blue(image: &ProcessImageObj) -> ImageRgba {
    let (img, mut img_buffer) = set_buffer_to_rgba8(image);

    for (w, h, pixel) in img_buffer.enumerate_pixels_mut() {
        *pixel = img.get_pixel(w, h);
        let grayscale = gray_scale_operation(pixel);
        *pixel = image::Rgba([0, 0, grayscale, pixel[3]]);
    }
    img_buffer
}

pub fn green(image: &ProcessImageObj) -> ImageRgba {
    let (img, mut img_buffer) = set_buffer_to_rgba8(image);

    for (w, h, pixel) in img_buffer.enumerate_pixels_mut() {
        *pixel = img.get_pixel(w, h);
        let grayscale = gray_scale_operation(pixel);
        *pixel = image::Rgba([0, grayscale, 0, pixel[3]]);
    }
    img_buffer
}

pub fn red(image: &ProcessImageObj) -> ImageRgba {
    let (img, mut img_buffer) = set_buffer_to_rgba8(image);

    for (w, h, pixel) in img_buffer.enumerate_pixels_mut() {
        *pixel = img.get_pixel(w, h);
        let grayscale = gray_scale_operation(pixel);
        *pixel = image::Rgba([grayscale, 0, 0, pixel[3]]);
    }
    img_buffer
}

pub fn darken(image: &ProcessImageObj) -> ImageRgba {
    let (img, mut img_buffer) = set_buffer_to_rgba8(image);

    for (w, h, pixel) in img_buffer.enumerate_pixels_mut() {
        *pixel = img.get_pixel(w, h);
        let (r, g, b) = utils::darken(pixel, image.gama);
        *pixel = image::Rgba([r, g, b, pixel[3]]);
    }
    img_buffer
}

pub fn lighten(image: &ProcessImageObj) -> ImageRgba {
    let (img, mut img_buffer) = set_buffer_to_rgba8(image);

    for (w, h, pixel) in img_buffer.enumerate_pixels_mut() {
        *pixel = img.get_pixel(w, h);
        let (r, g, b) = utils::lighten(pixel, image.gama);
        *pixel = image::Rgba([r, g, b, pixel[3]]);
    }
    img_buffer
}

pub fn invert(image: &ProcessImageObj) -> ImageRgba {
    let (img, mut img_buffer) = set_buffer_to_rgba8(image);

    for (w, h, pixel) in img_buffer.enumerate_pixels_mut() {
        *pixel = img.get_pixel(w, h);

        let (r, g, b) = utils::invert(pixel);

        *pixel = image::Rgba([r, g, b, pixel[3]]);
    }
    img_buffer
}

pub fn low_contrast(image: &ProcessImageObj) -> ImageRgba {
    let (img, mut img_buffer) = set_buffer_to_rgba8(image);

    for (w, h, pixel) in img_buffer.enumerate_pixels_mut() {
        *pixel = img.get_pixel(w, h);
        let (r, g, b) = utils::low_contrast(pixel);
        *pixel = image::Rgba([r, g, b, pixel[3]]);
    }
    img_buffer
}

pub fn high_contrast(image: &ProcessImageObj) -> ImageRgba {
    let (img, mut img_buffer) = set_buffer_to_rgba8(image);

    for (w, h, pixel) in img_buffer.enumerate_pixels_mut() {
        *pixel = img.get_pixel(w, h);
        let (r, g, b) = utils::high_contrast(pixel);
        *pixel = image::Rgba([r, g, b, pixel[3]]);
    }
    img_buffer
}