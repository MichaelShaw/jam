
use render::TextureRegion;
use image::{RgbaImage, Rgba};
use {HashMap, load_file_contents};

use std::path::PathBuf;

use std::io;

use std::ops::Range;

use rusttype::{FontCollection, Scale, point};

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
	pub point_size: u32, // what about, what code points do you want ... and what 
}	


#[derive(Debug)]
pub struct BitmapGlyph {
	pub texture_region: TextureRegion,
	pub advance: i32, // I think advance should always be u32 ... right?!
}

pub struct LoadedBitmapFont {
	pub image: RgbaImage,
	pub font: BitmapFont,
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
	What,
}


pub fn build_font(resource_path: &str, font_description: &FontDescription, image_size: u32) -> Result<LoadedBitmapFont, FontLoadError> {
    let full_path = PathBuf::from(format!("{}/{}.{}", resource_path, font_description.family, "ttf"));
    // println!("full_path -> {:?}", full_path);
    let font_data = load_file_contents(&full_path).map_err(|io| FontLoadError::CouldntLoadFile(full_path.clone(), io))?;
	let collection = FontCollection::from_bytes(&font_data[..]);
	let font = collection.into_font().ok_or(FontLoadError::CouldntReadAsFont(full_path.clone()))?; // this is an option



	let scale = Scale { x: font_description.point_size as f32, y: font_description.point_size as f32 };
    let pixel_height = scale.y.ceil() as i32;

    // println!("pixel height {:?}", pixel_height);

    let v_metrics = font.v_metrics(scale);
    
    let offset = point(0.0, v_metrics.ascent);

    let line_height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;

    // println!("line height -> {:?}", line_height);

    let char_range : Range<u8> = (32)..(127); // from space to tilde
    let chars : Vec<char> = char_range.map(|n| n as char).collect();

    // println!("mah chars -> {:?}", chars);

	let padding : i32 = 2; // to avoid texture bleeding

	let mut img = RgbaImage::from_pixel(image_size, image_size, Rgba { data: [255,255,255,0] }); // transparent white

	// top left write location of the next glyph
	let mut write_x : i32 = padding;
    let mut write_y : i32 = padding;

    let mut glyphs : HashMap<char, BitmapGlyph> = HashMap::default();

	for &c in &chars {
		if let Some(glyph) = font.glyph(c) {
			let scaled = glyph.scaled(scale);
			let h_metrics = scaled.h_metrics();

			let positioned = scaled.positioned(offset);

			if let Some(bb) = positioned.pixel_bounding_box() {
				if bb.width() + write_x > image_size as i32 {
					write_x = padding;
					write_y += pixel_height + padding;
				}

				write_x -= bb.min.x;
	    		
	    		positioned.draw(|x, y, v| {
	                let c = (v * 255.0) as u8;
	                let x = (x as i32 + bb.min.x + write_x) as u32;
	                let y = (y as i32 + bb.min.y + write_y) as u32;
	                // img.put_pixel(x, y, Rgba { data: [c,c,c,255] });
	                img.put_pixel(x, y, Rgba { data: [255,255,255, c] });
	            });

	            // write_x += (h_metrics.advance_width.ceil() as i32) + 1;
	            

	            // let bearing = bb.min.x as i32;
	            let advance = h_metrics.advance_width.ceil() as i32;

	            let bitmap_glyph = BitmapGlyph {
					texture_region: TextureRegion {
						// u_min: (bb.min.x + write_x) as u32,
					 //    u_max: (bb.max.x + write_x) as u32,
					 //    v_min: (bb.min.y + write_y) as u32,
					 //    v_max: (bb.max.y + write_y) as u32,
					 	u_min: (write_x + bb.min.x - 1) as u32,
				    	u_max: (write_x + bb.max.x + 1) as u32,
				    	v_min: (write_y) as u32,
				    	v_max: (write_y + pixel_height) as u32,
				    	texture_size: image_size,
					},
					advance: advance,
				};
				// debug marks
				// {
				// 	let tr = &bitmap_glyph.texture_region;
				// 	img.put_pixel(tr.u_min, tr.v_min, Rgba { data: [255, 0, 0,255]});
				// 	img.put_pixel(tr.u_max - 1, tr.v_min, Rgba { data: [0, 255, 0,255]});
				// 	img.put_pixel(tr.u_min, tr.v_max - 1, Rgba { data: [0, 0, 255,255]});
				// 	img.put_pixel(tr.u_max - 1, tr.v_max - 1, Rgba { data: [255, 105, 180,255]});	
				// }
				
				
				glyphs.insert(c, bitmap_glyph);
				write_x += (bb.max.x + padding) as i32;
				// println!("{:?} ->  h_metrics are {:?} and bb is {:?} bearing {:?} advance {:?}", c, h_metrics, bb, bearing, advance);
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