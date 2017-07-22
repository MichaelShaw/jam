#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Dimensions {
    pub pixels: (u32,u32),
    pub points: (u32,u32),
}

impl Dimensions {
    pub fn scale(&self) -> f64 {
        self.pixels.0 as f64 / self.points.0 as f64
    }
}


