use image::{DynamicImage, ImageBuffer, Rgba};

use super::{models::ImageRgba, resize::ImgDimensions, ProcessImageObj};

pub fn open(path: &String) -> DynamicImage {
    image::open(path).expect("File not found!")
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

