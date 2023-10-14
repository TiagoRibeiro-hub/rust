use image::Rgba;

use crate::process_img::{
    models::ImageRgba, utils::set_buffer_and_dimensions_to_rgba8, ProcessImageObj,
};

pub fn box_fn(image: &ProcessImageObj) -> ImageRgba {
    let (mut old_img, dimensions, mut new_img) = set_buffer_and_dimensions_to_rgba8(image);

    let mask: Vec<i32> = vec![1, 2, 1, 2, 4, 2, 1, 2, 1];
    let divisor: i32 = mask.iter().sum();
    let n = 3;
    let half = n / 2;
    for y in 0..dimensions.1 {
        for x in 0..dimensions.0 {
            let (pix_r, pix_g, pix_b, pix_a) =
                kernel_pixel(n, half, y, x, dimensions, &mut old_img);

            if y > 150 {
                println!("")
            }
            let r: i32 = average(pix_r, &mask, divisor);
            let g: i32 = average(pix_g, &mask, divisor);
            let b: i32 = average(pix_b, &mask, divisor);
            let a: i32 = average(pix_a, &mask, divisor);

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

fn kernel_pixel(
    kernel_size: u32,
    kernel_half_size: u32,
    y: u32,
    x: u32,
    dimensions: (u32, u32),
    old_img: &mut image::ImageBuffer<Rgba<u8>, Vec<u8>>,
) -> (Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>) {
    let mut pix_r: Vec<i32> = Vec::new();
    let mut pix_g: Vec<i32> = Vec::new();
    let mut pix_b: Vec<i32> = Vec::new();
    let mut pix_a: Vec<i32> = Vec::new();

    let mut count_sub_y = kernel_half_size;
    for kernel_y in 0..kernel_size {
        let mut count_col = 0;
        let mut h = 0;
        // HEIGHT
        if kernel_y < kernel_half_size {
            if kernel_y < kernel_half_size {
                let h_res = y.checked_sub(count_sub_y);
                match h_res {
                    Some(point_h) => h = point_h,
                    None => h = 0,
                }
            }
        } else if kernel_y == kernel_half_size {
            h = y;
        } else {
            h = y + kernel_y - 1;
        }

        let mut count_sub_x = kernel_half_size;
        for kernel_x in 0..kernel_size {
            let mut w;
            // WIDHT
            if kernel_x < kernel_half_size {
                let w_res = x.checked_sub(count_sub_x);
                match w_res {
                    Some(point_w) => w = point_w,
                    None => w = 0,
                }
            } else if kernel_x == kernel_half_size {
                w = x;
                count_col += 1;
            } else {
                w = x + count_col;
                count_col += 1;
            }

            if h > (dimensions.1) - 1 {
                h = (dimensions.1) - 1;
            }

            if w > (dimensions.0) - 1 {
                w = (dimensions.0) - 1;
            }

            let pixel = *old_img.get_pixel_mut(w, h);

            pix_r.push(pixel[0] as i32);
            pix_g.push(pixel[1] as i32);
            pix_b.push(pixel[2] as i32);
            pix_a.push(pixel[3] as i32);

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

    (pix_r, pix_g, pix_b, pix_a)
}

fn average(vec: Vec<i32>, mask: &Vec<i32>, divisor: i32) -> i32 {
    let mut somatory: i32 = 0;
    let iter = vec.iter();

    for (i, pix) in iter.enumerate() {
        let x = mask[i] * pix;
        somatory += x;
    }

    somatory / divisor
}

#[test]
fn filters() {
    let start = std::time::Instant::now();
    // original 800 x 596
    let image = ProcessImageObj::from("/home/tiago/rust/projects/cli/imgs/low_quality_image.jpg");
    // ! box
    let result = box_fn(&image);
    let _ = result.save("/home/tiago/rust/projects/cli/imgs/filter_box_fn.png");

    println!("{:?}", start.elapsed());
}
