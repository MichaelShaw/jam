
use glutin;
use gfx;
use gfx_device_gl;
use gfx_window_glutin;
use gfx::traits::FactoryExt;
use gfx::Factory;
use gfx::format::{Srgba8};
use gfx::texture::ImageInfoCommon;
use gfx::format::R8_G8_B8_A8;
use color;
use aphid::HashMap;

use render::TextureArrayDimensions;

use {JamResult, JamError, InputState, Dimensions};

use render::FileResources;
use super::{Renderer, ColorFormat, DepthFormat, OpenGLRenderer, UI, texture_kind_for};

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

    let ui_layers = 8;
    let ui_store_dimensions = TextureArrayDimensions {
        width: 512,
        height: 512,
        layers: ui_layers,
    };

    let kind = texture_kind_for(&ui_store_dimensions);
    let bind = gfx::SHADER_RESOURCE;
    let cty = gfx::format::ChannelType::Unorm;
    let ui_tex = factory.create_texture(kind, 1, bind, gfx::memory::Usage::Dynamic, Some(cty)).map_err(JamError::TextureCreationError)?;
    let ui_tex_view = factory.view_texture_as_shader_resource::<Srgba8>(&ui_tex, (0, 0), gfx::format::Swizzle::new()).map_err(JamError::ResourceViewError)?;

    for l in 0..ui_layers {
        let image_info = ImageInfoCommon {
            xoffset: 0,
            yoffset: 0,
            zoffset: l as u16,
            width: ui_store_dimensions.width as u16,
            height: ui_store_dimensions.height as u16,
            depth: 1,
            format: (),
            mipmap: 0,
        };
        let pixels = ui_store_dimensions.width * ui_store_dimensions.height;

        let color_raw = color::ALL[l as usize].raw();



        let mut data : Vec<[u8; 4]> = (0..pixels).map(|sl| color_raw ).collect();

        encoder.update_texture::<R8_G8_B8_A8, Srgba8>(
            &ui_tex,
            None,
            image_info,
            &data,
        ).expect("updating the texture");
    }

    let ui = UI {
        dimensions: ui_store_dimensions,
        texture_resource: ui_tex,
        texture_view: ui_tex_view,
        elements: HashMap::default(),
        tick: 0,
        free_layers: (0..ui_store_dimensions.layers).collect(),
    };

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
        ui: ui,
    })
}