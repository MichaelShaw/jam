
use glium::{Program, Surface};

use glium::VertexBuffer;
use glium::index;
use glium;
use glutin;

use render::shader::ShaderPair;
use render::texture_array::{TextureDirectory, TextureArrayDimensions};

use input;

use input::InputState;
use color::Color;

use glium::texture::srgb_texture2d_array::SrgbTexture2dArray;

use render::command::{Uniforms, Blend};

use std::sync::mpsc::{channel, Receiver};

use notify::{RecommendedWatcher, Watcher, RecursiveMode, RawEvent};

use super::window;
use super::program;

use dimensions::Dimensions;
use render::vertex::Vertex;

use font::*;

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
    pub display: glium::Display,
    pub input_state: InputState,
    pub program : Option<Program>,
    pub texture : Option<(SrgbTexture2dArray, TextureArrayDimensions)>,
    pub last_dimensions : Dimensions,
    pub fonts: Vec<LoadedBitmapFont>,
}

fn dimensions_for(display : &glium::Display) -> Dimensions {
    let (width_pixels, height_pixels) = display.get_framebuffer_dimensions();

    let scale : f64 = display.get_window().map(|w| w.hidpi_factor() as f64).unwrap_or(1.0);

    Dimensions {
        pixels: (width_pixels, height_pixels),
        scale: scale,
    }  
}

impl Renderer {
    pub fn new(shader_pair : ShaderPair, texture_directory: TextureDirectory, font_directory: FontDirectory, initial_dimensions: (u32, u32), vsync:bool, window_name: String) -> JamResult<Renderer> { //  
        let (tx, notifier_rx) = channel::<RawEvent>();

        let mut resource_file_watcher : RecommendedWatcher = Watcher::new_raw(tx).expect("a watcher");
        resource_file_watcher.watch(&shader_pair.vertex_path, RecursiveMode::Recursive).expect("watching shader vertex path");
        resource_file_watcher.watch(&shader_pair.fragment_path, RecursiveMode::Recursive).expect("watching shader fragment path");
        resource_file_watcher.watch(&texture_directory.path, RecursiveMode::Recursive).expect("watching texture directory path");

        let display = window::create_window(&window_name, vsync, initial_dimensions)?;
        
        let dimensions = dimensions_for(&display);

        Ok(Renderer {
            shader_pair : shader_pair,
            texture_directory: texture_directory,
            font_directory: font_directory,
            resource_file_watcher : resource_file_watcher,
            resource_file_change_events: notifier_rx,
            display: display,
            input_state: InputState::default(),
            program : None,
            texture : None,
            last_dimensions : dimensions,
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
       self.texture.as_ref().map(|&(_, d)| d )
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
                    texture_data.data.push(loaded_font.image.clone().into_raw());
                }
                let mut dimensions = texture_data.dimensions;
                dimensions.layers += self.fonts.len() as u32;
                texture_data.load(&self.display).map(|t| (t, dimensions))
            });
            println!("texture load result -> {:?}", texture_load_result);
            self.texture = texture_load_result.ok();
        }
    }

    pub fn begin(&mut self) -> (Dimensions, InputState) {
        self.load_resources();

        let events : Vec<glutin::Event> = self.display.poll_events().collect();
        self.input_state = input::produce(&self.input_state, &events);

        let new_dimensions = dimensions_for(&self.display);

        if new_dimensions != self.last_dimensions { // this is a fix for Glutin/WINIT, when changing windows with different scales it does (but same points size), it won't properly resize
            println!("resize detected from {:?} to {:?}", self.last_dimensions, new_dimensions);
            if Dimensions::approx_equal_point_size(new_dimensions, self.last_dimensions)  {
                println!("missed resize case detected");
                if let Some(window) = self.display.get_window() {
                    if let Some((w, h)) = window.get_inner_size_points() {
                        println!("jiggling a little");
                        window.set_inner_size(w+1, h);
                        window.set_inner_size(w, h);
                    }
                }    
            }
           
            self.last_dimensions = new_dimensions;
        }

        (new_dimensions, self.input_state.clone())
    }

    pub fn render<'b>(&'b self, clear_color: Color) -> JamResult<RenderFrame<'b>> {
        if let (&Some(ref pr), &Some((ref tr, _))) = (&self.program, &self.texture) {
            let mut frame = self.display.draw(); 
            frame.clear_color_srgb_and_depth(clear_color.float_tup(), 1.0);
            Ok(RenderFrame {
                display: &self.display,
                frame: frame,
                program: pr,
                texture: tr,
            })
            

            // frame.finish().map_err(JamError::SwapBufferError)
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
}

impl<'a> RenderFrame<'a> {
    pub fn finish(self) {
        self.frame.finish().unwrap()
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
}

// do we _really_ need a pass abstraction .... let's at least not race to it..


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