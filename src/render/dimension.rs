
#[derive(Copy, Clone, Debug)]
pub struct Dimensions {
    pub width_pixels:u32,
    pub height_pixels:u32,
    pub scale: f32,
}

impl Dimensions {
    pub fn points(&self) -> (f32, f32) {
        (self.width_pixels as f32 / self.scale, self.height_pixels as f32 / self.scale)
    }
}

pub struct Pixels<S>(pub S);

pub struct Points<S>(pub S);