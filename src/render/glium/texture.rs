
use glium;
use glium::texture;

use std::path::Path;

use render::texture_array::TextureArrayData;

impl TextureArrayData {
    pub fn load(self, display: &glium::Display) -> texture::Texture2dArray {
        let dimensions = self.dimensions;

        let raw_images : Vec<_> = self.data.into_iter().map(|raw_image_data|{
            texture::RawImage2d::from_raw_rgba_reversed(raw_image_data, dimensions)
        }).collect();

        texture::Texture2dArray::new(display, raw_images).unwrap()
    }
}