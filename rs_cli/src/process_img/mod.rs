use self::models::{ColorsProcesses, ImageRgba, ResizeForm, Filter};

mod utils;
mod colors;
mod pixelate;
mod resize;
mod filter;
pub mod models;

#[derive(Debug, Clone)]
pub struct ProcessImageObj {
    pub path: String,
    pub gama: u8,
    pub dimensions: (u32, u32),
    pub k_size: u32
}

impl From<&str> for ProcessImageObj {
    fn from(path: &str) -> Self {
        ProcessImageObj {
            path: path.to_string(),
            gama: 0,
            dimensions: (0, 0),
            k_size: 21
        }
    }
}

impl ProcessImageObj {
    pub fn color_scale(&self, color: ColorsProcesses) -> ImageRgba {
        match color {
            ColorsProcesses::Gray() => {
                colors::gray(self)
            },
            ColorsProcesses::Blue() => {
                colors::blue(self)
            },
            ColorsProcesses::Green() => {
                colors::green(self)
            },
            ColorsProcesses::Red() => {
                colors::red(self)
            },
            ColorsProcesses::Darken() => {
                colors::darken(self)
            },
            ColorsProcesses::Lighten() => {
                colors::lighten(self)
            },
            ColorsProcesses::Invert() => {
                colors::invert(self)
            },
            ColorsProcesses::LowContrast() => {
                colors::low_contrast(self)
            },
            ColorsProcesses::HighContrast() => {
                colors::high_contrast(self)
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
