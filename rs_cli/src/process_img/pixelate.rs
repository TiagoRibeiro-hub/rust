use image::{GenericImageView, ImageBuffer};

use super::{Image, ImageRgba, utils::open};

pub fn pixelate(image: &Image) -> ImageRgba {
    let img = open(&image.path);
    let img_dims = img.dimensions();
    let small_img = resize(
        &img.to_rgba8(),
        (
            (img_dims.0 / image.dimensions.0),
            (img_dims.1 / image.dimensions.1),
        ),
    );
    resize(&small_img, img_dims) // pixelate
}

fn resize(img: &ImageRgba, dims: (u32, u32)) -> ImageRgba {
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