

use {JamResult, JamError};
use rusttype::{self, FontCollection};
use {FontLoadError, load_file_contents};
use std::path::Path;
use render::read_directory_paths;

pub struct OurFont {
    pub font: rusttype::Font<'static>,
}

pub fn load_font(full_path: &Path) -> Result<OurFont, FontLoadError> {
    let bytes = load_file_contents(&full_path).map_err(|io| FontLoadError::CouldntLoadFile(full_path.to_path_buf(), io))?;
    let collection = FontCollection::from_bytes(bytes.to_vec());

    let font = collection.into_font().ok_or(FontLoadError::CouldntReadAsFont(full_path.to_path_buf()))?;

    Ok(OurFont {
        font: font,
    })
}

pub fn load_fonts_in_path(full_path: &Path) -> JamResult<Vec<OurFont>> {
    let mut fonts = Vec::new();

    let mut paths = read_directory_paths(full_path)?;
    paths.sort();

    println!("sorted paths for fonts -> {:?}", paths);


    for path in paths {
        if let Some(extension) = path.extension().and_then(|p| p.to_str()).map(|s| s.to_lowercase()) {
            if extension == "ttf" {
                let font = load_font(path.as_path()).map_err(JamError::FontLoadError)?;
                fonts.push(font);
            }
        }
    }

    Ok(fonts)
}