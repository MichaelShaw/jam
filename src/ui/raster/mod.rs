
pub mod text;

pub use self::text::*;

use super::Element;
use OurFont;
use cgmath::{Vector2, vec2};
use image::{RgbaImage, Rgba};
use rusttype::{FontCollection, Scale, point, PositionedGlyph};

pub fn raster(element:&Element, size: Vector2<i32>, fonts: &[OurFont]) -> (RgbaImage, Vector2<i32>) { // secon
    let mut img = RgbaImage::new(size.x as u32, size.y as u32);
    let width = size.x;
    let height = size.y;

    /*
    pub struct Text {
    pub characters : String,
    pub color: Color,
    pub size: i32,
    pub horizontal_alignment: HorizontalAlignment,
    pub vertical_alignment: VerticalAlignment,
}*/

    for px in img.pixels_mut() {
        *px = Rgba { data: [255, 255, 255, 255] };
    }

    match element {
        &Element::Text(ref text) => {
            let color = text.color;
            if let Some(font) = fonts.first() {
                let f = &font.font;
                let scale = Scale { x: text.size as f32, y: text.size as f32 };
                let pixel_height = scale.y.ceil() as i32;
                let v_metrics = f.v_metrics(scale);
                let offset = point(0.0, v_metrics.ascent);

                let glyphs: Vec<PositionedGlyph> = f.layout(&text.characters, scale, offset).collect();
                let width = glyphs.iter().rev()
                    .filter_map(|g| g.pixel_bounding_box()
                        .map(|b| b.min.x as f32 + g.unpositioned().h_metrics().advance_width))
                    .next().unwrap_or(0.0).ceil() as usize;

                println!("text raster width: {}, height: {}", width, pixel_height);
                for g in glyphs {
                    if let Some(bb) = g.pixel_bounding_box() {
                        g.draw(|x, y, v| {
                            let x = x as i32 + bb.min.x;
                            let y = y as i32 + bb.min.y;

                            // height - 5 -
//                            println!("write @ {:?} {:?}", x, y);

                            let pixel_color = color.with_alpha(v);
                            img.put_pixel(x as u32, y as u32, Rgba { data: pixel_color.raw() });
                        });
                    }
                }
            }
        },
        &Element::Image(ref img) => {

        },
    }



    (img, vec2(0, 0))
}


