#![allow(dead_code)]


extern crate jam;

extern crate image;
extern crate rusttype;
extern crate time;

use jam::font::{FontDescription, build_font};

fn main() {
	let font_description = FontDescription {
		family: "DejaVuSerif".into(),
		point_size: 48,
		image_size: 512,
	};

	let start_time = time::precise_time_ns();
	
    let font = build_font("./resources/fonts", &font_description).unwrap();

    let duration = time::precise_time_ns() - start_time; 

    let seconds = (duration as f64) / 1_000_000_000.0;
    println!("completed in {:} seconds", seconds);

	println!("font description -> {:?}", font.description);
    println!("font gylphs -> {:?}", font.glyphs);
    println!("font kerning -> {:?}", font.kerning);
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
