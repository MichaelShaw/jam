extern crate gfx;
// extern crate image;

use std::fmt;
use std::sync::mpsc::channel;

use notify::{RecommendedWatcher, Watcher, RecursiveMode, RawEvent};

use glutin;
use glutin::GlRequest;
use glutin::GlProfile;
use glutin::Api;

use time;


use gfx::traits::FactoryExt;
use gfx::Device;
use gfx::texture;
use gfx::Factory;
use gfx_window_glutin;

use cgmath::SquareMatrix;

pub use {DepthFormat, ColorFormat};
use {Mat4, Vec3};
use JamError;
use input;
use input::InputState;
use color::Color;
use render::texture::texture_region::TextureRegion;

gfx_defines!{
    vertex Vertex {
        position: [f32; 3] = "a_position",
        tex_coord: [f32; 3] = "a_tex_coord",
        color: [f32; 4] = "a_color",
        normal: [f32; 3] = "a_normal",
    }
  
    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        u_texture_array: gfx::TextureSampler<[f32; 4]> = "u_texture_array",
        u_matrix: gfx::Global<[[f32; 4]; 4]> = "u_matrix",
        u_color: gfx::Global<[f32; 4]> = "u_color",
        u_alpha_minimum : gfx::Global<f32> = "u_alpha_minimum",
        u_sun_direction: gfx::Global<[f32; 3]> = "u_sun_direction",
        out_color: gfx::RenderTarget<ColorFormat> = "f_color",
        out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }

    pipeline pipe_alpha {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        u_texture_array: gfx::TextureSampler<[f32; 4]> = "u_texture_array",
        u_matrix: gfx::Global<[[f32; 4]; 4]> = "u_matrix",
        u_color: gfx::Global<[f32; 4]> = "u_color",
        u_alpha_minimum : gfx::Global<f32> = "u_alpha_minimum",
        u_sun_direction: gfx::Global<[f32; 3]> = "u_sun_direction",
        out_color: gfx::BlendTarget<ColorFormat> = ("f_color", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
        out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_TEST,
    }
}

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

const BLUE_COLOR: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

const Z_UP : [f32; 3] = [0.0, 0.0, 1.0];

const WHITE_COLOR : [f32; 4] = [1.0, 1.0, 1.0, 1.0];
// we need a render state struct ...

pub fn down_size_m4(arr: [[f64; 4];4]) -> [[f32; 4]; 4] {
    let mut out : [[f32; 4]; 4] = [[0.0; 4]; 4];
    for a in 0..4 {
        for b in 0..4 {
            out[a][b] = arr[a][b] as f32
        }
    }

    out
}

pub type BufferKey = String;
pub type BufferData = Vec<Vertex>;
pub type Transform = [[f32; 4]; 4];

#[derive(Copy, Clone, Debug)]
pub struct Uniforms {
    pub transform : Transform,
    pub color: Color,
}

pub enum Command {
    Delete { prefix: BufferKey },
    Update { key: BufferKey, vertices:BufferData }, 
    Draw { key: BufferKey, uniforms: Uniforms },
    DrawNew { key: Option<BufferKey>, vertices: BufferData, uniforms: Uniforms },
    Close,
}

pub type Seconds = f64;
pub type Dimensions = (u32, u32);

pub trait Application {
    fn new(&mut self);
    fn render(&mut self, input:&InputState, dimensions:Dimensions, delta_time: Seconds) -> Vec<Command>; // sizing (window) ?
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Command::*;
        match self {
            &Delete { ref prefix } => write!(f, "Delete {{ prefix: {:?} }}", prefix),
            &Update { ref key, ref vertices} => write!(f, "Update {{ key: {:?} vertices: {:?} }}", key, vertices.len()),
            &Draw { ref key, ref uniforms } => write!(f, "Draw {{ key: {:?} uniforms: {:?} }}", key, uniforms),
            &DrawNew { ref key, ref vertices, ref uniforms } => write!(f, "DrawNew {{ key: {:?} vertices: {:?} uniforms: {:?} }}", key, vertices.len(), uniforms),
            &Close => write!(f, "Close"),
        }
    }
}



use render::shader::ShaderPair;
use render::texture::texture_array::TextureDirectory;

extern crate gfx_device_gl;

type PipelineState = gfx::PipelineState<gfx_device_gl::Resources, pipe::Meta>;
type PipelineStateAlpha = gfx::PipelineState<gfx_device_gl::Resources, pipe_alpha::Meta>;


struct Vertices<R> where R: gfx::Resources {
    buffer : gfx::handle::Buffer<R, Vertex>,
    slice : gfx::Slice<R>,
}

pub fn fat_example<T>(mut app:T, shader_pair:ShaderPair, texture_directory: TextureDirectory, dimensions: Dimensions) where T: Application {
    println!("shader pair -> {:?}", shader_pair);

    app.new();

    let (tx, rx) = channel::<RawEvent>();
    // , Duration::from_secs(0)
    let mut watcher : RecommendedWatcher = Watcher::new_raw(tx).expect("a watcher");
    watcher.watch(&shader_pair.vertex_path, RecursiveMode::Recursive).expect("watching shader vertex path");
    watcher.watch(&shader_pair.fragment_path, RecursiveMode::Recursive).expect("watching shader fragment path");
    watcher.watch(&texture_directory.path, RecursiveMode::Recursive).expect("watching texture directory path");

    let mut input_state = InputState::default();

    let (w, h) = dimensions;
    
    let builder = glutin::WindowBuilder::new()
        .with_title("Fat example".to_string())
        .with_dimensions(w, h)
        .with_vsync()
        .with_gl_profile(GlProfile::Core)
        .with_gl(GlRequest::Specific(Api::OpenGl,(3,3)));

    let (window, mut device, mut factory, main_color, main_depth) = gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let mut pso : Option<PipelineState> = None;
    // let mut pso : Option<PipelineStateAlpha> = None;
    
    let texture_data = texture_directory.load().expect("texture data");

    // let kind = texture_data.kind();
    println!(":: pre create");
    // gfx::format::R8_G8_B8_A8
    let mut texture_pair = texture_data.load::<_,_,gfx::format::Rgba8>(&mut factory).expect("a damn texture");
    println!(":: post create");
  
    // texture_view = 5;

    let sampler_info = texture::SamplerInfo::new(texture::FilterMethod::Scale, texture::WrapMode::Clamp);
    let sampler = factory.create_sampler(sampler_info);

    let default_transform : [[f64; 4]; 4] = Mat4::identity().into();

    use render::quads::GeometryTesselator;

    let base_pixels_per_unit = 16.0_f64;
    let units_per_pixel = 1.0 / base_pixels_per_unit;
    let tesselator_scale = Vec3::new(units_per_pixel, units_per_pixel, units_per_pixel);
    let mut tesselator = GeometryTesselator::new(tesselator_scale);
    let texture_region = TextureRegion {
        u_min: 0,
        u_max: 128,
        v_min: 0,
        v_max: 128,
        texture_size: 128,
    };
    tesselator.draw_floor_tile(&texture_region, 0, 0.0, 0.0, 0.0, 0.0, false);
    tesselator.draw_floor_tile(&texture_region, 1, 1.0, 0.0, 1.0, 0.0, false);
    tesselator.draw_floor_tile(&texture_region, 0, 2.0, 0.0, 2.0, 0.0, false);
    tesselator.draw_floor_tile(&texture_region, 1, 3.0, 0.0, 3.0, 0.0, false);
    let (empty_vertex_buffer, _) = factory.create_vertex_buffer_with_slice(&tesselator.tesselator.vertices, ());

    // let (empty_vertex_buffer, _) = factory.create_vertex_buffer_with_slice(&[], ());
    // data
    // let mut data = pipe::Data {
    //     vbuf: empty_vertex_buffer,
    //     u_texture_array: (texture_view, sampler), // resource, sampler
    //     u_matrix: down_size_m4(default_transform),
    //     u_color: WHITE_COLOR,
    //     u_alpha_minimum: 0.0,
    //     u_sun_direction: Z_UP,
    //     out_color: main_color,
    //     out_depth: main_depth,
    // };

    let mut data = pipe::Data {
        vbuf: empty_vertex_buffer,
        u_texture_array: (texture_pair.1, sampler), // resource, sampler
        u_matrix: down_size_m4(default_transform),
        u_color: WHITE_COLOR,
        u_alpha_minimum: 0.0,
        u_sun_direction: Z_UP,
        out_color: main_color,
        out_depth: main_depth,
    };

    // `gfx::handle::Buffer<render::shader::fatter::gfx_device_gl::Resources, render::shader::fatter::Vertex>
    use HashMap;

    let mut vertex_buffers : HashMap<BufferKey, Vertices<gfx_device_gl::Resources>> = HashMap::default();
    
    let mut last_time = time::precise_time_ns();
    
    'main: loop {
        // CHECK RESOURCES
        let mut reload_pso : bool = pso.is_none();
        let mut reload_texture : bool = false;

        'fs: loop {
            match rx.try_recv() {
                Ok(RawEvent { path, op:_, cookie:_ }) => {
                    if let Some(p) = path {
                        if shader_pair.contains(&p) {
                            reload_pso = true;
                        } else if texture_directory.contains(&p) {
                            reload_texture = true;
                        } else {
                            use std::path;
                            let components: Vec<path::Component> = p.components().collect();
                            println!("fs event {:?} -> {:?}", p, components);
                        }
                    }
                },
                Err(_) => break 'fs,
            }
        }

        if reload_pso {
            println!("reloading pso!");
            let pso_result = shader_pair.load().map_err(JamError::IO).and_then(|shader_data| {
                factory.create_pipeline_simple(
                    &shader_data.vertex_data,
                    &shader_data.fragment_data,
                    pipe::new()
                ).map_err(JamError::Pipeline)
            });
            match pso_result {
                Ok(ps) => pso = Some(ps),
                Err(err) => println!("pso error -> {:?}", err),
            }
        }

        if reload_texture { // reload_texture
            let new_texture_data = texture_directory.load();
            match new_texture_data {
                Ok(texture_data) => {
                    texture_pair = texture_data.load::<_,_,gfx::format::Rgba8>(&mut factory).expect("a damn texture");
                    data.u_texture_array.0 = texture_pair.1;
                },
                Err(err) => println!("texture load error -> {:?}", err),
            }
        }


        // loop over events
        let events : Vec<glutin::Event> = window.poll_events().collect();
        input_state = input::produce(&input_state, &events);
        
        // update view port size etc.
        gfx_window_glutin::update_views(&window, &mut data.out_color, &mut data.out_depth);
        let (ww, hh, _, _) = data.out_color.get_dimensions();
        
        let deeps = window.hidpi_factor();
        

        let now  = time::precise_time_ns();
        let delta = ((now - last_time) as f64) / 1000000000.0;
        let commands = app.render(&input_state, ((ww as f32 / deeps) as u32, (hh as f32  / deeps) as u32), delta);
        last_time = now;


        encoder.clear(&data.out_color, CLEAR_COLOR);
        encoder.clear_depth(&data.out_depth, 1.0);

        use self::Command::*;

        for command in commands {
            // println!("received command -> {:?}", command);
            match command {
                Delete { prefix } => {
                    let keys_to_remove : Vec<String> = vertex_buffers.keys().filter(|k| k.starts_with(&prefix) ).cloned().collect();
                    for key in keys_to_remove.iter() {
                        // println!("deleting {:?}", key);
                        vertex_buffers.remove(key);
                    }
                },
                Update { key, vertices } => {
                    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&vertices, ());
                    vertex_buffers.insert(key, Vertices {
                        buffer: vertex_buffer,
                        slice: slice,
                    });
                },
                Draw { key, uniforms } => {
                    if let Some(vertices) = vertex_buffers.get(&key) {
                        data.vbuf = vertices.buffer.clone();
                        data.u_matrix = uniforms.transform;
                        data.u_color = uniforms.color.float_raw();
                        if let Some(ref ps) = pso {
                            encoder.draw(&vertices.slice, &ps, &data);
                        }
                    } else {
                        // println!("couldnt draw for {:?}", key);
                    }
                },
                DrawNew { key , vertices, uniforms } => {
                    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&vertices, ());

                    if let Some(name) = key {
                        vertex_buffers.insert(name, Vertices {
                            buffer: vertex_buffer.clone(),
                            slice: slice.clone(),
                        });
                    }
                    data.vbuf = vertex_buffer;
                    data.u_matrix = uniforms.transform;
                    data.u_color = uniforms.color.float_raw();

                    if let Some(ref ps) = pso {
                        encoder.draw(&slice, &ps, &data);
                    }
                },
                Close => {
                    break 'main
                },
            }
        }

        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}