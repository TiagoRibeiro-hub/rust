use image::Rgba;

pub mod r#box;


fn get_kernel_pixel(
    kernel_size: u32,
    kernel_half_size: u32,
    y: u32,
    x: u32,
    dimensions: (u32, u32),
    old_img: &mut image::ImageBuffer<Rgba<u8>, Vec<u8>>,
) -> (Vec<u32>, Vec<u32>, Vec<u32>, Vec<u32>) {
    let mut pix_r: Vec<u32> = Vec::new();
    let mut pix_g: Vec<u32> = Vec::new();
    let mut pix_b: Vec<u32> = Vec::new();
    let mut pix_a: Vec<u32> = Vec::new();

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
        for kernel_x in 0..kernel_size {
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

            pix_r.push(pixel[0] as u32);
            pix_g.push(pixel[1] as u32);
            pix_b.push(pixel[2] as u32);
            pix_a.push(pixel[3] as u32);

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