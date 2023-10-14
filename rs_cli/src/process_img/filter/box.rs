use image::Rgba;

use crate::process_img::{
    models::ImageRgba, utils::set_buffer_and_dimensions_to_rgba8, ProcessImageObj,
};


pub fn func(image: &ProcessImageObj) -> ImageRgba {
    let (mut old_img, dimensions, mut new_img) = set_buffer_and_dimensions_to_rgba8(image);

    let divisor: u32 = image.k_size * image.k_size;
    let kernel_half_size = image.k_size / 2;
    for y in 0..dimensions.1 {
        for x in 0..dimensions.0 {
            let mut somatory_r: u32 = 0;
            let mut somatory_g: u32 = 0;
            let mut somatory_b: u32 = 0;
            let mut somatory_a: u32 = 0;
            
            let mut count_sub_y = kernel_half_size;
            for kernel_y in 0..image.k_size {
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
                } else {
                    if kernel_y == kernel_half_size {
                        h = y;
                    } else {
                        h = y + kernel_y - 1;
                    }
                    
                    if h > (dimensions.1) - 1 {
                        h = (dimensions.1) - 1;
                    }
                }
                
                let mut count_sub_x = kernel_half_size;
                for kernel_x in 0..image.k_size {
                    let mut w;
                    // WIDHT
                    if kernel_x < kernel_half_size {
                        let w_res = x.checked_sub(count_sub_x);
                        match w_res {
                            Some(point_w) => w = point_w,
                            None => w = 0,
                        }
                    } else {
                        if kernel_x == kernel_half_size {
                            w = x;
                        } else {
                            w = x + count_col;
                        }
                        
                        count_col += 1;
            
                        if w > (dimensions.0) - 1 {
                            w = (dimensions.0) - 1;
                        }
                    }
                    
                    let pixel = *old_img.get_pixel_mut(w, h);
        
                    somatory_r += pixel[0] as u32;
                    somatory_g += pixel[1] as u32;
                    somatory_b += pixel[2] as u32;
                    somatory_a += pixel[3] as u32;
        
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

            let r: u32 = somatory_r / divisor;
            let g: u32 = somatory_g / divisor;
            let b: u32 = somatory_b / divisor;
            let a: u32 = somatory_a / divisor;

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


#[test]
fn filters() { 
    // original 800 x 596
    let image = ProcessImageObj::from("/home/tiago/rust/projects/cli/imgs/chestnut_tailed_starling.jpg");
    // ! box - 21 n_size 588.619807ms
    let start = std::time::Instant::now();
    let result = func(&image);
    let _ = result.save("/home/tiago/rust/projects/cli/imgs/chestnut_tailed_starling_filter_box_fn.png");
    println!("{:?}", start.elapsed());
}
