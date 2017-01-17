use std::fs;
use std::io;
use std::path::{Path,PathBuf};

use image;
use image::GenericImage;
use std::fmt;

// hot loading ability for texture arrays
// load files from a single directory

#[derive(Debug)]
pub enum LoadError {
    IOError(io::Error),
    ImageError(image::ImageError),
    NoFiles,
    MismatchingDimensions, // path buf, expectation
}

impl From<io::Error> for LoadError {
    fn from(err: io::Error) -> Self {
        LoadError::IOError(err)
    }
}

impl From<image::ImageError> for LoadError {
    fn from(err: image::ImageError) -> Self {
        LoadError::ImageError(err)
    }
}

type Dimensions = (u16, u16);

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

    pub fn load(&self) -> Result<TextureArrayData, LoadError> {
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

pub fn read_directory_paths(path:&Path) -> io::Result<Vec<PathBuf>> {
    let mut paths : Vec<PathBuf> = Vec::new();

    for entry in try!(fs::read_dir(path)) {
        let entry = try!(entry);
        let file_path = entry.path().to_path_buf();
        paths.push(file_path);
    }

    Ok(paths)
}

pub fn load_directory(path:&Path) -> Result<TextureArrayData, LoadError> {
    let mut file_data : Vec<Vec<[u8; 4]>> = Vec::new();

    let mut dimensions : Option<Dimensions> = None;

    let mut paths = try!(read_directory_paths(path));
    paths.sort();

    println!("sorted paths -> {:?}", paths);

    for path in paths {
        let img = try!(image::open(path));

        let d = img.dimensions();
        let w = d.0 as u16;
        let h = d.1 as u16;

        if let Some(ed) = dimensions {
            if ed != (w, h) {
                return Err(LoadError::MismatchingDimensions);
            }
        } else {
            dimensions = Some((w, h));
        }
        
        let image_buffer = img.to_rgba();
        let mut data : Vec<[u8; 4]> = Vec::new();

        for pixel in image_buffer.pixels() {
            data.push(pixel.data);
        }

        file_data.push(data);
    }

    if let Some(d)  = dimensions {
        Ok(TextureArrayData {
            dimensions: d,
            data: file_data,
        })
    } else {
        Err(LoadError::NoFiles)
    }    
}


// &[&[u8]]
// is the final needed type
// an array of references to slices

// basically you keep the TextureArrayFiles as a key, equality changes

// note: we can throw this away post load ... it's a temporary construct    
pub struct TextureArrayData {
    pub dimensions : Dimensions,
    pub data: Vec<Vec<[u8; 4]>>,
}

impl fmt::Debug for TextureArrayData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TextureArrayData {{  dimensions: {:?}, data: {} }}", self.dimensions, self.data.len())
    }
}

use gfx;
use gfx::handle::Texture;
use gfx::texture; 
use gfx::format::SurfaceTyped;

impl TextureArrayData {
    pub fn kind(&self) -> gfx::texture::Kind {
        let (width, height) = self.dimensions;
        let layers = self.data.len() as u16;
        texture::Kind::D2Array(width, height, layers, texture::AaMode::Single)
    } 

    pub fn load<R, F, S>(&self, factory: &mut F) -> Result<Texture<R, S>, texture::CreationError>
     where R: gfx::Resources, F: gfx::Factory<R>, S: SurfaceTyped {
        let bind = gfx::SHADER_RESOURCE;
        let cty = gfx::format::ChannelType::Srgb;

        factory.create_texture(self.kind(), 1, bind, gfx::memory::Usage::Dynamic, Some(cty))
    }
}

