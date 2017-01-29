#![allow(dead_code)]

pub mod quads;
pub mod shader;
pub mod texture_array;
pub mod texture_region;
pub mod renderer;

pub use self::texture_region::TextureRegion;
pub use self::quads::GeometryTesselator;
pub use self::shader::ShaderPair;
pub use self::texture_array::TextureDirectory;