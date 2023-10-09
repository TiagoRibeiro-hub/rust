// weighted average of the nearest pixels on the boundary of the 4-neighboring

use image::ImageBuffer;

use crate::process_img::{
    utils::{gray_scale_operation, open},
    ImageRgba, ProcessImageObj,
};

#[allow(unused_variables, dead_code)]
pub fn resize(mut image: ProcessImageObj) -> ImageRgba {
    let img = open(&image.path);
    let old_img = img.to_rgba8();

    // * Dimensions
    let new_dim = image.dimensions.new_dim;
    image.dimensions.old_dim = old_img.dimensions();
    let scale_factor = image.dimensions.scale_factor();

    let mut new_img: ImageBuffer<image::Rgba<u8>, Vec<u8>> = ImageBuffer::new(new_dim.0, new_dim.1);

    for y in 0..image.dimensions.new_dim.1 {
        for x in 0..image.dimensions.new_dim.0 {
            // * map the coordinates back to the original image
            let original_h = y as f64 * scale_factor.1;
            let original_w = x as f64 * scale_factor.0;

            // * calculate the coordinate values for 4 surrounding pixels.
            let h_floor = original_h.floor() as u32;
            let mut h_ceil = original_h.ceil() as u32;
            if h_ceil > (image.dimensions.old_dim.1) - 1 {
                h_ceil = (image.dimensions.old_dim.1) - 1
            }

            let w_floor = original_w.floor() as u32;
            let mut w_ceil = original_w.ceil() as u32;
            if w_ceil > (image.dimensions.old_dim.0) - 1 {
                w_ceil = (image.dimensions.old_dim.0) - 1
            }

            let q: u8;
            // * if original_h and original_w have integer values q will be always 0
            // * so we use the original pixel
            if (h_ceil == h_floor) && (w_ceil == w_floor) {
                let pix_l = old_img.get_pixel(w_floor, h_floor);
                q = gray_scale_operation(pix_l, 1.0);
            }  else if h_ceil == h_floor {
                // * if original_h have integer values we use linear interpolation
                // * get the 2 vertical neighbouring pixel values
                let pix_u = old_img.get_pixel(w_floor, original_h as u32); 
                let pix_d = old_img.get_pixel(w_ceil,original_h as u32);

                // * grayscale
                let mut q_one = gray_scale_operation(pix_u, 1.0);
                let mut q_two = gray_scale_operation(pix_d, 1.0);

                // * estimate the pixel value q using pixel values of neighbours
                q_one = q_one * (w_ceil as u32 - original_w as u32) as u8;
                q_two = q_two * (original_w as u32 - w_floor as u32) as u8;
                q = q_one + q_two;

            } else if w_ceil == w_floor {
                // * if original_w have integer values we use linear interpolation
                // * get the 2 horizontal neighbouring pixel values
                let pix_l = old_img.get_pixel(original_w as u32, h_floor); 
                let pix_r = old_img.get_pixel(original_w as u32, h_ceil);

                // * grayscale
                let mut q_one = gray_scale_operation(pix_l, 1.0);
                let mut q_two = gray_scale_operation(pix_r, 1.0);

                // * estimate the pixel value q using pixel values of neighbours
                q_one = q_one * (h_ceil as u32 - original_h as u32) as u8;
                q_two = q_two  *(original_h as u32 - h_floor as u32) as u8;
                q = q_one + q_two;
            } else {
                // * get the 4 neighbouring pixel values
                let pix_l = old_img.get_pixel(w_floor, h_floor);
                let pix_l_d = old_img.get_pixel(w_floor, h_ceil);
                let pix_r = old_img.get_pixel(w_ceil, h_floor);
                let pix_r_d = old_img.get_pixel(w_ceil, h_ceil);

                // * grayscale
                let v_l = gray_scale_operation(pix_l, 1.0) as f64;
                let v_l_d = gray_scale_operation(pix_l_d, 1.0) as f64;
                let v_r = gray_scale_operation(pix_r, 1.0) as f64;
                let v_r_d = gray_scale_operation(pix_r_d, 1.0) as f64;

                // * estimate the pixel value q using pixel values of neighbours
                let q_one =
                    v_l * (h_ceil as f64 - original_h) + v_l_d * (original_h - h_floor as f64);
                let q_two =
                    v_r * (h_ceil as f64 - original_h) + v_r_d * (original_h - h_floor as f64);
                q = (q_one * (w_ceil as f64 - original_w) + q_two * (original_w - w_floor as f64))
                    as u8;
            }

            // set pixel
            let pixel: &mut image::Rgba<u8> = new_img.get_pixel_mut(x, y);
            *pixel = image::Rgba([q, q, q, 255]);
        }
    }

    new_img
}

#[test]
fn bilinear() {
    // original 800 x 596
    let mut image =
        ProcessImageObj::from("/home/tiago/rust/projects/cli/imgs/chestnut_tailed_starling.jpg");
    // ! Up scaling
    let mut img_clone = image.clone();
    img_clone.dimensions.new_dim = (1000, 796);
    let result = resize(img_clone);
    let _ = result.save("/home/tiago/rust/projects/cli/imgs/resize_bilinear_up_scaling.png");
    // ! Down scaling
    image.dimensions.new_dim = (350, 296);
    let result = resize(image);
    let _ = result.save("/home/tiago/rust/projects/cli/imgs/resize_bilinear_down_scaling.png");
}
