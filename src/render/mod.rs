#![allow(dead_code)]

pub mod command;
pub mod quads;
pub mod shader;
pub mod vertex;
pub mod texture_array;
pub mod texture_region;
pub mod glium;
pub mod text;

pub use self::texture_region::TextureRegion;
pub use self::quads::GeometryTesselator;
pub use self::shader::ShaderPair;
pub use self::texture_array::TextureDirectory;

pub type Seconds = f64;


pub fn down_size_m4(arr: [[f64; 4];4]) -> [[f32; 4]; 4] {
    let mut out : [[f32; 4]; 4] = [[0.0; 4]; 4];
    for a in 0..4 {
        for b in 0..4 {
            out[a][b] = arr[a][b] as f32
        }
    }

    out
}
