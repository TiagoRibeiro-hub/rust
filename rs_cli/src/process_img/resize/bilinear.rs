use image::ImageBuffer;

use crate::process_img::{ProcessImageObj, ImageRgba, utils::open};

pub fn resize(mut image: ProcessImageObj) -> ImageRgba {
    
    let img = open(&image.path);
    let old_img = img.to_rgba8();

    // * Dimensions
    let new_dim = image.scale.new_dim;
    image.scale.old_dim = old_img.dimensions();
    let scale = image.scale.set();

    let mut new_img: ImageBuffer<image::Rgba<u8>, Vec<u8>> =
    ImageBuffer::new(new_dim.0, new_dim.1);

    for (w, h, pixel) in new_img.enumerate_pixels_mut() {
        let old_pixel = old_img.get_pixel(w, h);
    }


    new_img
}