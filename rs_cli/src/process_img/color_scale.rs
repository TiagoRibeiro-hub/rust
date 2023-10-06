use super::{*, utils::{set_buffer_to_rgba8, gray_scale_operation}};

pub fn gray(image: &Image) -> ImageRgba {
    let (img, mut img_buffer) = set_buffer_to_rgba8(image);

    for (w, h, pixel) in img_buffer.enumerate_pixels_mut() {
        *pixel = img.get_pixel(w, h);
        let grayscale = gray_scale_operation(pixel, image);
        *pixel = image::Rgba([grayscale, grayscale, grayscale, pixel[3]]);
    }
    img_buffer
}

pub fn blue(image: &Image) -> ImageRgba {
    let (img, mut img_buffer) = set_buffer_to_rgba8(image);

    for (w, h, pixel) in img_buffer.enumerate_pixels_mut() {
        *pixel = img.get_pixel(w, h);
        let grayscale = gray_scale_operation(pixel, image);
        *pixel = image::Rgba([0, 0, grayscale, pixel[3]]);
    }
    img_buffer
}

pub fn green(image: &Image) -> ImageRgba {
    let (img, mut img_buffer) = set_buffer_to_rgba8(image);

    for (w, h, pixel) in img_buffer.enumerate_pixels_mut() {
        *pixel = img.get_pixel(w, h);
        let grayscale = gray_scale_operation(pixel, image);
        *pixel = image::Rgba([0, grayscale, 0, pixel[3]]);
    }
    img_buffer
}

pub fn red(image: &Image) -> ImageRgba {
    let (img, mut img_buffer) = set_buffer_to_rgba8(image);

    for (w, h, pixel) in img_buffer.enumerate_pixels_mut() {
        *pixel = img.get_pixel(w, h);
        let grayscale = gray_scale_operation(pixel, image);
        *pixel = image::Rgba([grayscale, 0, 0, pixel[3]]);
    }
    img_buffer
}