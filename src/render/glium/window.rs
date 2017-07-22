
use glium;
use glium::glutin::{GlRequest, GlProfile, Api, WindowBuilder};
use glium::glutin;

use JamResult;
use JamError;

// note (from glium): pub use backend::glutin_backend::GlutinFacade as Display;

pub fn create_window(title: &str, vsync: bool, dimensions: (u32, u32)) -> JamResult<(glutin::EventsLoop, glium::Display)> {
	let (width, height) = dimensions;

    let mut events_loop = glutin::EventsLoop::new();

    let mut builder = WindowBuilder::new()
        .with_title(title)
        .with_dimensions(width, height);

    let mut context = glutin::ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .with_gl_profile(GlProfile::Core).with_depth_buffer(24);
    if vsync {
        context = context.with_vsync(true);
    }

    glium::Display::new(builder, context, &events_loop).map(|d| (events_loop, d) ).map_err(JamError::WindowCreationError)
}