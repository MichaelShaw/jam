
use glutin;
use gfx;
use gfx_device_gl;
use gfx_window_glutin;
use gfx::traits::FactoryExt;

use {JamResult, InputState, Dimensions};

use render::FileResources;
use super::{Renderer, ColorFormat, DepthFormat, OpenGLRenderer};

pub fn get_dimensions(window: &glutin::GlWindow) -> Dimensions { // make this optional at some point
    Dimensions {
        pixels: window.get_inner_size_pixels().unwrap_or((100, 100)),
        points: window.get_inner_size_points().unwrap_or((100, 100)),
    }
}

pub fn construct_opengl_renderer(file_resources: FileResources, dimensions: (u32, u32), vsync: bool, window_name: &str) -> JamResult<OpenGLRenderer> {
    let (width, height) = dimensions;
    println!("pre events");
    let mut events_loop = glutin::EventsLoop::new();
    let window_config = glutin::WindowBuilder::new()
        .with_title("Triangle example".to_string())
        .with_dimensions(width, height);
    use glutin::{GlRequest, Api};
    let context = glutin::ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .with_vsync(true);


    //    context = 4;

    println!("pre build");
    let (window, mut device, mut factory, mut main_color, mut main_depth) = gfx_window_glutin::init::<ColorFormat, DepthFormat>(window_config, context, &events_loop);

    println!("post build");
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    println!("post encoder");
    let sampler = factory.create_sampler_linear();

    let dimensions = get_dimensions(&window);

    println!("pre watch");
    let file_watcher = file_resources.watch();
    println!("post watch");
    Ok(Renderer {
        file_resources,
        file_watcher,
        window,
        events_loop,
        device,
        factory,
        screen_colour_target: main_color,
        screen_depth_target: main_depth,
        encoder: encoder,
        texture: None,
        sampler,
        pipelines: None,
        dimensions,
        input_state: InputState::default(),
    })
}