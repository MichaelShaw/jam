use ui::RectI;
use cgmath::vec2;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Dimensions {
    pub pixels: (u32,u32),
    pub points: (u32,u32),
}

impl Dimensions {
    pub fn scale(&self) -> f64 {
        self.pixels.0 as f64 / self.points.0 as f64
    }

    pub fn pixels_rect(&self) -> RectI {
        let (w, h) = self.pixels;
        RectI {
            min: vec2(0, 0),
            max: vec2(w as i32, h as i32)
        }
    }
    pub fn points_rect(&self) -> RectI {
        let (w, h) = self.points;
        RectI {
            min: vec2(0, 0),
            max: vec2(w as i32, h as i32)
        }
    }
}


