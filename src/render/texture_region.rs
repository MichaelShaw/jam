// use std::u32::abs;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct TextureRegion {
    pub u_min: u32,
    pub u_max: u32,
    pub v_min: u32,
    pub v_max: u32,
    pub layer: u32,
    pub texture_size: u32,
}

impl TextureRegion {
    pub fn h_flipped(&self, flip: bool) -> TextureRegion {
        if flip {
            self.h_flip()
        } else {
            *self
        }
    }

    pub fn v_flipped(&self, flip: bool) -> TextureRegion {
        if flip {
            self.v_flip()
        } else {
            *self
        }   
    }

    pub fn h_flip(&self) -> TextureRegion {
        TextureRegion {
            u_min: self.u_max,
            u_max: self.u_min,
            v_min: self.v_min,
            v_max: self.v_max,
            layer: self.layer,
            texture_size: self.texture_size,
        }
    }

    pub fn v_flip(&self) -> TextureRegion {
        TextureRegion {
            u_min: self.u_min,
            u_max: self.u_max,
            v_min: self.v_max,
            v_max: self.v_min,
            layer: self.layer,
            texture_size: self.texture_size,
        }
    }

    pub fn width(&self) -> u32 {
        if self.u_max > self.u_min {
            self.u_max - self.u_min
        } else {
            self.u_min - self.u_max
        }
    }

    pub fn height(&self) -> u32 {
        if self.v_max > self.v_min {
            self.v_max - self.v_min
        } else {
            self.v_min - self.v_max
        }
    }

    pub fn nu_min(&self) -> f32 {
         (self.u_min as f32) / (self.texture_size as f32)
    }

    pub fn nu_max(&self) -> f32 {
        (self.u_max as f32) / (self.texture_size as f32)
    }

    pub fn nv_min(&self) -> f32 {
        (self.v_min as f32) / (self.texture_size as f32)
    }

    pub fn nv_max(&self) -> f32 {
        (self.v_max as f32) / (self.texture_size as f32)
    }

    pub fn nu_mid(&self) -> f32 {
        (self.nu_min() + self.nu_max()) / 2.0
    }

    pub fn nv_mid(&self) -> f32 {
        (self.nv_min() + self.nv_max()) / 2.0
    }

    pub fn n_width(&self) -> f32 {
        (self.width() as f32) / (self.texture_size as f32)
    }

    pub fn n_height(&self) -> f32 {
        (self.height() as f32) / (self.texture_size as f32)
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct TextureAtlas {
    pub texture_size: u32,
    pub tile_size: u32,
}

impl TextureAtlas {
    pub fn layer(&self, layer: u32) -> TextureAtlasLayer {
        TextureAtlasLayer {
            tile_size: self.tile_size,
            texture_size: self.texture_size,
            layer,
        }
    }
}

pub struct TextureAtlasLayer {
    pub tile_size: u32,
    pub texture_size: u32,
    pub layer: u32,
}

pub fn tile_extents_x(tile_size: u32, u:u32, width:u32) -> (u32, u32) {
    let u_min = tile_size * u;
    let u_max = u_min + tile_size * width;
    (u_min, u_max)
}

pub fn tile_extents_y(tile_size: u32, v:u32, height: u32) -> (u32, u32) {
    let v_min = tile_size * v;
    let v_max = v_min + tile_size * height;
    (v_min, v_max)
}

impl TextureAtlas {
    pub fn at(&self, u: u32, v:u32) -> TextureRegion {
        let (u_min, u_max) = tile_extents_x(self.tile_size, u, 1);
        let (v_min, v_max) = tile_extents_y(self.tile_size, v, 1);
        TextureRegion {
            u_min: u_min,
            u_max: u_max,
            v_min: v_min,
            v_max: v_max,
            layer: 0,
            texture_size: self.texture_size,
        }
    }

    pub fn get(&self, u: u32, v: u32, wide: u32, high: u32) -> TextureRegion {
        let (u_min, u_max) = tile_extents_x(self.tile_size, u, wide);
        let (v_min, v_max) = tile_extents_y(self.tile_size, v, high);
         TextureRegion {
            u_min: u_min,
            u_max: u_max,
            v_min: v_min,
            v_max: v_max,
            layer: 0,
            texture_size: self.texture_size,
        }
    }
}



impl TextureAtlasLayer {
    pub fn at(&self, u: u32, v:u32) -> TextureRegion {
        let (u_min, u_max) = tile_extents_x(self.tile_size, u, 1);
        let (v_min, v_max) = tile_extents_y(self.tile_size, v, 1);
        TextureRegion {
            u_min: u_min,
            u_max: u_max,
            v_min: v_min,
            v_max: v_max,
            layer: self.layer,
            texture_size: self.texture_size,
        }
    }

    pub fn get(&self, u: u32, v: u32, wide: u32, high: u32) -> TextureRegion {
        let (u_min, u_max) = tile_extents_x(self.tile_size, u, wide);
        let (v_min, v_max) = tile_extents_y(self.tile_size, v, high);

        TextureRegion {
            u_min: u_min,
            u_max: u_max,
            v_min: v_min,
            v_max: v_max,
            layer: self.layer,
            texture_size: self.texture_size,
        }
    }
}