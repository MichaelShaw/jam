use glium;
use glium::{Program, ProgramCreationError};
use render::shader::ShaderData;
use {JamResult, JamError};

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



impl ShaderData {
    pub fn load(&self, display: &glium::Display) -> JamResult<Program> {
        let vertex_shader = String::from_utf8(self.vertex_data.clone()).unwrap();
        let fragment_shader = String::from_utf8(self.fragment_data.clone()).unwrap();

        println!("vert -> {:?}", vertex_shader);

        println!("frag -> {:?}", fragment_shader);

        Program::from_source(display, &vertex_shader, &fragment_shader, None).map_err(JamError::ProgramLoadError)
    }    
}
