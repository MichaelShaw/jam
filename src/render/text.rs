
use font::BitmapFont;
use super::quads;

use Vec2;

pub fn render(text:&str, font: &BitmapFont, layer:u32, top_left:Vec2, tesselator: &mut quads::GeometryTesselator) {
    let mut at = top_left;

    // yes, we're allocating here, I know it's evil ... it's to make kerning easy
    let chars : Vec<char> = text.chars().collect();

    for (i,c) in chars.iter().enumerate() {
        if let Some(glyph) = font.glyphs.get(&c) {
            // no kerning yet
            if let Some(ref tr) = glyph.texture_region {
                tesselator.draw_wall_tile(tr, layer, at.x as f64, at.y as f64, 0.0, 0.0, false);
            }

            let kerning : i32 = if i < chars.len() - 1 {
                *font.kerning.get(&(*c, chars[i+1])).unwrap_or(&0)
            } else {
                0
            };

            let advance = (glyph.advance + kerning) as f64;

            at.x += advance * tesselator.scale.x;
        }
    }
}
