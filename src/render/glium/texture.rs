
use glium;
use glium::texture;

use render::texture_array::TextureArrayData;

use glium::texture::srgb_texture2d_array::SrgbTexture2dArray;

use JamResult;
use JamError;

impl TextureArrayData {
    pub fn load(self, display: &glium::Display) -> JamResult<SrgbTexture2dArray> {
        let dimensions = self.dimensions;

        let raw_images : Vec<_> = self.data.into_iter().map(|raw_image_data|{
            texture::RawImage2d::from_raw_rgba_reversed(raw_image_data, (dimensions.width, dimensions.height))
        }).collect();

		SrgbTexture2dArray::new(display, raw_images).map_err(JamError::TextureLoadError)
    }
}
