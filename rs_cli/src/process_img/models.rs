use std::cmp;

use image::{ImageBuffer, Rgba};

use super::utils::open;

pub type ImageRgba = ImageBuffer<Rgba<u8>, Vec<u8>>;

#[derive(Debug, Clone)]
pub struct ProcessImageObj {
    pub path: String,
    pub gama: f64,
    pub dimensions: (u32, u32),
}

impl From<&str> for ProcessImageObj {
    fn from(path: &str) -> Self {
        ProcessImageObj {
            path: path.to_string(),
            gama: 1.0,
            dimensions: (0, 0),
        }
    }
}

impl ProcessImageObj {
    pub fn set_props_for_processing(&self) -> (
        ImageBuffer<Rgba<u8>, Vec<u8>>,
        ImgDimensions,
        (f64, f64),
        ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) {
        let img = open(&self.path);
        let old_img = img.to_rgba8();

        // * Dimensions
        let dimensions = ImgDimensions {
            new_dim: self.dimensions,
            old_dim: old_img.dimensions(),
        };
        let scale_factor = dimensions.scale_factor();

        let new_img: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::new(dimensions.new_dim.0, dimensions.new_dim.1);
        (old_img, dimensions, scale_factor, new_img)
    }
}
pub enum ColorScale {
    Gray(),
    Blue(),
    Green(),
    Red(),
}
impl ColorScale {
    pub fn defaut() -> ColorScale {
        ColorScale::Gray()
    }
}
pub enum ResizeForm {
    Bilinear(),
    Bicubic(),
}

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

    pub fn map_original_coordinates(y: u32, x: u32, scale_factor: (f64, f64)) -> (f64, f64) {
        let original_y = y as f64 * scale_factor.1 - 0.5;
        let original_x = x as f64 * scale_factor.0 - 0.5;
        (original_y, original_x)
    }
}
