

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coord: [f32; 3],
    pub color: [f32; 4],
    pub normal: [f32; 3],
}

implement_vertex!(Vertex, position, tex_coord, color, normal);