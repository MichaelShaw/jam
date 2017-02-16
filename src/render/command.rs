
use super::vertex::Vertex;

use std::fmt;
use std::fmt::Debug;
use color::Color;

pub type BufferData = Vec<Vertex>;
pub type Transform = [[f32; 4]; 4];

#[derive(Copy, Clone, Debug)]
pub struct Uniforms {
    pub transform : Transform,
    pub color: Color,
}

pub enum Command<BufferKey> { // where BufferKey : Sized 
    Delete { key: BufferKey },
    DeleteMatching { pred: Box<Fn(&BufferKey) -> bool> },
    Update { key: BufferKey, vertices:BufferData }, 
    Draw { key: BufferKey, uniforms: Uniforms },
    DrawNew { key: Option<BufferKey>, vertices: BufferData, uniforms: Uniforms },
}

impl <BufferKey> fmt::Debug for Command<BufferKey> where BufferKey : Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Command::*;
        match self {
            &Delete { ref key } => write!(f, "Delete {{ key: {:?} }}", key),
            &DeleteMatching { .. } => write!(f, "DeleteMatching {{ pred: <function> }}"),
            &Update { ref key, ref vertices} => write!(f, "Update {{ key: {:?} vertices: {:?} }}", key, vertices.len()),
            &Draw { ref key, ref uniforms } => write!(f, "Draw {{ key: {:?} uniforms: {:?} }}", key, uniforms),
            &DrawNew { ref key, ref vertices, ref uniforms } => write!(f, "DrawNew {{ key: {:?} vertices: {:?} uniforms: {:?} }}", key, vertices.len(), uniforms),
        }
    }
}

pub struct Pass<BufferKey> {
    pub blend: Blend,
    pub commands: Vec<Command<BufferKey>>,
    pub clear_depth: bool,
}

impl <BufferKey> fmt::Debug for Pass<BufferKey> where BufferKey : Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pass {{ blend: {:?} commands: {:?} clear_depth: {:?} }}", self.blend, self.commands.len(), self.clear_depth)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Blend {
    None,
    Add,
    Alpha,
}