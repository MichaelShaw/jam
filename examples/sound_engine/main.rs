#![allow(dead_code)]

extern crate jam;
extern crate alto;
extern crate cgmath;

use alto::Alto;
// use jam::audio::*;
use jam::audio::{Listener, SoundEvent};
use jam::audio::engine::{process};
use jam::audio::engine::SoundEngineUpdate::*;
use jam::HashMap;
use jam::Vec3f;
use cgmath::Zero;

fn main() {
    let alto = Alto::load("./OpenAL64.dll").unwrap();
    let dev = alto.open(None).unwrap();
    let ctx = dev.new_context(None).unwrap();
    let mut cb = jam::audio::context::create_sound_context(&ctx, "resources/sound", "ogg");
    cb.create_sources(32, 4).unwrap();

    let sounds = vec![("teleport".into(), 1.0), ("water".into(), 1.0)];
    process(&mut cb, Preload(sounds)).unwrap();

    let listener = Listener::default();

    let sound_event = SoundEvent {
        name: "teleport".into(),
        position: Vec3f::zero(),
        gain: 1.0,
        pitch: 1.0,
        attenuation:1.0,
    };

    process(&mut cb, Render { master_gain: 1.0, sounds:vec![sound_event], persistent_sounds:HashMap::default(), listener: listener }).unwrap();

    std::thread::sleep(std::time::Duration::new(2, 0));

    process(&mut cb, Clear).unwrap();

    std::thread::sleep(std::time::Duration::new(1, 0));
}