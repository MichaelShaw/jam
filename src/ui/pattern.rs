
use image::RgbaImage;
use super::{ColourSource, RectI};

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub enum Pattern {
    All,
    Border(BorderMask),
}

pub trait Mask { // or "pattern"
    fn pull(&self, source: Box<&ColourSource>, image: &mut RgbaImage);
}

pub fn all_pull(source: Box<&ColourSource>, image: &mut RgbaImage) {
    let width = image.width();
    let height = image.height();
    for x in 0..width {
        for y in 0..height {
            image.put_pixel(x, y, source.get(x as i32, y as i32));
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub struct BorderMask {
    pub thickness: u32,
}

impl Mask for BorderMask {
    fn pull(&self, source: Box<&ColourSource>, image: &mut RgbaImage) {
        let width = image.width();
        let height = image.height();

        for t in 0..self.thickness {
            let top_y = height - t - 1;
            for x in 0..width {
                image.put_pixel(x, t, source.get(x as i32, t as i32));
                image.put_pixel(x, top_y, source.get(x as i32, top_y as i32));
            }

            let right_x = width - t - 1;
            for y in 0..height {
                image.put_pixel(t, y, source.get(t as i32, y as i32));
                image.put_pixel(right_x, y, source.get(right_x as i32, y as i32));
            }
        }
    }
}