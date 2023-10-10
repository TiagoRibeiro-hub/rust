use image::{ImageBuffer, Rgba};

use crate::process_img::{
    utils::open,
    ImageRgba, ProcessImageObj,
};

struct Linear {
    pub p1: Rgba<u8>,
    pub p2: Rgba<u8>,
    pub n: f64,
    pub n1: u32,
    pub n2: u32,
}

impl Linear {
    fn rgba(&self) -> [u8; 4] {
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
        let q1 = self.p1[index] * (self.n2 as u32 - self.n as u32) as u8;
        // * Q2 = P2 * (n - n1)
        let q2 = self.p2[index] * (self.n as u32 - self.n1 as u32) as u8;
        // * SQ = P1 + P2
        q1 + q2
    }
}

struct Bilinear {
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
    fn rgba(&self) -> [u8; 4] {
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
        // * Stotage Row major => HeightWidht
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
            let original_y = y as f64 * scale_factor.1;
            let original_x = x as f64 * scale_factor.0;

            // * calculate the coordinate values for 4 surrounding pixels.
            let y1 = original_y.floor() as u32;
            let mut y2 = original_y.ceil() as u32;
            if y2 > (image.dimensions.old_dim.1) - 1 {
                y2 = (image.dimensions.old_dim.1) - 1
            }

            let x1 = original_x.floor() as u32;
            let mut x2 = original_x.ceil() as u32;
            if x2 > (image.dimensions.old_dim.0) - 1 {
                x2 = (image.dimensions.old_dim.0) - 1
            }

            // set pixel
            let pixel: &mut image::Rgba<u8> = new_img.get_pixel_mut(x, y);
            // * if original_h and original_w have integer values q will be always 0
            // * so we use the original pixel
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
    let mut img_clone = image.clone();
    img_clone.dimensions.new_dim = (1000, 796);
    let result = resize(img_clone);
    let _ = result.save("/home/tiago/rust/projects/cli/imgs/resize_bilinear_up_scaling.png");
    // ! Down scaling
    image.dimensions.new_dim = (350, 296);
    let result = resize(image);
    let _ = result.save("/home/tiago/rust/projects/cli/imgs/resize_bilinear_down_scaling.png");
}
