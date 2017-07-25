use super::Element;
use cgmath::{Vector2, vec2};
use image::{RgbaImage, Rgba};

pub fn raster(element:&Element, size: Vector2<i32>) -> (RgbaImage, Vector2<i32>) { // secon
    let mut img = RgbaImage::new(size.x as u32, size.y as u32);

    for px in img.pixels_mut() {
        *px = Rgba { data: [255, 255, 0, 255] };
    }

    (img, vec2(0, 0))
}


