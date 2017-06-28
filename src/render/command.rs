
use super::vertex::Vertex;

use color::Color;

pub type BufferData = Vec<Vertex>;
pub type Transform = [[f32; 4]; 4];

#[derive(Copy, Clone, Debug)]
pub struct Uniforms {
    pub transform : Transform,
    pub color: Color,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Blend {
    None,
    Add,
    Alpha,
}