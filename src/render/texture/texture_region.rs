
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
