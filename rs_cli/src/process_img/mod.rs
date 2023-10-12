use self::models::*;

mod utils;
mod color_scale;
mod pixelate;
mod resize;
mod filter;
pub mod models;

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

    pub fn filter(&self) -> ImageRgba {
        filter::blur::gaussian(self)
    }
}
