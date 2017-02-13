
use glium::{Program, Surface};
use glium::texture::{Texture2dArray};
use glium::VertexBuffer;
use glium::index;
use glium;
use glutin;

use render::shader::ShaderPair;
use render::texture_array::TextureDirectory;

use input;
use HashMap;
use input::InputState;
use color::{rgb};

use std::sync::mpsc::{channel, Receiver};

use notify::{RecommendedWatcher, Watcher, RecursiveMode, RawEvent};

use super::window;
use super::program;
use render::command::*;
use render::command::Command::*;
use render::{Seconds};
use render::dimension::Dimensions;
use render::vertex::Vertex;

use {JamResult, JamError};

pub struct Renderer {
    pub shader_pair : ShaderPair,
    pub texture_directory: TextureDirectory,
    pub resource_file_watcher : RecommendedWatcher,
    pub resource_file_change_events: Receiver<RawEvent>,
    pub display: glium::Display,
    pub input_state: InputState,
    pub program : Option<Program>,
    pub texture : Option<Texture2dArray>,
    pub vertex_buffers : HashMap<BufferKey, VertexBuffer<Vertex>>,
    pub last_dimensions : Dimensions,
}

fn dimensions_for(display : &glium::Display) -> Dimensions {
    let (width_pixels, height_pixels) = display.get_framebuffer_dimensions();

    let scale : f32 = display.get_window().map(|w| w.hidpi_factor()).unwrap_or(1.0);

    Dimensions {
        width_pixels: width_pixels,
        height_pixels:height_pixels,
        scale: scale,
    }  
}

impl Renderer {
    pub fn new(shader_pair : ShaderPair, texture_directory: TextureDirectory, initial_dimensions: (u32, u32)) -> JamResult<Renderer> { //  
        let (tx, notifier_rx) = channel::<RawEvent>();

        let mut resource_file_watcher : RecommendedWatcher = Watcher::new_raw(tx).expect("a watcher");
        resource_file_watcher.watch(&shader_pair.vertex_path, RecursiveMode::Recursive).expect("watching shader vertex path");
        resource_file_watcher.watch(&shader_pair.fragment_path, RecursiveMode::Recursive).expect("watching shader fragment path");
        resource_file_watcher.watch(&texture_directory.path, RecursiveMode::Recursive).expect("watching texture directory path");

        let display = window::create_window("mah window", true, initial_dimensions)?;
        
        let dimensions = dimensions_for(&display);

        Ok(Renderer {
            shader_pair : shader_pair,
            texture_directory: texture_directory,
            resource_file_watcher : resource_file_watcher,
            resource_file_change_events: notifier_rx,
            display: display,
            input_state: InputState::default(),
            program : None,
            texture : None,
            vertex_buffers : HashMap::default(),
            last_dimensions : dimensions,
        })
    }

    pub fn begin(&mut self) -> (Dimensions, InputState) {
        let (reload_program, reload_texture) = check_reload(&self.resource_file_change_events, &self.shader_pair, &self.texture_directory);

        if reload_program || self.program.is_none() {
            let program_load_result = self.shader_pair.load().and_then(|shader_data| shader_data.load(&self.display));
            println!("program load result -> {:?}", program_load_result);
            self.program = program_load_result.ok();
        }
        
        if reload_texture || self.texture.is_none() {
            println!("reload texture");
            let texture_load_result = self.texture_directory.load().and_then(|texture_data| {
                texture_data.load(&self.display)
            });
            println!("texture load result -> {:?}", texture_load_result);
            self.texture = texture_load_result.ok();
        }

        let events : Vec<glutin::Event> = self.display.poll_events().collect();
        self.input_state = input::produce(&self.input_state, &events);

        let new_dimensions = dimensions_for(&self.display);

        if new_dimensions != self.last_dimensions { // this is a fix for Glutin/WINIT, when changing windows with different scales it does (but same points size), it won't properly resize
            self.last_dimensions = new_dimensions;
            if let Some(window) = self.display.get_window() {
                if let Some((w, h)) = window.get_inner_size_points() {
                    window.set_inner_size(w+1, h);
                    window.set_inner_size(w, h);
                }
            }
        }

        (new_dimensions, self.input_state.clone())
    }

    pub fn render(&mut self, commands: Vec<Command>) -> JamResult<()> {
        if let (&Some(ref pr), &Some(ref tr)) = (&self.program, &self.texture) {
            let mut target = self.display.draw();

            let sky_blue = rgb(132, 193, 255);

            target.clear_color_and_depth(sky_blue.float_tup(), 1.0);

            for command in commands {
                // println!("received command -> {:?}", command);
                match command {
                    Delete { prefix } => {
                        let keys_to_remove : Vec<String> = self.vertex_buffers.keys().filter(|k| k.starts_with(&prefix) ).cloned().collect();
                        for key in keys_to_remove.iter() {
                            self.vertex_buffers.remove(key);
                        }
                    },
                    Update { key, vertices } => {
                        let new_vertex_buffer = VertexBuffer::persistent(&self.display,&vertices).unwrap();
                        self.vertex_buffers.insert(key, new_vertex_buffer);
                    },
                    Draw { key, uniforms } => {
                        if let Some(vertex_buffer) = self.vertex_buffers.get(&key) {
                            let uniforms = uniform! {
                                u_matrix: uniforms.transform,
                                u_texture_array: tr.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                                u_color: uniforms.color.float_raw(),
                                u_alpha_minimum: 0.01_f32,
                            };
                            target.draw(vertex_buffer, &index::NoIndices(index::PrimitiveType::TrianglesList), &pr, &uniforms, &program::opaque_draw_params()).unwrap();
                        } else {
                            // println!("couldnt draw for {:?}", key);
                        }
                    },
                    DrawNew { key , vertices, uniforms } => {
                        let new_vertex_buffer = VertexBuffer::persistent(&self.display,&vertices).unwrap();

                        let uniforms = uniform! {
                            u_matrix: uniforms.transform,
                            u_texture_array: tr.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                            u_color: uniforms.color.float_raw(),
                            u_alpha_minimum: 0.01_f32,
                        };

                        target.draw(&new_vertex_buffer, &index::NoIndices(index::PrimitiveType::TrianglesList), &pr, &uniforms, &program::opaque_draw_params()).unwrap();

                        if let Some(name) = key {
                            self.vertex_buffers.insert(name,new_vertex_buffer);
                        }
                    },
                }
            }
            target.finish().map_err(JamError::SwapBufferError)
        } else {
            Err(JamError::RenderingPipelineIncomplete)
        }
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