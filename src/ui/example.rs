
use aphid::{HashSet, HashMap, Seconds};

use FontDirectory;
use {Camera, Vec3, Vec2, InputState, JamResult, rgb};
use color;
use render::*;
use render::gfx::{Renderer,OpenGLRenderer, GeometryBuffer, construct_opengl_renderer};

use time;
use cgmath::Rad;

use std::f64::consts::PI;
use Dimensions;
use std::path::PathBuf;
use ui::*;

pub fn run() {
    let resources_path = PathBuf::from("resources");
    let shader_pair = ShaderPair::for_paths("resources/shader/fat.vert", "resources/shader/fat.frag");
    let texture_dir = TextureDirectory::for_path("resources/textures", hashset!["png".into()]);
    let font_dir = FontDirectory::for_path("resources/fonts");

    let file_resources = FileResources {
        resources: resources_path,
        shader_pair: shader_pair,
        texture_directory: texture_dir,
        font_directory: font_dir,
    };

    let renderer = construct_opengl_renderer(file_resources, (800, 600), true, "ui example").expect("a renderer");

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
            let (dimensions, input_state) = self.renderer.begin_frame(color::BLACK);

            let time = time::precise_time_ns();
            let delta_time = ((time - last_time) as f64) / 1_000_000.0;

            self.update(&input_state, dimensions, delta_time);

            let res = self.render();

            self.renderer.finish_frame().expect("no errors");

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

        self.renderer.draw_view(&self.widget_runner.view());

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

        view.sub_views.push(label_view(RectI::new(vec2(0, 0), vec2(100, 100)), state.score_a.to_string() ));
        view.sub_views.push(label_view(RectI::new(vec2(100, 0), vec2(100, 100)), state.score_b.to_string() ));
        view.sub_views.push(label_view(RectI::new(vec2(200, 0), vec2(100, 100)), format!("P{} {}", state.period, state.time_remaining)));
        view.sub_views.push(label_view(RectI::new(vec2(0, 100), vec2(300, 40)), state.play_status.clone()));

        view
    }
}
