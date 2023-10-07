use image::{ImageBuffer, Rgba};

pub type ImageRgba = ImageBuffer<Rgba<u8>, Vec<u8>>;

#[derive(Debug, Clone)]
pub struct ProcessImageObj {
    pub path: String,
    pub gama: f64,
    pub scale: ScaleFactor,
}

impl From<&str> for ProcessImageObj{
    fn from(path: &str) -> Self {       
        ProcessImageObj {
            path: path.to_string(),
            gama: 1.0,
            scale: ScaleFactor { new_dim: (0,0), old_dim: (0,0) }
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
pub enum ReziseForm {
    Bilinear(),
    Bicubic(),
}

#[derive(Debug, Clone)]
pub struct ScaleFactor {
    pub new_dim:(u32, u32), 
    pub old_dim:(u32, u32),
}

impl ScaleFactor {
    pub fn set(&self) -> (u32,u32) {
        let mut w = self.old_dim.0;
        let mut h = self.old_dim.1;
        if self.new_dim.0 > self.old_dim.0 {
            w = self.new_dim.0 / self.old_dim.0;
        }
        else if self.new_dim.0 < self.old_dim.0 {
            w = self.old_dim.0 / self.new_dim.0;
        }
        if self.new_dim.1 > self.old_dim.1 {
            h = self.new_dim.1 / self.old_dim.1;
        }
        else if self.new_dim.1 < self.old_dim.1 {
            h = self.old_dim.1 / self.new_dim.1;
        }
        (w, h)
    }
}