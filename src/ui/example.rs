
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

    let dimensions = Dimensions {
        pixels: (800,600),
        points: (800,600),
    };

    let mut app = App {
        name: "mixalot".into(),
        camera: Camera {
            at: Vec3::new(0.0, 0.0, 0.0),
            pitch: Rad(PI / 4.0_f64),
            viewport: dimensions,
            points_per_unit: 16.0 * 1.0,
        },
        zoom: 1.0,
        points_per_unit: 16.0,
        n: 0, // frame counter
        renderer: renderer,
        widget_runner: WidgetRunner::new(ExampleWidget {}, ExampleState::sample(), dimensions),
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
            let (dimensions, input_state) = self.renderer.begin_frame(rgb(210, 228, 237));

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
        let mut external_events = Vec::new();

        if input_state.mouse.left_released() {
            external_events.push(ExampleEvent::IncrementScoreA);
        }

        self.widget_runner.run(input_state.clone(), external_events, dimensions);
    }

    fn render(&mut self) -> JamResult<()> {
        let mut t = self.tesselator();

        self.renderer.draw_view(&self.widget_runner.view());

        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ExampleEvent {
    IncrementScoreA,
    IncrementScoreB,
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
            time_remaining: "5 00".into(),
            play_status: "Faceoff in 0 00".into(),
        }
    }
}

pub struct ExampleWidget {

}

impl Widget for ExampleWidget {
    type State = ExampleState;
    type Event = ExampleEvent;

    fn update(&self, state:&ExampleState, ev:&ExampleEvent) -> ExampleState {
        let mut new_state = state.clone();
        match ev {
            &ExampleEvent::IncrementScoreA => new_state.score_a += 1,
            &ExampleEvent::IncrementScoreB => new_state.score_b += 1,
        }
        new_state
    }

    fn view(&self, state:&ExampleState, dimensions:Dimensions) -> View<ExampleEvent> {
        let mut view = empty_view(RectI::new(vec2(20, 20), vec2(300, 140)));

        let blue_outter = rgb(78, 117, 137);
        let blue_inner = rgb(116, 181, 231);
        let red_outter = rgb(255, 0, 0);
        let red_inner = rgb(231, 116, 116);

        let light_outter = rgb(255, 255, 255);
        let light_inner = rgb(204, 204, 204);

        let dark_outter = rgb(20, 20, 20);
        let dark_inner = rgb(76, 76, 76);

        let light_text = rgb(255,255,255);
        let dark_text = rgb(51, 51, 51);

        println!("generating view for state -> {:?}", state);


        view.sub_views.push(label_view(RectI::new(vec2(0, 40), vec2(100, 100)), state.score_a.to_string(), light_text, Some(blue_outter), Some(blue_inner)));
        view.sub_views.push(label_view(RectI::new(vec2(100, 40), vec2(100, 100)), state.score_b.to_string(), light_text, Some(red_outter), Some(red_inner)));
        view.sub_views.push(label_view(RectI::new(vec2(200, 40), vec2(100, 100)),  format!("P{} {}", state.period, state.time_remaining), dark_text, Some(light_outter), Some(light_inner)));
        view.sub_views.push(label_view(RectI::new(vec2(0, 0), vec2(300, 40)), state.play_status.clone(), light_text, Some(dark_outter), Some(dark_inner)));

        view
    }
}
