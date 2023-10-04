use super::*;
use image::DynamicImage;
use image::ImageBuffer;

pub fn open(path: &String) -> DynamicImage {
    image::open(path).expect("File not found!")
}

fn gray_scale_operation(pixel: &mut Rgba<u8>, image: &Image) -> u8 {
    let r = pixel[0] as f64;
    let g = pixel[1] as f64;
    let b = pixel[2] as f64;
    let r = (0.2126 * r).powf(image.gama) as u8;
    let g = (0.7152 * g).powf(image.gama) as u8;
    let b = (0.0722 * b).powf(image.gama) as u8;
    r + g + b // grayscale
    
}

pub fn gray_scale(image: Image) -> ImageRgba {
    let img = open(&image.path);
    let img_dims = img.to_rgba8().dimensions();

    let mut img_buffer: ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        ImageBuffer::new(img_dims.0, img_dims.1);

    for (w, h, pixel) in img_buffer.enumerate_pixels_mut() {
        *pixel = img.get_pixel(w, h);
        let grayscale = gray_scale_operation(pixel, &image);
        *pixel = image::Rgba([grayscale, grayscale, grayscale, pixel[3]]);
    }
    img_buffer
}

pub fn resize(img: &ImageRgba, dims: (u32, u32)) -> ImageRgba {
    let (old_width, old_height) = img.dimensions();
    let (new_width, new_height) = dims;

    let mut resized: ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        ImageBuffer::new(new_width, new_height);

    for (current_w, current_h, pixel) in resized.enumerate_pixels_mut() {
        let w = (current_w as f32 * (old_width as f32 / new_width as f32)) as u32;
        let h = (current_h as f32 * (old_height as f32 / new_height as f32)) as u32;

        *pixel = *img.get_pixel(w, h);
    }
    resized
}
