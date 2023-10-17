use image::Rgba;
use super::traits::*;

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

pub fn darken(pixel: &mut image::Rgba<u8>, gama: u8) -> (u8, u8, u8) {
    let r = pixel[0].saturating_sub(gama) as u32;
    let g = pixel[1].saturating_sub(gama) as u32;
    let b = pixel[2].saturating_sub(gama) as u32;
    (r, g, b).to_rgb_clamp()
}

pub fn lighten(pixel: &mut image::Rgba<u8>, gama: u8) -> (u8, u8, u8) {
    let r = pixel[0].saturating_add(gama) as u32;
    let g = pixel[1].saturating_add(gama) as u32;
    let b = pixel[2].saturating_add(gama) as u32;
    (r, g, b).to_rgb_clamp()
}

pub fn invert(pixel: &mut image::Rgba<u8>) -> (u8, u8, u8) {
    let r = pixel[0].abs_diff(255) as i32;
    let g = pixel[1].abs_diff(255) as i32;
    let b = pixel[2].abs_diff(255) as i32;
    (r, g, b).to_rgb_clamp()
}

pub fn low_contrast(pixel: &mut image::Rgba<u8>) -> (u8, u8, u8) {
    let r = pixel[0] as f32 / 2.0;
    let g = pixel[1] as f32 / 2.0;
    let b = pixel[2] as f32 / 2.0;
    (r, g, b).to_rgb_clamp()
}

pub fn high_contrast(pixel: &mut image::Rgba<u8>) -> (u8, u8, u8) {
    let r = pixel[0] as u32 * 2;
    let g = pixel[1] as u32 * 2;
    let b = pixel[2] as u32 * 2;
    (r, g, b).to_rgb_clamp()
}
