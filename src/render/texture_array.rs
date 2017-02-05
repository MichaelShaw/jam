use std::fs;
use std::io;
use std::path::{Path,PathBuf};

use image;
use image::GenericImage;
use std::fmt;

use JamResult;
use JamError;

#[derive(Debug)]
pub struct TextureDirectory {
    pub path: PathBuf, 
}

impl TextureDirectory {
    pub fn for_path(path:&str) -> TextureDirectory {
        TextureDirectory {
            path: PathBuf::from(path) // convert to absolute here?
        }
    }

    pub fn load(&self) -> JamResult<TextureArrayData> {
        load_directory(&self.path)
    }

    pub fn contains(&self, path:&Path) -> bool {
        use std::path;
        let my_components : Vec<path::Component> = self.path.components().collect();
        let components : Vec<path::Component> = path.components().collect();

        components.windows(my_components.len()).position(|window| {
            window == &my_components[..]
        }).is_some()
    }
}

pub fn read_directory_paths(path:&Path) -> JamResult<Vec<PathBuf>> {
    let mut paths : Vec<PathBuf> = Vec::new();

    for entry in try!(fs::read_dir(path)) {
        let entry = try!(entry);
        let file_path = entry.path().to_path_buf();
        paths.push(file_path);
    }

    Ok(paths)
}

pub fn load_directory(path:&Path) -> JamResult<TextureArrayData> {
    let mut file_data : Vec<Vec<u8>> = Vec::new();

    let mut dimensions : Option<Dimensions> = None;

    let mut paths = try!(read_directory_paths(path));
    paths.sort();

    println!("sorted paths -> {:?}", paths);

    for path in paths {
        let img = try!(image::open(path));

        let d = img.dimensions();
        let w = d.0 as u32;
        let h = d.1 as u32;

        if let Some(ed) = dimensions {
            if ed != (w, h) {
                return Err(JamError::MismatchingDimensions);
            }
        } else {
            dimensions = Some((w, h));
        }
        
        let image_buffer = img.to_rgba().into_raw();
        
        file_data.push(image_buffer);
    }

    if let Some(d)  = dimensions {
        Ok(TextureArrayData {
            dimensions: d,
            data: file_data,
        })
    } else {
        Err(JamError::NoFiles)
    }    
}

type Dimensions = (u32, u32); // rename this as TextureDimensions?

// hrm, we currently load it all in to ram in uncompressed form :-/ zero reason why this isn't streamed in as a whole
pub struct TextureArrayData {
    pub dimensions : Dimensions,
    pub data: Vec<Vec<u8>>,
}

impl fmt::Debug for TextureArrayData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TextureArrayData {{  dimensions: {:?}, data: {} }}", self.dimensions, self.data.len())
    }
}
