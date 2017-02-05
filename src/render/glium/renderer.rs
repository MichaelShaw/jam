use glium;
use glium::{Program, Surface};
use glium::texture::{Texture2dArray};
use glium::VertexBuffer;
use glium::index;
use glutin;

use render::shader::ShaderPair;
use render::texture_array::TextureDirectory;

use {Mat4};
use JamError;
use input;
use HashMap;
use input::InputState;
use color::{Color, rgb};

use std::sync::mpsc::{channel, Receiver};

use notify::{RecommendedWatcher, Watcher, RecursiveMode, RawEvent};

use super::window;
use super::program;
use render::command::*;
use render::command::Command::*;
use render::{Dimensions, Seconds};
use render::vertex::Vertex;

use time;


pub trait Application {
    fn new(&mut self);
    fn render(&mut self, input:&InputState, dimensions:Dimensions, delta_time: Seconds) -> Vec<Command>; // sizing (window) ?
}

pub fn run_app<T : Application>(mut app:T, shader_pair:ShaderPair, texture_directory: TextureDirectory, initial_dimensions: (u32, u32)) {
    println!("shader pair -> {:?}", shader_pair);
    
    app.new();

    let display = window::create_window("mah window", false);

    let (tx, notifier_rx) = channel::<RawEvent>();
    // , Duration::from_secs(0)
    let mut watcher : RecommendedWatcher = Watcher::new_raw(tx).expect("a watcher");
    watcher.watch(&shader_pair.vertex_path, RecursiveMode::Recursive).expect("watching shader vertex path");
    watcher.watch(&shader_pair.fragment_path, RecursiveMode::Recursive).expect("watching shader fragment path");
    watcher.watch(&texture_directory.path, RecursiveMode::Recursive).expect("watching texture directory path");

    let mut input_state = InputState::default();



    let mut program : Option<Program> = None;
    let mut texture_array : Option<Texture2dArray> = None;
    let mut vertex_buffers : HashMap<BufferKey, VertexBuffer<Vertex>> = HashMap::default();

    let mut last_time = time::precise_time_ns();

    'main: loop {
        let (reload_program, reload_texture) = check_reload(&notifier_rx, &shader_pair, &texture_directory);

        if reload_program || program.is_none() {
            let program_load_result = shader_pair.load().and_then(|shader_data| shader_data.load(&display));
            println!("program load result -> {:?}", program_load_result);
            program = program_load_result.ok();
        }
        
        if reload_texture || texture_array.is_none() {
            println!("reload texture");
            let texture_load_result = texture_directory.load().and_then(|texture_data| texture_data.load(&display));
            println!("texture load result -> {:?}", texture_load_result);
            texture_array = texture_load_result.ok();
        }

        let events : Vec<glutin::Event> = display.poll_events().collect();
        input_state = input::produce(&input_state, &events);

        let (width_pixels, height_pixels) = display.get_framebuffer_dimensions();

        let dimensions = Dimensions {
            width_pixels: width_pixels,
            height_pixels:height_pixels,
            scale: 1.0,
        };
        

        let now  = time::precise_time_ns();
        let delta = ((now - last_time) as f64) / 1000000000.0;
        let commands = app.render(&input_state, dimensions, delta);
        last_time = now;


        if let (&Some(ref pr), &Some(ref tr)) = (&program, &texture_array) {
            let mut target = display.draw();

            let sky_blue = rgb(132, 193, 255);

            target.clear_color_and_depth(sky_blue.float_tup(), 1.0);

            let mut close_after = false;

            for command in commands {
                // println!("received command -> {:?}", command);
                match command {
                    Delete { prefix } => {
                        let keys_to_remove : Vec<String> = vertex_buffers.keys().filter(|k| k.starts_with(&prefix) ).cloned().collect();
                        for key in keys_to_remove.iter() {
                            vertex_buffers.remove(key);
                        }
                    },
                    Update { key, vertices } => {
                        let new_vertex_buffer = VertexBuffer::persistent(&display,&vertices).unwrap();
                        vertex_buffers.insert(key, new_vertex_buffer);
                    },
                    Draw { key, uniforms } => {
                        if let Some(vertex_buffer) = vertex_buffers.get(&key) {
                            let uniforms = uniform! {
                                matrix: uniforms.transform,
                                u_texture_array: tr,
                                u_color: uniforms.color.float_raw(),
                                u_alpha_minimum: 0.05_f32,
                            };

                            target.draw(&vertex_buffer, &index::NoIndices(index::PrimitiveType::TrianglesList), &pr, &uniforms, &program::opaque_draw_params()).unwrap();

                        //     data.vbuf = vertices.buffer.clone();
                        //     data.u_matrix = uniforms.transform;
                        //     data.u_color = uniforms.color.float_raw();
                        } else {
                            // println!("couldnt draw for {:?}", key);
                        }
                    },
                    DrawNew { key , vertices, uniforms } => {
                        // let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&vertices, ());

                        // if let Some(name) = key {
                        //     vertex_buffers.insert(name, Vertices {
                        //         buffer: vertex_buffer.clone(),
                        //         slice: slice.clone(),
                        //     });
                        // }
                        // data.vbuf = vertex_buffer;
                        // data.u_matrix = uniforms.transform;
                        // data.u_color = uniforms.color.float_raw();

                        // if let Some(ref ps) = pso {
                            // encoder.draw(&slice, &ps, &data);
                        // }
                    },
                    Close => {
                        close_after = true;
                    },
                }
            }

            // target.draw(&vertex_buffer, &index::NoIndices(index::PrimitiveType::TrianglesList), &rs.program, &uniforms, &opaque_draw_params()).unwrap();
            target.finish().unwrap();    

            if close_after {
                break 'main;
            }
        } else {
            use std::{thread, time};
            println!("can't render, we're missing resources");
            let ten_millis = time::Duration::from_millis(100);
            thread::sleep(ten_millis);
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