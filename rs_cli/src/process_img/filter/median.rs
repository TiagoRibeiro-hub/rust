
use image::Rgba;

use crate::process_img::{
    models::ImageRgba, utils::set_buffer_and_dimensions_to_rgba8, ProcessImageObj,
};

use super::{get_kernel_x, get_kernel_y};

pub fn func(image: &ProcessImageObj) -> ImageRgba {
    let (mut old_img, dimensions, mut new_img) = set_buffer_and_dimensions_to_rgba8(image);

    let len = (image.k_size * image.k_size) as usize;
    let median: usize = len / 2;
    let kernel_half_size = image.k_size / 2;
    for y in 0..dimensions.1 {
        for x in 0..dimensions.0 {
            let mut pix_r: Vec<u8> = Vec::new();
            pix_r.reserve(len);
            let mut pix_g: Vec<u8> = Vec::new();
            pix_g.reserve(len);
            let mut pix_b: Vec<u8> = Vec::new();
            pix_b.reserve(len);
            let mut pix_a: Vec<u8> = Vec::new();
            pix_a.reserve(len);

            let mut count_sub_y = kernel_half_size;
            for kernel_y in 0..image.k_size {
                let mut count_col = 0;
                // HEIGHT
                let h = get_kernel_y(kernel_y, kernel_half_size, y, count_sub_y, dimensions);

                let mut count_sub_x = kernel_half_size;
                for kernel_x in 0..image.k_size {
                    // WIDHT
                    let w = get_kernel_x(
                        kernel_x,
                        kernel_half_size,
                        x,
                        count_sub_x,
                        &mut count_col,
                        dimensions,
                    );

                    let pixel = *old_img.get_pixel_mut(w, h);

                    pix_r.push(pixel[0]);
                    pix_g.push(pixel[1]);
                    pix_b.push(pixel[2]);
                    pix_a.push(pixel[3]);

                    if count_sub_x != 0 {
                        let count_x_res = count_sub_x.checked_sub(1);
                        match count_x_res {
                            Some(res) => count_sub_x = res,
                            None => count_sub_x = 0,
                        }
                    }
                }

                if count_sub_y != 0 {
                    let count_y_res = count_sub_y.checked_sub(1);
                    match count_y_res {
                        Some(res) => count_sub_y = res,
                        None => count_sub_y = 0,
                    }
                }
            }

            pix_r.select_nth_unstable(median);
            pix_g.select_nth_unstable(median);
            pix_b.select_nth_unstable(median);
            pix_a.select_nth_unstable(median);

            let r = pix_r[median];
            let g = pix_g[median];
            let b = pix_b[median];
            let a = pix_a[median];

            let pixel = new_img.get_pixel_mut(x, y);
            *pixel = Rgba([r, g, b, a])
        }
    }
    new_img
}

#[test]
fn filters() {
    // original 800 x 596
    let image =
        ProcessImageObj::from("/home/tiago/rust/projects/cli/imgs/chestnut_tailed_starling.jpg");
    // ! median 21 k_size 3.468167633s
    let start = std::time::Instant::now();
    let result = func(&image);
    let _ = result
        .save("/home/tiago/rust/projects/cli/imgs/chestnut_tailed_starling_filter_median_teste.png");
    println!("{:?}", start.elapsed());
}
