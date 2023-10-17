pub trait Parse {
    fn to_rgb_clamp(&self) -> (u8, u8, u8) ;
}

impl Parse for (u32, u32, u32) {
    fn to_rgb_clamp(&self) -> (u8, u8, u8) {
        (
            self.0.clamp(0, 255) as u8,
            self.1.clamp(0, 255) as u8,
            self.2.clamp(0, 255) as u8,
        )
    }
}

impl Parse for (i32, i32, i32) {
    fn to_rgb_clamp(&self) -> (u8, u8, u8) {
        (
            self.0.clamp(0, 255) as u8,
            self.1.clamp(0, 255) as u8,
            self.2.clamp(0, 255) as u8,
        )
    }
}

impl Parse for (f32, f32, f32) {
    fn to_rgb_clamp(&self) -> (u8, u8, u8) {
        (
            self.0.clamp(0.0, 255.0) as u8,
            self.1.clamp(0.0, 255.0) as u8,
            self.2.clamp(0.0, 255.0) as u8,
        )
    }
}