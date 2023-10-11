use image::{ImageBuffer, Rgba};

pub type ImageRgba = ImageBuffer<Rgba<u8>, Vec<u8>>;

#[derive(Debug, Clone)]
pub struct ProcessImageObj {
    pub path: String,
    pub gama: f64,
    pub dimensions: ImgDimensions,
}

impl From<&str> for ProcessImageObj{
    fn from(path: &str) -> Self {       
        ProcessImageObj {
            path: path.to_string(),
            gama: 1.0,
            dimensions: ImgDimensions { new_dim: (0,0), old_dim: (0,0) }
        }
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

#[derive(Debug, Clone)]
pub struct ImgDimensions {
    pub new_dim:(u32, u32), 
    pub old_dim:(u32, u32),
}

impl ImgDimensions {
    pub fn scale_factor(&self) -> (f64,f64) {
        let mut w = self.old_dim.0 as f64 / self.new_dim.0 as f64 ;
        let mut h = self.old_dim.1 as f64 / self.new_dim.1 as f64 ;
        if w < 0.0 {
            w = 0.0;
        }
        if h < 0.0 {
            h = 0.0;
        }
        (w, h)
    }
}