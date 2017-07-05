
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct TextureRegion {
    pub u_min: u32,
    pub u_max: u32,
    pub v_min: u32,
    pub v_max: u32,
    pub texture_size: u32,
}

impl TextureRegion {
    pub fn width(&self) -> u32 {
        self.u_max - self.u_min
    }

    pub fn height(&self) -> u32 {
        self.v_max - self.v_min
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
        ((self.u_max - self.u_min) as f32) / (self.texture_size as f32)
    }

    pub fn n_height(&self) -> f32 {
        ((self.v_max - self.v_min) as f32) / (self.texture_size as f32)
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct TextureAtlas {
    pub texture_size: u32,
    pub tile_size: u32,
}

impl TextureAtlas {
    pub fn tile_extents_x(&self, u:u32, width:u32) -> (u32, u32) {
        let u_min = self.tile_size * u;
        let u_max = u_min + self.tile_size * width;
        (u_min, u_max)
    }

    pub fn tile_extents_y(&self, v:u32, height: u32) -> (u32, u32) {
        let v_min = self.tile_size * v;
        let v_max = v_min + self.tile_size * height;
        (v_min, v_max)
    }

    pub fn at(&self, u: u32, v:u32) -> TextureRegion {
        let (u_min, u_max) = self.tile_extents_x(u, 1);
        let (v_min, v_max) = self.tile_extents_y(v, 1);
        TextureRegion {
            u_min: u_min,
            u_max: u_max,
            v_min: v_min,
            v_max: v_max,
            texture_size: self.texture_size,
        }
    }

    pub fn get(&self, u: u32, v: u32, wide: u32, high: u32) -> TextureRegion {
        let (u_min, u_max) = self.tile_extents_x(u, wide);
        let (v_min, v_max) = self.tile_extents_y(v, high);
         TextureRegion {
            u_min: u_min,
            u_max: u_max,
            v_min: v_min,
            v_max: v_max,
            texture_size: self.texture_size,
        }
    }
}