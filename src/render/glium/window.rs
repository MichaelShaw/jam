
use glium;
use glium::DisplayBuild;
use glium::glutin::{GlRequest, GlProfile, Api, WindowBuilder};

// note (from glium): pub use backend::glutin_backend::GlutinFacade as Display;

pub fn create_window(title: &str, vsync: bool, dimensions: (u32, u32)) -> glium::Display {
	let (width, height) = dimensions;

    let mut builder = WindowBuilder::new()
        .with_title(title)
        .with_gl_profile(GlProfile::Core)
        .with_dimensions(width, height)
        .with_gl(GlRequest::Specific(Api::OpenGl,(3,3)))
        .with_depth_buffer(24);
    if vsync {
        builder = builder.with_vsync();
    }
    builder.build_glium().unwrap()
}