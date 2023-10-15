use image::{DynamicImage, ImageBuffer, Rgba};

use super::{models::ImageRgba, resize::ImgDimensions, ProcessImageObj};

pub fn open(path: &String) -> DynamicImage {
    image::open(path).expect("File not found!")
}

pub fn gray_scale_operation(pixel: &Rgba<u8>) -> u8 {
    let r = pixel[0] as f64;
    let g = pixel[1] as f64;
    let b = pixel[2] as f64;
    if r == g && g == b {
        // already on grayscale
        return r as u8;
    }
    let r = (0.2126 * r) as u8;
    let g = (0.7152 * g) as u8;
    let b = (0.0722 * b) as u8;
    r + g + b // grayscale
}

pub fn set_buffer_to_rgba8(
    image: &ProcessImageObj,
) -> (DynamicImage, ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let img = open(&image.path);
    let img_dims = img.to_rgba8().dimensions();

    let img_buffer: ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        ImageBuffer::new(img_dims.0, img_dims.1);
    (img, img_buffer)
}

pub fn set_props_for_resize_to_rgba8(
    image: &ProcessImageObj,
) -> (ImageRgba, ImgDimensions, (f64, f64), ImageRgba) {
    let img = open(&image.path);
    let old_img = img.to_rgba8();

    // * Dimensions
    let dimensions = ImgDimensions {
        new_dim: image.dimensions,
        old_dim: old_img.dimensions(),
    };
    let scale_factor = dimensions.scale_factor();

    let new_img: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::new(dimensions.new_dim.0, dimensions.new_dim.1);
    (old_img, dimensions, scale_factor, new_img)
}

pub fn set_buffer_and_dimensions_to_rgba8(
    image: &ProcessImageObj,
) -> (ImageRgba, (u32, u32), ImageRgba) {
    let img = open(&image.path);
    let old_img = img.to_rgba8();
    let dimensions = old_img.dimensions();
    let new_img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(dimensions.0, dimensions.1);
    (old_img, dimensions, new_img)
}

pub fn darken(pixel: &mut image::Rgba<u8>, gama: u8) -> (u8, u8, u8) {
    let r = pixel[0].saturating_sub(gama) as u32;
    let g = pixel[1].saturating_sub(gama) as u32;
    let b = pixel[2].saturating_sub(gama) as u32;
    (
        r.clamp(0, 255) as u8,
        g.clamp(0, 255) as u8,
        b.clamp(0, 255) as u8,
    )
}

pub fn lighten(pixel: &mut image::Rgba<u8>, gama: u8) -> (u8, u8, u8) {
    let r = pixel[0].saturating_add(gama) as u32;
    let g = pixel[1].saturating_add(gama) as u32;
    let b = pixel[2].saturating_add(gama) as u32;
    (
        r.clamp(0, 255) as u8,
        g.clamp(0, 255) as u8,
        b.clamp(0, 255) as u8,
    )
}

pub fn invert(pixel: &mut image::Rgba<u8>) -> (u8, u8, u8) {
    let r = pixel[0].abs_diff(255) as i32;
    let g = pixel[1].abs_diff(255) as i32;
    let b = pixel[2].abs_diff(255) as i32;
    (
        r.clamp(0, 255) as u8,
        g.clamp(0, 255) as u8,
        b.clamp(0, 255) as u8,
    )
}

pub fn low_contrast(pixel: &mut image::Rgba<u8>) -> (u8, u8, u8) {
    let r = pixel[0] as f32 / 2.0;
    let g = pixel[1] as f32 / 2.0;
    let b = pixel[2] as f32 / 2.0;
    (
        r.clamp(0.0, 255.0) as u8,
        g.clamp(0.0, 255.0) as u8,
        b.clamp(0.0, 255.0) as u8,
    )
}

pub fn high_contrast(pixel: &mut image::Rgba<u8>) -> (u8, u8, u8) {
    let r = pixel[0] as u32 * 2;
    let g = pixel[1] as u32 * 2;
    let b = pixel[2] as u32 * 2;
    (
        r.clamp(0, 255) as u8,
        g.clamp(0, 255) as u8,
        b.clamp(0, 255) as u8,
    )
}
