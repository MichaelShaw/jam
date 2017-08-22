
pub mod text;

pub use self::text::*;

use super::{Element, ColourSource, Source, Pattern, all_pull};
use {OurFont, as_rgba8};
use cgmath::{Vector2, vec2};
use image::{RgbaImage, Rgba};
use rusttype::{FontCollection, Scale, point, PositionedGlyph};
use ui::pattern::Mask;
use std::cmp::max;

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

//    println!("raster image size {:?} x {:?}", width, height);

    for px in img.pixels_mut() {
        *px = Rgba { data: [0, 0, 0, 0] };
    }

    match element {
        &Element::Draw(ref pattern, ref source) => {
            let s : Box<&ColourSource> = match source {
                &Source::ConstantColour(ref cc) => Box::new(cc),
            };
            match pattern {
                &Pattern::All => all_pull(s, &mut img),
                &Pattern::Border(ref b) => b.pull(s, &mut img),
            }
        },
        &Element::Text(ref text) => {
            let color = as_rgba8(text.color);
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

//                println!("text \"{}\" text raster {:?} x {:?}", text.characters, width, pixel_height);

                for g in glyphs {
//                    println!("glyph pos -> {:?} bb -> {:?}", g.position(), g.pixel_bounding_box());
                    if let Some(bb) = g.pixel_bounding_box() {
                        g.draw(|x, y, v| {
                            let x = x as i32 + bb.min.x;
                            let y = y as i32 + bb.min.y;

                            let tx = x as u32;
                            let ty = (height - 1 - y) as u32;

                            if tx < 0 || tx > width as u32 || ty < 0 || ty > height as u32 {
                                println!("starting x y was {:?} {:?}", x, y);
                                println!("text attempting to write to invalid location ({:?}, {:?}) but space is {:?} x {:?}", tx, ty, width, height);
                            } else {
                                let source = *img.get_pixel(tx, ty);

                                let c_alpha = (255.0 * v) as u8;
                                let new_alpha = max(c_alpha, source.data[3]);

                                let mut c = color;
                                c.data[3] = new_alpha;

                                img.put_pixel(tx, ty, c);
                            }
                        });
                    }
                }
            }
        },
        &Element::Image(ref img) => {
//            println!("no image element support yet");
        },
    }

    (img, vec2(0, 0)) // no fancy sizing for now
}
