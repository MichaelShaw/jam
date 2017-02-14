#![allow(dead_code)]

extern crate jam;
extern crate cgmath;
extern crate time;

use jam::render::ShaderPair;
use jam::render::TextureDirectory;
use jam::font::FontDirectory;
use jam::input::InputState;
use jam::camera::Camera;
use jam::color;
use jam::color::Color;

use jam::render::GeometryTesselator;
use jam::render::TextureRegion;
use jam::Vec3;
use std::f64::consts::PI;

use jam::render::command::*;
use jam::render::command::Command::*;
use jam::render::command::Blend;
use jam::render::{Seconds};
use jam::render::dimension::Dimensions;
use jam::render::glium::renderer;
use jam::render::down_size_m4;
use jam::render::glium::renderer::Renderer;

use cgmath::Rad;




fn main() {
    let shader_pair = ShaderPair::for_paths("resources/shader/fat.vert", "resources/shader/fat.frag");
    let texture_dir = TextureDirectory::for_path("resources/textures");
    let font_dir = FontDirectory::for_path("resources/fonts");

    let renderer = Renderer::new(shader_pair, texture_dir, font_dir, (600, 600)).expect("a renderer");

    let mut app = App {
        name: "mixalot".into(),
        camera: Camera {
            at: Vec3::new(0.0, 0.0, 0.0),
            pitch: Rad(PI / 4.0_f64),
            viewport: (800, 600),
            pixels_per_unit: 16.0 * 1.0,
        },
        zoom: 1.0,
        pixels_per_unit: 16.0,
        n: 0, // frame counter
        renderer: renderer,
    };
    app.run();
}

struct App {
    name : String,
    camera : Camera,
    zoom : f64,
    pixels_per_unit : f64,
    n : u64,
    renderer:Renderer<String>,
}

impl App {
    fn run(&mut self) {
        let mut last_time = time::precise_time_ns();
        'main: loop {
            let (dimensions, input_state) = self.renderer.begin();

            let time = time::precise_time_ns();
            let delta_time = ((time - last_time) as f64) / 1_000_000.0;

            self.update(&input_state, dimensions, delta_time);  

            let render_passes = self.render();

            self.renderer.render(render_passes);

            last_time = time;
            if input_state.close {
                break;
            }
        }
    }

    fn units_per_pixel(&self) -> f64 {
        1.0 / self.pixels_per_unit
    }

    fn tesselator(&self) -> GeometryTesselator {
        let upp = self.units_per_pixel();
        let tesselator_scale = Vec3::new(upp, upp, upp);
        GeometryTesselator::new(tesselator_scale)
    }

    fn raster(&self, color:Color, x:f64, z:f64) -> GeometryTesselator {
         let texture_region = TextureRegion {
            u_min: 0,
            u_max: 128,
            v_min: 0,
            v_max: 128,
            texture_size: 1024,
        };

        let texture_region_small = TextureRegion {
            u_min: 16,
            u_max: 32,
            v_min: 16,
            v_max: 32,
            texture_size: 1024,
        };
        
        let mut t = self.tesselator();
        t.color = color.float_raw();
        t.draw_floor_tile(&texture_region, 0, x, 0.0, z, 0.0, false);
        t.color = color::RED.float_raw();
        t.draw_wall_tile(&texture_region_small, 0, x, 0.0, z, 0.0, false);
        t.color = color::GREEN.float_raw();
        t.draw_floor_centre_anchored(&texture_region_small, 0, x + 2.0, 0.0, z + 2.0, 0.1, false);
        t.color = color::YELLOW.float_raw();

        t.draw_floor_centre_anchored_rotated(&texture_region_small, 0, x + 4.0, 0.0, z + 4.0, PI / 4.0, 0.1);

        t.color = color::RED.float_raw();
        t.draw_wall_base_anchored(&texture_region_small, 0, x + 3.0, 0.0, z, 0.0, false);
        t.color = color::YELLOW.float_raw();
        t.draw_wall_centre_anchored(&texture_region_small, 0, x + 5.0, 1.0, z, 0.0, false);
        t
    }

    fn update(&mut self, input_state:&InputState, dimensions:Dimensions, delta_time: Seconds) {
        self.n += 1;

        self.camera.at = Vec3::new(17.0, 0.0, 17.0);
        self.camera.pixels_per_unit = self.pixels_per_unit * self.zoom;

        let (width_points, height_points) = dimensions.points();

        self.camera.viewport = (width_points as u32, height_points as u32);
    }

    fn render(&mut self) -> Vec<Pass<String>> {
        use jam::font::FontDescription;
        let font_description = FontDescription { family: "DejaVuSerif".into(), pixel_size: 16 };
        let loaded = self.renderer.load_font(&font_description);
        match loaded {
            Err(e) => println!("font load error -> {:?}", e),
            Ok(_) => (),
        }

        if let Some((font, layer)) = self.renderer.get_font(&font_description) {
            println!("ok we got a font to use to draw layer -> {:?}", layer);
        }
        // let font = self.renderer.load_font(&font_description);

        // println!("render with delta -> {:?}", delta_time);
        let colors = vec![color::WHITE, color::BLUE, color::RED];
        
        let mut opaque_commands : Vec<Command<String>> = Vec::new();
        let mut translucent_commands : Vec<Command<String>> = Vec::new();
        let mut additive_commands : Vec<Command<String>> = Vec::new();
        
        let an = self.n / 60;

        let on_second = (self.n % 60) == 0;

        let raster_color = colors[(((an / 16) % 16) % 3) as usize]; // cycles every 16 seconds

        if on_second && an % 5 == 0 { // every fifth second
            let column = (an / 4) % 4;
            let name : String = format!("zone_{}", column);
            println!("delete {}", name);
            let pred : Box<Fn(&String) -> bool> = Box::new(move |key| key.starts_with(&name));
            opaque_commands.push(DeleteMatching { pred: pred });
        }

        // k.starts_with(&prefix)

        let n = (((an % 16) as f64) / 16.0 * 255.0) as u8;


        for i in 0..16 {
            let xo = i % 4;
            let zo = i / 4;
            let name : String = format!("zone_{}_{}", xo, zo);

            if (an % 16) == i && on_second {
                let t = self.raster(raster_color, (xo * 9) as f64, (zo * 9) as f64);
                opaque_commands.push(DrawNew {
                    key: Some(name), 
                    vertices: t.tesselator.vertices, 
                    uniforms: Uniforms {
                        transform : down_size_m4(self.camera.view_projection().into()),
                        color: color::WHITE,
                    }
                });
            } else if ((an+8) % 16) == i && on_second {
                let t = self.raster(raster_color, (xo * 9) as f64, (zo * 9) as f64);
                opaque_commands.push(Update {
                    key: name, 
                    vertices: t.tesselator.vertices,
                }); 
            } else {
                let rem = (xo + zo) % 3;

                let color = match rem {
                    0 => color::rgba(255,255,255, 128),
                    1 => color::rgba(255,255,255, 50),
                    _ => color::WHITE,
                };
                let command = Draw {
                    key: name,
                     uniforms: Uniforms {
                        transform: down_size_m4(self.camera.view_projection().into()),
                        color: color,
                    }
                };
                
                match rem {
                    0 => translucent_commands.push(command),   
                    1 => additive_commands.push(command),
                    _ => opaque_commands.push(command),
                };
            }
        }


        vec![Pass {
            blend: Blend::None,
            commands: opaque_commands,
        }, Pass {
            blend: Blend::Alpha,
            commands: translucent_commands,
        }, Pass {
            blend: Blend::Add,
            commands: additive_commands,
        }]
    }
}
