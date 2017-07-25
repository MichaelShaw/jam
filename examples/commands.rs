#![allow(dead_code)]
extern crate jam;
extern crate cgmath;
extern crate time;
extern crate glutin;
extern crate image;
extern crate gfx_device_gl;

#[macro_use]
extern crate aphid;

use std::f64::consts::PI;
use std::path::{Path, PathBuf};

use cgmath::Rad;

use aphid::{HashSet, Seconds};

use jam::color;
use jam::{Vec3, Vec2, JamResult, Dimensions, Color, rgb, Camera, InputState, FontDirectory};

use jam::render::*;
use jam::render::gfx::{Renderer, GeometryBuffer, OpenGLRenderer, construct_opengl_renderer};

use aphid::HashMap;


fn main() {
    let resources_path = PathBuf::from("resources");
    let shader_pair = ShaderPair::for_paths("resources/shader/fat.vert", "resources/shader/fat.frag");
    let texture_dir = TextureDirectory::for_path("resources/textures", hashset!["png".into()]);
    let font_dir = FontDirectory::for_path("resources/fonts");

    let file_resources = FileResources {
        resources: resources_path,
        shader_pair : shader_pair,
        texture_directory: texture_dir,
        font_directory: font_dir,
    };

    println!("creating renderer");
    let renderer = construct_opengl_renderer(file_resources, (800, 600), true, "commands example".into()).expect("a renderer");
    println!("done creating renderer");
    let mut app = App {
        name: "mixalot".into(),
        camera: Camera {
            at: Vec3::new(0.0, 0.0, 0.0),
            pitch: Rad(PI / 4.0_f64),
            viewport: Dimensions { 
                pixels: (800,600),
                points: (800,600),
            },
            points_per_unit: 16.0 * 1.0,
        },
        zoom: 1.0,
        points_per_unit: 16.0,
        n: 0, // frame counter
        renderer: renderer,
        geometry: HashMap::default(),
    };
    app.run();
}

struct App {
    name : String,
    camera : Camera,
    zoom : f64,
    points_per_unit : f64,
    n : u64,
    renderer: OpenGLRenderer,
    geometry : HashMap<String, GeometryBuffer<gfx_device_gl::Resources>>,
}

impl App {
    fn run(&mut self) {
        let mut last_time = time::precise_time_ns();
        'main: loop {
            let (dimensions, input_state) = self.renderer.begin_frame(rgb(132, 193, 255));

            if input_state.close {
                break;
            }

//            println!("dimensions -> {:?}", dimensions);
            let time = time::precise_time_ns();
            let delta_time = ((time - last_time) as f64) / 1_000_000.0;

            self.update(&input_state, dimensions, delta_time);  

            let res = self.render().expect("successful rendering");

            last_time = time;

        }
    }

    fn units_per_point(&self) -> f64 {
        1.0 / self.points_per_unit
    }

    fn tesselator(&self) -> GeometryTesselator {
        let upp = self.units_per_point();
        let tesselator_scale = Vec3::new(upp, upp, upp);
        GeometryTesselator::new(tesselator_scale)
    }

    fn update(&mut self, input_state:&InputState, dimensions:Dimensions, delta_time: Seconds) {
        use glutin::VirtualKeyCode;
        self.n += 1;

        self.camera.at = Vec3::new(17.0, 0.0, 17.0);
        // self.camera.at = Vec3::new(8.0, 0.0, 8.0);
        self.camera.points_per_unit = self.points_per_unit * self.zoom;
        self.camera.viewport = dimensions;

//        println!("Camera viewpoinrt -> {:?}", self.camera.viewport);

        // let (mx, my) = input_state.mouse.at;
        // let mouse_at = self.camera.ui_line_segment_for_mouse_position(mx, my);

        if input_state.keys.pushed.contains(&VirtualKeyCode::P) {
//            println!("take a screenshot!");
//            let image = self.renderer.screenshot();
//            let mut output = std::fs::file::create(&path::new("screenshot.png")).unwrap();
//            image.save(&mut output, image::imageformat::png).unwrap();
        }
    }

    fn render(&mut self) -> JamResult<()> {
//        use jam::font::FontDescription;
        
//        let font_description = FontDescription { family: "Roboto-Medium".into(), pixel_size: (32f64 * self.camera.viewport.scale()) as u32 };
//        let loaded = self.renderer.load_font(&font_description);
//        match loaded {
//            Err(e) => println!("font load error -> {:?}", e),
//            Ok(_) => (),
//        }

        // println!("render with delta -> {:?}", delta_time);
        let colors = vec![color::WHITE, color::BLUE, color::RED];
        
        // let (w, h) = self.camera.viewport.pixels;
        // let line = self.camera.ray_for_mouse_position((w / 2) as i32, (h / 2) as i32);
        // println!("forward line -> {:?}", line);
   
        let an = self.n / 60;

        let on_second = (self.n % 60) == 0;

        let raster_color = colors[(((an / 16) % 16) % 3) as usize]; // cycles every 16 seconds

        if on_second && an % 5 == 0 { // every fifth second
            let column = (an / 4) % 4;
            let name : String = format!("zone_{}", column);
            println!("delete {}", name);
            let pred : Box<Fn(&String) -> bool> = Box::new(move |key| key.starts_with(&name));

            let keys_to_delete : Vec<_>= self.geometry.keys().filter(|e| pred(e)).cloned().collect();
            for key in keys_to_delete.iter() {
                self.geometry.remove(key);
            }
        }

        // k.starts_with(&prefix)

        let n = (((an % 16) as f64) / 16.0 * 255.0) as u8;

        let mut t = self.tesselator();
        let mut vertices = Vec::new();
        let cache = &mut self.geometry;
        let camera = &self.camera;

        // this closure shit is just dangerous


        
        // render a grid of various bits of geo
        for i in 0..16 {
            let xo = i % 4;
            let zo = i / 4;
            let name : String = format!("zone_{}_{}", xo, zo);

            if (an % 16) == i && on_second {
                raster(&mut t, &mut vertices, raster_color, (xo * 9) as f64, (zo * 9) as f64);
                let geo = self.renderer.draw_vertices(&vertices, Uniforms {
                    transform : down_size_m4(camera.view_projection().into()),
                    color: color::WHITE,
                }, Blend::None)?;
                cache.insert(name, geo);
            } else if ((an+8) % 16) == i && on_second {
                raster(&mut t, &mut vertices, raster_color, (xo * 9) as f64, (zo * 9) as f64);
                cache.insert(name, self.renderer.upload(&vertices));
            } else {
                let rem = (xo + zo) % 3;
                let color = match rem {
                    0 => color::rgba(255,255,255, 128),
                    1 => color::rgba(255,255,255, 50),
                    _ => color::WHITE,
                };
                if let Some(geo) = cache.get(&name) {
                    let blend = match rem {
                        0 => Blend::Alpha,   
                        1 => Blend::Add,
                        _ => Blend::None,
                    };
                    self.renderer.draw(geo, Uniforms {
                        transform: down_size_m4(self.camera.view_projection().into()),
                        color: color,
                    },blend)?;
                }
            }
        }

        // draw ui text

//        if let Some((font, layer)) = self.renderer.get_font(&font_description) {
//            // println!("ok we got a font to use to draw layer -> {:?}", layer);
//            let scale = 1.0 / self.camera.viewport.scale();
//            let mut t = GeometryTesselator::new(Vec3::new(1.0, 1.0, 1.0));
//
//            let texture_region = TextureRegion {
//                u_min: 0,
//                u_max: 128,
//                v_min: 0,
//                v_max: 128,
//                texture_size: 1024,
//            };
//            t.color = color::WHITE.float_raw();
//            t.draw_ui(&mut vertices, &texture_region, 0, 20.0, 20.0, 0.0, 1.0);
//
//            let at = Vec2::new(0.0, 400.0);
//            t.color = color::BLACK.float_raw();
//            text::render_text(
//                "Why oh why does a silly cow fly, you idiot.\n\nGo die in a pie.\n\nPls.",
//                font,
//                layer,
//                at,
//                -1.0, // i assume this is because our coordinate system is hosed ...
//                scale,
//                &t,
//                &mut vertices,
//                Some(300.0)
//            );
//
//            frame.draw_vertices(&vertices, Uniforms {
//                transform : down_size_m4(self.camera.ui_projection().into()),
//                color: color::WHITE,
//            }, Blend::Alpha);
//        }

        self.renderer.finish_frame().expect("a finished frame");

        Ok(())
    }
}

fn raster(t: &mut GeometryTesselator, vertices: &mut Vec<Vertex>, color:Color, x:f64, z:f64) {
    vertices.clear();

    let texture_region = TextureRegion {
        u_min: 0,
        u_max: 128,
        v_min: 0,
        v_max: 128,
        layer: 0,
        texture_size: 1024,
    };

    let texture_region_small = TextureRegion {
        u_min: 16,
        u_max: 32,
        v_min: 16,
        v_max: 32,
        layer: 0,
        texture_size: 1024,
    };

    t.color = color.float_raw();
    // .h_flip().v_flip()
    t.draw_floor_tile(vertices, &texture_region, x, 0.0, z, 0.0);
    t.color = color::RED.float_raw();
    t.draw_wall_tile(vertices, &texture_region_small, x, 0.0, z, 0.0);
    t.color = color::GREEN.float_raw();
    t.draw_floor_centre_anchored(vertices, &texture_region_small, x + 2.0, 0.0, z + 2.0, 0.1);
    t.color = color::YELLOW.float_raw();

    t.draw_floor_centre_anchored_rotated(vertices, &texture_region_small, x + 4.0, 0.0, z + 4.0, 0.0, 0.1);

    t.color = color::RED.float_raw();
    t.draw_wall_base_anchored(vertices, &texture_region_small, x + 3.0, 0.0, z, 0.0);
    t.color = color::YELLOW.float_raw();
    t.draw_wall_centre_anchored(vertices, &texture_region_small, x + 5.0, 1.0, z, 0.0);
}
