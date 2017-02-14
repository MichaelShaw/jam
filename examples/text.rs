#![allow(dead_code)]


extern crate jam;

extern crate image;
extern crate rusttype;
extern crate time;

use jam::font::{FontDescription, build_font};
use std::path::Path;

fn main() {
	let font_description = FontDescription {
		family: "DejaVuSerif".into(),
		pixel_size: 48,
	};

	let start_time = time::precise_time_ns();
	
    let loaded_font = build_font(Path::new("./resources/fonts/DejaVuSerif.ttf"), &font_description, 512).unwrap();

    let duration = time::precise_time_ns() - start_time; 

    let seconds = (duration as f64) / 1_000_000_000.0;
    println!("completed in {:} seconds", seconds);

	println!("font description -> {:?}", loaded_font.font.description);
    println!("font gylphs -> {:?}", loaded_font.font.glyphs);
    println!("font kerning -> {:?}", loaded_font.font.kerning);

    loaded_font.image.save("DejaBuSerif.png").unwrap();
}