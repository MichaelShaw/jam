use glium;
use glium::Program;
use render::shader::ShaderData;
use {JamResult, JamError};
use glium::LinearBlendingFactor;
use render;
// use glium::Blend;

pub fn draw_params_for_blend<'a>(blend:render::command::Blend) -> glium::DrawParameters<'a> {
    use render::command::Blend::*;
    match blend {
    None => opaque_draw_params(),
    Add => additive_draw_params(),
    Alpha => translucent_draw_params(),
    }
}

pub fn translucent_draw_params<'a>() -> glium::DrawParameters<'a> {
   glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: false,
            .. Default::default()
        },
        blend: glium::Blend::alpha_blending(),
        .. Default::default()
    }
}

pub fn additive_draw_params<'a>() -> glium::DrawParameters<'a> {
    use glium::BlendingFunction;
    glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: false,
            .. Default::default()
        },
        blend: glium::Blend {
            color: BlendingFunction::Addition {
                source: LinearBlendingFactor::SourceAlpha,
                destination: LinearBlendingFactor::One,
            },
            alpha: BlendingFunction::Addition {
                source: LinearBlendingFactor::One,
                destination: LinearBlendingFactor::One,
            },
            constant_value: (0.0, 0.0, 0.0, 0.0)
        },
        .. Default::default()
    }
}

pub fn opaque_draw_params<'a>() -> glium::DrawParameters<'a> {
    glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        .. Default::default()
    }
}

impl ShaderData {
    pub fn load(&self, display: &glium::Display) -> JamResult<Program> {
        let vertex_shader = String::from_utf8(self.vertex_data.clone()).unwrap();
        let fragment_shader = String::from_utf8(self.fragment_data.clone()).unwrap();
        Program::from_source(display, &vertex_shader, &fragment_shader, None).map_err(JamError::ProgramLoadError)
    }    
}
