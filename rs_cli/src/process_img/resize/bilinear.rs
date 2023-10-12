use image::Rgba;

use crate::process_img::{
    ImageRgba, ProcessImageObj, models::ImgDimensions,
};

pub struct Linear {
    pub p1: Rgba<u8>,
    pub p2: Rgba<u8>,
    pub n: f64,
    pub n1: u32,
    pub n2: u32,
}

impl Linear {
    pub fn rgba(&self) -> [u8; 4] {
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
        // * Q1 = P1 * (n2 - n)
        let q1 = self.p1[index] * (self.n2 - self.n as u32) as u8;
        // * Q2 = P2 * (n - n1)
        let q2 = self.p2[index] * (self.n as u32 - self.n1) as u8;
        // * SQ = P1 + P2
        q1 + q2
    }
}

pub struct Bilinear {
    pub q11: Rgba<u8>,
    pub q12: Rgba<u8>,
    pub q21: Rgba<u8>,
    pub q22: Rgba<u8>,
    pub y: f64,
    pub y1: u32,
    pub y2: u32,
    pub x: f64,
    pub x1: u32,
    pub x2: u32,
}

impl Bilinear {
    pub fn rgba(&self) -> [u8; 4] {
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
        // * Row major => HeightWidht
        // * Q1 = Q11 * (y2 - y) + Q12 * (y1 -y)
        let q1 = self.q11[index] as f64 * (self.y2 as f64 - self.y)
            + self.q12[index] as f64 * (self.y - self.y1 as f64);
        // * Q2 = Q22 * (y2 - y) + Q22 * (y - y1)
        let q2 = self.q21[index] as f64 * (self.y2 as f64 - self.y)
            + self.q22[index] as f64 * (self.y - self.y1 as f64);
        // * Q = Q1 * (x2 - x) + Q2 * (x - x1)
        (q1 * (self.x2 as f64 - self.x)
            + q2 * (self.x - self.x1 as f64)) as u8
    }
}

#[allow(unused_variables, dead_code)]
pub fn resize(image: &ProcessImageObj) -> ImageRgba {
    let (old_img, dimensions, scale_factor, mut new_img) = image.set_props_for_processing();

    for y in 0..dimensions.new_dim.1 {
        for x in 0..dimensions.new_dim.0 {
            // * map the coordinates back to the original image, also need to offset by half a pixel to keep image from shifting down and left half a pixel
            let (original_y, original_x) = ImgDimensions::map_original_coordinates(y, x, scale_factor);

            // * calculate the coordinate values for 4 surrounding pixels.
            let (y1, y2, x1, x2) = dimensions.map_surrounding_coordinates(original_y, original_x);

            // set pixel
            let pixel: &mut image::Rgba<u8> = new_img.get_pixel_mut(x, y);
            // * if original_h and original_w have integer values q will be always 0, so we use the original pixel
            if (y2 == y1) && (x2 == x1) {
                *pixel = *old_img.get_pixel(x1, y1);
            } else if y2 == y1 {
                // * if original_h have integer values we use linear interpolation
                let linear = Linear {
                    p1: *old_img.get_pixel(x1, original_y as u32),
                    p2: *old_img.get_pixel(x2, original_y as u32),
                    n: original_x,
                    n1: x1,
                    n2: x2,
                };
                *pixel = image::Rgba(linear.rgba());

            } else if x2 == x1 {
                // * if original_w have integer values we use linear interpolation
                let linear = Linear {
                    p1: *old_img.get_pixel(original_x as u32, y1),
                    p2: *old_img.get_pixel(original_x as u32, y2),
                    n: original_y,
                    n1: y1,
                    n2: y2,
                };
                *pixel = image::Rgba(linear.rgba());
            } else {
                let bilinear = Bilinear {
                    q11: *old_img.get_pixel(x1, y1),
                    q12: *old_img.get_pixel(x1, y2),
                    q21: *old_img.get_pixel(x2, y1),
                    q22: *old_img.get_pixel(x2, y2),
                    y: original_y,
                    y1,
                    y2,
                    x: original_x,
                    x1,
                    x2,
                };
                *pixel = image::Rgba(bilinear.rgba());
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
    image.dimensions = (1000, 796);
    let result = resize(&image);
    let _ = result.save("/home/tiago/rust/projects/cli/imgs/resize_bilinear_up_scaling.png");
    // ! Down scaling
    image.dimensions = (350, 296);
    let result = resize(&image);
    let _ = result.save("/home/tiago/rust/projects/cli/imgs/resize_bilinear_down_scaling.png");
}
