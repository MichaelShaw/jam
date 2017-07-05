
use font::BitmapFont;
use super::quads;

use render::vertex::Vertex;

use Vec2;

// simple naive render
pub fn render(text:&str, font: &BitmapFont, layer:u32, top_left:Vec2, z: f64, scale: f64, tesselator: &quads::GeometryTesselator, vertices: &mut Vec<Vertex>) -> Vec2 {
    let mut at = top_left;

    // yes, we're allocating here, I know it's evil ... it's to make kerning easy
    let chars : Vec<char> = text.chars().collect();

    for (i,c) in chars.iter().enumerate() {
        if let Some(glyph) = font.glyphs.get(&c) {
            // no kerning yet
            if let Some(ref tr) = glyph.texture_region {
                tesselator.draw_ui(vertices, tr, layer, at.x as f64, at.y as f64, z, false, scale);
            }

            let kerning : i32 = if i < chars.len() - 1 {
                *font.kerning.get(&(*c, chars[i+1])).unwrap_or(&0)
            } else {
                0
            };

            let advance = (glyph.advance + kerning) as f64;

            at.x += advance * scale;
        }
    }

    at
}

pub fn render_text(text: &str, font: &BitmapFont, layer:u32, top_left:Vec2, z: f64, scale: f64, tesselator: &quads::GeometryTesselator, vertices: &mut Vec<Vertex>, max_width : Option<f64>) -> Vec2 {
    let mut at = top_left;

    let per_line = (font.description.pixel_size as f64) * scale;

    let space_advance : f64 = font.glyphs.get(&' ').map(|g|g.advance as f64 * scale).unwrap_or(0.0);

    let mut max_x : f64 = 0.0;

    for line in text.lines() {
        for word in line.split_whitespace() {
            let word_width =  measure_width(word, font, scale);

            if let Some(width) = max_width {
                if at.x + space_advance + word_width > top_left.x + width {
                    // new line
                    at.x = top_left.x;
                    at.y -= per_line;
                }
            } 

            // DO THE DRAWING HERE
            render(word, font, layer, at, z, scale, tesselator, vertices);

            at.x += space_advance + word_width;
            if at.x > max_x {
                max_x = at.x;    
            }
        }

        at.x = top_left.x;
        at.y -= per_line;
    }


    if at.x > top_left.x {
        at.y -= per_line;
    }
    
    // unsure what we should be returning here to be honest ... extents I guess?
    Vec2::new(max_x, top_left.y + at.y)
}

pub fn measure(text: &str, font: &BitmapFont, scale: f64, max_width: Option<f64>) -> Vec2 {
    let per_line = (font.description.pixel_size as f64) * scale;
    let mut at = Vec2::new(0.0, 0.0);

    let space_advance : f64 = font.glyphs.get(&' ').map(|g|g.advance as f64 * scale).unwrap_or(0.0);

    let mut max_x : f64 = 0.0;

    for line in text.lines() {
        for word in line.split_whitespace() {
            let word_width = measure_width(word, font, scale);

            if let Some(width) = max_width {
                if at.x + space_advance + word_width > width {
                    // new line
                    at.x = 0.0;
                    at.y -= per_line;
                }
            } 

            at.x += space_advance + word_width;
            if at.x > max_x {
                max_x = at.x;    
            }
        }

        at.x = 0.0;
        at.y -= per_line;
    }


    if at.x > 0.0 {
        at.y -= per_line;
    }
    
    Vec2::new(max_x, -at.y)
}

pub fn measure_width(text: &str, font: &BitmapFont, scale: f64) -> f64 {
    let chars : Vec<char> = text.chars().collect();

    let mut at = Vec2::new(0.0, 0.0);

    for (i,c) in chars.iter().enumerate() {
        if let Some(glyph) = font.glyphs.get(&c) {
            let kerning : i32 = if i < chars.len() - 1 {
                *font.kerning.get(&(*c, chars[i+1])).unwrap_or(&0)
            } else {
                0
            };

            let advance = (glyph.advance + kerning) as f64;

            at.x += advance * scale;
        }
    }

    at.x
}
