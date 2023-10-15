pub mod r#box;
pub mod median;

// fuzzy filter
// bilateral filter

fn get_kernel_x(
    kernel_x: u32,
    kernel_half_size: u32,
    x: u32,
    count_sub_x: u32,
    count_col: &mut u32,
    dimensions: (u32, u32),
) -> u32 {
    let mut w: u32;
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
            w = x + *count_col;
        }

        *count_col += 1;

        if w > (dimensions.0) - 1 {
            w = (dimensions.0) - 1;
        }
    }
    w
}

fn get_kernel_y(
    kernel_y: u32,
    kernel_half_size: u32,
    y: u32,
    count_sub_y: u32,
    dimensions: (u32, u32),
) -> u32 {
    let mut h = 0;
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
    h
}
