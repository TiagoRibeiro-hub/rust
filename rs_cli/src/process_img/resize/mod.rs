use std::cmp;

pub mod bilinear;
pub mod bicubic;

#[derive(Debug)]
pub struct ImgDimensions {
    pub new_dim: (u32, u32),
    pub old_dim: (u32, u32),
}

impl ImgDimensions {
    pub fn scale_factor(&self) -> (f64, f64) {
        let mut w = self.old_dim.0 as f64 / self.new_dim.0 as f64;
        let mut h = self.old_dim.1 as f64 / self.new_dim.1 as f64;
        if w < 0.0 {
            w = 0.0;
        }
        if h < 0.0 {
            h = 0.0;
        }
        (w, h)
    }

    pub fn map_surrounding_coordinates(&self, original_y: f64, original_x: f64) -> (u32, u32, u32, u32) {
        let y1 = original_y.floor() as u32;
        let y2 = cmp::min(original_y.ceil() as u32, (self.old_dim.1) - 1);
    
        let x1 = original_x.floor() as u32;
        let x2 = cmp::min(original_x.ceil() as u32, (self.old_dim.0) - 1);
        (y1, y2, x1, x2)
    }

    pub fn map_edge_coordinates(&self, v1: u32, v2: u32, min_value: u32) -> (u32, u32) {
        let mut v0 = 0;
        if v1 >= 1 {
            v0 = v1 - 1;
        }
        let v3 = cmp::min(v2 + 1, min_value);
        (v0, v3)
    }

    pub fn map_original_coordinates(&self, y: u32, x: u32, scale_factor: (f64, f64)) -> (f64, f64) {
        let original_y = y as f64 * scale_factor.1 - 0.5;
        let original_x = x as f64 * scale_factor.0 - 0.5;
        (original_y, original_x)
    }
}
