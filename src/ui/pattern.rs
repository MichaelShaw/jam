
use image::RgbaImage;
use super::{ColourSource, RectI};

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub enum Pattern {
    Rect(RectMask),
    Border(BorderMask),
}

pub trait Mask { // or "pattern"
    fn pull(&self, source: Box<&ColourSource>, image: &mut RgbaImage);
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub struct RectMask {
    pub rect: RectI,
}

impl Mask for RectMask {
    fn pull(&self, source: Box<&ColourSource>, image: &mut RgbaImage) {
        for x in self.rect.min.x..self.rect.max.x {
            for y in self.rect.min.y..self.rect.max.y {
                image.put_pixel(x as u32, y as u32, source.get(x, y));
            }
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub struct BorderMask {
    pub rect: RectI,
    pub thickness: u32,
}

impl Mask for BorderMask {
    fn pull(&self, source: Box<&ColourSource>, image: &mut RgbaImage) {
        let width = image.width();
        let height = image.height();
        for x in 0..width {
            for y in 0..height {
                image.put_pixel(x, y, source.get(x as i32, y as i32));
            }
        }
    }
}