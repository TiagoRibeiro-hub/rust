use image::Rgba;

use crate::process_img::{
    models::ImageRgba, utils::set_buffer_and_dimensions_to_rgba8, ProcessImageObj,
};

use super::get_kernel_pixel;

pub fn func(image: &ProcessImageObj) -> ImageRgba {
    let (mut old_img, dimensions, mut new_img) = set_buffer_and_dimensions_to_rgba8(image);

    let divisor: u32 = image.n_size * image.n_size;
    let half = image.n_size / 2;
    for y in 0..dimensions.1 {
        for x in 0..dimensions.0 {
            let (pix_r, pix_g, pix_b, pix_a) =
                get_kernel_pixel(image.n_size, half, y, x, dimensions, &mut old_img);

            let r: u32 = average(pix_r, divisor);
            let g: u32 = average(pix_g, divisor);
            let b: u32 = average(pix_b, divisor);
            let a: u32 = average(pix_a, divisor);

            let pixel = new_img.get_pixel_mut(x, y);
            *pixel = Rgba([
                r.clamp(0, 255) as u8,
                g.clamp(0, 255) as u8,
                b.clamp(0, 255) as u8,
                a.clamp(0, 255) as u8,
            ])
        }
    }

    new_img
}


fn average(vec: Vec<u32>, divisor: u32) -> u32 {
    let mut somatory: u32 = vec.iter().sum();
    somatory / divisor
}

#[test]
fn filters() { 
    // original 800 x 596
    let image = ProcessImageObj::from("/home/tiago/rust/projects/cli/imgs/chestnut_tailed_starling.jpg");
    // ! box - 122.336207ms
    let start = std::time::Instant::now();
    let result = func(&image);
    let _ = result.save("/home/tiago/rust/projects/cli/imgs/chestnut_tailed_starling_filter_box_fn.png");
    println!("{:?}", start.elapsed());
}
