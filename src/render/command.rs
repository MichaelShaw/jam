
use super::vertex::Vertex;

use std::fmt;
use color::Color;

pub type BufferKey = String;
pub type BufferData = Vec<Vertex>;
pub type Transform = [[f32; 4]; 4];

#[derive(Copy, Clone, Debug)]
pub struct Uniforms {
    pub transform : Transform,
    pub color: Color,
}

pub enum Command {
    Delete { prefix: BufferKey },
    Update { key: BufferKey, vertices:BufferData }, 
    Draw { key: BufferKey, uniforms: Uniforms },
    DrawNew { key: Option<BufferKey>, vertices: BufferData, uniforms: Uniforms },
    Close,
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Command::*;
        match self {
            &Delete { ref prefix } => write!(f, "Delete {{ prefix: {:?} }}", prefix),
            &Update { ref key, ref vertices} => write!(f, "Update {{ key: {:?} vertices: {:?} }}", key, vertices.len()),
            &Draw { ref key, ref uniforms } => write!(f, "Draw {{ key: {:?} uniforms: {:?} }}", key, uniforms),
            &DrawNew { ref key, ref vertices, ref uniforms } => write!(f, "DrawNew {{ key: {:?} vertices: {:?} uniforms: {:?} }}", key, vertices.len(), uniforms),
            &Close => write!(f, "Close"),
        }
    }
}