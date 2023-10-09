use image::{ImageBuffer, Rgba};

use crate::process_img::{
    utils::{gray_scale_operation, open},
    ImageRgba, ProcessImageObj,
};

struct Linear {
    pub pix_one: Rgba<u8>,
    pub pix_two: Rgba<u8>,
    pub original_value: f64,
    pub floor_value: u32,
    pub ceil_value: u32,
}

impl Linear {
    fn get(&self) -> [u8; 4] {
        // * red
        let r = self.linear_func(0);
        // * green
        let g = self.linear_func(1);
        // * blue
        let b = self.linear_func(2);
        // * alpha
        let a = self.linear_func(3);
        [r, g, b, a]
    }

    fn linear_func(&self, index: usize) -> u8 {
        let q_one = self.pix_one[index] * (self.ceil_value as u32 - self.original_value as u32) as u8;
        let q_two = self.pix_two[index] * (self.original_value as u32 - self.floor_value as u32) as u8;
        q_one + q_two
    }
}

struct Bilinear {
    pub pix_l: Rgba<u8>,
    pub pix_l_d: Rgba<u8>,
    pub pix_r: Rgba<u8>,
    pub pix_r_d: Rgba<u8>,
    pub height: f64,
    pub h_floor: u32,
    pub h_ceil: u32,
    pub width: f64,
    pub w_floor: u32,
    pub w_ceil: u32,
}

impl Bilinear {
    fn get(&self) -> [u8; 4] {
        // * red
        let r = self.bilinear_func(0);
        // * green
        let g = self.bilinear_func(1);
        // * blue
        let b = self.bilinear_func(2);
        // * alpha
        let a = self.bilinear_func(3);
        [r, g, b, a]
    }

    fn bilinear_func(&self, index: usize) -> u8 {
        let q_one = self.pix_l[index] as f64 * (self.h_ceil as f64 - self.height)
            + self.pix_l_d[index] as f64 * (self.height - self.h_floor as f64);
        let q_two = self.pix_r[index] as f64 * (self.h_ceil as f64 - self.height)
            + self.pix_r_d[index] as f64 * (self.height - self.h_floor as f64);
        (q_one * (self.w_ceil as f64 - self.width)
            + q_two * (self.width - self.w_floor as f64)) as u8
    }
}

#[allow(unused_variables, dead_code)]
pub fn resize(mut image: ProcessImageObj) -> ImageRgba {
    let img = open(&image.path);
    let old_img = img.to_rgba8();

    // * Dimensions
    let new_dim = image.dimensions.new_dim;
    image.dimensions.old_dim = old_img.dimensions();
    let scale_factor = image.dimensions.scale_factor();

    let mut new_img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(new_dim.0, new_dim.1);

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

            // set pixel
            let pixel: &mut image::Rgba<u8> = new_img.get_pixel_mut(x, y);
            // * if original_h and original_w have integer values q will be always 0
            // * so we use the original pixel
            if (h_ceil == h_floor) && (w_ceil == w_floor) {
                *pixel = *old_img.get_pixel(w_floor, h_floor);
            } else if h_ceil == h_floor {
                // * if original_h have integer values we use linear interpolation
                let linear = Linear {
                    pix_one: *old_img.get_pixel(w_floor, original_h as u32),
                    pix_two: *old_img.get_pixel(w_ceil, original_h as u32),
                    original_value: original_w,
                    floor_value: w_floor,
                    ceil_value: w_ceil,
                };
                *pixel = image::Rgba(linear.get());

            } else if w_ceil == w_floor {
                // * if original_w have integer values we use linear interpolation
                let linear = Linear {
                    pix_one: *old_img.get_pixel(original_w as u32, h_floor),
                    pix_two: *old_img.get_pixel(original_w as u32, h_ceil),
                    original_value: original_h,
                    floor_value: h_floor,
                    ceil_value: h_ceil,
                };
                *pixel = image::Rgba(linear.get());
            } else {
                let bilinear = Bilinear {
                    pix_l: *old_img.get_pixel(w_floor, h_floor),
                    pix_l_d: *old_img.get_pixel(w_floor, h_ceil),
                    pix_r: *old_img.get_pixel(w_ceil, h_floor),
                    pix_r_d: *old_img.get_pixel(w_ceil, h_ceil),
                    height: original_h,
                    h_floor,
                    h_ceil,
                    width: original_w,
                    w_floor,
                    w_ceil,
                };
                *pixel = image::Rgba(bilinear.get());
            }
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
