
use aphid::{HashSet, HashMap, Seconds};

use font::FontDirectory;
use {Camera, Vec3, Vec2, InputState, JamResult, rgb};
use color;
use render::*;
use render::gfx::{Renderer,OpenGLRenderer, GeometryBuffer, construct_opengl_renderer};

use time;
use cgmath::Rad;

use std::f64::consts::PI;
use Dimensions;

use ui::*;

pub fn run() {
    let shader_pair = ShaderPair::for_paths("resources/shader/fat.vert", "resources/shader/fat.frag");
    let texture_dir = TextureDirectory::for_path("resources/textures", hashset!["png".into()]);
    let font_dir = FontDirectory::for_path("resources/fonts");

    let file_resources = FileResources {
        shader_pair: shader_pair,
        texture_directory: texture_dir,
        font_directory: font_dir,
    };



    let renderer = construct_opengl_renderer(file_resources, (800, 600), true, "commands example").expect("a renderer");

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
        widget_runner: WidgetRunner::new(ExampleWidget {}, ExampleState::sample()),
    };
    app.run()
}


struct App {
    name : String,
    camera : Camera,
    zoom : f64,
    points_per_unit : f64,
    n : u64,
    renderer : OpenGLRenderer,
    widget_runner: WidgetRunner<ExampleWidget>,
}

impl App {
    fn run(&mut self) {
        let mut last_time = time::precise_time_ns();
        'main: loop {
            let (dimensions, input_state) = self.renderer.begin_frame(color::PINK);

            let time = time::precise_time_ns();
            let delta_time = ((time - last_time) as f64) / 1_000_000.0;

            self.update(&input_state, dimensions, delta_time);

            let res = self.render();

            last_time = time;
            if input_state.close {
                break;
            }
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

    #[allow(unused_variables)]
    fn update(&mut self, input_state:&InputState, dimensions:Dimensions, delta_time: Seconds) {
        self.widget_runner.run(input_state.clone(), Vec::new());
    }

    fn render(&mut self) -> JamResult<()> {
        let mut t = self.tesselator();
//        let mut vertices = Vec::new();

//        let mut frame = self.renderer.render(rgb(132, 193, 255))?;
//
////        let layer = 0;
//        let scale = 1.0 / self.camera.viewport.scale() as f64;
//        let texture_region = TextureRegion {
//            u_min: 0,
//            u_max: 128,
//            v_min: 0,
//            v_max: 128,
//            texture_size: 1024,
//        };
//        t.color = color::WHITE.float_raw();
//        t.draw_ui(&mut vertices, &texture_region, 0, 20.0, 20.0, 0.0, 1.0);
//
//        frame.draw_vertices(&vertices, Uniforms {
//            transform : down_size_m4(self.camera.ui_projection().into()),
//            color: color::WHITE,
//        }, Blend::Alpha);
//
//        frame.draw_view(self.widget_runner.view());
//
//        frame.finish();

        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ExampleEvent {

}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ExampleState {
    score_a: u32,
    score_b: u32,
    period: u32,
    time_remaining:String,
    play_status: String,
}

impl ExampleState {
    pub fn sample() -> ExampleState {
        ExampleState {
            score_a: 0,
            score_b: 1,
            period: 1,
            time_remaining: "5:00".into(),
            play_status: "Faceoff in 0:00".into(),
        }
    }
}

pub struct ExampleWidget {

}

impl Widget for ExampleWidget {
    type State = ExampleState;
    type Event = ExampleEvent;

    fn update(&self, state:&ExampleState, ev:&ExampleEvent) -> ExampleState {
        state.clone()
    }

    fn view(&self, state:&ExampleState) -> View<ExampleEvent> {
        let mut view = empty_view(RectI::new(vec2(20, 20), vec2(300, 140)));

        view.sub_views.push(label_view(RectI::new(vec2(0, 0), vec2(100, 100)), format!("{}", state.score_a )));
        view.sub_views.push(label_view(RectI::new(vec2(100, 0), vec2(100, 100)), format!("{}", state.score_b )));
        view.sub_views.push(label_view(RectI::new(vec2(200, 0), vec2(100, 100)), format!("P{} {}", state.period, state.time_remaining)));
        view.sub_views.push(label_view(RectI::new(vec2(0, 100), vec2(300, 40)), state.play_status.clone()));

        view
    }
}
