#![allow(dead_code)]


extern crate jam;

extern crate image;
extern crate rusttype;
extern crate time;


// fonts
use rusttype::{FontCollection, Scale};

use std::fs::File;
use std::path::{PathBuf, Path};
use std::io;
use std::io::Read;

use image::{RgbaImage, Rgba};

use jam::render::TextureRegion;
use jam::HashMap;

#[derive(Debug)]
pub struct FontDescription {
	pub family: String,
	pub point_size: u32, // what about, what code points do you want ... and what 
	pub image_size: u32,
}

#[derive(Debug)]
pub struct BitmapGlyph {
	pub texture_region: TextureRegion,
	pub advance: u32,
}

pub struct BitmapFont {
	pub description: FontDescription,
	pub image: RgbaImage,
	pub glyphs: HashMap<char, BitmapGlyph>,
	pub kerning: HashMap<(char, char), u32>,
}

pub enum FontLoadError {
	CouldntLoadFile(PathBuf, io::Error),
	What,
}

fn build_font(resource_path: &str, font_description: &FontDescription) -> Result<BitmapFont, FontLoadError> {
    let full_path = PathBuf::from(format!("{}/{}.{}", resource_path, font_description.family, "ttf"));
    println!("full_path -> {:?}", full_path);
    let font_data = load_file_contents(&full_path).map_err(|io| FontLoadError::CouldntLoadFile(full_path, io))?;
	let collection = FontCollection::from_bytes(&font_data[..]);
	let font = collection.into_font(); // this is an option

    Err(FontLoadError::What)
}

fn main() {
	let font_description = FontDescription {
		family: "DejaVuSerif".into(),
		point_size: 48,
		image_size: 512,
	};



	let image_size = 512;


	let mut img = RgbaImage::from_pixel(image_size, image_size, Rgba { data: [25,25,25,255] });



	let start_time = time::precise_time_ns();
	let font_path = PathBuf::from("./resources/fonts/DejaVuSerif.ttf");
	let font_data = load_file_contents(&font_path).unwrap();

	let collection = FontCollection::from_bytes(&font_data[..]);
    let font = collection.into_font().unwrap(); // only succeeds if collection consists of one font

    let scale = Scale { x: 48.0, y: 48.0 };
    let pixel_height = scale.y.ceil() as i32;

    // println!("pixel height -> {:?}", pixel_height);

    let v_metrics = font.v_metrics(scale);
    use std::ops::Range;
    let offset = rusttype::point(0.0, v_metrics.ascent);

    // println!("offset -> {:?}", offset);
    // println!("v_metrics -> {:?}", v_metrics);

    let char_range : Range<u8> = (32)..(127); // from space to tilde
    let chars : Vec<char> = char_range.map(|n| n as char).collect();

    // println!("mah chars -> {:?}", chars);

	let padding : i32 = 1; // to avoid texture bleeding

	let mut write_x : i32 = padding;
    let mut write_y : i32 = padding;
    

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
	                img.put_pixel(x, y, Rgba { data: [c,c,c,255] });
	            });

	            // write_x += (h_metrics.advance_width.ceil() as i32) + 1;
	            write_x += (bb.max.x + padding) as i32;

	            let bearing = bb.min.x as i32;
	            let advance = h_metrics.advance_width.ceil() as i32;

				// println!("{:?} ->  h_metrics are {:?} and bb is {:?} bearing {:?} advance {:?}", c, h_metrics, bb, bearing, advance);
	    	}
		} else {
			println!("wtf, no glyph for '{:?}'", c);
		}
    }

    img.save("./font.png").unwrap();
    
    let duration = time::precise_time_ns() - start_time; 

    let seconds = (duration as f64) / 1_000_000_000.0;
    println!("completed in {:} seconds", seconds);

	// println!("we have a font!");
}

fn display_independent_scale(points: u32, dpi_w: f32, dpi_h: f32) -> rusttype::Scale {
    // Calculate pixels per point
    let points = points as f32;
    let points_per_inch = 72.0;
    let pixels_per_point_w = dpi_w * (1.0 / points_per_inch);
    let pixels_per_point_h = dpi_h * (1.0 / points_per_inch);

    // rusttype::Scale is in units of pixels, so.
    rusttype::Scale {
        x: pixels_per_point_w * points,
        y: pixels_per_point_h * points,
    }
}

fn load_file_contents(path:&Path) -> io::Result<Vec<u8>> {
    let mut file = try!(File::open(path));
    let mut buffer : Vec<u8> = Vec::new();
    try!(file.read_to_end(&mut buffer));
    Ok(buffer)
}
