use glium;
use glium::{Program, ProgramCreationError};
use render::shader::ShaderData;

pub fn translucent_draw_params<'a>() -> glium::DrawParameters<'a> {
    let draw_parameters = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        blend: glium::Blend::alpha_blending(),
        .. Default::default()
    };
    draw_parameters
}

pub fn opaque_draw_params<'a>() -> glium::DrawParameters<'a> {
    let draw_parameters = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        .. Default::default()
    };
    draw_parameters
}

use render::vertex::Vertex;

implement_vertex!(Vertex, position, tex_coord, color, normal);

impl ShaderData {
    pub fn load(&self, display: &glium::Display) -> Result<Program, ProgramCreationError> {
        let vertex_shader = String::from_utf8(self.vertex_data.clone()).unwrap();
        let fragment_shader = String::from_utf8(self.fragment_data.clone()).unwrap();

        Program::from_source(display, &vertex_shader, &vertex_shader, None)
    }    
}
