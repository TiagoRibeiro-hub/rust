use self::models::{ColorScale, ImageRgba, ResizeForm, Filter};

mod utils;
mod color_scale;
mod pixelate;
mod resize;
mod filter;
pub mod models;

#[derive(Debug, Clone)]
pub struct ProcessImageObj {
    pub path: String,
    pub gama: f64,
    pub dimensions: (u32, u32),
    pub k_size: u32
}

impl From<&str> for ProcessImageObj {
    fn from(path: &str) -> Self {
        ProcessImageObj {
            path: path.to_string(),
            gama: 1.0,
            dimensions: (0, 0),
            k_size: 21
        }
    }
}

impl ProcessImageObj {
    pub fn color_scale(&self, color: ColorScale) -> ImageRgba {
        match color {
            ColorScale::Gray() => {
                color_scale::gray(self)
            },
            ColorScale::Blue() => {
                color_scale::blue(self)
            },
            ColorScale::Green() => {
                color_scale::green(self)
            },
            ColorScale::Red() => {
                color_scale::red(self)
            },
                
        }
    }

    pub fn pixelate(&self) -> ImageRgba {
        pixelate::pixelate(self)
    }

    pub fn resize(&self, reziseform: ResizeForm) -> ImageRgba {
        match reziseform {
            ResizeForm::Bilinear() => {
                resize::bilinear::resize(self)
            },
            ResizeForm::Bicubic() => {
                resize::bicubic::resize(self)
            },
        }
    }

    pub fn filter(&self, filter: Filter) -> ImageRgba {
        match filter {
            Filter::Box() => {
                filter::r#box::func(self)
            },
        }
    }
}
