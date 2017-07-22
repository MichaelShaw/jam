
use glium::{Program, Surface};

use glium::VertexBuffer;
use glium::index;
use glium;
use glutin;

use render::*;


use input;

use input::InputState;
use color::Color;

use glium::texture::srgb_texture2d_array::SrgbTexture2dArray;

use std::sync::mpsc::{channel, Receiver};

use notify::{RecommendedWatcher, Watcher, RecursiveMode, RawEvent};

use super::window;
use super::program;

use dimensions::Dimensions;

use font::*;

use camera;
use color;
use ui;
use ui::View;

use cgmath::vec3;

use {JamResult, JamError};

use image;

pub struct GeometryBuffer {
    pub vertex_buffer: VertexBuffer<Vertex>,
}

pub struct Renderer {
    pub shader_pair : ShaderPair,
    pub texture_directory: TextureDirectory,
    pub font_directory: FontDirectory,

    pub resource_file_watcher : RecommendedWatcher,
    pub resource_file_change_events: Receiver<RawEvent>,

    pub input_state: InputState,
    pub dimensions: Dimensions,

    pub display: glium::Display,
    pub events_loop: glutin::EventsLoop,
    pub program : Option<Program>,
    pub texture : Option<(TextureArrayData, SrgbTexture2dArray)>,

    // ui cache
    pub ui_texture: Option<SrgbTexture2dArray>,

    pub fonts: Vec<LoadedBitmapFont>,

    // do we build in a ui cache?
}

fn dimensions_for(display : &glium::Display) -> Dimensions {
    let pixels = display.gl_window().get_inner_size_pixels().unwrap_or((100, 100));
    let points = display.gl_window().get_inner_size_points().unwrap_or((100, 100));

    Dimensions {
        pixels: pixels,
        points: points,
    }  
}

impl Renderer {
    pub fn new(shader_pair : ShaderPair, texture_directory: TextureDirectory, font_directory: FontDirectory, initial_dimensions: (u32, u32), vsync:bool, window_name: String) -> JamResult<Renderer> { //  
        let (tx, notifier_rx) = channel::<RawEvent>();

        let mut resource_file_watcher : RecommendedWatcher = Watcher::new_raw(tx).expect("a watcher");
        resource_file_watcher.watch(&shader_pair.vertex_path, RecursiveMode::Recursive).expect("watching shader vertex path");
        resource_file_watcher.watch(&shader_pair.fragment_path, RecursiveMode::Recursive).expect("watching shader fragment path");
        resource_file_watcher.watch(&texture_directory.path, RecursiveMode::Recursive).expect("watching texture directory path");

        let (events_loop, display) = window::create_window(&window_name, vsync, initial_dimensions)?;
        
        let dimensions = dimensions_for(&display);

        Ok(Renderer {
            shader_pair : shader_pair,
            texture_directory: texture_directory,
            font_directory: font_directory,
            resource_file_watcher : resource_file_watcher,
            resource_file_change_events: notifier_rx,
            display: display,
            events_loop: events_loop,
            input_state: InputState::default(),
            program : None,
            texture : None,
            ui_texture: None,
            dimensions : dimensions,
            fonts: Vec::new(),
        })
    }

    pub fn load_font(&mut self, font_description: &FontDescription) -> JamResult<()> {
        let found_font = self.fonts.iter().any(|lf| &lf.font.description == font_description); // { f.font.description == font_description }

        if found_font {
            Ok(())
        } else if let Some(dimensions) = self.texture_array_dimensions() {
            let mut full_path = self.font_directory.path.clone();
            full_path.push(font_description.family.clone());
            full_path.set_extension("ttf");

            // println!("load font {:?} with full_path -> {:?}", font_description, full_path);

            let font = build_font(full_path.as_path(), font_description, dimensions.width)?;

            self.fonts.push(font);
            self.texture = None;

            Ok(())
        } else {
            // println!("must load texture before font, you doophus");
            Err(JamError::MustLoadTextureBeforeFont)
        }
    }

    pub fn get_font(&self, font_description: &FontDescription) -> Option<(&BitmapFont, u32)> {
        if let Some(dimensions) = self.texture_array_dimensions() {
            if let Some(font_position) = self.fonts.iter().position(|loaded_font| &loaded_font.font.description == font_description) {
                let font_count = self.fonts.len();
                let layer = dimensions.layers as usize - font_count + font_position;
                Some((&self.fonts[font_position].font, layer as u32))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn texture_array_dimensions(&self) -> Option<TextureArrayDimensions> {
       self.texture.as_ref().map(|&(ref d, _)| d.dimensions )
    }

    pub fn clear_fonts(&mut self) {
        if !self.fonts.is_empty() {
            self.fonts.clear();
        }
    }

    pub fn screenshot(&mut self) -> image::DynamicImage {
        let image: glium::texture::RawImage2d<u8> = self.display.read_front_buffer();
        let image = image::ImageBuffer::from_raw(image.width, image.height, image.data.into_owned()).unwrap();
        let image = image::DynamicImage::ImageRgba8(image).flipv();
        image
    }

    pub fn load_resources(&mut self) {
        let (reload_program, reload_texture) = check_reload(&self.resource_file_change_events, &self.shader_pair, &self.texture_directory);

        if reload_program || self.program.is_none() {
            let program_load_result = self.shader_pair.load().and_then(|shader_data| shader_data.load(&self.display));
            println!("program load result -> {:?}", program_load_result);
            self.program = program_load_result.ok();
        }
        
        if reload_texture || self.texture.is_none() {
            println!("reload texture");
            let texture_load_result = self.texture_directory.load().and_then(|mut texture_data| {
                for loaded_font in &self.fonts {
                    println!("copying in font -> {:?} to texture array",loaded_font.font.description);
                    texture_data.images.push(loaded_font.image.clone());
                    texture_data.dimensions.layers += 1;
                }
                texture_data.load(&self.display).map(|texture| (texture_data, texture))
            });

            // .load(&self.display).map(|t| (t, dimensions))
            println!("texture load result -> {:?}", texture_load_result);
            self.texture = texture_load_result.ok();
        }
    }

    pub fn begin(&mut self) -> (Dimensions, InputState) {
        self.load_resources();


        let mut events : Vec<glutin::Event> = Vec::new();

        self.events_loop.poll_events(|ev| events.push(ev));


        self.input_state = input::produce(&self.input_state, &events);

        let new_dimensions = dimensions_for(&self.display);

        (new_dimensions, self.input_state.clone())
    }

//    pub fn render<'b>(&'b mut self, clear_color: Color) -> JamResult<RenderFrame<'b>> {
//        if let (&Some(ref pr), &mut Some((_, ref mut texture))) = (&self.program, &mut self.texture) {

    pub fn render<'b>(&'b self, clear_color: Color) -> JamResult<RenderFrame<'b>> {
        if let (&Some(ref pr), &Some((_, ref texture))) = (&self.program, &self.texture) {
            let mut frame = self.display.draw(); 
            frame.clear_color_srgb_and_depth(clear_color.float_tup(), 1.0);
            Ok(RenderFrame {
                display: &self.display,
                frame: frame,
                program: pr,
                texture: texture,
                dimensions: self.dimensions,
            })
        } else {
            Err(JamError::RenderingPipelineIncomplete)
        }
    }
}

pub struct RenderFrame<'a> {
    pub frame: glium::Frame,
    pub display: &'a glium::Display,
    pub program: &'a Program,
    pub texture: &'a glium::texture::SrgbTexture2dArray,
    pub dimensions: Dimensions,
}

impl<'a> RenderFrame<'a> {
    pub fn finish(self) -> JamResult<()> {
        self.frame.finish().map_err(JamError::SwapBufferError)
    }

    pub fn clear_depth(&mut self) {
        self.frame.clear_depth(1.0)
    }

    pub fn upload(&self, vertices: &[Vertex]) -> GeometryBuffer {
        let vbo = VertexBuffer::persistent(self.display, vertices).unwrap();
        GeometryBuffer { vertex_buffer: vbo }
    }

    pub fn draw_vertices(&mut self, vertices: &[Vertex], uniforms: Uniforms, blend:Blend) -> GeometryBuffer {
        let vbo = VertexBuffer::persistent(self.display, vertices).unwrap();
        let geometry = GeometryBuffer { vertex_buffer: vbo };
        self.draw(&geometry, uniforms, blend);
        geometry
    }

    pub fn draw(&mut self, geometry: &GeometryBuffer, uniforms: Uniforms, blend:Blend) {
        // need to get this outta here
        let tex = self.texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest);

        let u = uniform! {
            u_matrix: uniforms.transform,
            u_texture_array: tex,
            u_color: uniforms.color.float_raw(),
            u_alpha_minimum: 0.01_f32,
        };

        let bl = program::draw_params_for_blend(blend);

        self.frame.draw(&geometry.vertex_buffer, &index::NoIndices(index::PrimitiveType::TrianglesList), self.program, &u, &bl).unwrap();
    }

    pub fn draw_view<Ev>(&mut self, view:&View<Ev>) { // transform?
        println!("draw view");
        let mut vertices : Vec<Vertex> = Vec::new();

        let t = GeometryTesselator::new(vec3(1.0, 1.0, 1.0));
//        draw_ui(&self, vertices: &mut Vec<Vertex>, tr:&TextureRegion, layer: u32, x:f64, y:f64, z:f64, scale: f64)

//        let texture_region = TextureRegion {
//            pub u_min: u32,
//            pub u_max: u32,
//            pub v_min: u32,
//            pub v_max: u32,
//            pub texture_size: u32,
//        };
        let scale = 1.0;
        for (l, rect_abs, (v_z, l_z)) in view.layer_iter() {
            let z = v_z as f64 * 1.0 + l_z as f64 * 0.1;

//            t.draw_ui(&mut vertices, tr:&TextureRegion, layer: u32, x:f64, y:f64, z, scale);
        }

        let tex = self.texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest);

        let (pixels_wide, pixels_high) = self.dimensions.pixels;

        let transform = camera::ui_projection(pixels_wide as f64, pixels_high as f64);
        let u = uniform! {
            u_matrix: down_size_m4(transform.into()),
            u_texture_array: tex,
            u_color: color::WHITE.float_raw(),
            u_alpha_minimum: 0.01_f32,
        };
        let vbo = VertexBuffer::persistent(self.display, &vertices).unwrap();
        let bl = program::draw_params_for_blend(Blend::None);
        self.frame.draw(&vbo, &index::NoIndices(index::PrimitiveType::TrianglesList), self.program, &u, &bl).unwrap();
    }
}

pub fn check_reload(rx: &Receiver<RawEvent>, shader_pair:&ShaderPair, texture_directory: &TextureDirectory) -> (bool, bool) {
    let mut reload_program = false;
    let mut reload_texture = false;

    'fs: loop {
        match rx.try_recv() {
            Ok(RawEvent { path, op:_, cookie:_ }) => {
                if let Some(p) = path {
                    if shader_pair.contains(&p) {
                        reload_program = true;
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

    (reload_program, reload_texture)
}