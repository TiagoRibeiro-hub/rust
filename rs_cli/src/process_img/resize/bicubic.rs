use image::Rgba;

use crate::process_img::{ImageRgba, ProcessImageObj, utils::set_props_for_resize_to_rgba8};

struct Bicubic {
    // 1st row
    q00: Rgba<u8>,
    q10: Rgba<u8>,
    q20: Rgba<u8>,
    q30: Rgba<u8>,
    // 2nd row
    q01: Rgba<u8>,
    q11: Rgba<u8>,
    q21: Rgba<u8>,
    q31: Rgba<u8>,
    // 3th row
    q02: Rgba<u8>,
    q12: Rgba<u8>,
    q22: Rgba<u8>,
    q32: Rgba<u8>,
    // 4th row
    q03: Rgba<u8>,
    q13: Rgba<u8>,
    q23: Rgba<u8>,
    q33: Rgba<u8>,
    fract_y: f64,
    fract_x: f64,
}

impl Bicubic {
    pub fn rgba(&self) -> [u8; 4] {
        let mut rgba: [u8; 4] = [0; 4];
        for i in 0..4 {
            let col0 = Bicubic::bicubic_func(
                self.q00[i] as f64,
                self.q10[i] as f64,
                self.q20[i] as f64,
                self.q30[i] as f64,
                self.fract_x,
            );
            let col1 = Bicubic::bicubic_func(
                self.q01[i] as f64,
                self.q11[i] as f64,
                self.q21[i] as f64,
                self.q31[i] as f64,
                self.fract_x,
            );
            let col2 = Bicubic::bicubic_func(
                self.q02[i] as f64,
                self.q12[i] as f64,
                self.q22[i] as f64,
                self.q32[i] as f64,
                self.fract_x,
            );
            let col3 = Bicubic::bicubic_func(
                self.q03[i] as f64,
                self.q13[i] as f64,
                self.q23[i] as f64,
                self.q33[i] as f64,
                self.fract_x,
            );
            let color = Bicubic::bicubic_func(col0, col1, col2, col3, self.fract_y);
            rgba[i] = color.clamp(0.0, 255.0) as u8;
        }
        rgba
    }

    fn bicubic_func(p0: f64, p1: f64, p2: f64, p3: f64, t: f64) -> f64 {
        // a =  -frac{1}{2}p0  + frac{3}{2}p1     - frac{3}{2}p2     + frac{1}{2}p3
        let a = -p0 / 2.0 + (3.0 * p1) / 2.0 - (3.0 * p2) / 2.0 + p3 / 2.0;
        // b =       p0 - frac{5}{2}p1     + 2p  * 2  - frac{1}{2}p3
        let b = p0 - (5.0 * p1) / 2.0 + 2.0 * p2 - p3 / 2.0;
        // c =  -frac{1}{2}p0  +  frac{1}{2}p2
        let c = -p0 / 2.0 + p2 / 2.0;
        // d = p1
        let d = p1;

        //  (a)x^3  + (b)x^2 + (c)x + d
        a * t.powf(3.0) + b * t.powf(2.0) + c * t + d
    }
}

#[allow(unused_variables, dead_code, unused_assignments)]
pub fn resize(image: &ProcessImageObj) -> ImageRgba {
    let (old_img, dimensions, scale_factor, mut new_img) = set_props_for_resize_to_rgba8(image);

    for y in 0..dimensions.new_dim.1 {
        for x in 0..dimensions.new_dim.0 {
            // * map the coordinates back to the original image, also need to offset by half a pixel to keep image from shifting down and left half a pixel
            let (original_y, original_x) = dimensions.map_original_coordinates(y, x, scale_factor);

            // * calculate the coordinate values for 8 surrounding pixels.
            let (y1, y2, x1, x2) = dimensions.map_surrounding_coordinates(original_y, original_x);

            let (y0, y3) = dimensions.map_edge_coordinates(y1, y2, (dimensions.old_dim.1) - 1);
            let (x0, x3) = dimensions.map_edge_coordinates(x1, x2, (dimensions.old_dim.0) - 1);

            let fract_y = original_y - original_y.floor();
            let fract_x = original_x - original_x.floor();

            let pixel: &mut image::Rgba<u8> = new_img.get_pixel_mut(x, y);

            let bicubic = Bicubic {
                q00: *old_img.get_pixel(x0, y0),
                q10: *old_img.get_pixel(x1, y0),
                q20: *old_img.get_pixel(x2, y0),
                q30: *old_img.get_pixel(x3, y0),
                q01: *old_img.get_pixel(x0, y1),
                q11: *old_img.get_pixel(x1, y1),
                q21: *old_img.get_pixel(x2, y1),
                q31: *old_img.get_pixel(x3, y1),
                q02: *old_img.get_pixel(x0, y2),
                q12: *old_img.get_pixel(x1, y2),
                q22: *old_img.get_pixel(x2, y2),
                q32: *old_img.get_pixel(x3, y2),
                q03: *old_img.get_pixel(x0, y3),
                q13: *old_img.get_pixel(x1, y3),
                q23: *old_img.get_pixel(x2, y3),
                q33: *old_img.get_pixel(x3, y3),
                fract_y,
                fract_x,
            };
            *pixel = Rgba(bicubic.rgba());
        }
    }
    new_img
}





#[test]
fn bicubic() {
    // original 800 x 596
    let mut image =
        ProcessImageObj::from("/home/tiago/rust/projects/cli/imgs/chestnut_tailed_starling.jpg");
    // ! Up scaling
    image.dimensions = (1000, 796);
    let result = resize(&image);
    let _ = result.save("/home/tiago/rust/projects/cli/imgs/resize_bicubic_up_scaling.png");
    // ! Down scaling
    image.dimensions = (350, 296);
    let result = resize(&image);
    let _ = result.save("/home/tiago/rust/projects/cli/imgs/resize_bicubic_down_scaling.png");
}
