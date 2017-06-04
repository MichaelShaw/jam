
use render::TextureRegion;
use image::{RgbaImage, Rgba};
use {load_file_contents};

use aphid::HashMap;

use std::path::{PathBuf, Path};

use std::io;

use std::ops::Range;

use rusttype::{FontCollection, Scale, Font, point};

use std::fmt;

#[derive(Debug, Clone)]
pub struct FontDirectory {
    pub path: PathBuf, 
}

impl FontDirectory {
    pub fn for_path(path:&str) -> FontDirectory {
        FontDirectory {
            path: PathBuf::from(path) // convert to absolute here?
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct FontDescription {
    pub family: String,
    pub pixel_size: u32, // what about, what code points do you want ... and what 
}   


#[derive(Debug)]
pub struct BitmapGlyph {
    pub texture_region: Option<TextureRegion>, // space doesnt have a texture region :-/ stupid space
    pub advance: i32, // I think advance should always be u32 ... right?!
}

pub struct LoadedBitmapFont {
    pub image: RgbaImage,
    pub font: BitmapFont,
}

impl fmt::Debug for LoadedBitmapFont {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LoadedBitmapFont {{ image: {:?} font: {:?} }}", self.image.dimensions(), self.font)
    }
}

pub struct BitmapFont {
    pub description: FontDescription,
    pub glyphs: HashMap<char, BitmapGlyph>,
    pub kerning: HashMap<(char, char), i32>,
}

impl fmt::Debug for BitmapFont {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BitmapFont {{ description: {:?} glyphs: {:?} kerning: {:?} }}", self.description, self.glyphs, self.kerning)
    }
}

#[derive(Debug)]
pub enum FontLoadError {
    CouldntLoadFile(PathBuf, io::Error),
    CouldntReadAsFont(PathBuf),
    TextureSizeTooSmall { requested: u32, required: Option<u32> },
    What,
}

const PADDING: i32 = 2;

pub fn min_square_texture_size(font: &Font, chars: &Vec<char>, pixel_size: u32) -> Option<u32> {
    let scale = Scale { x: pixel_size as f32, y: pixel_size as f32 };
    let v_metrics = font.v_metrics(scale);
    let offset = point(0.0, v_metrics.ascent);
    let pixel_height = scale.y.ceil() as i32;

    'sizes: for n in 8..14 { // exclusive, 256 -> 2048
        let texture_size : u32 = 2u32.pow(n);

        let mut at_x : i32 = PADDING;
        let mut at_y : i32 = PADDING;

        for &c in chars {
            if let Some(glyph) = font.glyph(c) {
                let scaled = glyph.scaled(scale);
                let positioned = scaled.positioned(offset);

                if let Some(bb) = positioned.pixel_bounding_box() {
                    if bb.width() + at_x > texture_size as i32 {
                        at_x = PADDING;
                        at_y += pixel_height + PADDING;
                    }


                    if at_y + pixel_height >= texture_size as i32 - PADDING {

                        continue 'sizes;
                    }

                    at_x -= bb.min.x;
                    at_x += (bb.max.x + PADDING) as i32;
                }

            }
        }

        return Some(texture_size)
    }

    None
}

pub fn build_font(full_path: &Path, font_description: &FontDescription, image_size: u32) -> Result<LoadedBitmapFont, FontLoadError> {
    let font_data = load_file_contents(&full_path).map_err(|io| FontLoadError::CouldntLoadFile(full_path.to_path_buf(), io))?;
    let collection = FontCollection::from_bytes(&font_data[..]);
    let font = collection.into_font().ok_or(FontLoadError::CouldntReadAsFont(full_path.to_path_buf()))?; // this is an option

    let scale = Scale { x: font_description.pixel_size as f32, y: font_description.pixel_size as f32 };
    let pixel_height = scale.y.ceil() as i32;

    // println!("pixel height {:?}", pixel_height);

    let v_metrics = font.v_metrics(scale);
    
    let offset = point(0.0, v_metrics.ascent);

    // let line_height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;

    let char_range : Range<u8> = (32)..(127); // from space to tilde
    let chars : Vec<char> = char_range.map(|n| n as char).collect();

    let mut img = RgbaImage::from_pixel(image_size, image_size, Rgba { data: [255,255,255,0] }); // transparent white

    // top left write location of the next glyph
    let mut write_x : i32 = PADDING;
    let mut write_y : i32 = PADDING;

    let mut glyphs : HashMap<char, BitmapGlyph> = HashMap::default();

    for &c in &chars {
        if let Some(glyph) = font.glyph(c) {
            let scaled = glyph.scaled(scale);
            let h_metrics = scaled.h_metrics();

            let advance = h_metrics.advance_width.ceil() as i32;

            let positioned = scaled.positioned(offset);

            if let Some(bb) = positioned.pixel_bounding_box() {
                // println!("x {:?} y {:?}", write_x, write_y);
                // println!("check stuff -> {:?} pixel height -> {:?}", bb.width() + write_x, pixel_height);
                if bb.width() + write_x > image_size as i32 {
                    write_x = PADDING;
                    write_y += pixel_height + PADDING;

                    // println!("new height {:?} max {:?} for image size {:?}", write_y, write_y + pixel_height, image_size);
                } 
                if write_y + pixel_height >= image_size as i32 - PADDING {
                    return Err(FontLoadError::TextureSizeTooSmall { requested: image_size, required: min_square_texture_size(&font, &chars, font_description.pixel_size) });
                }

                write_x -= bb.min.x;
                
                positioned.draw(|x, y, v| {
                    let c = (v * 255.0) as u8;
                    let x = (x as i32 + bb.min.x + write_x) as u32;
                    let y = (y as i32 + bb.min.y + write_y) as u32;
                    // img.put_pixel(x, y, Rgba { data: [c,c,c,255] });
                    img.put_pixel(x, y, Rgba { data: [255,255,255, c] });
                });

                // let bearing = bb.min.x as i32;
                let bitmap_glyph = BitmapGlyph {
                    texture_region: Some(TextureRegion {
                        u_min: (write_x + bb.min.x - 1) as u32,
                        u_max: (write_x + bb.max.x + 1) as u32,
                        v_min: image_size - (write_y + pixel_height) as u32,
                        v_max: image_size - (write_y) as u32,
                        texture_size: image_size,
                    }),
                    advance: advance,
                };
                
                glyphs.insert(c, bitmap_glyph);
                write_x += (bb.max.x + PADDING) as i32;
                // println!("{:?} ->  h_metrics are {:?} and bb is {:?} bearing {:?} advance {:?}", c, h_metrics, bb, bearing, advance);
            } else {
                let bitmap_glyph = BitmapGlyph {
                    texture_region: None,
                    advance: advance,
                };
                glyphs.insert(c, bitmap_glyph);
            }
        } else {
            println!("wtf, no glyph for '{:?}'", c);
        }
    }

    let mut kerning_map : HashMap<(char, char), i32> = HashMap::default();

    for &from in &chars {
        for &to in &chars {
            let kerning = font.pair_kerning(scale, from, to);
            let kerning_i : i32 = kerning.round() as i32;
            if kerning_i != 0 {
                kerning_map.insert((from,to), kerning_i);
            }
        }
    }

    Ok(LoadedBitmapFont {
        image: img,
        font: BitmapFont {
            description: font_description.clone(),
            glyphs: glyphs,
            kerning: kerning_map,
        }
    })
}